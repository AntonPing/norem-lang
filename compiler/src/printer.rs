use std::fmt;

use crate::symbol::*;
use crate::ast::*;

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pr = Printer::new(50);
        pr.print_expr(f, self)
    }
}

//#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Printer {
    indent: usize,
    width: usize,
    max_width: usize,
}

impl Printer {
    pub fn new(max: usize) -> Printer {
        Printer {
            indent: 0,
            width: 0,
            max_width: max,
        }
    }

    pub fn newline<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        write!(f, "\n")?;
        for _ in 0..self.indent {
            write!(f, "{}", ' ')?;
        }
        Ok(())
    }

    pub fn nested<F>(&mut self, indent: usize, func: F) -> fmt::Result
    where F : FnOnce(&mut Printer) -> fmt::Result {
        self.indent += indent;
        let res = func(self);
        self.indent -= indent;
        res
    }

    pub fn print_lit_val(&self, f: &mut fmt::Formatter, lit: &LitVal) -> fmt::Result {
        match lit {
            LitVal::Int(x) => write!(f,"{x}"),
            LitVal::Real(x) => write!(f,"{x}"),
            LitVal::Bool(x) => write!(f,"{x}"),
            LitVal::Char(x) => write!(f,"{x}"),
        }
    }

    pub fn print_expr(&mut self, f: &mut fmt::Formatter, expr: &Expr) -> fmt::Result {
        match expr {
            Expr::Lit(lit) => {
                let ExprLit { lit, span: _ } = lit;
                self.print_lit_val(f, lit)
            }
            Expr::Prim(prim) => {
                let ExprPrim { prim, span: _ } = prim;
                write!(f,"{prim}")
            }
            Expr::Var(var) => {
                let ExprVar { ident, span: _ } = var;
                write!(f,"{ident}")
            }
            Expr::Lam(lam) => {
                let ExprLam { args, body, span: _ } = lam;
                write!(f,"fn")?;
                for arg in args {
                    write!(f," {arg}")?;
                }
                write!(f," => ")?;
                self.nested(2, |p| {
                    p.newline(f)?;
                    p.print_expr(f, body)
                })
            }
            Expr::App(app) => {
                let ExprApp { func, args, span: _ } = app;
                write!(f,"(")?;
                self.print_expr(f, func)?;
                for arg in args {
                    write!(f," ")?;
                    self.print_expr(f, arg)?;
                }
                write!(f,")")
            }
            Expr::Let(_) => todo!(),
            Expr::Case(_) => todo!(),
        }
    }
}

#[test]
fn printer_test() {
    todo!();
}