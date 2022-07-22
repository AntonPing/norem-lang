use crate::ast::*;

use super::*;

pub trait Visitor {

    fn visit_cexpr(&mut self, expr: Expr) -> Expr {
        self.walk_cexpr(expr)
    }

    fn walk_cexpr(&mut self, expr: Expr) -> Expr {
        match expr {
            Expr::App(expr) => self.visit_app(expr),
            Expr::Let(expr) => self.visit_let(expr),
            Expr::Opr(expr) => self.visit_opr(expr),
            Expr::Tag(tag, cont) => self.visit_tag(tag, cont),
        }
    }

    fn visit_var_def(&mut self, sym: Symbol) -> Symbol {
        sym
    }

    fn visit_var_use(&mut self, sym: Symbol) -> Symbol {
        sym
    }

    fn visit_atom(&mut self, atom: Atom) -> Atom {
        self.walk_atom(atom)
    }

    fn walk_atom(&mut self, atom: Atom) -> Atom {
        if let Atom::Var(var) = atom {
            Atom::Var(self.visit_var_use(var))
        } else {
            atom
        }
    }

    fn visit_decl(&mut self, decl: Decl) -> Decl {
        self.walk_decl(decl)
    }

    fn walk_decl(&mut self, decl: Decl) -> Decl {
        let Decl { func, args, body, rec_ref } = decl;
        let func = self.visit_var_def(func);
        let args = args.iter()
            .map(|arg| self.visit_var_def(*arg))
            .collect();
        let body = self.visit_cexpr(body);
        Decl { func, args, body, rec_ref }
    }

    fn visit_let(&mut self, expr: ExprLet) -> Expr {
        self.walk_let_top_down(expr)
    }

    fn walk_let_top_down(&mut self, expr: ExprLet) -> Expr {
        let ExprLet { decls, cont } = expr;
        let decls = decls.into_iter()
            .map(|def| self.visit_decl(def))
            .collect();
        let cont = Box::new(self.visit_cexpr(*cont));
        Expr::Let(ExprLet { decls, cont })
    }

    fn walk_let_down_top(&mut self, expr: ExprLet) -> Expr {
        let ExprLet { decls, cont } = expr;
        let cont = Box::new(self.visit_cexpr(*cont));
        let decls = decls.into_iter()
            .map(|def| self.visit_decl(def))
            .collect();
        Expr::Let(ExprLet { decls, cont })
    }

    fn visit_opr(&mut self, expr: ExprOpr) -> Expr {
        self.walk_opr_top_down(expr)
    }

    fn walk_opr_top_down(&mut self, expr: ExprOpr) -> Expr {
        let ExprOpr { prim, args, binds, conts } = expr;
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        let binds = binds.into_iter()
            .map(|bind| self.visit_var_def(bind))
            .collect();
        let conts = conts.into_iter()
            .map(|cont| self.visit_cexpr(cont))
            .collect();
        Expr::Opr(ExprOpr { prim, args, binds, conts })
    }

    fn walk_opr_down_top(&mut self, expr: ExprOpr) -> Expr {
        let ExprOpr { prim, args, binds, conts } = expr;
        let conts = conts.into_iter()
            .map(|cont| self.visit_cexpr(cont))
            .collect();
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        let binds = binds.into_iter()
            .map(|bind| self.visit_var_def(bind))
            .collect();
        Expr::Opr(ExprOpr { prim, args, binds, conts })
    }

    fn visit_app(&mut self, expr: ExprApp) -> Expr {
        self.walk_app(expr)
    }

    fn walk_app(&mut self, expr: ExprApp) -> Expr {
        let ExprApp { func, args } = expr;
        let func = self.visit_atom(func);
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        Expr::App(ExprApp { func, args })
    }

    fn visit_tag(&mut self, tag: Tag, cont: Box<Expr>) -> Expr {
        Expr::Tag(tag, Box::new(self.visit_cexpr(*cont)))
    }
}
