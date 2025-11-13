// LLVM Code Generation Module
use crate::types::{OakType, PrimitiveType};
use crate::errors::{OakError, Result};
use std::collections::HashMap;

/// Code generator for LLVM IR
pub struct CodeGenerator {
    ir_code: Vec<String>,
    var_counter: usize,
    type_map: HashMap<String, String>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        let mut type_map = HashMap::new();
        
        // Map Oak types to LLVM types
        type_map.insert("u8".to_string(), "i8".to_string());
        type_map.insert("u16".to_string(), "i16".to_string());
        type_map.insert("u32".to_string(), "i32".to_string());
        type_map.insert("u64".to_string(), "i64".to_string());
        type_map.insert("i8".to_string(), "i8".to_string());
        type_map.insert("i16".to_string(), "i16".to_string());
        type_map.insert("i32".to_string(), "i32".to_string());
        type_map.insert("i64".to_string(), "i64".to_string());
        type_map.insert("posint8".to_string(), "i8".to_string());
        type_map.insert("posint16".to_string(), "i16".to_string());
        type_map.insert("posint32".to_string(), "i32".to_string());
        type_map.insert("posint64".to_string(), "i64".to_string());
        type_map.insert("f32".to_string(), "float".to_string());
        type_map.insert("f64".to_string(), "double".to_string());
        type_map.insert("bool".to_string(), "i1".to_string());
        type_map.insert("str".to_string(), "i8*".to_string());
        type_map.insert("Text".to_string(), "i8*".to_string());
        
