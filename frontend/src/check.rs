use std::cell::RefCell;
use std::net::ToSocketAddrs;
use std::ops::Deref;
use std::rc::Rc;

use hashbag::HashBag;

use crate::utils::*;
use crate::symbol::*;
use crate::parser::*;
use crate::pretty::*;
use crate::ast::*;


pub struct Checker<'src> {
    source: &'src str,
    var_env: HashBag<Symbol>,
    data_env: HashBag<Symbol>,
    cons_env: HashBag<Symbol>,
    type_env: HashBag<Symbol>,
    table: Rc<RefCell<SymTable>>,
}

pub fn check_unique(xs: &Vec<Symbol>) -> bool {
    let mut ys: Vec<Symbol> = Vec::new();
    for x in xs {
        for y in &ys {
            if x == y {
                return false;
            }
        }
        ys.push(*x);
    }
    true
}


impl<'src> Checker<'src> {

    pub fn env_extend(&mut self, x: Symbol) {
        self.var_env.insert(x);
    }

    pub fn env_delete(&mut self, x: Symbol) {
        if let k = self.var_env.remove(&x) {
            assert!(k >= 1);
            self.var_env.insert_many(x, k - 1);
        }
    }

    pub fn env_extend_many(&mut self, xs: &Vec<Symbol>) {
        for x in xs {
            self.var_env.insert(*x);
        }
    }

    pub fn env_delete_many(&mut self, xs: &Vec<Symbol>) {
        for x in xs {
            if let k = self.var_env.remove(&x) {
                assert!(k >= 1);
                self.var_env.insert_many(*x, k - 1);
            }
        }
    }

    pub fn var_lookup(&mut self, x: &Symbol) -> Result<(),String> {
        if self.var_env.contains(x) >= 1 {
            Ok(())
        } else {
            Err("variable not found!".to_string())
        }
    }

    pub fn check_decl(&mut self, decl: &Decl) -> Result<(),String> {
        match decl {
            Decl::Val(val) => { self.check_val_decl(val) }
            Decl::Data(data) => { self.check_data_decl(data) }
            Decl::Type(typ) => { self.check_type_decl(typ) }
        }
    }

    pub fn check_val_decl(&mut self, decl: &ValDecl) -> Result<(),String> {
        let ValDecl { name, args, body } = decl;
        self.env_extend(*name);
        self.env_extend_many(args);
        self.check_expr(body)?;
        self.env_delete_many(args);
        Ok(())
    }

    pub fn check_data_decl(&mut self, decl: &DataDecl) -> Result<(),String> {
        let DataDecl { name, args, vars } = decl;
        self.env_extend(*name);
        self.env_extend_many(args);
        //self.check_expr(body)?;
        self.env_delete_many(args);
        Ok(())
    }

    pub fn check_type_decl(&mut self, decl: &TypeDecl) -> Result<(),String> {
        let TypeDecl { name, args, typ } = decl;
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
                self.env_extend(*x);
                Ok(())
            }
            Pattern::Wild => {
                Ok(())
            }
        }
    }

    pub fn start_pattern(&mut self, pat: &Pattern) {


    }

    pub fn end_pattern(&mut self, pat: &Pattern) {
        match pat {
            Pattern::Lit(_) => {}
            Pattern::App(_, args) => {
                for arg in args {
                    self.end_pattern(arg);
                }
            }
            Pattern::Var(x) => {
                self.env_delete(*x);
            }
            Pattern::Wild => {}
        }
    }


    pub fn check_rule(&mut self, rule: &Rule) -> Result<(),String> {
        let Rule { pat, expr } = rule;
        self.check_pattern(pat.deref())?;
        self.check_expr(expr.deref())?;
        self.end_pattern(pat.deref());
        Ok(())
    }

    

    pub fn check_expr(&mut self, expr: &Expr) -> Result<(),String> {
        match expr {
            Expr::Lit(_) => { Ok(()) }
            Expr::Var(x) => {
                self.var_lookup(x)
            }
            Expr::Lam(xs, body) => {
                if !check_unique(xs) {
                    return Err("Same ident!".to_string());
                }
                self.env_extend_many(xs);
                self.check_expr(body)?;
                self.env_delete_many(xs);
                Ok(())
            }
            Expr::App(exprs) => {
                assert!(exprs.len() >= 1);
                for expr in exprs {
                    self.check_expr(expr)?;
                }
                Ok(())
            }
            Expr::Let(decls, body) => {
                for decl in decls {
                    self.check_decl(decl)?;
                }
                self.check_expr(body)?;
                Ok(())
            }
            Expr::Case(expr, rules) => {
                self.check_expr(expr)?;
                for rule in rules {
                    self.check_rule(rule.deref())?;
                }
                Ok(())
            }
            _ => unimplemented!()
        }
    }
}
