// Script Runner
use crate::parser::parse_script;
use crate::interpreter::Interpreter;
use std::error::Error;

pub fn run(source: String) -> Result<(), Box<dyn Error>> {
    println!("Running script with Oak version 0.1.0...");

    let ast = parse_script(source)?;
    let mut interpreter = Interpreter::new();

    // Walk the AST with the interpreter (all nodes)
    for node in ast {
        node.accept(&mut interpreter);
    }

    Ok(())
}