        Self {
            ir_code: Vec::new(),
            var_counter: 0,
            type_map,
        }
    }
    
    pub fn oak_type_to_llvm(&self, oak_type: &OakType) -> String {
        match oak_type {
            OakType::Primitive(prim) => self.primitive_to_llvm(prim),
            OakType::Pointer(inner) => {
                format!("{}*", self.oak_type_to_llvm(inner))
            }
            OakType::Reference(inner) => {
                format!("{}*", self.oak_type_to_llvm(inner))
            }
            OakType::Array(elem_type, Some(size)) => {
                format!("[{} x {}]", size, self.oak_type_to_llvm(elem_type))
            }
            OakType::Array(elem_type, None) => {
                format!("{}*", self.oak_type_to_llvm(elem_type))
            }
            OakType::Function { params, return_type } => {
                let param_str = params
                    .iter()
                    .map(|p| self.oak_type_to_llvm(p))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{} ({}, ...)", self.oak_type_to_llvm(return_type), param_str)
            }
            _ => "i32".to_string(), // default
        }
    }
    
    pub fn primitive_to_llvm(&self, prim: &PrimitiveType) -> String {
        self.type_map
            .get(&prim.to_string())
            .cloned()
            .unwrap_or_else(|| "i32".to_string())
    }
    
    pub fn emit(&mut self, code: String) {
        self.ir_code.push(code);
    }
    
    pub fn emit_function_declaration(
        &mut self,
        name: &str,
        params: &[(String, OakType)],
        return_type: &OakType,
    ) {
        let param_str = params
            .iter()
            .map(|(_, ty)| self.oak_type_to_llvm(ty))
            .collect::<Vec<_>>()
            .join(", ");
        
        let return_ty = self.oak_type_to_llvm(return_type);
        self.emit(format!("declare {} @{}({})", return_ty, name, param_str));
    }
    
    pub fn emit_function_definition(
        &mut self,
        name: &str,
        params: &[(String, OakType)],
        return_type: &OakType,
    ) {
        let param_str = params
            .iter()
            .map(|(pname, ty)| format!("{} %{}", self.oak_type_to_llvm(ty), pname))
            .collect::<Vec<_>>()
            .join(", ");
        
        let return_ty = self.oak_type_to_llvm(return_type);
        self.emit(format!("define {} @{}({}) {{", return_ty, name, param_str));
    }
    
    pub fn emit_function_end(&mut self) {
        self.emit("}".to_string());
    }
    
    pub fn emit_extern_function(&mut self, name: &str, params: &[OakType], return_type: &OakType) {
        let param_str = params
            .iter()
            .map(|ty| self.oak_type_to_llvm(ty))
            .collect::<Vec<_>>()
            .join(", ");
        
        let return_ty = self.oak_type_to_llvm(return_type);
        self.emit(format!("declare {} @{}({})", return_ty, name, param_str));
    }
    
    pub fn emit_variable_declaration(&mut self, name: &str, ty: &OakType) -> String {
        let var_id = format!("%{}", name);
        let llvm_type = self.oak_type_to_llvm(ty);
        self.emit(format!("{} = alloca {}", var_id, llvm_type));
        var_id
    }
    
    pub fn emit_binary_op(&mut self, op: &str, left: &str, right: &str, left_type: &OakType) -> String {
        let result_var = format!("%tmp.{}", self.var_counter);
        self.var_counter += 1;
        
        let llvm_op = match op {
            "+" => {
                if left_type.is_float() {
                    "fadd"
                } else {
                    "add"
                }
            }
            "-" => {
                if left_type.is_float() {
                    "fsub"
                } else {
                    "sub"
                }
            }
            "*" => {
                if left_type.is_float() {
                    "fmul"
                } else {
                    "mul"
                }
            }
            "/" => {
                if left_type.is_float() {
                    "fdiv"
                } else {
                    "sdiv"
                }
            }
            "%" => "srem",
            "==" => "eq",
            "!=" => "ne",
            "<" => "slt",
            ">" => "sgt",
            "<=" => "sle",
            ">=" => "sge",
            _ => "add", // default
        };
        
        let llvm_type = self.oak_type_to_llvm(left_type);
        
        if matches!(op, "==" | "!=" | "<" | ">" | "<=" | ">=") {
            self.emit(format!(
                "{} = icmp {} {} {}, {}",
                result_var, llvm_op, llvm_type, left, right
            ));
        } else {
            self.emit(format!(
                "{} = {} {} {}, {}",
                result_var, llvm_op, llvm_type, left, right
            ));
        }
        
        result_var
    }
    
    pub fn emit_array_access(&mut self, array: &str, index: &str, elem_type: &OakType) -> String {
        let result_var = format!("%tmp.{}", self.var_counter);
        self.var_counter += 1;
        
        let llvm_type = self.oak_type_to_llvm(elem_type);
        self.emit(format!(
            "{} = getelementptr {}, {}* {}, i64 {}",
            result_var, elem_type, llvm_type, array, index
        ));
        
        result_var
    }
    
    pub fn emit_call(&mut self, function: &str, args: &[String], return_type: &OakType) -> String {
        let result_var = format!("%tmp.{}", self.var_counter);
        self.var_counter += 1;
        
        let args_str = args.join(", ");
        let return_ty = self.oak_type_to_llvm(return_type);
        
        self.emit(format!(
            "{} = call {} @{}({})",
            result_var, return_ty, function, args_str
        ));
        
        result_var
    }
    
    pub fn emit_return(&mut self, value: Option<&str>, return_type: &OakType) {
        let llvm_type = self.oak_type_to_llvm(return_type);
        if let Some(val) = value {
            self.emit(format!("ret {} {}", llvm_type, val));
        } else {
            self.emit("ret void".to_string());
        }
    }
    
    pub fn emit_module_prologue(&mut self) {
        self.emit("; ModuleID = 'oak'".to_string());
        self.emit("source_filename = \"oak\"".to_string());
        self.emit("target triple = \"x86_64-pc-linux-gnu\"".to_string());
        self.emit("target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"".to_string());
    }
    
    pub fn get_ir(&self) -> String {
        self.ir_code.join("\n")
    }
    
    pub fn generate_llvm_ir(&mut self) -> Result<String> {
        self.emit_module_prologue();
        Ok(self.get_ir())
    }
}
