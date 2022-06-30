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

impl Visitor for Opt1Scan {
    fn visit_app(&mut self, expr: CoreApp) -> Core {
        if let Atom::Var(sym) = &expr.func {
            self.ref_count.insert(*sym);
            self.call_count.insert(*sym);
        }
        for arg in &expr.args {
            if let Atom::Var(sym) = arg {
                self.ref_count.insert(*sym);
            }
        }
        Core::App(expr)
    }

    fn visit_halt(&mut self, arg: Atom) -> Core {
        if let Atom::Var(sym) = &arg {
            self.ref_count.insert(*sym);
        }
        Core::Halt(arg)
    }

    fn visit_let(&mut self, expr: CoreLet) -> Core {
        let CoreLet { decls, body } = expr;
        let body = Box::new(self.walk_cexpr(*body));

        // let (emtpy) in foo =====> foo
        if decls.is_empty() {
            return *body;
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

        Core::Let(CoreLet {
            decls,
            body: tagvec.into_iter()
                .fold(body, |acc, x|
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

        // constant folding
        match args.len() {
            0 => {
                todo!()
            }
            1 => {
                match (&prim, &args[0]) {
                    (Prim::INeg, Atom::Int(x)) => {
                        self.change = true;
                        Core::Tag(Tag::SubstAtom(bind, Atom::Int(-x)), cont)
                    }
                    (Prim::BNot, Atom::Bool(x)) => {
                        self.change = true;
                        Core::Tag(Tag::SubstAtom(bind, Atom::Bool(!x)), cont)
                    }
                    _ => {
                        Core::Opr(CoreOpr { prim, args, bind, cont })
                    }
                }
            }
            2 => {
                match (&prim, &args[0], &args[1]) {
                    (Prim::IAdd, Atom::Int(x), Atom::Int(y)) => {
                        self.change = true;
                        Core::Tag(Tag::SubstAtom(bind, Atom::Int(x+y)), cont)
                    }
                    (Prim::ISub, Atom::Int(x), Atom::Int(y)) => {
                        self.change = true;
                        Core::Tag(Tag::SubstAtom(bind, Atom::Int(x-y)), cont)
                    }
                    (Prim::IMul, Atom::Int(x), Atom::Int(y)) => {
                        self.change = true;
                        Core::Tag(Tag::SubstAtom(bind, Atom::Int(x*y)), cont)
                    }
                    (Prim::IDiv, Atom::Int(x), Atom::Int(y)) => {
                        self.change = true;
                        Core::Tag(Tag::SubstAtom(bind, Atom::Int(x/y)), cont)
                    }
                    _ => {
                        Core::Opr(CoreOpr { prim, args, bind, cont })
                    }
                }
            }
            _ => {
                todo!()
            }
        }        
    }
}


pub struct Opt1Reduce {
    atom_map: HashMap<Symbol,Atom>,
    app_map: HashMap<Symbol,CoreDecl>,
}

impl Opt1Reduce {
    pub fn new() -> Opt1Reduce {
        Opt1Reduce {
            atom_map: HashMap::new(),
            app_map: HashMap::new()
        }
    }
    pub fn run_opt1_reduce(expr: Core) -> Core {
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


impl Visitor for Opt1Reduce {
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

        Core::App(CoreApp {
            func: self.visit_atom(func),
            args: args.into_iter()
                .map(|arg| self.visit_atom(arg))
                .collect(),
        })
    }
    /*
    fn visit_opr(&mut self, expr: CoreOpr) -> Core {
        let CoreOpr { prim, args, bind, cont } = expr;

        Core::Opr(CoreOpr {
            prim,
            args: args.into_iter()
                .map(|arg| self.visit_atom(arg))
                .collect(),
            bind,
            cont: Box::new(self.walk_cexpr(*cont)),
        })
    }
    */

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
        if scan.change {
            let mut reduce = Opt1Reduce::new();
            expr = reduce.walk_cexpr(expr);
            println!("\n{n}:\n{}", expr);
        } else {
            println!("\n{n}:\n{}", expr);
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