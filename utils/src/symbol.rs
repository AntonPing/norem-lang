use std::fmt;
use crate::interner::InternStr;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Symbol {
    Var(InternStr),
    Gen(char,usize)
    /*
    Cons(InternStr),
    TyVar(InternStr),
    TyCons(InternStr),
    Opr(InternStr),
    TempTy(usize),
    */
}


impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Var(x) => {
                write!(f, "{x}")
            }
            Symbol::Gen(ch,n) => {
                write!(f, "{ch}{n}")
            }
            /* 
            Symbol::Cons(x) => {
                write!(f, "{x}")
            }
            Symbol::TyVar(x) => {
                write!(f, "{x}")
            }
            Symbol::TyCons(x) => {
                write!(f, "{x}")
            }
            Symbol::Opr(x) => {
                write!(f, "{x}")
            }
            Symbol::Rename(x, n) => {
                write!(f, "{x}_{n}")
            }
            
            Symbol::TempTy(n) => {
                write!(f, "t{n}")
            }
            */
            
        }
    }
}