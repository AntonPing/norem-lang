use logos::Lexer;

use crate::symbol::{Symbol, newvar, genvar};
use crate::core::*;
use crate::visitor::subst_expr;

pub struct CpsTrans {
    // return stack
    stack: Vec<(Symbol,CExpr)>,
}

//pub fn subst(expr:CExpr, x: Symbol, y:Atom)


pub fn cps_trans(expr: &LExpr, hole: Symbol, cont: Box<CExpr>) -> CExpr {
    match expr {
        LExpr::Val(x) => {
            // atoms just fill into the hole
            CExpr::Tag(Tag::SubstAtom(hole,*x),cont)
        }
        LExpr::Lam(args, body) => {

            let k = genvar('k');
            
            // append an additional argument k to the function
            let funcvar = genvar('f');
            let mut argsvar = Vec::new();
            argsvar.push(k);
            for arg in args {
                argsvar.push(*arg);
            }

            // eval the body and apply the result to k
            let temp = genvar('~');
            let result = cps_trans(body, temp, 
                Box::new(CExpr::App(Atom::Var(k),vec![Atom::Var(temp)])));

            // eliminate the lambda and transform to let-binding instead
            CExpr::Let(Def{
                func: funcvar,
                args: argsvar,
                body: Box::new(result),
            }, Box::new(CExpr::Tag(
                // fill the hole with the function we just defined
                Tag::SubstAtom(hole,Atom::Var(funcvar)),cont)))

        }
        LExpr::App(func, args) => {                
            // make a return continuation function
            let def = {
                let func = genvar('r');
                let args = vec![genvar('x')];
                let body = Box::new(CExpr::Tag(
                    Tag::SubstAtom(hole,Atom::Var(args[0])),cont));
                Def {func, args, body}
            };

            // generate a bunch of fresh variable
            let funcvar = genvar('f');
            let mut argsvar = Vec::new();
            for _ in args {
                argsvar.push(genvar('x'));
            }
            argsvar.push(def.func);

            // make "f(x1,x2,...,xn,r)"
            let mut result = CExpr::App(
                Atom::Var(funcvar),
                argsvar.iter()
                    .map(|x| Atom::Var(*x)).collect()
            );

            /* 
                eval the function and arguments to fill the correspoding
                hole(fresh variable), update the result each time.
            */
            result = cps_trans(func, funcvar, Box::new(result));
            for (i,arg) in args.iter().enumerate() {
                result = cps_trans(arg, argsvar[i], Box::new(result));
            }

            CExpr::Let(def,Box::new(result))
        }
        LExpr::Uniop(prim, arg) => {
            // generate new variable "r" and fill the hole
            let ret = genvar('r');
            let cont = Box::new(CExpr::Tag(
                Tag::SubstAtom(hole, Atom::Var(ret)), cont));
            
            // name the argument "x"
            let x = genvar('x');
            let result = CExpr::Uniop(*prim, Atom::Var(x), ret, cont);
            
            // eval the argument to fill the "x" hole
            let result = cps_trans(arg, x, Box::new(result));
            result
        }
        LExpr::Binop(prim, arg1, arg2) => {
            // generate new variable "r" and fill the hole
            let ret = genvar('r');
            let cont = Box::new(CExpr::Tag(
                Tag::SubstAtom(hole, Atom::Var(ret)), cont));
            
            // name the argument "x"
            let x1 = genvar('x');
            let x2 = genvar('x');
            let result = CExpr::Binop(*prim, Atom::Var(x1), Atom::Var(x2), ret, cont);
            
            // eval the argument to fill the "x" hole
            let result = cps_trans(arg1, x1, Box::new(result));
            let result = cps_trans(arg2, x2, Box::new(result));
            result
        }
        LExpr::Switch(_, _) => todo!(),
        LExpr::Ifte(_, _, _) => todo!(),
        LExpr::Record(_) => todo!(),
        LExpr::Select(_, _) => todo!(),
    }

}


pub fn cps_trans_top(expr: &LExpr) -> CExpr {
    let temp = genvar('~');
    cps_trans(expr, temp, Box::new(
        CExpr::Halt(Atom::Var(temp))))
}

#[test]
fn cps_trans_test() {
    let f = newvar("f");
    let g = newvar("g");
    let x = newvar("x");
    let y = newvar("y");
    let z = newvar("z");

    use Atom::*;
    use LExpr::*;
    use Prim::*;
    let expr =
        App(Box::new(
            Lam(vec![x,y],Box::new(
                Binop(IAdd, 
                    Box::new(Val(Var(x))),
                    Box::new(Val(Var(y))),
                )
            ))),
            vec![Val(Int(41)),Val(Int(42))]
        );
    
    let expr = cps_trans_top(&expr);
    println!("{:#?}", expr);
    let expr = subst_expr(expr);
    println!("\n\n{:#?}", expr);


}