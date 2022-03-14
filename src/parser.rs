use logos::Span;

use crate::lexer::*;




pub struct Parser<'src> {
    pub lexer: Lexer<'src>,
    pub tokens: Vec<(Token,Span)>,
    pub index: usize,
}

impl<'src> Parser<'src> {
    pub fn new(string: &'src String) -> Parser {
        Parser { 
            lexer: Lexer::from_string(string),
            tokens: Vec::new(),
            index: 0,
        }
    }
    pub fn next(&mut self) -> Option<Token> {

    }


    pub fn is_end(&mut self) -> Option<()> {
        assert!(self.index <= self.lexer.len());
        if self.index == self.text.len() { Some(()) }
        else { None }
    }

    pub fn read_string(&mut self, string: &str) -> Option<()> {
        let new_index = self.index + string.len();
        if new_index > self.text.len() {
            return None;
        }
        let cut = &self.text[self.index..new_index];
        if string == cut {
            self.index = new_index;
            Some(())
        } else {
            None
        }
    }
    pub fn try_read<T>(&mut self,
                func: fn(&mut Parser)->Option<T>) -> Option<T> {
        let record = self.index;
        if let Some(value) = func(self) {
            return Some(value);
        } else {
            self.index = record;
            return None;
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
    /*
    pub fn try_peek<T>(&mut self,
        func: fn(&mut Parser)->Option<T>) -> Option<T> {
        let record = self.index;
        if let Some(value) = func(self) {
            self.index = record;
            return Some(value);
        } else {
            self.index = record;
            return None;
        }
    }
    */
    pub fn get_rest(&mut self) -> String {
        String::from(&self.text[self.index..])
    }
}

lazy_static::lazy_static! {
    static ref CHAR_RE: Regex = Regex::new(
        r"^.").unwrap();
    static ref SPACE_RE: Regex = Regex::new(
        r"^[\s\t\n]*").unwrap();
    static ref INT_RE: Regex = Regex::new(
        r"^\d+").unwrap();
    static ref SYMB_RE: Regex = Regex::new(
        r"^[_A-Za-z][_A-Za-z0-9]*").unwrap();
    static ref PATH_RE: Regex = Regex::new(
        r"^.+").unwrap();
}

pub fn read_int(par: &mut Parser) -> Option<i64> {
    par.try_read(|p|{
        let string = p.read_regex(&*INT_RE)?;
        let result = string.parse::<i64>().ok()?;
        Some(result)
    })
}
pub fn read_symb(par: &mut Parser) -> Option<Symb> {
    par.try_read(|p|{
        let string = p.read_regex(&*SYMB_RE)?;
        Some(Symb::new(string))
    })
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