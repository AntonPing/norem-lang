
use std::collections::HashMap;

use std::ops::Deref;

use crate::symbol::{Symbol, newvar};
use crate::visitor::Visitor;
use crate::core::*;

use itertools::Itertools;
use pretty::*;


use std::fmt::{*, self};

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Atom::Var(x) => write!(f,"{:?}",x),
            Atom::Glob(x) => write!(f,"#{:?}",x),
            Atom::Reg(x) => write!(f,"r{}",x),
            Atom::Int(x) => write!(f,"{:?}",x),
            Atom::Real(x) => write!(f,"{:?}",x),
            Atom::Bool(x) => write!(f,"{:?}",x),
            Atom::Char(x) => write!(f,"{:?}",x),
        }
    }
}


impl 


impl CExpr {
    /// Return a pretty printed format of self.
    pub fn to_doc(&self) -> RcDoc<()> {
        match self {
            CExpr::App(func, args) => {
                RcDoc::as_string(func)
                    .append(RcDoc::text("("))
                    .append(RcDoc::intersperse(
                        args.iter().map(|arg|RcDoc::as_string(arg)), 
                        Doc::line())
                        .nest(1)
                        .group())
                    .append(RcDoc::text(")"))
            }
            CExpr::Let(_, _) => todo!(),
            CExpr::Fix(_, _) => todo!(),
            CExpr::Uniop(_, _, _, _) => todo!(),
            CExpr::Binop(_, _, _, _, _) => todo!(),
            CExpr::Switch(_, _) => todo!(),
            CExpr::Ifte(_, _, _) => todo!(),
            CExpr::Record(_, _, _) => todo!(),
            CExpr::Select(_, _, _, _) => todo!(),
            CExpr::Halt(_) => todo!(),
            CExpr::Tag(_, _) => todo!(),
        }
    }
}










/*


/*
struct PadAdapter<'buf> {
    buf: &'buf mut (dyn Write + 'buf),
    state: bool,
}

impl Write for PadAdapter<'_> {
    fn write_str(&mut self, mut s: &str) -> Result {
        while !s.is_empty() {
            if self.state {
                self.buf.write_str("    ")?;
            }

            let split = match s.find('\n') {
                Some(pos) => {
                    self.state = true;
                    pos + 1
                }
                None => {
                    self.state = false;
                    s.len()
                }
            };
            self.buf.write_str(&s[..split])?;
            s = &s[split..];
        }

        Ok(())
    }
}


fn nested_fmt<'a,'b>(
    f: &'a mut Formatter<'a>,
    slot: Option<Formatter<'b>>,
    indent: usize
) -> &'b mut Formatter<'b> {
    
    let flags = f.flags();
    let fill = f.fill();
    let align = match f.align() {
        Some(Alignment::Left) => {
            std::fmt::rt::v1::Alignment::Left
        }
        Some(Alignment::Center) => {
            std::fmt::rt::v1::Alignment::Center
        }
        Some(Alignment::Right) => {
            std::fmt::rt::v1::Alignment::Right
        }
        None => {
            std::fmt::rt::v1::Alignment::Unknown
        }
    };

    let width = f.width();
    let precision = f.precision();


    let result = unsafe {
        std::mem::transmute::<
            usize,
            fn(
                &'a mut Formatter<'a>,
                FnOnce(&'a mut (dyn Write + 'a)) -> &'b mut (dyn Write + 'b)
            ) -> Formatter<'b>
        >((core::fmt::pad as usize) - 64)(
            f,
            move |buf| {
                slot.insert(PadAdapter{buf,state: false})
            }
        )
    };
}

*/





/*
impl<T:fmt::Debug> fmt::Debug for Def<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Def {func,args,body} = self;
        write!(f,"{func:?}({:?}) = ",args.iter().format(","))?;
        f.debug_struct("")
            .field("",body)
            .finish()
    }
}

impl fmt::Debug for CExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CExpr::App(func, args) => {
                write!(f,"{func:?}({:?})",args.iter().format(","))
            }
            CExpr::Let(def, cont) => {
                writeln!(f,"let {def:?} in\n {cont:?}") 
            }
            CExpr::Fix(defs, cont) => {
                writeln!(f,"letrec {:?} in\n {cont:?}",
                    defs.iter().format("\n")) 
            }
            CExpr::Binop(prim, arg1, arg2, res, cont) => {
                writeln!(f,"{res:?} <- {prim:?} {arg1:?} {arg2:?}")?;
                write!(f,"{:?}",cont)
            }
            CExpr::Uniop(prim, arg, res, cont) => {
                writeln!(f,"{res:?} <- {prim:?} {arg:?}")?;
                write!(f,"{:?}",cont)
            }
            CExpr::Ifte(cond, trbr, flbr) => {
                f.debug_struct("Ifte")
                    .field("cond", cond)
                    .field("then", trbr)
                    .field("else", flbr)
                    .finish()
            }
            CExpr::Switch(_, _) => todo!(),
            CExpr::Record(_, _, _) => todo!(),
            CExpr::Select(_,_,_, _) => todo!(),
            CExpr::Halt(x) => {
                write!(f,"Halt({x:?})")
            }
            CExpr::Tag(tag, expr) => {
                write!(f,"{{ {tag:?} | {expr:?} }}")
            }
            
        }
    }
}

impl fmt::Debug for Tag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tag::SubstAtom(x, y) => {
                write!(f,"[{x:?} := {y:?}]")
            }
        }
    }
}
*/

//#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Printer<'a> {
    fmt: &'a mut fmt::Formatter<'a>,
    result: fmt::Result,
    indent: usize,
}

