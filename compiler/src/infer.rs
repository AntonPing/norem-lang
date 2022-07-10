use std::collections::HashMap;
use std::hash::Hash;

use crate::utils::*;
use crate::error::*;
use crate::symbol::*;
use crate::ast::*;

impl Type {
    pub fn subst(&self, map: &HashMap<Symbol,Type>) -> Type {
        match self {
            Type::Lit(_) => {
                self.clone()
            }
            Type::Var(x) => {
                if let Some(ty) = map.get(&x) {
                    ty.clone()
                } else {
                    self.clone()
                }
            }
            Type::Arr(ty1, ty2) => {
                let ty1 = Box::new(ty1.subst(map));
                let ty2 = Box::new(ty2.subst(map));
                Type::Arr(ty1,ty2)
            }
            Type::App(ty1, ty2) => {
                let ty1 = Box::new(ty1.subst(map));
                let ty2 = Box::new(ty2.subst(map));
                Type::App(ty1,ty2)
            }
            Type::Rec(_) => {
                //todo
                self.clone()
            }
            Type::Temp(_) => {
                self.clone()
            }
        }
    }
}



type InferResult<T> = Result<T,Diagnostic>;
type InferFunc<T> = fn(&mut Infer) -> InferResult<T>;

pub struct Infer {
    var_env: HashMap<Symbol,Scheme>,
    cons_env: HashMap<Symbol,Scheme>,
    val_env: HashMap<Symbol,DeclVal>,
    data_env: HashMap<Symbol,DeclData>,
    type_env: HashMap<Symbol,DeclType>,
    opr_env: HashMap<Symbol,DeclOpr>,
    arena: Vec<Option<Type>>
}

impl Infer {
    pub fn new() -> Infer {
        Infer {
            var_env: HashMap::new(),
            cons_env: HashMap::new(),
            val_env: HashMap::new(),
            data_env: HashMap::new(),
            type_env: HashMap::new(),
            opr_env: HashMap::new(),
            arena: Vec::new(),
        }
    }

    pub fn tempvar(&mut self) -> usize {
        self.arena.push(None);
        self.arena.len() - 1
    }

    pub fn assign(&mut self, n: usize, ty: Type) -> InferResult<()> {
        if let Some(ty2) = self.arena[n].clone() {
            self.unify(&ty, &ty2)?;
            Ok(())
        } else {
            self.arena[n] = Some(ty);
            Ok(())
        }
    }

    pub fn lookup(&self, x: &Symbol) -> InferResult<Scheme> {
        if let Some(sc) = self.var_env.get(x) {
            Ok(sc.clone())
        } else {
            Err(Diagnostic::error("Variable not bound"))
        }
    }

    pub fn is_unbind(&self, n: usize) -> bool {
        self.arena[n].is_none()
    }

