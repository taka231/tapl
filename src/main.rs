#[macro_use]
extern crate lalrpop_util;
mod ast;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

fn main() {
    parser::TopParser::new()
        .parse(r"\w. \a. x")
        .unwrap()
        .into_term()
        .printtm(&vec![
            "x".to_string(),
            "y".to_string(),
            "z".to_string(),
            "a".to_string(),
            "b".to_string(),
        ])
}
