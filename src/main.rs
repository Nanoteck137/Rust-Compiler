#[derive(Debug, PartialEq)]
enum Token {
    Unknown,
    
    Plus,
    Minus,
    Multiply,
    Divide,
    
    Identifier(String),
    Number(f64),
    
    EOF
}

struct Tokenizer {
    at: String,
    index: u32
}

impl Tokenizer {
    fn new(at: String) -> Self {
        Tokenizer {
            at: at,
            index: 0
        }
    }
    
    fn next_char(&mut self) -> char {
        let result = match self.at.chars().nth(self.index as usize) {
            Some(c) => c,
            None => '\0',
        };
        self.index += 1;
        
        result
    }
    
    fn peek_char(&self) -> char {
        return match self.at.chars().nth(self.index as usize) {
            Some(c) => c,
            None => '\0',
        }
    }
    
    fn skip(&mut self) {
        self.index += 1;
    }
}

//TODO: Move this in to the tokenizer impl
fn get_token(tokenizer: &mut Tokenizer) -> Option<Token> {
    let mut c = tokenizer.next_char();
    
    return match c {
        '\0' => Some(Token::EOF),
        '+' => Some(Token::Plus),
        '-' => Some(Token::Minus),
        '*' => Some(Token::Multiply),
        '/' => Some(Token::Divide),
        _ => {
            if c.is_alphabetic() {
                let mut ident = String::new();
                ident.push(c);
                
                while tokenizer.peek_char().is_alphanumeric() {
                    c = tokenizer.next_char();
                    ident.push(c);
                }
                
                Some(Token::Identifier(ident))
            }
            else if c.is_numeric() {
                println!("Found Digit");
                Some(Token::Number(12.0))
            } else {
                panic!("Unknown character");
            }
        },
    }
}

fn main() {
    let mut tokenizer = Tokenizer::new(String::from("HH+"));

    let mut t = get_token(&mut tokenizer);
    println!("Token: {:?}", t);
    
    t = get_token(&mut tokenizer);
    println!("Token: {:?}", t);
}
