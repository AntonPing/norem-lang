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
    pub fn run(expr: Core) -> Core {
        let mut ctx = ClosConv::new();
        ctx.walk_cexpr(expr)
    }
}

impl VisitorDownTop for ClosConv {

    fn visit_app(&mut self, expr: CoreApp) -> Core {
        /*  
            f(a,b,...,z) 
            
            =====> becomes =====>
            
            get f[0] -> f';
            f'(f,a,b,...,z)
        */

        let CoreApp { func, args } = expr;

        if let Atom::Var(sym) = &func {
            self.freevars.insert(*sym);
        }
        for arg in &args {
            if let Atom::Var(sym) = arg {
                self.freevars.insert(*sym);
            }
        }
        
        let f = genvar('f');

        Core::Get(CoreGet {
            rec: func,
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

    fn visit_let(&mut self, expr: CoreLet) -> Core {
        /*
            let foo(x,y,...,z) =
                bar; in
            baz

            =======> becomes =======>

            let f(c,x,y,...,z) = 
                get c[0] -> v0;
                ...
                get c[n] -> vn;
                bar; in
            record(n) -> foo
            set foo[0] := v0
            ...
            set foo[n] := vn
            baz

            where c and f are generated variables
        */

        let CoreLet { decl, cont } = expr;

        let CoreDecl { func, mut args, body } = decl;

        let c = genvar('c');
        let f = genvar('f');

        let body = Box::new(self.walk_cexpr(*body));
        for arg in &args {
            self.freevars.remove(arg);
        }

        // record all freevar at this point for later use
        let freevarvec : Vec<Symbol> = self.freevars.iter()
            .copied().collect();

        let body = freevarvec.iter()
            .copied().enumerate()
            .fold(
                body,
                |cont, (idx, bind)| {
                    Box::new(Core::Get(CoreGet {
                        rec: Atom::Var(c),
                        idx,
                        bind,
                        cont,
                    }))
                }
            );
        
        args.insert(0, c);
        let decl = CoreDecl {
            func: f,
            args,
            body,
        };

        let cont = Box::new(self.walk_cexpr(*cont));
        self.freevars.remove(&f);
        
        let cont = [f].iter()
            .chain(freevarvec.iter())
            .map(|sym| Atom::Var(*sym))
            .enumerate()
            .fold(
                cont,
                |cont, (idx, arg)| {
                    Box::new(Core::Set(CoreSet {
                        rec: Atom::Var(func),
                        idx,
                        arg,
                        cont,
                    }))
                },
            );

        let cont = Box::new(Core::Rec(CoreRec {
            // for each freevar and function itself
            size: freevarvec.len() + 1,
            bind: func,
            cont,
        }));

        Core::Let(CoreLet {
            decl,
            cont,
        })
    }

    fn visit_fix(&mut self, expr: CoreFix) -> Core {
        todo!()
        /*
            todo!
        

        let CoreFix { decls, cont: body } = expr;
        
        let c = genvar('c');

        let decls: Vec<CoreDecl> = decls.into_iter()
            .map(|decl| {
                let CoreDecl { func, mut args, body } = decl;
                let body = Box::new(self.walk_cexpr(*body));
                self.freevars.remove(&func);
                for arg in &args {
                    self.freevars.remove(arg);
                }

                let newbody = self.freevars.iter()
                    .copied().enumerate()
                    .fold(
                        *body,
                        |acc,(i,x)| {
                            Core::Get(CoreGet {
                                rec: Atom::Var(c),
                                idx: i,
                                bind: x,
                                cont: Box::new(acc),
                            })
                        }
                    );
                args.insert(0, c);
                CoreDecl {
                    func,
                    args,
                    body: Box::new(newbody),
                }
            })
            .collect();

        let body = Box::new(self.walk_cexpr(*body));
        // todo: support mutual recursive later
        let wrongbind = decls[0].func;

        Core::Fix(CoreFix {
            decls,
            cont: Box::new(Core::Rec(CoreRec {
                flds: [Atom::Var(wrongbind)].into_iter()
                    .chain(self.freevars.iter()
                        .map(|sym| Atom::Var(*sym)))
                    .collect(),
                bind: wrongbind,
                cont: body,
            })),
        })
        */
        
    }

    fn visit_opr(&mut self, expr: CoreOpr) -> Core {
        let CoreOpr { prim, args, bind, cont } = expr;

        let cont = Box::new(self.walk_cexpr(*cont));
        
        self.freevars.remove(&bind);
        for arg in &args {
            if let Atom::Var(x) = arg {
                self.freevars.insert(*x);
            }
        }

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
        let expr = cps_trans::cps_trans_top(&res);
        println!("\n{}", expr);

        let expr = ClosConv::run(expr);

        println!("\n{}", expr);
    } else {
        par.print_err();
    }
}