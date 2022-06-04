use std::collections::HashMap;
use std::fmt::{self, write};

use crate::core::*;
use crate::symbol::{Symbol, newvar};

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



impl fmt::Debug for Atom {
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
        write!(f,"{func:?}({:?}) = {body:?}",args.iter().format(","))
    }
}



impl fmt::Debug for CExpr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CExpr::App(func, args) => {
                write!(f,"{func:?}({:?})",args.iter().format(","))
            }
            CExpr::Let(def, cont) => {
                //writeln!(f,"let {def:?} in")?;
                f.debug_struct("let")
                    .field("val", def)
                    .finish()?;
                write!(f,"\nin {cont:?}")       
            }
            CExpr::Fix(_, _) => {
                todo!()
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
            CExpr::Select(_, _) => todo!(),
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

    println!("{:#?}", outer);
    
}