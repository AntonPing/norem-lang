use std::fmt;

use lazy_static::__Deref;

use crate::ast::*;
use crate::core::*;
use crate::symbol::*;

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pr = Printer::new(50);
        pr.print_expr(f, self)
    }
}

impl fmt::Display for Decl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pr = Printer::new(50);
        pr.print_decl(f, self)
    }
}

impl fmt::Display for CExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pr = Printer::new(50);
        pr.print_cexpr(f, self)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pr = Printer::new(50);
        pr.print_type(f, self)
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
    where
        F: FnOnce(&mut Printer) -> fmt::Result,
    {
        self.indent += indent;
        let res = func(self);
        self.indent -= indent;
        res
    }

    pub fn print_lit_val(&self, f: &mut fmt::Formatter, lit: &LitVal) -> fmt::Result {
        match lit {
            LitVal::Int(x) => write!(f, "{x}"),
            LitVal::Real(x) => write!(f, "{x}"),
            LitVal::Bool(x) => write!(f, "{x}"),
            LitVal::Char(x) => write!(f, "{x}"),
        }
    }

    pub fn print_varient(&mut self, f: &mut fmt::Formatter, var: &Variant) -> fmt::Result {
        write!(f, "<varient>")
    }

    pub fn print_type(&mut self, f: &mut fmt::Formatter, typ: &Type) -> fmt::Result {
        write!(f, "<type>")
    }

    pub fn print_decl(&mut self, f: &mut fmt::Formatter, decl: &Decl) -> fmt::Result {
        match decl {
            Decl::Val(decl) => {
                let DeclVal { name, args, body, span: _ } = decl;
                write!(f, "val {name}")?;
                for arg in args {
                    write!(f, " {arg}")?;
                }
                write!(f, " = ")?;
                self.print_expr(f, body)
            }
            Decl::Data(decl) => {
                let DeclData { name, args, vars, span: _ } = decl;
                write!(f, "data {name}")?;
                for arg in args {
                    write!(f, " {arg}")?;
                }
                write!(f, " = ")?;

                let mut first = true;
                for var in vars {
                    if first {
                        first = false;
                    } else {
                        write!(f, " | ")?;
                    }
                    self.print_varient(f, var)?;
                }
                Ok(())
            }
            Decl::Type(decl) => {
                let DeclType {
                    name,
                    args,
                    typ,
                    span: _,
                } = decl;
                write!(f, "type {name}")?;
                for arg in args {
                    write!(f, " {arg}")?;
                }
                write!(f, " = ")?;
                self.print_type(f, typ)
            }
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
                write!(f, "{prim:?}")
            }
            Expr::Var(expr) => {
                let ExprVar { ident, span: _ } = expr;
                write!(f, "{ident}")
            }
            Expr::Lam(expr) => {
                let ExprLam { args, body, span: _ } = expr;
                write!(f, "fn")?;
                for arg in args {
                    write!(f, " {arg}")?;
                }
                write!(f, " => ")?;
                self.nested(2, |p| {
                    p.newline(f)?;
                    p.print_expr(f, body)
                })
            }
            Expr::App(expr) => {
                let ExprApp { func, args, span: _ } = expr;
                write!(f, "(")?;
                self.print_expr(f, func)?;
                for arg in args {
                    write!(f, " ")?;
                    self.print_expr(f, arg)?;
                }
                write!(f, ")")
            }
            Expr::Let(expr) => {
                let ExprLet { decls, body, span: _ } = expr;
                write!(f, "let")?;
                self.nested(2, |p| {
                    p.newline(f)?;
                    let last = decls.len() - 1;
                    for decl in &decls[0..last] {
                        p.print_decl(f, decl)?;
                        p.newline(f)?;
                    }
                    p.print_decl(f, &decls[last])?;
                    Ok(())
                })?;
                self.newline(f)?;
                write!(f, "in")?;
                self.nested(2, |p| {
                    p.newline(f)?;
                    p.print_expr(f, body)
                })?;
                self.newline(f)?;
                write!(f, "end")
            }
            Expr::Case(expr) => {
                let ExprCase { expr, rules, span: _ } = expr;
                write!(f, "case ")?;
                self.print_expr(f, expr)?;
                write!(f, " of")?;
                self.nested(2, |p| {
                    p.newline(f)?;
                    let last = rules.len() - 1;
                    for rule in &rules[0..last] {
                        p.print_rule(f, rule)?;
                        p.newline(f)?;
                    }
                    p.print_rule(f, &rules[last])?;
                    Ok(())
                })
            }
        }
    }
    pub fn print_rule(&mut self, f: &mut fmt::Formatter, rule: &Rule) -> fmt::Result {
        let Rule { pat, body, span: _ } = rule;
        write!(f, "| ")?;
        self.print_pattern(f, pat)?;
        write!(f, " => ")?;
        self.print_expr(f, body)
    }

    pub fn print_pattern(&mut self, f: &mut fmt::Formatter, pat: &Pattern) -> fmt::Result {
        match pat {
            Pattern::App(cons, args) if args.len() > 0 => {
                write!(f, "({cons}")?;
                for arg in args {
                    write!(f, " ")?;
                    self.print_pattern(f, arg)?;
                }
                write!(f, ")")
            }
            Pattern::App(cons, _) => write!(f, "{cons}"),
            Pattern::Lit(lit) => self.print_lit_val(f, lit),
            Pattern::Var(sym) => write!(f, "{sym}"),
            Pattern::Wild => write!(f, "_"),
        }
    }

    pub fn print_cdecl(&mut self, f: &mut fmt::Formatter, decl: &CDecl) -> fmt::Result {
        let CDecl { func, args, body } = decl;
        write!(f,"{func} ")?;
        let mut first = true;
        for arg in args {
            if first {
                first = false;
                write!(f,"{arg}")?;
            } else {
                write!(f," {arg}")?;
            }
        }
        write!(f," = ")?;
        self.nested(2, |p| {
            p.newline(f)?;
            p.print_cexpr(f, body)
        })
    }

    pub fn print_tag(&mut self, f: &mut fmt::Formatter, tag: &Tag) -> fmt::Result {
        match tag {
            Tag::SubstAtom(_, _) => write!(f,"{tag:?}"),
            Tag::SubstApp(_) => write!(f,"{tag:?}"),
            _ => write!(f,"<tag>"),
        }
    }

    pub fn print_cexpr(&mut self, f: &mut fmt::Formatter, expr: &CExpr) -> fmt::Result {
        match expr {
            CExpr::App(func, args) => {
                write!(f,"{func}(")?;
                let mut first = true;
                for arg in args {
                    if first {
                        first = false;
                        write!(f,"{arg}")?;
                    } else {
                        write!(f," {arg}")?;
                    }
                }
                write!(f,")")
            }
            CExpr::Let(decl, body) => {
                write!(f,"let ")?;
                self.print_cdecl(f, decl)?;
                write!(f," in")?;
                self.newline(f)?;
                self.print_cexpr(f, body)
            }
            CExpr::Fix(decls, body) => {
                if decls.is_empty() {
                    write!(f, "let (empty) in ")?;
                    self.print_cexpr(f, body)?;
                    write!(f, " end")?;
                    return Ok(())
                }

                write!(f, "let")?;
                self.nested(2, |p| {
                    p.newline(f)?;
                    let last = decls.len() - 1;
                    for decl in &decls[0..last] {
                        p.print_cdecl(f, decl)?;
                        p.newline(f)?;
                    }
                    p.print_cdecl(f, &decls[last])?;
                    Ok(())
                })?;
                self.newline(f)?;
                write!(f, "in")?;
                self.nested(2, |p| {
                    p.newline(f)?;
                    p.print_cexpr(f, body)
                })?;
                self.newline(f)?;
                write!(f, "end")
            }
            CExpr::Uniop(prim, arg, ret, cont) => {
                write!(f,"{ret} <- {prim} {arg}")?;
                self.newline(f)?;
                self.print_cexpr(f, cont.deref())
            }
            CExpr::Binop(prim, arg1, arg2, ret, cont) => {
                write!(f,"{ret} <- {prim} {arg1} {arg2}")?;
                self.newline(f)?;
                self.print_cexpr(f, cont.deref())
            }
            CExpr::Switch(_, _) => todo!(),
            CExpr::Ifte(_, _, _) => todo!(),
            CExpr::Record(_, _, _) => todo!(),
            CExpr::Select(_, _, _, _) => todo!(),
            CExpr::Halt(arg) => {
                write!(f,"halt({arg})")
            }
            CExpr::Tag(tag, expr) => {
                write!(f,"tag{{")?;
                self.print_tag(f, tag)?;
                write!(f," : ")?;
                self.print_cexpr(f, expr)?;
                write!(f,"}}")
            }
        }
    }
}

#[test]
fn printer_test() {
    todo!();
}
