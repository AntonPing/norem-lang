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
            TypeVar::Lit(lit) => {
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
    fn occur_check(&self, x: Symbol) -> bool {
        self.ftv().contains(&x) > 0
    }
}

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

    fn rename(&self, table: SymTable) -> TypeVar {
        match self {
            Scheme::Mono(ty) => { ty.clone() }
            Scheme::Poly(xs, ty) => {
                let mut sub = HashMap::new();
                for x in xs {
                    let newvar = table.gensym();
                    sub.insert(Symbol::Forall(*x), TypeVar::Var(newvar));
                }
                ty.subst(&sub)
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
            match (ty1,ty2) {
                (TypeVar::Var(x), _) => {
                    if ty2.occur_check(x) {
                        return Err("Occur check failed!".to_string());
                    } else {
                        map.insert(x, ty2.clone());
                    }
                }
                (_, TypeVar::Var(x)) => {
                    if ty1.occur_check(x) {
                        return Err("Occur check failed!".to_string());
                    } else {
                        map.insert(x, ty1.clone());
                    }
                }
                (TypeVar::Lit(a), TypeVar::Lit(b)) => {
                    if a == b {
                        continue;
                    } else {
                        return Err(format!("Can't unify {:?} and {:?}!",a,b));
                    }
                }
                (TypeVar::Arr(a1,a2),
                    TypeVar::Arr(b1,b2)) => {
                    self.cons.push((*a1,*b1));
                    self.cons.push((*a2,*b2));
                }
                (a,b) => {
                    return Err(format!("Can't unify {:?} and {:?}!",a,b))
                }
            }
        }
        Ok(map)
    }
}


struct Infer<'src> {
    env: HashMap<Symbol,Scheme>,
    cons: Constraints,
    table: Rc<RefCell<SymTable<'src>>>,
    err_msg: Vec<String>,
}

impl<'src> Infer<'src> {
    fn new(table: Rc<RefCell<SymTable<'src>>>) -> Infer<'src> {
        Infer {
            env: HashMap::new(),
            cons: Constraints::new(),
            table: table,
            err_msg: Vec::new()
        }
    }

    fn update(&mut self, x: Symbol, sc: Scheme) -> Option<(Symbol,Scheme)> {
        let old = self.env.insert(x, sc);
        old.map(|sc| (x,sc))
    }

    fn recover(&mut self, record: Option<(Symbol,Scheme)>) {
        if let Some((x,sc)) = record {
            self.env.insert(x, sc);
        }
    }

    fn newvar(&mut self) -> TypeVar {
        TypeVar::Var(self.table.borrow_mut().gensym())
    }

    fn generalize(&mut self, ty: &TypeVar) -> Scheme {
        let mut sub = HashMap::new();
        let mut len = 0;
        for (x,_) in ty.ftv() {
            if !self.env.contains_key(&x) {
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
                    let new = self.newvar();
                    sub.insert(Symbol::Forall(x),new);
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
            &Expr::Lit(lit) => {
                Ok(TypeVar::Lit(lit_value_type(lit)))
            }
            Expr::Var(sym) => {
                match sym {
                    Symbol::Var(x) => {
                        if let Some(sc) = self.env.get(sym) {
                            Ok(self.instantiate(sc))
                        } else {
                            Err("Variable not in the environment!".to_string())
                        }
                    }
                    Symbol::Gen(x) => {
                        Ok(exp.clone())
                    }
                }
                
            }
            Expr::Lam(x,e) => {
                let x2 = self.newvar();
                let old = self.update(*x, Scheme::Mono(x2));
                let t2 = self.infer(e)?;
                self.recover(old);
                Ok(TypeVar::Arr(Box::new(x2),Box::new(t2)))
            }
            Expr::App(ea,eb) => {
                let ta = self.infer(ea)?;
                let tb = self.infer(eb)?;
                let tc = self.newvar();
                self.unify(&ta, 
                    &TypeVar::Arr(Box::new(tb),Box::new(tc.clone())));
                Ok(tc)
            },
            Expr::Let(decl, body) => {
                let ta = self.infer(ea)?;
                let sc = self.generalize(&ta);
                let mark = self.env.update(*x, &sc);
                let tb = self.infer(eb)?;
                self.env.recover(mark);
                Ok(tb)
            }
        }
    }
    fn infer_top(&mut self, exp: &ExprRef) -> Result<Scheme,String> {
        let mark = self.env.backup();
        let ty = self.infer(&exp)?;
        let sub = self.cons.solve()?;
        self.env.recover(mark);


        let sc = self.generalize(&ty.subst(&sub));
        Ok(sc)
    }
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
        match 


    }

}