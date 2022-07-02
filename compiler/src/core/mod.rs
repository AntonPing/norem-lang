use crate::ast::Prim;
use crate::symbol::Symbol;

pub mod visitor;
pub mod cps_trans;
pub mod opt1;
pub mod clos_conv;
pub mod reg_alloc;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Atom {
    Var(Symbol),
    Glob(Symbol),
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
    Opr(CoreOpr),
    Let(CoreLet),
    Fix(CoreFix),
    Case(CoreCase),
    //Cond(CoreCond),
    Rec(CoreRec),
    Set(CoreSet),
    Get(CoreGet),
    Halt(Atom),
    Tag(Tag, Box<Core>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    SubstAtom(Symbol, Atom),
    SubstSetGet(Symbol,usize,Atom),
    SubstApp(CoreDecl),
    VarFree(Vec<Symbol>),
    VarFreeAfter(Vec<Symbol>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreApp {
    pub func: Atom,
    pub args: Vec<Atom>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreOpr {
    pub prim: Prim,
    pub args: Vec<Atom>,
    pub bind: Symbol,
    pub cont: Box<Core>, 
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreDecl {
    pub func: Symbol,
    pub args: Vec<Symbol>,
    pub body: Box<Core>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreLet {
    pub decl: CoreDecl,
    pub cont: Box<Core>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreFix {
    pub decls: Vec<CoreDecl>,
    pub cont: Box<Core>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct CoreCase {
    pub arg: Atom,
    pub brs: Vec<Core>, 
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreRec {
    pub size: usize,
    pub bind: Symbol,
    pub cont: Box<Core>, 
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreSet {
    pub rec: Atom,
    pub idx: usize,
    pub arg: Atom, 
    pub cont: Box<Core>, 
}

#[derive(Clone, Debug, PartialEq)]
pub struct CoreGet {
    pub rec: Atom,
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

