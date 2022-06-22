use std::collections::HashMap;

use crate::ast::Prim;
use crate::symbol::{Symbol, newvar, genvar};
use crate::core::*;
use crate::utils::MultiSet;
use crate::visitor::*;


pub struct Subst {
    map: HashMap<Symbol,Atom>,
}

impl Subst {
    pub fn new() -> Subst {
        Subst { map: HashMap::new() }
    }
    pub fn run(expr: CExpr) -> CExpr {
        let mut pass = Subst::new();
        pass.walk_cexpr(expr)
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

pub struct DeadElim {
    count: MultiSet<Symbol>
}

impl DeadElim {
    pub fn new() -> DeadElim {
        DeadElim { count: MultiSet::new() }
    }
    pub fn run(expr: CExpr) -> CExpr {
        let mut pass = DeadElim::new();
        pass.walk_cexpr(expr)
    }
}

impl CExprVisitor for DeadElim {
    fn visit_app(&mut self, func: Atom, args: Vec<Atom>) -> CExpr {
        if let Atom::Var(sym) = &func {
            self.count.insert(*sym);
        }
        for arg in &args {
            if let Atom::Var(sym) = arg {
                self.count.insert(*sym);
            }
        }
        CExpr::App(func, args)
    }

    fn visit_let(&mut self, decl: CDecl, cont: Box<CExpr>) -> CExpr {
        let decl = self.visit_cdecl(decl);
        let cont = self.walk_cexpr(*cont);

        if self.count.get(&decl.func) == 0 {
            cont
        } else {
            CExpr::Let(decl, Box::new(cont))
        }
    }

    /* todo
    fn visit_fix(&mut self, decls: Vec<CDecl>, cont: Box<CExpr>) -> CExpr {
        let decl = self.visit_cdecl(decl);
        let cont = self.walk_cexpr(*cont);

        if self.count.get(&decl.func) == 0 {
            cont
        } else {
            CExpr::Let(decl, Box::new(cont))
        }
    }
    */

}

pub struct SafeInline {
    count: MultiSet<Symbol>
}

impl SafeInline {
    pub fn new() -> SafeInline {
        SafeInline { count: MultiSet::new() }
    }
    pub fn run(expr: CExpr) -> CExpr {
        let mut pass = SafeInline::new();
        pass.walk_cexpr(expr)
    }
}

pub struct ConstFold {}

impl ConstFold {
    pub fn new() -> ConstFold {
        ConstFold {}
    }
    pub fn run(expr: CExpr) -> CExpr {
        let mut pass = ConstFold::new();
        let expr = pass.walk_cexpr(expr);
        Subst::run(expr)
    }
}
/*
fn is_const_val(atom: Atom) -> bool {
    match atom {
        Atom::Int(_) => true,
        Atom::Real(_) => todo!(),
        Atom::Bool(_) => todo!(),
        Atom::Char(_) => todo!(),
    }
}
*/

impl CExprVisitor for ConstFold {
    fn visit_uniop(&mut self,
        prim: Prim,
        arg: Atom,
        ret: Symbol,
        cont: Box<CExpr>
    ) -> CExpr {
        match (prim, arg) {
            (Prim::INeg, Atom::Int(x)) => {
                CExpr::Tag(Tag::SubstAtom(ret, Atom::Int(-x)), cont)
            }
            (Prim::BNot, Atom::Bool(x)) => {
                CExpr::Tag(Tag::SubstAtom(ret, Atom::Bool(!x)), cont)
            }
            _ => {
                CExpr::Uniop(prim, arg, ret, cont)
            }
        }
    }

    fn visit_binop(
            &mut self,
            prim: Prim,
            arg1: Atom,
            arg2: Atom,
            ret: Symbol,
            cont: Box<CExpr>,
        ) -> CExpr {
        match (prim, arg1, arg2) {
            (Prim::IAdd, Atom::Int(x), Atom::Int(y)) => {
                CExpr::Tag(Tag::SubstAtom(ret, Atom::Int(x+y)), cont)
            }
            (Prim::ISub, Atom::Int(x), Atom::Int(y)) => {
                CExpr::Tag(Tag::SubstAtom(ret, Atom::Int(x-y)), cont)
            }
            (Prim::IMul, Atom::Int(x), Atom::Int(y)) => {
                CExpr::Tag(Tag::SubstAtom(ret, Atom::Int(x*y)), cont)
            }
            (Prim::IDiv, Atom::Int(x), Atom::Int(y)) => {
                CExpr::Tag(Tag::SubstAtom(ret, Atom::Int(x/y)), cont)
            }
            _ => {
                CExpr::Binop(prim, arg1, arg2, ret, cont)
            }
        }
    }
}

#[test]
fn opt_test() {
    use crate::parser::*;
    let string = "
        let
            val a = 1
            val b = 2
        in
            (fn f x y => (f (+ x a) (- y b))) * 3 4
        end
    ";
    let mut par = Parser::new(string);
    par.next().unwrap();


    let res = parse_expr(&mut par);
    if let Ok(res) = res {
        println!("{res}");
        let cexpr = cps_trans_top(&res);
        println!("{}", cexpr);

        let cexpr = Subst::run(cexpr);
        println!("{}", cexpr);

        let cexpr = DeadElim::run(cexpr);
        println!("{}", cexpr);

        let cexpr = ConstFold::run(cexpr);
        println!("{}", cexpr);

        println!("{}", cexpr);
    } else {
        par.print_err();
    }
}