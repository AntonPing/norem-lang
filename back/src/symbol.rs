use std::collections::HashMap;
use std::fmt;

const BUILTIN: [&'static str; 11] = [
    "and",
    "or",
    "not",
    "+",
    "-",
    "*",
    "/",
    "Int",
    "Char",
    "Bool",
    "Real"
];

pub const MIN_BUILDIN : usize = 0;
pub const MAX_BUILDIN : usize = 10;

pub const AND_ID : usize = 0;
pub const OR_ID : usize = 1;
pub const NOT_ID : usize = 2;
pub const ADD_ID : usize = 3;
pub const SUB_ID : usize = 4;
pub const MUL_ID : usize = 5;
pub const DIV_ID : usize = 6;
pub const INT_ID : usize = 7;
pub const CHAR_ID : usize = 8;
pub const BOOL_ID : usize = 9;
pub const REAL_ID : usize = 10;


#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Symbol {
    Var(usize),
    Gen(usize),
    Forall(usize),
    //Wild,
}

impl Symbol {
    pub fn is_buildin(&self, id: usize) -> bool {
        if let Symbol::Var(u) = self {
            *u == id
        } else { false }
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct SymTable {
    // for symb to int
    sym_map: HashMap<String, usize>,
    // for int to symb
    sym_vec: Vec<String>,
    // number of generated symbol
    gensym_idx : usize,
}

impl SymTable {
    pub fn new() -> SymTable {
        let mut table = SymTable::with_capacity(256);
        for sym in BUILTIN {
            table.newsym(sym);
        }
        table
    }
    
    pub fn with_capacity(n: usize) -> SymTable {
        SymTable {
            sym_map: HashMap::with_capacity(n),
            sym_vec: Vec::with_capacity(n),
            gensym_idx: 0,
        }
    }

    pub fn newsym(&mut self, s: &str) -> Symbol {
        if let Some(sym) = self.sym_map.get(s) {
            return Symbol::Var(*sym);
        } else {
            let len = self.sym_vec.len();
            self.sym_map.insert(s.to_string(), len);
            self.sym_vec.push(s.to_string());
            Symbol::Var(len)
        }
    }

    pub fn gensym(&mut self) -> Symbol {
        let old = self.gensym_idx;
        self.gensym_idx += 1;
        Symbol::Gen(self.gensym_idx)
    }

    pub fn get(&self, s: &str) -> Option<Symbol> {
        let idx = self.sym_map.get(s)?;
        Some(Symbol::Var(*idx))
    }

    pub fn get_str(&self, idx: usize) -> Option<String> {
        self.sym_vec.get(idx).map(|s| s.clone())
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            &Symbol::Var(n) => write!(f, "sym({})", n),
            &Symbol::Gen(n) => write!(f, "#{}", n),
            &Symbol::Forall(n) => write!(f,"{}",
                "abcdefghijklmnopqrstuvwxyz".to_string().chars().nth(n).unwrap()
            ),
            //&Symbol::Wild => write!(f, "_")
        }
    }
}

