use std::collections::HashMap;


use crate::utils::*;
use crate::ast::*;

#[derive(Clone, Debug, PartialEq)]
pub enum CoreExpr {
    Int(i64),
    Real(f64),
    Bool(bool),
    Char(char),
    Var(Symbol),
    Lam(Symbol,Box<CoreExpr>),
    App(Box<CoreExpr>,Box<CoreExpr>),
    Record(Vec<CoreExpr>),
    Select(usize,Box<CoreExpr>),
    Switch(Box<CoreExpr>,Vec<CoreExpr>),
    Let(Vec<(Symbol,Box<CoreExpr>)>,Box<CoreExpr>),
}

pub trait TransCore {
    fn translate(&self, trs: &mut Translator) -> Result<CoreExpr,String>;
}

pub struct Translator {
    var_env: Env<Symbol,Scheme>,
    cons_env: Env<Symbol,(Variant,Type)>,
    type_env: Env<Symbol,Type>,
    arena: Vec<Option<Type>>
}