    pub fn occur_check(&self, x: &Symbol, ty: &Type) -> bool {
        match ty {
            Type::Lit(_) => {
                false
            }
            Type::Var(y) => {
                x == y
            }
            Type::Arr(ty1,ty2) => {
                self.occur_check(x, ty1) &&
                    self.occur_check(x, ty2)
            }
            Type::App(ty1, ty2) => {
                self.occur_check(x, ty1) &&
                    self.occur_check(x, ty2)
            }
            Type::Temp(n) => {
                if let Some(ty2) = &self.arena[*n] {
                    self.occur_check(x, &ty2)
                } else {
                    false
                }
            }
            Type::Rec(_) => {
                // todo
                false
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
                Type::Var(x) => {
                    vec.push(x);
                }
                Type::Arr(ty1,ty2) => {
                    stack.push(*ty1);
                    stack.push(*ty2);
                }
                Type::App(ty1,ty2) => {
                    stack.push(*ty1);
                    stack.push(*ty2);
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
                Type::Rec(_) => {} //todo
            }
        }

        vec
    }

    pub fn generalize(&mut self, ty: &Type) -> Scheme {
        let args = self.freevar(ty);

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
                    let new = self.tempvar();
                    sub.insert(arg.clone(), Type::Temp(new));
                }
                ty.clone().subst(&sub)
            }
        }
    }
    pub fn unify(&mut self, ty1: &Type, ty2: &Type) -> InferResult<()> {
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
                    Err(Diagnostic::error("Failed to infer type")
                        .desc(format!("Can't unify {:?} and {:?}!",a,b)))
                } else {
                    Ok(())
                }
            }
            (Type::Arr(a1,b1), Type::Arr(a2,b2)) => {
                self.unify(a1, a2)?;
                self.unify(b1, b2)?;
                Ok(())
            }
            (Type::App(a1,b1), Type::App(a2,b2)) => {
                self.unify(a1, a2)?;
                self.unify(b1, b2)?;
                Ok(())
            }
            (ty1, ty2) => {
                Err(Diagnostic::error("Failed to infer type")
                        .desc(format!("Can't unify {} and {}!",ty1,ty2)))
            }
        }
    }

    pub fn merge_type(&self, ty: &Type) -> Type {
        match ty {
            Type::Lit(_) | Type::Var(_) => {
                ty.clone()
            }
            Type::Arr(ty1,ty2) => {
                let ty1 = self.merge_type(ty1);
                let ty2 = self.merge_type(ty2);
                Type::Arr(Box::new(ty1), Box::new(ty2))
            }
            Type::App(ty1,ty2) => {
                let ty1 = self.merge_type(ty1);
                let ty2 = self.merge_type(ty2);
                Type::App(Box::new(ty1), Box::new(ty2))
            }
            Type::Temp(n) => {
                if let Some(ref res) = self.arena[*n] {
                    self.merge_type(res)
                } else {
                    ty.clone()
                }
            }
            Type::Rec(_) => {
                //todo
                ty.clone()
            }
            
        }
    }

    pub fn with_var_env<I,F,T>(
        &mut self,
        map: I,
        func: F
    ) -> InferResult<T> where 
        I: Iterator<Item = (Symbol,Scheme)>,
        F: Fn(&mut Self) -> InferResult<T>
    {
        let old: Vec<(Symbol,Scheme)> = map.into_iter()
            .map(|(k,v)| {
                if let Some(v2) = self.var_env.insert(k, v) {
                    Some((k,v2))
                } else {
                    None
                }
            })
            .filter_map(|x| x)
            .collect();
        
        let res = func(self);
        // wrong impl
        for (k,v) in old {
            self.var_env.insert(k, v);
        }

        res
    }

    pub fn infer(&mut self, expr: &Expr) -> InferResult<Type> {
        match expr {
            Expr::Lit(expr) => self.infer_lit(expr),
            Expr::Prim(expr) => self.infer_prim(expr),
            Expr::Var(expr) => self.infer_var(expr),
            Expr::Lam(expr) => self.infer_lam(expr),
            Expr::App(expr) => self.infer_app(expr),
            Expr::Chain(_) => todo!(),
            Expr::Let(_) => todo!(),
            Expr::Case(_) => todo!(),
            Expr::Block(_) => todo!(),
            Expr::Rec(_) => todo!(),
        }
    }

    pub fn infer_lit(&mut self, expr: &ExprLit) -> InferResult<Type> {
        match expr.lit {
            LitVal::Int(_) => Ok(Type::Lit(LitType::Int)),
            LitVal::Real(_) => Ok(Type::Lit(LitType::Real)),
            LitVal::Bool(_) => Ok(Type::Lit(LitType::Bool)),
            LitVal::Char(_) => Ok(Type::Lit(LitType::Char)),
        }
    }

    pub fn infer_prim(&mut self, expr: &ExprPrim) -> InferResult<Type> {
        // yep, this is "Int -> Int -> Int"
        let binop_int = Type::Arr(
            Box::new(Type::Lit(LitType::Int)),
            Box::new(Type::Arr(
                Box::new(Type::Lit(LitType::Int)), 
                Box::new(Type::Lit(LitType::Int)))));

        let uniop_int = Type::Arr(
            Box::new(Type::Lit(LitType::Int)),
            Box::new(Type::Lit(LitType::Int)));

        let uniop_bool = Type::Arr(
            Box::new(Type::Lit(LitType::Bool)),
            Box::new(Type::Lit(LitType::Bool)));

        match expr.prim {
            Prim::IAdd => Ok(binop_int),
            Prim::ISub => Ok(binop_int),
            Prim::IMul => Ok(binop_int),
            Prim::IDiv => Ok(binop_int),
            Prim::INeg => Ok(uniop_int),
            Prim::BNot => Ok(uniop_bool),
        }
    }

    pub fn infer_var(&mut self, expr: &ExprVar) -> InferResult<Type> {
        let sym = expr.ident;
        if sym.is_upper() {
            if let Some(sc) = self.cons_env.get(&sym) {
                // just to make ownership checker happy
                let sc = sc.clone(); 
                Ok(self.instantiate(&sc))
            } else {
                Err(Diagnostic::error("Constructor not Bound"))
            }
        } else {
            if let Some(sc) = self.var_env.get(&sym) {
                // just to make ownership checker happy
                let sc = sc.clone();
                Ok(self.instantiate(&sc))
            } else {
                Err(Diagnostic::error("Variable not Bound"))
            }

        }
    }

    pub fn infer_lam(&mut self, expr: &ExprLam) -> InferResult<Type> {

        let intro: Vec<usize> = expr.args.iter()
            .map(|_| self.tempvar())
            .collect();

        let map: Vec<(Symbol, Scheme)> = expr.args.iter()
            .zip(intro.iter())
            .map(|(arg,n)| (*arg, Scheme::Mono(Type::Temp(*n))))
            .collect();

        let res = self.with_var_env(map.into_iter(), |ti| {
            ti.infer(&*expr.body)
        })?;


        let res = intro.iter().fold(res, |acc, n| {
            Type::Arr(Box::new(Type::Temp(*n)), Box::new(acc))
        });

        Ok(res)
    }

    pub fn infer_app(&mut self, expr: &ExprApp) -> InferResult<Type> {

        let func = self.infer(&expr.func)?;
        
        let args: InferResult<Vec<Type>> = expr.args.iter()
            .map(|arg| self.infer(arg))
            .collect();

        let args = args?;

        let res = self.tempvar();
        
        let func_ty = args.into_iter().rev()
            .fold(
                Type::Temp(res), 
                |acc, ty| {
                    Type::Arr(Box::new(ty), Box::new(acc))
                }
            );
        
        self.unify(&func, &func_ty)?;

        Ok(Type::Temp(res))
    }

    pub fn infer_let(&mut self, expr: &ExprLet) -> InferResult<Type> {
        let mut var_old = Vec::new();
        


        for decl in &expr.decls {
            match decl {
                Decl::Val(_) => todo!(),
                Decl::Data(_) => todo!(),
                Decl::Type(_) => todo!(),
                Decl::Opr(_) => todo!(),
            }
        }
        todo!()
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

    let mut chk = Infer::new();
    let ty = res.infer(&mut chk)?;
    println!("typeVar: {:?}", ty);

    let ty2 = chk.merge_type(&ty);
    println!("type: {:#?}", ty2);

    Ok(())
}