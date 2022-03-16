use std::ptr::NonNull;
use std::str::FromStr;

use logos::{Lexer,Span, Source, Logos};

use crate::lexer::*;
use crate::symbol::{SymTable, Symbol};
use crate::ast::*;

pub struct Parser<'src> {
    lexer: Lexer<'src,Token>,
    table: SymTable<'src>,
    // for caching lexed tokens, spans, and slices
    stack: Vec<(Token,Span,&'src str)>,
    // since we sometimes backtracks
    index: usize,
}

impl<'src> Parser<'src> {
    pub fn new(input: &'src str) -> Parser {
        Parser { 
            lexer: Lexer::new(input),
            table: SymTable::with_capacity(256),
            stack: vec![(
                Token::Error,
                std::ops::Range {start: 0, end: 0 },
                "???")],
            index: 0,
        }
    }
    pub fn next(&mut self) -> Option<Token> {
        assert!(self.index <= self.stack.len() - 1);
        self.index += 1;
        // println!("{} ?= {}",self.index, self.stack.len());
        if self.index == self.stack.len(){
            if let Some(tok) = self.lexer.next() {
                self.stack.push(
                    (tok, self.lexer.span(), self.lexer.slice())
                );
                // println!("new token {:?}", tok);
                Some(tok)
            } else {
                None
            }
        } else {
            let (tok,_,_) = self.stack[self.index];
            // println!("old token {:?}", tok);
            Some(tok)
        }
    }
    pub fn token(&self) -> Token {
        let (token,_,_) = self.stack[self.index];
        token
    }

    pub fn span(&self) -> Span {
        let (_,span,_) = &self.stack[self.index];
        span.clone()
    }

    pub fn slice(&self) -> &'src str{
        let (_,_,slice) = self.stack[self.index];
        slice
    }

    /*
    pub fn parse<T:FromStr>(&self) -> Option<T> {
        self.slice().parse().ok()
    } 
    */

    pub fn try_read<T>(&mut self,
            func: fn(&mut Parser)->Option<T>) -> Option<T> {
        let record = self.index;
        if let Some(value) = func(self) {
            Some(value)
        } else {
            self.index = record;
            None
        }
    }

    pub fn choices<T>(&mut self,
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

    pub fn many<T>(&mut self,
            func: fn(&mut Parser)->Option<T>) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(res) = self.try_read(func) {
            vec.push(res);
        }
        vec
    }

    pub fn many1<T>(&mut self,
            func: fn(&mut Parser)->Option<T>) -> Option<Vec<T>> {
        let mut vec = Vec::new();
        vec.push(self.try_read(func)?); // at least one element
        while let Some(res) = self.try_read(func) {
            vec.push(res);
        }
        Some(vec)
    }

    pub fn with_paren<T>(&mut self,
            func: fn(&mut Parser)->Option<T>) -> Option<T> {
        self.read_token(Token::LParen)?;
        let res = func(self)?;
        self.read_token(Token::RParen)?;
        Some(res)
    }


    pub fn read_token(&mut self, token: Token) -> Option<()> {
        let tok = self.next()?;
        if tok == token {
            println!("{:?} == {:?}", tok, token);
            Some(())
        } else {
            println!("{:?} != {:?}", tok, token);
            None
        }
    }

    pub fn read_eof(&mut self) -> Option<()> {
        if self.next().is_none() {
            Some(())
        } else { None }
    }

    /*
    pub fn check_token(&self, token: Token) -> Option<()> {
        if self.token() == token {
            Some(())
        } else { None }
    }
    */

    pub fn parse_ident(&mut self) -> Option<Symbol> {
        assert_eq!(self.token(), Token::Var);
        Some(self.table.newsym(self.slice()))
    }

    pub fn read_ident(&mut self) -> Option<Symbol> {
        println!("ident!");
        self.read_token(Token::Var)?;
        self.parse_ident()
    }

    pub fn parse_int(&self) -> Option<i64> {
        assert_eq!(self.token(), Token::Int);
        Some(self.slice().parse().unwrap())
    }

    pub fn parse_real(&self) -> Option<f64> {
        assert_eq!(self.token(), Token::Real);
        Some(self.slice().parse().unwrap())
    }

    pub fn parse_bool(&self) -> Option<bool> {
        assert_eq!(self.token(), Token::Bool);
        if self.slice() == "true" {
            Some(true)
        } else if self.slice() == "false" {
            Some(false)
        } else {
            panic!("wrong input!");
        }
    }

    

    pub fn read_lit_value(&mut self) -> Option<LitValue> {
        match self.next()? {
            Token::Int => 
                { self.parse_int().map(|x| LitValue::Int(x)) }
            Token::Real =>
                { self.parse_real().map(|x| LitValue::Real(x)) }
            Token::Bool =>
                { self.parse_bool().map(|x| LitValue::Bool(x)) }
            _ => None
        }
    }

    pub fn read_lam(&mut self) -> Option<Expr> {
        println!("lam!");
        self.read_token(Token::Fn)?;
        println!("pass!");
        let args= self.many1(|p| p.read_ident())?;
        self.read_token(Token::EArrow)?;
        let body = self.read_app()?;
        Some(args.iter().fold(body,
            |e ,x| Expr::Lam(*x,Box::new(e))))
    }

    pub fn read_app(&mut self) -> Option<Expr> {
        let exprs= self.many1(|p| p.read_expr())?;
        Some(exprs.into_iter().reduce(
            |e1,e2| Expr::App(Box::new(e1),Box::new(e2))).unwrap())        
    }
    
    pub fn read_expr(&mut self) -> Option<Expr> {
        println!("expr!");
        self.choices(vec![
            { |p| p.read_lit_value().map(|x| Expr::Lit(x)) },
            { |p| p.read_ident().map(|x| Expr::Var(x)) },
            { |p| p.read_lam() },
            { |p| p.with_paren(|p2| p2.read_app()) },
        ])
    }

}


#[test]
pub fn parser_test() {
    let string = "fn f x => f x";

    let mut par = Parser::new(string);

    let expr = par.read_app();

    println!("{:?}",expr);


}


/*

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
*/