// Enhanced Parser + AST Definitions
use crate::tokenizer::Token;
use crate::types::OakType;
use std::{collections::VecDeque, error::Error, fs::File, io::Read};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Bool(bool),
    Identifier(String),
    BinaryOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
    UnaryOp {
        op: String,
        operand: Box<Expr>,
    },
    Cast {
        expr: Box<Expr>,
        target_type: OakType,
    },
    Array(Vec<Expr>),
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expr),
    Block(Vec<Statement>),
    IfElse {
        condition: Expr,
        then_block: Box<Statement>,
        else_block: Option<Box<Statement>>,
    },
    While {
        condition: Expr,
        body: Box<Statement>,
    },
    For {
        var: String,
        iter: Expr,
        body: Box<Statement>,
    },
    Return(Option<Expr>),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub enum AstNode {
    VarDeclaration {
        name: String,
        ty: OakType,
        value: Option<Expr>,
    },
    FunctionDeclaration {
        name: String,
        params: Vec<(String, OakType)>,
        return_type: OakType,
        body: Vec<AstNode>,
    },
    StructDeclaration {
        name: String,
        fields: Vec<(String, OakType)>,
    },
    ExternFunction {
        name: String,
        params: Vec<OakType>,
        return_type: OakType,
    },
    Statement(Statement),
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

    pub fn parse(&mut self) -> Result<Vec<AstNode>, Box<dyn Error>> {
        let mut nodes = Vec::new();

        while !self.tokens.is_empty() {
            self.skip_comments();
            if !self.tokens.is_empty() {
                nodes.push(self.parse_top_level()?);
            }
        }

        Ok(nodes)
    }

    fn skip_comments(&mut self) {
        while let Some(Token::Comment(_)) = self.tokens.front() {
            self.tokens.pop_front();
        }
    }

    fn parse_top_level(&mut self) -> Result<AstNode, Box<dyn Error>> {
        self.skip_comments();

        match self.tokens.front() {
            Some(Token::Let) | Some(Token::Var) | Some(Token::Mut) => {
                self.parse_variable_declaration()
            }
            Some(Token::Fn) => self.parse_function_declaration(),
            Some(Token::Struct) => self.parse_struct_declaration(),
            Some(Token::Extern) => self.parse_extern_function(),
            _ => {
                let stmt = self.parse_statement()?;
                Ok(AstNode::Statement(stmt))
            }
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<AstNode, Box<dyn Error>> {
        self.tokens.pop_front(); // consume let/var/mut

        let name = self.expect_identifier()?;

        let ty = if self.match_token(&Token::Colon) {
            self.parse_type()?
        } else {
            OakType::Inferred
        };

        let value = if self.match_token(&Token::Assign) {
            Some(self.parse_expression()?)
        } else {
            None
        };

        self.consume_semicolon();

        Ok(AstNode::VarDeclaration { name, ty, value })
    }

    fn parse_function_declaration(&mut self) -> Result<AstNode, Box<dyn Error>> {
        self.tokens.pop_front(); // consume fn

        let name = self.expect_identifier()?;

        self.expect(&Token::LeftParen)?;
        let params = self.parse_parameter_list()?;
        self.expect(&Token::RightParen)?;

        let return_type = if self.match_token(&Token::Arrow) {
            self.parse_type()?
        } else {
            OakType::Primitive(crate::types::PrimitiveType::Void)
        };

        self.expect(&Token::LeftBrace)?;
        let body = self.parse_block()?;
        self.expect(&Token::RightBrace)?;

        Ok(AstNode::FunctionDeclaration {
            name,
            params,
            return_type,
            body,
        })
    }

    fn parse_struct_declaration(&mut self) -> Result<AstNode, Box<dyn Error>> {
        self.tokens.pop_front(); // consume struct

        let name = self.expect_identifier()?;

        self.expect(&Token::LeftBrace)?;

        let mut fields = Vec::new();
        while !self.check(&Token::RightBrace) {
            let field_name = self.expect_identifier()?;
            self.expect(&Token::Colon)?;
            let field_type = self.parse_type()?;
            fields.push((field_name, field_type));

            if !self.check(&Token::RightBrace) {
                self.expect(&Token::Comma)?;
            }
        }

        self.expect(&Token::RightBrace)?;

        Ok(AstNode::StructDeclaration { name, fields })
    }

    fn parse_extern_function(&mut self) -> Result<AstNode, Box<dyn Error>> {
        self.tokens.pop_front(); // consume extern
        self.expect_keyword("fn")?;

        let name = self.expect_identifier()?;

        self.expect(&Token::LeftParen)?;
        let params = self.parse_type_list()?;
        self.expect(&Token::RightParen)?;

        let return_type = if self.match_token(&Token::Arrow) {
            self.parse_type()?
        } else {
            OakType::Primitive(crate::types::PrimitiveType::Void)
        };

        self.consume_semicolon();

        Ok(AstNode::ExternFunction {
            name,
            params,
            return_type,
        })
    }

    fn parse_block(&mut self) -> Result<Vec<AstNode>, Box<dyn Error>> {
        let mut statements = Vec::new();

        while !self.check(&Token::RightBrace) && !self.tokens.is_empty() {
            self.skip_comments();
            if !self.check(&Token::RightBrace) {
                statements.push(self.parse_top_level()?);
            }
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, Box<dyn Error>> {
        self.skip_comments();

        match self.tokens.front() {
            Some(Token::LeftBrace) => {
                self.tokens.pop_front();
                let mut stmts = Vec::new();
                while !self.check(&Token::RightBrace) {
                    stmts.push(self.parse_statement()?);
                }
                self.expect(&Token::RightBrace)?;
                Ok(Statement::Block(stmts))
            }
            Some(Token::If) => {
                self.tokens.pop_front();
                let condition = self.parse_expression()?;
                let then_block = Box::new(self.parse_statement()?);
                let else_block = if self.match_token(&Token::Else) {
                    Some(Box::new(self.parse_statement()?))
                } else {
                    None
                };
                Ok(Statement::IfElse {
                    condition,
                    then_block,
                    else_block,
                })
            }
            Some(Token::While) => {
                self.tokens.pop_front();
                let condition = self.parse_expression()?;
                let body = Box::new(self.parse_statement()?);
                Ok(Statement::While { condition, body })
            }
            Some(Token::For) => {
                self.tokens.pop_front();
                let var = self.expect_identifier()?;
                self.expect_keyword("in")?;
                let iter = self.parse_expression()?;
                let body = Box::new(self.parse_statement()?);
                Ok(Statement::For { var, iter, body })
            }
            Some(Token::Ret) => {
                self.tokens.pop_front();
                let expr = if self.check(&Token::Semicolon) {
                    None
                } else {
                    Some(self.parse_expression()?)
                };
                self.consume_semicolon();
                Ok(Statement::Return(expr))
            }
            Some(Token::Break) => {
                self.tokens.pop_front();
                self.consume_semicolon();
                Ok(Statement::Break)
            }
            Some(Token::Continue) => {
                self.tokens.pop_front();
                self.consume_semicolon();
                Ok(Statement::Continue)
            }
            _ => {
                let expr = self.parse_expression()?;
                self.consume_semicolon();
                Ok(Statement::Expression(expr))
            }
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, Box<dyn Error>> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, Box<dyn Error>> {
        self.parse_logical_or()
    }

    fn parse_logical_or(&mut self) -> Result<Expr, Box<dyn Error>> {
        let mut expr = self.parse_logical_and()?;

        while self.match_op("||") {
            let op = "||".to_string();
            let right = self.parse_logical_and()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, Box<dyn Error>> {
        let mut expr = self.parse_equality()?;

        while self.match_op("&&") {
            let op = "&&".to_string();
            let right = self.parse_equality()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expr, Box<dyn Error>> {
        let mut expr = self.parse_comparison()?;

        while let Some(op) = self.match_any_op(&["==", "!="]) {
            let right = self.parse_comparison()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, Box<dyn Error>> {
        let mut expr = self.parse_additive()?;

        while let Some(op) = self.match_any_op(&["<", ">", "<=", ">="]) {
            let right = self.parse_additive()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_additive(&mut self) -> Result<Expr, Box<dyn Error>> {
        let mut expr = self.parse_multiplicative()?;

        while let Some(op) = self.match_any_op(&["+", "-"]) {
            let right = self.parse_multiplicative()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_multiplicative(&mut self) -> Result<Expr, Box<dyn Error>> {
        let mut expr = self.parse_unary()?;

        while let Some(op) = self.match_any_op(&["*", "/", "%"]) {
            let right = self.parse_unary()?;
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expr, Box<dyn Error>> {
        if let Some(op) = self.match_any_op(&["!", "-", "+", "*", "&"]) {
            let operand = self.parse_unary()?;
            Ok(Expr::UnaryOp {
                op,
                operand: Box::new(operand),
            })
        } else {
            self.parse_postfix()
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, Box<dyn Error>> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.match_token(&Token::LeftBracket) {
                let index = self.parse_expression()?;
                self.expect(&Token::RightBracket)?;
                expr = Expr::Index {
                    array: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.match_token(&Token::As) {
                let target_type = self.parse_type()?;
                expr = Expr::Cast {
                    expr: Box::new(expr),
                    target_type,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, Box<dyn Error>> {
        self.skip_comments();

        match self.tokens.front() {
            Some(Token::Number(n)) => {
                let val = *n;
                self.tokens.pop_front();
                Ok(Expr::Number(val))
            }
            Some(Token::StringLiteral(s)) => {
                let val = s.clone();
                self.tokens.pop_front();
                Ok(Expr::String(val))
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.tokens.pop_front();

                if self.match_token(&Token::LeftParen) {
                    let args = self.parse_argument_list()?;
                    self.expect(&Token::RightParen)?;
                    Ok(Expr::FunctionCall { name, args })
                } else {
                    Ok(Expr::Identifier(name))
                }
            }
            Some(Token::LeftBracket) => {
                self.tokens.pop_front();
                let mut elements = Vec::new();

                while !self.check(&Token::RightBracket) {
                    elements.push(self.parse_expression()?);
                    if !self.check(&Token::RightBracket) {
                        self.expect(&Token::Comma)?;
                    }
                }

                self.expect(&Token::RightBracket)?;
                Ok(Expr::Array(elements))
            }
            Some(Token::LeftParen) => {
                self.tokens.pop_front();
                let expr = self.parse_expression()?;
                self.expect(&Token::RightParen)?;
                Ok(expr)
            }
            _ => Err("Unexpected token in expression".into()),
        }
    }

    fn parse_type(&mut self) -> Result<OakType, Box<dyn Error>> {
        self.skip_comments();

        match self.tokens.front() {
            Some(Token::Star) => {
                self.tokens.pop_front();
                let inner = self.parse_type()?;
                Ok(OakType::Pointer(Box::new(inner)))
            }
            Some(Token::Ampersand) => {
                self.tokens.pop_front();
                let inner = self.parse_type()?;
                Ok(OakType::Reference(Box::new(inner)))
            }
            Some(Token::LeftBracket) => {
                self.tokens.pop_front();
                let elem_type = self.parse_type()?;

                let size = if self.match_token(&Token::Semicolon) {
                    if let Some(Token::Number(n)) = self.tokens.front() {
                        let s = Some(*n as usize);
                        self.tokens.pop_front();
                        s
                    } else {
                        None
                    }
                } else {
                    None
                };

                self.expect(&Token::RightBracket)?;
                Ok(OakType::Array(Box::new(elem_type), size))
            }
            Some(Token::Identifier(name)) => {
                let name = name.clone();
                self.tokens.pop_front();

                match name.as_str() {
                    "u8" => Ok(OakType::Primitive(crate::types::PrimitiveType::Uint8)),
                    "u16" => Ok(OakType::Primitive(crate::types::PrimitiveType::Uint16)),
                    "u32" => Ok(OakType::Primitive(crate::types::PrimitiveType::Uint32)),
                    "u64" => Ok(OakType::Primitive(crate::types::PrimitiveType::Uint64)),
                    "i8" => Ok(OakType::Primitive(crate::types::PrimitiveType::Int8)),
                    "i16" => Ok(OakType::Primitive(crate::types::PrimitiveType::Int16)),
                    "i32" => Ok(OakType::Primitive(crate::types::PrimitiveType::Int32)),
                    "i64" => Ok(OakType::Primitive(crate::types::PrimitiveType::Int64)),
                    "posint8" => Ok(OakType::Primitive(crate::types::PrimitiveType::Posint8)),
                    "posint16" => Ok(OakType::Primitive(crate::types::PrimitiveType::Posint16)),
                    "posint32" => Ok(OakType::Primitive(crate::types::PrimitiveType::Posint32)),
                    "posint64" => Ok(OakType::Primitive(crate::types::PrimitiveType::Posint64)),
                    "f32" => Ok(OakType::Primitive(crate::types::PrimitiveType::Float32)),
                    "f64" => Ok(OakType::Primitive(crate::types::PrimitiveType::Float64)),
                    "bool" => Ok(OakType::Primitive(crate::types::PrimitiveType::Bool)),
                    "str" => Ok(OakType::Primitive(crate::types::PrimitiveType::Str)),
                    "Text" => Ok(OakType::Primitive(crate::types::PrimitiveType::Text)),
                    "void" => Ok(OakType::Primitive(crate::types::PrimitiveType::Void)),
                    _ => Ok(OakType::Struct(name)),
                }
            }
            _ => Err("Expected type".into()),
        }
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<(String, OakType)>, Box<dyn Error>> {
        let mut params = Vec::new();

        while !self.check(&Token::RightParen) {
            let name = self.expect_identifier()?;
            self.expect(&Token::Colon)?;
            let ty = self.parse_type()?;
            params.push((name, ty));

            if !self.check(&Token::RightParen) {
                self.expect(&Token::Comma)?;
            }
        }

        Ok(params)
    }

    fn parse_type_list(&mut self) -> Result<Vec<OakType>, Box<dyn Error>> {
        let mut types = Vec::new();

        while !self.check(&Token::RightParen) {
            types.push(self.parse_type()?);
            if !self.check(&Token::RightParen) {
                self.expect(&Token::Comma)?;
            }
        }

        Ok(types)
    }

    fn parse_argument_list(&mut self) -> Result<Vec<Expr>, Box<dyn Error>> {
        let mut args = Vec::new();

        while !self.check(&Token::RightParen) {
            args.push(self.parse_expression()?);
            if !self.check(&Token::RightParen) {
                self.expect(&Token::Comma)?;
            }
        }

        Ok(args)
    }

    // Helper methods
    fn check(&self, token: &Token) -> bool {
        if let Some(t) = self.tokens.front() {
            std::mem::discriminant(t) == std::mem::discriminant(token)
        } else {
            false
        }
    }

    fn match_token(&mut self, token: &Token) -> bool {
        if self.check(token) {
            self.tokens.pop_front();
            true
        } else {
            false
        }
    }

    fn match_op(&mut self, op: &str) -> bool {
        if let Some(Token::Operator(s)) = self.tokens.front() {
            if s == op {
                self.tokens.pop_front();
                return true;
            }
        }
        false
    }

    fn match_any_op(&mut self, ops: &[&str]) -> Option<String> {
        for op in ops {
            match self.tokens.front() {
                Some(Token::Plus) if *op == "+" => {
                    self.tokens.pop_front();
                    return Some("+".to_string());
                }
                Some(Token::Minus) if *op == "-" => {
                    self.tokens.pop_front();
                    return Some("-".to_string());
                }
                Some(Token::Star) if *op == "*" => {
                    self.tokens.pop_front();
                    return Some("*".to_string());
                }
                Some(Token::Slash) if *op == "/" => {
                    self.tokens.pop_front();
                    return Some("/".to_string());
                }
                Some(Token::Percent) if *op == "%" => {
                    self.tokens.pop_front();
                    return Some("%".to_string());
                }
                Some(Token::Equals) if *op == "==" => {
                    self.tokens.pop_front();
                    return Some("==".to_string());
                }
                Some(Token::NotEquals) if *op == "!=" => {
                    self.tokens.pop_front();
                    return Some("!=".to_string());
                }
                Some(Token::LessThan) if *op == "<" => {
                    self.tokens.pop_front();
                    return Some("<".to_string());
                }
                Some(Token::GreaterThan) if *op == ">" => {
                    self.tokens.pop_front();
                    return Some(">".to_string());
                }
                Some(Token::LessEquals) if *op == "<=" => {
                    self.tokens.pop_front();
                    return Some("<=".to_string());
                }
                Some(Token::GreaterEquals) if *op == ">=" => {
                    self.tokens.pop_front();
                    return Some(">=".to_string());
                }
                Some(Token::Bang) if *op == "!" => {
                    self.tokens.pop_front();
                    return Some("!".to_string());
                }
                Some(Token::Ampersand) if *op == "&" => {
                    self.tokens.pop_front();
                    return Some("&".to_string());
                }
                Some(Token::Pipe) if *op == "|" => {
                    self.tokens.pop_front();
                    return Some("|".to_string());
                }
                _ => {}
            }
        }
        None
    }

    fn expect(&mut self, token: &Token) -> Result<(), Box<dyn Error>> {
        if self.check(token) {
            self.tokens.pop_front();
            Ok(())
        } else {
            Err(format!("Expected {:?}", token).into())
        }
    }

    fn expect_identifier(&mut self) -> Result<String, Box<dyn Error>> {
        if let Some(Token::Identifier(name)) = self.tokens.front() {
            let name = name.clone();
            self.tokens.pop_front();
            Ok(name)
        } else {
            Err("Expected identifier".into())
        }
    }

    fn expect_keyword(&mut self, keyword: &str) -> Result<(), Box<dyn Error>> {
        if let Some(Token::Identifier(id)) = self.tokens.front() {
            if id == keyword {
                self.tokens.pop_front();
                return Ok(());
            }
        }
        Err(format!("Expected keyword: {}", keyword).into())
    }

    fn consume_semicolon(&mut self) {
        if self.match_token(&Token::Semicolon) {
            // consumed
        }
    }
}

pub fn parse_script(source: String) -> Result<Vec<AstNode>, Box<dyn Error>> {
    use crate::tokenizer::tokenize;

    let mut file = File::open(source)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    let tokens = tokenize(&content);
    let mut parser = Parser::new(tokens);
    parser.parse()
}
