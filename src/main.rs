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

fn remove_whitespace(tokenizer: &mut Tokenizer) {
    //TODO: Remove comments
    while tokenizer.peek_char().is_whitespace() {
        tokenizer.skip();
    }
}

//TODO: Move this in to the tokenizer impl
fn get_token(tokenizer: &mut Tokenizer) -> Token {
    remove_whitespace(tokenizer);
    
    let mut c = tokenizer.next_char();
    
    return match c {
        '\0' => Token::EOF,
        '+'  => Token::Plus,
        '-'  => Token::Minus,
        '*'  => Token::Multiply,
        '/'  => Token::Divide,
        _ => {
            if c.is_alphabetic() {
                let mut ident = String::new();
                ident.push(c);
                
                while tokenizer.peek_char().is_alphanumeric() {
                    c = tokenizer.next_char();
                    ident.push(c);
                }
                
                Token::Identifier(ident)
            }
            else if c.is_numeric() {
                let mut num: f64 = 0.0;
                num += c.to_digit(10).unwrap() as f64;
                
                while tokenizer.peek_char().is_alphanumeric() {
                    c = tokenizer.next_char();
                    num *= 10.0;
                    num += c.to_digit(10).unwrap() as f64;
                }                
                
                Token::Number(num)
            } else {
                panic!("Unknown character");
            }
        },
    }
}

fn get_all_tokens(tokenizer: &mut Tokenizer) -> Vec<Token> {
    let mut res = Vec::new();
    
    let mut token = get_token(tokenizer);
    while token != Token::Unknown && token != Token::EOF {
        res.push(token);
        token = get_token(tokenizer);
    }
    
    res
}

fn main() {
    let mut tokenizer = Tokenizer::new(String::from("1+2332"));

    let tokens = get_all_tokens(&mut tokenizer);
    for token in tokens {
        println!("Token: {:?}", token);
    }
}
