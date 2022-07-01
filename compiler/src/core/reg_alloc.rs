use std::collections::{HashSet, HashMap};


use super::*;
use crate::symbol::*;
use super::visitor::*;

/*
    register allocation
*/

pub struct AllocScan {
    freevars: HashSet<Symbol>,
}

impl AllocScan {
    pub fn new() -> AllocScan {
        AllocScan {
            freevars: HashSet::new(),
        }
    }
    pub fn run(expr: Core) -> Core {
        let mut ctx = AllocScan::new();
        ctx.walk_cexpr(expr)
    }
}

impl Visitor for AllocScan {
    fn visit_app(&mut self, expr: CoreApp) -> Core {


        let syms = [expr.func].iter()
            .chain(expr.args.iter())
            .filter_map(|atom| {
                if let Atom::Var(sym) = atom {
                    if !self.freevars.contains(sym) {
                        self.freevars.insert(*sym);
                        Some(sym)
                    } else {
                        panic!("application is the last commond!");
                    }
                } else {
                    None
                }
            })
            .copied().collect();
            
        Core::Tag(Tag::VarFreeEnd(syms),
            Box::new(Core::App(expr)))
    }

    fn visit_halt(&mut self, arg: Atom) -> Core {
        if let Atom::Var(sym) = &arg {
            if !self.freevars.contains(&sym) {
                self.freevars.insert(*sym);
                Core::Tag(Tag::VarFreeEnd(vec![*sym]), 
                    Box::new(Core::Halt(arg)))
            } else {
                Core::Halt(arg)
            }
        } else {
            Core::Halt(arg)
        }
        
    }

    fn visit_opr(&mut self, expr: CoreOpr) -> Core {
        let CoreOpr { prim, args, bind, cont } = expr;

        let cont = Box::new(self.walk_cexpr(*cont));
        let mut tagvec = Vec::new();
        
        for arg in &args {
            if let Atom::Var(sym) = arg {
                if !self.freevars.contains(&sym) {
                    self.freevars.insert(*sym);
                    tagvec.push(Tag::VarFree(*sym));
                }
            }
        }

        if self.freevars.contains(&bind) {
            self.freevars.remove(&bind);
            tagvec.push(Tag::VarAlloc(bind));
        } else {
            panic!("a variable that never used! it should be eliminated after opt1 pass!");
        }

        let cont = tagvec.into_iter()
            .fold(cont,
                |cont,tag| Box::new(Core::Tag(tag, cont)));
        
        Core::Opr(CoreOpr { prim, args, bind, cont })
    }
    
    fn visit_let(&mut self, expr: CoreLet) -> Core {
        // we assume there are no free variables in each decl
        // because they are eliminated by clos_conv pass

        let CoreLet { decls, cont } = expr;

        let cont = Box::new(self.walk_cexpr(*cont));

        let decls = decls.into_iter()
            .map(|decl| {
                let CoreDecl { func, args, body } = decl;
                let body = Box::new(self.walk_cexpr(*body));
                let body = args.iter()
                    .fold(body, |body, arg| {
                        if self.freevars.contains(&arg) {
                            Box::new(Core::Tag(Tag::VarAlloc(*arg), body))
                        } else {
                            // argument never used
                            body
                        }
                    });
                CoreDecl { func, args, body }
            })
            .collect();


        Core::Let(CoreLet { decls, cont })
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

impl Visitor for RegAlloc {

    fn visit_atom(&mut self, atom: Atom) -> Atom {
        if let Atom::Var(sym) = atom {
            if let Some(n) = self.map.get(&sym) {
                Atom::Var(reg(*n))
            } else {
                //panic!("{sym} not fount in context!");
                Atom::Var(sym) // for testing, this should be permitted!
            }
            
        } else {
            atom
        }
    }
    fn visit_app(&mut self, expr: CoreApp) -> Core {
        let CoreApp { func, args } = expr;
        let func = self.visit_atom(func);

        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        
        Core::App(CoreApp { func, args })
    }

    fn visit_halt(&mut self, arg: Atom) -> Core {
        Core::Halt(self.visit_atom(arg))
    }

    fn visit_tag(&mut self, tag: Tag, cont: Box<Core>) -> Core {
        match tag {
            Tag::VarAlloc(sym) => {
                if let Some(reg) = self.pool.pop() {
                    self.map.insert(sym, reg);
                } else {
                    self.map.insert(sym, self.maxreg);
                    self.maxreg += 1;
                }
                self.walk_cexpr(*cont)
            }
            Tag::VarFree(sym) => {
                if let Some(reg) = self.map.remove(&sym) {
                    self.pool.push(reg);
                } else {
                    //panic!("{sym} not fount in context!");
                }
                self.walk_cexpr(*cont)
            }
            Tag::VarFreeEnd(syms) => {
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

    fn visit_opr(&mut self, expr: CoreOpr) -> Core {
        let CoreOpr { prim, args, bind, cont } = expr;

        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();

        let bind = if let Some(n) = self.map.get(&bind) {
            reg(*n)
        } else {
            //panic!("{sym} not fount in context!");
            bind // for testing, this should be permitted!
        };
        
        let cont = Box::new(self.walk_cexpr(*cont));

        Core::Opr(CoreOpr { prim, args, bind, cont })

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