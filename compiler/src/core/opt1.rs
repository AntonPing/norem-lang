use std::collections::HashMap;

use super::*;
use super::visitor::*;

use crate::ast::*;
use crate::symbol::*;
use crate::utils::MultiSet;


/*
    level one optimizer:
    1. safe beta inline (variable that used only once)
    2. dead code elimination (variable that never used)
    3. constant folding
*/
pub struct Opt1Scan {
    change: bool,
    ref_count: MultiSet<Symbol>,
    call_count: MultiSet<Symbol>,

}

impl Opt1Scan {
    pub fn new() -> Opt1Scan {
        Opt1Scan {
            change: false,
            ref_count: MultiSet::new(),
            call_count: MultiSet::new(),
        }
    }
}

impl VisitorDownTop for Opt1Scan {
    fn visit_app(&mut self, expr: CoreApp) -> Core {
        let CoreApp { func, args } = expr;

        if let Atom::Var(sym) = &expr.func {
            self.ref_count.insert(*sym);
            self.call_count.insert(*sym);
        }
        for arg in &args {
            if let Atom::Var(sym) = arg {
                self.ref_count.insert(*sym);
            }
        }

        let func = self.visit_atom(func);
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        Core::App(CoreApp { func, args })
    }

    fn visit_halt(&mut self, arg: Atom) -> Core {
        if let Atom::Var(sym) = &arg {
            self.ref_count.insert(*sym);
        }

        let arg = self.visit_atom(arg);
        Core::Halt(arg)
    }

    fn visit_let(&mut self, expr: CoreLet) -> Core {
        let CoreLet { decl, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));

        if self.ref_count.remove_all(&decl.func) == 0 {
            // dead code elimination
            assert_eq!(self.call_count.remove_all(&decl.func),0);
            self.change = true;
            *cont
        } else if self.call_count.remove_all(&decl.func) == 1 {
            // safe beta inlining
            self.change = true;
            let decl = self.visit_decl(decl);
            Core::Tag(Tag::SubstApp(decl.clone()),
                Box::new(Core::Let(CoreLet { decl, cont })))
        } else {
            // no changes
            let decl = self.visit_decl(decl);
            Core::Let(CoreLet { decl, cont })
        }
    }

    fn visit_fix(&mut self, expr: CoreFix) -> Core {

        /*
            todo!
        */

        let CoreFix { decls, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));

        // let (emtpy) in foo =====> foo
        if decls.is_empty() {
            return *cont;
        }

        let mut tagvec: Vec<Tag> = Vec::new();
        
        let decls = decls.into_iter()
            .filter_map(|decl| {
                if self.ref_count.remove_all(&decl.func) == 0 {
                    // dead code elimination
                    assert_eq!(self.call_count.remove_all(&decl.func),0);
                    self.change = true;
                    None
                } else if self.call_count.remove_all(&decl.func) == 1 {
                    // safe beta inlining
                    self.change = true;
                    tagvec.push(Tag::SubstApp(decl.clone()));
                    Some(self.visit_decl(decl))
                } else {
                    // no changes
                    Some(self.visit_decl(decl))
                }
            })
            .collect();

        Core::Fix(CoreFix {
            decls,
            cont: tagvec.into_iter()
                .fold(cont, |acc, x|
                    Box::new(Core::Tag(x, acc)))
        })
    }

    fn visit_opr(&mut self, expr: CoreOpr) -> Core {
        let CoreOpr { prim, args, bind, cont } = expr;
        let cont = Box::new(self.walk_cexpr(*cont));
        
        if self.ref_count.remove_all(&bind) == 0 {
            // dead code elimination
            assert_eq!(self.call_count.remove_all(&bind),0);
            self.change = true;
            return *cont;
        }
        for arg in &args {
            if let Atom::Var(sym) = &arg {
                self.ref_count.insert(*sym);
            }
        }

        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        let bind = self.visit_var_def(bind);
        Core::Opr(CoreOpr { prim, cont, args, bind })
    }
}


pub struct Opt1Reduce {
    atom_map: HashMap<Symbol,Atom>,
    app_map: HashMap<Symbol,CoreDecl>,
    setget_map: HashMap<(Symbol,usize),Atom>,
    change: bool,
}

impl Opt1Reduce {
    pub fn new() -> Opt1Reduce {
        Opt1Reduce {
            atom_map: HashMap::new(),
            app_map: HashMap::new(),
            setget_map: HashMap::new(),
            change: false,
        }
    }
    pub fn run(expr: Core) -> Core {
        let mut scan = Opt1Reduce::new();
        scan.walk_cexpr(expr)
    }
}

