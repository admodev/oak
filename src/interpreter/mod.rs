// Interpreter (Legacy - using new parser, this is minimally stub)
// The new MVP uses semantic analysis and code generation instead

use std::collections::HashMap;

pub struct Interpreter {
    variables: HashMap<String, f64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }
}
