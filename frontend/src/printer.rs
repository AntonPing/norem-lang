use std::borrow::Borrow;
use std::cell::RefCell;
use std::fmt::{self, Display};
use crate::ast::*;
//use itertools::Itertools;

/*
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pr = Printer::new(50);
        pr.print_expr(f, self)
    }
}
*/

struct NewLine;
struct Indent;
struct Dedent;

static NEWLINE: NewLine = NewLine;
static INDENT: Indent = Indent;
static DEDENT: Dedent = Dedent;

/*
thread_local! {
    static INDENT_COUNT: RefCell<usize>  = RefCell::new(0);
}
*/

static mut INDENT_COUNT: usize = 0;

pub fn indent_zero() {
    unsafe { INDENT_COUNT = 0; }
}

impl Display for NewLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = unsafe { INDENT_COUNT };
        write!(f, "\n")?;
        for _ in 0..n {
            write!(f, "  ")?;
        }
        Ok(())
    }
}

impl Display for Indent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe { INDENT_COUNT += 1; }
        Ok(())
    }
}

impl Display for Dedent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe { INDENT_COUNT -= 1; }
        Ok(())
    }
}

//////////////////


impl Display for LitVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LitVal::Int(x) => write!(f, "{x}"),
            LitVal::Real(x) => write!(f, "{x}"),
            LitVal::Bool(x) => write!(f, "{x}"),
            LitVal::Char(x) => write!(f, "{x}"),
        }
    }
}

impl Display for LitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Prim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Fixity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<Ident,Extra> Display for Expr<Ident,Extra> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Lit(expr) => write!(f, "{expr}"),
            Expr::Prim(expr) => write!(f, "{expr}"),
            Expr::Var(expr) => write!(f, "{expr}"),
            Expr::Lam(expr) => write!(f, "{expr}"),
            Expr::App(expr) => write!(f, "{expr}"),
            Expr::Chain(expr) => write!(f, "{expr}"),
            Expr::Let(expr) => write!(f, "{expr}"),
            Expr::Case(expr) => write!(f, "{expr}"),
        }
    }
}

impl<Extra> Display for ExprLit<Extra> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ExprLit { lit, ext: _ } = self;
        write!(f, "{lit}")
    }
}

impl<Extra> Display for ExprPrim<Extra> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ExprPrim { prim, ext: _ } = self;
        write!(f, "{prim}")
    }
}

impl<Ident,Extra> Display for ExprVar<Ident, Extra> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ExprVar { name, ext: _ } = self;
        write!(f, "{name}")
    }
}

impl<Ident,Extra> Display for ExprLam<Ident, Extra> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ExprLam { args, body, ext: _ } = self;
        write!(f, "fn")?;
        for arg in args {
            write!(f, " {arg}")?;
        }
        write!(f, "=> {body}")
    }
}

impl<Ident,Extra> Display for ExprApp<Ident, Extra> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ExprApp { func, args, ext: _ } = self;
        write!(f, "{func}")?;
        for arg in args {
            write!(f, " {arg}")?;
        }
        Ok(())
    }
}

impl<Ident,Extra> Display for ExprChain<Ident, Extra> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ExprChain { head, tail, ext: _ } = self;
        write!(f,"{head}")?;
        for (op,expr) in tail {
            write!(f, " {op} {expr}")?;
        }
        Ok(())
    }
}

impl<Ident,Extra> Display for ExprLet<Ident, Extra> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ExprLet { decls, body, ext: _ } = self;

        write!(f, "let {INDENT}")?;
        for decl in decls {
            write!(f, "{NEWLINE}{decl}")?;
        }
        write!(f, "{DEDENT}{NEWLINE}\
            in {INDENT}{NEWLINE}\
                {body} {DEDENT}{NEWLINE}\
            end
        ")
    }
}


impl<Ident,Extra> Display for ExprCase<Ident, Extra> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ExprCase { expr, rules, ext: _ } = self;

        write!(f, "case {expr} of {INDENT}")?;
        for rule in rules {
            write!(f, "{NEWLINE}{rule}")?;
        }
        write!(f, "{DEDENT}{NEWLINE}\
            end\
        ")
    }
}

impl<Ident,Extra> Display for Decl<Ident,Extra> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Decl::Val(decl) => write!(f,"{decl}"),
            Decl::Data(decl) => write!(f,"{decl}"),
            Decl::Type(decl) => write!(f,"{decl}"),
            Decl::Opr(decl) => write!(f,"{decl}"),
        }
    }
}

impl<Ident,Extra> Display for DeclVal<Ident,Extra> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let DeclVal { name, args, body, ext: _ } = self;

        write!(f, "val {name}")?;
        for arg in args {
            write!(f, " {arg}")?;
        }
        write!(f, " = {INDENT}{NEWLINE}\
                {body} {DEDENT}\
        ")
    }
}

impl<Ident,Extra> Display for DeclData<Ident,Extra> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let DeclData { name, args, vars, ext: _ } = self;
        write!(f, "data {name}")?;
        for arg in args {
            write!(f, " {arg}")?;
        }
        write!(f, " = {INDENT}")?;
        for var in vars {
            write!(f, "{NEWLINE}| {var}")?;
        }
        write!(f, "{DEDENT}")
    }
}


impl<Ident,Extra> Display for DeclType<Ident,Extra> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let DeclType { name, args, typ, ext: _ } = self;
        write!(f, "type {name}")?;
        for arg in args {
            write!(f, " {arg}")?;
        }
        write!(f, " = {typ}")
    }
}

impl<Ident,Extra> Display for DeclOpr<Ident,Extra> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let DeclOpr { name, fixity, prec, func, ext: _ } = self;
        let fixity = match fixity {
            Fixity::Infixl => "infixl",
            Fixity::Infixr => "infixr",
            Fixity::Nonfix => "nonfix",
        };
        write!(f, "{fixity} {prec} {name} = {func}")
    }
}

impl<Ident> Display for Type<Ident> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<type>")
    }
}

impl<Ident,Extra> Display for Variant<Ident,Extra> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<varient>")
    }
}

impl<Ident,Extra> Display for Rule<Ident,Extra> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<rule>")
    }
}

impl<Ident> Display for Pattern<Ident> where
    Ident: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<pattern>")
    }
}


/*

pub fn print_stat(&mut self, f: &mut fmt::Formatter, stat: &Stat) -> fmt::Result {
    match stat {
        Stat::Let(stat) => {
            write!(f, "let {} = ", stat.name)?;
            self.print_expr(f, &stat.body)?;
            write!(f, ";")
        }
        Stat::Bind(stat) => {
            write!(f, "let {} <- ", stat.name)?;
            self.print_expr(f, &stat.body)?;
            write!(f, ";")
        }
        Stat::Drop(stat) => {
            self.print_expr(f, &stat.body)?;
            write!(f, ";")
        }
        Stat::Ret(stat) => {
            write!(f, "return ")?;
            self.print_expr(f, &stat.body)?;
            write!(f, ";")
        }
    } 
}



pub fn print_rule(&mut self, f: &mut fmt::Formatter, rule: &Rule) -> fmt::Result {
    let Rule { pat, body, ext: _ } = rule;
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

*/

#[test]
pub fn printer_test() {
    println!("\
        hello{INDENT}\
            hey!{NEWLINE}\
            hey!{NEWLINE}\
            hey!{DEDENT}\
        world{NEWLINE}
    ");
}