impl VisitorTopDown for Opt1Reduce {
    fn visit_atom(&mut self, atom: Atom) -> Atom {
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

    fn visit_app(&mut self, expr: CoreApp) -> Core {
        let CoreApp { func, args } = expr;

        if let Atom::Var(sym) = &func {
            if let Some(decl) = self.app_map.get(&sym).cloned() {
                assert_eq!(args.len(), decl.args.len());
                for (i,arg) in decl.args.iter().enumerate() {
                    self.atom_map.insert(*arg, args[i]);
                }
                return self.walk_cexpr(*decl.body);
            }
        }

        let func = self.visit_atom(func);
        let args = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        Core::App(CoreApp { func, args })
    }
    fn visit_opr(&mut self, expr: CoreOpr) -> Core {
        let CoreOpr { prim, args, bind, cont } = expr;
        let args: Vec<Atom> = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        let bind = self.visit_var_def(bind);
        
        // constant folding
        match args.len() {
            0 => {
                todo!()
            }
            1 => {
                match (&prim, &args[0]) {
                    (Prim::INeg, Atom::Int(x)) => {
                        self.change = true;
                        self.atom_map.insert(bind,  Atom::Int(-x));
                        self.walk_cexpr(*cont)
                    }
                    (Prim::BNot, Atom::Bool(x)) => {
                        self.change = true;
                        self.atom_map.insert(bind,  Atom::Bool(!x));
                        self.walk_cexpr(*cont)
                    }
                    _ => {
                        let cont = Box::new(self.walk_cexpr(*cont));
                        Core::Opr(CoreOpr { prim, args, bind, cont })
                    }
                }
            }
            2 => {
                match (&prim, &args[0], &args[1]) {
                    (Prim::IAdd, Atom::Int(x), Atom::Int(y)) => {
                        self.change = true;
                        self.atom_map.insert(bind,  Atom::Int(x+y));
                        self.walk_cexpr(*cont)
                    }
                    (Prim::ISub, Atom::Int(x), Atom::Int(y)) => {
                        self.change = true;
                        self.atom_map.insert(bind,  Atom::Int(x-y));
                        self.walk_cexpr(*cont)
                    }
                    (Prim::IMul, Atom::Int(x), Atom::Int(y)) => {
                        self.change = true;
                        self.atom_map.insert(bind,  Atom::Int(x*y));
                        self.walk_cexpr(*cont)
                    }
                    (Prim::IDiv, Atom::Int(x), Atom::Int(y)) => {
                        self.change = true;
                        self.atom_map.insert(bind,  Atom::Int(x/y));
                        self.walk_cexpr(*cont)
                    }
                    _ => {
                        let cont = Box::new(self.walk_cexpr(*cont));
                        Core::Opr(CoreOpr { prim, args, bind, cont })
                    }
                }
            }
            _ => {
                todo!()
            }
        }
    }

    fn visit_set(&mut self, expr: CoreSet) -> Core {
        let CoreSet { rec, idx, arg, cont } = expr;

        if let Atom::Var(sym) = rec {
            self.setget_map.insert((sym,idx), arg);
        }

        Core::Set(CoreSet {
            rec: self.visit_atom(rec),
            idx,
            arg: self.visit_atom(arg),
            cont: Box::new(self.walk_cexpr(*cont)),
        })
    }

    fn visit_get(&mut self, expr: CoreGet) -> Core {
        let CoreGet { rec, idx, bind, cont } = expr;

        if let Atom::Var(sym) = rec {
            if let Some(atom) = self.setget_map.get(&(sym,idx)) {
                // set-get optimization
                self.atom_map.insert(sym, *atom);
                return self.walk_cexpr(*cont);
            }
        }

        Core::Get(CoreGet {
            rec: self.visit_atom(rec),
            idx,
            bind: self.visit_var_def(bind),
            cont: Box::new(self.walk_cexpr(*cont)),
        })
    }


    fn visit_tag(&mut self, tag: Tag, cont: Box<Core>) -> Core {
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
                Core::Tag(other,
                    Box::new(self.walk_cexpr(*cont)))
            }
        }
    }
}

pub fn opt_level1(expr: Core) -> Core {
    let mut expr = expr;
    let mut n = 0;
    
    loop {
        n += 1;

        let mut scan = Opt1Scan::new();
        expr = scan.walk_cexpr(expr);
        println!("\nafter scan {n}:\n{}", expr);
        let mut reduce = Opt1Reduce::new();
        expr = reduce.walk_cexpr(expr);
        println!("\nafter reduce {n}:\n{}", expr);
        if scan.change || reduce.change {
            continue;
        } else {
            return expr;
        }
    }
}


#[test]
fn opt_test() {
    use crate::parser::*;
    
    let string = "
        (fn x y => (* (+ x 1) (- y 2))) 3 4
    ";
    
    /*
    let string = "
        (fn f g x => ((f x) (g x))) + (fn x => x) 5
    ";
    */
    let mut par = Parser::new(string);

    let res = parse_program(&mut par);
    if let Ok(res) = res {
        println!("\n{res}");
        let expr = cps_trans::cps_trans_top(&res);
        println!("\n{}", expr);

        let expr = opt_level1(expr);
        println!("\n{}", expr);
    } else {
        par.print_err();
    }
}