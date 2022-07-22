use super::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Prim {
    IAdd,
    ISub,
    IMul,
    IDiv,
    INeg,
    BNot,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Atom {
    Var(Symbol),
    Label(Symbol),
    Index(usize),
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
            Atom::Label(x) => write!(f,"@{x}"),
            Atom::Index(x) => write!(f,"#{x}"),
            Atom::Int(x) => write!(f,"{x}"),
            Atom::Real(x) => write!(f,"{x}"),
            Atom::Bool(x) => write!(f,"{x}"),
            Atom::Char(x) => write!(f,"{x}"),
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Let(ExprLet),
    Opr(ExprOpr),
    App(ExprApp),
    Tag(Tag,Box<Expr>),
}


#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    SubstAtom(Symbol, Atom),
    SubstSetGet(Symbol,usize,Atom),
    SubstApp(Box<Decl>),
    VarFree(Vec<Symbol>),
    VarFreeAfter(Vec<Symbol>),
}


#[derive(Clone, Debug, PartialEq)]
pub struct ExprOpr {
    pub prim: Prim,
    pub args: Vec<Atom>,
    pub binds: Vec<Symbol>,
    pub conts: Vec<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprApp {
    pub func: Atom,
    pub args: Vec<Atom>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Decl {
    pub func: Symbol,
    pub args: Vec<Symbol>,
    pub body: Expr,
    // recursive reference information
    pub rec_ref: Vec<Symbol>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLet {
    pub decls: Vec<Decl>,
    pub cont: Box<Expr>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ByteCode {

    Move(Atom, usize),
    Jump(Atom),
    Halt(Atom),

    IAdd(Atom, Atom, usize),
    ISub(Atom, Atom, usize),
    IMul(Atom, Atom, usize),
    IDiv(Atom, Atom, usize),
    INeg(Atom, usize),
    BNot(Atom, usize),
}

pub struct ByteCodeBlock {
    pub func: Symbol,
    pub args: usize,
    pub body: Vec<ByteCode>,
}

