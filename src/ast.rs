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
            AST::Var(string) => match ctx.iter().rev().position(|r| r == string) {
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
        self.into_term_helper(
            &vec![
                "x".to_string(),
                "y".to_string(),
                "z".to_string(),
                "a".to_string(),
                "b".to_string(),
            ],
            0,
        )
    }
}

impl Term {
    pub fn printtm(&self, ctx: &Context) {
        match self {
            Term::TmAbs(_, x, t1) => {
                let (new_ctx, new_x) = Term::pickfreshname(&ctx, x.clone());
                print!("(λ{}. ", new_x);
                t1.printtm(&new_ctx);
                print!(")")
            }
            Term::TmApp(_, t1, t2) => {
                print!("(");
                t1.printtm(ctx);
                print!(" ");
                t2.printtm(ctx);
                print!(")");
            }
            Term::TmVar(_, x, n) => {
                if ctx.len() == *n {
                    let ctx_reverse: Vec<&String> = ctx.iter().rev().collect();
                    print!("{}", ctx_reverse[*x]);
                } else {
                    print!("[bad index]");
                }
            }
        }
    }

    fn pickfreshname(ctx: &Context, x: String) -> (Context, String) {
        let mut x_mut = x;
        loop {
            if ctx.contains(&x_mut) {
                x_mut = x_mut + "'";
            } else {
                let mut new_ctx = ctx.clone();
                new_ctx.push(x_mut.clone());
                return (new_ctx, x_mut);
            }
        }
    }
}
