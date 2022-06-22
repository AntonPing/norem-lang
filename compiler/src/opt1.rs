use std::collections::HashMap;

use crate::ast::Prim;
use crate::symbol::{Symbol, newvar, genvar};
use crate::core::*;
use crate::utils::MultiSet;
use crate::visitor::*;

/*
    level one optimizer:
    1. safe beta inline (variable that used only once)
    2. dead code elimination (variable that never used)
    3. constant folding
*/
pub struct Opt1Scan {
    change: bool,
    count: MultiSet<Symbol>,
}

impl Opt1Scan {
    pub fn run_opt1_scan(expr: CExpr) -> (CExpr,bool) {
        let mut scan = Opt1Scan::new();
        let expr = scan.walk_cexpr(expr);
        (expr, scan.change)
    }

    pub fn run_opt1_scan_loop(expr: CExpr) -> (CExpr,usize) {
        let mut times: usize = 0;
        let mut expr = expr;

        loop {
            let (new_expr,p) = Opt1Scan::run_opt1_scan(expr);
            if p {
                times += 1;
                expr = new_expr;
            } else {
                return (new_expr,times);
            }
        }
    }

    pub fn new() -> Opt1Scan {
        Opt1Scan {
            change: false,
            count: MultiSet::new(),
        }
    }
}

impl CExprVisitor for Opt1Scan {
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

    fn visit_halt(&mut self, arg: Atom) -> CExpr {
        if let Atom::Var(sym) = &arg {
            self.count.insert(*sym);
        }
        CExpr::Halt(arg)
    }


    fn visit_let(&mut self, decl: CDecl, cont: Box<CExpr>) -> CExpr {
        let decl = self.visit_cdecl(decl);
        let cont = self.walk_cexpr(*cont);

        match self.count.get(&decl.func) {
            0 => {
                // dead code elimination
                self.change = true;
                cont
            }
            1 => {
                // safe beta inlining
                self.change = true;
                CExpr::Tag(Tag::SubstApp(decl),Box::new(cont))
            }
            _ => {
                // do nothing
                CExpr::Let(decl, Box::new(cont))
            }
        }
    }


    fn visit_fix(&mut self, decls: Vec<CDecl>, cont: Box<CExpr>) -> CExpr {
        if decls.is_empty() {
            // empty fix block elimination
            return *cont;
        }


        let decls: Vec<CDecl> = decls.into_iter()
            .map(|decl| self.visit_cdecl(decl))
            .collect();

        let mut cont = self.walk_cexpr(*cont);

        let mut newdecls = Vec::new();

        for decl in decls {
            match self.count.get(&decl.func) {
                0 => {
                    // dead code elimination
                    self.change = true;
                }
                1 => {
                    // safe beta inlining
                    self.change = true;
                    cont = CExpr::Tag(Tag::SubstApp(decl),Box::new(cont));
                }
                _ => {
                    // do nothing
                    newdecls.push(decl);
                }
            }
        }
        CExpr::Fix(newdecls, Box::new(cont))
    }

    fn visit_uniop(&mut self,
        prim: Prim,
        arg: Atom,
        ret: Symbol,
        cont: Box<CExpr>
    ) -> CExpr {
        match (prim, arg) {
            (Prim::INeg, Atom::Int(x)) => {
                self.change = true;
                CExpr::Tag(Tag::SubstAtom(ret, Atom::Int(-x)), cont)
            }
            (Prim::BNot, Atom::Bool(x)) => {
                self.change = true;
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
                self.change = true;
                CExpr::Tag(Tag::SubstAtom(ret, Atom::Int(x+y)), cont)
            }
            (Prim::ISub, Atom::Int(x), Atom::Int(y)) => {
                self.change = true;
                CExpr::Tag(Tag::SubstAtom(ret, Atom::Int(x-y)), cont)
            }
            (Prim::IMul, Atom::Int(x), Atom::Int(y)) => {
                self.change = true;
                CExpr::Tag(Tag::SubstAtom(ret, Atom::Int(x*y)), cont)
            }
            (Prim::IDiv, Atom::Int(x), Atom::Int(y)) => {
                self.change = true;
                CExpr::Tag(Tag::SubstAtom(ret, Atom::Int(x/y)), cont)
            }
            _ => {
                CExpr::Binop(prim, arg1, arg2, ret, cont)
            }
        }
    }
}


pub struct Opt1Reduce {
    atom_map: HashMap<Symbol,Atom>,
    app_map: HashMap<Symbol,CDecl>,
}

impl Opt1Reduce {
    pub fn new() -> Opt1Reduce {
        Opt1Reduce {
            atom_map: HashMap::new(),
            app_map: HashMap::new()
        }
    }
    pub fn run_opt1_reduce(expr: CExpr) -> CExpr {
        let mut scan = Opt1Reduce::new();
        scan.walk_cexpr(expr)
    }
}

impl Opt1Reduce {
    pub fn subst_atom(&mut self, atom: Atom) -> Atom {
        let mut atom = atom;
        while let Atom::Var(sym) = atom {
            if let Some(res) = self.atom_map.get(&sym) {
                atom = *res
            } else {
                return atom
            }
        }
        atom
    }
}


impl CExprVisitor for Opt1Reduce {

    fn visit_var_def(&mut self, sym: Symbol) -> Symbol {
        //assert!(!self.map.contains_key(&var));
        //self.atom_map.remove(&sym);
        //self.app_map.remove(&sym);
        sym
    }

    /*
    fn visit_var_use(&mut self, sym: Symbol) -> Symbol {
        sym
    }
    */

    fn visit_atom(&mut self, atom: Atom) -> Atom {
        self.subst_atom(atom)
    }

    fn visit_app(&mut self, func: Atom, args: Vec<Atom>) -> CExpr {
        if let Atom::Var(sym) = func {
            if let Some(decl) = self.app_map.get(&sym).cloned() {
                assert_eq!(args.len(),decl.args.len());
                for (i,arg) in decl.args.iter().enumerate() {
                    self.atom_map.insert(*arg, args[i]);
                }
                
                self.walk_cexpr(*decl.body)

            } else {
                CExpr::App(
                    self.visit_atom(func),
                    args.into_iter().map(|arg| self.visit_atom(arg)).collect(),
                )
            }
        } else {
            CExpr::App(
                self.visit_atom(func),
                args.into_iter().map(|arg| self.visit_atom(arg)).collect(),
            )
        }
    }

    fn visit_tag(&mut self, tag: Tag, cont: Box<CExpr>) -> CExpr {
        match tag {
            Tag::SubstAtom(k, v) => {
                self.atom_map.insert(k, v);
                self.walk_cexpr(*cont)
            }
            Tag::SubstApp(decl) => {
                self.app_map.insert(decl.func,decl);
                self.walk_cexpr(*cont)
            }
            other => {
                let res = self.walk_cexpr(*cont);
                CExpr::Tag(other, Box::new(res))
            }
        }
    }
}

pub fn opt_level1(expr: CExpr) -> CExpr {
    let mut expr = expr;

    loop {
        let (new_expr, times) = Opt1Scan::run_opt1_scan_loop(expr);
        if times == 0 {
            return new_expr;
        } else {
            expr = Opt1Reduce::run_opt1_reduce(new_expr);
        }
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

        let mut reduce = Opt1Reduce::new();
        let cexpr = reduce.walk_cexpr(cexpr);
        println!("\n{}", cexpr);

        let cexpr = opt_level1(cexpr);
        println!("\n{}", cexpr);
    } else {
        par.print_err();
    }
}