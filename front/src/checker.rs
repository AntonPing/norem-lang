use std::collections::HashMap;

use crate::utils::*;
use crate::types::*;
use crate::ast::*;

pub trait Checkable {
    fn check(&self, chk: &mut Checker) -> Result<(),String>;
}

pub trait Typable {
    fn infer(&self, chk: &mut Checker) -> Result<TypeVar,String>;
}

pub struct Checker {
    var_env: MultiSet<Symbol>,
    cons_env: MultiSet<Symbol>,
    type_env: MultiSet<Symbol>,
    pub environment: HashMap<Symbol,TypeVar>,
    arena: Vec<Option<TypeVar>>
}

impl Checker {
    pub fn new() -> Checker {
        Checker {
            var_env: MultiSet::new(),
            cons_env: MultiSet::new(),
            type_env: MultiSet::new(),
            environment: HashMap::new(),
            arena: Vec::new(),
        }
    }

    pub fn newvar(&mut self) -> usize {
        self.arena.push(None);
        self.arena.len() - 1
    }

    pub fn is_unbind(&self, n: usize) -> bool {
        self.arena[n].is_none()
    }

    pub fn assign(&mut self, n: usize, ty: TypeVar) -> Result<(),String> {
        if let Some(_) = self.arena[n] {
            Err("Can't unify!".to_string())
        } else {
            self.arena[n] = Some(ty);
            Ok(())
        }
    }

    pub fn lookup(&self, x: &Symbol) -> Result<TypeVar,String> {
        if let Some(ty) = self.environment.get(x) {
            Ok(ty.clone())
        } else {
            Err("variable not found in scope!".to_string())
        }
    }

    fn freevar(&self, ty: &TypeVar) -> Vec<usize> {
        let mut vec = Vec::new();
        let mut stack = Vec::new();
        stack.push(ty);

        while let Some(ty) = stack.pop() {
            match ty {
                TypeVar::Lit(_) => {}
                TypeVar::Var(x) => {
                    if self.is_unbind(*x) {
                        vec.push(*x);
                    }
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

        vec
    }

    fn generalize(&mut self, ty: &TypeVar) -> Scheme {
        let mut args = self.freevar(ty);
        let mut len = 0;

        if args.len() == 0 {
            Scheme::Mono(ty.clone())
        } else {
            Scheme::Poly(args, ty.clone())
        }
    }

    fn instantiate(&mut self, sc: &Scheme) -> TypeVar {
        match sc {
            Scheme::Mono(ty) => { ty.clone() }
            Scheme::Poly(args, ty) => {
                let mut sub = HashMap::new();
                for arg in args {
                    let new = self.newvar();
                    sub.insert(*arg, TypeVar::Var(new));
                }
                ty.subst(&sub)
            }
        }
    }
    pub fn unify(&mut self, ty1: &TypeVar, ty2: &TypeVar) -> Result<(),String> {
        match (ty1,ty2) {
            (TypeVar::Var(x), ty) => {
                self.assign(*x,ty.clone())?;
                Ok(())
            }
            (ty, TypeVar::Var(x)) => {
                self.assign(*x,ty.clone())?;
                Ok(())
            }
            (TypeVar::Lit(a), TypeVar::Lit(b)) => {
                if a != b {
                    Err(format!("Can't unify {:?} and {:?}!",a,b))
                } else {
                    Ok(())
                }
            }
            (TypeVar::Arr(a1,b1),
                TypeVar::Arr(a2,b2)) => {

                self.unify(a1, a2)?;
                self.unify(b1, b2)?;
                Ok(())
            }
            (ty1, ty2) => {
                Err(format!("Can't unify {:?} and {:?}!", ty1, ty2))
            }
        }
    }
}

#[test]
fn checker_test() {

}