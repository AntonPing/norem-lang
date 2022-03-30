use std::cell::RefCell;
use std::fmt::{self, Formatter, Display};
use std::io::{self, Write};
use std::collections::VecDeque;
use std::ops::Deref;
use std::rc::Rc;

use crate::symbol::*;
use crate::ast::*;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum Command {
    Indent(usize),
    Dedent(usize),
    Wrap(usize),
    Unwrap,
    Text(String),
    Line,
}

//#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PrettyPrinter<'src> {
    indent: usize,
    width: usize,
    max_width: usize,
    table: Rc<RefCell<SymTable<'src>>>,
    commands: VecDeque<Command>,
}

impl<'src> PrettyPrinter<'src> {
    pub fn new(
        max: usize,
        table: Rc<RefCell<SymTable<'src>>>
    ) -> PrettyPrinter<'src> {

        PrettyPrinter {
            indent: 0,
            width: 0,
            max_width: max,
            table: table,
            commands: VecDeque::new(),
        }
    }

    pub fn newline<W: fmt::Write>(&self, f: &mut W) -> fmt::Result {
        write!(f, "\n")?;
        for _ in 0..self.indent {
            write!(f, "{}", ' ')?;
        }
        Ok(())
    }

    pub fn write_fmt<W: fmt::Write>(&mut self, f: &mut W) -> fmt::Result {
        while let Some(cmd) = self.commands.pop_front() {
            match cmd {
                Command::Indent(w) => {
                    self.indent += w;
                }
                Command::Dedent(w) => {
                    self.indent -= w;
                }
                Command::Wrap(w) => {
                    let record = self.max_width;
                    self.max_width = w;
                    self.write_fmt(f)?;
                    self.max_width = record;
                }
                Command::Unwrap => {
                    return Ok(());
                }
                Command::Line => {
                    self.newline(f)?;
                    self.width = self.indent;
                }
                Command::Text(s) => {
                    self.width += s.len();
                    if self.width  >= self.max_width {
                        self.newline(f)?;
                        self.width = self.indent + s.len();
                    }
                    write!(f, "{}", s)?;
                }
            }
        }
        Ok(())
    }

    pub fn render(&mut self) -> String {
        let mut res = String::new();
        self.write_fmt(&mut res).unwrap();
        res
    }

    pub fn wrapped<F>(&mut self, width: usize, body: F) -> &mut Self
    where F: for<'a> Fn(&'a mut PrettyPrinter<'src>) -> &'a mut PrettyPrinter<'src> {
        self.commands.push_back(Command::Wrap(width));
        body(self);
        self.commands.push_back(Command::Unwrap);
        self
    }

    pub fn nested<F>(&mut self, width: usize, body: F) -> &mut Self
    where F: for<'a> Fn(&'a mut PrettyPrinter<'src>) -> &'a mut PrettyPrinter<'src> {
        self.commands.push_back(Command::Indent(width));
        body(self);
        self.commands.push_back(Command::Dedent(width));
        self
    }

    pub fn line(&mut self) -> &mut Self {
        self.commands.push_back(Command::Line);
        self
    }

    pub fn text<S: AsRef<str>>(&mut self, s: S) -> &mut Self {
        self.commands.push_back(Command::Text(s.as_ref().into()));
        self
    }

    pub fn surrounded<F>(&mut self, left: F, body: F, right: F) -> &mut Self
    where F: for<'a> Fn(&'a mut PrettyPrinter<'src>) -> &'a mut PrettyPrinter<'src> {
        left(self);
        body(self);
        right(self);
        self
    }

    pub fn seperated<F>(&mut self, vec: Vec<F>, delim: F) -> &mut Self
    where F: for<'a> Fn(&'a mut PrettyPrinter<'src>) -> &'a mut PrettyPrinter<'src> {
        if !vec.is_empty() {
            vec[0](self);
            for elem in &vec[1..] {
                delim(self);
                elem(self);
            }
        }
        self
    }

    pub fn print<T: Print>(&mut self, t: &T) -> &mut Self {
        t.print(self);
        self
    }

    pub fn print_many<T: Print,D: Display>(&mut self, vec: &Vec<T>, delim: &D) -> &mut Self {
        if !vec.is_empty() {
            vec[0].print(self);
            for elem in &vec[1..] {
                delim.print(self);
                elem.print(self);
            }
        }
        self
    }
    pub fn print_many_ref<T: Deref<Target = U>,U: Print,D: Display>(&mut self, vec: &Vec<T>, delim: &D) -> &mut Self {
        if !vec.is_empty() {
            vec[0].print(self);
            for elem in &vec[1..] {
                delim.print(self);
                elem.print(self);
            }
        }
        self
    }
}

