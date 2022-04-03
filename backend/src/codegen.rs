use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use hashbag::HashBag;

use norem_frontend::symbol::*;

use crate::code::*;



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

pub struct CodeGen {
    table: Rc<RefCell<SymTable>>,
    env: HashMap<Symbol,usize>,
    code: Vec<ByteCode>,
}

impl CodeGen {
    pub fn new(table: Rc<RefCell<SymTable>>) -> Self {
        CodeGen {
            table,
            env: HashMap::new(),
            code: Vec::new(),
        }
    }

    pub fn address(&self) -> usize {
        self.code.len() - 1
    }

    pub fn codegen(&mut self, args: &Vec<Symbol>, body: &CombExpr) {
        let adr = self.address();
        let arg_len = args.len();

        let mut arg_map = HashMap::new();
        for (i, arg) in args.iter().enumerate() {
            self.code.push(ByteCode::Pop(i));
            arg_map.insert(arg, i);
        }

        //self.code.push(ByteCode::PopArgs(arg_len));

        let mut stack = Vec::new();
        stack.push(body);

        while let Some(with) = stack.pop() {
            match with {
                CombExpr::Var(x) => {
                    let reg = arg_map.get(&x).unwrap();
                    self.code.push(ByteCode::Push(*reg))
                }
                CombExpr::Glob(x) => {
                    self.code.push(ByteCode::GlobCall(*x));
                }
                CombExpr::Int(n) => {
                    self.code.push(ByteCode::PushInt(*n));
                }
                CombExpr::Comb(_,_) => {
                    panic!("Combinators should be lifted to the toplevel!");
                }
                CombExpr::App(e1,e2) => {
                    stack.push(e1.deref());
                    stack.push(e2.deref());
                }
                CombExpr::Add => {
                    self.code.push(ByteCode::IntAdd);
                }
            }
        }
    }
}