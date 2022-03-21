use std::collections::HashMap;
use std::fmt;
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Symbol {
    Var(usize),
    Gen(usize),
}

#[derive(Clone, PartialEq)]
pub struct SymTable<'src> {
    // for symb to int
    sym_map: HashMap<&'src str, usize>,
    // for int to symb
    sym_vec: Vec<&'src str>,
    // number of generated symbol
    gensym_idx : usize,
}

impl<'src> SymTable<'src> {
    pub fn new() -> SymTable<'src> {
        SymTable::with_capacity(256)
    }
    
    pub fn with_capacity(n: usize) -> SymTable<'src> {
        SymTable {
            sym_map: HashMap::with_capacity(n),
            sym_vec: Vec::with_capacity(n),
            gensym_idx: 0,
        }
    }

    pub fn newsym(&mut self, s: &'src str) -> Symbol {
        if let Some(sym) = self.sym_map.get(s) {
            return Symbol::Var(*sym);
        } else {
            let len = self.sym_vec.len();
            self.sym_map.insert(s, len);
            self.sym_vec.push(s);
            Symbol::Var(len)
        }
    }

    pub fn gensym(&mut self) -> Symbol {
        let old = self.gensym_idx;
        self.gensym_idx += 1;
        Symbol::Gen(self.gensym_idx)
    }

    pub fn get(&self, s: &'src str) -> Option<Symbol> {
        let idx = self.sym_map.get(s)?;
        Some(Symbol::Var(*idx))
    }

    pub fn get_str(&self, idx: usize) -> Option<&'src str> {
        self.sym_vec.get(idx).map(|s|*s)
    }
}

impl fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Var(n) => write!(f, "sym({})", n),
            Symbol::Gen(n) => write!(f, "#{}", n),
        }
    }
}