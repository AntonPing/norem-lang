use crate::ast::*;
use crate::symbol::Symbol;
use crate::utils::*;

pub struct Checker {
    val_env: MultiSet<Symbol>,
    data_env: MultiSet<Symbol>,
    cons_env: MultiSet<Symbol>,
    type_env: MultiSet<Symbol>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CheckError {
    ValNotBound(Symbol),
    DataNotBound(Symbol),
    ConsNotBound(Symbol),
    TypeNotBound(Symbol),
    FuncArgsNotUnique(Symbol),
    ErrorIn(&'static str, Span),
}

type CheckResult<T> = Result<T,Vec<CheckError>>;
type CheckFunc<T> = fn(&mut Checker) -> CheckResult<T>;

pub fn check_unique(xs: &Vec<Symbol>) -> Option<Symbol> {
    let mut ys: Vec<Symbol> = Vec::new();
    for x in xs {
        for y in &ys {
            if x == y {
                return Some(*x);
            }
        }
        ys.push(*x);
    }
    None
}

impl Checker {
    pub fn new() -> Checker {
        Checker {
            val_env: MultiSet::new(),
            data_env: MultiSet::new(),
            cons_env: MultiSet::new(),
            type_env: MultiSet::new(),
        }
    }

    pub fn local_val<F,T>(&mut self, sym: Symbol, func: F) -> CheckResult<T>
    where F: Fn(&mut Checker) -> CheckResult<T> {
        self.val_env.insert(sym);
        let res = func(self);
        self.val_env.remove(&sym);
        res
    }
    pub fn local_data<F,T>(&mut self, sym: Symbol, func: F) -> CheckResult<T>
    where F: Fn(&mut Checker) -> CheckResult<T> {
        self.data_env.insert(sym);
        let res = func(self);
        self.data_env.remove(&sym);
        res
    }

    pub fn scope_val(&self, sym: &Symbol) -> CheckResult<()> {
        if self.val_env.contains(sym) {
            Ok(())
        } else {
            Err(vec![CheckError::ValNotBound(*sym)])
        }
    }
    pub fn scope_data(&self, sym: &Symbol) -> CheckResult<()> {
        if self.data_env.contains(sym) {
            Ok(())
        } else {
            Err(vec![CheckError::DataNotBound(*sym)])
        }
    }
    pub fn scope_cons(&self, sym: &Symbol) -> CheckResult<()> {
        if self.cons_env.contains(sym) {
            Ok(())
        } else {
            Err(vec![CheckError::ConsNotBound(*sym)])
        }
    }
    pub fn scope_type(&self, sym: &Symbol) -> CheckResult<()> {
        if self.type_env.contains(sym) {
            Ok(())
        } else {
            Err(vec![CheckError::DataNotBound(*sym)])
        }
    }

    


    /*

    pub fn check_decl(&mut self, decl: &Decl) -> Result<(),String> {
        match decl {
            Decl::Val(val) => {
                self.check_val_decl(val)
            }
            Decl::Data(data) => {
                self.check_data_decl(data)
            }
            Decl::Type(typ) => {
                self.check_type_decl(typ)
            }
        }
    }

    pub fn check_val_decl(&mut self, decl: &ValDecl) -> Result<(),String> {
        let ValDecl { name, args, body } = decl;
        self.var_extend(*name);
        check_unique(args)?;       
        self.var_extend_many(args);
        self.check_expr(body)?;
        self.var_delete_many(args);
        Ok(())
    }

    pub fn check_data_decl(&mut self, decl: &DataDecl) -> Result<(),String> {
        let DataDecl { name, args, vars } = decl;
        self.var_extend(*name);
        check_unique(args)?;
        self.var_extend_many(args);
        let constructors : Vec<Symbol> = vars
            .iter()
            .map(|x| x.cons)
            .collect();
        
        check_unique(&constructors)?;
        for var in vars {
            self.check_varient(var.deref())?;
        } 

        self.var_delete_many(args);
        Ok(())
    }

    pub fn check_varient(&mut self, var: &Variant) -> Result<(),String> {
        let Variant { cons, args } = var;
        for arg in args {
            self.check_type(&arg)?;    
        }
        self.cons_env.insert(*cons);
        Ok(())

    }

    pub fn check_type_decl(&mut self, decl: &TypeDecl) -> Result<(),String> {
        let TypeDecl { name, args, typ } = decl;
        self.type_env.insert(*name);
        check_unique(args)?;
        self.var_extend_many(args);
        self.check_type(typ)?;
        self.var_delete_many(args);
        Ok(())
    }

    pub fn free_decl(&mut self, decl: &Decl) -> Result<(),String> {
        match decl {
            Decl::Val(val) => {
                self.free_val_decl(val)
            }
            Decl::Data(data) => {
                self.free_data_decl(data)
            }
            Decl::Type(typ) => {
                self.free_type_decl(typ)
            }
        }
    }

    pub fn free_val_decl(&mut self, decl: &ValDecl) -> Result<(),String> {
        self.var_env.try_take(&decl.name);
        Ok(())
    }

    pub fn free_data_decl(&mut self, decl: &DataDecl) -> Result<(),String> {
        let DataDecl { name, args, vars } = decl;
        self.type_env.try_take(name);

        for var in vars {
            self.free_varient(var);
        }

        Ok(())
    }

    pub fn free_type_decl(&mut self, decl: &TypeDecl) -> Result<(),String> {
        self.type_env.try_take(&decl.name);
        Ok(())
    }

    pub fn free_varient(&mut self, var: &Variant) -> Result<(),String> {
        self.cons_env.try_take(&var.cons);
        Ok(())
    }

    pub fn check_pattern(&mut self, pat: &Pattern) -> Result<(),String> {
        match pat {
            Pattern::Lit(lit) => {
                if let LitValue::Real(_) = lit {
                    Err("real numbers couldn't appear in pattern matching!".to_string())
                } else { Ok(()) }
            }
            Pattern::App(cons, args) => {
                if let Some(_) = self.cons_env.get(&cons) {
                    for arg in args {
                        self.check_pattern(arg)?;
                    }
                    Ok(())
                } else {
                    Err("constructor not found!".to_string())
                }
            }
            Pattern::Var(x) => {
                self.var_extend(*x);
                Ok(())
            }
            Pattern::Wild => {
                Ok(())
            }
        }
    }

    pub fn free_pattern(&mut self, pat: &Pattern) {
        match pat {
            Pattern::Lit(_) => {}
            Pattern::App(_, args) => {
                for arg in args {
                    self.free_pattern(arg);
                }
            }
            Pattern::Var(x) => {
                self.var_delete(*x);
            }
            Pattern::Wild => {}
        }
    }

    pub fn check_rule(&mut self, rule: &Rule) -> Result<(),String> {
        let Rule { pat, expr } = rule;
        self.check_pattern(pat.deref())?;
        self.check_expr(expr.deref())?;
        self.free_pattern(pat.deref());
        Ok(())
    }

    pub fn check_type(&self, typ: &Type) -> Result<(),String> {
        match typ {
            Type::Var(x) => {
                self.check_var_scope(x)?;
                Ok(())
            }
            Type::App(cons, args) => {
                if self.cons_env.contains(cons) == 0 {
                    return Err("Constructor not defined!".to_string());
                }
                for arg in args {
                    self.check_type(arg)?;
                }
                Ok(())
            }
            Type::Arr(ty1, ty2) => {
                self.check_type(ty1)?;
                self.check_type(ty2)?;
                Ok(())
            }
            Type::Lit(_) => {
                Ok(())
            }
        }
    }
    */


    pub fn check_expr_var(&mut self, var: &ExprVar) -> CheckResult<()> {
        let ExprVar { ident, span } = var;
        (|| {
            self.scope_val(ident)
        })().map_err(|mut e: Vec<CheckError>| {
            e.push(CheckError::ErrorIn("Variable", span.clone()));
            e
        })
    }

    pub fn check_expr_lam(&mut self, lam: &ExprLam) -> CheckResult<()> {
        let ExprLam { args, body, span } = lam;
        (|| {
            if let Some(sym) = check_unique(args) {
                return Err(vec![CheckError::FuncArgsNotUnique(sym)]);
            }
            for arg in args {
                self.val_env.insert(*arg);
            }
            self.check_expr(body)?;
            for arg in args {
                self.val_env.remove(arg);
            }
            Ok(())
        })().map_err(|mut e: Vec<CheckError>| {
            e.push(CheckError::ErrorIn("Lambda Abstraction", span.clone()));
            e
        })
    }

    pub fn check_expr_app(&mut self, app: &ExprApp) -> CheckResult<()> {
        let ExprApp { func, args, span } = app;
        (|| {
            self.check_expr(func)?;
            for arg in args {
                self.check_expr(arg)?;
            }
            Ok(())
        })().map_err(|mut e: Vec<CheckError>| {
            e.push(CheckError::ErrorIn("Lambda Application", span.clone()));
            e
        })
    }

    pub fn check_expr(&mut self, expr: &Expr) -> CheckResult<()> {
        match expr {
            Expr::Lit(_) => { Ok(()) }
            Expr::Var(var) => {
                self.check_expr_var(var)
            }
            Expr::Lam(lam) => {
                self.check_expr_lam(lam)
            }
            Expr::App(app) => {
                self.check_expr_app(app)
            }
            Expr::Let(expr) => {
                todo!()
            }
            Expr::Case(expr) => {
                todo!()
            }
            _ => unimplemented!()
        }
    }
}
