use std::collections::HashMap;
use std::ops::Mul;

use crate::utils::*;
use crate::ast::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum LitType {
    Int,
    Real,
    Bool,
    Char,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeVar {
    Lit(LitType),
    Var(usize),
    Arr(Box<TypeVar>, Box<TypeVar>),
    //App(Box<TypeVar>, Box<TypeVar>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Scheme {
    Mono(TypeVar),
    Poly(Vec<usize>,TypeVar),
}

impl Scheme {
    pub fn ftv(&self) -> MultiSet<usize> {
        match self {
            Scheme::Mono(ty) => { ty.ftv() }
            Scheme::Poly(args, ty) => {
                let mut set = ty.ftv();
                for arg in args {
                    set.remove_all(arg);
                }
                set
            }
        }
    }
}

impl TypeVar {
    pub fn ftv(&self) -> MultiSet<usize> {
        let mut set = MultiSet::new();
        let mut stack = Vec::<&TypeVar>::new();
        stack.push(self);

        while let Some(elem) = stack.pop() {
            match elem {
                TypeVar::Lit(_) => {}
                TypeVar::Var(x) => {
                    set.insert(*x);
                }
                TypeVar::Arr(ty1,ty2) => {
                    stack.push(ty1);
                    stack.push(ty2);
                }
                /*
                TypeVar::App(cons, args) => {
                    for arg in args {
                        stack.push(arg);
                    }
                }
                */
            }
        }
        set
    }


    pub fn subst(&self, sub: &HashMap<usize,TypeVar>) -> TypeVar {
        match self {
            TypeVar::Lit(_) => {
                self.clone()
            }
            TypeVar::Var(x) => {
                if let Some(t) = sub.get(&x) {
                    t.clone()
                } else {
                    self.clone()
                }
            }
            TypeVar::Arr(t1,t2) => {
                TypeVar::Arr(
                    Box::new(t1.subst(sub)),
                    Box::new(t2.subst(sub)))
            }
        }
    }
}