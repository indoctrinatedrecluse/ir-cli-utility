use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    LParen,
    RParen,
    Assign,
    EOF,
}

fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = expr.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c.is_ascii_digit() || c == '.' {
            let mut num_str = String::new();
            let mut has_dot = false;
            while let Some(&nc) = chars.peek() {
                if nc.is_ascii_digit() {
                    num_str.push(nc);
                    chars.next();
                } else if nc == '.' {
                    if has_dot {
                        return Err("Invalid number format (multiple decimal points)".to_string());
                    }
                    has_dot = true;
                    num_str.push(nc);
                    chars.next();
                } else {
                    break;
                }
            }
            let val = num_str.parse::<f64>().map_err(|e| format!("Failed to parse number '{}': {}", num_str, e))?;
            tokens.push(Token::Number(val));
        } else if c.is_alphabetic() || c == '_' {
            let mut ident = String::new();
            while let Some(&nc) = chars.peek() {
                if nc.is_alphanumeric() || nc == '_' {
                    ident.push(nc);
                    chars.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Identifier(ident));
        } else {
            match c {
                '+' => { tokens.push(Token::Plus); chars.next(); }
                '-' => { tokens.push(Token::Minus); chars.next(); }
                '*' => { tokens.push(Token::Multiply); chars.next(); }
                '/' => { tokens.push(Token::Divide); chars.next(); }
                '%' => { tokens.push(Token::Modulo); chars.next(); }
                '^' => { tokens.push(Token::Power); chars.next(); }
                '(' => { tokens.push(Token::LParen); chars.next(); }
                ')' => { tokens.push(Token::RParen); chars.next(); }
                '=' => { tokens.push(Token::Assign); chars.next(); }
                _ => return Err(format!("Unexpected character '{}' in expression", c)),
            }
        }
    }
    tokens.push(Token::EOF);
    Ok(tokens)
}

struct Parser<'a> {
    tokens: Vec<Token>,
    pos: usize,
    variables: &'a mut HashMap<String, f64>,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token>, variables: &'a mut HashMap<String, f64>) -> Self {
        Parser { tokens, pos: 0, variables }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn consume(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        if t != Token::EOF {
            self.pos += 1;
        }
        t
    }

    fn statement(&mut self) -> Result<f64, String> {
        if let Token::Identifier(name) = self.peek().clone() {
            if self.tokens.get(self.pos + 1) == Some(&Token::Assign) {
                self.consume(); // consume identifier
                self.consume(); // consume '='
                let val = self.expr()?;
                self.variables.insert(name.to_lowercase(), val);
                return Ok(val);
            }
        }
        self.expr()
    }

    fn expr(&mut self) -> Result<f64, String> {
        let mut val = self.term()?;
        loop {
            match self.peek() {
                Token::Plus => {
                    self.consume();
                    val += self.term()?;
                }
                Token::Minus => {
                    self.consume();
                    val -= self.term()?;
                }
                _ => break,
            }
        }
        Ok(val)
    }

    fn term(&mut self) -> Result<f64, String> {
        let mut val = self.factor()?;
        loop {
            match self.peek() {
                Token::Multiply => {
                    self.consume();
                    val *= self.factor()?;
                }
                Token::Divide => {
                    self.consume();
                    let divisor = self.factor()?;
                    if divisor == 0.0 {
                        return Err("Division by zero".to_string());
                    }
                    val /= divisor;
                }
                Token::Modulo => {
                    self.consume();
                    let divisor = self.factor()?;
                    if divisor == 0.0 {
                        return Err("Modulo by zero".to_string());
                    }
                    val %= divisor;
                }
                _ => break,
            }
        }
        Ok(val)
    }

    fn factor(&mut self) -> Result<f64, String> {
        let val = self.primary()?;
        if *self.peek() == Token::Power {
            self.consume();
            let exponent = self.factor()?;
            Ok(val.powf(exponent))
        } else {
            Ok(val)
        }
    }

    fn primary(&mut self) -> Result<f64, String> {
        match self.peek().clone() {
            Token::Number(val) => {
                self.consume();
                Ok(val)
            }
            Token::Identifier(name) => {
                self.consume();
                // Check if followed by '(' -> function call
                if *self.peek() == Token::LParen {
                    self.consume(); // consume '('
                    let arg = self.expr()?;
                    if self.consume() != Token::RParen {
                        return Err(format!("Missing matching closing parenthesis ')' for function '{}'", name));
                    }
                    let name_lower = name.to_lowercase();
                    match name_lower.as_str() {
                        "sin" => Ok(arg.sin()),
                        "cos" => Ok(arg.cos()),
                        "tan" => Ok(arg.tan()),
                        "log" => Ok(arg.log10()),
                        "ln" => Ok(arg.ln()),
                        "sqrt" => {
                            if arg < 0.0 {
                                return Err("Cannot compute square root of a negative number".to_string());
                            }
                            Ok(arg.sqrt())
                        }
                        "abs" => Ok(arg.abs()),
                        "round" => Ok(arg.round()),
                        _ => Err(format!("Unknown function '{}'", name)),
                    }
                } else {
                    // Variable lookup
                    let name_lower = name.to_lowercase();
                    if name_lower == "pi" {
                        Ok(std::f64::consts::PI)
                    } else if name_lower == "e" {
                        Ok(std::f64::consts::E)
                    } else if let Some(&val) = self.variables.get(&name_lower) {
                        Ok(val)
                    } else {
                        Err(format!("Undefined variable '{}'", name))
                    }
                }
            }
            Token::LParen => {
                self.consume();
                let val = self.expr()?;
                if self.consume() != Token::RParen {
                    return Err("Missing matching closing parenthesis ')'".to_string());
                }
                Ok(val)
            }
            Token::Minus => {
                self.consume();
                let val = self.primary()?;
                Ok(-val)
            }
            Token::Plus => {
                self.consume();
                let val = self.primary()?;
                Ok(val)
            }
            _ => Err(format!("Unexpected token: {:?}", self.peek())),
        }
    }
}

