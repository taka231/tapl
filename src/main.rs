#[macro_use]
extern crate lalrpop_util;
mod ast;
use std::io::{self, Write};

use crate::ast::Context;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

macro_rules! print_flush {
    ( $( $x:expr ),* ) => {
        print!( $($x, )* );

        std::io::stdout().flush().expect("Could not flush to standard output.");
    };
}

fn main() {
    repl()
}

fn repl() {
    let parser = parser::TopParser::new();
    let mut mode = ast::Mode::EvalInnerLambda;
    loop {
        println!();
        print_flush!("untyped?> ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Could not read from standard input.");

        if &input == ":q\n" || &input == ":quit\n" {
            break;
        } else if input.chars().all(char::is_whitespace) {
            continue;
        } else if &input == ":set EvalInnerLambda\n" {
            mode = ast::Mode::EvalInnerLambda;
            continue;
        } else if &input == ":set NotEvalInnerLambda\n" {
            mode = ast::Mode::NotEvalInnerLambda;
            continue;
        }

        match parser.parse(&input) {
            Ok(ast) => ast
                .into_term(mode.clone())
                .eval(&Context {
                    var: vec![
                        "x".to_string(),
                        "y".to_string(),
                        "z".to_string(),
                        "a".to_string(),
                        "b".to_string(),
                    ],
                    mode: mode.clone(),
                })
                .printtm(&Context {
                    var: vec![
                        "x".to_string(),
                        "y".to_string(),
                        "z".to_string(),
                        "a".to_string(),
                        "b".to_string(),
                    ],
                    mode: mode,
                }),
            Err(e) => println!("{}", e),
        }
    }
}
