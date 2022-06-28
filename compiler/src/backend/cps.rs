use crate::backend::*;
use super::visitor::*;
use crate::utils::Span;

pub fn cps_trans(expr: &Expr, hole: Symbol, cont: Box<CExpr>) -> CExpr {
    /*
    if cfg!(test) {
        println!("In context fn {} => {}", hole, &cont);
        println!("Eval expresssion {}", expr);
    }
    */

    match expr {
        Expr::Lit(ExprLit { lit, span: _ }) => {
            // literals just fill into the hole
            let atom = match lit {
                LitVal::Int(x) => Atom::Int(*x),
                LitVal::Real(x) => Atom::Real(*x),
                LitVal::Bool(x) => Atom::Bool(*x),
                LitVal::Char(x) => Atom::Char(*x),
            };
            CExpr::Tag(Tag::SubstAtom(hole,atom),cont)
        }
        Expr::Var(ExprVar { ident, span: _ }) => {
            // variables just fill into the hole
            let atom = Atom::Var(*ident);
            CExpr::Tag(Tag::SubstAtom(hole,atom),cont)
        }
        Expr::Prim(_) => {
            let app = Expr::App(ExprApp {
                func: Box::new(expr.clone()),
                args: Vec::new(),
                span: Span::default(),
            });
            // primitives will be handled in application, recursively
            // see the match branch for application, somewhere below
            return cps_trans(&app, hole, cont);
        }
        Expr::Lam(ExprLam { args, body, span: _ }) => {
            
            // append an additional argument k to the function
            let funcvar = genvar('f');
            let mut argsvar = Vec::new();
            let k = genvar('k');
            argsvar.push(k);
            for arg in args {
                argsvar.push(*arg);
            }

            // eval the body and apply the result to k
            let temp = genvar('t');
            let result = cps_trans(body, temp, 
                Box::new(CExpr::App(Atom::Var(k),vec![Atom::Var(temp)])));

            // eliminate the lambda and transform to let-binding instead
            CExpr::Let(CDecl{
                func: funcvar,
                args: argsvar,
                body: Box::new(result),
            }, Box::new(CExpr::Tag(
                // fill the hole with the function we just defined
                Tag::SubstAtom(hole,Atom::Var(funcvar)),cont)))
        }

        Expr::App(ExprApp { func , args, span: _ }) if func.is_prim() => {
            if let Expr::Prim(ExprPrim { prim, span: _ }) = *func.clone() {
                let arity = prim.get_arity();

                if arity < args.len() {
                    panic!("application of a primitive with too much arguments {arity} {}", args.len());
                }

                if arity > args.len() {
                    // not enough argument, do the eta-expansion!
                    // for example (f x y z) becames (fn u v => (f x y z u v))
                    let mut args = args.clone();
                    let mut newargs = Vec::new();

                    for _ in 0..arity - args.len() {
                        let var = genvar('x');
                        newargs.push(var);
                        args.push(Expr::Var(ExprVar {
                            ident: var,
                            span: Span::default(),
                        }));
                    }

                    let newapp = Expr::Lam(ExprLam {
                        args: newargs,
                        body: Box::new(
                            Expr::App(ExprApp {
                                func: func.clone(),
                                args,
                                span: Span::default(),
                            })
                        ),
                        span: Span::default(),
                    });

                    // then transform again, recursively
                    return cps_trans(&newapp, hole, cont);
                }

                // otherwise, in the case arity == args.len()
                match arity {
                    1 => {
                        // name the argument "x"
                        let x = genvar('x');
                        let result = CExpr::Uniop(prim, Atom::Var(x), hole, cont);
                        
                        // eval the argument to fill the "x" hole
                        let result = cps_trans(&args[0], x, Box::new(result));
                        result
                    }
                    2 => {
                        // name the argument "x"
                        let x1 = genvar('x');
                        let x2 = genvar('x');
                        let result = CExpr::Binop(prim, Atom::Var(x1), Atom::Var(x2), hole, cont);
                        
                        // eval the argument to fill the "x" hole
                        let result = cps_trans(&args[0], x1, Box::new(result));
                        let result = cps_trans(&args[1], x2, Box::new(result));
                        result
                    }
                    _ => {
                        panic!("unsupported arity!");
                    }
                }

            } else {
                unreachable!()
            }
        }
        Expr::App(ExprApp { func, args, span: _ }) => {
            // sometimes people write unecessary parens like (x)
            // in such case we treat it as x
            if args.len() == 0 {
                return cps_trans(func, hole, cont);
            }

            
            
            // transform the return continuation into a function declaration
            let def = {
                let func = genvar('r');
                let args = vec![genvar('x')];
                let body = Box::new(CExpr::Tag(
                    Tag::SubstAtom(hole,Atom::Var(args[0])),cont));
                CDecl { func, args, body }
            };

            // generate a bunch of fresh variable
            let funcvar = genvar('f');
            let mut argsvar = Vec::new();
            argsvar.push(def.func);
            for _ in args {
                argsvar.push(genvar('x'));
            }
            
            // make "f(r,x1,x2,...,xn)"
            let mut result = CExpr::App(
                Atom::Var(funcvar),
                argsvar.iter()
                    .map(|x| Atom::Var(*x)).collect()
            );

            // println!("argslen = {}, {result}", args.len());

            /* 
                eval the function and arguments to fill the correspoding
                hole(fresh variable), update the result each time.
            */
            result = cps_trans(func, funcvar, Box::new(result));
            for (i,arg) in args.iter().enumerate() {
                // println!("eval {i} {arg}");
                // the original i-th argument became (i+1)-th now
                result = cps_trans(arg, argsvar[i+1], Box::new(result));
            }

            CExpr::Let(def,Box::new(result))
        }
        Expr::Let(ExprLet { decls, body, span: _ }) => {
            
            let mut cdecls = Vec::new();
            for decl in decls {
                if let Decl::Val(DeclVal { name, args, body, span: _ }) = decl {
                    // generate k and insert it to the args list
                    let k = genvar('k');
                    let mut args = args.clone();
                    args.insert(0, k);

                    // make a new continuation with t
                    let temp = genvar('t');
                    let result = CExpr::App(Atom::Var(k), vec![Atom::Var(temp)]);

                    // eval body in this new context
                    let body = cps_trans(body, temp, Box::new(result));
                    let body = Box::new(body);

                    let cdecl = CDecl { func: *name, args, body };
                    cdecls.push(cdecl);
                }
            }
            
            let body = cps_trans(body, hole, cont);
            let body = Box::new(body);
            CExpr::Fix(cdecls, body)
        }
        Expr::Case(ExprCase { expr, rules, span: _ }) => {
            todo!()
        }
    }
}


