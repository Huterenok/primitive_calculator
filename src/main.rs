use std::io::stdin;

use primitive_calculator::{Calculator, Error};

fn main() -> Result<(), Error> {
    loop {
        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_) => {
                let tokens = Calculator::parse(input);
                if tokens.is_err() {
                    println!("{:?}", tokens.err().unwrap());
                    continue;
                }
                let expr = Calculator::expression(tokens.unwrap());
                if let Some(v) = Calculator::evaluate(expr) {
                    println!("{v}")
                }
            }
            Err(err) => println!("{err}"),
        }
    }
}
