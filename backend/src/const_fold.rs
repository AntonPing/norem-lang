use std::collections::{HashMap, HashSet};


use super::*;
use crate::ast::*;
use crate::visitor::Visitor;

pub struct ConstFold {
    atom_map: HashMap<Symbol,Atom>,
    app_map: HashMap<Symbol,Decl>,
    setget_map: HashMap<(Symbol,usize),Atom>,
    change: usize,
}

impl ConstFold {
    pub fn new() -> ConstFold {
        ConstFold {
            atom_map: HashMap::new(),
            app_map: HashMap::new(),
            setget_map: HashMap::new(),
            change: 0,
        }
    }
    pub fn run(expr: Expr) -> Expr {
        let mut scan = ConstFold::new();
        scan.walk_expr(expr)
    }
}

impl Visitor for ConstFold {
    fn visit_var_def(&mut self, sym: Symbol) -> Symbol {
        self.atom_map.remove(&sym);
        self.app_map.remove(&sym);
        sym
    }

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

    /*
    fn visit_app(&mut self, expr: ExprApp) -> Expr {
        let ExprApp { func, args } = &expr;

        if let Atom::Var(sym) = func {
            if let Some(decl) = self.app_map.get(&sym).cloned() {
                assert_eq!(args.len(), decl.args.len());
                for (i,arg) in decl.args.iter().enumerate() {
                    self.atom_map.insert(*arg, args[i]);
                }
                return self.walk_expr(decl.body);
            }
        }

        self.walk_app(expr)
    }
    */

    fn visit_opr(&mut self, expr: ExprOpr) -> Expr {
        let ExprOpr { prim, args, binds, cont } = expr;
        let args: Vec<Atom> = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();
        let binds: Vec<Symbol> = binds.into_iter()
            .map(|bind| self.visit_var_def(bind))
            .collect();
        
        match prim {
            Prim::IAdd => {
                if let (Atom::Int(x), Atom::Int(y)) = (args[0], args[1]) {
                    self.change += 1;
                    self.atom_map.insert(binds[0],  Atom::Int(x+y));
                    return self.walk_expr(*cont);
                }
            }
            Prim::ISub => {
                if let (Atom::Int(x), Atom::Int(y)) = (args[0], args[1]) {
                    self.change += 1;
                    self.atom_map.insert(binds[0],  Atom::Int(x-y));
                    return self.walk_expr(*cont);
                }
            }
            Prim::IMul => {
                if let (Atom::Int(x), Atom::Int(y)) = (args[0], args[1]) {
                    self.change += 1;
                    self.atom_map.insert(binds[0],  Atom::Int(x*y));
                    return self.walk_expr(*cont);
                }
            }
            Prim::IDiv => {
                if let (Atom::Int(x), Atom::Int(y)) = (args[0], args[1]) {
                    self.change += 1;
                    self.atom_map.insert(binds[0],  Atom::Int(x/y));
                    return self.walk_expr(*cont);
                }
            }
            Prim::INeg => {
                if let Atom::Int(x) = args[0] {
                    self.change += 1;
                    self.atom_map.insert(binds[0],  Atom::Int(-x));
                    return self.walk_expr(*cont);
                }
            }
            Prim::BNot => {
                if let Atom::Bool(x) = args[0] {
                    self.change += 1;
                    self.atom_map.insert(binds[0],  Atom::Bool(!x));
                    return self.walk_expr(*cont);
                }
            }
        }

        let cont = Box::new(self.walk_expr(*cont));

        Expr::Opr(ExprOpr { prim, args, binds, cont })

    }

    fn visit_brs(&mut self, expr: ExprBrs) -> Expr {
        
        let ExprBrs { prim, args, brs } = expr;
        let args: Vec<Atom> = args.into_iter()
            .map(|arg| self.visit_atom(arg))
            .collect();

        match prim {
            BrsPrim::Switch => {
                if let Atom::Int(x) = args[0] {
                    self.change += 1;
                    return self.walk_expr(brs.into_iter().nth(x as usize).unwrap());
                }
            }
        }

        Expr::Brs(ExprBrs { prim, args, brs })
    }

    fn visit_set(&mut self, expr: ExprSet) -> Expr {
        let ExprSet { rec, idx, arg, cont } = &expr;

        let sym = rec.unwrap_var();
        self.setget_map.insert((sym,*idx),*arg);

        self.walk_set(expr)
    }

    fn visit_get(&mut self, expr: ExprGet) -> Expr {
        let ExprGet { rec, idx, bind, cont } = &expr;

        let sym = rec.unwrap_var();
        if let Some(atom) = self.setget_map.get(&(sym,*idx)) {
            // set-get optimization
            self.atom_map.insert(sym, *atom);
            return self.walk_expr(*expr.cont);
        }

        self.walk_get(expr)
    }

    /*
    fn visit_tag(&mut self, tag: Tag, cont: Box<Expr>) -> Expr {
        match tag {
            Tag::SubstAtom(k, v) => {
                self.atom_map.insert(k, v);
                self.walk_expr(*cont)
            }
            Tag::SubstApp(decl) => {
                self.app_map.insert(decl.func,decl);
                self.walk_expr(*cont)
            }
            other => {
                Expr::Tag(other,
                    Box::new(self.walk_expr(*cont)))
            }
        }
    }
    */
}


/*
pub fn opt_level1(expr: Expr) -> Expr {
    let mut expr = expr;
    let mut n = 0;
    
    loop {
        n += 1;

        let mut scan = Opt1Scan::new();
        expr = scan.walk_cexpr(expr);
        println!("\nafter scan {n}:\n{}", expr);
        let mut reduce = ConstFold::new();
        expr = reduce.walk_expr(expr);
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
*/