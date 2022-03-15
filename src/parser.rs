use logos::{Lexer,Span};

use crate::{lexer::*, symbol::SymTable};

pub struct Parser<'src> {
    lexer: Lexer<'src>,
    table: SymTable<'src>,
    stack: Vec<(Token,Span)>,
    index: usize,
}

impl<'src> Parser<'src> {
    pub fn new(string: &'src String) -> Parser {
        Parser { 
            lexer: Lexer::from_string(string),
            table: SymTable::with_capacity(256),
            stack: Vec::new(),
            index: 0,
        }
    }
    pub fn next(&mut self) -> Option<Token> {
        self.index += 1;

        if self.index > self.stack.len() {
            if let Some(tok) = self.lexer.next() {
                self.stack.push((tok,self.lexer.span()));
                Some(tok)
            } else {
                None
            }
        } else {
            let (tok,_) = self.stack[self.index];
            Some(tok)
        }
    }
    pub fn token(&self) -> Token {
        let (tok,_) = self.stack[self.index];
        tok
    }

    pub fn span(&self) -> Span {

    }

    pub fn read_token(&mut self, token: Token) -> Option<()> {
        let tok = self.next()?;
        if tok == token {
            Some(())
        } else { None }
    }

    pub fn read_ident(&mut self) -> Option<String> {
        let tok = self.next()?;
        if let Token::Var(x) = tok {
            Some(x)
        } else { None }
    }

    pub fn read_int(&mut self) -> Option<i64> {
        let tok = self.next()?;
        if let Token::Int(x) = tok {
            Some(x)
        } else { None }
    }

    pub fn try_read<T>(&mut self, func: fn(&mut Parser)->Option<T>) -> Option<T> {
        let record = self.index;
        if let Some(value) = func(self) {
            Some(value)
        } else {
            self.index = record;
            None
        }
    }

    pub fn try_read_many<T>(&mut self,
            funcs: Vec<fn(&mut Parser)->Option<T>>) -> Option<T> {
        let record = self.index;
        for func in funcs.iter() {
            if let Some(value) = func(self) {
                return Some(value);
            } else {
                self.index = record;
            }
        }
        None
    }

    pub fn try_peek<T>(&mut self,func: fn(&mut Parser)->Option<T>) -> Option<T> {
        let record = self.index;
        if let Some(value) = func(self) {
            self.index = record;
            return Some(value);
        } else {
            self.index = record;
            return None;
        }
    }

}


pub fn read_term(par: &mut Parser) -> Option<TermRef> {
    par.try_read_many(vec![
        |p|{ read_const_func(p) },
        |p|{
            let value = read_int(p)?;
            Some(i!(value))
        },
        |p|{ read_var(p) },
        |p|{ read_lam(p) },
        |p|{ read_app(p) }
    ])
}
pub fn read_var(par: &mut Parser) -> Option<TermRef> {
    par.try_read(|p|{
        let string = p.read_regex(&*SYMB_RE)?;
        Some(var!(Symb::new(string)))
    })
}
pub fn read_lam(par: &mut Parser) -> Option<TermRef> {
    par.try_read(|p|{
        p.read_string("\\")?;
        p.skip_space();
        let x = read_symb(p)?;
        p.skip_space();
        p.read_string(".")?;
        p.skip_space();
        let t = read_app_list(p)?;
        Some(lam!(x,t))
    })
}
pub fn read_app(par: &mut Parser) -> Option<TermRef> {
    par.try_read(|p|{
        p.read_string("(")?;
        p.skip_space();
        let t = read_app_list(p)?;
        p.skip_space();
        p.read_string(")")?;
        Some(t)
    })
}

pub fn read_app_list(par: &mut Parser) -> Option<TermRef> {
    par.try_read(|p|{
        let mut t1 = read_term(p)?;
        p.skip_space();
        loop {
            if let Some(t2) = read_term(p) {
                t1 = app!(t1,t2);
                p.skip_space();
            } else if let Some(()) = p.read_string(";") {
                p.skip_space();
                let list = read_app_list(p)?;
                t1 = app!(t1,list);
                p.skip_space();
            } else {
                break;
            }
        }
        Some(t1)
    })
}

pub fn read_const_func(par: &mut Parser) -> Option<TermRef> {
    macro_rules! const_parser {
        ($term:expr, $str:expr) => {
            |p| {
                p.read_string($str)?;
                Some($term)
            }
        };
    }
}

pub fn read_path(par: &mut Parser) -> Option<String> {
    par.try_read(|p|{
        let string = p.read_regex(&*PATH_RE)?;
        Some(string.to_string())
    })
}