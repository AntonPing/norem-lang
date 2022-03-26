use core::panic;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

use logos::{Lexer,Span, Source, Logos};

use crate::{lexer::*, symbol};
use crate::symbol::*;
use crate::ast::*;

pub struct Parser<'src> {
    lexer: Lexer<'src,Token>,
    table: Rc<RefCell<SymTable<'src>>>,
    // for caching lexed tokens, spans, and slices
    stack: Vec<(Token,Span,&'src str)>,
    // since we sometimes backtracks
    index: usize,
}

type Parsing<T> = fn(&mut Parser) -> Option<T>;

impl<'src> Parser<'src> {
    pub fn new(
        input: &'src str,
        table: Rc<RefCell<SymTable<'src>>>
    ) -> Parser<'src> {
        Parser { 
            lexer: Lexer::new(input),
            table: table,
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

    pub fn slice(&self) -> &'src str {
        let (_,_,slice) = self.stack[self.index];
        slice
    }

    pub fn try_read<T>(&mut self, func: Parsing<T>) -> Option<T> {
        let record = self.index;
        if let Some(value) = func(self) {
            Some(value)
        } else {
            self.index = record;
            None
        }
    }

    pub fn try_peek<T>(&mut self, func: Parsing<T>) -> Option<T> {
        let record = self.index;
        if let Some(value) = func(self) {
            self.index = record;
            return Some(value);
        } else {
            self.index = record;
            return None;
        }
    }

    pub fn choices<T>(&mut self, funcs: Vec<Parsing<T>>) -> Option<T> {
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

    pub fn many<T>(&mut self, func: Parsing<T>) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(res) = self.try_read(func) {
            vec.push(res);
        }
        vec
    }

    pub fn many1<T>(&mut self, func: Parsing<T>) -> Option<Vec<T>> {
        let mut vec = Vec::new();
        vec.push(self.try_read(func)?); // at least one element
        while let Some(res) = self.try_read(func) {
            vec.push(res);
        }
        Some(vec)
    }

    pub fn sep_by<T,D>(&mut self, func: Parsing<T>, delim: Parsing<D>)
            -> Vec<T> {
        self.sep_by1(func, delim).unwrap_or(Vec::new())
    }

