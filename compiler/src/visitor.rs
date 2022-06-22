use std::collections::HashMap;

use crate::ast::*;
use crate::core::*;
use crate::symbol::*;
use crate::utils::Span;

pub trait CExprVisitor {

    fn visit_var_def(&mut self, sym: Symbol) -> Symbol {
        sym
    }

    fn visit_var_use(&mut self, sym: Symbol) -> Symbol {
        sym
    }

    fn visit_atom(&mut self, atom: Atom) -> Atom {
        if let Atom::Var(var) = atom {
            Atom::Var(self.visit_var_use(var))
        } else {
            atom
        }
    }

    fn visit_cdecl(&mut self, decl: CDecl) -> CDecl {
        let CDecl { func, args, body } = decl;
        let args = args.iter()
            .map(|arg| self.visit_var_def(*arg)).collect();
        let body = Box::new(self.walk_cexpr(*body));
        CDecl { func, args, body }
    }

    // for visiting CExpr
    fn visit_app(&mut self, func: Atom, args: Vec<Atom>) -> CExpr {
        CExpr::App(
            self.visit_atom(func),
            args.into_iter().map(|arg| self.visit_atom(arg)).collect(),
        )
    }

    fn visit_let(&mut self, decl: CDecl, cont: Box<CExpr>) -> CExpr {
        CExpr::Let(self.visit_cdecl(decl), Box::new(self.walk_cexpr(*cont)))
    }

    fn visit_fix(&mut self, decls: Vec<CDecl>, cont: Box<CExpr>) -> CExpr {
        CExpr::Fix(
            decls.into_iter().map(|def| self.visit_cdecl(def)).collect(),
            Box::new(self.walk_cexpr(*cont)),
        )
    }

    fn visit_uniop(&mut self, prim: Prim, arg: Atom, ret: Symbol, cont: Box<CExpr>) -> CExpr {
        CExpr::Uniop(
            prim,
            self.visit_atom(arg),
            self.visit_var_def(ret),
            Box::new(self.walk_cexpr(*cont)),
        )
    }

    fn visit_binop(
        &mut self,
        prim: Prim,
        arg1: Atom,
        arg2: Atom,
        ret: Symbol,
        cont: Box<CExpr>,
    ) -> CExpr {
        CExpr::Binop(
            prim,
            self.visit_atom(arg1),
            self.visit_atom(arg2),
            self.visit_var_def(ret),
            Box::new(self.walk_cexpr(*cont)),
        )
    }

    fn visit_switch(&mut self, idx: Atom, brs: Vec<CExpr>) -> CExpr {
        CExpr::Switch(
            self.visit_atom(idx),
            brs.into_iter().map(|br| self.walk_cexpr(br)).collect(),
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
            flds.into_iter().map(|fld| self.visit_atom(fld)).collect(),
            self.visit_var_def(ret),
            Box::new(self.walk_cexpr(*cont)),
        )
    }
    fn visit_select(&mut self, idx: usize, from: Atom, ret: Symbol, cont: Box<CExpr>) -> CExpr {
        CExpr::Select(
            idx,
            self.visit_atom(from),
            self.visit_var_def(ret),
            Box::new(self.walk_cexpr(*cont)),
        )
    }

    fn visit_halt(&mut self, arg: Atom) -> CExpr {
        CExpr::Halt(self.visit_atom(arg))
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

pub trait ExprVisitor {
    fn visit_lit_val(&mut self, lit: LitVal) -> LitVal {
        lit
    }

    fn visit_sym_def(&mut self, sym: Symbol) -> Symbol {
        sym
    }

    fn visit_sym_use(&mut self, sym: Symbol) -> Symbol {
        sym
    }

    fn visit_lit(&mut self, expr: ExprLit) -> ExprLit {
        let ExprLit { lit, span } = expr;
        let lit = self.visit_lit_val(lit);
        ExprLit { lit, span }
    }

    fn visit_var(&mut self, expr: ExprVar) -> ExprVar {
        let ExprVar { ident, span } = expr;
        let ident = self.visit_sym_use(ident);
        ExprVar { ident, span }
    }

    fn visit_prim(&mut self, expr: ExprPrim) -> ExprPrim {
        let ExprPrim { prim, span } = expr;
        //let prim = self.visit_prim(prim);
        ExprPrim { prim, span }
    }

    fn visit_lam(&mut self, expr: ExprLam) -> ExprLam {
        let ExprLam { args, body, span } = expr;
        let args = args
            .into_iter()
            .map(|arg| self.visit_sym_def(arg))
            .collect();
        let body = Box::new(self.walk_expr(*body));
        ExprLam { args, body, span }
    }

    fn visit_app(&mut self, expr: ExprApp) -> ExprApp {
        let ExprApp { func, args, span } = expr;
        let func = Box::new(self.walk_expr(*func));
        let args = args.into_iter().map(|arg| self.walk_expr(arg)).collect();
        ExprApp { func, args, span }
    }

    fn visit_let(&mut self, expr: ExprLet) -> ExprLet {
        let ExprLet { decls, body, span } = expr;
        let decls = decls
            .into_iter()
            .map(|decl| self.visit_decl(decl))
            .collect();
        let body = Box::new(self.walk_expr(*body));
        ExprLet { decls, body, span }
    }

    fn visit_case(&mut self, expr: ExprCase) -> ExprCase {
        todo!()
    }

    fn visit_decl(&mut self, decl: Decl) -> Decl {
        todo!()
    }

    fn walk_expr(&mut self, expr: Expr) -> Expr {
        match expr {
            Expr::Lit(expr) => Expr::Lit(self.visit_lit(expr)),
            Expr::Prim(expr) => Expr::Prim(self.visit_prim(expr)),
            Expr::Var(expr) => Expr::Var(self.visit_var(expr)),
            Expr::Lam(expr) => Expr::Lam(self.visit_lam(expr)),
            Expr::App(expr) => Expr::App(self.visit_app(expr)),
            Expr::Let(expr) => Expr::Let(self.visit_let(expr)),
            Expr::Case(expr) => Expr::Case(self.visit_case(expr)),
        }
    }
}

/*
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

pub fn subst_expr(expr: CExpr) -> CExpr {
    let mut sub = Subst { env: HashMap::new() };
    sub.walk_cexpr(expr)
}

*/
