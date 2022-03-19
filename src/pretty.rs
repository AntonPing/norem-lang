use std::fmt;
use std::io::{self, Write};
use std::collections::VecDeque;

use crate::symbol::*;
use crate::ast::*;

//#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PrettyPrinter<'src> {
    indent: usize,
    width: usize,
    max_width: usize,
    table: SymTable<'src>,
    commands: VecDeque<Command>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum Command {
    Indent(usize),
    Dedent(usize),
    Wrap(usize),
    Unwrap,
    Text(String),
    Line,
}

impl<'src> PrettyPrinter<'src> {
    pub fn new(max: usize, table: SymTable<'src>) -> PrettyPrinter<'src> {
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
                    write!(f, "{}", s);
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

    pub fn wrapped<F>(&mut self, width: usize, func: F)
    where F: Fn(&mut PrettyPrinter<'src>) {
        self.commands.push_back(Command::Wrap(width));
        func(self);
        self.commands.push_back(Command::Unwrap);
    }

    pub fn nested<F>(&mut self, width: usize, func: F)
    where F: Fn(&mut PrettyPrinter<'src>) {
        self.commands.push_back(Command::Indent(width));
        func(self);
        self.commands.push_back(Command::Dedent(width));
    }

    pub fn line(&mut self) {
        self.commands.push_back(Command::Line);
    }

    pub fn text<S: AsRef<str>>(&mut self, s: S) {
        self.commands.push_back(Command::Text(s.as_ref().into()));
    }
}

pub trait Print {
    fn print(&self, pp: &mut PrettyPrinter);
}

impl<T: fmt::Display> Print for T {
    fn print<'a>(&self, pp: &'a mut PrettyPrinter) {
        pp.text(self.to_string())
    }
}

/*
impl<'src> fmt::Display for PrettyPrinter<'src> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = self.render();
        write!(f,"{}",string)
    }
}
*/

impl Print for Symbol {
    fn print<'a>(&self, pp: &'a mut PrettyPrinter){
        match self {
            Symbol::Var(n) => {
                pp.text(n.to_string());
            },
            Symbol::Gen(n) => {
                pp.text(format!("#{}", n));
            }
        }
    }
}

impl Print for LitValue {
    fn print(&self, pp: &mut PrettyPrinter) {
        match *self {
            LitValue::Int(x) => { pp.text(format!("{}", x)) }
            LitValue::Real(x) => { pp.text(format!("{}", x)) }
            LitValue::Char(x) => { pp.text(format!("{}", x)) }
            LitValue::Bool(x) => { pp.text(format!("{}", x)) }
        }
    }
}





#[test]
pub fn test() {
    let mut pp = PrettyPrinter::new(120,SymTable::with_capacity(0));
    pp.wrapped(20, |pp| {
        pp.text("case");
        pp.text("x");
        pp.nested(2, |pp| {
            pp.line();
            pp.text("of _ => bar");
        });
        pp.nested(3, |pp| {
            pp.line();
            pp.nested(2, |pp| {
                pp.text("| _ => foo");
                pp.text("bar baz qux");
                pp.text("flub");
                pp.text("mosoaic");
            })
        });
    });
    println!("{}",pp.render());
}