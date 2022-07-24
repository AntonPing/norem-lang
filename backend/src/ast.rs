use super::*;

/// prim that used in opr
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Prim {
    IAdd,
    ISub,
    IMul,
    IDiv,
    INeg,
    BNot,
}

/// prim that used in brs
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BrsPrim {
    Switch,
}

impl Prim {
    pub fn is_pure(&self) -> bool {
        match self {
            Prim::IAdd => true,
            Prim::ISub => true,
            Prim::IMul => true,
            Prim::IDiv => true,
            Prim::INeg => true,
            Prim::BNot => true,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Atom {
    Var(Symbol),
    Label(Symbol),
    Int(i64),
    Real(f64),
    Bool(bool),
    Char(char),
}

impl Atom {
    pub fn unwrap_var(&self) -> Symbol {
        if let Atom::Var(sym) = self {
            *sym
        } else {
            panic!("failed to unwrap atom!");
        }
    }
}


use std::fmt;

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Atom::Var(x) => write!(f,"{x}"),
            Atom::Label(x) => write!(f,"@{x}"),
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
    Brs(ExprBrs),
    App(ExprApp),
    Rec(ExprRec),
    Set(ExprSet),
    Get(ExprGet),
    Tag(Tag,Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Decl {
    pub func: Symbol,
    pub args: Vec<Symbol>,
    pub body: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLet {
    pub decls: Vec<Decl>,
    pub cont: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprOpr {
    pub prim: Prim,
    pub args: Vec<Atom>,
    pub binds: Vec<Symbol>,
    pub cont: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprBrs {
    pub prim: BrsPrim,
    pub args: Vec<Atom>,
    pub brs: Vec<Expr>, 
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprApp {
    pub func: Atom,
    pub args: Vec<Atom>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprRec {
    pub size: usize,
    pub bind: Symbol,
    pub cont: Box<Expr>, 
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprSet {
    pub rec: Atom,
    pub idx: usize,
    pub arg: Atom,
    pub cont: Box<Expr>, 
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprGet {
    pub rec: Atom,
    pub idx: usize,
    pub bind: Symbol,
    pub cont: Box<Expr>, 
}

#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    SubstAtom(Symbol, Atom),
    SubstSetGet(Symbol,usize,Atom),
    SubstApp(Box<Decl>),
    VarFree(Vec<Symbol>),
    VarFreeAfter(Vec<Symbol>),
}

/*

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

*/