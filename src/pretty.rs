use crate::{symbol::*, ast::LitValue};
use std::{fmt, io::{self, Write}, collections::VecDeque};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PrettyPrinter {
    indent: usize,
    width: usize,
    max_width: usize,
    //Table: SymTable<'src>,
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

#[inline]
pub fn newline_indent<W: fmt::Write>(f: &mut W, indent: usize) -> fmt::Result {
    write!(f, "\n")?;
    for _ in 0..indent {
        write!(f, "{}", ' ')?;
    }
    Ok(())
}

impl PrettyPrinter {
    pub fn new(max: usize) -> PrettyPrinter {
        PrettyPrinter {
            indent: 0,
            width: 0,
            max_width: max,
            commands: VecDeque::new(),
        }
    }

    pub fn write_io<W: io::Write>(&mut self, f: &mut W) -> io::Result<()> {
        let mut buffer = String::new();
        if let Ok(()) = self.write_fmt(&mut buffer) {
            write!(f,"{}",buffer)?;
            Ok(())
        } else {
            // don't really understand how to throw an error
            Err(io::Error::last_os_error())
        }
    }

    pub fn write_fmt<W: fmt::Write>(&mut self, f: &mut W) -> fmt::Result {
        while let Some(cmd) = self.commands.pop_front() {
            use Command::*;
            match cmd {
                Indent(w) => {
                    self.indent += w;
                }
                Dedent(w) => {
                    self.indent -= w;
                }
                Wrap(w) => {
                    let record = self.max_width;
                    self.max_width = w;
                    self.write_fmt(f)?;
                    self.max_width = record;
                }
                Unwrap => {
                    return Ok(());
                }
                Line => {
                    newline_indent(f, self.indent)?;
                    self.width = self.indent;
                }
                Text(s) => {
                    self.width += s.len();
                    if self.width  >= self.max_width {
                        newline_indent(f, self.indent)?;
                        self.width = self.indent + s.len();
                    }
                    write!(f, "{}", s);
                }
            }
        }
        Ok(())
    }

    pub fn wrapped<F>(&mut self, width: usize, f: F) -> &mut Self
    where F: Fn(&mut PrettyPrinter) -> &mut PrettyPrinter
    {
        self.commands.push_back(Command::Wrap(width));
        f(self);
        self.commands.push_back(Command::Unwrap);
        self
    }

    pub fn nested<F>(&mut self, width: usize, f: F) -> &mut Self
    where F: Fn(&mut PrettyPrinter) -> &mut PrettyPrinter
    {
        self.commands.push_back(Command::Indent(width));
        f(self);
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

    //pub fn print<P: Print>(&mut self, p: &P) -> &mut Self {
        //p.print(self)
    //}
}

pub trait Print {
    fn print<'a>(&self, pp: &'a mut PrettyPrinter) -> &'a mut PrettyPrinter;
}

impl Print for Symbol {
    fn print<'a>(&self, pp: &'a mut PrettyPrinter) -> &'a mut PrettyPrinter {
        match self {
            Symbol::Var(n) => pp.text(n.to_string()),
            Symbol::Gen(n) => pp.text(format!("#{}", n)),
        }
    }
}

impl Print for LitValue {
    fn print<'a> (&self, pp: &'a mut PrettyPrinter) -> &'a mut PrettyPrinter {
        match *self {
            LitValue::Int(x) => { pp.text(format!("{}", x)) }
            LitValue::Real(x) => { pp.text(format!("{}", x)) }
            LitValue::Char(x) => { pp.text(format!("{}", x)) }
            LitValue::Bool(x) => { pp.text(format!("{}", x)) }
        }
    }
}

impl<T: fmt::Display> Print for T {
    fn print<'a>(&self, pp: &'a mut PrettyPrinter) -> &'a mut PrettyPrinter {
        pp.text(self.to_string())
    }
}

impl fmt::Display for PrettyPrinter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut copy = self.clone();
        copy.write_fmt(f)?;
        Ok(())
    }
}

#[test]
pub fn test() {
    let mut pp = PrettyPrinter::new(120);
    pp.wrapped(20, |pp| {
        pp.text("case")
            .text("x")
            .nested(2, |pp| pp.line().text("of _ => bar"))
            .nested(3, |pp| {
                pp.line().nested(2, |pp| {
                    pp.text("| _ => foo")
                        .text("bar baz qux")
                        .text(" flub ")
                        .text(" mosoaic")
                })
            })
            .line()
            .text("goodbye")
            .text(",")
            .text("world")
            .nested(2, |pp| pp.line().text("indent!").text("is fun!"))
    });
    println!("{}",pp);
}