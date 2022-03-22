use std::collections::HashMap;

use crate::symbol::*;
use crate::parser::*;
use crate::pretty::*;
use crate::ast::*;

pub enum Scheme {
    Mono(Type),
    Poly(Vec<usize>,Type),
}

pub struct inferState {
    variable: Vec<Type>,
    enviroment: HashMap<Symbol,Type>,
}


impl inferState {

    pub fn new() -> Self {
        inferState { variable: Vec::new(), enviroment: HashMap::new() }
    }

    pub fn dive<'a>(&self, ty: Type) -> Type {
        match ty {
            Type::Lit(_) => { ty }
            Type::Arr(_ , _) => { ty }
            Type::Var(Symbol::Gen(n)) => {
                self.variable[n].clone()
            }
            Type::Var(_) => { ty }
        }
    } 

    pub fn unify(ty1: &Type, ty2: &Type) {


    }

}