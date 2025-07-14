// Parser + AST Definitions
use regex::Error as RegexError;
use std::{fs::File, io::Read, result::Result, collections::VecDeque, error::Error};
use thiserror::Error;
use crate::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    None,
}

#[derive(Error, Debug)]
pub enum ScriptError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Regex error: {0}")]
    Regex(#[from] RegexError),
    #[error("Parse error: {0}")]
    Parse(String),
}

pub trait Node {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value;
}

pub struct Parser {
    tokens: VecDeque<Token>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens.into_iter().collect(),
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Box<dyn Node>>, Box<dyn Error>> {
        let mut statements = Vec::new();
        
        while !self.tokens.is_empty() {
            match &self.tokens[0] {
                Token::BeginProj(_) => {
                    let proj_statements = self.parse_project()?;
                    statements.extend(proj_statements);
                }
                _ => {
                    let statement = self.parse_statement()?;
                    statements.push(statement);
                }
            }
        }
        
        Ok(statements)
    }

    pub fn parse_project(&mut self) -> Result<Vec<Box<dyn Node>>, Box<dyn Error>> {
        let _proj_name = if let Token::BeginProj(name) = self.tokens.pop_front().unwrap() {
            name
        } else {
            return Err("Expected project name".into());
        };
        
        let mut statements = Vec::new();
        while !self.tokens.is_empty() {
            match &self.tokens[0] {
                Token::EndProj(_) => {
                    self.tokens.pop_front(); // consume END PROJ
                    break;
                }
                Token::BeginSection(_) => {
                    let section_statements = self.parse_section()?;
                    statements.extend(section_statements);
                }
                _ => {
                    let statement = self.parse_statement()?;
                    statements.push(statement);
                }
            }
        }
        Ok(statements)
    }

    pub fn parse_section(&mut self) -> Result<Vec<Box<dyn Node>>, Box<dyn Error>> {
        let _section_name = if let Token::BeginSection(name) = self.tokens.pop_front().unwrap() {
            name
        } else {
            return Err("Expected section name".into());
        };
        
        let mut statements = Vec::new();
        while !self.tokens.is_empty() {
            match &self.tokens[0] {
                Token::EndSection(_) => {
                    self.tokens.pop_front(); // consume END SECTION
                    break;
                }
                _ => {
                    let statement = self.parse_statement()?;
                    statements.push(statement);
                }
            }
        }
        Ok(statements)
    }

    pub fn parse_statement(&mut self) -> Result<Box<dyn Node>, Box<dyn Error>> {
        if self.tokens.is_empty() {
            return Err("Unexpected end of input".into());
        }

        match &self.tokens[0] {
            Token::Var => {
                self.tokens.pop_front(); // consume "var"
                let var_name = if let Token::Identifier(name) = &self.tokens[0] {
                    name.clone()
                } else {
                    return Err("Expected variable name after 'var'".into());
                };
                self.tokens.pop_front(); // consume identifier
                
                if let Some(Token::Assign) = self.tokens.get(0) {
                    self.tokens.pop_front(); // consume ":="
                    let expr = self.parse_expression()?;
                    Ok(Box::new(Assign::parse(var_name, expr)))
                } else {
                    Err("Expected assignment operator ':='".into())
                }
            }
            Token::Ret => {
                self.tokens.pop_front(); // consume "ret"
                let expr = self.parse_expression()?;
                Ok(Box::new(FunctionCall::parse("print".to_string(), vec![expr])))
            }
            Token::Identifier(name) if name == "print" => {
                self.tokens.pop_front(); // consume "print"
                let expr = self.parse_expression()?;
                Ok(Box::new(FunctionCall::parse("print".to_string(), vec![expr])))
            }
            _ => {
                let expr = self.parse_expression()?;
                Ok(expr)
            }
        }
    }

