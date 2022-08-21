#[derive(Debug)]
pub enum AST {
    Var(String),
    LmAbs(String, Box<AST>),
    LmApp(Box<AST>, Box<AST>),
}
