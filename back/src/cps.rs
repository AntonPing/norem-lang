use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::vec;

//use norem_frontend::ast::ValDecl;
//use norem_frontend::symbol::*;

type Symbol = String;

#[derive(Copy,Clone, Debug, PartialEq)]
pub enum Prim {
    IAdd,
    ISub,
    IMul,
    IDiv,
    INeg,
    BNot,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Label(Symbol),
    Int(i64),
    Real(f64),
    Bool(bool),
    Char(char),
}


#[derive(Clone, Debug, PartialEq)]
pub enum LExpr {
    Lit(Literal),
    Prim(Prim),
    Var(Symbol),
    Lam(Vec<Symbol>,Box<LExpr>),
    App(Box<LExpr>,Vec<LExpr>),
    //Record(Vec<LExpr>),
    //Select(usize,Box<LExpr>),
}


#[derive(Clone, Debug, PartialEq)]
pub enum CVal {
    // halt
    Halt,
    // lit c
    Lit(Literal),
    // var x
    Var(Symbol),
    // cont x p
    Lam(Symbol,Box<CExpr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum CExpr {
    // (lam (x k) p) x' k'
    LApp(CVal,CVal,CVal),
    // (cont x p) x'
    CApp(CVal,CVal),
    // let x = (? y) in expr
    Prim1(Prim,CVal,Symbol,Box<CExpr>),
    // let x = (y ? z) in expr
    Prim2(Prim,CVal,CVal,Symbol,Box<CExpr>),
    // let k = k' in expr
    // LetC(Symbol,CVal,Box<CExpr>),

    // these should not appear in the optimized cps
    // only used as a helping marker structure
    // e[x:=x']
    Subst(Symbol,CVal,Box<CExpr>),
}

pub fn k() -> Symbol {
    "k".to_string()
}

impl CVal {
    pub fn is_atom(&self) -> bool {
        match self {
            CVal::Halt => { true }
            CVal::Lit(_) => { true }
            CVal::Var(_) => { true }
            CVal::Lam(_, _) => { false }
        }
    }
}


use CExpr::*;
use hashbag::HashBag;

pub struct CpsTrans {
    gensym: usize,
    vars: HashBag<Symbol>,
    subst: HashMap<Symbol,CVal>,
}
impl CpsTrans {
    pub fn new() -> CpsTrans {
        CpsTrans {
            gensym: 0,
            vars: HashBag::new(),
            subst: HashMap::new(),
        }
    }

    pub fn newvar(&mut self) -> Symbol {
        let sym = self.gensym;
        let mut res = sym.to_string();
        res.insert(0, '#');
        self.gensym += 1;
        res
    }

    pub fn cps_trans(&mut self, expr: LExpr, cont: CVal) -> CExpr {
        match expr {
            LExpr::Lit(lit) => {
                CExpr::CApp(cont, CVal::Lit(lit))
            }
            LExpr::Prim(op) => {
                unimplemented!()
                //CExpr::CApp(cont, LVal::LVar(x))
            }
            LExpr::Var(x) => {
                CExpr::CApp(cont, CVal::Var(x))
            }
            LExpr::Lam(args, body) => {
                let k = "k".to_string();
                let kvar = CVal::Var(k.clone());
                let mut temp = CVal::Lam(k,Box::new(
                    self.cps_trans(*body, kvar)));
                
                for arg in args {
                    temp = CVal


                }

                CExpr::CApp(cont, CVal::LLam(arg, k(), Box::new(
                    self.cps_trans(*body, CVal::CVar(k()))
                )))
            }
            LExpr::App(func, arg) => {
                let vf = self.newvar();
                let va = self.newvar();
                let inside = self.cps_trans(*arg, CVal::CLam(va.clone(),Box::new(
                    CExpr::LApp(LVal::LVar(vf.clone()),LVal::LVar(va), cont)
                )));
                self.cps_trans(*func, CVal::CLam(vf,Box::new(inside)))
            }
            
            _ => {
                unimplemented!()
            }
        }
    }

    pub fn reduce_expr(&mut self, expr: CExpr) -> CExpr {
        match expr {
            // Lambda Application
            LApp(LLam(x1, k1, body), x2 , k2) => {

                println!("(lapp (fn {:?} {:?} -> {:?}) {:?} {:?})",x1,k1,body,x2,k2);

                let body = self.reduce_expr(*body);
                let x2 = self.reduce_lval(x2);
                let k2 = self.reduce_cval(k2);

                let xref = self.lam_var.take_all(&x1).map(|(_,n)| n).unwrap_or(0);
                let kref = self.cps_var.take_all(&k1).map(|(_,n)| n).unwrap_or(0);

                if xref == 0 && kref == 0 {
                    // (\_.e x) => e
                    body
                } else if (xref == 1 || x2.is_atom()) && (kref == 1 || k2.is_atom()) {
                    // (\x.e y) => e[x:=y], where ref(x,e) = 1
                    // (\x.e c) => e[x:=c], where c is atom
                    LSubst(x1, x2, k1, k2, Box::new(body))
                } else {
                    // otherwise do nothing
                    LApp(LLam(x1, k1, Box::new(body)), x2 , k2)
                }
            }

            // Continuation Application
            CApp(CLam(x1, body), x2) => {
                let body = self.reduce_expr(*body);
                let x2 = self.reduce_lval(x2);
                let xref = self.lam_var.take_all(&x1).map(|(_,n)| n).unwrap_or(0);
                
                // (\_.e x) => e
                if xref == 0 {
                    return body;
                } else if xref == 1 || x2.is_atom() {
                    // (\x.e y) => e[x:=y], where ref(x,e) = 1
                    // (\x.e c) => e[x:=c], where c is atom
                    CSubst(x1, x2, Box::new(body))
                } else {
                    // otherwise do nothing
                    CApp(CLam(x1, Box::new(body)), x2)
                }
                
            }
            LApp(f, x, k) => {
                let f = self.reduce_lval(f);
                let x = self.reduce_lval(x);
                let k = self.reduce_cval(k);
                LApp(f, x, k)
            }
            CApp(f, x) => {
                let f = self.reduce_cval(f);
                let x = self.reduce_lval(x);
                CApp(f, x)
            }
            _ => {
                unimplemented!()
            }
        }
    }
    
    pub fn reduce_lval(&mut self, val: LVal) -> LVal {
        match val {
            LLit(_) => { val }
            LVar(ref x) => {
                self.lam_var.insert(x.clone());
                val
            }
            LLam(x1, k1, box LApp(f,LVar(x2), CVar(k2)))
                if x1 == x2 && k1 == k2 => {
                // eta reduction on \x k. f x k => f
                self.reduce_lval(f)
            }
            LLam(x, k, body) => {
                let body2 = self.reduce_expr(*body);
                self.lam_var.take_all(&x);
                self.cps_var.take_all(&k);
                LLam(x, k, Box::new(body2))
            }
        }
    }

    pub fn reduce_cval(&mut self, val: CVal) -> CVal {    
        match val {
            Halt => { val }
            CVar(ref x) => {
                self.cps_var.insert(x.clone());
                val
            }
            CLam(x, box CApp(f,LVar(y)))
                if x == y => {
                // eta reduction on \x. f x => f
                self.reduce_cval(f) 
            }
            CLam(x, body) => {
                let body2 = self.reduce_expr(*body);
                self.lam_var.take_all(&x);
                CLam(x, Box::new(body2))
            }
        }
    }

    pub fn subst_expr(&mut self, expr: CExpr) -> CExpr {
        match expr {
            LApp(e, x, k) => {
                let e = self.subst_lval(e);
                let x = self.subst_lval(x);
                let k = self.subst_cval(k);
                LApp(e, x, k)
            }
            CApp(e, x) => {
                let e = self.subst_cval(e);
                let x = self.subst_lval(x);
                CApp(e, x)
            }
            LSubst(x1, x2, k1, k2, body) => {
                let x3 = self.lam_subst.insert(x1.clone(), x2);
                let k3 = self.cps_subst.insert(k1.clone(), k2);

                let body = self.subst_expr(*body);

                if let Some(x3) = x3 {
                    self.lam_subst.insert(x1, x3);
                }
                if let Some(k3) = k3 {
                    self.cps_subst.insert(k1, k3);
                }

                body
            }
            CSubst(x1, x2, body) => {
                let x3 = self.lam_subst.insert(x1.clone(), x2);

                let body = self.subst_expr(*body);

                if let Some(x3) = x3 {
                    self.lam_subst.insert(x1, x3);
                }

                body
            }
            _ => {
                unimplemented!()
            }
        }
    }

    pub fn subst_lval(&mut self, val: LVal) -> LVal {
        match val {
            LLit(_) => { val }
            LVar(ref x) => {
                if let Some(y) = self.lam_subst.get(x) {
                    y.clone()
                } else {
                    val
                }
            }
            LLam(x, k, body) => {
                self.lam_subst.remove(&x);
                self.cps_subst.remove(&k);
                let body = self.subst_expr(*body);
                LLam(x, k, Box::new(body))
            }
        }
    }

    pub fn subst_cval(&mut self, val: CVal) -> CVal {
        match val {
            Halt => { val }
            CVar(ref x) => {
                if let Some(y) = self.cps_subst.get(x) {
                    y.clone()
                } else {
                    val
                }
            }
            CLam(x, body) => {
                self.lam_subst.remove(&x);
                let body = self.subst_expr(*body);
                CLam(x, Box::new(body))
            }
        }
    }
}

#[test]
pub fn cps_trans_test() {
    use LExpr::*;
    let x = "x".to_string();
    let y = "y".to_string();
    
    let expr = App(Box::new(Lam(x.clone(),Box::new(Var(x)))),
        Box::new(Lam(y.clone(),Box::new(Var(y)))));

    let mut cps = CpsTrans::new();

    let expr = cps.cps_trans(expr, CVal::Halt);
    println!("{:?}", &expr);

    let expr = cps.reduce_expr(expr);
    println!("1 {:?}", &expr);

    let expr = cps.subst_expr(expr);
    println!("1 {:?}", &expr);

    let expr = cps.reduce_expr(expr);
    println!("2 {:?}", &expr);

    let expr = cps.subst_expr(expr);
    println!("2 {:?}", &expr);

}




/*

impl CExpr {
    pub fn subst(&mut self, hole: Symbol, val: Value) -> &mut CExpr {
        match self {
            CExpr::App(func, args) => {
                func.subst(hole, val);
                for arg in args {
                    arg.subst(hole, val);
                }
            }
            //CpsExpr::Ifte(, (), ())
            CExpr::Prim1( op , x, bind, next) => {
                x.subst(hole, val);
                next.subst(hole, val);
            }
            CExpr::Prim2( op, x, y, bind, next) => {
                x.subst(hole, val);
                y.subst(hole, val);
                next.subst(hole, val);
            }
            CExpr::Record( fields , bind, next) => {
                for field in fields {
                    field.subst(hole, val);
                }
                next.subst(hole, val);
            }
            CExpr::Select(record, idx, bind, next) => {
                record.subst(hole, val);
                next.subst(hole, val);
            }
        }

        self
    }
}

*/