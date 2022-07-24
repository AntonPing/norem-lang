use crate::ast::*;

use super::*;

pub trait Visitor {

    fn visit_expr(&mut self, expr: Expr) -> Expr {
        self.walk_expr(expr)
    }

    fn walk_expr(&mut self, expr: Expr) -> Expr {
        match expr {
            Expr::Let(expr) => self.visit_let(expr),
            Expr::Opr(expr) => self.visit_opr(expr),
            Expr::Brs(expr) => self.visit_brs(expr),
            Expr::App(expr) => self.visit_app(expr),
            Expr::Rec(expr) => self.visit_rec(expr),
            Expr::Set(expr) => self.visit_set(expr),
            Expr::Get(expr) => self.visit_get(expr),
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
        let Decl { func, args, body } = decl;
        let func = self.visit_var_def(func);
        let args = args.iter()
            .map(|arg| self.visit_var_def(*arg))
            .collect();
        let body = self.visit_expr(body);
        Decl { func, args, body }
    }

    fn visit_let(&mut self, expr: ExprLet) -> Expr {
        self.walk_let_top_down(expr)
    }

    fn walk_let_top_down(&mut self, expr: ExprLet) -> Expr {
        let ExprLet { decls, cont } = expr;
        let decls = decls.into_iter()
            .map(|def| self.visit_decl(def))
            .collect();
        let cont = Box::new(self.visit_expr(*cont));
        Expr::Let(ExprLet { decls, cont })
    }

    fn walk_let_down_top(&mut self, expr: ExprLet) -> Expr {
        let ExprLet { decls, cont } = expr;
        let cont = Box::new(self.visit_expr(*cont));
        let decls = decls.into_iter()
            .map(|def| self.visit_decl(def))
            .collect();
        Expr::Let(ExprLet { decls, cont })
    }

    fn visit_opr(&mut self, expr: ExprOpr) -> Expr {
        self.walk_opr_top_down(expr)
    }

    fn walk_opr_top_down(&mut self, expr: ExprOpr) -> Expr {
        let ExprOpr { prim, args, binds, cont } = expr;
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        let binds = binds.into_iter()
            .map(|bind| self.visit_var_def(bind))
            .collect();
        let cont = Box::new(self.visit_expr(*cont));
        Expr::Opr(ExprOpr { prim, args, binds, cont })
    }

    fn walk_opr_down_top(&mut self, expr: ExprOpr) -> Expr {
        let ExprOpr { prim, args, binds, cont } = expr;
        let cont = Box::new(self.visit_expr(*cont));
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        let binds = binds.into_iter()
            .map(|bind| self.visit_var_def(bind))
            .collect();
        Expr::Opr(ExprOpr { prim, args, binds, cont })
    }

    fn visit_brs(&mut self, expr: ExprBrs) -> Expr {
        self.walk_brs_top_down(expr)
    }

    fn walk_brs_top_down(&mut self, expr: ExprBrs) -> Expr {
        let ExprBrs { prim, args, brs } = expr;

        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        
        let brs = brs.into_iter()
            .map(|br| self.visit_expr(br))
            .collect();

        Expr::Brs(ExprBrs { prim, args, brs })
    }

    fn walk_brs_down_top(&mut self, expr: ExprBrs) -> Expr {
        let ExprBrs { prim, args, brs } = expr;
        
        let brs = brs.into_iter()
            .map(|br| self.visit_expr(br))
            .collect();

        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        
        Expr::Brs(ExprBrs { prim, args, brs })
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

    fn visit_rec(&mut self, expr: ExprRec) -> Expr {
        self.walk_rec(expr)
    }

    fn walk_rec(&mut self, expr: ExprRec) -> Expr {
        let ExprRec { size, bind, cont } = expr;

        let bind = self.visit_var_def(bind);
        let cont = Box::new(self.visit_expr(*cont));

        Expr::Rec(ExprRec { size, bind, cont })
    }

    fn visit_get(&mut self, expr: ExprGet) -> Expr {
        self.walk_get(expr)
    }

    fn walk_get(&mut self, expr: ExprGet) -> Expr {
        let ExprGet { rec, idx, bind, cont } = expr;

        let rec = self.visit_atom(rec);
        let bind = self.visit_var_def(bind);
        let cont = Box::new(self.visit_expr(*cont));

        Expr::Get(ExprGet { rec, idx, bind, cont })
    }

    fn visit_set(&mut self, expr: ExprSet) -> Expr {
        self.walk_set(expr)
    }

    fn walk_set(&mut self, expr: ExprSet) -> Expr {
        let ExprSet { rec, idx, arg, cont } = expr;

        let rec = self.visit_atom(rec);
        let arg = self.visit_atom(arg);
        let cont = Box::new(self.visit_expr(*cont));

        Expr::Set(ExprSet { rec, idx, arg, cont })
    }    
    
    fn visit_tag(&mut self, tag: Tag, cont: Box<Expr>) -> Expr {
        Expr::Tag(tag, Box::new(self.visit_expr(*cont)))
    }

}
