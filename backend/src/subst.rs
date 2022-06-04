use std::collections::HashMap;

use crate::symbol::Symbol;
use crate::core::*;

/*
let func = self.visit(func);
let args = args.into_iter()
    .map(|arg| {
        self.visit(arg)
    });
CExpr::App(func, args)
*/

pub trait Visitor: Sized {

    //fn visit_atom()


    
    fn visit_cexpr(&mut self, expr: CExpr) -> CExpr {
        expr // overloaded by implmentation by need
    }
    
    fn visit_def(&mut self, def: Def<CExpr>) -> Def<CExpr> {
        def // overloaded by implmentation by need
    }

    fn walk_def(&mut self, def: Def<CExpr>) -> Def<CExpr> {
        def
    }
    
    fn walk_cexpr(&mut self, expr: CExpr) -> CExpr {
        return expr;
        match expr {
            e@CExpr::App(_, _) => {
                e
            }
            CExpr::Let(_, _) => todo!(),
            CExpr::Fix(_, _) => todo!(),
            CExpr::Uniop(_, _, _, _) => todo!(),
            CExpr::Binop(_, _, _, _, _) => todo!(),
            CExpr::Switch(_, _) => todo!(),
            CExpr::Ifte(_, _, _) => todo!(),
            CExpr::Record(_, _, _) => todo!(),
            CExpr::Select(_, _) => todo!(),
            CExpr::Halt(_) => todo!(),
            CExpr::Tag(_, _) => todo!(),
        }
    }

    fn walk_lexpr(&mut self, expr: LExpr) -> LExpr {
        expr
    }
}

pub struct Subst {
    env: HashMap<Symbol,Atom>,
}

impl Visitor for Subst {}

