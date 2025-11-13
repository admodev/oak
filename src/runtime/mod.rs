// Script Runner
use crate::codegen::CodeGenerator;
use crate::parser::parse_script;
use crate::semantic::SemanticAnalyzer;
use std::error::Error;

pub fn run(source: String) -> Result<(), Box<dyn Error>> {
    println!("Running script with Oak version 0.1.0...");

    // Parse
    let ast = parse_script(source)?;

    // Semantic Analysis
    let mut analyzer = SemanticAnalyzer::new();
    analyzer.analyze(&ast)?;

    // Code Generation (LLVM IR)
    let mut codegen = CodeGenerator::new();
    let _ir = codegen.generate_llvm_ir()?;

    println!("Generated LLVM IR successfully");

    Ok(())
}
