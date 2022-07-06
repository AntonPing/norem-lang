use lazy_static::__Deref;

use super::*;
use super::visitor::*;
use crate::ast::*;
use crate::symbol::*;
use crate::utils::Span;

pub fn cps_trans(
    expr0: &Expr,
    bind0: Symbol,
    cont0: Box<Core>
) -> Core {
    /*
    if cfg!(test) {
        println!("In context fn {} => {}", hole, &cont);
        println!("Eval expresssion {}", expr);
    }
    */

    match expr0 {
        Expr::Lit(ExprLit { lit, span: _ }) => {
            // literals just fill into the hole
            let atom = match lit {
                LitVal::Int(x) => Atom::Int(*x),
                LitVal::Real(x) => Atom::Real(*x),
                LitVal::Bool(x) => Atom::Bool(*x),
                LitVal::Char(x) => Atom::Char(*x),
            };
            Core::Tag(Tag::SubstAtom(bind0,atom),cont0)
        }
        Expr::Var(ExprVar { ident, span: _ }) => {
            // variables just fill into the hole
            let atom = Atom::Var(*ident);
            Core::Tag(Tag::SubstAtom(bind0,atom),cont0)
        }
        Expr::Prim(_) => {
            let app = Expr::App(ExprApp {
                func: Box::new(expr0.clone()),
                args: Vec::new(),
                span: Span::default(),
            });
            /*
                primitives will be handled in application recursively. See the match branch for application, it's somewhere below.
            */
            cps_trans(&app, bind0, cont0)
        }
        Expr::Lam(ExprLam { args, body, span: _ }) => {
            // println!("here lam {}", &expr);

            /*
                trans(fn x y ... z => body, h, c)

                =====> becomes =====>

                let f k x y ... z = 
                    trans(body,t,(k t)) ; in
                subst(h,f,c)
            */

            let f = genvar('f');
            let k = genvar('k');
            let t = genvar('t');

            // eliminate the lambda and transform to let-binding instead
            // and fill the hole with the function we just defined
            Core::Let(CoreLet {
                decl: CoreDecl{
                    func: f,
                    args: [k].iter()
                        .chain(args.iter())
                        .copied().collect(),
                    body: Box::new(cps_trans(body, t, Box::new(
                        Core::App(CoreApp{
                            func: Atom::Var(k),
                            args: vec![Atom::Var(t)],
                        })
                    ))),
                },
                cont: Box::new(Core::Tag(
                    Tag::SubstAtom(bind0,Atom::Var(f)),cont0)),
            })
        }

        Expr::App(ExprApp { func , args, span: _ }) if func.is_prim() => {
            // println!("here app prim {}", &expr);

            if let Expr::Prim(ExprPrim { prim, span: _ }) = *func.clone() {
                let arity = prim.get_arity();
                let len = args.len();

                if arity < len {
                    panic!("
                        application with too much arguments!
                        primitive {prim} excepted {arity} argument, but provided {len} in expression {}
                    ",
                        "<todo>",
                    );
                }

                if arity > len {
                    // not enough argument, do the eta-expansion!
                    // for example (f x y z) becames (fn u v => (f x y z u v))

                    let newargs: Vec<Symbol> = (0..arity - len)
                        .map(|_| genvar('x'))
                        .collect();

                    let extargs: Vec<Expr> = args.iter().cloned()
                        .chain(newargs.iter()
                        .map(|sym| Expr::Var(ExprVar {
                            ident: *sym, 
                            span: Span::default()
                        })))
                        .collect();

                    let newapp = Expr::Lam(ExprLam {
                        args: newargs,
                        body: Box::new(Expr::App(ExprApp {
                            func: func.clone(),
                            args: extargs,
                            span: Span::default(),
                        })),
                        span: Span::default(),
                    });

                    // then transform again, recursively
                    return cps_trans(&newapp, bind0, cont0);
                }

                // otherwise, in the case arity == args.len()
                /*
                    trans(op x y .. z, h, c)

                    =====> becomes =====>

                    trans(x,t1,
                        trans(y,t2
                            ...
                                trans(z,tn,
                                    op t1 t2 ... tn -> h;
                                    c
                                )...)
                */
                
                let argsvar: Vec<Symbol> = args.iter()
                    .map(|_| genvar('t'))
                    .collect();
                
                args.iter()
                    .zip(argsvar.iter())
                    .fold(Core::Opr(CoreOpr {
                            prim,
                            args: argsvar.iter()
                                .map(|sym| Atom::Var(*sym))
                                .collect(),
                            bind: bind0,
                            cont: cont0,
                        }), 
                        |acc, (expr, bind)| {
                            cps_trans(expr, *bind, Box::new(acc))
                        }
                    )
            } else {
                unreachable!()
            }
        }
        Expr::App(ExprApp { func, args, span: _ }) => {
            // println!("here app {}", &expr);

            // sometimes people write unecessary parens like (x)
            // in such case we treat it as x
            if args.len() == 0 {
                return cps_trans(func, bind0, cont0);
            }
            
            /*
                trans(f x y .. z, h, c)

                =====> becomes =====>

                trans(f,f0,
                    trans(x,t1,
                        trans(y,t2
                            ...
                                trans(z,tn,
                                    let r(h) = c in
                                    f0(r,t1,t2,... tn))...)

                where f0, r and t1...tn are generated variables
            */
           
            let f = genvar('f');
            let r = genvar('r');
            let ts: Vec<Symbol> = args.iter()
                .map(|_| genvar('t'))
                .collect();
            
            [(func.deref(), &f)].into_iter()
                .chain(args.iter().zip(ts.iter()))
                .fold(Core::Let(CoreLet {
                        decl: CoreDecl {
                            func: r,
                            args: vec![bind0],
                            body: cont0,
                        },
                        cont: Box::new(Core::App(CoreApp {
                            func: Atom::Var(f),
                            args: [r].iter()
                                .chain(ts.iter())
                                .map(|sym| Atom::Var(*sym))
                                .collect(),
                        }))
                    }),
                    |cont, (expr, bind)| {
                        cps_trans(expr, *bind, 
                            Box::new(cont))
                    }
                )
            
        }
        Expr::Let(ExprLet { decls, body, span: _ }) => {
            /*
                trans(
                    let
                        val f1 x1 y1 ... z1 = body1
                        ...
                        val fn xn yn ... zn = bodyn
                    in
                        body'
                    end,
                    h,
                    c,
                )

                =====> becomes =====>

                let
                    f1(k1,x1,y1,...,z1) = trans(body1,t1,(k1 t1))
                    ...
                    fn(kn,xn,yn,...,zn) = trans(bodyn,tn,(kn tn))
                in
                    trans(body',h,c)
                end
            */

            Core::Fix(CoreFix {
                decls: decls.iter()
                    .filter_map(|decl| {
                        if let Decl::Val(decl) = decl {
                            Some(decl)
                        } else {
                            None
                        }
                    })
                    .map(|decl| {
                        let k = genvar('k');
                        let t = genvar('t');
                        CoreDecl {
                            func: decl.name,
                            args: [k].iter()
                                .chain(decl.args.iter())
                                .copied().collect(),
                            body: Box::new(cps_trans(&decl.body, t, 
                                Box::new(Core::App(CoreApp {
                                    func: Atom::Var(k),
                                    args: vec![Atom::Var(t)]
                                }))))
                        }
                    })
                    .collect(),
                cont: Box::new(cps_trans(body, bind0, cont0))
            })
        }
        Expr::Case(ExprCase { expr, rules, span: _ }) => {
            todo!()
        }
        _ => {
            todo!()
        }
    }
}


pub fn cps_trans_top(expr: &Expr) -> Core {
    let temp = genvar('t');
    let res = cps_trans(expr, temp, Box::new(
        Core::Halt(Atom::Var(temp))));
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
        let expr = cps_trans_top(&res);
        println!("\n{}", expr);

        let mut reduce = super::opt1::Opt1Reduce::new();
        let expr = reduce.walk_cexpr(expr);
        println!("\n{}", expr);

        let expr = super::opt1::opt_level1(expr);
        println!("\n{}", expr);
    } else {
        par.print_err();
    }
}

