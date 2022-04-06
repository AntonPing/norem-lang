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
    Lam(Vec<(Symbol,Type)>,Rc<LamExpr>),
    App(Rc<LamExpr>,Vec<Rc<LamExpr>>),
    Record(Vec<LamExpr>),
    Select(usize,Rc<LamExpr>),
    Prim(Prim),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Real,
    Bool,
    Star,
    Func(Box<Type>,Box<Type>),
}

impl Type {
    pub fn arity(&self) -> usize {
        if let Type::Func(f, x) = self {
            f.arity() + 1
        } else {
            0
        }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub enum CombExpr {
    Lit(Value),
    Arg(Symbol),
    Glob(Symbol),
    App(Rc<CombExpr>, Vec<Rc<CombExpr>>),
    Record(Vec<CombExpr>),
    Select(usize,Rc<CombExpr>),
    Prim(Prim),
}

#[derive(Clone, Debug, PartialEq)]
pub struct SuperComb {
    name: Symbol,
    args: Vec<(Symbol,Type)>,
    body: Rc<CombExpr>,
}

impl SuperComb {
    pub fn arity(&self) -> usize {
        self.args.len()
    }
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ByteCode {
    Push(usize),
    Pop(usize),
    PushInt(i64),
    PushReal(f64),
    PushBool(bool),
    PushPtr(usize),
    PushPtrHole(Symbol),

    Jump(usize),
    JumpTrue(usize),
    JumpFalse(usize),
    
    CallHole(Symbol),
    Call(usize),
    CallArg(usize),
    Ret,
    Halt,
    
    MkPair,
    Head,
    Tail,

    IAdd,
    ISub,
    IMul,
    IDiv,
    INeg,
}
