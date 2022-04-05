use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::vec;

use hashbag::HashBag;

use norem_frontend::ast::ValDecl;
use norem_frontend::symbol::*;

use crate::code::*;

#[derive(Copy,Clone, Debug, PartialEq)]
pub enum Prim {
    IAdd,
    ISub,
    IMul,
    IDiv,
    INeg,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LamExpr {
    Lit(Value),
    Var(Symbol),
    Lam(Symbol,Rc<LamExpr>),
    App(Rc<LamExpr>,Rc<LamExpr>),
    Record(Vec<LamExpr>),
    Select(usize,Rc<LamExpr>),
    Prim(Prim),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CpsExpr {
    App(Value,Vec<Value>),
    Let(Vec<(Symbol,Vec<Symbol>,Box<CpsExpr>)>,Box<CpsExpr>),
    Record(Vec<Value>,Symbol,Box<CpsExpr>),
    Select(Value,usize,Symbol,Box<CpsExpr>),
    Prim1(Prim,Value,Symbol,Box<CpsExpr>),
    Prim2(Prim,Value,Value,Symbol,Box<CpsExpr>),
    //Ifte(Value,Rc<CpsExpr>,Rc<CpsExpr>)
}

#[derive(Copy,Clone, Debug, PartialEq)]
pub enum Value {
    Var(Symbol),
    Label(Symbol),
    Int(i64),
    Real(f64),
    Bool(bool),
    MetaVar(Symbol),
    //Func(Symbol,Rc<CpsExpr>),
}

impl Value {
    pub fn subst(&mut self, x: Symbol, v: Value) {
        if let Value::MetaVar(y) = self {
            if x == *y {
                *self = v;
            }
        }
    }
}

impl CpsExpr {

    pub fn subst(&mut self, hole: Symbol, val: Value) -> &mut CpsExpr {

        match self {
            CpsExpr::App(func, args) => {
                func.subst(hole, val);
                for arg in args {
                    arg.subst(hole, val);
                }
            }
            //CpsExpr::Ifte(, (), ())
            CpsExpr::Let(decls, next) => {
                for (_name, _args, def) in decls {
                    def.subst(hole, val);
                }
                next.subst(hole, val);
            }
            CpsExpr::Prim1( op , x, bind, next) => {
                x.subst(hole, val);
                next.subst(hole, val);
            }
            CpsExpr::Prim2( op, x, y, bind, next) => {
                x.subst(hole, val);
                y.subst(hole, val);
                next.subst(hole, val);
            }
            CpsExpr::Record( fields , bind, next) => {
                for field in fields {
                    field.subst(hole, val);
                }
                next.subst(hole, val);
            }
            CpsExpr::Select(record, idx, bind, next) => {
                record.subst(hole, val);
                next.subst(hole, val);
            }
        }

        self
    }
}

pub struct CpsTrans {
    table: Rc<RefCell<SymTable>>,
    cont: CpsExpr,
    // unbounded metavars
    free: Vec<Symbol>,
    subst: HashMap<Symbol,Value>,
}



impl CpsTrans {

    pub fn newvar(&mut self) -> Symbol {
        self.table.borrow_mut().gensym()
    }

    pub fn fill(&mut self, v: Value) {
        let x = self.free.pop().unwrap();
        self.cont.subst(x,v);
    }

    pub fn cps_trans(&mut self, expr: &LamExpr, hole: Symbol, mut cont: CpsExpr) -> CpsExpr {
        match expr {
            LamExpr::Lit(val) => {
                cont.subst(hole, *val);
                cont
            }
            LamExpr::Var(x) => {
                cont.subst(hole, Value::Var(*x));
                cont
            }
            LamExpr::Lam(arg, body) => {
                let f = self.newvar();
                let k = self.newvar();
                let y = self.newvar();


                cont.subst(hole, Value::Var(f));

                let new_c = CpsExpr::App(
                    Value::Var(k), vec![Value::MetaVar(y)]);
                
                CpsExpr::Let(
                    vec![(
                        f, vec![*arg,k],
                        Box::new(self.cps_trans(body, y, new_c))
                    )],
                    Box::new(cont)
                )
            }

            LamExpr::App(e1, e2) => {
                let f1 = self.newvar();
                let f2 = self.newvar();
                let x = self.newvar();
                let r = self.newvar();

                let body = CpsExpr::App(
                    Value::MetaVar(f1), vec![Value::MetaVar(f2)]);
                
                let body1 = self.cps_trans(e2, f2, body);
                let body2 = self.cps_trans(e1, f1, body1);
                
                cont.subst(hole, Value::Var(x));
                
                CpsExpr::Let(
                    vec![(
                        r, vec![x],
                        Box::new(cont)
                    )],
                    Box::new(body2)
                )
            }
            LamExpr::Prim(op) => {
                use Prim::*;
                match op {
                    IAdd | ISub | IMul | IDiv => {
                        let x = self.newvar();
                        let y = self.newvar();
                        let body = LamExpr::Lam(x, LamExpr::Lam(y,
                            LamExpr::App(LamExpr::App(, ()), ())))

                            }
                    _ => {

                    }




                }


                

            }
            _ => {
                unimplemented!()
            }
        }
    }
}