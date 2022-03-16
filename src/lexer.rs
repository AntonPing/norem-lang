use std::fmt::Display;

use logos::{self,Span,Logos,Lexer};

/*
use crate::symbol::*;

pub struct Lexer<'src> {
    lexer: logos::Lexer<'src,Token>,
    token: Token, // this is a buffer for peeking
}

impl<'src> Lexer<'src> {
    pub fn from_string(string: &'src String) -> Lexer<'src> {
        Lexer {
            lexer: logos::Lexer::new(string.as_str()),
            token: Token::Error // need initialize
        }
    }
    pub fn next(&mut self) -> Option<Token> {
        if let Some(tok) = self.lexer.next() {
            self.token = tok.clone();
            Some(tok)
        } else { None }
    }
    pub fn token(&self) -> Token {
        // peeking the last token
        self.token.clone()
    }
    pub fn span(&self) -> Span {
        self.lexer.span()
    }
    pub fn slice(&self) -> &'src str  {
        self.lexer.slice()
    }
    fn dump_all(&mut self, f: &mut fmt::Formatter) -> fmt::Result {
        while let Some(t) = self.next() {
            writeln!(f,"{:?} {:?} {}",t,self.span(),self.slice())?;
        }
        Ok(())
    }
}

impl<'src> Display for Lexer<'src> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{:?} {:?} {}",self.token(),self.span(),self.slice())
    }
}
*/


#[derive(Logos, Debug, Eq, PartialEq, Clone, Copy)]
pub enum Token {
    #[token("(")]
    LParen,
    
    #[token(")")]
    RParen,

    #[token("[")]
    LBracket,
    
    #[token("]")]
    RBracket,

    #[token("{")]
    LBrace,
    
    #[token("}")]
    RBrace,

    #[token(",")]
    Comma,
    
    #[token(";")]
    Semicolon,

    #[token("let")]
    Let,
    
    #[token("in")]
    In,
    
    #[token("end")]
    End,

    #[token("fn")]
    Fn,
    
    #[token("=>")]
    EArrow,
    
    #[token("->")]
    Arrow,
    
    #[token("+")]
    Add,
    
    #[token("-")]
    Sub,
    
    #[token("*")]
    Mul,
    
    #[token("/")]
    Div,

    #[token("=")]
    Equal,

    #[regex(r#"[0-9]+"#)]
    Int,

    #[regex(r#"[0-9]+\.[0-9]+"#)]
    Real,

    #[token("true")]
    #[token("false")]
    Bool,

    #[regex("\".+\"")]
    String,

    #[regex(r#"[a-zA-Z][a-zA-Z]*"#)]
    Var,

    #[error]
    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    Error,
}

/// Comparing Reals are not recommanded
// impl Eq for Token {}

#[test]
fn lexer_test() {
    // let string = "fn f x => { f 42 (true)}";
    let string = "fn f x => f x";
    let mut lex = Lexer::<Token>::new(string);

    while let Some(tok) = lex.next() {
        println!("{:?} [{:?}] \"{}\"", tok, lex.span(), lex.slice());
    }
}