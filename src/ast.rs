#[derive(Debug)]
pub enum AST {
    Var(String),
    LmAbs(String, Box<AST>),
    LmApp(Box<AST>, Box<AST>),
}

#[derive(Debug, Clone)]
pub enum Term {
    TmVar(Info, usize, usize),
    TmAbs(Info, String, Box<Term>),
    TmApp(Info, Box<Term>, Box<Term>),
}

#[derive(Debug, Clone)]
pub enum Info {
    Info,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
pub enum Mode {
    EvalInnerLambda,
    NotEvalInnerLambda,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub var: Vec<String>,
    pub mode: Mode,
}

impl AST {
    fn into_term_helper(&self, ctx: &Context, nest: usize) -> Term {
        match self {
            AST::Var(string) => match ctx.var.iter().rev().position(|r| r == string) {
                Some(index) => Term::TmVar(Info::Info, index, ctx.var.len()),
                None => {
                    panic!("unexpected var")
                }
            },
            AST::LmAbs(string, ast) => {
                let mut ctx_copy = ctx.clone();
                ctx_copy.var.push(string.clone());
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

    pub fn into_term(&self, mode: Mode) -> Term {
        self.into_term_helper(
            &Context {
                var: vec![
                    "x".to_string(),
                    "y".to_string(),
                    "z".to_string(),
                    "a".to_string(),
                    "b".to_string(),
                ],
                mode: mode,
            },
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
                if ctx.var.len() == *n {
                    let ctx_reverse: Vec<&String> = ctx.var.iter().rev().collect();
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
            if ctx.var.contains(&x_mut) {
                x_mut = x_mut + "'";
            } else {
                let mut new_ctx = ctx.clone();
                new_ctx.var.push(x_mut.clone());
                return (new_ctx, x_mut);
            }
        }
    }

    fn shift(&self, d: i64) -> Term {
        self.shift_walk(d, 0)
    }

    fn shift_walk(&self, d: i64, c: usize) -> Term {
        match self {
            Term::TmVar(fi, x, n) => {
                if *x >= c {
                    Term::TmVar(
                        fi.clone(),
                        (*x as i64 + d) as usize,
                        (*n as i64 + d) as usize,
                    )
                } else {
                    Term::TmVar(fi.clone(), *x, (*n as i64 + d) as usize)
                }
            }
            Term::TmAbs(fi, x, t1) => {
                Term::TmAbs(fi.clone(), x.clone(), Box::new(t1.shift_walk(d, c + 1)))
            }

            Term::TmApp(fi, t1, t2) => Term::TmApp(
                fi.clone(),
                Box::new(t1.shift_walk(d, c)),
                Box::new(t2.shift_walk(d, c)),
            ),
        }
    }

    fn subst(&self, j: usize, s: Term) -> Term {
        self.subst_walk(j, s, 0)
    }

    fn subst_walk(&self, j: usize, s: Term, c: usize) -> Term {
        match self {
            Term::TmVar(fi, x, n) => {
                if *x == j + c {
                    s.shift(c as i64)
                } else {
                    Term::TmVar(fi.clone(), *x, *n)
                }
            }
            Term::TmAbs(fi, x, t1) => {
                Term::TmAbs(fi.clone(), x.clone(), Box::new(t1.subst_walk(j, s, c + 1)))
            }
            Term::TmApp(fi, t1, t2) => Term::TmApp(
                fi.clone(),
                Box::new(t1.subst_walk(j, s.clone(), c)),
                Box::new(t2.subst_walk(j, s, c)),
            ),
        }
    }

    fn subst_top(&self, s: Term) -> Term {
        self.subst(0, s.shift(1)).shift(-1)
    }

    fn isval(&self, ctx: &Context) -> bool {
        match self {
            Term::TmAbs(_, _, _) => true,
            _ => false,
        }
    }

    pub fn eval(&self, ctx: &Context) -> Term {
        match self {
            Term::TmApp(fi, t1, t2) => {
                let v1 = t1.eval(ctx);
                let v2 = t2.eval(ctx);
                match v1 {
                    Term::TmAbs(_, x, t12) => t12.subst_top(v2).eval(ctx),
                    t => Term::TmApp(fi.clone(), Box::new(t), Box::new(v2)),
                }
            }
            ast @ Term::TmAbs(i, var, e) => {
                if ctx.mode == Mode::EvalInnerLambda {
                    Term::TmAbs(i.clone(), var.to_owned(), Box::new(e.eval(ctx)))
                } else {
                    ast.clone()
                }
            }
            t => t.clone(),
        }
    }
}
