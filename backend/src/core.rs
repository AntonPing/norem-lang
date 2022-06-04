use crate::symbol::Symbol;


#[derive(Copy, Clone, PartialEq)]
pub enum Atom {
    Var(Symbol),
    Glob(Symbol),
    Reg(usize),
    Int(i64),
    Real(f64),
    Bool(bool),
    Char(char),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Prim {
    IAdd,
    ISub,
    IMul,
    IDiv,
    INeg,
    BNot,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LExpr {
    Val(Atom),
    Lam(Vec<Symbol>,Box<LExpr>),
    App(Box<LExpr>,Vec<LExpr>),
    Uniop(Prim,Box<LExpr>),
    Binop(Prim,Box<LExpr>,Box<LExpr>),
    Switch(usize,Vec<LExpr>),
    Ifte(Atom,Box<LExpr>,Box<LExpr>),
    Record(Vec<LExpr>),
    Select(usize,Box<LExpr>),
}

#[derive(Clone, PartialEq)]
pub struct Def<T> {
    pub func: Symbol,
    pub args: Vec<Symbol>,
    pub body: Box<T>,
}

#[derive(Clone, PartialEq)]
pub enum CExpr {
    App(Atom, Vec<Atom>),
    Let(Def<CExpr>,Box<CExpr>),
    Fix(Vec<Def<CExpr>>,Box<CExpr>),
    Uniop(Prim,Atom,Symbol,Box<CExpr>),
    Binop(Prim,Atom,Atom,Symbol,Box<CExpr>),
    Switch(Atom,Vec<CExpr>),
    Ifte(Atom,Box<CExpr>,Box<CExpr>),
    Record(Vec<Atom>,Symbol,Box<CExpr>),
    Select(usize,Vec<CExpr>),
    Halt(Atom),
    Tag(Tag,Box<CExpr>),
}

#[derive(Clone, PartialEq)]
pub enum Tag {
    SubstAtom(Symbol,Atom),
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ByteCode {
    MkPair,
    Head,
    Tail,

    Move(usize,Atom),
    Swap(usize,usize),
    Jump(Atom),
    JumpTrue(Atom),
    JumpFalse(Atom),

    IAdd(Atom,Atom,Atom),
    ISub(Atom,Atom,Atom),
    IMul(Atom,Atom,Atom),
    IDiv(Atom,Atom,Atom),
    INeg(Atom,Atom),
    BNot(Atom,Atom),
    Halt(Atom),
}


