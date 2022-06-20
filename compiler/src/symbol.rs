use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;

use crate::ast::Prim;

lazy_static::lazy_static! {
    static ref GLOB_TABLE: Mutex<SymbTable> = {
        Mutex::new(SymbTable::new())
    };
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Symbol {
    BuiltIn(usize),
    Var(usize),
    VarN(usize, usize),
    Gen(char, usize),
    Str(usize),
    //Forall(usize),
}

impl Default for Symbol {
    fn default() -> Self {
        Symbol::BuiltIn(0)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SymbTable {
    // for symb to int
    sym_map: HashMap<String, usize>,
    // for int to symb
    sym_vec: Vec<String>,
    // number of generated symbol
    gensym_idx: usize,
    // for all constant string
    str_vec: Vec<String>,
}

impl SymbTable {
    pub fn new() -> SymbTable {
        let table = SymbTable::with_capacity(256);
        /*
        for sym in BUILTIN {
            table.newsym(sym);
        }
        */
        table
    }

    pub fn with_capacity(n: usize) -> SymbTable {
        SymbTable {
            sym_map: HashMap::with_capacity(n),
            sym_vec: Vec::with_capacity(n),
            gensym_idx: 0,
            str_vec: Vec::with_capacity(n),
        }
    }

    pub fn newsym(&mut self, s: &str) -> Symbol {
        if let Some(n) = self.sym_map.get(s) {
            return Symbol::Var(*n);
        } else {
            let len = self.sym_vec.len();
            self.sym_vec.push(s.to_string());
            self.sym_map.insert(s.to_string(), len);
            Symbol::Var(len)
        }
    }

    pub fn gensym(&mut self, ch: char) -> Symbol {
        let old = self.gensym_idx;
        self.gensym_idx += 1;
        Symbol::Gen(ch, old)
    }

    pub fn newstr(&mut self, s: String) -> Symbol {
        let len = self.str_vec.len();
        self.str_vec.push(s);
        Symbol::Str(len)
    }
    /*
    pub fn get_sym(&self, s: &str) -> Option<Symbol> {
        let idx = self.sym_map.get(s)?;
        Some(Symbol::Var(*idx))
    }
    */

    pub fn get_idx(&self, idx: usize) -> Option<String> {
        let str = self.sym_vec.get(idx)?;
        Some(str.clone())
    }

    pub fn get_str(&self, idx: usize) -> Option<String> {
        let str = self.str_vec.get(idx)?;
        Some(str.clone())
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        let table = GLOB_TABLE.lock().unwrap();
        match self {
            &Symbol::BuiltIn(n) => {
                write!(f, "{}", BUILTIN[n])
            }
            &Symbol::Var(x) => {
                write!(f, "{}", table.get_idx(x).unwrap())
            }
            &Symbol::VarN(x, n) => {
                write!(f, "{}_{n}", table.get_idx(x).unwrap())
            }
            &Symbol::Gen(ch, n) => {
                write!(f, "#{}_{}", ch, n)
            }
            &Symbol::Str(n) => {
                write!(f, "{}", table.get_str(n).unwrap())
            }
        }
    }
}

pub fn newvar(s: &str) -> Symbol {
    let mut table = GLOB_TABLE.lock().unwrap();
    table.newsym(s)
}

pub fn genvar(ch: char) -> Symbol {
    let mut table = GLOB_TABLE.lock().unwrap();
    table.gensym(ch)
}

pub fn builtin(id: usize) -> Symbol {
    Symbol::BuiltIn(id)
}


macro_rules! globals {
    (@step $idx:expr, ) => {
        pub const S_TOTAL_GLOBALS: usize = $idx;
    };
    (@step $idx:expr, $it:ident, $($rest:ident,)*) => {
        pub const $it: Symbol = Symbol::BuiltIn($idx as usize);
        globals!(@step $idx+1usize, $($rest,)*);
    };
    ($($name:ident),+) => {
        //const BUILTIN: [&'static str; S_TOTAL_GLOBALS];
        globals!(@step 0usize, $($name,)*);
    };
}

globals!(
    S_TY_INT,
    S_TY_REAL,
    S_TY_BOOL,
    S_TY_CHAR,
    S_IADD,
    S_ISUB,
    S_IMUL,
    S_IDIV,
    S_INEG,
    S_BNOT
);

const BUILTIN: [&'static str; S_TOTAL_GLOBALS] = [
    "Int",
    "Real",
    "Bool",
    "Char",
    "+",
    "-",
    "*",
    "/",
    "~",
    "!",
];



impl Symbol {
    pub fn as_buildin(&self) -> Option<usize> {
        if let Symbol::BuiltIn(n) = self {
            Some(*n)
        } else {
            None
        }
    }
    pub fn to_prim(&self) -> Prim {
        if let Some(id) = self.as_buildin() {
            match id {
                ADD_ID => Prim::IAdd,
                SUB_ID => Prim::ISub,
                MUL_ID => Prim::IMul,
                DIV_ID => Prim::IDiv,
                NEG_ID => Prim::INeg,
                NOT_ID => Prim::BNot,
            }
        } else {
            panic!("should be a built-in!");
        }
    }
}
