#[macro_use]
extern crate lalrpop_util;
mod ast;
use std::io::{self, Write};

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
        }

        parser
            .parse(&input)
            .unwrap()
            .into_term()
            .eval(&vec![
                "x".to_string(),
                "y".to_string(),
                "z".to_string(),
                "a".to_string(),
                "b".to_string(),
            ])
            .printtm(&vec![
                "x".to_string(),
                "y".to_string(),
                "z".to_string(),
                "a".to_string(),
                "b".to_string(),
            ])
    }
}
