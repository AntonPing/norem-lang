use std::collections::HashMap;
use std::fmt::{self, write, Display};

use std::ops::Deref;

use crate::core::*;
use crate::symbol::{Symbol, newvar};
use crate::visitor::Visitor;

use itertools::Itertools;


/*
pub trait Pretty<S> {
    fn print(&self, f: &mut fmt::Formatter, s: &mut S) -> fmt::Result;
}

impl<T,S> Display for T 
    where T:Pretty<S>, S: Default {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = S::default();
        self.print(f,&mut s)
    }
}
*/

/*
fn iter_print<T>(
    vec: IntoIterator<Item=T, IntoIter= Vec<T>>,
    f: &mut fmt::Formatter
) -> fmt::Result {



}

struct IterDebug<T> {
    data: IntoIterator<Item=T, IntoIter=Vec<T>>
}



struct MyVec<T>(Vec<T>);

impl<T:fmt::Debug> fmt::Debug for MyVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}


*/



impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

impl<T:fmt::Debug> fmt::Debug for Def<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Def {func,args,body} = self;
        write!(f,"{func:?}({:?}) = ",args.iter().format(","))?;
        f.debug_struct("")
            .field("",body)
            .finish()
    }
}
/*
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
    f: fmt::Formatter<'a>,
    indent: usize,
}

impl<'a> Printer<'a> {
    pub fn new(f: fmt::Formatter) -> Printer {
        Printer {
            f,
            indent: 0,
        }
    }

    pub fn print<T>(&mut self, obj: &T) -> fmt::Result
    where T: Display {
        write!(self.f, "{}", obj)
    }
    pub fn print_ref<T,U> (&mut self, obj: &T) -> fmt::Result
    where T: Deref<Target = U>, U: fmt::Display{
        write!(self.f, "{}", obj.deref())
    }

    pub fn print_many<T,D>(&mut self, vec: &Vec<T>, delim: &D) -> fmt::Result
    where T: Display,D: Display{
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
        write!(self.f, "\n")?;
        for _ in 0..self.indent {
            write!(self.f, "{}", ' ')?;
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

impl<T: fmt::Display> Print for T {
    fn print<'a>(&self, pr: &mut Printer<'a>) -> fmt::Result {
        pr.print(self)
    }
}
/*
impl<T: Print> fmt::Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pr = Printer {
            f,
            indent: 0,
        };
        pr.print(self)
    }
}
*/

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