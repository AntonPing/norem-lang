use std::collections::HashSet;
use super::*;
use super::visitor::*;
use crate::symbol::*;

/*
    closure conversion
*/

pub struct ClosConv {
    freevars: HashSet<Symbol>,
}

impl ClosConv {
    pub fn new() -> ClosConv {
        ClosConv {
            freevars: HashSet::new(),
        }
    }
}

impl Visitor for ClosConv {

    fn visit_app(&mut self, expr: CoreApp) -> Core {
        /*  
            f(a,b,...,z) 
            
            =====> becomes =====>
            
            select f[0] -> f';
            f'(f,a,b,...,z)
        */

        if let Atom::Var(sym) = &expr.func {
            self.freevars.insert(*sym);
        }
        for arg in &expr.args {
            if let Atom::Var(sym) = arg {
                self.freevars.insert(*sym);
            }
        }
        let CoreApp { func, args } = expr;
        let f = genvar('f');

        Core::Sel(CoreSel {
            arg: func,
            idx: 0,
            bind: f,
            cont: Box::new(Core::App(CoreApp {
                func: Atom::Var(f),
                args: [func].into_iter()
                    .chain(args.into_iter())
                    .collect(),
            })),
        })
    }

    fn visit_halt(&mut self, arg: Atom) -> Core {
        if let Atom::Var(sym) = &arg {
            self.freevars.insert(*sym);
        }
        Core::Halt(arg)
    }

    /*
        let foo(x,y,...,z) =
            bar; in
        baz

        =======> becomes =======>

        let foo(c,x,y,...,z) = 
            select 0 c -> v1;
            ...
            select n c -> vn;
            bar; in
        record { foo, v1, ..., vn } -> foo;
        baz
    */
    fn visit_let(&mut self, decl: CDecl, cont: Box<Core>) -> Core {
        
        let mut decl = self.visit_cdecl(decl);
        for arg in &decl.args {
            self.freevars.remove(arg);
        }

        for var in &self.freevars {
            println!("free {var}");
        }

        let clos = genvar('c');

        for (i,var) in self.freevars.iter().enumerate() {
            decl.body = Box::new(Core::Select(i, 
                Atom::Var(clos), *var, decl.body));
        }

        decl.args.insert(0, clos);

        

        let mut rec = Vec::new();
        rec.push(Atom::Var(decl.func));
        for var in &self.freevars {
            rec.push(Atom::Var(*var));
        }

        
        let cont = Box::new(self.walk_cexpr(*cont));
        let newbind = decl.func;
        Core::Let(decl,
            Box::new(Core::Record(rec, newbind, cont)))

    }


    fn visit_fix(&mut self, decls: Vec<CDecl>, cont: Box<Core>) -> Core {

        todo!()
    }

    fn visit_uniop(&mut self, prim: Prim, arg: Atom, ret: Symbol, cont: Box<Core>) -> Core {
        let cont = Box::new(self.walk_cexpr(*cont));
        self.freevars.remove(&ret);
        if let Atom::Var(x) = arg {
            self.freevars.insert(x);
        }
        Core::Uniop(prim, arg, ret, cont)
    }

    fn visit_binop(
        &mut self,
        prim: Prim,
        arg1: Atom,
        arg2: Atom,
        ret: Symbol,
        cont: Box<Core>,
    ) -> Core {
        let cont = Box::new(self.walk_cexpr(*cont));
        self.freevars.remove(&ret);
        if let Atom::Var(x) = arg1 {
            self.freevars.insert(x);
        }
        if let Atom::Var(x) = arg2 {
            self.freevars.insert(x);
        }
        Core::Binop(prim, arg1, arg2, ret, cont)
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
        let cexpr = cps_trans::cps_trans_top(&res);
        println!("\n{}", cexpr);

        let cexpr = ClosConv::run(cexpr);
        println!("\n{}", cexpr);
    } else {
        par.print_err();
    }
}