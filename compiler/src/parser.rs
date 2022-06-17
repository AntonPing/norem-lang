use core::panic;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

use crate::utils::*;
use crate::lexer::*;
use crate::symbol::*;
use crate::ast::*;

pub struct Parser<'src> {
    lexer: Lexer<'src>,
    // current token and span of lexer
    token: Token,
    span: Span,
    // the "end" field of last token position
    last_end: Position,
    // for error message
    //error: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    LexerError(String),
    Unexpected(Vec<Token>,Token), // Expecting "foo" but found "bar"
    FailedToParse(String), 
    AtLeastOne,
}

type ParseResult<T> = Result<T,Vec<ParseError>>;
type ParseFunc<T> = fn(&mut Parser) -> ParseResult<T>;

impl<'src> Parser<'src> {
    pub fn new(input: &'src str) -> Parser<'src> {
        Parser { 
            lexer: Lexer::new(input),
            token: Token::StartOfFile,
            span: Span::default(),
            last_end: Position::default(),
            //error: Vec::new(),
        }
    }

    pub fn token(&self) -> Token {
        self.token
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn start(&self) -> Position {
        self.span.start
    }

    pub fn end(&mut self, start: Position) -> Span {
        let span = Span::new(start,self.last_end);
        span
    } 

    pub fn next(&mut self) -> ParseResult<()>{
        match self.lexer.next_token() {
            Ok((token,span)) => {
                // record the end of last token
                self.last_end = self.span.end;
                // updating new token and span
                self.token = token;
                self.span = span;
                Ok(())
            }
            Err(msg) => {
                Err(vec![ParseError::LexerError(msg)])
            }
        }
    }

    pub fn next_token(&mut self) -> ParseResult<Token> {
        let _ = self.next()?;
        Ok(self.token())
    }

    pub fn match_token(&mut self, token: Token) -> ParseResult<()> {
        if self.token() == token {
            self.next()?;
            Ok(())
        } else {
            Err(vec![ParseError::Unexpected(vec![token], self.token())])
        }
    }

    pub fn many<T>(&mut self, func: ParseFunc<T>) -> ParseResult<Vec<T>> {
        let mut vec = Vec::new();
        let mut pos = self.span.start.pos;

        loop {
            match func(self) {
                Ok(res) => {
                    vec.push(res);
                    pos = self.span.start.pos;
                }
                Err(err) => {
                    // check if it failed without consuming any token
                    if self.span.start.pos == pos {
                        return Ok(vec);
                    } else {
                        return Err(err);
                    }
                }
            }
        }
    }

    pub fn many1<T>(&mut self, func: ParseFunc<T>) -> ParseResult<Vec<T>> {
        let first = func(self).map_err(|mut e|
            { e.push(ParseError::AtLeastOne); e })?;
        let mut vec = self.many(func)?;
        vec.insert(0, first);
        Ok(vec)
    }

    pub fn sepby<T>(&mut self, func: ParseFunc<T>, delim: Token) -> ParseResult<Vec<T>> {
        let mut vec = Vec::new();
        let pos = self.span.start.pos;
        
        match func(self) {
            Ok(res) => {
                vec.push(res);
            }
            Err(err) => {
                // check if it failed without consuming any token
                if self.span.start.pos == pos {
                    return Ok(vec);
                } else {
                    return Err(err);
                }
            }
        }
        
        while self.token == delim {
            self.next()?;
            let res = func(self)?;
            vec.push(res);
        }

        Ok(vec)
    }

    pub fn sepby1<T>(&mut self, func: ParseFunc<T>, delim: Token) -> ParseResult<Vec<T>> {
        let first = func(self).map_err(|mut e|
            { e.push(ParseError::AtLeastOne); e })?;
        let mut vec = self.sepby(func,delim)?;
        vec.insert(0, first);
        Ok(vec)
    }

    
    pub fn with_paren<T>(&mut self, func: ParseFunc<T>) -> ParseResult<T> {
        self.match_token(Token::LParen)?;
        let res = func(self)?;
        self.match_token(Token::RParen)?;
        Ok(res)
    }

}


pub fn parse_lit_val(p: &mut Parser) -> ParseResult<LitVal> {
    let res = match p.token() {
        Token::Int(n) => Ok(LitVal::Int(n)),
        Token::Real(n) => Ok(LitVal::Real(n)),
        Token::Bool(n) => Ok(LitVal::Bool(n)),
        Token::Char(n) => Ok(LitVal::Char(n)),
        other => Err(vec![
            ParseError::Unexpected(vec![
                Token::Int(0), Token::Real(0.0),
                Token::Bool(true), Token::Char('a'),
            ], other),
            ParseError::FailedToParse("Literal Value".to_string()),
        ]),
    };
    if res.is_ok() {
        p.next()?;
    }
    res
}

pub fn parse_prim(p: &mut Parser) -> ParseResult<Symbol> {
    match p.token() {
        Token::Prim(sym) => {
            p.next()?;
            Ok(sym)
        }
        other => Err(vec![
            ParseError::Unexpected(vec![
                Token::Prim(Symbol::default()),
            ], other),
        ]),
    }
}

pub fn parse_var(p: &mut Parser) -> ParseResult<Symbol> {
    match p.token() {
        Token::Var(sym) => {
            p.next()?;
            Ok(sym)
        }
        other => Err(vec![
            ParseError::Unexpected(vec![
                Token::Var(Symbol::default()),
            ], other),
        ]),
    }
}

pub fn parse_upvar(p: &mut Parser) -> ParseResult<Symbol> {
    match p.token() {
        Token::UpVar(sym) => {
            p.next()?;
            Ok(sym)
        }
        other => Err(vec![
            ParseError::Unexpected(vec![
                Token::UpVar(Symbol::default()),
            ], other),
        ]),
    }
}

pub fn parse_expr_var(p: &mut Parser) -> ParseResult<ExprVar> {
    (|| {
        let start = p.start();
        let ident = parse_var(p)?;
        let span = p.end(start);
        Ok(ExprVar{ ident, span })
    })().map_err(|mut e : Vec<ParseError>| {
        e.push(ParseError::FailedToParse(
            "Variable".to_string()));
        e
    })
}

pub fn parse_expr_lam(p: &mut Parser) -> ParseResult<ExprLam> {
    (|| {
        let start = p.start();
        p.match_token(Token::Fn)?;
        let args = p.many1(parse_var)?;
        p.match_token(Token::EArrow)?;
        let body = parse_expr(p)?;
        let body = Box::new(body);
        let span = p.end(start);
        Ok(ExprLam{ args, body, span})
    })().map_err(|mut e : Vec<ParseError>| {
        e.push(ParseError::FailedToParse(
            "Lambda Abstraction".to_string()));
        e
    })
}

pub fn parse_expr_app(p: &mut Parser) -> ParseResult<ExprApp> {
    (|| {
        let start = p.start();
        let mut args = p.many1(parse_expr)?;
        let func = Box::new(args.remove(0));
        let span = p.end(start);
        Ok(ExprApp{ func, args, span})
    })().map_err(|mut e : Vec<ParseError>| {
        e.push(ParseError::FailedToParse(
            "Function Application".to_string()));
        e
    })
}

pub fn parse_expr(p: &mut Parser) -> ParseResult<Expr> {
    match p.token() {
        Token::Int(_) | Token::Real(_) |
        Token::Bool(_) | Token::Char(_)  => {
            let start = p.start();
            let lit = parse_lit_val(p)?;
            let span = p.end(start);
            Ok(Expr::Lit(ExprLit{ lit, span }))
        }
        Token::Prim(_) => {
            let start = p.start();
            let prim = parse_prim(p)?;
            let span = p.end(start);
            Ok(Expr::Prim(ExprPrim{ prim, span }))
        }
        Token::Var(_) => {
            let res = parse_expr_var(p)?;
            Ok(Expr::Var(res))
        }
        Token::Fn => {
            let res = parse_expr_lam(p)?;
            Ok(Expr::Lam(res))
        }
        Token::LParen => {
            let res = p.with_paren(parse_expr_app)?;
            Ok(Expr::App(res))
        }
        other => {
            Err(vec![
                ParseError::Unexpected(vec![
                    Token::Int(0), // ....
                ], other)
            ])
        }
    }
}


#[test]
fn parser_test() {
    let string = "fn f x => (f 42 (true) 3.1415)";
    //let string = "fn f g x => (f x) g x";
    let mut par = Parser::new(string);
    assert_eq!(par.token(),Token::StartOfFile);
    par.next().unwrap();

    let res = parse_expr(&mut par);

    println!("{}",res.unwrap())
}