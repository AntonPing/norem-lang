use core::panic;
use std::rc::Rc;

use crate::code::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Object {
    Pair(usize,usize),
    Int(i64),
    Real(f64),
    Bool(bool),
    Ptr(usize),
    Null,
}

pub struct Machine {
    stack: Vec<Object>,
    heap: Vec<Object>,
    caller: Vec<usize>,
    code: Vec<ByteCode>,
    ptr: usize,
}

impl Machine {
    pub fn new(code: Vec<ByteCode>) -> Machine {
        Machine {
            stack: Vec::new(),
            heap: Vec::new(),
            caller: Vec::new(),
            code,
            ptr: 0,
        }
    }

    pub fn eval(&mut self) {
        loop {
            match self.code[self.ptr] {
                ByteCode::Push(n) => {
                    let top_idx = self.stack.len() - 1;
                    let obj = self.stack[top_idx - n];
                    self.stack.push(obj);
                }
                ByteCode::Pop(n) => {
                    for _ in 0..n {
                        self.stack.pop();
                    }
                }
                ByteCode::PushInt(x) => {
                    self.stack.push(Object::Int(x));
                }
                ByteCode::PushReal(x) => {
                    self.stack.push(Object::Real(x));
                }
                ByteCode::PushBool(x) => {
                    self.stack.push(Object::Bool(x));
                }

                ByteCode::Jump(adr) => {
                    self.ptr = adr;
                    continue;
                }
                ByteCode::JumpTrue(adr) => {
                    if let Object::Bool(p) = self.stack.pop().unwrap() {
                        if p {
                            self.ptr = adr;
                            continue;
                        }
                    } else {
                        panic!("need bool!");
                    }
                }
                ByteCode::JumpFalse(adr) => {
                    if let Object::Bool(p) = self.stack.pop().unwrap() {
                        if !p {
                            self.ptr = adr;
                            continue;
                        }
                    } else {
                        panic!("need bool!");
                    }
                }
                ByteCode::Call(adr) => {
                    self.caller.push(self.ptr + 1);
                    self.ptr = adr;
                    continue;
                }
                ByteCode::CallArg(n) => {
                    let top_idx = self.stack.len() - 1;
                    let mut obj = self.stack[top_idx - n];

                    while let Object::Pair(hd, tl) = obj {
                        self.stack.push(*tl);
                        obj = *hd;
                    }

                    if let Object::Ptr(adr) = obj {
                        self.caller.push(self.ptr + 1);
                        self.ptr = adr;
                        continue;

                    } else {
                        panic!("calling non-function!");
                    }
                }

                ByteCode::Ret => {
                    let adr = self.caller.pop().unwrap();
                    self.ptr = adr;
                    continue;
                }
                ByteCode::MkPair => {
                    let hd = self.stack.pop().unwrap();
                    let tl = self.stack.pop().unwrap();
                    self.stack.push(Object::Pair(Rc::new(hd), Rc::new(tl)));
                }
                ByteCode::Head => {
                    let obj = self.stack.pop().unwrap();
                    if let Object::Pair(hd, tl) = obj {
                        self.stack.push(*hd)
                    } else {
                        panic!("Unpacking a non-pair!");
                    }
                }
                ByteCode::Tail => {
                    let obj = self.stack.pop().unwrap();
                    if let Object::Pair(hd, tl) = obj {
                        self.stack.push(*tl)
                    } else {
                        panic!("Unpacking a non-pair!");
                    }
                }
                ByteCode::IAdd => {
                    let Object::Int(x) = self.stack.pop().unwrap();
                    let Object::Int(y) = self.stack.pop().unwrap();
                    self.stack.push(Object::Int(x + y));
                }
                ByteCode::ISub => {
                    let Object::Int(x) = self.stack.pop().unwrap();
                    let Object::Int(y) = self.stack.pop().unwrap();
                    self.stack.push(Object::Int(x - y));
                }
                ByteCode::IMul => {
                    let Object::Int(x) = self.stack.pop().unwrap();
                    let Object::Int(y) = self.stack.pop().unwrap();
                    self.stack.push(Object::Int(x * y));
                }
                ByteCode::IDiv => {
                    let Object::Int(x) = self.stack.pop().unwrap();
                    let Object::Int(y) = self.stack.pop().unwrap();
                    self.stack.push(Object::Int(x / y));
                }
                ByteCode::INeg => {
                    let Object::Int(x) = self.stack.pop().unwrap();
                    self.stack.push(Object::Int(-x));
                }

                ByteCode::Halt => todo!(),
            }
            self.ptr += 1;
        }
    }
}