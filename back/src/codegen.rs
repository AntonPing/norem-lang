use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use norem_frontend::symbol::*;

use crate::code::*;

pub struct CodeGen {
    bind: HashMap<Symbol,SuperComb>,
    table: Rc<RefCell<SymTable>>,
    env: HashMap<Symbol,(usize,Type)>,
    //hole: Vec<(usize,Symbol)>,
    code: Vec<ByteCode>,
    offset: usize,
}

impl CodeGen {
    pub fn new(bind: HashMap<Symbol,SuperComb>, table: Rc<RefCell<SymTable>>) -> Self {
        CodeGen {
            bind,
            table,
            env: HashMap::new(),
            code: Vec::new(),
            //hole: Vec::new(),
            offset: 0,
        }
    }

    pub fn address(&self) -> usize {
        self.code.len() - 1
    }

    /*
    pub fn mark_hole(&mut self, label: Symbol) {
        let adr = self.code.len() - 1;
        self.hole.push((adr,label));

    }
    */

    pub fn codegen(&mut self, body: &CombExpr, map: &HashMap<Symbol,(usize,Type)>) {
        match body.deref() {
            CombExpr::Arg(x) => {
                let (idx, _) = map.get(&x).unwrap();
                self.code.push(ByteCode::Push(*idx + self.offset));
                self.offset += 1;
            }
            CombExpr::Glob(x) => {
                self.code.push(ByteCode::Call(0));
                //self.mark_hole(*x);
                self.offset += 1;
            }
            CombExpr::App(func, args) => {
                for arg in args {
                    self.codegen(arg, map);
                }

                match func.deref() {
                    CombExpr::Arg(f) => {
                        let (idx, typ) = map.get(f).unwrap();

                        if args.len() < typ.arity() {
                            self.code.push(ByteCode::Push(*idx + self.offset));
                            self.offset += 1;

                            for _ in 0..args.len() {
                                self.code.push(ByteCode::MkPair);
                                self.offset -= 1;
                            }

                            self.offset -= args.len();
                            self.offset += 1;
                        } else {
                            self.code.push(ByteCode::CallArg(*idx));
                            self.offset -= typ.arity();
                            self.offset += 1;
                        }
                    }

                    CombExpr::Glob(f) => {
                        let sprc = self.bind.get(f).unwrap();
                        if args.len() < sprc.arity() {
                            self.code.push(ByteCode::PushPtrHole(*f));
                            
                            for _ in 0..args.len() {
                                self.code.push(ByteCode::MkPair);
                            }

                            self.offset -= args.len();
                            self.offset += 1;
                        } else {
                            self.code.push(ByteCode::CallHole(*f));
                            self.offset -= sprc.arity();
                            self.offset += 1;
                        }
                    }
                    _ => {
                        panic!("unexcepted redex!");
                    }                
                }
            }
            CombExpr::Record(fields) => {
                assert!(fields.len() >= 1);

                let mut flag = false; 

                for field in fields.iter().rev() {
                    if flag {
                        self.code.push(ByteCode::MkPair);
                        self.offset -= 1;
                    } else {
                        flag = true;
                    }
                    self.codegen(field,map);
                }
            }
            CombExpr::Select(i, rec) => {
                self.codegen(rec, map);
                for j in 0..*i - 1 {
                    self.code.push(ByteCode::Tail);
                }
                self.code.push(ByteCode::Head);
            }
            CombExpr::Lit(lit) => {
                match lit {
                    Value::Int(x) => {
                        self.code.push(ByteCode::PushInt(*x));
                    }
                    Value::Real(x) => {
                        self.code.push(ByteCode::PushReal(*x));
                    }
                    Value::Bool(x) => {
                        self.code.push(ByteCode::PushBool(*x));
                    }
                }
                self.offset += 1;
            }
            
            CombExpr::Prim(op) => {
                match op {
                    Prim::IAdd => {
                        self.code.push(ByteCode::IAdd);
                        self.offset -= 1;
                    }
                    Prim::ISub => {
                        self.code.push(ByteCode::ISub);
                        self.offset -= 1;
                    }
                    Prim::IMul => {
                        self.code.push(ByteCode::IMul);
                        self.offset -= 1;
                    }
                    Prim::IDiv => {
                        self.code.push(ByteCode::IDiv);
                        self.offset -= 1;
                    }
                    Prim::INeg => {
                        self.code.push(ByteCode::INeg);
                    }
                }
            }
        }
    }

    pub fn codegen_super(&mut self, sprc: &SuperComb ) {
        let adr = self.address();

        let SuperComb { name, args, body } = sprc;
        
        let arg_len = args.len();
        let mut arg_map = HashMap::new();
        for (i, arg) in args.iter().enumerate() {
            arg_map.insert(arg, i);
        }



        self.code.push(ByteCode::Pop(arg_len + self.offset));
        self.offset = 0;

    }
}