use std::collections::HashMap;

use crate::symbol::*;
use crate::ast::*;
use crate::core::*;
use crate::utils::Span;

pub trait CExprVisitor {

    fn visit_atom(&mut self, atom: Atom) -> Atom {
        atom
    }

    fn visit_cdef(&mut self, def: Def<CExpr>) -> Def<CExpr> {
        let Def {func, args, body} = def;
        let body = Box::new(self.walk_cexpr(*body));
        Def { func, args, body }
    }

    // for visiting CExpr
    fn visit_app(&mut self, func: Atom, args: Vec<Atom>) -> CExpr {
        CExpr::App(
            self.visit_atom(func),
            args.into_iter()
                .map(|arg| self.visit_atom(arg))
                .collect()
        )
    }

    fn visit_let(&mut self, def: Def<CExpr>, cont: Box<CExpr>) -> CExpr {
        CExpr::Let(
            self.visit_cdef(def),
            Box::new(self.walk_cexpr(*cont))
        )
    }

    fn visit_fix(&mut self, defs: Vec<Def<CExpr>>, cont: Box<CExpr>) -> CExpr {
        CExpr::Fix(
            defs.into_iter()
                .map(|def| self.visit_cdef(def))
                .collect(),
            Box::new(self.walk_cexpr(*cont))
        )
    }

    fn visit_uniop(&mut self, prim: Prim, arg: Atom, ret: Symbol, cont: Box<CExpr>) -> CExpr {
        CExpr::Uniop(
            prim,
            self.visit_atom(arg),
            ret,
            Box::new(self.walk_cexpr(*cont))
        )
    }

    fn visit_binop(&mut self, prim: Prim, arg1: Atom, arg2: Atom, ret: Symbol, cont: Box<CExpr>) -> CExpr {
        CExpr::Binop(
            prim,
            self.visit_atom(arg1),
            self.visit_atom(arg2),
            ret,
            Box::new(self.walk_cexpr(*cont))
        )
    }

    fn visit_switch(&mut self, idx: Atom, brs: Vec<CExpr>) -> CExpr {
        CExpr::Switch(
            self.visit_atom(idx),
            brs.into_iter()
                .map(|br| self.walk_cexpr(br))
                .collect()
        )
    }

    fn visit_ifte(&mut self, cond: Atom, trbr: Box<CExpr>, flbr: Box<CExpr>) -> CExpr {
        CExpr::Ifte(
            self.visit_atom(cond),
            Box::new(self.walk_cexpr(*trbr)),
            Box::new(self.walk_cexpr(*flbr)),
        )
    }

    fn visit_record(&mut self, flds: Vec<Atom>, ret: Symbol, cont: Box<CExpr>) -> CExpr {
        CExpr::Record(
            flds.into_iter()
                .map(|fld| self.visit_atom(fld))
                .collect()
            ,
            ret,
            Box::new(self.walk_cexpr(*cont)),
        )
    }
    fn visit_select(&mut self, idx: usize, from: Atom, ret: Symbol, cont: Box<CExpr>) -> CExpr {
        CExpr::Select(
            idx,
            from,
            ret,
            Box::new(self.walk_cexpr(*cont)),
        )
    }

    fn visit_halt(&mut self, arg: Atom) -> CExpr {
        CExpr::Halt(
            self.visit_atom(arg)
        )
    }

    fn visit_tag(&mut self, tag: Tag, cont: Box<CExpr>) -> CExpr {
        CExpr::Tag(tag, cont)
    }
    
    fn walk_cexpr(&mut self, expr: CExpr) -> CExpr {
        match expr {
            CExpr::App(func, args) => {
                self.visit_app(func, args)
            }
            CExpr::Let(def, cont) => {
                self.visit_let(def, cont)
            }
            CExpr::Fix(defs, cont) => {
                self.visit_fix(defs, cont)
            }
            CExpr::Uniop(prim, arg, ret, cont) => {
                self.visit_uniop(prim, arg, ret, cont)
            }
            CExpr::Binop(prim, arg1, arg2, ret, cont) => {
                self.visit_binop(prim, arg1, arg2, ret, cont)
            }
            CExpr::Switch(idx, brs) => {
                self.visit_switch(idx, brs)
            }
            CExpr::Ifte(cond, trbr, flbr) => {
                self.visit_ifte(cond, trbr, flbr)
            }
            CExpr::Record(flds, ret, cont) => {
                self.visit_record(flds, ret, cont)
            }
            CExpr::Select(idx, from, ret, cont) => {
                self.visit_select(idx, from, ret, cont)
            }
            CExpr::Halt(arg) => {
                self.visit_halt(arg)
            }
            CExpr::Tag(tag, cont) => {
                self.visit_tag(tag, cont)
            }
        }
    }
}

/*
pub trait ExprVisitor {
    fn visit_val(&mut self, lit: LitVal, span: Span) -> LitVal {
        lit
    }

    fn visit_prim(&mut self, prim: Prim) -> Prim {
        prim
    }
    
    fn visit_var(&mut self, var: ExprVar) -> ExprVar {
        var
    }

    fn visit_lam(&mut self, lam: ExprLam) -> ExprLam {
        lam
    }

    fn visit_app(&mut self, app: ExprApp) -> ExprApp {
        app
    }

    fn walk_expr(&mut self, expr: Expr) -> Expr {
        match expr {
            Expr::Lit(ExprLit{ lit, span }) =>
                self.visit_val(lit, span),
            Expr::Prim(_) => todo!(),
            Expr::Var(_) => todo!(),
            Expr::Lam(_) => todo!(),
            Expr::App(_) => todo!(),
            Expr::Let(_) => todo!(),
            Expr::Case(_) => todo!(),
        }
    }
}

pub struct Subst {
    env: HashMap<Symbol,Atom>,
}

impl CExprVisitor for Subst {
    fn visit_atom(&mut self, atom: Atom) -> Atom {
        if let Atom::Var(k) = atom {
            if let Some(v) = self.env.get(&k) {
                return *v;
            }
        }
        atom
    }

    fn visit_tag(&mut self, tag: Tag, cont: Box<CExpr>) -> CExpr {
        if let Tag::SubstAtom(k, v) = tag {
            self.env.insert(k, v);
            self.walk_cexpr(*cont)
        } else {
            CExpr::Tag(tag, 
                Box::new(self.walk_cexpr(*cont)))
        }
    }
}
*/


pub fn subst_expr(expr: CExpr) -> CExpr {
    let mut sub = Subst { env: HashMap::new() };
    sub.walk_cexpr(expr)
}

