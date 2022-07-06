use std::collections::{HashSet, HashMap};

use super::*;
use crate::symbol::*;
use super::visitor::*;

/*
    register allocation
*/

pub struct AllocScan {
    freevars: HashSet<Symbol>,
    newfree: Vec<Symbol>,
}

impl AllocScan {
    pub fn new() -> AllocScan {
        AllocScan {
            freevars: HashSet::new(),
            newfree: Vec::new(),
        }
    }
    pub fn run(expr: Core) -> Core {
        let mut ctx = AllocScan::new();
        ctx.walk_cexpr(expr)
    }
    pub fn drain_newfree(&mut self) -> Vec<Symbol> {
        self.newfree.drain(0..).collect()
    }
}

impl VisitorDownTop for AllocScan {

    fn visit_var_use(&mut self, sym: Symbol) -> Symbol {
        if !self.freevars.contains(&sym) {
            self.freevars.insert(sym);
            self.newfree.push(sym);
        }
        sym
    }

    fn visit_app(&mut self, expr: CoreApp) -> Core {
        let CoreApp { func, args } = expr;
        let func = self.visit_atom(func);
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
    
        let new = self.drain_newfree();
        if new.is_empty() {
            Core::App(CoreApp { func, args })
        } else {
            Core::Tag(Tag::VarFreeAfter(new), Box::new(
                Core::App(CoreApp { func, args })))
        }
    }

    fn visit_halt(&mut self, arg: Atom) -> Core {
        let arg = self.visit_atom(arg);

        let new = self.drain_newfree();
        if new.is_empty() {
            Core::Halt(arg)
        } else {
            Core::Tag(Tag::VarFreeAfter(new), Box::new(
                Core::Halt(arg)))
        }        
    }

    fn visit_opr(&mut self, expr: CoreOpr) -> Core {
        let CoreOpr { prim, args, bind, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));

        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        let bind = self.visit_var_def(bind);

        let new = self.drain_newfree();
        if new.is_empty() {
            Core::Opr(CoreOpr { prim, args, bind, cont })
        } else {
            Core::Opr(CoreOpr {
                prim,
                args,
                bind,
                cont: Box::new(Core::Tag(
                    Tag::VarFree(new),cont)),
            })
            
        }
    }
}

pub struct RegAlloc {
    pool: Vec<usize>,
    maxreg: usize,
    map: HashMap<Symbol,usize>,
}

impl RegAlloc {
    pub fn new() -> RegAlloc {
        RegAlloc {
            pool: Vec::new(),
            maxreg: 0,
            map: HashMap::new(),
        }
    }
    pub fn run(expr: Core) -> Core {
        let mut ctx = RegAlloc::new();
        ctx.walk_cexpr(expr)
    }
}

impl VisitorTopDown for RegAlloc {

    fn visit_var_def(&mut self, sym: Symbol) -> Symbol {
        if let Some(n) = self.pool.pop() {
            self.map.insert(sym, n);
            reg(n)
        } else {
            let old = self.maxreg;
            self.map.insert(sym, old);
            self.maxreg += 1;
            reg(old)
        }
    }

    fn visit_var_use(&mut self, sym: Symbol) -> Symbol {
        if let Some(n) = self.map.get(&sym) {
            reg(*n)
        } else {
            sym
        }
    }

    fn visit_atom(&mut self, atom: Atom) -> Atom {
        if let Atom::Var(sym) = atom {
            if let Some(n) = self.map.get(&sym) {
                Atom::Var(reg(*n))
            } else {
                Atom::Glob(sym)
            }
        } else {
            atom
        }
    }

    fn visit_decl(&mut self, decl: CoreDecl) -> CoreDecl {
        let CoreDecl { func, args, body } = decl;
        // let func = self.visit_var_def(func);

        // new enviroment
        let old_pool: Vec<usize> = self.pool.drain(0..).collect();
        let old_maxreg = self.maxreg;
        self.maxreg = 0;

        // visit
        let args = args.iter()
            .map(|arg| self.visit_var_def(*arg))
            .collect();
        let body = Box::new(self.walk_cexpr(*body));
        
        // recover old enviroment
        self.pool = old_pool.into_iter().collect();
        self.maxreg = old_maxreg;

        CoreDecl { func, args, body }
    }

    fn visit_let(&mut self, expr: CoreLet) -> Core {
        // todo: some change
        let CoreLet { decl, cont } = expr;
        let decl = self.visit_decl(decl);
        let cont = Box::new(self.walk_cexpr(*cont));
        Core::Let(CoreLet { decl, cont })
    }

    fn visit_tag(&mut self, tag: Tag, cont: Box<Core>) -> Core {
        match tag {
            Tag::VarFree(syms) => {
                for sym in syms {
                    if let Some(reg) = self.map.remove(&sym) {
                        self.pool.push(reg);
                    } else {
                        //panic!("{sym} not fount in context!");
                    }
                }
                self.walk_cexpr(*cont)
            }
            Tag::VarFreeAfter(syms) => {
                let cont = self.walk_cexpr(*cont);
                for sym in syms {
                    if let Some(reg) = self.map.remove(&sym) {
                        self.pool.push(reg);
                    } else {
                        //panic!("{sym} not fount in context!");
                    }
                }
                cont
            }
            other => {
                Core::Tag(other, cont)
            }
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
        let expr = super::cps_trans::cps_trans_top(&res);
        println!("\n{}", expr);
        let expr = AllocScan::run(expr);
        println!("\n{}", expr);
        let expr = RegAlloc::run(expr);
        println!("\n{}", expr);
    } else {
        par.print_err();
    }
}