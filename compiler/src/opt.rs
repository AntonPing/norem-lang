use std::collections::HashMap;

use crate::symbol::{Symbol, newvar, genvar};
use crate::core::*;
use crate::visitor::*;


pub struct Subst {
    map: HashMap<Symbol,Atom>,
}

impl Subst {
    pub fn new() -> Subst {
        Subst { map: HashMap::new() }
    }
    pub fn run(&mut self, expr: CExpr) -> CExpr {
        self.walk_cexpr(expr)
    }
}

impl CExprVisitor for Subst {

    fn visit_var_def(&mut self, sym: Symbol) -> Symbol {
        //assert!(!self.map.contains_key(&var));
        self.map.remove(&sym);
        sym
    }

    fn visit_var_use(&mut self, sym: Symbol) -> Symbol {
        sym
    }

    fn visit_atom(&mut self, atom: Atom) -> Atom {
        if let Atom::Var(sym) = atom {
            if let Some(res) = self.map.get(&sym) {
                *res
            } else {
                atom
            }
        } else {
            atom
        }
    }

    fn visit_tag(&mut self, tag: Tag, cont: Box<CExpr>) -> CExpr {
        match tag {
            Tag::SubstAtom(k, v) => {
                self.map.insert(k, v);
                self.walk_cexpr(*cont)
            }
            other => {
                let res = self.walk_cexpr(*cont);
                CExpr::Tag(other, Box::new(res))
            }
        }
        
    }
}



pub struct ConstFold {}

impl ConstFold {
    pub fn new() -> ConstFold {
        ConstFold {}
    }
    pub fn run(&mut self, expr: CExpr) -> CExpr {
        self.walk_cexpr(expr)
    }
}

impl CExprVisitor for ConstFold {

    


}