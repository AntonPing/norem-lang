use crate::ast::*;
use crate::symbol::*;

pub mod visitor;
pub mod opt1;
pub mod clos;
pub mod cps;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Atom {
    Var(Symbol),
    Glob(Symbol),
    Reg(usize),
    Prim(Prim),
    Int(i64),
    Real(f64),
    Bool(bool),
    Char(char),
}

use std::fmt;

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::Var(x) => write!(f,"{x}"),
            Atom::Glob(x) => write!(f,"@{x}"),
            Atom::Reg(x) => write!(f,"reg{x}"),
            Atom::Prim(x) => write!(f,"{x}"),
            Atom::Int(x) => write!(f,"{x}"),
            Atom::Real(x) => write!(f,"{x}"),
            Atom::Bool(x) => write!(f,"{x}"),
            Atom::Char(x) => write!(f,"{x}"),
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct CDecl {
    pub func: Symbol,
    pub args: Vec<Symbol>,
    pub body: Box<CExpr>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CExpr {
    App(Atom, Vec<Atom>),
    Let(CDecl, Box<CExpr>),
    Fix(Vec<CDecl>, Box<CExpr>),
    Uniop(Prim, Atom, Symbol, Box<CExpr>),
    Binop(Prim, Atom, Atom, Symbol, Box<CExpr>),
    Switch(Atom, Vec<CExpr>),
    Ifte(Atom, Box<CExpr>, Box<CExpr>),
    Record(Vec<Atom>, Symbol, Box<CExpr>),
    Select(usize, Atom, Symbol, Box<CExpr>),
    Halt(Atom),
    Tag(Tag, Box<CExpr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    SubstAtom(Symbol, Atom),
    SubstApp(CDecl),
}

impl CDecl {
    pub fn size(&self) -> usize {
        self.body.size()
    }
}


impl CExpr {
    pub fn size(&self) -> usize {
        match self {
            CExpr::App(_, args) => args.len() + 1,
            CExpr::Let(decl, cont) => {
                decl.size() + cont.size()
            }
            CExpr::Fix(decls, cont) => {
                decls.iter().fold(0, |n,decl| n + decl.size()) + cont.size()
            }
            CExpr::Uniop(_, _, _, cont) => cont.size() + 1,
            CExpr::Binop(_, _, _, _, cont) => cont.size() + 1,
            CExpr::Switch(_, brs) => brs.iter().fold(0, |n, br| n + br.size()),
            CExpr::Ifte(_, trbr, flbr) => trbr.size() + flbr.size() + 1,
            CExpr::Record(xs, _, cont) => xs.len() + cont.size(),
            CExpr::Select(_, _, _, cont) => cont.size() + 1,
            CExpr::Halt(_) => 1,
            CExpr::Tag(_, cont) => cont.size(),
        }
    }

    pub fn size_less_than(&self, n: usize) -> bool {
        let mut n = n;
        match self {
            CExpr::App(func, args) => {
                if n >= args.len() + 1 {
                    return true;
                } else {
                    return false;
                }
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