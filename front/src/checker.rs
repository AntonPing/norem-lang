use std::collections::HashMap;

use crate::utils::*;
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
    fn infer(&self, chk: &mut Checker) -> Result<Type,String>;
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
    arena: Vec<Option<Type>>
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

    pub fn assign(&mut self, n: usize, ty: Type) -> Result<(),String> {
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

    fn freevar(&self, ty: &Type) -> Vec<Symbol> {
        let mut vec = Vec::new();
        let mut stack = Vec::new();
        stack.push(ty.clone());

        while let Some(ty) = stack.pop() {
            match ty {
                Type::Lit(_) => {}
                Type::Cons(_) => {}
                Type::Var(x) => {
                    vec.push(x);
                }
                Type::Arr(ty1,ty2) => {
                    stack.push(*ty1);
                    stack.push(*ty2);
                }
                Type::App(cons, args) => {
                    todo!();
                }
                Type::Temp(n) => {
                    /*
                    if self.is_unbind(n) {
                        let mut str = n.to_string();
                        str.insert(0, '#');
                        vec.push(str.into());
                    }
                    */
                }
                //Type::Poly((), ())
            }
        }

        vec
    }

    pub fn generalize(&mut self, ty: &Type) -> Scheme {
        let mut args = self.freevar(ty);
        let mut len = 0;

        if args.len() == 0 {
            Scheme::Mono(ty.clone())
        } else {
            Scheme::Poly(args, ty.clone())
        }
    }

    pub fn instantiate(&mut self, sc: &Scheme) -> Type {
        match sc {
            Scheme::Mono(ty) => { ty.clone() }
            Scheme::Poly(args, ty) => {
                let mut sub = HashMap::new();
                for arg in args {
                    let new = self.newvar();
                    sub.insert(arg.clone(), Type::Temp(new));
                }
                ty.subst(&sub)
            }
        }
    }
    pub fn unify(&mut self, ty1: &Type, ty2: &Type) -> Result<(),String> {
        println!("unify {:?} ~ {:?}",ty1,ty2);
        match (ty1,ty2) {
            (Type::Temp(x), Type::Temp(y)) if *x == *y => {
                Ok(())
            }
            (Type::Temp(x), ty) => {
                self.assign(*x,ty.clone())?;
                Ok(())
            }
            (ty, Type::Temp(x)) => {
                self.assign(*x,ty.clone())?;
                Ok(())
            }
            (Type::Lit(a), Type::Lit(b)) => {
                if a != b {
                    Err(format!("Can't unify {:?} and {:?}!",a,b))
                } else {
                    Ok(())
                }
            }
            (Type::Arr(a1,b1),
                Type::Arr(a2,b2)) => {

                self.unify(a1, a2)?;
                self.unify(b1, b2)?;
                Ok(())
            }
            (ty1, ty2) => {
                Err(format!("Can't unify {:?} and {:?}!", ty1, ty2))
            }
        }
    }

    pub fn merge_type(&self, ty: &Type) -> Type {
        match ty {
            Type::Lit(_) | Type::Cons(_) | Type::Var(_) => {
                ty.clone()
            }
            Type::Arr(ty1,ty2) => {
                let res_ty1 = self.merge_type(ty1);
                let res_ty2 = self.merge_type(ty2);
                Type::Arr(Box::new(res_ty1), Box::new(res_ty2))
            }
            Type::App(cons, args) => {
                todo!();
            }
            Type::Temp(n) => {
                if let Some(ref res) = self.arena[*n] {
                    self.merge_type(res)
                } else {
                    ty.clone()
                }
            }
            
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