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
    Let(Vec<(Symbol,Box<CoreExpr>)>,Box<CoreExpr>),
    
}
