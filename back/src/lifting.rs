use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};


use norem_frontend::symbol::*;

use crate::code::*;

#[derive(Clone, Debug, PartialEq)]
pub struct LamLifter {
    table: Rc<RefCell<SymTable>>,
    bind: HashMap<Symbol,SuperComb>,
    env: HashSet<Symbol>, // the symbol walked
    typ: HashMap<Symbol,Type>,
}

impl LamLifter {
    pub fn new(table: Rc<RefCell<SymTable>>) -> LamLifter {
        LamLifter {
            table,
            bind: HashMap::new(),
            env: HashSet::new(),
            typ: HashMap::new(),
        }
    }

    pub fn newvar(&mut self) -> Symbol {
        self.table.borrow_mut().gensym()
    }

    pub fn supercomb(&mut self, args: &Vec<(Symbol,Type)>, body: CombExpr) -> CombExpr {
        
        let name = self.newvar();

        let args = self.env.iter()
            .map(|x| (*x, *self.typ.get(x).unwrap()))
            .chain(args.iter()
                .map(|x| *x))
            .collect();

        let body = Rc::new(body);

        let sprc = SuperComb { name, args, body };

        self.bind.insert(name,sprc);

        let res = self.env.iter()
            .map(|x| Rc::new(CombExpr::Arg(*x)))
            .collect();

        CombExpr::App(Rc::new(CombExpr::Glob(name)), res)
    }
    
    pub fn lambda_lift(&mut self, lexp: &LamExpr) -> CombExpr {
        match lexp {
            LamExpr::Lit(lit) => {
                CombExpr::Lit(*lit)
            }
            LamExpr::Var(x) => {
                self.env.insert(*x);
                CombExpr::Arg(*x)
            }
            LamExpr::Lam(args, body) => {

                let body = self.lambda_lift(body);
                for (arg, _typ) in args {
                    self.env.remove(arg);
                }
                
                self.supercomb(args, body)
                
            }
            LamExpr::App(func, args) => {
                let func2 = self.lambda_lift(func);
                let args2 = args
                    .iter()
                    .map(|arg| Rc::new(self.lambda_lift(arg)))
                    .collect();
            
                CombExpr::App(Rc::new(func2), args2)
            }
            
            LamExpr::Record(fields) => {
                let fields2 = fields
                    .iter()
                    .map(|field| self.lambda_lift(field))
                    .collect();
                CombExpr::Record(fields2)
            }
            LamExpr::Select(n, record) => {
                let record2 = self.lambda_lift(record);
                CombExpr::Select(*n, Rc::new(record2))
            }
            LamExpr::Prim(op) => {
                CombExpr::Prim(*op)
            }

        }
    }

    pub fn dump_code(&self) -> HashMap<Symbol,Vec<ByteCode>> {
        unimplemented!()
    }
}



#[test]
pub fn lambda_lift_test() {
    use LamExpr::*;

    let mut table = SymTable::new();
    let x = table.gensym();
    let y = table.gensym();
    let z = table.gensym();
    let a = table.gensym();

    let table = Rc::new(RefCell::new(table));
    let mut ll = LamLifter::new(table.clone());
    /* 
    let expr = 
        Lam(x,Rc::new(App(
            Rc::new(Lam(y,Rc::new(App(
                Rc::new(App(
                    Rc::new(Add),
                    Rc::new(Var(y)))),
                Rc::new(Var(x)))))),
            Rc::new(Var(x)))));
    */

    //println!("{:?}", &expr);

    //let res = ll.lambda_lift(&expr);
    //println!("{:?} in {:#?}", &res, ll);
    
}
