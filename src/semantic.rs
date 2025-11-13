// Semantic Analysis Module
use crate::types::{OakType, PrimitiveType, TypeContext};
use crate::errors::{OakError, SourceLocation, Result};
use crate::parser::{AstNode, Expr, Statement};
use std::collections::HashMap;

pub struct SemanticAnalyzer {
    type_context: TypeContext,
    current_file: String,
    current_line: usize,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        Self {
            type_context: TypeContext::new(),
            current_file: "unknown".to_string(),
            current_line: 1,
        }
    }
    
    pub fn analyze(&mut self, nodes: &[AstNode]) -> Result<()> {
        for node in nodes {
            self.analyze_node(node)?;
        }
        Ok(())
    }
    
    pub fn analyze_node(&mut self, node: &AstNode) -> Result<()> {
        match node {
            AstNode::VarDeclaration { name, ty, value } => {
                // Infer or check type
                if let Some(init_value) = value {
                    let inferred_type = self.infer_type(init_value)?;
                    if let OakType::Inferred = ty {
                        self.type_context.declare_variable(name.clone(), inferred_type);
                    } else {
                        // Check type compatibility
                        if !self.types_compatible(ty, &inferred_type) {
                            return Err(OakError::TypeError {
                                message: "Type mismatch in variable declaration".to_string(),
                                expected: ty.to_string(),
                                found: inferred_type.to_string(),
                                location: self.current_location(),
                            });
                        }
                        self.type_context.declare_variable(name.clone(), ty.clone());
                    }
                } else {
                    self.type_context.declare_variable(name.clone(), ty.clone());
                }
                Ok(())
            }
            AstNode::FunctionDeclaration { name, params, return_type, body } => {
                let param_types: Vec<OakType> = params.iter().map(|(_, ty)| ty.clone()).collect();
                self.type_context.declare_function(name.clone(), param_types, return_type.clone());
                
                // Analyze function body
                for stmt in body {
                    self.analyze_node(stmt)?;
                }
                Ok(())
            }
            AstNode::StructDeclaration { name, fields } => {
                self.type_context.declare_struct(name.clone(), fields.clone());
                Ok(())
            }
            AstNode::Statement(stmt) => {
                self.analyze_statement(stmt)
            }
            _ => Ok(()),
        }
    }
    
    pub fn analyze_statement(&mut self, stmt: &Statement) -> Result<()> {
        match stmt {
            Statement::Block(statements) => {
                for s in statements {
                    self.analyze_statement(s)?;
                }
                Ok(())
            }
            Statement::IfElse { condition, then_block, else_block } => {
                let cond_type = self.infer_expr_type(condition)?;
                if !matches!(cond_type, OakType::Primitive(PrimitiveType::Bool)) {
                    return Err(OakError::TypeError {
                        message: "If condition must be boolean".to_string(),
                        expected: "bool".to_string(),
                        found: cond_type.to_string(),
                        location: self.current_location(),
                    });
                }
                self.analyze_statement(then_block)?;
                if let Some(else_b) = else_block {
                    self.analyze_statement(else_b)?;
                }
                Ok(())
            }
            Statement::While { condition, body } => {
                let cond_type = self.infer_expr_type(condition)?;
                if !matches!(cond_type, OakType::Primitive(PrimitiveType::Bool)) {
                    return Err(OakError::TypeError {
                        message: "While condition must be boolean".to_string(),
                        expected: "bool".to_string(),
                        found: cond_type.to_string(),
                        location: self.current_location(),
                    });
                }
                self.analyze_statement(body)?;
                Ok(())
            }
            Statement::For { var, iter, body } => {
                let iter_type = self.infer_expr_type(iter)?;
                // Check if iterable
                match &iter_type {
                    OakType::Array(elem_type, _) => {
                        self.type_context.declare_variable(var.clone(), (**elem_type).clone());
                    }
                    _ => {
                        return Err(OakError::TypeError {
                            message: "For loop requires iterable".to_string(),
                            expected: "array or iterable".to_string(),
                            found: iter_type.to_string(),
                            location: self.current_location(),
                        });
                    }
                }
                self.analyze_statement(body)?;
                Ok(())
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    let _ = self.infer_expr_type(e)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
    
    pub fn infer_expr_type(&self, expr: &Expr) -> Result<OakType> {
        self.infer_type(expr)
    }
    
    pub fn infer_type(&self, expr: &Expr) -> Result<OakType> {
        match expr {
            Expr::Number(_) => Ok(OakType::Primitive(PrimitiveType::Float64)),
            Expr::String(_) => Ok(OakType::Primitive(PrimitiveType::Text)),
            Expr::Bool(_) => Ok(OakType::Primitive(PrimitiveType::Bool)),
            Expr::Identifier(name) => {
                self.type_context.get_variable(name)
                    .cloned()
                    .ok_or_else(|| OakError::SemanticError {
                        message: format!("Unknown variable: {}", name),
                        location: self.current_location(),
                    })
            }
            Expr::BinaryOp { left, op, right } => {
                let left_type = self.infer_type(left)?;
                let right_type = self.infer_type(right)?;
                
                if !self.types_compatible(&left_type, &right_type) {
                    return Err(OakError::TypeError {
                        message: format!("Incompatible types in binary operation: {}", op),
                        expected: left_type.to_string(),
                        found: right_type.to_string(),
                        location: self.current_location(),
                    });
                }
                
                match op.as_str() {
                    "==" | "!=" | "<" | ">" | "<=" | ">=" => Ok(OakType::Primitive(PrimitiveType::Bool)),
                    _ => Ok(left_type),
                }
            }
            Expr::UnaryOp { op, operand } => {
                let operand_type = self.infer_type(operand)?;
                match op.as_str() {
                    "!" => Ok(OakType::Primitive(PrimitiveType::Bool)),
                    "-" | "+" => {
                        if operand_type.is_numeric() {
                            Ok(operand_type)
                        } else {
                            Err(OakError::TypeError {
                                message: format!("Cannot apply {} to non-numeric type", op),
                                expected: "numeric type".to_string(),
                                found: operand_type.to_string(),
                                location: self.current_location(),
                            })
                        }
                    }
                    "*" => Ok(OakType::Pointer(Box::new(operand_type))),
                    "&" => Ok(OakType::Reference(Box::new(operand_type))),
                    _ => Err(OakError::SemanticError {
                        message: format!("Unknown unary operator: {}", op),
                        location: self.current_location(),
                    }),
                }
            }
            Expr::Cast { expr, target_type } => {
                let expr_type = self.infer_type(expr)?;
                if !expr_type.can_cast_to(target_type) {
                    return Err(OakError::TypeError {
                        message: "Invalid type cast".to_string(),
                        expected: target_type.to_string(),
                        found: expr_type.to_string(),
                        location: self.current_location(),
                    });
                }
                Ok(target_type.clone())
            }
            Expr::Array(elements) => {
                if elements.is_empty() {
                    Ok(OakType::Array(Box::new(OakType::Inferred), Some(0)))
                } else {
                    let first_type = self.infer_type(&elements[0])?;
                    for elem in elements {
                        let elem_type = self.infer_type(elem)?;
                        if !self.types_compatible(&first_type, &elem_type) {
                            return Err(OakError::TypeError {
                                message: "Array elements must have same type".to_string(),
                                expected: first_type.to_string(),
                                found: elem_type.to_string(),
                                location: self.current_location(),
                            });
                        }
                    }
                    Ok(OakType::Array(Box::new(first_type), Some(elements.len())))
                }
            }
            Expr::Index { array, index } => {
                let array_type = self.infer_type(array)?;
                let index_type = self.infer_type(index)?;
                
                if !index_type.is_integer() {
                    return Err(OakError::TypeError {
                        message: "Array index must be integer".to_string(),
                        expected: "integer".to_string(),
                        found: index_type.to_string(),
                        location: self.current_location(),
                    });
                }
                
                match array_type {
                    OakType::Array(elem_type, _) => Ok(*elem_type),
                    _ => Err(OakError::TypeError {
                        message: "Cannot index non-array type".to_string(),
                        expected: "array".to_string(),
                        found: array_type.to_string(),
                        location: self.current_location(),
                    }),
                }
            }
            Expr::FunctionCall { name, args } => {
                if let Some((params, return_type)) = self.type_context.get_function(name) {
                    if args.len() != params.len() {
                        return Err(OakError::SemanticError {
                            message: format!(
                                "Function {} expects {} arguments, got {}",
                                name,
                                params.len(),
                                args.len()
                            ),
                            location: self.current_location(),
                        });
                    }
                    for (arg, param_type) in args.iter().zip(params.iter()) {
                        let arg_type = self.infer_type(arg)?;
                        if !self.types_compatible(&arg_type, param_type) {
                            return Err(OakError::TypeError {
                                message: "Argument type mismatch".to_string(),
                                expected: param_type.to_string(),
                                found: arg_type.to_string(),
                                location: self.current_location(),
                            });
                        }
                    }
                    Ok(return_type.clone())
                } else {
                    Err(OakError::SemanticError {
                        message: format!("Unknown function: {}", name),
                        location: self.current_location(),
                    })
                }
            }
        }
    }
    
    fn types_compatible(&self, t1: &OakType, t2: &OakType) -> bool {
        if t1 == t2 {
            return true;
        }
        
        // Allow numeric type compatibility
        if t1.is_numeric() && t2.is_numeric() {
            return true;
        }
        
        false
    }
    
    fn current_location(&self) -> SourceLocation {
        SourceLocation {
            line: self.current_line,
            column: 0,
            file: self.current_file.clone(),
        }
    }
}
