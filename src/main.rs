#[derive(Debug, PartialEq, Clone)]
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

/*
    struct Expr;
    
    
*/

/*trait Expr {

}*/

struct ParseState {
    tokenizer: Box<Tokenizer>,
    current_token: Token
}

impl ParseState {
    fn new(tokenizer: Box<Tokenizer>) -> ParseState {
        ParseState {
            tokenizer: tokenizer,
            current_token: Token::Unknown,
        }
    }
    
    fn get_token(&mut self) -> Token {
        let res = get_token(&mut self.tokenizer.as_mut());
        self.current_token = res.clone();
        res
    }
}


trait Expr {
    fn eval(&mut self) -> f64;
}

struct ExprNumber {
    number: f64
}

impl Expr for ExprNumber {
    fn eval(&mut self) -> f64 {
        self.number
    }
}

struct ExprOp {
    left: Box<Expr>,
    right: Box<Expr>,
    op: Token,
}

impl Expr for ExprOp {
    fn eval(&mut self) -> f64 {
        return match self.op {
            Token::Plus => self.left.eval() + self.right.eval(),
            _ => {
                panic!("Unknown operation");
            }
        }
    }
}

impl ExprOp {
    fn new(left: Box<Expr>, right: Box<Expr>, op: Token) -> ExprOp {
        ExprOp {
            left,
            right,
            op
        }
    }
    
    fn new_boxed(left: Box<Expr>, right: Box<Expr>, op: Token) -> Box<ExprOp> {
        Box::new(ExprOp::new(left, right, op))
    }
}

fn parse_factor(state: &mut ParseState) -> Box<Expr> {
    return match state.current_token {
        Token::Number(n) => {
            let res = Box::new(ExprNumber { number: n });
            state.get_token();
            res
        },
        _ => {
            panic!("Unknown token");
        }
    }
}

fn parse_mul(state: &mut ParseState) -> Box<Expr> {
    let mut left = parse_factor(state);
    
    while state.current_token == Token::Multiply || state.current_token == Token::Divide {
        let op = state.current_token.clone();
        
        state.get_token();
        
        let right = parse_factor(state);
        
        left = ExprOp::new_boxed(left, right, op);
    }
    
    left    
}

fn parse_add(state: &mut ParseState) -> Box<Expr> {
    let mut left = parse_mul(state);
    
    while state.current_token == Token::Plus || state.current_token == Token::Minus {
        let op = state.current_token.clone();
        
        state.get_token();
        
        let right = parse_mul(state);
        
        left = ExprOp::new_boxed(left, right, op);
    }
    
    left
}

fn parse_expr(state: &mut ParseState) -> Box<Expr> {
    parse_add(state)
}

//usr/local/sbin:/usr/local/bin:/usr/bin:/usr/bin/site_perl:/usr/bin/vendor_perl:/usr/bin/core_perl


fn main() {
    let tokenizer = Box::new(Tokenizer::new(String::from("5+4")));
    
    let mut parse_state = ParseState::new(tokenizer);
    parse_state.get_token();
    
    let mut expr = parse_expr(&mut parse_state);
    let val = expr.eval();
    
    println!("Expr: {}", val);
}
