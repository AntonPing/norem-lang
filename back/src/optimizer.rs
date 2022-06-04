use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::vec;

//use norem_frontend::ast::ValDecl;
//use norem_frontend::symbol::*;

pub type Symbol = usize;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Operator {
    IAdd,
    ISub,
    IMul,
    IDiv,
    INeg,
    BNot,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Atom {
    Var(Symbol),
    Label(Symbol),
    Int(i64),
    Real(f64),
    Bool(bool),
    Char(char),
}

impl Atom {
    pub fn subst(&mut self, name: &Symbol, value: Atom) {
        if let Atom::Var(x) = self {
            if *x == *name {
                *self = value;
            }
        }
    }
}

pub type InstrId = usize;
pub type BlockId = usize;

#[derive(Clone, Debug, PartialEq)]
pub struct Block {
    index: BlockId,
    name: Symbol,
    args: Vec<Symbol>,
    start: InstrId,
    end: InstrId,
}

#[derive(Clone, Debug, PartialEq)]
pub enum InstrBody {
    Goto(Atom,Vec<Atom>),
    IAdd(Symbol,Atom,Atom),
    ISub(Symbol,Atom,Atom),
    Ifte(Atom,InstrId,InstrId),
    Switch(Atom,Vec<InstrId>),
    Halt(Atom),
}

impl InstrBody {
    pub fn subst(&mut self, name: &Symbol, value: Atom) {
        match self {
            InstrBody::Goto(k, args) => {
                k.subst(name, value);
                for arg in args {
                    arg.subst(name, value);
                }
            }
            InstrBody::IAdd(r, x, y) => {
                x.subst(name, value);
                y.subst(name, value);
            }
            InstrBody::ISub(r, x, y) => {
                x.subst(name, value);
                y.subst(name, value);
            }
            InstrBody::Ifte(p, t, f) => {
                p.subst(name, value);
            }
            _ => unimplemented!()
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Instr {
    index: InstrId,
    block: BlockId,
    last: Option<InstrId>,
    next: Option<InstrId>,
    body: InstrBody,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Optimizer {
    block_arena: Vec<Block>,
    instr_arena: Vec<Instr>,
    blcok_map: HashMap<Symbol,BlockId>,
}

impl Optimizer {
    pub fn new() -> Optimizer {
        Optimizer {
            block_arena: Vec::new(),
            instr_arena: Vec::new(),
            blcok_map: HashMap::new(),
        }
    }

    pub fn instr(&mut self, ins: InstrId) -> &mut Instr {
        &mut self.instr_arena[ins]
    }

    pub fn block(&mut self, blk: BlockId) -> &mut Block {
        &mut self.block_arena[blk]
    }

    pub fn set_start(&mut self, blk: InstrId, start: InstrId) {
        self.block_arena[blk].start = start;
    }

    pub fn set_end(&mut self, blk: InstrId, end: InstrId) {
        self.block_arena[blk].end = end;
    }

    pub fn get_block(&mut self, ins: InstrId) -> BlockId {
        self.instr_arena[ins].block
    }

    pub fn set_block(&mut self, ins: InstrId, block: BlockId) {
        self.instr_arena[ins].block = block;
    }

    pub fn get_next(&mut self, ins: InstrId) -> Option<InstrId> {
        self.instr_arena[ins].next
    }

    pub fn set_next(&mut self, ins: InstrId, next: Option<InstrId>) {
        self.instr_arena[ins].next = next;
    }

    pub fn get_last(&mut self, ins: InstrId) -> Option<InstrId> {
        self.instr_arena[ins].last
    }

    pub fn set_last(&mut self, ins: InstrId, last: Option<InstrId>) {
        self.instr_arena[ins].last = last;
    }

    pub fn get_body(&mut self, ins: InstrId) -> InstrBody {
        self.instr_arena[ins].body.clone()
    }

    pub fn set_body(&mut self, ins: InstrId, body: InstrBody) {
        self.instr_arena[ins].body = body;
    }

    pub fn new_instr(&mut self, body: InstrBody) -> InstrId {
        let len = self.instr_arena.len();
        let newins = Instr {
            index: len,
            block: 0, // need init
            next: None, // need init
            last: None, // need init
            body
        };
        self.instr_arena.push(newins);
        len
    }

    pub fn new_block(&mut self, name: Symbol, args: Vec<Symbol>, body: InstrBody) -> BlockId {
        let len = self.block_arena.len();
        let ins = self.new_instr(body);
        let newblk = Block {
            index: len,
            name,
            args,
            start: ins,
            end: ins,
        };
        self.block_arena.push(newblk);
        len
    }

    pub fn delete_instr(&mut self, ins: InstrId) {
        let block = self.get_block(ins);

        match (self.get_last(ins), self.get_next(ins)) {
            (Some(last),Some(next)) => {
                self.set_next(last, Some(next));
                self.set_last(next, Some(last));
            }
            (Some(last), None) => {
                self.set_next(last, None);
                self.set_end(block, last);
            }
            (None, Some(next)) => {
                self.instr(next).last = None;
                self.set_start(block, next);
            }
            (None, None) => {
                panic!("empty block!")
            }
        }
    }

    pub fn insert_instr(&mut self, ins: InstrId, body: InstrBody) -> InstrId {
        
        let new = self.new_instr(body);

        let block = self.get_block(ins);
        self.set_block(new, block);

        if let Some(next) = self.get_next(ins) {
            self.set_next(ins, Some(new));
            self.set_last(new, Some(ins));
            self.set_next(new, Some(next));
            self.set_last(next, Some(new));
        } else {
            self.set_next(ins, Some(new));
            self.set_last(new, Some(ins));
            self.set_next(new, None);
            self.set_end(block, new);
        }

        new
    }

    pub fn update_instr(&mut self, ins1: InstrId, ins2: InstrId) {
        assert_eq!(self.get_block(ins1),self.get_block(ins2));
        
        let last = self.get_last(ins1);
        self.set_last(ins2, last);
        
        let next = self.get_next(ins1);
        self.set_next(ins2, next);

        if let Some(last) = self.get_last(ins1) {
            self.set_next(last, Some(ins2));
        }
        if let Some(next) = self.get_next(ins1) {
            self.set_last(next, Some(ins2));
        }
    }

}