use std::collections::{HashMap, HashSet};

use super::*;
use crate::ast::*;
use crate::visitor::Visitor;

/// level one optimizer:
/// 1. safe beta inline (variable that used only once)
/// 2. dead code elimination (variable that never used)
/// 3. constant folding

pub struct DeadElim {
    change: usize,
    free_occur: HashSet<Symbol>,
    get_occur: HashSet<(Symbol,usize)>,
}

impl DeadElim {
    pub fn new() -> DeadElim {
        DeadElim {
            change: 0,
            free_occur: HashSet::new(),
            get_occur: HashSet::new(),
        }
    }
}


impl Visitor for DeadElim {
    
    fn visit_var_use(&mut self, sym: Symbol) -> Symbol {
        self.free_occur.insert(sym);
        sym
    }
    
    fn visit_let(&mut self, expr: ExprLet) -> Expr {

        let ExprLet { decls, cont } = expr;

        let cont = Box::new(self.visit_expr(*cont));

        // first, we assume all functions that has no refernce in "cont" are dead code
        // but it is not correct, since a function could be used by another function, which is not a dead code.
        // we have to look into these function definition and find all "dependancies"
        // and then we use a iteration method to look for a "fix point"

        // all function that used in "cont"
        let mut alive_names: Vec<Symbol> = decls.iter()
            .map(|decl| &decl.func)
            .filter(|name| self.free_occur.remove(name))
            .copied().collect();

        // record all function names in this "let-block"
        let func_list : Vec<Symbol> = decls.iter()
            .map(|decl| decl.func).collect();

        // now we look for all dependacy <K,V>, it means K calls V in his body
        let mut dependancy: HashMap<Symbol,Vec<Symbol>> = HashMap::new();

        let decls: Vec<Decl> = decls.into_iter()
            .map(|decl| {
                let decl = self.visit_decl(decl);
                
                let vec = func_list.iter()
                    .filter(|func| self.free_occur.remove(&func))
                    .copied().collect();

                dependancy.insert(decl.func, vec);
                
                decl
            })
            .collect();
        
        // find the fix point!
        loop {
            let mut new = Vec::new();

            for caller in &alive_names {
                for callee in dependancy.get(&caller).unwrap() {
                    if !alive_names.contains(callee) {
                        new.push(callee);
                    }
                }
            }

            if new.is_empty() {
                break;
            } else {
                new.into_iter()
                    .for_each(|name| alive_names.push(*name));
                continue;
            }
        }
        
        
        let decls = decls.into_iter()
            .filter(|decl| alive_names.contains(&decl.func))
            .collect();
    

        Expr::Let(ExprLet { decls, cont })
    }

    fn visit_opr(&mut self, expr: ExprOpr) -> Expr {

        let ExprOpr { prim, args, binds, cont } = expr;
        
        let cont = Box::new(self.visit_expr(*cont));

        // check if all binds are not used
        let mut free_flag: bool = false;
        for bind in &binds {
            if self.free_occur.remove(&bind) {
                free_flag = true;
            }
        }

        if !free_flag && prim.is_pure() {
            // dead code elimination
            return *cont;
        }

        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();

        Expr::Opr(ExprOpr { prim, args, binds, cont })

    }

    fn visit_rec(&mut self, expr: ExprRec) -> Expr {
        let ExprRec { size, bind, cont } = expr;

        let cont = Box::new(self.visit_expr(*cont));
        if !self.free_occur.remove(&expr.bind) {
            return *cont;
        }

        Expr::Rec(ExprRec { size, bind, cont })
    }

    fn visit_get(&mut self, expr: ExprGet) -> Expr {
        let ExprGet { rec, idx, bind, cont } = expr;
        
        let cont = Box::new(self.visit_expr(*cont));
        if !self.free_occur.remove(&expr.bind) {
            return *cont;
        }

        let rec = self.visit_atom(rec);
        self.get_occur.insert((rec.unwrap_var(),idx));

        Expr::Get(ExprGet { rec, idx, bind, cont })
    }

    fn visit_set(&mut self, expr: ExprSet) -> Expr {
        let ExprSet { rec, idx, arg, cont } = expr;
        
        let cont = Box::new(self.visit_expr(*cont));
        if self.get_occur.remove(&(rec.unwrap_var(),idx)) {
            // dead set elimination
            return *cont;
        }

        let arg = self.visit_atom(arg);
        let rec = self.visit_atom(rec);

        Expr::Set(ExprSet { rec, idx, arg, cont })
    }


}


/*
#[test]
fn opt_test() {
    use crate::parser::*;
    
    let string = "
        (fn x y => (* (+ x 1) (- y 2))) 3 4
    ";
    
    /*
    let string = "
        (fn f g x => ((f x) (g x))) + (fn x => x) 5
    ";
    */
    let mut par = Parser::new(string);

    let res = parse_program(&mut par);
    if let Ok(res) = res {
        println!("\n{res}");
        let expr = cps_trans::cps_trans_top(&res);
        println!("\n{}", expr);

        let expr = opt_level1(expr);
        println!("\n{}", expr);
    } else {
        par.print_err();
    }
}

*/
