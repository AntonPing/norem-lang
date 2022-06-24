use core::panic;
use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;

use crate::ast::*;
use crate::lexer::*;
use crate::symbol::*;
use crate::utils::*;

pub struct Parser<'src> {
    lexer: Lexer<'src>,
    // current token and span of lexer
    token: Token,
    span: Span,
    // the "end" field of last token position
    last_end: Position,
    // for error message
    error: Vec<ParseError>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    LexerError(Position, String),
    Unexpected(Span, Token, &'static str),
    FailedToParse(Span, &'static str),
    HalfParsed(Span),
    AtLeastOne(Span),
}

type ParseResult<T> = Result<T,()>;
type ParseFunc<T> = fn(&mut Parser) -> ParseResult<T>;

impl<'src> Parser<'src> {
    pub fn new(input: &'src str) -> Parser<'src> {
        Parser {
            lexer: Lexer::new(input),
            token: Token::StartOfFile,
            span: Span::default(),
            last_end: Position::default(),
            error: Vec::new(),
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
        let span = Span::new(start, self.last_end);
        span
    }

    pub fn log_err(&mut self, err: ParseError) {
        self.error.push(err);
    }

    pub fn print_err(&self) {
        for err in &self.error {
            println!("{err:?}");
        }
    }

    pub fn next(&mut self) -> ParseResult<()> {
        match self.lexer.next_token() {
            Ok((token, span)) => {
                // record the end of last token
                self.last_end = self.span.end;
                // updating new token and span
                self.token = token;
                self.span = span;
                Ok(())
            }
            Err(msg) => {
                self.log_err(ParseError::LexerError(self.span.end, msg));
                Err(())
            }
        }
    }

    pub fn next_token(&mut self) -> ParseResult<Token> {
        self.next()?;
        Ok(self.token())
    }

    pub fn match_token(&mut self, token: Token) -> ParseResult<()> {
        let start = self.start();
        if self.token() == token {
            self.next()?;
            Ok(())
        } else {
            let span = self.end(start);
            self.log_err(ParseError::Unexpected(span, token, "Basic"));
            Err(())
        }
    }

    pub fn many<T>(&mut self, func: ParseFunc<T>) -> ParseResult<Vec<T>> {
        let start = self.start();        
        let mut vec = Vec::new();
        let mut len = self.error.len();
        let mut pos = self.span;

        loop {
            match func(self) {
                Ok(res) => {
                    vec.push(res);
                    len = self.error.len();
                    pos = self.span;
                }
                Err(_) => {
                    // check if it failed without consuming any token
                    if self.span == pos {
                        for _ in 0..self.error.len() - len {
                            // undo these logs
                            self.error.pop();
                        }
                        return Ok(vec);
                    } else {
                        let span = self.end(start);
                        self.log_err(ParseError::HalfParsed(span));
                        return Err(());
                    }
                }
            }
        }
    }

    pub fn many1<T>(&mut self, func: ParseFunc<T>) -> ParseResult<Vec<T>> {
        let start = self.start();
        let vec = self.many(func)?;
        if vec.is_empty() {
            let span = self.end(start);
            self.log_err(ParseError::AtLeastOne(span));
            Err(())
        } else {
            Ok(vec)
        }
    }

    pub fn sepby<T>(&mut self, func: ParseFunc<T>, delim: Token) -> ParseResult<Vec<T>> {
        let start = self.start();
        let mut vec = Vec::new();
        let len = self.error.len();
        let pos = self.span;

        match func(self) {
            Ok(res) => {
                vec.push(res);
            }
            Err(_) => {
                // check if it failed without consuming any token
                if self.span == pos {
                    for _ in 0..self.error.len() - len {
                        // undo these logs
                        self.error.pop();
                    }
                    return Ok(vec);
                } else {
                    let span = self.end(start);
                    self.log_err(ParseError::HalfParsed(span));
                    return Err(());
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
        let start = self.start();
        let vec = self.sepby(func,delim)?;
        if vec.is_empty() {
            let span = self.end(start);
            self.log_err(ParseError::AtLeastOne(span));
            Err(())
        } else {
            Ok(vec)
        }
    }

    pub fn with_paren<T>(&mut self, func: ParseFunc<T>) -> ParseResult<T> {
        self.match_token(Token::LParen)?;
        let res = func(self)?;
        self.match_token(Token::RParen)?;
        Ok(res)
    }
}

pub fn parse_lit_val(p: &mut Parser) -> ParseResult<LitVal> {
    let start = p.start();
    (||{
        match p.token() {
            Token::Int(n) => { p.next()?; Ok(LitVal::Int(n)) }
            Token::Real(n) => { p.next()?; Ok(LitVal::Real(n)) }
            Token::Bool(n) => { p.next()?; Ok(LitVal::Bool(n)) }
            Token::Char(n) => { p.next()?; Ok(LitVal::Char(n)) }
            other => {
                let span = p.end(start);
                p.log_err(ParseError::Unexpected(span, other, "Int, Real, Bool or Char"));
                Err(())
            }
        }
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Literal Value"));

    })
}

pub fn parse_prim(p: &mut Parser) -> ParseResult<Symbol> {
    let start = p.start();
    (||{
        match p.token() {
            Token::Prim(sym) => {
                p.next()?;
                Ok(sym)
            }
            other => {
                let span = p.end(start);
                p.log_err(ParseError::Unexpected(span, other, "Prim"));
                Err(())
            }
        }
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Primitive"));
    })
}

pub fn parse_var(p: &mut Parser) -> ParseResult<Symbol> {
    let start = p.start();
    (||{
        match p.token() {
            Token::Var(sym) => {
                p.next()?;
                Ok(sym)
            }
            other => {
                let span = p.end(start);
                p.log_err(ParseError::Unexpected(span, other, "Var"));
                Err(())
            }
        }
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Variable"));
    })
}

pub fn parse_upvar(p: &mut Parser) -> ParseResult<Symbol> {
    let start = p.start();
    (||{
        match p.token() {
            Token::UpVar(sym) => {
                p.next()?;
                Ok(sym)
            }
            other => {
                let span = p.end(start);
                p.log_err(ParseError::Unexpected(span, other, "UpVar"));
                Err(())
            }
        }
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Uppercase Variable"));
    })
}

pub fn parse_expr_var(p: &mut Parser) -> ParseResult<ExprVar> {
    let start = p.start();
    (||{
        let ident = parse_var(p)?;
        let span = p.end(start);
        Ok(ExprVar { ident, span })
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Variable Expression"));
    })
}

pub fn parse_expr_lam(p: &mut Parser) -> ParseResult<ExprLam> {
    let start = p.start();
    (||{
        p.match_token(Token::Fn)?;
        let args = p.many1(parse_var)?;
        p.match_token(Token::EArrow)?;
        let body = parse_expr_outer(p)?;
        let body = Box::new(body);
        let span = p.end(start);
        Ok(ExprLam { args, body, span })
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Lambda Abstraction"));
    })
}

pub fn parse_expr_app(p: &mut Parser) -> ParseResult<ExprApp> {
    let start = p.start();
    (||{
        let mut args = p.many1(parse_expr)?;
        let func = Box::new(args.remove(0));
        let span = p.end(start);
        Ok(ExprApp { func, args, span })
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Function Application"));
    })
}

pub fn parse_expr_let(p: &mut Parser) -> ParseResult<ExprLet> {
    let start = p.start();
    (||{ 
        p.match_token(Token::Let)?;
        let decls = p.many1(parse_decl)?;
        p.match_token(Token::In)?;
        let body = parse_expr_outer(p)?;
        let body = Box::new(body);
        p.match_token(Token::End)?;
        let span = p.end(start);
        Ok(ExprLet { decls, body, span })
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Let-Block"));
    })
}

pub fn parse_expr_case(p: &mut Parser) -> ParseResult<ExprCase> {
    let start = p.start();
    (||{
        p.match_token(Token::Case)?;
        let expr = parse_expr_outer(p)?;
        let expr = Box::new(expr);
        p.match_token(Token::Of)?;
        let rules = p.many(parse_rule)?;
        p.match_token(Token::End)?;
        let span = p.end(start);
        Ok(ExprCase { expr, rules, span })
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Case Expression"));
    })
}

pub fn parse_expr_outer(p: &mut Parser) -> ParseResult<Expr> {
    let start = p.start();
    (||{
        let mut args = p.many1(parse_expr)?;
        if args.len() == 1 {
            Ok(args.pop().unwrap())
        } else {
            let func = Box::new(args.remove(0));
            let span = p.end(start);
            Ok(Expr::App(ExprApp { func, args, span }))
        }
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Function Application or Expression"));
    })
}

pub fn parse_expr(p: &mut Parser) -> ParseResult<Expr> {
    let start = p.start();
    match p.token() {
        Token::Int(_) | Token::Real(_) | Token::Bool(_) | Token::Char(_) => {
            let lit = parse_lit_val(p)?;
            let span = p.end(start);
            Ok(Expr::Lit(ExprLit { lit, span }))
        }
        Token::Prim(_) => {
            let prim = parse_prim(p)?;
            let prim = prim.to_prim();
            let span = p.end(start);
            Ok(Expr::Prim(ExprPrim { prim, span }))
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
        Token::Let => {
            let res = parse_expr_let(p)?;
            Ok(Expr::Let(res))
        }
        Token::Case => {
            let res = parse_expr_case(p)?;
            Ok(Expr::Case(res))
        }
        other => {
            let span = p.end(start);
            p.log_err(ParseError::Unexpected(span, other, "
                Expression!!!!!!!!!!!!
            "));
            Err(())
        }
    }
}

pub fn parse_decl(p: &mut Parser) -> ParseResult<Decl> {
    let start = p.start();
    match p.token() {
        Token::Val => {
            let decl = parse_decl_val(p)?;
            Ok(Decl::Val(decl))
        }
        Token::Data => {
            let decl = parse_decl_data(p)?;
            Ok(Decl::Data(decl))
        }
        Token::Type => {
            let decl = parse_decl_type(p)?;
            Ok(Decl::Type(decl))
        }
        other => {
            let span = p.end(start);
            p.log_err(ParseError::Unexpected(span, other, "
                Decl!!!!!!!!!!!
            "));
            Err(())
        }
    }
}

pub fn parse_decl_val(p: &mut Parser) -> ParseResult<DeclVal> {
    let start = p.start();
    (||{
        p.match_token(Token::Val)?;
        let name = parse_var(p)?;
        let args = p.many(parse_var)?;
        p.match_token(Token::Equal)?;
        let body = parse_expr_outer(p)?;
        let span = p.end(start);
        Ok(DeclVal { name, args, body, span })
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Declaration of Value"));
    })
}

pub fn parse_decl_data(p: &mut Parser) -> ParseResult<DeclData> {
    let start = p.start();
    (||{
        p.match_token(Token::Data)?;
        let name = parse_upvar(p)?;
        let args = p.many(parse_var)?;
        p.match_token(Token::Equal)?;
        let vars = p.sepby(parse_varient, Token::Bar)?;
        let span = p.end(start);
        Ok(DeclData { name, args, vars, span })
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Declaration of Data"));
    })
}

pub fn parse_decl_type(p: &mut Parser) -> ParseResult<DeclType> {
    let start = p.start();
    (||{
        
        p.match_token(Token::Type)?;
        let name = parse_upvar(p)?;
        let args = p.many(parse_var)?;
        p.match_token(Token::Equal)?;
        let typ = parse_type(p)?;
        let span = p.end(start);
        Ok(DeclType { name, args, typ, span })
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Declaration of Data"));
    })
}

pub fn parse_varient(p: &mut Parser) -> ParseResult<Variant> {
    let start = p.start();
    (||{
        let cons = parse_upvar(p)?;
        let args = p.many(parse_type)?;
        let span = p.end(start);
        Ok(Variant { cons, args, span })
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Declaration of Value"));
    })
}

pub fn parse_type(p: &mut Parser) -> ParseResult<Type> {
    let start = p.start();
    (||{
        let mut tys = p.sepby1(parse_single_type, Token::Arrow)?;
        let mut res = tys.remove(0);
        for ty in tys {
            res = Type::Arr(Box::new(res), Box::new(ty));
        }
        Ok(res)
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Declaration of Value"));
    })
}

pub fn parse_single_type(p: &mut Parser) -> ParseResult<Type> {
    match p.token() {
        Token::LitType(sym) => {
            if sym.is_buildin() {
                p.next()?;
                Ok(Type::Lit(sym.to_lit_type()))
            } else {
                panic!("A non-builtin symbol is lexed as Token::LitType!");
            }
        }
        Token::LParen => {
            todo!()
        }
        other => {
            Err(())
        }
    }
}

pub fn parse_rule(p: &mut Parser) -> ParseResult<Rule> {
    let start = p.start();
    (||{
        p.match_token(Token::Bar)?;
        let pat = parse_pattern(p)?;
        p.match_token(Token::EArrow)?;
        let body = parse_expr(p)?;
        let span = p.end(start);
        Ok(Rule { pat, body, span })
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Rule"));
    })
}

pub fn parse_pattern(p: &mut Parser) -> ParseResult<Pattern> {
    let start = p.start();
    (||{
        match p.token() {
            Token::Int(_) | Token::Real(_) |
            Token::Bool(_) | Token::Char(_) =>{
                let lit = parse_lit_val(p)?;
                Ok(Pattern::Lit(lit))
            }
            Token::Var(sym) => {
                p.next()?;
                Ok(Pattern::Var(sym))
            }
            Token::UpVar(sym) => {
                p.next()?;
                // A constructor without arguments doesn't need parens
                Ok(Pattern::App(sym, Vec::new()))
            }
            Token::LParen => p.with_paren(|p| {
                let cons = parse_upvar(p)?;
                let args = p.many(parse_pattern)?;
                Ok(Pattern::App(cons, args))
            }),
            other => {
                let span = p.end(start);
                p.log_err(ParseError::Unexpected(span, other, "
                    Pattern!!!!!!
                "));
                Err(())
            }
        }
    })().map_err(|()| {
        let span = p.end(start);
        p.log_err(ParseError::FailedToParse(span, "Pattern"));
    })
}

pub fn parse_program(p: &mut Parser) -> ParseResult<Expr> {
    p.match_token(Token::StartOfFile)?;
    let res = parse_expr_outer(p)?;
    if p.token() != Token::EndOfFile {
        Err(())
    } else {
        Ok(res)
    }
}

//pub fn parse_program_from_text(text: &str) -> Pas


#[test]
fn parser_test() {
    //let string = "fn f x => (f 42 (true) 3.1415)";
    let string = "
        let
            val x = 42
            type MyInt = Int
            data Color = Red | Blue | Green
        in
            case c of
            | (Red x 42) => 3
            | (Blue (Red x 12) Green) => 2
            | Green => 3
            end
        end
    ";
    let mut par = Parser::new(string);


    let res = parse_program(&mut par);
    if let Ok(res) = res {
        println!("{res}");
    } else {
        par.print_err();
    }
    
}
