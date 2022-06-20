use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use hashbag::HashBag;

use crate::ast::*;
use crate::parser::*;
use crate::pretty::*;
use crate::symbol::*;
use crate::utils::*;

type Subst = HashMap<Symbol, Type>;
type VarSet = HashBag<Symbol>;

impl Type {
    fn new(ty: Type) -> Type {
        unimplemented!()
    }

    fn ftv(&self) -> VarSet {
        let mut result = HashBag::new();
        let mut stack = Vec::<&Type>::new();
        stack.push(self);

        while let Some(elem) = stack.pop() {
            match elem {
                Type::Lit(_) => {}
                Type::Var(x) => {
                    result.insert(*x);
                }
                Type::Arr(ty1, ty2) => {
                    stack.push(ty1);
                    stack.push(ty2);
                }
                Type::App(cons, args) => {
                    for arg in args {
                        stack.push(arg);
                    }
                }
            }
        }
        result
    }

    fn subst(&self, sub: &Subst) -> Type {
        match self {
            Type::Lit(_) => self.clone(),
            Type::Var(x) => {
                if let Some(t) = sub.get(&x) {
                    t.clone().subst(sub)
                } else {
                    self.clone()
                }
            }
            Type::Arr(t1, t2) => Type::Arr(Ptr(t1.subst(sub)), Ptr(t2.subst(sub))),
            Type::App(cons, args) => {
                let new_args = args.iter().map(|arg| Ptr(arg.subst(sub))).collect();
                Type::App(*cons, new_args)
            }
        }
    }
    fn occur_check(&self, x: &Symbol) -> bool {
        self.ftv().contains(x) > 0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Scheme {
    Mono(Type),
    Poly(usize, Type),
}

impl Scheme {
    fn ftv(&self) -> VarSet {
        match self {
            Scheme::Mono(ty) => ty.ftv(),
            Scheme::Poly(len, ty) => {
                let mut set = ty.ftv();
                for x in 0..*len {
                    set.take_all(&Symbol::Forall(x));
                }
                set
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum EnvHistory {
    // in env is such key, old data covered
    Update(Symbol, Scheme),
    // in env no such key, symbol was inserted
    Insert(Symbol),
    // symbol was deleted from env
    Delete(Symbol, Scheme),
    // symbol not in env, nothing happened
    Nothing,
}

#[derive(Clone, Debug, PartialEq)]
struct Environment {
    current: HashMap<Symbol, Scheme>,
    freevars: HashBag<Symbol>,
    history: Vec<EnvHistory>,
}

impl Environment {
    fn new() -> Environment {
        Environment {
            current: HashMap::new(),
            freevars: HashBag::new(),
            history: Vec::new(),
        }
    }

    fn debug(&self) {
        dbg!(self);
        for x in &self.freevars {
            dbg!(x);
        }
    }

    fn lookup(&self, x: Symbol) -> Option<&Scheme> {
        self.current.get(&x)
    }

    fn contains(&self, x: Symbol) -> bool {
        self.freevars.contains(&x) > 0
    }

    fn add_scheme(&mut self, sc: &Scheme) {
        for (x, n) in sc.ftv() {
            self.freevars.insert_many(x, n);
        }
    }

    fn remove_scheme(&mut self, sc: &Scheme) {
        for (x, n) in sc.ftv() {
            if let Some((_, m)) = self.freevars.get(&x) {
                self.freevars.take_all(&x);
                assert!(m >= n);
                if m > n {
                    self.freevars.insert_many(x, m - n);
                }
            } else {
                panic!("symbol not found in freevars!");
            }
        }
    }

    fn update(&mut self, k: Symbol, v: Scheme) -> usize {
        let back = self.backup();
        self.add_scheme(&v);
        if let Some(old) = self.current.insert(k, v) {
            self.remove_scheme(&old);
            self.history.push(EnvHistory::Update(k, old));
        } else {
            self.history.push(EnvHistory::Insert(k));
        }
        back
    }

    fn delete(&mut self, k: Symbol) -> usize {
        let back = self.backup();
        if let Some(old) = self.current.remove(&k) {
            self.remove_scheme(&old);
            self.history.push(EnvHistory::Delete(k, old));
        } else {
            self.history.push(EnvHistory::Nothing);
        }
        back
    }
    fn backup(&self) -> usize {
        self.history.len()
    }

    fn recover(&mut self, mark: usize) {
        //println!("recover {} from {}",mark,self.history.len());
        for _ in mark..self.history.len() {
            if let Some(row) = self.history.pop() {
                match row {
                    EnvHistory::Update(x, sc) => {
                        self.add_scheme(&sc);
                        let r = self.current.insert(x, sc);
                        self.remove_scheme(&r.unwrap());
                        //assert!(r.is_some());
                    }
                    EnvHistory::Insert(x) => {
                        let r = self.current.remove(&x);
                        self.remove_scheme(&r.unwrap());
                        //assert!(r.is_some());
                    }
                    EnvHistory::Delete(x, sc) => {
                        self.add_scheme(&sc);
                        let r = self.current.insert(x, sc);
                        assert!(r.is_none());
                    }
                    EnvHistory::Nothing => {
                        // Well, Nothing...
                    }
                }
            } else {
                panic!("history underflow!");
            }
        }
    }
}

pub struct Infer {
    env: Environment,
    subst: Subst,
    table: Mut<SymTable>,
    err_msg: Vec<String>,
}

impl Infer {
    pub fn new(table: Mut<SymTable>) -> Infer {
        Infer {
            env: Environment::new(),
            subst: HashMap::new(),
            table,
            err_msg: Vec::new(),
        }
    }

    fn newvar(&mut self) -> Symbol {
        self.table.borrow_mut().gensym()
    }

    fn update_subst(&mut self, key: Symbol, value: Type) {
        let map = HashMap::new();
        map.insert(key, value);

        for (k, v) in self.subst {
            let new_v = v.subst(&map);
            self.subst.insert(k, new_v);
        }

        self.subst.insert(key, value);
    }

    fn generalize(&mut self, ty: &Type) -> Scheme {
        let mut sub = HashMap::new();
        let mut len = 0;
        for (x, _) in ty.ftv() {
            if !self.env.contains(x) {
                sub.insert(x, Type::Var(Symbol::Forall(len)));
                len += 1;
            } else {
                dbg!(x);
            }
        }
        if len == 0 {
            Scheme::Mono(ty.clone())
        } else {
            Scheme::Poly(len, ty.subst(&sub))
        }
    }

    fn instantiate(&mut self, sc: &Scheme) -> Type {
        match sc {
            Scheme::Mono(ty) => ty.clone(),
            Scheme::Poly(n, ty) => {
                let mut sub = HashMap::new();
                for x in 0..*n {
                    let new = Type::Var(self.newvar());
                    sub.insert(Symbol::Forall(x), new);
                }
                ty.subst(&sub)
            }
        }
    }

    fn unify(&mut self, ty1: &Type, ty2: &Type) -> Result<(), String> {
        match (ty1, ty2) {
            (Type::Var(x), _) => {
                if ty2.occur_check(&x) {
                    Err("Occur check failed!".to_string())
                } else {
                    self.update_subst(*x, *ty2);
                    self.subst.insert(*x, *ty2);
                    Ok(())
                }
            }
            (_, Type::Var(x)) => {
                if ty1.occur_check(&x) {
                    Err("Occur check failed!".to_string())
                } else {
                    self.update_subst(*x, *ty1);
                    self.subst.insert(*x, *ty1);
                    Ok(())
                }
            }
            (Type::Lit(a), Type::Lit(b)) => {
                if a != b {
                    return Err(format!("Can't unify {:?} and {:?}!", a, b));
                }
                Ok(())
            }
            (Type::Arr(a1, a2), Type::Arr(b1, b2)) => {
                self.unify(a1.deref(), b1.deref())?;
                self.unify(a2.deref(), b2.deref())?;
                Ok(())
            }
            (Type::App(cons1, args1), Type::App(cons2, args2)) => {
                if cons1 != cons2 {
                    return Err(format!("diffent constructor {:?} {:?}!", cons1, cons2));
                }

                for (arg1, arg2) in args1.iter().zip(args2.iter()) {
                    self.unify(arg1.deref(), arg2.deref())?;
                }
                Ok(())
            }
            (a, b) => Err(format!("Can't unify {:?} and {:?}!", a, b)),
        }
    }
    fn infer(&mut self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::Lit(lit) => Ok(Type::Lit(lit_value_type(lit.clone()))),
            Expr::Var(sym) => {
                match sym {
                    Symbol::Var(_) => {
                        let res = self.env.lookup(*sym).map(|x| x.clone());
                        if let Some(sc) = res {
                            let ty = self.instantiate(&sc);
                            Ok(ty)
                        } else {
                            Err("Variable not in the environment!".to_string())
                        }
                    }
                    Symbol::Gen(_) => {
                        // maybe somthing???
                        unimplemented!()
                    }
                    Symbol::Forall(_) => {
                        unimplemented!()
                    }
                }
            }
            Expr::Lam(args, body) => {
                let old = self.env.backup();

                let vec = Vec::new();

                for arg in args {
                    let new = Type::Var(self.newvar());
                    vec.push(new);
                    self.env.update(*arg, Scheme::Mono(new));
                }

                let body_typ = self.infer(body)?;

                self.env.recover(old);

                Ok(vec
                    .iter()
                    .rev()
                    .fold(body_typ, |iter, x| Type::Arr(Ptr(*x), Ptr(iter))))
            }
            Expr::App(func, args) => {
                let tyf = self.infer(func)?;

                let mut tyargs = Vec::new();
                for arg in args {
                    let ty = self.infer(arg)?;
                    tyargs.push(ty);
                }

                let tyres = Type::Var(self.newvar());

                let tyfunc = tyargs
                    .iter()
                    .rev()
                    .fold(tyres, |iter, tyarg| Type::Arr(Ptr(*tyarg), Ptr(iter)));
                self.unify(&tyf, &tyfunc)?;

                Ok(tyres)
            }
            Expr::Let(decls, body) => {
                let old = self.env.backup();

                for decl in decls {
                    self.infer_delc(decl)?;
                }
                let ty = self.infer(body)?;

                self.env.recover(old);

                Ok(ty)
            }
            //Expr::Case((), ())
            _ => {
                unimplemented!()
            }
        }
    }

    fn infer_delc(&mut self, decl: &Decl) -> Result<(), String> {
        match decl {
            Decl::Val(ValDecl { name, args, body }) => {
                let old = self.env.backup();

                for arg in args {
                    let new = Type::Var(self.newvar());
                    self.env.update(*arg, Scheme::Mono(new));
                }

                let ty = self.infer(body)?;
                self.env.recover(old);

                let res = self.generalize(&ty);
                self.env.update(*name, res);

                Ok(())
            }
            _ => {
                unimplemented!()
            }
        }
    }

    pub fn infer_top(&mut self, exp: &Expr) -> Result<Scheme, String> {
        let old = self.env.backup();

        let ty = self.infer(&exp)?;
        let sub = self.cons.solve()?;

        self.env.recover(old);

        let sc = self.generalize(&ty.subst(&sub));
        Ok(sc)
    }
}
