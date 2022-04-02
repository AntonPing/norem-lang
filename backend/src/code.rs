use std::{rc::Rc, collections::HashMap, ops::Deref, cell::RefCell};

use norem_frontend::symbol::{*, SymTable};
use hashbag::HashBag;

#[derive(Clone, Debug, PartialEq)]
pub enum LamExpr {
    Var(Symbol),
    Lam(Symbol,Rc<LamExpr>),
    App(Rc<LamExpr>,Rc<LamExpr>),
    Int(i64),
    Add,
}

impl LamExpr {
    pub fn get_args(&self) -> Vec<Symbol> {
        let mut args = Vec::new();
        let mut with = self;
        while let LamExpr::Lam(x, e) = with {
            args.push(*x);
            with = e;
        }
        args
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CombExpr {
    Var(Symbol),
    Glob(Symbol),
    Comb(Vec<Symbol>,Rc<CombExpr>),
    App(Rc<CombExpr>, Rc<CombExpr>),
    Int(i64),
    Add,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ByteCode {
    App(u8), // arg < 64
    Push(u8), // arg < 64
    Pop(u8), // arg < 64
    Add(u8),
    Call,
    Ret,
}
/*
impl CombExpr {
    pub fn dump_code(&self) -> Vec<ByteCode> {
        let mut code = Vec::new();
        

    }
}
*/


#[derive(Clone, Debug, PartialEq)]
pub struct LamLifter {
    table: Rc<RefCell<SymTable>>,
    bind: HashMap<Symbol,Rc<CombExpr>>,
    env: HashBag<Symbol>, // the symbol walked
    is_top: bool,
}

impl LamLifter {
    pub fn new(table: Rc<RefCell<SymTable>>) -> LamLifter {
        LamLifter {
            table,
            bind: HashMap::new(),
            env: HashBag::new(),
            is_top: true,
        }
    }

    pub fn newvar(&mut self) -> Symbol {
        self.table.borrow_mut().gensym()
    }

    pub fn supercomb(&mut self, args: &Vec<Symbol>, body: CombExpr) -> CombExpr {
        let mut vec = Vec::new();

        for x in self.env.iter() {
            vec.push(*x);
        }

        for x in args {
            vec.push(*x);
        }

        let var = self.newvar();

        self.bind.insert(var,
            Rc::new(CombExpr::Comb(vec, Rc::new(body))));

        let mut res = CombExpr::Var(var);
        for x in self.env.iter() {
            res = CombExpr::App(
                Rc::new(res),
                Rc::new(CombExpr::Var(*x)));
        } 
        res
    }
    
    pub fn lambda_lift(&mut self, lexp: &LamExpr) -> CombExpr {
        match lexp {
            LamExpr::Var(x) => {
                self.env.insert(*x);
                CombExpr::Var(*x)
            }
            LamExpr::Lam(x,e) => {
                let old = self.is_top;
                
                // update the state
                self.is_top = false;
                let body = self.lambda_lift(e);
                self.env.take_all(x);

                self.is_top = old;

                if old {
                    let args = lexp.get_args();
                    self.supercomb(&args, body)
                } else {
                    body
                }
                
                // recover the old state
            }
            LamExpr::App(e1, e2) => {
                let old = self.is_top;
                
                self.is_top = true;
                let f1 = self.lambda_lift(e1);
                let f2 = self.lambda_lift(e2);

                self.is_top = old;

                CombExpr::App(Rc::new(f1), Rc::new(f2))
            }
            LamExpr::Int(n) => {
                CombExpr::Int(*n)
            }
            LamExpr::Add => {
                CombExpr::Add
            }
            _ => {
                unimplemented!()
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

    let expr = 
        Lam(x,Rc::new(App(
            Rc::new(Lam(y,Rc::new(App(
                Rc::new(App(
                    Rc::new(Add),
                    Rc::new(Var(y)))),
                Rc::new(Var(x)))))),
            Rc::new(Var(x)))));

    println!("{:?}", &expr);

    let res = ll.lambda_lift(&expr);
    println!("{:?} in {:#?}", &res, ll);
    


}