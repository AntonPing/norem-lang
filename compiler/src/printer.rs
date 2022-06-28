use std::fmt;

use lazy_static::__Deref;

use crate::ast::*;
use crate::backend::*;
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

    pub fn indent(&mut self, len: usize) {
        self.indent += len;
    }

    pub fn indent_newline<W: fmt::Write>(
        &mut self,
        f: &mut W,
        len: usize
    ) -> fmt::Result {
        self.indent += len;
        self.newline(f)
    }

    pub fn dedent(&mut self, len: usize) {
        self.indent -= len;
    }

    pub fn dedent_newline<W: fmt::Write>(
        &mut self,
        f: &mut W,
        len: usize
    ) -> fmt::Result {
        self.indent -= len;
        self.newline(f)
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
                let DeclVal { name, args, body, span: _ } = &decl;
                write!(f, "val {name}")?;
                for arg in args {
                    write!(f, " {arg}")?;
                }
                write!(f, " = ")?;
                self.print_expr(f, body)
            }
            Decl::Data(decl) => {
                let DeclData { name, args, vars, span: _ } = &decl;
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
                let DeclType { name, args, typ, span: _ } = &decl;
                write!(f, "type {name}")?;
                for arg in args {
                    write!(f, " {arg}")?;
                }
                write!(f, " = ")?;
                self.print_type(f, typ)
            }
        }
    }

    pub fn print_expr_outer(&mut self, f: &mut fmt::Formatter, expr: &Expr) -> fmt::Result {
        if let Expr::App(expr) = expr {
            self.print_expr(f, &expr.func)?;
            for arg in &expr.args {
                write!(f, " ")?;
                self.print_expr(f, arg)?;
            }
            Ok(())
        } else {
            self.print_expr(f, expr)
        }
    }

    pub fn print_expr(&mut self, f: &mut fmt::Formatter, expr: &Expr) -> fmt::Result {
        match expr {
            Expr::Lit(expr) => {
                self.print_lit_val(f, &expr.lit)
            }
            Expr::Prim(expr) => {
                write!(f, "{}", &expr.prim)
            }
            Expr::Var(expr) => {
                write!(f, "{}", &expr.ident)
            }
            Expr::Lam(expr) => {
                write!(f, "fn")?;
                for arg in &expr.args {
                    write!(f, " {arg}")?;
                }
                write!(f, " => ")?;
                self.indent_newline(f, 2)?;
                self.print_expr(f, &expr.body)?;
                self.dedent(2);
                Ok(())
            }
            Expr::App(expr) => {
                write!(f, "(")?;
                self.print_expr(f, &expr.func)?;
                for arg in &expr.args {
                    write!(f, " ")?;
                    self.print_expr(f, arg)?;
                }
                write!(f, ")")
            }
            Expr::Let(expr) => {
                write!(f, "let")?;
                self.indent(2);
                for decl in &expr.decls {
                    self.newline(f)?;
                    self.print_decl(f, decl)?; 
                }
                self.dedent_newline(f, 2)?;
                write!(f, "in")?;
                self.indent_newline(f, 2)?;
                self.print_expr(f, &expr.body)?;
                self.dedent_newline(f, 2)?;
                write!(f, "end")
            }
            Expr::Case(expr) => {
                write!(f, "case ")?;
                self.print_expr(f, &expr.expr)?;
                write!(f, " of")?;
                self.indent(2);
                for rule in &expr.rules {
                    self.newline(f)?;
                    self.print_rule(f, rule)?;
                }
                self.dedent_newline(f, 2)?;
                write!(f, "end")
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
        write!(f,"{func}")?;

        for arg in args {
            write!(f," {arg}")?;
        }
        write!(f," = ")?;
        self.indent_newline(f, 2)?;
        self.print_cexpr(f, body)?;
        write!(f, ";")?;
        self.dedent(2);
        Ok(())
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
                if !args.is_empty() {
                    write!(f,"{}", &args[0])?;
                    for arg in &args[1..] {
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
                self.indent(2);

                for decl in decls {
                    self.newline(f)?;
                    self.print_cdecl(f, decl)?;
                }
                self.dedent_newline(f, 2)?;
                write!(f, "in")?;
                self.indent_newline(f, 2)?;
                self.print_cexpr(f, body)?;
                self.dedent_newline(f, 2)?;
                write!(f, "end")
            }
            CExpr::Uniop(prim, arg, ret, cont) => {
                write!(f,"{prim} {arg} -> {ret}")?;
                self.newline(f)?;
                self.print_cexpr(f, cont.deref())
            }
            CExpr::Binop(prim, arg1, arg2, ret, cont) => {
                write!(f,"{prim} {arg1} {arg2} -> {ret}")?;
                self.newline(f)?;
                self.print_cexpr(f, cont.deref())
            }
            CExpr::Switch(n, conts) => {
                write!(f,"switch {n}")?;
                self.indent(2);
                for cont in conts {
                    self.newline(f)?;
                    write!(f,"| ")?;
                    self.print_cexpr(f, cont)?;
                }
                self.dedent_newline(f, 2)?;
                write!(f, "end")
            }
            CExpr::Ifte(cond, trbr, flbr) => {
                write!(f,"if {cond}")?;
                self.newline(f)?;
                write!(f,"then ")?;
                self.print_cexpr(f, trbr)?;
                self.newline(f)?;
                write!(f,"else ")?;
                self.print_cexpr(f, flbr)
            }
            CExpr::Record(xs, ret, cont) => {
                write!(f,"{{")?;
                for x in xs {
                    write!(f, " {x}")?;
                }
                write!(f," }} -> {ret}")?;
                self.newline(f)?;
                self.print_cexpr(f, cont)
            }
            CExpr::Select(n, rec, ret, cont) => {
                write!(f,"select {n} of {rec} -> {ret}")?;
                self.newline(f)?;
                self.print_cexpr(f, cont)
            }
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
