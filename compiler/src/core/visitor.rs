use crate::ast::*;
use crate::symbol::*;
use super::*;

pub trait VisitorTopDown {
    fn walk_cexpr(&mut self, expr: Core) -> Core {
        match expr {
            Core::App(expr) => self.visit_app(expr),
            Core::Let(expr) => self.visit_let(expr),
            Core::Fix(expr) => self.visit_fix(expr),
            Core::Opr(expr) => self.visit_opr(expr),
            Core::Case(expr) => self.visit_case(expr),
            Core::Rec(expr) => self.visit_rec(expr),
            Core::Set(expr) => self.visit_set(expr),
            Core::Get(expr) => self.visit_get(expr),
            Core::Halt(arg) => self.visit_halt(arg),
            Core::Tag(tag, cont) => self.visit_tag(tag, cont),
        }
    }
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

    fn visit_decl(&mut self, decl: CoreDecl) -> CoreDecl {
        let CoreDecl { func, args, body } = decl;
        let func = self.visit_var_def(func);
        let args = args.iter()
            .map(|arg| self.visit_var_def(*arg))
            .collect();
        let body = Box::new(self.walk_cexpr(*body));
        CoreDecl { func, args, body }
    }

    fn visit_app(&mut self, expr: CoreApp) -> Core {
        let CoreApp { func, args } = expr;
        let func = self.visit_atom(func);
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        Core::App(CoreApp { func, args })
    }

    fn visit_let(&mut self, expr: CoreLet) -> Core {
        let CoreLet { decl, cont } = expr;
        let decl = self.visit_decl(decl);
        let cont = Box::new(self.walk_cexpr(*cont));
        Core::Let(CoreLet { decl, cont })
    }

    fn visit_fix(&mut self, expr: CoreFix) -> Core {
        let CoreFix { decls, cont } = expr;
        let decls = decls.into_iter()
            .map(|def| self.visit_decl(def))
            .collect();
        let cont = Box::new(self.walk_cexpr(*cont));
        Core::Fix(CoreFix { decls, cont })
    }

    fn visit_opr(&mut self, expr: CoreOpr) -> Core {
        let CoreOpr { prim, args, bind, cont } = expr;
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        let bind = self.visit_var_def(bind);
        let cont = Box::new(self.walk_cexpr(*cont));
        Core::Opr(CoreOpr { prim, cont, args, bind })
    }

    fn visit_case(&mut self, expr: CoreCase) -> Core {
        let CoreCase { arg, brs } = expr;
        let arg = self.visit_atom(arg);
        let brs = brs.into_iter()
            .map(|br| self.walk_cexpr(br))
            .collect();
        Core::Case(CoreCase { arg, brs })
    }

    fn visit_rec(&mut self, expr: CoreRec) -> Core {
        let CoreRec { size, bind, cont } = expr;
        let bind = self.visit_var_def(bind);
        let cont = Box::new(self.walk_cexpr(*cont));
        Core::Rec(CoreRec { size, bind, cont })
    }

    fn visit_set(&mut self, expr: CoreSet) -> Core {
        let CoreSet { rec, idx, arg, cont } = expr;
        let rec = self.visit_atom(rec);
        let arg = self.visit_atom(arg);
        let cont = Box::new(self.walk_cexpr(*cont));
        Core::Set(CoreSet { rec, idx, arg, cont })
    }

    fn visit_get(&mut self, expr: CoreGet) -> Core {
        let CoreGet { rec, idx, bind, cont } = expr;
        let rec = self.visit_atom(rec);
        let bind = self.visit_var_def(bind);
        let cont = Box::new(self.walk_cexpr(*cont));
        Core::Get(CoreGet { rec, idx, bind, cont })
    }

    fn visit_halt(&mut self, arg: Atom) -> Core {
        Core::Halt(self.visit_atom(arg))
    }

    fn visit_tag(&mut self, tag: Tag, cont: Box<Core>) -> Core {
        Core::Tag(tag, Box::new(self.walk_cexpr(*cont)))
    }
}

pub trait VisitorDownTop {
    fn walk_cexpr(&mut self, expr: Core) -> Core {
        match expr {
            Core::App(expr) => self.visit_app(expr),
            Core::Let(expr) => self.visit_let(expr),
            Core::Fix(expr) => self.visit_fix(expr),
            Core::Opr(expr) => self.visit_opr(expr),
            Core::Case(expr) => self.visit_case(expr),
            Core::Rec(expr) => self.visit_rec(expr),
            Core::Set(expr) => self.visit_set(expr),
            Core::Get(expr) => self.visit_get(expr),
            Core::Halt(arg) => self.visit_halt(arg),
            Core::Tag(tag, cont) => self.visit_tag(tag, cont),
        }
    }
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

    fn visit_decl(&mut self, decl: CoreDecl) -> CoreDecl {
        let CoreDecl { func, args, body } = decl;
        let body = Box::new(self.walk_cexpr(*body));
        let func = self.visit_var_def(func);
        let args = args.iter()
            .map(|arg| self.visit_var_def(*arg))
            .collect();
        CoreDecl { func, args, body }
    }

    fn visit_app(&mut self, expr: CoreApp) -> Core {
        let CoreApp { func, args } = expr;
        let func = self.visit_atom(func);
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        Core::App(CoreApp { func, args })
    }

    fn visit_let(&mut self, expr: CoreLet) -> Core {
        let CoreLet { decl, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));
        let decl = self.visit_decl(decl);
        Core::Let(CoreLet { decl, cont })
    }

    fn visit_fix(&mut self, expr: CoreFix) -> Core {
        let CoreFix { decls, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));
        let decls = decls.into_iter()
            .map(|def| self.visit_decl(def))
            .collect();
        Core::Fix(CoreFix { decls, cont })
    }

    fn visit_opr(&mut self, expr: CoreOpr) -> Core {
        let CoreOpr { prim, args, bind, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        let bind = self.visit_var_def(bind);
        Core::Opr(CoreOpr { prim, cont, args, bind })
    }

    fn visit_case(&mut self, expr: CoreCase) -> Core {
        let CoreCase { arg, brs } = expr;
        let brs = brs.into_iter()
            .map(|br| self.walk_cexpr(br))
            .collect();
        let arg = self.visit_atom(arg);
        Core::Case(CoreCase { arg, brs })
    }

    fn visit_rec(&mut self, expr: CoreRec) -> Core {
        let CoreRec { size, bind, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));
        let bind = self.visit_var_def(bind);
        Core::Rec(CoreRec { size, bind, cont })
    }

    fn visit_set(&mut self, expr: CoreSet) -> Core {
        let CoreSet { rec, idx, arg, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));
        let rec = self.visit_atom(rec);
        let arg = self.visit_atom(arg);
        Core::Set(CoreSet { rec, idx, arg, cont })
    }

    fn visit_get(&mut self, expr: CoreGet) -> Core {
        let CoreGet { rec, idx, bind, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));
        let rec = self.visit_atom(rec);
        let bind = self.visit_var_def(bind);
        Core::Get(CoreGet { rec, idx, bind, cont })
    }

    fn visit_halt(&mut self, arg: Atom) -> Core {
        Core::Halt(self.visit_atom(arg))
    }

    fn visit_tag(&mut self, tag: Tag, cont: Box<Core>) -> Core {
        Core::Tag(tag, Box::new(self.walk_cexpr(*cont)))
    }
}
