use std::collections::HashSet;
use crate::backend::*;
use super::visitor::*;

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
    pub fn run(expr: CExpr) -> CExpr {
        let mut clos = ClosConv::new();
        clos.walk_cexpr(expr)
    }
}

impl CExprVisitor for ClosConv {
    /*  
        f(a,b,...,z) 
        
        ======> becomes =======>
        
        select f -> f';
        f'(f,a,b,...,z)
    */
    fn visit_app(&mut self, func: Atom, mut args: Vec<Atom>) -> CExpr {
        if let Atom::Var(sym) = &func {
            self.freevars.insert(*sym);
        }
        for arg in &args {
            if let Atom::Var(sym) = arg {
                self.freevars.insert(*sym);
            }
        }
        let func2 = genvar('c');
        args.insert(0, func);
        CExpr::Select(0, func, func2, Box::new(
            CExpr::App(Atom::Var(func2), args)
        ))
    }

    fn visit_halt(&mut self, arg: Atom) -> CExpr {
        if let Atom::Var(sym) = &arg {
            self.freevars.insert(*sym);
        }
        CExpr::Halt(arg)
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
    fn visit_let(&mut self, decl: CDecl, cont: Box<CExpr>) -> CExpr {
        
        let mut decl = self.visit_cdecl(decl);
        for arg in &decl.args {
            self.freevars.remove(arg);
        }

        for var in &self.freevars {
            println!("free {var}");
        }

        let clos = genvar('c');

        for (i,var) in self.freevars.iter().enumerate() {
            decl.body = Box::new(CExpr::Select(i, 
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
        CExpr::Let(decl,
            Box::new(CExpr::Record(rec, newbind, cont)))

    }


    fn visit_fix(&mut self, decls: Vec<CDecl>, cont: Box<CExpr>) -> CExpr {

        todo!()
    }

    fn visit_uniop(&mut self, prim: Prim, arg: Atom, ret: Symbol, cont: Box<CExpr>) -> CExpr {
        let cont = Box::new(self.walk_cexpr(*cont));
        self.freevars.remove(&ret);
        if let Atom::Var(x) = arg {
            self.freevars.insert(x);
        }
        CExpr::Uniop(prim, arg, ret, cont)
    }

    fn visit_binop(
        &mut self,
        prim: Prim,
        arg1: Atom,
        arg2: Atom,
        ret: Symbol,
        cont: Box<CExpr>,
    ) -> CExpr {
        let cont = Box::new(self.walk_cexpr(*cont));
        self.freevars.remove(&ret);
        if let Atom::Var(x) = arg1 {
            self.freevars.insert(x);
        }
        if let Atom::Var(x) = arg2 {
            self.freevars.insert(x);
        }
        CExpr::Binop(prim, arg1, arg2, ret, cont)
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