    pub fn parse_expression(&mut self) -> Result<Box<dyn Node>, Box<dyn Error>> {
        if self.tokens.is_empty() {
            return Err("Unexpected end of input".into());
        }

        match &self.tokens[0] {
            Token::Number(value) => {
                let value = *value;
                self.tokens.pop_front();
                Ok(Box::new(Number::parse(&value.to_string())))
            }
            Token::StringLiteral(value) => {
                let value = value.clone();
                self.tokens.pop_front();
                Ok(Box::new(StringLiteral::parse(value)))
            }
            Token::Identifier(name) => {
                let name = name.clone();
                self.tokens.pop_front();
                
                // Check if this is a function call (followed by an identifier or literal)
                if !self.tokens.is_empty() {
                    match &self.tokens[0] {
                        Token::Identifier(_) | Token::Number(_) | Token::StringLiteral(_) => {
                            let args = vec![self.parse_expression()?];
                            Ok(Box::new(FunctionCall::parse(name, args)))
                        }
                        _ => Ok(Box::new(Var::parse(name))),
                    }
                } else {
                    Ok(Box::new(Var::parse(name)))
                }
            }
            _ => {
                let token = self.tokens.pop_front().unwrap();
                Err(format!("Unexpected token: {:?}", token).into())
            }
        }
    }
}

pub struct EvalMathExp {
    pub expr: String,
}

impl EvalMathExp {
    pub fn parse(expr_tokens: Vec<&str>) -> Self {
        Self {
            expr: expr_tokens.join(" "),
        }
    }
}

impl Node for EvalMathExp {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_eval_math_exp(self)
    }
}

pub struct BinOp {
    pub left: Box<dyn Node>,
    pub op: String,
    pub right: Box<dyn Node>,
}

impl BinOp {
    pub fn parse(left: Box<dyn Node>, op: String, right: Box<dyn Node>) -> Self {
        Self { left, op, right }
    }
}

impl Node for BinOp {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_bin_op(self)
    }
}

pub struct Number {
    pub value: f64,
}

impl Number {
    pub fn parse(value: &str) -> Self {
        Self {
            value: value.parse().unwrap(),
        }
    }
}

impl Node for Number {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_number(self)
    }
}

pub struct Var {
    pub name: String,
}

impl Var {
    pub fn parse(name: String) -> Self {
        Self { name }
    }
}

impl Node for Var {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_var(self)
    }
}

pub struct Assign {
    pub name: String,
    pub expr: Box<dyn Node>,
}

impl Assign {
    pub fn parse(name: String, expr: Box<dyn Node>) -> Self {
        Self { name, expr }
    }
}

impl Node for Assign {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_assign(self)
    }
}

pub struct StringLiteral {
    pub value: String,
}

impl StringLiteral {
    pub fn parse(value: String) -> Self {
        Self { value }
    }
}

impl Node for StringLiteral {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_string_literal(self)
    }
}

pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Box<dyn Node>>,
}

impl FunctionCall {
    pub fn parse(name: String, args: Vec<Box<dyn Node>>) -> Self {
        Self { name, args }
    }
}

impl Node for FunctionCall {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_function_call(self)
    }
}

pub struct Comment {
    pub value: String,
}

impl Comment {
    pub fn parse(value: String) -> Self {
        Self { value }
    }
}

impl Node for Comment {
    fn accept(&self, visitor: &mut dyn Visitor) -> Value {
        visitor.visit_comment(self)
    }
}

pub trait Visitor {
    fn visit_eval_math_exp(&mut self, node: &EvalMathExp) -> Value;
    fn visit_bin_op(&mut self, node: &BinOp) -> Value;
    fn visit_number(&mut self, node: &Number) -> Value;
    fn visit_var(&mut self, node: &Var) -> Value;
    fn visit_assign(&mut self, node: &Assign) -> Value;
    fn visit_string_literal(&mut self, node: &StringLiteral) -> Value;
    fn visit_function_call(&mut self, node: &FunctionCall) -> Value;
    fn visit_comment(&mut self, node: &Comment) -> Value;
}

pub fn parse_script(source: String) -> Result<Vec<Box<dyn Node>>, Box<dyn Error>> {
    use crate::tokenizer::tokenize;

    let mut file = File::open(source)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let tokens = tokenize(&content);
    let mut parser = Parser::new(tokens);
    parser.parse_program()
}
