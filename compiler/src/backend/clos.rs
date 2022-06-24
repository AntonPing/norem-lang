use std::collections::{HashMap, HashSet};

use crate::ast::Prim;
use crate::symbol::{Symbol, newvar, genvar};
use crate::utils::MultiSet;

use super::core::*;
use super::visitor::*;

/*
    closure conversion
*/

pub struct ClosConv {
    freevars: HashSet<Symbol>,
}

impl ClosConv {
    pub fn new() -> ClosConv {
        ClosConv {
            freevars: HashSet::new(),
        }
    }
    pub fn run(expr: CExpr) -> CExpr {
        let mut clos = ClosConv::new();
        clos.walk_cexpr(expr)
    }
}

impl CExprVisitor for ClosConv {
    fn visit_app(&mut self, func: Atom, mut args: Vec<Atom>) -> CExpr {

        if let Atom::Var(sym) = &func {
            self.freevars.insert(*sym);
        }
        for arg in &args {
            if let Atom::Var(sym) = arg {
                self.freevars.insert(*sym);
            }
        }

        // f(a,b,...,z) => select f -> f'; f'(f,a,b,...,z)
        let func2 = genvar('c');
        args.insert(0, func);
        CExpr::Select(0, func, func2, Box::new(
            CExpr::App(Atom::Var(func2), args)
        ))
    }

    fn visit_halt(&mut self, arg: Atom) -> CExpr {
        if let Atom::Var(sym) = &arg {
            self.freevars.insert(*sym);
        }
        CExpr::Halt(arg)
    }


    fn visit_let(&mut self, decl: CDecl, cont: Box<CExpr>) -> CExpr {
        let mut decl = self.visit_cdecl(decl);
        for arg in &decl.args {
            self.freevars.remove(arg);
        }

        let clos = genvar('c');

        for (i,var) in self.freevars.iter().enumerate() {
            decl.body = Box::new(CExpr::Select(i, 
                Atom::Var(clos), *var, decl.body));
        }

        decl.args.insert(0, clos);

        let cont = self.walk_cexpr(*cont);

        let mut rec = Vec::new();
        rec.push(Atom::Var(decl.func));
        for var in &self.freevars {
            rec.push(Atom::Var(*var));
        }
        
        CExpr::Record(rec, decl.func, Box::new(cont))

    }


    fn visit_fix(&mut self, decls: Vec<CDecl>, cont: Box<CExpr>) -> CExpr {

        todo!()
    }
}

#[test]
fn opt_test() {
    use crate::parser::*;
    let string = "
        (fn x y => (* (+ x 1) (- y 2))) 3 4
    ";
    let mut par = Parser::new(string);

    let res = parse_program(&mut par);
    if let Ok(res) = res {
        println!("\n{res}");
        let cexpr = cps_trans_top(&res);
        println!("\n{}", cexpr);
        
        let cexpr = ClosConv::run(cexpr);
        println!("\n{}", cexpr);
    } else {
        par.print_err();
    }
}