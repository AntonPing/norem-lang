use std::fmt::Display;

use logos::{self, Lexer, Logos, Span};

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

    #[token("|")]
    Bar,

    #[token("_")]
    Wild,

    #[token("=")]
    Equal,

    #[token("let")]
    Let,

    #[token("in")]
    In,

    #[token("end")]
    End,

    #[token("val")]
    Val,

    #[token("data")]
    Data,

    #[token("type")]
    Type,

    #[token("case")]
    Case,

    #[token("of")]
    Of,

    #[token("fn")]
    Fn,

    #[token("=>")]
    EArrow,

    #[token("->")]
    Arrow,

    #[regex(r#"[0-9]+"#)]
    Int,

    #[regex(r#"[0-9]+\.[0-9]+"#)]
    Real,

    #[token("true")]
    #[token("false")]
    Bool,

    #[regex("\".+\"")]
    String,

    #[regex(r#"[:!#$%&*+./<=>?@\\^|\-~]+"#)]
    Opr,

    #[regex(r#"[a-z][a-zA-Z]*"#)]
    Var,

    #[regex(r#"[A-Z][a-zA-Z]*"#)]
    CapVar,

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
