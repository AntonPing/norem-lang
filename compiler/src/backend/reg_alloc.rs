use std::collections::HashSet;
use crate::backend::*;
use super::visitor::*;

/*
    register allocation
*/

pub struct DefUseScan {
    usevars: HashSet<Symbol>,
}

impl DefUseScan {
    pub fn new() -> DefUseScan {
        DefUseScan {
            usevars: HashSet::new(),
        }
    }
}

impl CExprVisitor for DefUseScan {

    fn visit_app(&mut self, func: Atom, mut args: Vec<Atom>) -> CExpr {
        // f(a,b,...,z) => 
        // tag(use f) { tag(use a) { ... { f(a,b,...,z)}}}
        let vec : Vec<Symbol> = [func].iter()
            .chain(args.iter())
            .filter_map(|elem|
                if let Atom::Var(sym) = elem
                { Some(*sym) } else { None })
            .collect();

        vec.into_iter().fold(
            CExpr::App(func, args), 
        |init, elem|{
            CExpr::Tag(Tag::VarUse(elem), Box::new(init))
        })
    }

    fn visit_halt(&mut self, arg: Atom) -> CExpr {
        /*
            halt(x) ===> tag(use x) { halt(x) }
        */
        if let Atom::Var(sym) = &arg {
            CExpr::Tag(Tag::VarUse(*sym), Box::new(
                CExpr::Halt(arg)
            ))
        } else {
            CExpr::Halt(arg)
        }
    }

    fn visit_uniop(&mut self, prim: Prim, arg: Atom, ret: Symbol, cont: Box<CExpr>) -> CExpr {
        
        let cont = Box::new(self.walk_cexpr(*cont));

        if let Atom::Var(sym) = &arg {
            CExpr::Tag(Tag::VarUse(*sym), Box::new(
                CExpr::Uniop(prim, arg, ret, cont)))
        } else {
            CExpr::Uniop(prim, arg, ret, cont)
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
        let cont = Box::new(self.walk_cexpr(*cont));

        

        CExpr::Binop(prim, arg1, arg2, ret, cont)
    }

    fn visit_let(&mut self, decl: CDecl, cont: Box<CExpr>) -> CExpr {
        
        let mut decl = self.visit_cdecl(decl);
        for arg in &decl.args {
            self.usevars.remove(arg);
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
        let cexpr = cps::cps_trans_top(&res);
        println!("\n{}", cexpr);

        let cexpr = ClosConv::run(cexpr);
        println!("\n{}", cexpr);
    } else {
        par.print_err();
    }
}