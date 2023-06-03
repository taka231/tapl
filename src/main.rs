#[macro_use]
extern crate lalrpop_util;
mod ast;
use std::{
    collections::HashMap,
    io::{self, Write},
};

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
    let mut var_table: HashMap<String, ast::AST> = HashMap::new();
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
        } else if input.starts_with(":let") {
            let mut split_input = input.split(" ");
            split_input.next();
            let var = split_input.next().unwrap();
            let ast_str: String = split_input
                .map(|x| x.to_owned())
                .collect::<Vec<String>>()
                .join(" ");
            match parser.parse(&ast_str) {
                Ok(ast) => {
                    var_table.insert(var.to_owned(), replace_ast(ast, &var_table));
                }
                Err(e) => print!("{}", e),
            }
            continue;
        }

        match parser.parse(&input).map(|ast| replace_ast(ast, &var_table)) {
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

fn replace_ast(ast: ast::AST, var_table: &HashMap<String, ast::AST>) -> ast::AST {
    match ast {
        ast::AST::Var(s) => match var_table.get(&s) {
            Some(ast) => ast.clone(),
            None => ast::AST::Var(s),
        },
        ast::AST::LmAbs(s, ast) => ast::AST::LmAbs(s, Box::new(replace_ast(*ast, var_table))),
        ast::AST::LmApp(ast1, ast2) => ast::AST::LmApp(
            Box::new(replace_ast(*ast1, var_table)),
            Box::new(replace_ast(*ast2, var_table)),
        ),
    }
}
