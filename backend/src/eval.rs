use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use norem_frontend::symbol::*;

use crate::code::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    App(Rc<Object>,Rc<Object>),
    Int(i64),
    Bool(bool),
    Ptr(usize),
    Null,
}

pub struct Machine {
    stack: Vec<Object>,
    frame: Vec<Object>,
    base: usize,
    code: Vec<ByteCode>,
    ip: usize,
}


impl Machine {
    pub fn push(&mut self, obj: Object) {
        self.stack.push(obj);

    }
    pub fn pop(&mut self) -> Object {
        self.stack.pop().unwrap()
    }

    pub fn eval(&mut self) {
        loop {
            match self.code[self.ip] {
                ByteCode::App => {
                    let e1 = self.pop();
                    let e2 = self.pop();
                    let app = Object::App(Rc::new(e1), Rc::new(e2));
                    self.push(app);
                }
                ByteCode::Push(n) => {
                    let len = self.frame.len();
                    let obj = self.frame[len - n].clone();
                    self.push(obj);
                }
                ByteCode::Pop(n) => {
                    let len = self.frame.len();
                    let obj =  self.pop();
                    self.frame[len - n] = obj;
                }
                ByteCode::Jump(adr) => {
                    self.ip = adr;
                    continue;
                }
                ByteCode::JumpTrue(adr) => {
                    if let Object::Bool(p) = self.pop() {
                        if p {
                            self.ip = adr;
                            continue;
                        }
                    } else {
                        panic!("need bool!");
                    }
                }
                ByteCode::JumpFalse(adr) => {
                    if let Object::Bool(p) = self.pop() {
                        if !p {
                            self.ip = adr;
                            continue;
                        }
                    } else {
                        panic!("need bool!");
                    }
                }
                ByteCode::GlobCall(_) => {
                    panic!("this should not appear in the compiled code!");
                }
                ByteCode::Call(adr) => {
                    
                }

                _ => {
                    unimplemented!()
                }
            }

            self.ip += 1;
        }
    }
}