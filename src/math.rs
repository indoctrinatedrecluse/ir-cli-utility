#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    LParen,
    RParen,
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
                _ => return Err(format!("Unexpected character '{}' in expression", c)),
            }
        }
    }
    tokens.push(Token::EOF);
    Ok(tokens)
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
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

pub fn evaluate(expr: &str) {
    let tokens = match tokenize(expr) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Error: Tokenization error: {}", e);
            std::process::exit(1);
        }
    };

    let mut parser = Parser::new(tokens);
    let result = match parser.expr() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error: Evaluation error: {}", e);
            std::process::exit(1);
        }
    };

    if *parser.peek() != Token::EOF {
        eprintln!("Error: Invalid expression (unexpected trailing characters).");
        std::process::exit(1);
    }

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
