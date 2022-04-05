use std::rc::Rc;
use std::collections::HashMap;

use norem_frontend::symbol::*;

#[derive(Copy,Clone, Debug, PartialEq)]
pub enum Prim {
    IAdd,
    ISub,
    IMul,
    IDiv,
    INeg,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LamExpr {
    Lit(Value),
    Var(Symbol),
    Lam(Vec<Symbol>,Rc<LamExpr>),
    App(Rc<LamExpr>,Vec<Rc<LamExpr>>),
    Record(Vec<LamExpr>),
    Select(usize,Rc<LamExpr>),
    Prim(Prim),
}


#[derive(Copy,Clone, Debug, PartialEq)]
pub enum Value {
    //Var(Symbol),
    //Label(Symbol),
    Int(i64),
    Real(f64),
    Bool(bool),
    //MetaVar(Symbol),
    //Func(Symbol,Rc<CpsExpr>),
}

type TopDecl = HashMap<Symbol,SuperComb>;

pub struct SuperComb {
    name: Symbol,
    args: Vec<Symbol>,
    body: Rc<CombExpr>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CombExpr {
    Lit(Value),
    Arg(Symbol),
    Glob(Symbol),
    App(Rc<CombExpr>, Vec<Rc<CombExpr>>),
    Prim(Prim),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ByteCode {
    Push(usize),
    Pop(usize),
    //PushArgs(usize),
    //PopArgs(usize),

    PushInt(i64),
    PushReal(f64),
    
    Jump(usize),
    JumpTrue(usize),
    JumpFalse(usize),
    App,
    IntAdd,

    Call(usize),
    GlobCall(Symbol),
    TopCall,
    Ret,
}
