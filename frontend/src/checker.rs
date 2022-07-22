use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::hash::Hash;

use norem_utils::diagnostic::Diagnostic;
use norem_utils::env_map::EnvMap;
use norem_utils::interner::InternStr;
use norem_utils::position::Span;
use norem_utils::symbol::Symbol;

use crate::ast::*;


impl Type {
    pub fn subst(self, map: &HashMap<Symbol,Type>) -> Type {
        match self {
            Type::Lit(lit) => {
                Type::Lit(lit)
            }
            Type::Var(x) => {
                if let Some(ty) = map.get(&x) {
                    ty.clone()
                } else {
                    Type::Var(x)
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
            Type::Temp(x) => {
                Type::Temp(x)
            }
        }
    }
}

type InferResult<T> = Result<T,Diagnostic>;
type InferFunc<T> = fn(&mut Infer) -> InferResult<T>;

pub struct Infer {
    var_env: EnvMap<Symbol,Scheme>,
    cons_env: EnvMap<Symbol,Scheme>,
    val_env: EnvMap<Symbol,DeclVal>,
    data_env: EnvMap<Symbol,DeclData>,
    type_env: EnvMap<Symbol,DeclType>,
    opr_env: EnvMap<Symbol,DeclOpr>,
    arena: Vec<Option<Type>>,
    error: Vec<Diagnostic>,
}

impl Infer {
    pub fn new() -> Infer {
        Infer {
            var_env: EnvMap::new(),
            cons_env: EnvMap::new(),
            val_env: EnvMap::new(),
            data_env: EnvMap::new(),
            type_env: EnvMap::new(),
            opr_env: EnvMap::new(),
            arena: Vec::new(),
            error: Vec::new(),
        }
    }

    pub fn tempvar(&mut self) -> usize {
        self.arena.push(None);
        self.arena.len() - 1
    }

    pub fn is_unbind(&self, n: usize) -> bool {
        self.arena[n].is_none()
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

    pub fn unify(&mut self, ty1: &Type, ty2: &Type) -> InferResult<()> {
        println!("unify {:?} ~ {:?}",ty1,ty2);
        match (ty1,ty2) {
            (Type::Temp(x), Type::Temp(y))
            if *x == *y => {
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
                        .line(format!("Can't unify {} and {}!",a,b)))
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
                    .line(format!("Can't unify {} and {}!",ty1,ty2)))
            }
        }
    }

    pub fn merge_type(&self, ty: &Type) -> Type {
        match ty {
            Type::Lit(lit) => {
                Type::Lit(*lit)
            }
            Type::Var(x) => {
                Type::Var(*x)
            }
            Type::Temp(x) => {
                if let Some(ref res) = self.arena[*x] {
                    self.merge_type(res)
                } else {
                    Type::Temp(*x)
                }
            }
            Type::Var(_) => {
                unreachable!()
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
        }
    }

    /*
    pub fn occur_check(&self, x: &InternStr, ty: &Type) -> bool {
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
                Type::Temp(x) => {
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
            }
        }

        vec
    }
    */

    fn generalize_aux(
        ty: Type,
        count: &mut usize,
        map: &mut HashMap<usize,Symbol>,
        free: &mut HashSet<Symbol>,
    ) -> Type {
        match ty {
            Type::Lit(_) => {
                ty
            }
            Type::Var(x) => {
                free.insert(x);
                ty
            }
            Type::Arr(a, b) => {
                let a = Self::generalize_aux(*a, count, map, free);
                let b = Self::generalize_aux(*b, count, map, free);
                Type::Arr(Box::new(a), Box::new(b))
            }
            Type::App(a, b) => {
                let a = Self::generalize_aux(*a, count, map, free);
                let b = Self::generalize_aux(*b, count, map, free);
                Type::App(Box::new(a), Box::new(b))
            }
            Type::Temp(x) => {
                if let Some(sym) = map.get(&x) {
                    Type::Var(*sym)
                } else {
                    let old = *count;
                    *count += 1;
                    map.insert(x, Symbol::Gen('t', old));
                    Type::Var(Symbol::Gen('t', old))
                }
            }
        }
    }

    pub fn generalize(&mut self, ty: Type) -> Scheme {
        let mut count = 0;
        let mut map = HashMap::new();
        let mut free = HashSet::new();

        let ty = Self::generalize_aux(ty, &mut count, &mut map, &mut free);

        let args: Vec<Symbol> = map.values()
            .chain(free.iter())
            .copied().collect();

        if args.len() == 0 {
            Scheme::Mono(ty)
        } else {
            Scheme::Poly(args, ty)
        }
    }

    pub fn instantiate(&mut self, sc: Scheme) -> Type {
        match sc {
            Scheme::Mono(ty) => {
                ty
            }
            Scheme::Poly(args, ty) => {
                let sub = args.into_iter()
                    .map(|arg| {
                        let new = self.tempvar();
                        (arg, Type::Temp(new))
                    })
                    .collect();
                ty.subst(&sub)
            }
        }
    }
    
    pub fn infer_expr(&mut self, expr: &Expr) -> InferResult<Type> {
        match expr {
            Expr::Lit(expr) => self.infer_lit(expr),
            Expr::Prim(expr) => self.infer_prim(expr),
            Expr::Var(expr) => self.infer_var(expr),
            Expr::Lam(expr) => self.infer_lam(expr),
            Expr::App(expr) => self.infer_app(expr),
            Expr::Chain(_) => todo!(),
            Expr::Let(_) => todo!(),
            Expr::Case(_) => todo!(),
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
        macro_rules! uniop {
            ($ty: expr) => {
                Type::Arr(
                    Box::new($ty),
                    Box::new($ty))
            };
        }
        macro_rules! binop {
            ($ty: expr) => {
                Type::Arr(
                    Box::new($ty),
                    Box::new(Type::Arr(
                        Box::new($ty), 
                        Box::new($ty))))
            };
        }
        let ty = match expr.prim {
            Prim::IAdd => binop!(Type::Lit(LitType::Int)),
            Prim::ISub => binop!(Type::Lit(LitType::Int)),
            Prim::IMul => binop!(Type::Lit(LitType::Int)),
            Prim::IDiv => binop!(Type::Lit(LitType::Int)),
            Prim::INeg => uniop!(Type::Lit(LitType::Int)),
            Prim::BNot => uniop!(Type::Lit(LitType::Bool)),
        };

        Ok(ty)
    }

    pub fn infer_var(&mut self, expr: &ExprVar) -> InferResult<Type> {
        let ExprVar { name, span: _ } = expr;

        match name {
            Symbol::Var(_) => {
                if let Some(sc) = self.var_env.get(name) {
                    let sc = sc.clone();
                    Ok(self.instantiate(sc))
                } else {
                    Err(Diagnostic::error("Variable not Bound"))
                }
            }
            /*
            Symbol::Cons(x) => {
                if let Some(sc) = self.cons_env.get(name) {
                    Ok(self.instantiate(&sc))
                } else {
                    Err(Diagnostic::error("Constructor not Bound"))
                }
            }
            */
            _ => {
                unreachable!()
            }
        }
    }

    pub fn infer_lam(&mut self, expr: &ExprLam) -> InferResult<Type> {
        
        let ExprLam { args, body, span: _ } = expr;

        let backup = self.var_env.backup();

        let intro: Vec<Type> = args.iter()
            .map(|arg| {
                let newty = Type::Temp(self.tempvar());
                self.var_env.insert(*arg, Scheme::Mono(newty.clone()));
                newty
            })
            .collect();

        let body_ty = self.infer_expr(body)?;
        
        self.var_env.recover(backup);

        let res = intro.into_iter().rev()
            .fold(body_ty, |acc, ty| {
                Type::Arr(Box::new(ty),Box::new(acc))
            });

        Ok(res)
    }

    pub fn infer_app(&mut self, expr: &ExprApp) -> InferResult<Type> {

        let ExprApp { func, args, span: _ } = expr;

        let func = self.infer_expr(func)?;
        
        let args: Vec<Type> = args.iter()
            .map(|arg| self.infer_expr(arg))
            .collect::<InferResult<Vec<Type>>>()?;

        let temp = self.tempvar();
        
        let func_ty = args.into_iter().rev()
            .fold(
                Type::Temp(temp), 
                |acc, ty| {
                    Type::Arr(Box::new(ty), Box::new(acc))
                }
            );
        
        self.unify(&func, &func_ty)?;

        Ok(Type::Temp(temp))
    }

    pub fn infer_let(&mut self, expr: &ExprLet) -> InferResult<Type> {
        let ExprLet { decls, body, span: _ } = expr;
        
        let backup_var = self.var_env.backup();
        let backup_cons = self.cons_env.backup();
        let backup_val = self.val_env.backup();
        let backup_data = self.data_env.backup();
        let backup_type = self.type_env.backup();
        let backup_opr = self.opr_env.backup();
        
        for decl in decls {
            match decl {
                Decl::Val(_) => todo!(),
                Decl::Data(_) => todo!(),
                Decl::Type(_) => todo!(),
                Decl::Opr(_) => todo!(),
            }
        }

        let body_ty = self.infer_expr(body)?;

        self.var_env.recover(backup_var);
        self.cons_env.recover(backup_cons);
        self.val_env.recover(backup_val);
        self.data_env.recover(backup_data);
        self.type_env.recover(backup_type);
        self.opr_env.recover(backup_opr);

        Ok(body_ty)
    }

    pub fn infer_decl_val(&mut self, decl: &DeclVal) -> InferResult<()> {
        let DeclVal { name, args, body, span: _ } = decl;

        self.val_env.insert(*name, decl.clone());
        
        let _body_ty = if args.is_empty() {
            self.infer_expr(body)?;
        } else {
            // todo: inline this for better performance
            let expr = Expr::Lam(ExprLam {
                args: args.clone(),
                body: Box::new(body.clone()),
                span: Span::dummy(),
            });
            self.infer_expr(&expr)?;
        };

        Ok(())
    }

    pub fn infer_decl_type(&mut self, decl: &DeclType) -> InferResult<()> {
        let DeclType { name, args, typ, span: _ } = decl;
        
        self.type_env.insert(*name, decl.clone());

        


        Ok(())
    }

}


/*

impl ExprVisitor for Infer {
    type Ident1 = InternStr;
    type Extra1 = Span;
    type Ident2 = InternStr;
    type Extra2 = (Span,Option<Type>);

    fn visit_ident(&mut self, ident: Self::Ident1) -> Self::Ident2 {
        ident
    }

    fn visit_extra(&mut self, ext: Self::Extra1) -> Self::Extra2 {
        (ext, None)
    }

    fn visit_expr_var(
        &mut self,
        expr: ExprVar<Self::Ident1,Self::Extra1>,
    ) -> ExprVar<Self::Ident2,Self::Extra2> {
        
        if expr.name.as_ref().chars().nth(0).unwrap().is_uppercase() {
            if let Some(sc) = self.cons_env.get(&expr.name) {
                let sc = sc.clone();
                let ty = self.instantiate(sc);
                let ExprVar { name, span: ext } = expr;
                ExprVar { name, span: (ext, Some(ty))}
            } else {
                self.error.push(
                    Diagnostic::error("Constructor not Bound")
                );
                self.walk_expr_var(expr)
            }
        } else {
            if let Some(sc) = self.var_env.get(&expr.name) {
                let sc = sc.clone();
                let ty = self.instantiate(sc);
                let ExprVar { name, span: ext } = expr;
                ExprVar { name, span: (ext, Some(ty))}
            } else {
                self.error.push(
                    Diagnostic::error("Variable not Bound")
                );
                self.walk_expr_var(expr)
            }
        }
    }

    fn visit_expr_lit(
        &mut self,
        expr: ExprLit<Self::Extra1>,
    ) -> ExprLit<Self::Extra2> {
        let ExprLit { lit, span: ext } = expr;
        let ty = match expr.lit {
            LitVal::Int(_) => Type::Lit(LitType::Int),
            LitVal::Real(_) => Type::Lit(LitType::Real),
            LitVal::Bool(_) => Type::Lit(LitType::Bool),
            LitVal::Char(_) => Type::Lit(LitType::Char),
        };

        ExprLit { lit, span: (ext, Some(ty)) }
    }

    fn visit_expr_prim(
        &mut self,
        expr: ExprPrim<Self::Extra1>
    ) -> ExprPrim<Self::Extra2> {

        let ExprPrim { prim, span: ext } = expr;

        macro_rules! uniop {
            ($ty: expr) => {
                Type::Arr(
                    Box::new($ty),
                    Box::new($ty))
            };
        }

        macro_rules! binop {
            ($ty: expr) => {
                Type::Arr(
                    Box::new($ty),
                    Box::new(Type::Arr(
                        Box::new($ty), 
                        Box::new($ty))))
            };
        }

        let ty = match &prim {
            Prim::IAdd => binop!(Type::Lit(LitType::Int)),
            Prim::ISub => binop!(Type::Lit(LitType::Int)),
            Prim::IMul => binop!(Type::Lit(LitType::Int)),
            Prim::IDiv => binop!(Type::Lit(LitType::Int)),
            Prim::INeg => uniop!(Type::Lit(LitType::Int)),
            Prim::BNot => uniop!(Type::Lit(LitType::Bool)),
        };

        ExprPrim { prim, span: (ext, Some(ty)) }
    }

    fn visit_expr_lam(
        &mut self,
        expr: ExprLam<Self::Ident1,Self::Extra1>,
    ) -> ExprLam<Self::Ident2,Self::Extra2> {

        let ExprLam { args, body, span: ext } = expr;

        let backup = self.var_env.backup();

        let intro: Vec<Type> = args.iter()
            .map(|arg| {
                let newty = Type::Var(MayTemp::Temp(self.tempvar()));
                self.var_env.insert(*arg, Scheme::Mono(newty.clone()));
                newty
            })
            .collect();

        let body = Box::new(self.visit_expr(*body));
        
        self.var_env.recover(backup);

        let res = if let Some(bodyty) = &body.get_ext_ref().1 {
            Some(intro.into_iter()
                .fold(bodyty.clone(), |acc, ty| {
                    Type::Arr(Box::new(ty),Box::new(acc))
            }))
        } else {
            None
        };

        ExprLam { args, body, span: (ext, res) }
    }

    fn visit_expr_app(
        &mut self,
        expr: ExprApp<Self::Ident1,Self::Extra1>,
    ) -> ExprApp<Self::Ident2,Self::Extra2> {
        let expr = self.walk_expr_app(expr);

        let functy = expr.func.get_ext_ref().1;
        let argsty = expr.args.iter()
            .map(|arg| arg.get_ext_ref().1)
            .collect();

        


        //Ok(Type::Temp(res))
        
    }

}
*/


#[test]
fn checker_test() -> Result<(),String> {
    todo!()
}