use std::{rc::Rc, collections::HashMap, ops::Deref, cell::RefCell};

use norem_frontend::symbol::{*, SymTable};
use hashbag::HashBag;

#[derive(Clone, Debug, PartialEq)]
pub enum LamExpr {
    Var(Symbol),
    Lam(Symbol,Rc<LamExpr>),
    App(Rc<LamExpr>,Rc<LamExpr>),
    Int(i64),
    Add,
}

impl LamExpr {
    pub fn get_args(&self) -> Vec<Symbol> {
        let mut args = Vec::new();
        let mut with = self;
        while let LamExpr::Lam(x, e) = with {
            args.push(*x);
            with = e;
        }
        args
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CombExpr {
    Var(Symbol),
    Glob(Symbol),
    Comb(Vec<Symbol>,Rc<CombExpr>),
    App(Rc<CombExpr>, Rc<CombExpr>),
    Int(i64),
    Add,
}


/*
impl CombExpr {
    pub fn dump_code(&self) -> Vec<ByteCode> {
        let mut code = Vec::new();
        

    }
}
*/


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
