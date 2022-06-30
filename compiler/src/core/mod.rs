use crate::ast::Prim;
use crate::symbol::Symbol;

pub mod visitor;
pub mod cps_trans;
pub mod opt1;
pub mod clos_conv;

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
pub enum Core {
    App(CoreApp),
    Let(CoreLet),
    Opr(CoreOpr),
    Case(CoreCase),
    //Cond(CoreCond),
    Rec(CoreRec),
    Sel(CoreSel),
    Halt(Atom),
    Tag(Tag, Box<Core>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    SubstAtom(Symbol, Atom),
    SubstApp(CoreDecl),
    VarUse(Symbol),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreApp {
    pub func: Atom,
    pub args: Vec<Atom>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreDecl {
    pub func: Symbol,
    pub args: Vec<Symbol>,
    pub body: Box<Core>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreLet {
    pub decls: Vec<CoreDecl>,
    pub body: Box<Core>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreOpr {
    pub prim: Prim,
    pub args: Vec<Atom>,
    pub bind: Symbol,
    pub cont: Box<Core>, 
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreCase {
    pub arg: Atom,
    pub brs: Vec<Core>, 
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreRec {
    pub flds: Vec<Atom>,
    pub bind: Symbol,
    pub cont: Box<Core>, 
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreSel {
    pub arg: Atom,
    pub idx: usize,
    pub bind: Symbol,
    pub cont: Box<Core>, 
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ByteCode {
    MkPair,
    Head,
    Tail,

    Move(usize, Atom),
    Swap(usize, usize),
    Jump(Atom),
    JumpTrue(Atom),
    JumpFalse(Atom),

    IAdd(Atom, Atom, Atom),
    ISub(Atom, Atom, Atom),
    IMul(Atom, Atom, Atom),
    IDiv(Atom, Atom, Atom),
    INeg(Atom, Atom),
    BNot(Atom, Atom),
    Halt(Atom),
}

