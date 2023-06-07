#[macro_use]
extern crate lalrpop_util;
mod ast;
use std::env;
use std::fs::File;
use std::io::prelude::*;
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
    let args: Vec<String> = env::args().collect();
    repl(args.get(1).map(|x| x.to_owned()));
}

fn repl(arg: Option<String>) {
    let parser = parser::TopParser::new();
    let mut mode = ast::Mode::EvalInnerLambda;
    let mut result: Option<ast::Term> = None;
    let mut var_table: HashMap<String, ast::AST> = HashMap::new();
    match arg {
        Some(arg) => {
            // ファイルが見つかりませんでした
            let mut f = File::open(arg).expect("file not found");

            let mut contents = String::new();
            f.read_to_string(&mut contents)
                // ファイルの読み込み中に問題がありました
                .expect("something went wrong reading the file");
            let contents = contents.split("\n");
            for content in contents {
                let mut split_content = content.split(" ");
                let var = match split_content.next() {
                    Some(var) => var,
                    None => {
                        println!("var name was expected");
                        continue;
                    }
                };
                let ast_str: String = split_content
                    .map(|x| x.to_owned())
                    .collect::<Vec<String>>()
                    .join(" ");
                match parser.parse(&ast_str) {
                    Ok(ast) => {
                        var_table.insert(var.to_owned(), replace_ast(ast, &var_table));
                    }
                    Err(e) => print!("{}", e),
                }
            }
        }
        None => (),
    }
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
            let var = match split_input.next() {
                Some(var) => var,
                None => {
                    println!("var name was expected");
                    continue;
                }
            };
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
        } else if input.starts_with(":result") {
            result.clone().map(|term| {
                term.eval(&Context {
                    var: vec![
                        "unexpected_var".to_owned(),
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
                        "unexpected_var".to_owned(),
                        "x".to_string(),
                        "y".to_string(),
                        "z".to_string(),
                        "a".to_string(),
                        "b".to_string(),
                    ],
                    mode: mode,
                })
            });
            continue;
        }

        match parser.parse(&input).map(|ast| replace_ast(ast, &var_table)) {
            Ok(ast) => {
                let term = ast.into_term(mode.clone()).eval(&Context {
                    var: vec![
                        "unexpected_var".to_owned(),
                        "x".to_string(),
                        "y".to_string(),
                        "z".to_string(),
                        "a".to_string(),
                        "b".to_string(),
                    ],
                    mode: mode.clone(),
                });
                result = Some(term.clone());
                term.printtm(&Context {
                    var: vec![
                        "unexpected_var".to_owned(),
                        "x".to_string(),
                        "y".to_string(),
                        "z".to_string(),
                        "a".to_string(),
                        "b".to_string(),
                    ],
                    mode: mode,
                })
            }
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
