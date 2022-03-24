use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

use hashbag::HashBag;

use crate::utils::*;
use crate::symbol::*;
use crate::parser::*;
use crate::pretty::*;
use crate::ast::*;

type Subst = HashMap<Symbol,TypeVar>;
type VarSet = HashBag<Symbol>;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeVar {
    Var(Symbol),
    Arr(Box<TypeVar>,Box<TypeVar>),
    Lit(LitType),
}

impl TypeVar {
    fn new(ty: Type) -> TypeVar {
        unimplemented!()
    }
    
    fn ftv(&self) -> VarSet {
        let mut result = HashBag::new();
        let mut stack = Vec::<&TypeVar>::new();
        stack.push(self);

        while let Some(elem) = stack.pop() {
            match elem {
                TypeVar::Var(x) => {
                    result.insert(*x);
                }
                TypeVar::Lit(_) => {}
                TypeVar::Arr(ty1,ty2) => {
                    stack.push(ty1);
                    stack.push(ty2);
                }
            }
        }
        result
    }

    fn subst(&self, sub: &Subst) -> TypeVar {
        match self {
            TypeVar::Lit(_) => {
                self.clone()
            }
            TypeVar::Var(x) => {
                if let Some(t) = sub.get(&x) {
                    t.clone().subst(sub)
                } else {
                    self.clone()
                }
            }
            TypeVar::Arr(t1,t2) => {
                TypeVar::Arr(
                    Box::new(t1.subst(sub)),Box::new(t2.subst(sub)))
            }
        }
    }
    fn occur_check(&self, x: &Symbol) -> bool {
        self.ftv().contains(x) > 0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Scheme {
    Mono(TypeVar),
    Poly(usize,TypeVar),
}

impl Scheme {
    fn new(ty: TypeVar) -> Scheme {
        Scheme::Mono(ty)
    }
    fn ftv(&self) -> VarSet {
        match self {
            Scheme::Mono(ty) => { ty.ftv() }
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


struct Constraints {
    cons: Vec<(TypeVar,TypeVar)>,
}

impl Constraints {
    fn new() -> Constraints {
        Constraints { cons: Vec::new() }
    }
    fn push(&mut self, t1: &TypeVar, t2: &TypeVar) {
        self.cons.push((t1.clone(),t2.clone()));
    }
    fn solve(&mut self) -> Result<Subst,String> {
        let mut map = HashMap::new();
        while let Some((ty1,ty2)) = self.cons.pop() {
            let ty1 = ty1.subst(&map);
            let ty2 = ty2.subst(&map);
            match (&ty1,&ty2) {
                (TypeVar::Var(x), _) => {
                    if ty2.occur_check(&x) {
                        return Err("Occur check failed!".to_string());
                    } else {
                        map.insert(*x, ty2);
                    }
                }
                (_, TypeVar::Var(x)) => {
                    if ty1.occur_check(&x) {
                        return Err("Occur check failed!".to_string());
                    } else {
                        map.insert(*x, ty1);
                    }
                }
                (TypeVar::Lit(a), TypeVar::Lit(b)) => {
                    if a == b {
                        continue;
                    } else {
                        return Err(format!("Can't unify {:?} and {:?}!",a,b));
                    }
                }
                (&TypeVar::Arr(_,_), &TypeVar::Arr(_,_)) => {

                    match (ty1, ty2) {
                        (TypeVar::Arr(a1,a2),
                        TypeVar::Arr(b1,b2)) => {
                        
                            self.cons.push((*a1,*b1));
                            self.cons.push((*a2,*b2));
                        }
                        _ => { unimplemented!() }
                    }

                }
                (a,b) => {
                    return Err(format!("Can't unify {:?} and {:?}!",a,b))
                }
            }
        }
        Ok(map)
    }
}


enum EnvHistory {
    // in env no such key, symbol was inserted
    Insert(Symbol),
    // in env is such key, old data covered
    Update(Symbol,Scheme),
    // symbol not in env, nothing happened
    Nothing,
    // symbol was deleted from env
    Delete(Symbol,Scheme),
}

struct Environment {
    current: HashMap<Symbol,Scheme>,
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

    fn lookup(&self, x: Symbol) -> Option<&Scheme> {
        self.current.get(&x)
    }

    fn contains(&self, x: Symbol) -> bool {
        self.freevars.contains(&x) > 0
    }

    fn add_scheme(&mut self, sc: &Scheme) {
        for (x,n) in sc.ftv() {
            self.freevars.insert_many(x, n);
        }
    }

    fn remove_scheme(&mut self, sc: &Scheme) {
        for (x,n) in sc.ftv() {
            if let Some((_,m)) = self.freevars.get(&x) {
                self.freevars.take_all(&x);
                self.freevars.insert_many(x, m - n);
            } else {
                self.freevars.insert_many(x, n);
            }
        }
    }

    fn update(&mut self, k: Symbol, v: Scheme) -> usize {
        self.add_scheme(&v);
        if let Some(old) = self.current.insert(k,v) {
            self.remove_scheme(&old);
            self.history.push(EnvHistory::Update(k,old));
        } else {
            self.history.push(EnvHistory::Insert(k));
        }
        self.history.len()
    }

    fn delete(&mut self, k: Symbol) -> usize {
        if let Some(old) = self.current.remove(&k) {
            self.remove_scheme(&old);
            self.history.push(EnvHistory::Delete(k,old));
        } else {
            self.history.push(EnvHistory::Nothing);
        }
        self.history.len()

    }
    fn backup(&self) -> usize {
        self.history.len()
    }

    fn recover(&mut self, mark: usize) {
        for _ in mark..self.history.len() {
            if let Some(row) = self.history.pop() {
                match row {
                    EnvHistory::Delete(x,sc) => {
                        self.add_scheme(&sc);
                        let r = self.current.insert(x, sc);
                        assert!(r.is_none());
                    }
                    EnvHistory::Insert(x) => {
                        let r = self.current.remove(&x);
                        assert!(r.is_some());
                    }
                    EnvHistory::Update(x,sc) => {
                        self.add_scheme(&sc);
                        let r = self.current.insert(x,sc);
                        self.remove_scheme(&r.unwrap());
                        //assert!(r.is_some());
                    }
                    EnvHistory::Nothing => {
                        // Well, Nothing...
                    }
                }
            } else {
                panic!("Can't Be!")
            }
        }
    }
}

pub struct Infer<'src> {
    env: Environment,
    cons: Constraints,
    table: Rc<RefCell<SymTable<'src>>>,
    err_msg: Vec<String>,
}

impl<'src> Infer<'src> {
    pub fn new(table: Rc<RefCell<SymTable<'src>>>) -> Infer<'src> {
        Infer {
            env: Environment::new(),
            cons: Constraints::new(),
            table: table,
            err_msg: Vec::new()
        }
    }

    /*
    fn update(&mut self, x: Symbol, sc: Scheme) -> Option<(Symbol,Scheme)> {
        let old = self.env.insert(x, sc);
        old.map(|sc| (x,sc))
    }

    fn recover(&mut self, record: Option<(Symbol,Scheme)>) {
        if let Some((x,sc)) = record {
            self.env.insert(x, sc);
        }
    }
    */


    fn newvar(&mut self) -> Symbol {
        self.table.borrow_mut().gensym()
    }

    fn generalize(&mut self, ty: &TypeVar) -> Scheme {
        let mut sub = HashMap::new();
        let mut len = 0;
        for (x,_) in ty.ftv() {
            if !self.env.contains(x) {
                sub.insert(x, TypeVar::Var(Symbol::Forall(len)));
                len += 1;
            }
        }
        if len == 0 {
            Scheme::Mono(ty.clone())
        } else {
            Scheme::Poly(len, ty.subst(&sub))
        }
    }
    fn instantiate(&mut self, sc: &Scheme) -> TypeVar {
        match sc {
            Scheme::Mono(ty) => { ty.clone() }
            Scheme::Poly(n, ty) => {
                let mut sub = HashMap::new();
                for x in 0..*n {
                    let new = TypeVar::Var(self.newvar());
                    sub.insert(Symbol::Forall(x), new);
                }
                ty.subst(&sub)
            }
        }
    }
    fn unify(&mut self, ty1: &TypeVar, ty2: &TypeVar) {
        self.cons.push(ty1,ty2);
    }
    fn infer(&mut self, expr: &Expr) -> Result<TypeVar,String> {
        match expr {
            Expr::Lit(lit) => {
                Ok(TypeVar::Lit(lit_value_type(lit.clone())))
            }
            Expr::Var(sym) => {
                match sym {
                    Symbol::Var(_) => {
                        let res = self.env.lookup(*sym).map(|x|x.clone());
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
            Expr::Lam(x,e) => {
                let x2 = TypeVar::Var(self.newvar());
                let old = self.env.update(*x, Scheme::Mono(x2.clone()));
                let t2 = self.infer(e)?;
                self.env.recover(old);
                Ok(TypeVar::Arr(Box::new(x2),Box::new(t2)))
            }
            Expr::App(ea,eb) => {
                let ta = self.infer(ea)?;
                let tb = self.infer(eb)?;
                let tc = TypeVar::Var(self.newvar());
                self.unify(&ta, &TypeVar::Arr(Box::new(tb),Box::new(tc.clone())));
                Ok(tc)
            },
            Expr::Let(decls, body) => {
                let old = self.env.backup();
                
                for decl in decls {
                    self.infer_delc(decl)?;
                }
                let ty = self.infer(body)?;
                
                self.env.recover(old);

                Ok(ty)
            }
            _ => {
                unimplemented!()
            }
        }
    }

    fn infer_delc(&mut self, decl: &DeclKind) -> Result<(),String> {
        match decl {
            DeclKind::Val(ValDecl{
                name,args,body,span
            }) => {

                let old = self.env.backup();

                for arg in args {
                    let new = TypeVar::Var(self.newvar());
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


    pub fn infer_top(&mut self, exp: &Expr) -> Result<Scheme,String> {
        let old = self.env.backup();
        
        let ty = self.infer(&exp)?;
        let sub = self.cons.solve()?;
        
        self.env.recover(old);

        let sc = self.generalize(&ty.subst(&sub));
        Ok(sc)
    }
}