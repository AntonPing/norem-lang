use std::collections::HashMap;

use crate::utils::*;
use crate::types::*;
use crate::ast::*;

/*
pub trait Checkable {
    type Record;
    fn check_enter(&self, chk: &mut Checker) -> Self::Record;
    fn check_body(&self, chk:&mut Checker) -> Result<(), String>;
    fn check_quit(&self, chk: &mut Checker, rec: Self::Record);
    fn check(&self, chk: &mut Checker) -> Result<(),String> {
        let rec = self.check_enter(chk);
        self.check_body(chk)?;
        self.check_quit(chk, rec);
        Ok(())
    }
}
*/

pub trait Typable {
    fn infer(&self, chk: &mut Checker) -> Result<TypeVar,String>;
    /*
    fn infer(&self, chk: &mut Checker) -> Result<TypeVar,String> {
        let rec = self.check_enter(chk);
        self.check_body(chk)?;
        let ty = self.infer_body(chk)?;
        self.check_quit(chk, rec);
        Ok(ty)
    }
    */
}

pub struct Checker {
    var_env: MultiSet<Symbol>,
    cons_env: MultiSet<Symbol>,
    type_env: MultiSet<Symbol>,
    pub environment: HashMap<Symbol,Scheme>,
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

    pub fn assign(&mut self, n: usize, ty: TypeVar) -> Result<(),String> {
        if let Some(ty2) = self.arena[n].clone() {
            self.unify(&ty, &ty2)?;
            Ok(())
        } else {
            self.arena[n] = Some(ty);
            Ok(())
        }
    }

    pub fn lookup(&self, x: &Symbol) -> Result<Scheme,String> {
        if let Some(sc) = self.environment.get(x) {
            Ok(sc.clone())
        } else {
            Err("variable not found in scope!".to_string())
        }
    }

    pub fn is_unbind(&self, n: usize) -> bool {
        self.arena[n].is_none()
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

    pub fn generalize(&mut self, ty: &TypeVar) -> Scheme {
        let mut args = self.freevar(ty);
        let mut len = 0;

        if args.len() == 0 {
            Scheme::Mono(ty.clone())
        } else {
            Scheme::Poly(args, ty.clone())
        }
    }

    pub fn instantiate(&mut self, sc: &Scheme) -> TypeVar {
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
        println!("unify {:?} ~ {:?}",ty1,ty2);
        match (ty1,ty2) {
            (TypeVar::Var(x), TypeVar::Var(y)) 
                if *x == *y => {
                Ok(())
            }
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

    pub fn merge_type(&self, ty: &TypeVar) -> Type {
        match ty {
            TypeVar::Lit(lit) => {
                Type::Lit(*lit)
            }
            TypeVar::Var(x) => {
                if let Some(ref res) = self.arena[*x] {
                    self.merge_type(res)
                } else {
                    Type::Var(*x)
                }
            }
            TypeVar::Arr(ty1,ty2) => {
                let res_ty1 = self.merge_type(ty1);
                let res_ty2 = self.merge_type(ty2);
                Type::Arr(Box::new(res_ty1), Box::new(res_ty2))
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
}

#[test]
fn checker_test() -> Result<(),String> {
    use crate::parser::*;
    let text = "fn f g x => f x (g x)";
    let mut par = Parser::new(text);
    let res = Expr::parse(&mut par)?;
    par.eof()?;

    println!("term: {:?}", res);

    let mut chk = Checker::new();
    let ty = res.infer(&mut chk)?;
    println!("typeVar: {:?}", ty);

    let ty2 = chk.merge_type(&ty);
    println!("type: {:#?}", ty2);

    Ok(())
}