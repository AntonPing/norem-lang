use std::cell::RefCell;
use std::hash::Hash;
use std::ops::Deref;
use std::rc::Rc;

use hashbag::HashBag;

use crate::ast::*;
use crate::symbol::*;
use crate::utils::*;

pub struct Checker {
    var_env: HashBag<Symbol>,
    //data_env: HashBag<Symbol>,
    cons_env: HashBag<Symbol>,
    type_env: HashBag<Symbol>,
    table: Mut<SymTable>,
}

pub fn check_unique(xs: &Vec<Symbol>) -> Result<(), String> {
    let mut ys: Vec<Symbol> = Vec::new();
    for x in xs {
        for y in &ys {
            if x == y {
                return Err("variables aren't unique!".to_string());
            }
        }
        ys.push(*x);
    }
    Ok(())
}

impl Checker {
    pub fn new(table: Mut<SymTable>) -> Checker {
        Checker {
            var_env: HashBag::new(),
            cons_env: HashBag::new(),
            type_env: HashBag::new(),
            table,
        }
    }

    pub fn var_extend(&mut self, x: Symbol) {
        self.var_env.insert(x);
    }

    pub fn var_delete(&mut self, x: Symbol) {
        let k = self.var_env.remove(&x);
        assert!(k >= 1);
        self.var_env.insert_many(x, k - 1);
    }

    pub fn var_extend_many(&mut self, xs: &Vec<Symbol>) {
        for x in xs {
            self.var_env.insert(*x);
        }
    }

    pub fn var_delete_many(&mut self, xs: &Vec<Symbol>) {
        for x in xs {
            let k = self.var_env.remove(&x);
            assert!(k >= 1);
            self.var_env.insert_many(*x, k - 1);
        }
    }

    pub fn check_var_scope(&self, x: &Symbol) -> Result<(), String> {
        if self.var_env.contains(x) >= 1 {
            Ok(())
        } else {
            Err("variable not found!".to_string())
        }
    }

    pub fn check_decl(&mut self, decl: &Decl) -> Result<(), String> {
        match decl {
            Decl::Val(val) => self.check_val_decl(val),
            Decl::Data(data) => self.check_data_decl(data),
            Decl::Type(typ) => self.check_type_decl(typ),
        }
    }

    pub fn check_val_decl(&mut self, decl: &ValDecl) -> Result<(), String> {
        let ValDecl { name, args, body } = decl;
        self.var_extend(*name);
        check_unique(args)?;
        self.var_extend_many(args);
        self.check_expr(body)?;
        self.var_delete_many(args);
        Ok(())
    }

    pub fn check_data_decl(&mut self, decl: &DataDecl) -> Result<(), String> {
        let DataDecl { name, args, vars } = decl;
        self.var_extend(*name);
        check_unique(args)?;
        self.var_extend_many(args);
        let constructors: Vec<Symbol> = vars.iter().map(|x| x.cons).collect();

        check_unique(&constructors)?;
        for var in vars {
            self.check_varient(var.deref())?;
        }

        self.var_delete_many(args);
        Ok(())
    }

    pub fn check_varient(&mut self, var: &Variant) -> Result<(), String> {
        let Variant { cons, args } = var;
        for arg in args {
            self.check_type(&arg)?;
        }
        self.cons_env.insert(*cons);
        Ok(())
    }

    pub fn check_type_decl(&mut self, decl: &TypeDecl) -> Result<(), String> {
        let TypeDecl { name, args, typ } = decl;
        self.type_env.insert(*name);
        check_unique(args)?;
        self.var_extend_many(args);
        self.check_type(typ)?;
        self.var_delete_many(args);
        Ok(())
    }

    pub fn free_decl(&mut self, decl: &Decl) -> Result<(), String> {
        match decl {
            Decl::Val(val) => self.free_val_decl(val),
            Decl::Data(data) => self.free_data_decl(data),
            Decl::Type(typ) => self.free_type_decl(typ),
        }
    }

    pub fn free_val_decl(&mut self, decl: &ValDecl) -> Result<(), String> {
        self.var_env.try_take(&decl.name);
        Ok(())
    }

    pub fn free_data_decl(&mut self, decl: &DataDecl) -> Result<(), String> {
        let DataDecl { name, args, vars } = decl;
        self.type_env.try_take(name);

        for var in vars {
            self.free_varient(var);
        }

        Ok(())
    }

    pub fn free_type_decl(&mut self, decl: &TypeDecl) -> Result<(), String> {
        self.type_env.try_take(&decl.name);
        Ok(())
    }

    pub fn free_varient(&mut self, var: &Variant) -> Result<(), String> {
        self.cons_env.try_take(&var.cons);
        Ok(())
    }

    pub fn check_pattern(&mut self, pat: &Pattern) -> Result<(), String> {
        match pat {
            Pattern::Lit(lit) => {
                if let LitValue::Real(_) = lit {
                    Err("real numbers couldn't appear in pattern matching!".to_string())
                } else {
                    Ok(())
                }
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
            Pattern::Wild => Ok(()),
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

    pub fn check_rule(&mut self, rule: &Rule) -> Result<(), String> {
        let Rule { pat, expr } = rule;
        self.check_pattern(pat.deref())?;
        self.check_expr(expr.deref())?;
        self.free_pattern(pat.deref());
        Ok(())
    }

    pub fn check_type(&self, typ: &Type) -> Result<(), String> {
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
            Type::Lit(_) => Ok(()),
        }
    }

    pub fn check_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Lit(_) => Ok(()),
            Expr::Var(x) => self.check_var_scope(x),
            Expr::Lam(xs, body) => {
                check_unique(xs)?;
                self.var_extend_many(xs);
                self.check_expr(body)?;
                self.var_delete_many(xs);
                Ok(())
            }
            Expr::App(func, args) => {
                self.check_expr(func)?;
                for arg in args {
                    self.check_expr(arg)?;
                }
                Ok(())
            }
            Expr::Let(decls, body) => {
                for decl in decls {
                    self.check_decl(decl)?;
                }
                self.check_expr(body)?;
                for decl in decls {
                    self.free_decl(decl)?;
                }

                Ok(())
            }
            Expr::Case(expr, rules) => {
                self.check_expr(expr)?;
                for rule in rules {
                    self.check_rule(rule.deref())?;
                }
                Ok(())
            }
            _ => unimplemented!(),
        }
    }
}
