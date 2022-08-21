#[derive(Debug)]
pub enum AST {
    Var(String),
    LmAbs(String, Box<AST>),
    LmApp(Box<AST>, Box<AST>),
}

#[derive(Debug)]
pub enum Term {
    TmVar(Info, usize, usize),
    TmAbs(Info, String, Box<Term>),
    TmApp(Info, Box<Term>, Box<Term>),
}

#[derive(Debug)]
pub enum Info {
    Info,
}

pub type Context = Vec<String>;

impl AST {
    fn into_term_helper(&self, ctx: &Context, nest: usize) -> Term {
        match self {
            AST::Var(string) => match ctx.iter().position(|r| r == string) {
                Some(index) => Term::TmVar(Info::Info, index, ctx.len()),
                None => {
                    panic!("unexpected var")
                }
            },
            AST::LmAbs(string, ast) => {
                let mut ctx_copy = ctx.clone();
                ctx_copy.push(string.clone());
                Term::TmAbs(
                    Info::Info,
                    string.clone(),
                    Box::new(ast.into_term_helper(&ctx_copy, nest + 1)),
                )
            }
            AST::LmApp(lh, rh) => Term::TmApp(
                Info::Info,
                Box::new(lh.into_term_helper(ctx, nest)),
                Box::new(rh.into_term_helper(&ctx.clone(), nest)),
            ),
        }
    }

    pub fn into_term(&self) -> Term {
        self.into_term_helper(&vec![], 0)
    }
}
