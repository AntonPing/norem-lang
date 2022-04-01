use std::rc::Rc;

use norem_frontend::symbol::*;

pub enum LamExpr {
    Var(Symbol),
    Lam(Symbol,Rc<LamExpr>),
    App(Rc<LamExpr>,Rc<LamExpr>),
    Int(i64),
    Add,
}

pub enum CombExpr {
    Comb(Vec<ByteCode>),
    App(Rc<CombExpr>, Rc<CombExpr>),
}

pub enum ByteCode {
    App(u8), // arg < 64
    Push(u8), // arg < 64
    Pop(u8), // arg < 64
    Call(u8),
    Ret,
}


pub struct SuperComb {
    args: usize,
    code: Vec<ByteCode>,
}

