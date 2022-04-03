use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use norem_frontend::symbol::*;

use crate::code::*;

pub enum Object {
    App(Rc<Object>,Rc<Object>),
    Int(i64),
    Ptr(usize),
    Null,
}

pub struct Machine {
    stack: Vec<Object>,
    stack_top: Object,
    frame: Vec<Object>,
    frame_top: Object,
    regs: [Object;256],
    reg_len: u8,
    code: Vec<ByteCode>,
    ip: usize,
}


impl Machine {
    pub fn push(&mut self, obj: Object) {
        self.stack.push(self.stack_top);
        self.stack_top = obj;

    }
    pub fn pop(&mut self, reg: usize) {
        self.regs[reg] = self.stack_top;
        self.stack_top = self.stack.pop().unwrap();
    }
    pub fn push_reg(&mut)


    pub fn eval(&mut self) {
        loop {
            match self.code[self.ip] {
                ByteCode::App => {
                    let e2 = self

                    self.ip += 1;
                }
                _ => {
                    unimplemented!()
                }
            }

            
        }
    }
}