pub fn evaluate(expr_opt: Option<&str>) {
    let mut variables = HashMap::new();

    if let Some(expr) = expr_opt {
        // Evaluate single expression and print
        match eval_str(expr, &mut variables) {
            Ok(result) => {
                print_result(result);
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Run REPL mode
        run_repl(&mut variables);
    }
}

fn eval_str(expr: &str, variables: &mut HashMap<String, f64>) -> Result<f64, String> {
    let tokens = tokenize(expr)?;
    let mut parser = Parser::new(tokens, variables);
    let result = parser.statement()?;
    if *parser.peek() != Token::EOF {
        return Err("Invalid expression (unexpected trailing characters).".to_string());
    }
    Ok(result)
}

fn print_result(result: f64) {
    if result.is_nan() {
        println!("NaN");
    } else if result.is_infinite() {
        if result.is_sign_positive() {
            println!("Infinity");
        } else {
            println!("-Infinity");
        }
    } else if result.fract() == 0.0 {
        println!("{}", result as i64);
    } else {
        println!("{}", result);
    }
}

fn run_repl(variables: &mut HashMap<String, f64>) {
    println!("\x1B[1;36mir-math Calculator REPL\x1B[0m");
    println!("Type mathematical expressions to evaluate.");
    println!("Assign variables like: x = 10 * sin(pi/4)");
    println!("Commands: 'vars' to list variables, 'clear' to reset, 'exit'/'quit' to exit.");
    println!();

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("math> ");
        let _ = io::stdout().flush();
        input.clear();
        match stdin.read_line(&mut input) {
            Ok(n) => {
                if n == 0 {
                    break; // EOF
                }
            }
            Err(_) => break,
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        let trimmed_lower = trimmed.to_lowercase();
        if trimmed_lower == "exit" || trimmed_lower == "quit" {
            break;
        }
        if trimmed_lower == "vars" {
            if variables.is_empty() {
                println!("No variables defined. Standard constants: pi, e");
            } else {
                for (name, val) in variables.iter() {
                    println!("  {} = {}", name, val);
                }
            }
            continue;
        }
        if trimmed_lower == "clear" {
            variables.clear();
            println!("Variables cleared.");
            continue;
        }

        match eval_str(trimmed, variables) {
            Ok(res) => {
                // If it was an assignment, echo the assignment value
                if trimmed.contains('=') {
                    // Find assigned name
                    if let Some(pos) = trimmed.find('=') {
                        let var_name = trimmed[..pos].trim();
                        println!("{} = {}", var_name, res);
                    }
                } else {
                    print_result(res);
                }
            }
            Err(e) => {
                println!("\x1B[31mError: {}\x1B[0m", e);
            }
        }
    }
}
