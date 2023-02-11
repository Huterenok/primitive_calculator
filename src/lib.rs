#[derive(PartialEq, PartialOrd, Debug, Eq, Ord)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Exp,
}

#[derive(PartialEq, PartialOrd, Debug, Eq, Ord)]
pub enum Token {
    Op(Operator),
    Num(u32),
    Bracket(char),
}

#[derive(Debug)]
pub enum Error {
    BadToken(char),
    MismatchedParens,
}

pub struct Calculator {}

impl Calculator {
    pub fn parse<T: AsRef<str>>(expr: T) -> Result<Vec<Token>, Error> {
        let expr = expr.as_ref();
        let chars = expr.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parens = Vec::new();
        for c in chars {
            match c {
                '0'..='9' => match tokens.last_mut() {
                    Some(Token::Num(num)) => {
                        *num = *num * 10 + (c as u32 - 48);
                    }
                    _ => {
                        let digit = c as u32 - 48;
                        tokens.push(Token::Num(digit));
                    }
                },
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push('(');
                }
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if parens.pop().unwrap() != '(' {
                        return Err(Error::MismatchedParens);
                    }
                }
                '+' => tokens.push(Token::Op(Operator::Add)),
                '-' => tokens.push(Token::Op(Operator::Sub)),
                '/' => tokens.push(Token::Op(Operator::Div)),
                '*' => tokens.push(Token::Op(Operator::Mul)),
                '^' => tokens.push(Token::Op(Operator::Exp)),
                ' ' => {}
                '\n' => {}
                '\r' => {}
                _ => return Err(Error::BadToken(c)),
            }
        }

        if parens.len() > 0 {
            return Err(Error::MismatchedParens);
        }

        Ok(tokens)
    }

    pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();

        let mut stack: Vec<Token> = Vec::new();
        let mut queue: Vec<Token> = Vec::new();
        let mut parens_stack: Vec<Token> = Vec::new();
        let mut parens_queue: Vec<Token> = Vec::new();

        while let Some(token) = tokens.pop() {
            match token {
                Token::Num(_) => {
                    // if stack[stack.len() - 1] == Token::Bracket('(') {
                    if stack.len() > 0 && stack[stack.len() - 1] == Token::Bracket('(') {
                        parens_queue.push(token);
                        continue;
                    }
                    queue.push(token);
                }
                Token::Op(_) => {
                    if stack.len() > 0 && stack[stack.len() - 1] == Token::Bracket('(') {
                        while !parens_stack.is_empty()
                            && parens_stack[parens_stack.len() - 1] >= token
                        {
                            parens_queue.push(parens_stack.pop().unwrap());
                        }
                        parens_stack.push(token)
                    } else {
                        while !stack.is_empty() && stack[stack.len() - 1] >= token {
                            queue.push(stack.pop().unwrap());
                        }
                        stack.push(token)
                    }
                }
                Token::Bracket('(') => stack.push(token),
                Token::Bracket(')') => {
                    while !stack.is_empty() && stack[stack.len() - 1] != Token::Bracket('(') {
                        queue.push(stack.pop().unwrap())
                    }
                    stack.pop();
                }
                _ => {}
            }
        }

        while parens_stack.len() > 0 {
            parens_queue.push(parens_stack.pop().unwrap())
        }
        parens_queue.append(&mut queue);
        while stack.len() > 0 {
            parens_queue.push(stack.pop().unwrap())
        }
        println!("{:?}", parens_queue);

        parens_queue
    }

    pub fn evaluate(mut tokens: Vec<Token>) -> Option<f32> {
        tokens.reverse();

        let mut stack: Vec<f32> = Vec::new();

        while let Some(token) = tokens.pop() {
            println!("{:?}", token);
            match token {
                Token::Num(num) => stack.push(num as f32),
                Token::Op(Operator::Add) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left + right);
                }
                Token::Op(Operator::Sub) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left - right);
                }
                Token::Op(Operator::Div) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left / right);
                }
                Token::Op(Operator::Mul) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left * right);
                }
                Token::Op(Operator::Exp) => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(left.powf(right));
                }
                Token::Bracket('(') => {
                    println!("im bracket");
                }
                _ => {}
            }
        }
        if stack.len() > 1 {
            None
        } else {
            stack.pop()
        }
    }
}