impl<'a> Printer<'a> {
    pub fn new(f: &'a mut fmt::Formatter<'a>) -> Printer<'a> {
        Printer {
            fmt: f,
            result: Ok(()),
            indent: 0,
        }
    }

    pub fn print<T>(&mut self, obj: &T) -> fmt::Result
    where T: Display {
        write!(self.fmt, "{}", obj)
    }

    pub fn print_ref<T,U> (&mut self, obj: &T) -> fmt::Result
    where T: Deref<Target = U>, U: fmt::Display {
        write!(self.fmt, "{}", obj.deref())
    }

    pub fn print_many<T,D>(&mut self, vec: &Vec<T>, delim: &D) -> fmt::Result
    where T: Display,D: Display {
        if !vec.is_empty() {
            self.print(&vec[0])?;
            for elem in &vec[1..] {
                self.print(delim)?;
                self.print(elem)?;
            }
        }
        Ok(())
    }
    pub fn print_many_ref<T,U,D>(&mut self, vec: &Vec<T>, delim: &D) -> fmt::Result
    where T: Deref<Target = U>, U: fmt::Display, D: Display{
        if !vec.is_empty() {
            self.print_ref(&vec[0])?;
            for elem in &vec[1..] {
                self.print(delim)?;
                self.print_ref(elem)?;
            }
        }
        Ok(())
    }

    pub fn newline(&mut self) -> fmt::Result {
        write!(self.fmt, "\n")?;
        for _ in 0..self.indent {
            write!(self.fmt, "{}", ' ')?;
        }
        Ok(())
    }

    pub fn nested<F>(&mut self, indent: usize, body: F) -> fmt::Result
    where F: for<'b> Fn(&'b mut Printer<'a>) -> fmt::Result {
        self.indent += indent;
        body(self)?;
        self.indent -= indent;
        Ok(())
    }

    pub fn surrounded<L,R,F>(&mut self, left: &L, right: &R, body: F) -> fmt::Result
    where L: Display, R: Display,
        F: (for<'b> Fn(&'b mut Printer) -> fmt::Result) {
        self.print(left)?;
        body(self)?;
        self.print(right)?;
        Ok(())
    }

    /*
  
    pub fn surrounded<F>(&mut self, left: F, body: F, right: F) -> fmt::Result
    where F: for<'b> Fn(&'b mut Printer) -> fmt::Result {
        left(self);
        body(self);
        right(self);
        Ok(())
    }

    pub fn seperated<F>(&mut self, vec: Vec<F>, delim: F) -> fmt::Result
    where F: for<'b> Fn(&'b mut Printer<'a>) -> fmt::Result {
        if !vec.is_empty() {
            self.print_ref(&vec[0])?;
            for elem in &vec[1..] {
                self.print(delim)?;
                self.print_ref(elem)?;
            }
        }
        Ok(())
    }
    */

}




pub trait Print {
    fn print<'a>(&self, pr: &mut Printer<'a>) -> fmt::Result;
}
/*
impl<T: fmt::Display> Print for T {
    fn print<'a>(&self, pr: &mut Printer<'a>) -> fmt::Result {
        pr.print(self)
    }
}

impl<T: Print> fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pr = Printer {
            fmt: f,
            result: Ok(()),
            indent: 0,
        };
        pr.print(self)
    }
}
*/

impl Print for Def<CExpr> {
    fn print<'a>(&self, pr: &mut Printer<'a>) -> fmt::Result {
        let Def {func, args, body} = self;
        pr.print(func)?;
        pr.surrounded(&'(', &')',|pr| {
            pr.print_many(args, &',')
        })?;
        pr.print(&" = ")?;
        pr.nested(2, |pr|
            body.print(pr))
    }
}

impl Print for CExpr {
    fn print<'a>(&self, pr: &mut Printer<'a>) -> fmt::Result {
        match self {
            CExpr::App(func, args) => {
                pr.print(func)?;
                pr.surrounded(&'(', &')',|pr| {
                    pr.print_many(args, &',')
                })
            }
            CExpr::Let(def, cont) => {
                pr.print(&"let ")?;
                def.print(pr)?;
                pr.newline()?;
                pr.print(&"in")?;
                pr.nested(2,|pr|
                    cont.print(pr))
            }
            CExpr::Fix(defs, cont) => {
                Ok(())
            }
            CExpr::Binop(prim, arg1, arg2, res, cont) => {
                Ok(())
            }
            CExpr::Uniop(prim, arg, res, cont) => {
                Ok(())
            }
            CExpr::Ifte(cond, trbr, flbr) => {
                Ok(())
            }
            CExpr::Switch(_, _) => {
                Ok(())
            }
            CExpr::Record(_, _, _) => {
                Ok(())
            }
            CExpr::Select(_,_,_, _) => {
                Ok(())
            }
            CExpr::Halt(x) => {
                pr.print(&"halt(")?;
                pr.print(x)?;
                pr.print(&")")
            }
            CExpr::Tag(tag, expr) => {
                Ok(())
            }
        }
    }
}


impl Display for CExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pr = Printer {
            fmt: f,
            result: Ok(()),
            indent: 0,
        };
        pr.print(self)
    }
}


#[test]
fn pretty_test() {
    use CExpr::*;
    use Atom::*;
    let f = newvar("f");
    let g = newvar("g");
    let x = newvar("x");
    let y = newvar("y");
    let z = newvar("z");


    let inner = Let(
        Def{
            func: g,
            args: vec![z],
            body: Box::new(App(Var(z),vec![Var(x)]))
        },
        Box::new(App(Var(y),vec![Var(g)]))
    );

    let outer = Let(
        Def{
            func: f,
            args: vec![x,y],
            body: Box::new(inner)
        },
        Box::new(App(Var(g),vec![Var(x),Var(y)]))
    );

    println!("{}", outer);
    
}

*/