    pub fn sep_by1<T,D>(&mut self, func: Parsing<T>, delim: Parsing<D>)
            -> Option<Vec<T>> {
        let mut vec = Vec::new();
        vec.push(self.try_read(func)?); // at least one element
        loop {
            let record = self.index;
            if let Some(_) = delim(self) {
                if let Some(res) = func(self) {
                    vec.push(res);
                } else { self.index = record; break; }
            } else { self.index = record; break; }
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

    pub fn parse_ident(&mut self) -> Symbol {
        assert!(self.token() == Token::Var
            || self.token() == Token::CapVar);
        self.table.borrow_mut().newsym(self.slice())
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

    pub fn read_token(&mut self, token: Token) -> Option<()> {
        let tok = self.next()?;
        if tok == token {
            //println!("{:?} == {:?}", tok, token);
            Some(())
        } else {
            //println!("{:?} != {:?}", tok, token);
            None
        }
    }

    pub fn read_eof(&mut self) -> Option<()> {
        if self.next().is_none() {
            Some(())
        } else { None }
    }

    pub fn read_ident(&mut self) -> Option<Symbol> {
        self.read_token(Token::Var)?;
        let ident = self.parse_ident();
        Some(ident)
    }

    pub fn read_cap_ident(&mut self) -> Option<Symbol> {
        self.read_token(Token::CapVar)?;
        let ident = self.parse_ident();
        Some(ident)
    }

    pub fn read_lit_value(&mut self) -> Option<LitValue> {
        match self.next()? {
            Token::Int =>  { Some(LitValue::Int(self.parse_int())) }
            Token::Real => { Some(LitValue::Real(self.parse_real())) }
            Token::Bool => { Some(LitValue::Bool(self.parse_bool())) }
            _ => None
        }
    }

    pub fn read_expr(&mut self) -> Option<Expr> {
        self.choices(vec![
            { |p| p.read_lit() },
            { |p| p.read_var() },
            { |p| p.read_cons() },
            { |p| p.read_lam() },
            { |p| p.with_paren(|p| p.read_app()) },
            { |p| p.read_let() },
            { |p| p.read_case() },
        ])
    }

    pub fn read_lit(&mut self) -> Option<Expr> {
        let lit = self.read_lit_value()?;
        Some(Expr::Lit(lit))
    }

    pub fn read_var(&mut self) -> Option<Expr> {
        self.read_token(Token::Var)?;
        let ident = self.parse_ident();
        Some(Expr::Var(ident))
    }

    pub fn read_cons(&mut self) -> Option<Expr> {
        self.read_token(Token::CapVar)?;
        let ident = self.parse_ident();
        Some(Expr::Cons(ident))
    }

    pub fn read_lam(&mut self) -> Option<Expr> {
        self.read_token(Token::Fn)?;
        let args= self.many1(|p| p.read_ident())?;
        self.read_token(Token::EArrow)?;
        let body = self.read_app()?;
        Some(args.iter().rev().fold(body,
            |e ,x| Expr::Lam(*x,Box::new(e))))
    }

    pub fn read_app(&mut self) -> Option<Expr> {
        let exprs = self.many1(|p| p.read_expr())?;
        Some(exprs.into_iter().reduce(
            |e1,e2| Expr::App(Box::new(e1),Box::new(e2)))
        .unwrap())        
    }

    pub fn read_let(&mut self) -> Option<Expr> {
        self.read_token(Token::Let)?;
        let decls = self.many1(|p| p.read_decl())?;
        self.read_token(Token::In)?;
        let expr = self.read_expr()?;
        self.read_token(Token::End)?;
        Some(Expr::Let(decls, Box::new(expr)))
    }

    pub fn read_case(&mut self) -> Option<Expr> {
        self.read_token(Token::Case)?;
        let expr = self.read_app()?;
        self.read_token(Token::Of)?;
        let rules = self.many1(|p| {
            p.read_token(Token::Bar)?;
            p.read_rule()
        })?;
        Some(Expr::Case(Box::new(expr), rules))
    }

    pub fn read_rule(&mut self) -> Option<Rule> {
        let first = self.span();

        let pat = self.read_pattern()?;
        self.read_token(Token::EArrow);
        let expr = self.read_expr()?;

        let last = self.span();
        let span = Range { start: first.start, end: last.end };
        
        Some(Rule { pat, expr, span})
    }

    pub fn read_decl(&mut self) -> Option<DeclKind> {
        self.choices(vec![
            { |p| p.read_val_decl() },
            { |p| p.read_data_decl() },
            { |p| p.read_type_decl() },
        ])
    }

    pub fn peek_decl_end(&mut self) -> Option<()> {
        self.try_peek(|p| p.choices(vec![
            { |p| p.read_token(Token::Val) },
            { |p| p.read_token(Token::Data) },
            { |p| p.read_token(Token::Type) },
            { |p| p.read_token(Token::In) },
        ]))?;
        Some(())
    }

    pub fn read_val_decl(&mut self) -> Option<DeclKind> {
        let first = self.span();
        self.read_token(Token::Val)?;
        let name = self.read_ident()?;
        let args = self.many(|p| p.read_ident());
        self.read_token(Token::Equal)?;
        let body = self.read_expr()?;
        self.peek_decl_end()?;
        let last = self.span();
        let span = Range { start: first.start, end: last.end };
        Some(DeclKind::Val(ValDecl{ name, args, body, span }))
    }

    pub fn read_data_decl(&mut self) -> Option<DeclKind> {
        let first = self.span();
        self.read_token(Token::Data)?;
        let name = self.read_cap_ident()?;
        let args = self.many(|p| p.read_ident());
        self.read_token(Token::Equal)?;
        let vars = self.sep_by1(
            |p| p.read_varient(),
            |p| p.read_token(Token::Bar))?;
        self.peek_decl_end()?;
        let last = self.span();
        let span = Range { start: first.start, end: last.end };
        Some(DeclKind::Data(DataDecl{ name, args, vars, span }))
    }

    pub fn read_varient(&mut self) -> Option<Variant> {
        let cons = self.read_cap_ident()?;
        let args = self.many(|p| p.read_type_arr());
        Some(Variant{ cons, args })
    }

    pub fn read_type_decl(&mut self) -> Option<DeclKind> {
        let first = self.span();
        self.read_token(Token::Type)?;
        let name = self.read_cap_ident()?;
        let args = self.many(|p| p.read_ident());
        self.read_token(Token::Equal)?;
        let typ = self.read_type()?;
        self.peek_decl_end()?;
        let last = self.span();
        let span = Range { start: first.start, end: last.end };
        Some(DeclKind::Type(TypeDecl{ name, args, typ, span }))
    }

    pub fn read_pattern(&mut self) -> Option<Pattern> {
        self.choices(vec![
            { |p| p.read_pat_lit() },
            { |p| p.read_pat_var() },
            // construtor without arguments
            { |p| p.read_pat_single_cons() },
            { |p| p.read_pat_app() },
            { |p| p.read_pat_wild() },
        ])
    }

    pub fn read_pat_lit(&mut self) -> Option<Pattern> {
        let lit = self.read_lit_value()?;
        Some(Pattern::Lit(lit))
    }

    pub fn read_pat_var(&mut self) -> Option<Pattern> {
        let x = self.read_ident()?;
        Some(Pattern::Var(x))
    }

    pub fn read_pat_single_cons(&mut self) -> Option<Pattern> {
        let cons = self.read_cap_ident()?;
        Some(Pattern::App(cons, Vec::new()))
    }

    pub fn read_pat_app(&mut self) -> Option<Pattern> {
        self.read_token(Token::LParen)?;
        let cons = self.read_cap_ident()?;
        let args = self.many(|p| p.read_pattern());
        self.read_token(Token::RParen)?;
        Some(Pattern::App(cons, args))
    }

    pub fn read_pat_wild(&mut self) -> Option<Pattern> {
        self.read_token(Token::Wild)?;
        Some(Pattern::Wild)
    }

    pub fn read_type(&mut self) -> Option<Type> {
        self.choices(vec![
            { |p| p.read_type_lit() },
            { |p| p.read_type_var() },
            { |p| p.with_paren(|p| p.read_type_arr()) },
        ])
    }

    pub fn read_type_lit(&mut self) -> Option<Type> {
        let lit = self.read_lit_type()?;
        Some(Type::Lit(lit))
    }

    pub fn read_type_var(&mut self) -> Option<Type> {
        let sym = self.read_cap_ident()?;
        Some(Type::Var(sym))
    }

    pub fn read_type_arr(&mut self) -> Option<Type> {
        let tys = self.sep_by1(
            |p| p.read_type(),
            |p| p.read_token(Token::Arrow)
        )?; 

        Some(tys.into_iter().reduce(
            |t1,t2| Type::Arr(Box::new(t1), Box::new(t2))
        ).unwrap())
    }

    pub fn read_lit_type(&mut self) -> Option<LitType> {
        let sym = self.read_cap_ident()?;

        if sym.is_buildin(INT_ID) {
            Some(LitType::Int)
        } else if sym.is_buildin(REAL_ID) {
            Some(LitType::Real)
        } else if sym.is_buildin(CHAR_ID) {
            Some(LitType::Char)
        } else if sym.is_buildin(BOOL_ID) {
            Some(LitType::Bool)
        } else {
            None
        }
    }
}