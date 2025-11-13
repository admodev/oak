// REPL (Read-Eval-Print Loop)

use std::io::{self, Write};
use crate::tokenizer::tokenize;
use crate::parser::Parser;

pub fn start_repl() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();

    println!("Oak REPL v0.1.0");
    println!("Type 'exit' to quit, 'help' for commands\n");

    loop {
        input.clear();
        print!("oak> ");
        let _ = stdout.flush();
        
        match stdin.read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim();
                
                if trimmed == "exit" {
                    println!("Goodbye!");
                    std::process::exit(0);
                }
                
                if trimmed == "help" {
                    print_help();
                    continue;
                }
                
                if trimmed.is_empty() {
                    continue;
                }
                
                // Try to tokenize and parse
                match evaluate_line(trimmed) {
                    Ok(output) => println!("{}", output),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Err(error) => eprintln!("Error reading input: {}", error),
        }
    }
}

fn evaluate_line(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let tokens = tokenize(input);
    
    // Skip comments
    let tokens: Vec<_> = tokens.into_iter()
        .filter(|t| !matches!(t, crate::tokenizer::Token::Comment(_)))
        .collect();
    
    if tokens.is_empty() {
        return Ok(String::new());
    }
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    
    Ok(format!("Parsed {} statements successfully", ast.len()))
}

fn print_help() {
    println!("Oak REPL Commands:");
    println!("  exit                - Exit the REPL");
    println!("  help                - Show this help message");
    println!("  let <var>: <type>   - Declare variable");
    println!("  fn <name>() -> type - Declare function");
    println!("  extern fn <name>()  - Declare extern C function");
    println!();
}