pub fn cps_trans_top(expr: &Expr) -> CExpr {
    let temp = genvar('t');
    let res = cps_trans(expr, temp, Box::new(
        CExpr::Halt(Atom::Var(temp))));
    let mut reduce = super::opt1::Opt1Reduce::new();
    let res = reduce.walk_cexpr(res);
    res
}


#[test]
fn cps_trans_test() {
    use crate::parser::*;
    let string = "
        (fn x => + x 1) 42
    ";
    let mut par = Parser::new(string);

    let res = parse_program(&mut par);
    if let Ok(res) = res {
        println!("\n{res}");
        let cexpr = cps_trans_top(&res);
        println!("\n{}", cexpr);

        let mut reduce = super::opt1::Opt1Reduce::new();
        let cexpr = reduce.walk_cexpr(cexpr);
        println!("\n{}", cexpr);

        let cexpr = super::opt1::opt_level1(cexpr);
        println!("\n{}", cexpr);
    } else {
        par.print_err();
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ByteCode {
    MkPair,
    Head,
    Tail,

    Move(usize, Atom),
    Swap(usize, usize),
    Jump(Atom),
    JumpTrue(Atom),
    JumpFalse(Atom),

    IAdd(Atom, Atom, Atom),
    ISub(Atom, Atom, Atom),
    IMul(Atom, Atom, Atom),
    IDiv(Atom, Atom, Atom),
    INeg(Atom, Atom),
    BNot(Atom, Atom),
    Halt(Atom),
}
