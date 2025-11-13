// Tokenizer
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Var,
    Ret,
    Fn,
    If,
    Else,
    While,
    For,
    Break,
    Continue,
    Struct,
    Enum,
    Extern,
    As,
    Let,
    Mut,
    
    // Identifiers and literals
    Identifier(String),
    Number(f64),
    StringLiteral(String),
    
    // Operators
    Assign,      // :=
    Operator(String),
    Equals,      // ==
    NotEquals,   // !=
    LessThan,    // <
    GreaterThan, // >
    LessEquals,  // <=
    GreaterEquals, // >=
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    Percent,     // %
    Caret,       // ^
    Ampersand,   // &
    Pipe,        // |
    Bang,        // !
    
    // Delimiters
    LeftParen,   // (
    RightParen,  // )
    LeftBrace,   // {
    RightBrace,  // }
    LeftBracket, // [
    RightBracket, // ]
    Comma,       // ,
    Dot,         // .
    Colon,       // :
    Semicolon,   // ;
    Arrow,       // ->
    DoubleColon, // ::
    
    // Blocks
    BeginProj(String),
    EndProj(String),
    BeginSection(String),
    EndSection(String),
    
    // Comments and special
    Comment(String),
    Unknown(String),
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    // Manual tokenization without regex (finite state machine) approach
    let chars: Vec<char> = source.chars().collect();
    let mut pos = 0;

    while pos < chars.len() {
        let c = chars[pos];

        match c {
            // If the character is whitespace it continues until the
            // tokenizer encounters a character to match
            c if c.is_whitespace() => pos += 1,
            // Handle # single line comments
            '#' => {
                pos += 1;
                let start = pos;
                while pos < chars.len() && chars[pos] != '\n' {
                    pos += 1;
                }
                let comment: String = chars[start..pos].iter().collect();
                tokens.push(Token::Comment(comment.trim().to_string()));
            }
            // Double colon ::
            ':' if pos + 1 < chars.len() && chars[pos + 1] == ':' => {
                tokens.push(Token::DoubleColon);
                pos += 2;
            }
            // Assignment operator :=
            ':' if pos + 1 < chars.len() && chars[pos + 1] == '=' => {
                tokens.push(Token::Assign);
                pos += 2;
            }
            // Arrow ->
            '-' if pos + 1 < chars.len() && chars[pos + 1] == '>' => {
                tokens.push(Token::Arrow);
                pos += 2;
            }
            // Less than or equal <=
            '<' if pos + 1 < chars.len() && chars[pos + 1] == '=' => {
                tokens.push(Token::LessEquals);
                pos += 2;
            }
            // Greater than or equal >=
            '>' if pos + 1 < chars.len() && chars[pos + 1] == '=' => {
                tokens.push(Token::GreaterEquals);
                pos += 2;
            }
            // Equality ==
            '=' if pos + 1 < chars.len() && chars[pos + 1] == '=' => {
                tokens.push(Token::Equals);
                pos += 2;
            }
            // Not equal !=
            '!' if pos + 1 < chars.len() && chars[pos + 1] == '=' => {
                tokens.push(Token::NotEquals);
                pos += 2;
            }
            // Single character operators and delimiters
            '+' => {
                tokens.push(Token::Plus);
                pos += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                pos += 1;
            }
            '*' => {
                tokens.push(Token::Star);
                pos += 1;
            }
            '/' => {
                tokens.push(Token::Slash);
                pos += 1;
            }
            '%' => {
                tokens.push(Token::Percent);
                pos += 1;
            }
            '^' => {
                tokens.push(Token::Caret);
                pos += 1;
            }
            '&' => {
                tokens.push(Token::Ampersand);
                pos += 1;
            }
            '|' => {
                tokens.push(Token::Pipe);
                pos += 1;
            }
            '!' => {
                tokens.push(Token::Bang);
                pos += 1;
            }
            '<' => {
                tokens.push(Token::LessThan);
                pos += 1;
            }
            '>' => {
                tokens.push(Token::GreaterThan);
                pos += 1;
            }
            ':' => {
                tokens.push(Token::Colon);
                pos += 1;
            }
            '(' => {
                tokens.push(Token::LeftParen);
                pos += 1;
            }
            ')' => {
                tokens.push(Token::RightParen);
                pos += 1;
            }
            '{' => {
                tokens.push(Token::LeftBrace);
                pos += 1;
            }
            '}' => {
                tokens.push(Token::RightBrace);
                pos += 1;
            }
            '[' => {
                tokens.push(Token::LeftBracket);
                pos += 1;
            }
            ']' => {
                tokens.push(Token::RightBracket);
                pos += 1;
            }
            ',' => {
                tokens.push(Token::Comma);
                pos += 1;
            }
            '.' => {
                tokens.push(Token::Dot);
                pos += 1;
            }
            ';' => {
                tokens.push(Token::Semicolon);
                pos += 1;
            }
            '"' => {
                pos += 1;
                let start = pos;
                while pos < chars.len() && chars[pos] != '"' {
                    pos += 1;
                }
                let literal: String = chars[start..pos].iter().collect();
                tokens.push(Token::StringLiteral(literal));
                pos += 1; // consumes closing quote
            }
            // Analyses if the current token is an ascii_digit and parses it as a Number token
            // In future releases, the language will implement different types of numerical values
            // and different types of operations depending on the type of numerical value given to the interpreter/compiler
            c if c.is_ascii_digit() => {
                let start = pos;
                while pos < chars.len() && (chars[pos].is_ascii_digit() || chars[pos] == '.') {
                    pos += 1;
                }
                let number_str: String = chars[start..pos].iter().collect();
                if let Ok(num) = number_str.parse::<f64>() {
                    tokens.push(Token::Number(num));
                } else {
                    tokens.push(Token::Unknown(number_str));
                }
            }
            // Gives names to variables (identifiers)
            c if c.is_ascii_alphabetic() || c == '_' => {
                let start = pos;
                while pos < chars.len() && (chars[pos].is_ascii_alphanumeric() || chars[pos] == '_')
                {
                    pos += 1;
                }
                let ident: String = chars[start..pos].iter().collect();
                match ident.as_str() {
                    "var" => tokens.push(Token::Var),
                    "ret" => tokens.push(Token::Ret),
                    "fn" => tokens.push(Token::Fn),
                    "if" => tokens.push(Token::If),
                    "else" => tokens.push(Token::Else),
                    "while" => tokens.push(Token::While),
                    "for" => tokens.push(Token::For),
                    "break" => tokens.push(Token::Break),
                    "continue" => tokens.push(Token::Continue),
                    "struct" => tokens.push(Token::Struct),
                    "enum" => tokens.push(Token::Enum),
                    "extern" => tokens.push(Token::Extern),
                    "as" => tokens.push(Token::As),
                    "let" => tokens.push(Token::Let),
                    "mut" => tokens.push(Token::Mut),
                    "BEGIN" => {
                        // Look ahead for PROJ or SECTION
                        let mut next_start = pos;
                        while next_start < chars.len() && chars[next_start].is_whitespace() {
                            next_start += 1;
                        }
                        if next_start < chars.len() && chars[next_start].is_ascii_alphabetic() {
                            let mut next_end = next_start;
                            while next_end < chars.len() && (chars[next_end].is_ascii_alphanumeric() || chars[next_end] == '_') {
                                next_end += 1;
                            }
                            let next_ident: String = chars[next_start..next_end].iter().collect();
                            match next_ident.as_str() {
                                "PROJ" => {
                                    // Skip to the string literal
                                    pos = next_end;
                                    while pos < chars.len() && chars[pos].is_whitespace() {
                                        pos += 1;
                                    }
                                    if pos < chars.len() && chars[pos] == '"' {
                                        pos += 1;
                                        let start = pos;
                                        while pos < chars.len() && chars[pos] != '"' {
                                            pos += 1;
                                        }
                                        let proj_name: String = chars[start..pos].iter().collect();
                                        tokens.push(Token::BeginProj(proj_name));
                                        pos += 1; // consume closing quote
                                    }
                                }
                                "SECTION" => {
                                    // Skip to the string literal
                                    pos = next_end;
                                    while pos < chars.len() && chars[pos].is_whitespace() {
                                        pos += 1;
                                    }
                                    if pos < chars.len() && chars[pos] == '"' {
                                        pos += 1;
                                        let start = pos;
                                        while pos < chars.len() && chars[pos] != '"' {
                                            pos += 1;
                                        }
                                        let section_name: String = chars[start..pos].iter().collect();
                                        tokens.push(Token::BeginSection(section_name));
                                        pos += 1; // consume closing quote
                                    }
                                }
                                _ => tokens.push(Token::Identifier(ident)),
                            }
                        } else {
                            tokens.push(Token::Identifier(ident));
                        }
                    }
                    "END" => {
                        // Look ahead for PROJ or SECTION
                        let mut next_start = pos;
                        while next_start < chars.len() && chars[next_start].is_whitespace() {
                            next_start += 1;
                        }
                        if next_start < chars.len() && chars[next_start].is_ascii_alphabetic() {
                            let mut next_end = next_start;
                            while next_end < chars.len() && (chars[next_end].is_ascii_alphanumeric() || chars[next_end] == '_') {
                                next_end += 1;
                            }
                            let next_ident: String = chars[next_start..next_end].iter().collect();
                            match next_ident.as_str() {
                                "PROJ" => {
                                    // Skip to the string literal
                                    pos = next_end;
                                    while pos < chars.len() && chars[pos].is_whitespace() {
                                        pos += 1;
                                    }
                                    if pos < chars.len() && chars[pos] == '"' {
                                        pos += 1;
                                        let start = pos;
                                        while pos < chars.len() && chars[pos] != '"' {
                                            pos += 1;
                                        }
                                        let proj_name: String = chars[start..pos].iter().collect();
                                        tokens.push(Token::EndProj(proj_name));
                                        pos += 1; // consume closing quote
                                    }
                                }
                                "SECTION" => {
                                    // Skip to the string literal
                                    pos = next_end;
                                    while pos < chars.len() && chars[pos].is_whitespace() {
                                        pos += 1;
                                    }
                                    if pos < chars.len() && chars[pos] == '"' {
                                        pos += 1;
                                        let start = pos;
                                        while pos < chars.len() && chars[pos] != '"' {
                                            pos += 1;
                                        }
                                        let section_name: String = chars[start..pos].iter().collect();
                                        tokens.push(Token::EndSection(section_name));
                                        pos += 1; // consume closing quote
                                    }
                                }
                                _ => tokens.push(Token::Identifier(ident)),
                            }
                        } else {
                            tokens.push(Token::Identifier(ident));
                        }
                    }
                    _ => tokens.push(Token::Identifier(ident)),
                }
            }
            // ALL other type of tokens aren't parsed and identified as "unknown"
            _ => {
                tokens.push(Token::Unknown(c.to_string()));
                pos += 1;
            }
        }
    }

    tokens
}
