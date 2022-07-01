use crate::ast::*;
use crate::symbol::*;
use super::*;


pub trait Visitor {
    fn walk_cexpr(&mut self, expr: Core) -> Core {
        match expr {
            Core::App(expr) => self.visit_app(expr),
            Core::Let(expr) => self.visit_let(expr),
            Core::Opr(expr) => self.visit_opr(expr),
            Core::Case(expr) => self.visit_case(expr),
            Core::Rec(expr) => self.visit_rec(expr),
            Core::Sel(expr) => self.visit_sel(expr),
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
        let args = args.iter()
            .map(|arg| self.visit_var_def(*arg))
            .collect();
        let body = Box::new(self.walk_cexpr(*body));
        CoreDecl { func, args, body }
    }

    fn visit_app(&mut self, expr: CoreApp) -> Core {
        Core::App(CoreApp {
            func:self.visit_atom(expr.func),
            args: expr.args.into_iter()
                .map(|arg| self.visit_atom(arg))
                .collect(),
        })
    }

    fn visit_let(&mut self, expr: CoreLet) -> Core {
        Core::Let(CoreLet {
            decls: expr.decls.into_iter()
                .map(|def| self.visit_decl(def))
                .collect(),
            cont: Box::new(self.walk_cexpr(*expr.cont))
        })
    }


    fn visit_opr(&mut self, expr: CoreOpr) -> Core {
        Core::Opr(CoreOpr {
            prim: expr.prim,
            args: expr.args.into_iter()
                .map(|arg| self.visit_atom(arg))
                .collect(),
            bind: self.visit_var_def(expr.bind),
            cont: Box::new(self.walk_cexpr(*expr.cont)),
        })
    }

    fn visit_case(&mut self, expr: CoreCase) -> Core {
        Core::Case(CoreCase {
            arg: self.visit_atom(expr.arg),
            brs: expr.brs.into_iter()
                .map(|br| self.walk_cexpr(br))
                .collect(),
        })
    }

    fn visit_rec(&mut self, expr: CoreRec) -> Core {
        Core::Rec(CoreRec {
            flds: expr.flds.into_iter()
                .map(|fld| self.visit_atom(fld))
                .collect(),
            bind: self.visit_var_def(expr.bind),
            cont: Box::new(self.walk_cexpr(*expr.cont)),
        })
    }
    fn visit_sel(&mut self, expr: CoreSel) -> Core {
        Core::Sel(CoreSel {
            idx: expr.idx,
            arg: self.visit_atom(expr.arg),
            bind: self.visit_var_def(expr.bind),
            cont: Box::new(self.walk_cexpr(*expr.cont)),
        })
    }

    fn visit_halt(&mut self, arg: Atom) -> Core {
        Core::Halt(self.visit_atom(arg))
    }

    fn visit_tag(&mut self, tag: Tag, cont: Box<Core>) -> Core {
        Core::Tag(tag, Box::new(self.walk_cexpr(*cont)))
    }
}
