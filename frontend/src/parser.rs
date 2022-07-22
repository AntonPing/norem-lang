use crate::ast::*;
use crate::lexer::*;
use norem_utils::diagnostic::*;
use norem_utils::interner::InternStr;
use norem_utils::position::{ Position, Span };
use norem_utils::symbol::Symbol;

pub struct Parser<'src> {
    source: &'src str,
    lexer: Lexer<'src>,
    // current token and span of lexer
    token: Token,
    span: Span,
    // the "end" field of last token position
    last_end: Position,
    // for error message
    error: Vec<Diagnostic>,
}

/*
#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    LexerError(Span, String),
    Unexpected(Span, Token, &'static str),
    FailedToParse(Span, &'static str),
    HalfParsed(Span),
    AtLeastOne(Span),
}
*/

type ParseResult<T> = Result<T,Diagnostic>;
type ParseFunc<T> = fn(&mut Parser) -> ParseResult<T>;

impl<'src> Parser<'src> {
    pub fn new(input: &'src str) -> Parser<'src> {
        Parser {
            source: input,
            lexer: Lexer::new(input),
            token: Token::StartOfFile,
            span: Span::dummy(),
            last_end: Position::dummy(),
            error: Vec::new(),
        }
    }

    pub fn print_err(&self) {
        for err in &self.error {
            println!("{}",err.report(self.source, 0));
        }
    }

    pub fn token(&self) -> Token {
        self.token
    }

    pub fn span(&self) -> Span {
        self.span.clone()
    }

    pub fn start(&self) -> Position {
        self.span.start
    }

    pub fn end(&mut self, start: Position) -> Span {
        let span = Span::new(start, self.last_end);
        span
    }

    pub fn next(&mut self) -> ParseResult<()> {
        let (tok, span) = self.lexer.next_token();
        // updating new token and span
        self.last_end = self.span.end;
        self.token = tok;
        self.span = span;
        if let Token::BadToken(msg) = tok {
            Err(Diagnostic::error("Bad Token")
                .line("an lexer error occured during the lexing pass")
                .span(self.span(), msg))
        } else {
            Ok(())
        }
    }

    pub fn next_token(&mut self) -> ParseResult<Token> {
        self.next()?;
        Ok(self.token)
    }

    pub fn match_token(&mut self, token: Token) -> ParseResult<()> {
        if self.token == token {
            self.next()?;
            Ok(())
        } else {
            Err(Diagnostic::error("Unexpected token")
                .line(format!("expected token {}",token))
                .line(format!("but found token {}",self.token))
                .span(self.span(), "here is the token")
            )
        }
    }

    pub fn match_int(&mut self) -> ParseResult<i64> {
        if let Token::Int(x) = self.token {
            self.next()?;
            Ok(x)
        } else {
            Err(Diagnostic::error("Unexpected token")
                .line("expected an interger token")
                .line(format!("but found token {}",self.token))
                .span(self.span(), "here is the token")
            )
        }
    }

    pub fn match_lit(&mut self) -> ParseResult<LitVal> {
        match self.token {
            Token::Int(n) => { self.next()?; Ok(LitVal::Int(n)) }
            Token::Real(n) => { self.next()?; Ok(LitVal::Real(n)) }
            Token::Bool(n) => { self.next()?; Ok(LitVal::Bool(n)) }
            Token::Char(n) => { self.next()?; Ok(LitVal::Char(n)) }
            other => {
                Err(Diagnostic::error("Unexpected token")
                    .line("expected Int, Real, Bool or Char")
                    .line(format!("but found token {}", other))
                    .span(self.span(), "here is the token")
                )
            }
        }
    }

    pub fn match_prim(&mut self) -> ParseResult<Prim> {
        if let Token::Prim(x) = self.token {
            self.next()?;
            Ok(x)
        } else {
            Err(Diagnostic::error("Unexpected token")
                .line("expected a primitive")
                .line(format!("but found token {}",self.token))
                .span(self.span(), "here is the token")
            )
        }
    }

    pub fn match_var(&mut self) -> ParseResult<Symbol> {
        if let Token::Var(x) = self.token {
            self.next()?;
            Ok(Symbol::Var(x))
        } else {
            Err(Diagnostic::error("Unexpected token")
                .line("expected an variable")
                .line(format!("but found token {}",self.token))
                .span(self.span(), "here is the token")
            )
        }
    }

    pub fn match_cons(&mut self) -> ParseResult<Symbol> {
        if let Token::UpVar(x) = self.token {
            self.next()?;
            Ok(Symbol::Var(x))
        } else {
            Err(Diagnostic::error("Unexpected token")
                .line("expected a constructor")
                .line(format!("but found token {}",self.token))
                .span(self.span(), "here is the token")
            )
        }
    }

    pub fn match_tyvar(&mut self) -> ParseResult<Symbol> {
        if let Token::Var(x) = self.token {
            self.next()?;
            Ok(Symbol::Var(x))
        } else {
            Err(Diagnostic::error("Unexpected token")
                .line("expected an type variable")
                .line(format!("but found token {}",self.token))
                .span(self.span(), "here is the token")
            )
        }
    }

    pub fn match_tycons(&mut self) -> ParseResult<Symbol> {
        if let Token::UpVar(x) = self.token {
            self.next()?;
            Ok(Symbol::Var(x))
        } else {
            Err(Diagnostic::error("Unexpected token")
                .line("expected an type constructor")
                .line(format!("but found token {}",self.token))
                .span(self.span(), "here is the token")
            )
        }
    }

    pub fn match_opr(&mut self) -> ParseResult<Symbol> {
        if let Token::Opr(x) = self.token {
            self.next()?;
            Ok(Symbol::Var(x))
        } else {
            Err(Diagnostic::error("Unexpected token")
                .line("expected an operator")
                .line(format!("but found token {}",self.token))
                .span(self.span(), "here is the token")
            )
        }
    }

    pub fn many<T>(
        &mut self,
        func: ParseFunc<T>,
    ) -> ParseResult<Vec<T>> {      
        let mut vec = Vec::new();
        let mut pos = self.span.start.abs;

        loop {
            match func(self) {
                Ok(res) => {
                    vec.push(res);
                    pos = self.span.start.abs;
                }
                Err(err) => {
                    // if it failed without consuming any token
                    if self.span.start.abs == pos {
                        // return the result
                        return Ok(vec);
                    } else {
                        // rethrow the error
                        return Err(err);
                    }
                }
            }
        }
    }

    pub fn many1<T>(
        &mut self,
        func: ParseFunc<T>,
    ) -> ParseResult<Vec<T>> {
        let first = func(self)?;
        let mut vec = self.many(func)?;
        vec.insert(0, first);
        Ok(vec)
    }

    pub fn sepby<T>(
        &mut self,
        delim: Token,
        func: ParseFunc<T>,
    ) -> ParseResult<Vec<T>> {
        let mut vec = Vec::new();
        let pos = self.span.start.abs;

        match func(self) {
            Ok(res) => {
                vec.push(res);
            }
            Err(err) => {
                // if it failed without consuming any token
                if self.span.start.abs == pos {
                    // return the empty vec
                    return Ok(vec);
                } else {
                    // rethrow the error
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

    pub fn sepby1<T>(
        &mut self,
        delim: Token,
        func: ParseFunc<T>,
    ) -> ParseResult<Vec<T>> {
        let first = func(self)?;
        if self.token == delim {
            self.next()?;
            let mut vec = self.sepby(delim, func)?;
            vec.insert(0, first);
            Ok(vec)
        } else {
            Ok(vec![first])
        }
    }
}

pub fn parse_expr_var(p: &mut Parser) -> ParseResult<ExprVar> {
    let start = p.start();
    let name = p.match_var().map_err(|err| err
        .line("failed to parse variable")
    )?;
    let span = p.end(start);
    Ok(ExprVar { name, span })
}

pub fn parse_expr_lam(p: &mut Parser) -> ParseResult<ExprLam> {
    let start = p.start();

    p.match_token(Token::Fn)?;
    let args = p.many1(|p| p.match_var())?;
    p.match_token(Token::EArrow)?;
    let body = Box::new(parse_maybe_expr_chain(p)?);

    let span = p.end(start);
    Ok(ExprLam { args, body, span })
}

/*
pub fn parse_expr_app(p: &mut Parser) -> ParseResult<ExprApp> {
    let start = p.start();

    p.match_token(Token::LParen)?;
    let func = Box::new(parse_expr(p)?);
    let args = p.many(parse_expr)?;
    p.match_token(Token::RParen)?;

    let span = p.end(start);
    Ok(ExprApp { func, args, span })
}
*/

pub fn parse_expr_let(
    p: &mut Parser
) -> ParseResult<ExprLet> {
    let start = p.start();

    p.match_token(Token::Let)?;
    let decls = p.many1(parse_decl)?;
    p.match_token(Token::In)?;
    let body = Box::new(parse_maybe_expr_chain(p)?);
    p.match_token(Token::End)?;

    let span = p.end(start);
    Ok(ExprLet { decls, body, span })
}

pub fn parse_expr_case(
    p: &mut Parser
) -> ParseResult<ExprCase> {
    let start = p.start();

    p.match_token(Token::Case)?;
    let expr = Box::new(parse_maybe_expr_chain(p)?);
    p.match_token(Token::Of)?;
    let rules = p.many(parse_rule)?;
    p.match_token(Token::End)?;

    let span = p.end(start);
    Ok(ExprCase { expr, rules, span })
}

/*
pub fn parse_expr_block(
    p: &mut Parser
) -> ParseResult<ExprBlock> {
    let start = p.start();

    p.match_token(Token::Do)?;
    let mut stats = Vec::new();
    loop {
        let stat = parse_statment(p)?;
        if let Stat::Ret(_) = &stat {
            stats.push(stat);
            break;
        } else {
            stats.push(stat);
        }
    }

    let span = p.end(start);
    Ok(ExprBlock { stats, span })
}

pub fn parse_statment(p: &mut Parser) -> ParseResult<Stat> {
    let start = p.start();

    match p.token() {
        Token::Let => {
            p.next()?;
            let name = p.match_var()?;

            let tok = p.token();

            if tok != Token::Equal && tok != Token::BArrow {
                return Err(Diagnostic::error("Unexpected token")
                    .line("expected token Equal or BArrow")
                    .line(format!("but found token {}",tok))
                    .span(p.span, "here is the token")
                );
            } else {
                p.next()?;
            }

            let body =  Box::new(parse_expr_chain(p)?);
            p.match_token(Token::Semicolon)?;
            let span = p.end(start);

            match tok {
                Token::Equal => {
                    Ok(Stat::Let(StatLet { name, body, span }))
                }
                Token::BArrow => {
                    Ok(Stat::Bind(StatBind { name, body, span }))
                }
                _ => unreachable!(),
                
            }
        }
        Token::Return => {
            p.next()?;
            let body = Box::new(parse_expr_chain(p)?);
            p.match_token(Token::Semicolon)?;

            let span = p.end(start);
            Ok(Stat::Ret(StatRet { body, span }))
        }
        _ => {
            let body = Box::new(parse_expr_chain(p)?);
            p.match_token(Token::Semicolon)?;

            let span = p.end(start);
            Ok(Stat::Drop(StatDrop { body, span }))
        }
    }
}

*/

pub fn parse_maybe_expr_app(
    p: &mut Parser
) -> ParseResult<Expr> {
    let start = p.start();

    let func = parse_expr(p)?;
    let args = p.many(parse_expr)?;

    if args.is_empty() {
        Ok(func)
    } else {
        let func = Box::new(func);
        let span = p.end(start);
        Ok(Expr::App(ExprApp { func, args, span }))
    }
}

pub fn parse_maybe_expr_chain(
    p: &mut Parser
) -> ParseResult<Expr> {
    let start = p.start();

    let head = parse_maybe_expr_app(p)?;
    let tail = p.many(|p| {
        let opr = p.match_opr()?;
        let expr = parse_maybe_expr_app(p)?;
        Ok((opr, expr))
    })?;

    if tail.is_empty() {
        Ok(head)
    } else {
        let head = Box::new(head);
        let span = p.end(start);
        Ok(Expr::Chain(ExprChain { head, tail, span }))
    }
}

pub fn parse_expr(
    p: &mut Parser
) -> ParseResult<Expr> {
    let start = p.start();
    match p.token() {
        Token::Int(_) | Token::Real(_) |
        Token::Bool(_) | Token::Char(_) => {
            let lit = p.match_lit()?;
            let span = p.end(start);
            Ok(Expr::Lit(ExprLit { lit, span }))
        }
        Token::Prim(prim) => {
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
            p.match_token(Token::LParen)?;
            let res = parse_maybe_expr_chain(p)?;
            p.match_token(Token::RParen)?;
            Ok(res)
        }
        Token::Let => {
            let res = parse_expr_let(p)?;
            Ok(Expr::Let(res))
        }
        Token::Case => {
            let res = parse_expr_case(p)?;
            Ok(Expr::Case(res))
        }
        /*
        Token::Do => {
            let res = parse_expr_block(p)?;
            Ok(Expr::Block(res))
        }
        */
        other => {
            let span = p.end(start);
            Err(Diagnostic::error("Unexpected token")
                .line("failed to parse an expression")
                .line(format!("first token {} doesn't match any pattern",other))
                .span(span, "here is the token")
            )
        }
    }
}

pub fn parse_decl(
    p: &mut Parser
) -> ParseResult<Decl> {
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
        Token::Infixl | Token::Infixr | Token::Nonfix => {
            let decl = parse_decl_opr(p)?;
            Ok(Decl::Opr(decl))
        }
        other => {
            let span = p.end(start);
            Err(Diagnostic::error("Unexpected token")
                .line("failed to parse an declaration")
                .line(format!("first token {} doesn't match any pattern",other))
                .span(span, "here is the token")
            )
        }
    }
}

pub fn parse_decl_val(
    p: &mut Parser
) -> ParseResult<DeclVal> {
    let start = p.start();

    p.match_token(Token::Val)?;
    let name = p.match_var()?;
    let args = p.many(|p| p.match_var())?;
    p.match_token(Token::Equal)?;
    let body = parse_maybe_expr_chain(p)?;

    let span = p.end(start);
    Ok(DeclVal { name, args, body, span })
}

pub fn parse_decl_data(
    p: &mut Parser
) -> ParseResult<DeclData> {
    let start = p.start();

    p.match_token(Token::Data)?;
    let name = p.match_tycons()?;
    let args = p.many(|p| p.match_tyvar())?;
    p.match_token(Token::Equal)?;
    let vars = p.sepby(Token::Bar, parse_varient)?;

    let span = p.end(start);
    Ok(DeclData { name, args, vars, span })
}

pub fn parse_decl_type(
    p: &mut Parser
) -> ParseResult<DeclType> {
    let start = p.start();

    p.match_token(Token::Type)?;
    let name = p.match_cons()?;
    let args = p.many(|p| p.match_var())?;
    p.match_token(Token::Equal)?;
    let typ = parse_type(p)?;
    let span = p.end(start);
    Ok(DeclType { name, args, typ, span })
}

pub fn parse_decl_opr(
    p: &mut Parser
) -> ParseResult<DeclOpr> {
    let start = p.start();

    let fixity = match p.token() {
        Token::Infixl => Fixity::Infixl,
        Token::Infixr => Fixity::Infixr,
        Token::Nonfix => Fixity::Nonfix,
        other => return Err(Diagnostic::error("Unexpected token")
            .line("expected infixl, infixr or nonfix")
            .line(format!("but found token {}", other))
            .span(p.span(), "here is the token")),
    };
    
    let prec = p.match_int()? as u8;
    let name = p.match_var()?;
    p.match_token(Token::Equal)?;
    // todo: not only lowercase function, but also cons
    let func = p.match_var()?;

    let span = p.end(start);
    Ok(DeclOpr { fixity, prec, name, func, span })
}

pub fn parse_varient(
    p: &mut Parser
) -> ParseResult<Variant> {
    let start = p.start();

    let cons = p.match_cons()?;
    let args = p.many(parse_type)?;

    let span = p.end(start);
    Ok(Variant { cons, args, span })
}

pub fn parse_type(
    p: &mut Parser
) -> ParseResult<Type> {
    let mut tys = p.sepby1(Token::Arrow, parse_single_type)?;
    let mut res = tys.remove(0);
    for ty in tys {
        res = Type::Arr(Box::new(res), Box::new(ty));
    }
    Ok(res)
}

pub fn parse_single_type(p: &mut Parser) -> ParseResult<Type> {
    match p.token() {
        Token::LitType(lit) => {
            p.next_token()?;
            Ok(Type::Lit(lit))
        }
        Token::LParen => {
            todo!()
        }
        other => {
            Err(Diagnostic::error("Failed to parse a single type"))
        }
    }
}

pub fn parse_rule(
    p: &mut Parser
) -> ParseResult<Rule> {
    let start = p.start();

    p.match_token(Token::Bar)?;
    let pat = parse_pattern(p)?;
    p.match_token(Token::EArrow)?;
    let body = parse_maybe_expr_chain(p)?;

    let span = p.end(start);
    Ok(Rule { pat, body, span })
}

pub fn parse_pattern(
    p: &mut Parser
) -> ParseResult<Pattern> {
    let start = p.start();

    match p.token() {
        Token::Int(_) | Token::Real(_) |
        Token::Bool(_) | Token::Char(_) =>{
            let lit = p.match_lit()?;
            Ok(Pattern::Lit(lit))
        }
        Token::Var(_) => {
            let sym = p.match_var()?;
            Ok(Pattern::Var(sym))
        }
        Token::UpVar(_) => {
            let sym = p.match_cons()?;
            // A constructor without arguments doesn't need parens
            Ok(Pattern::App(sym, Vec::new()))
        }
        Token::LParen => {
            p.match_token(Token::LParen)?;
            let cons = p.match_cons()?;
            let args = p.many(parse_pattern)?;
            p.match_token(Token::RParen)?;
            Ok(Pattern::App(cons, args))
        },
        _other => {
            let _span = p.end(start);
            Err(Diagnostic::error("Failed to parse a pattern"))
        }
    }
}

pub fn parse_program(
    p: &mut Parser
) -> ParseResult<Expr> {
    p.match_token(Token::StartOfFile)?;
    let res = parse_maybe_expr_chain(p)?;
    if p.token() != Token::EndOfFile {
        Err(Diagnostic::error("expecting tokens but file ended"))
    } else {
        Ok(res)
    }
}


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
            | (Red x 42) => 42
            | (Blue (Red x 12) Green) => 2
            | Green => 3
            end
        end
    ";
    let mut par = Parser::new(string);

    let res = parse_program(&mut par);

    match res {
        Ok(res) => println!("{res}"),
        Err(err) => println!("{}",err.report(string,1)),
    }
}
