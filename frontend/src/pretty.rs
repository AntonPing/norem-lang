use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt::{self, Display, Formatter};
use std::io::{self, Write};
use std::ops::Deref;
use std::rc::Rc;

use crate::ast::*;
use crate::symbol::*;

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
pub struct PrettyPrinter {
    indent: usize,
    width: usize,
    max_width: usize,
    table: Rc<RefCell<SymTable>>,
    commands: VecDeque<Command>,
}

impl PrettyPrinter {
    pub fn new(max: usize, table: Rc<RefCell<SymTable>>) -> PrettyPrinter {
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
                    if self.width >= self.max_width {
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
    where
        F: for<'a> Fn(&'a mut PrettyPrinter) -> &'a mut PrettyPrinter,
    {
        self.commands.push_back(Command::Wrap(width));
        body(self);
        self.commands.push_back(Command::Unwrap);
        self
    }

    pub fn nested<F>(&mut self, width: usize, body: F) -> &mut Self
    where
        F: for<'a> Fn(&'a mut PrettyPrinter) -> &'a mut PrettyPrinter,
    {
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
    where
        F: for<'a> Fn(&'a mut PrettyPrinter) -> &'a mut PrettyPrinter,
    {
        left(self);
        body(self);
        right(self);
        self
    }

    pub fn seperated<F>(&mut self, vec: Vec<F>, delim: F) -> &mut Self
    where
        F: for<'a> Fn(&'a mut PrettyPrinter) -> &'a mut PrettyPrinter,
    {
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

    pub fn print_ref<T: Deref<Target = U>, U: Print>(&mut self, t: &T) -> &mut Self {
        t.print(self);
        self
    }

    pub fn print_many<T: Print, D: Display>(&mut self, vec: &Vec<T>, delim: &D) -> &mut Self {
        if !vec.is_empty() {
            vec[0].print(self);
            for elem in &vec[1..] {
                delim.print(self);
                elem.print(self);
            }
        }
        self
    }
    pub fn print_many_ref<T: Deref<Target = U>, U: Print, D: Display>(
        &mut self,
        vec: &Vec<T>,
        delim: &D,
    ) -> &mut Self {
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
