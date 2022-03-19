use std::ops::Range;

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

type Parse<T> = fn(&mut Parser) -> Option<T>;

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

    pub fn parse<T>(&mut self, par: Parse<T>) -> Option<T> {
        par(self)
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
        self.parse_token(Token::LParen)?;
        let res = func(self)?;
        self.parse_token(Token::RParen)?;
        Some(res)
    }

    pub fn parse_token(&mut self, token: Token) -> Option<()> {
        let tok = self.next()?;
        if tok == token {
            //println!("{:?} == {:?}", tok, token);
            Some(())
        } else {
            //println!("{:?} != {:?}", tok, token);
            None
        }
    }

    pub fn parse_eof(&mut self) -> Option<()> {
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

    pub fn parse_ident(&mut self) -> Symbol {
        assert_eq!(self.token(), Token::Var);
        self.table.newsym(self.slice())
    }

    pub fn parse_int(&self) -> i64 {
        assert_eq!(self.token(), Token::Int);
        self.slice().parse().unwrap()
    }

    pub fn parse_real(&self) -> f64 {
        assert_eq!(self.token(), Token::Real);
        self.slice().parse().unwrap()
    }

    pub fn parse_bool(&self) -> bool {
        assert_eq!(self.token(), Token::Bool);
        if self.slice() == "true" {
            true
        } else if self.slice() == "false" {
            false
        } else {
            panic!("wrong input!");
        }
    }
}

pub fn read_ident(p: &mut Parser) -> Option<Symbol> {
    p.parse_token(Token::Var);
    let ident = p.parse_ident();
    Some(ident)
}

pub fn read_lit_value(p: &mut Parser) -> Option<LitValue> {
    match p.next()? {
        Token::Int =>  { Some(LitValue::Int(p.parse_int())) }
        Token::Real => { Some(LitValue::Real(p.parse_real())) }
        Token::Bool => { Some(LitValue::Bool(p.parse_bool())) }
        _ => None
    }
}

pub fn read_var(p: &mut Parser) -> Option<Expr> {
    p.parse_token(Token::Var);
    let ident = p.parse_ident();
    Some(Expr::Var(ident))
}

pub fn read_lam(p: &mut Parser) -> Option<Expr> {
    p.parse_token(Token::Fn)?;
    let args= p.many1(read_ident)?;
    p.parse_token(Token::EArrow)?;
    let body = read_app(p)?;
    Some(args.iter().fold(body,
        |e ,x| Expr::Lam(*x,Box::new(e))))
}

pub fn read_app(p: &mut Parser) -> Option<Expr> {
    let exprs= p.many1(read_expr)?;
    Some(exprs.into_iter().reduce(
        |e1,e2| Expr::App(Box::new(e1),Box::new(e2))).unwrap())        
}

pub fn read_let(p: &mut Parser) -> Option<Expr> {
    p.parse_token(Token::Let)?;
    let decls = p.many1(read_decl)?;
    p.parse_token(Token::In)?;


}

pub fn read_decl(p: &mut Parser) -> Option<DeclKind> {
    

}

pub fn read_val_decl(p: &mut Parser) -> Option<DeclKind> {
    let first = p.span();

    p.parse_token(Token::Val)?;
    let name = read_ident(p)?;
    let args = p.many(read_ident);
    p.parse_token(Token::Equal)?;
    let body = read_expr(p)?;
    p.try_read(|p| p.choices(vec![
        |p| p.parse_token(Token::Val),
        |p| p.parse_token(Token::Data),
        |p| p.parse_token(Token::Type),
        |p| p.parse_token(Token::In),
    ]))?;

    let last = p.span();

    Some(DeclKind::Val(ValDecl{
        name: name,
        args: args,
        body: body,
        span: Range { start: first.start, end: last.end }
    }))
}



pub fn read_expr(p: &mut Parser) -> Option<Expr> {
    println!("expr!");
    self.choices(vec![
        { |p| p.read_lit_value().map(|x| Expr::Lit(x)) },
        { |p| p.read_var() },
        { |p| p.read_lam() },
        { |p| p.with_paren(|p2| p2.read_app()) },
    ])
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