pub trait Print {
    fn print(&self, pp: &mut PrettyPrinter);
}

impl<T: fmt::Display> Print for T {
    fn print(&self, pp: &mut PrettyPrinter) {
        pp.text(self.to_string());
    }
}

impl Print for Symbol {
    fn print(&self, pp: &mut PrettyPrinter){
        match self {
            &Symbol::Var(n) => {
                let string = pp.table.borrow().get_str(n).unwrap();
                pp.text(string);
            }
            &Symbol::Gen(n) => {
                pp.text(format!("#{}", n));
            }
            &Symbol::Forall(n) => { 
                let char = 
                    "abcdefghijklmnopqrstuvwxyz"
                    .to_string().chars().nth(n).unwrap();
                pp.text(char.to_string());
            }
        }
    }
}

impl Print for LitValue {
    fn print(&self, pp: &mut PrettyPrinter) {
        match *self {
            LitValue::Int(x) => { pp.text(format!("{}", x)); }
            LitValue::Real(x) => { pp.text(format!("{}", x)); }
            LitValue::Char(x) => { pp.text(format!("{}", x)); }
            LitValue::Bool(x) => { pp.text(format!("{}", x)); }
        }
    }
}

impl Print for Expr {
    fn print(&self, pp: &mut PrettyPrinter) {
        match self {
            Expr::Lit(lit) => {
                lit.print(pp);
            }
            Expr::Var(x) => {
                x.print(pp);
            }
            Expr::Lam(xs, body) => {
                pp
                .text("fn ")
                .print_many(xs, &' ')
                .text(" => ")
                .print(body.deref());
            }
            Expr::App(exprs) => {
                pp
                .text("(")
                .print_many_ref(exprs, &' ')
                .text(")");
            }
            _ => {
                pp.text("???");
            }
        }
    }
}

impl Print for ValDecl {
    fn print(&self, pp: &mut PrettyPrinter) {
        let ValDecl { name, args, body } = self;
        pp
        .text("val ")
        .print(name)
        .print_many(args,&' ')
        .text(" = ")
        .print(body);
    }
}

impl Print for DataDecl {
    fn print(&self, pp: &mut PrettyPrinter) {
        let DataDecl { name, args, vars } = self;
        pp
        .text("data ")
        .print(name)
        .print_many(args,&' ')
        .text(" = ")
        .print_many_ref(vars, &'|');
    }
}

impl Print for TypeDecl {
    fn print(&self, pp: &mut PrettyPrinter) {
        let TypeDecl { name, args, typ } = self;
        pp
        .text("type ")
        .print(name)
        .print_many(args,&' ')
        .text(" = ")
        .print(typ);
    }
}


impl Print for Decl {
    fn print(&self, pp: &mut PrettyPrinter) {
        match self {
            Decl::Val(val) => { pp.print(val.deref()); }
            Decl::Data(data) => { pp.print(data.deref()); }
            Decl::Type(typ) => { pp.print(typ.deref()); }
        }
    }
}

impl Print for Rule {
    fn print(&self, pp: &mut PrettyPrinter) {
        let Rule { pat, expr } = self;
        pp
        .text("| ")
        .print(pat.deref())
        .text(" => ")
        .print(expr.deref());
    }
}

impl Print for Pattern {
    fn print(&self, pp: &mut PrettyPrinter) {
        match self {
            Pattern::App(cons,args) => {
                pp
                .text("(")
                .print(cons)
                .print_many_ref(args, &' ')
                .text(")");
            }
            Pattern::Lit(lit) => {
                pp.print(lit);
            }
            Pattern::Var(sym) => {
                pp.print(sym);
            }
            Pattern::Wild => {
                pp.text("_");
            }
        }
    }
}

impl Print for Type {
    fn print(&self, pp: &mut PrettyPrinter) {
        pp.text("[type]");
    }
}

impl Print for Variant {
    fn print(&self, pp: &mut PrettyPrinter) {
        let Variant { cons, args } = self;
        pp
        .print(cons)
        .print_many(args, &' ');
    }
}


#[test]
pub fn test() {
    let mut table = Rc::new(RefCell::new(SymTable::new()));
    let mut pp = PrettyPrinter::new(120,table);
    pp.wrapped(20, |pp| { pp
        .text("case")
        .text("x")
        .nested(2, |pp| { pp
            .line()
            .text("of _ => bar")
        })
        .nested(3, |pp| { pp
            .line()
            .nested(2, |pp| { pp
                .text("| _ => foo")
                .text("bar baz qux")
                .text("flub")
                .text("mosoaic")
            })
        })
    });
    println!("{}",pp.render());
    println!("{}",pp.render());
    println!("{}",pp.render());
}

