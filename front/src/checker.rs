use std::collections::HashMap;
use std::hash::Hash;

use crate::utils::*;
use crate::ast::*;

pub trait Checkable {
    fn check(&self, chk: &mut Checker) -> Result<(),String>;
}

pub trait Typable {
    fn infer(&self, chk: &mut Checker) -> Result<Type,String>;
}

#[derive(Clone, Debug, PartialEq)]
enum EnvOp<K,V> {
    // has such key, old value covered
    Update(K,V),
    // has no such key, the key was inserted
    Insert(K),
    // symbol was deleted from env
    Delete(K,V),
    // symbol not in env, no need to delete
    Nothing,
}

#[derive(Clone, Debug)]
pub struct Env<K,V> {
    context: HashMap<K,V>,
    history: Vec<EnvOp<K,V>>,
}

impl<K,V> Env<K,V> where K: Eq + Hash + Clone {
    pub fn new() -> Env<K,V> {
        Env {
            context: HashMap::new(),
            history: Vec::new()
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        self.context.contains_key(key)
    }

    pub fn lookup(&self, key: &K) -> Option<&V> {
        self.context.get(key)
    }

    pub fn update(&mut self, key: K, val: V) {
        if let Some(old) = self.context.insert(key.clone(),val) {
            self.history.push(EnvOp::Update(key,old));
        } else {
            self.history.push(EnvOp::Insert(key));
        }
    }

    pub fn delete(&mut self, key: K) {
        if let Some(old) = self.context.remove(&key) {
            self.history.push(EnvOp::Delete(key,old));
        } else {
            self.history.push(EnvOp::Nothing);
        }
    }

    pub fn backup(&self) -> usize {
        self.history.len()
    }

    pub fn recover(&mut self, mark: usize) {
        for _ in mark..self.history.len() {
            if let Some(op) = self.history.pop() {
                match op {    
                    EnvOp::Update(k,v) => {
                        let r = self.context.insert(k,v);
                        assert!(r.is_some());
                    }
                    EnvOp::Insert(k) => {
                        let r = self.context.remove(&k);
                        assert!(r.is_some());
                    }
                    EnvOp::Delete(k,v) => {
                        let r = self.context.insert(k,v);
                        assert!(r.is_none());
                    }
                    EnvOp::Nothing => {
                        // Well, Nothing...
                    }
                }
            } else {
                panic!("history underflow!");
            }
        }
    }
}


pub struct Checker {
    var_env: Env<Symbol,Scheme>,
    cons_env: Env<Symbol,(Variant,Type)>,
    type_env: Env<Symbol,Type>,
    arena: Vec<Option<Type>>
}

impl Checker {
    pub fn new() -> Checker {
        Checker {
            var_env: Env::new(),
            cons_env: Env::new(),
            type_env: Env::new(),
            arena: Vec::new(),
        }
    }

    pub fn cons_env(&mut self) -> &mut Env<Symbol,(Variant,Type)> {
        &mut self.cons_env
    }

    pub fn type_env(&mut self) -> &mut Env<Symbol,Type> {
        &mut self.type_env
    }

    pub fn var_env(&mut self) -> &mut Env<Symbol,Scheme> {
        &mut self.var_env
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
        if let Some(sc) = self.var_env.lookup(x) {
            Ok(sc.clone())
        } else {
            Err("variable not found in scope!".to_string())
        }
    }

    pub fn is_unbind(&self, n: usize) -> bool {
        self.arena[n].is_none()
    }

    pub fn occur_check(&self, x: &Symbol, ty: &Type) -> bool {
        match ty {
            Type::Lit(_) => { false }
            Type::Cons(_) => { false }
            Type::Var(y) => {
                x == y
            }
            Type::Arr(ty1,ty2) => {
                self.occur_check(x, ty1)
                && self.occur_check(x, ty2)
            }
            Type::App(ty1, ty2) => {
                self.occur_check(x, ty1)
                && self.occur_check(x, ty2)
            }
            Type::Temp(n) => {
                if let Some(ty2) = &self.arena[*n] {
                    self.occur_check(x, &ty2)
                } else {
                    false
                }
            }
        }
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