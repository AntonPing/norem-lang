use std::collections::HashMap;
use std::iter::Peekable;
use std::str::Chars;

use crate::utils::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Token {
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    
    Comma,
    Semicolon,
    Bar,
    Wild,
    Equal,

    Let,
    In,
    End,
    Val,
    Data,
    Type,
    Case,
    Of,
    Fn,
    EArrow,
    Arrow,

    Int,
    Real,
    Bool,
    Char,
    String,
    True,
    False,

    LitType,

    Opr,
    Var,
    UpVar,

    StartOfFile,
    EndOfFile,
}

pub fn is_reserved(str: &str) -> Option<Token> {
    let tok = match str {
        "fn" => Token::Fn,
        "let" => Token::Let,
        "in" => Token::In,
        "end" => Token::End,
        "val" => Token::Val,
        "data" => Token::Data,
        "type" => Token::Type,
        "case" => Token::Case,
        "of" => Token::Of,
        "true" => Token::Bool,
        "false" => Token::Bool,
        "Int" => Token::LitType,
        "Real" => Token::LitType,
        "Bool" => Token::LitType,
        "Char" => Token::LitType,
        _ => { return None; }
    };
    Some(tok)
}

pub struct Lexer<'src> {
    source: &'src str,
    stream: Peekable<Chars<'src>>,
    is_end: bool,
    pos: usize,
    row: usize,
    col: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(str: &'src str) -> Self {
        Lexer {
            source: str,
            stream: str.chars().peekable(),
            //peeked: None,
            is_end: false,
            pos: 0,
            row: 0,
            col: 0,
        }

    }

    fn peek_char(&mut self) -> Option<char> {
        let ch = self.stream.peek()?;
        Some(*ch)
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.stream.next()?;
        self.pos += 1;
        if ch == '\n' {
            self.row += 1;
            self.col = 0;
        }
        Some(ch)
    }


    fn peek_satisfy(&mut self, func: fn(char)->bool) -> Option<char> {
        if let Some(ch) = self.peek_char() {
            if func(ch) {
                Some(ch)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn next_satisfy(&mut self, func: fn(char)->bool) -> Option<char> {
        if let Some(ch) = self.peek_char() {
            if func(ch) {
                self.next_char();
                Some(ch)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn comment_block(&mut self) -> Result<(),String> {
        let mut level = 1;

        // an automat of 3 states
        let mut state = 0;

        while level > 0 {
            let ch = self.next_char()
                .ok_or("comment block not closed!".to_string())?;

            match state {
                0 => {
                    match ch {
                        '*' => { state = 1 }
                        '/' => { state = 2 }
                        _ => {}
                    }
                }
                1 => {
                    match ch {
                        '/' => { level -= 1; state = 0 }
                        _ => { state = 0 }
                    }
                }
                2 => {
                    match ch {
                        '*' => { level += 1; state = 0 }
                        _ => { state = 0 }
                    }
                }
                _ => { unreachable!() }
            }
        }

        Ok(())
    }


    pub fn next_token(&mut self) -> Result<(Token,Span,&'src str),String> {

        // skip all whitespaces
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }

        let start = self.pos;

        match self.next_char() {
            Some('/') => {
                match self.next_char() {
                    Some('/') => {
                        while let Some(ch) = self.next_char() {
                            if ch == '\n' {
                                break;
                            }
                        }
                        self.next_token()
                    }
                    Some('*') => {
                        self.comment_block()?;
                        self.next_token()
                    }
                    Some(_) => {
                        Err("divide not supported yet!".to_string())
                    }
                    None => {
                        Err("lexing failed!".to_string())
                    }
                }
            }
            Some('=') => {
                match self.peek_char() {
                    Some('>') => {
                        self.next_char();
                        let end = self.pos;
                        Ok((
                            Token::EArrow,
                            Span::new(start, end),
                            &self.source[start..end]
                        ))
                    }
                    _ => {
                        let end = self.pos;
                        Ok((
                            Token::Equal,
                            Span::new(start, end),
                            &self.source[start..end]
                        ))
                    }
                }
                
            }
            Some('-') => {
                match self.peek_char() {
                    Some('>') => {
                        self.next_char();
                        let end = self.pos;
                        Ok((
                            Token::Arrow,
                            Span::new(start, end),
                            &self.source[start..end]
                        ))
                    }
                    _ => {
                        let end = self.pos;
                        Ok((
                            Token::Opr,
                            Span::new(start, end),
                            &self.source[start..end]
                        ))
                    }
                }
            }
            Some('(') => {
                let end = self.pos;
                Ok((
                    Token::LParen,
                    Span::new(start, end),
                    &self.source[start..end]
                ))
            }
            Some(')') => {
                let end = self.pos;
                Ok((
                    Token::RParen,
                    Span::new(start, end),
                    &self.source[start..end]
                ))
            }
            Some('|') => {
                let end = self.pos;
                Ok((
                    Token::Bar,
                    Span::new(start, end),
                    &self.source[start..end]
                ))
            }
            Some(x) if x.is_ascii_alphabetic() => {
                let upper = x.is_uppercase();

                while let Some(x) = self.peek_char() {
                    if x.is_ascii_alphanumeric() {
                        self.next_char();
                    } else {
                        break;
                    }
                }
                let end = self.pos;

                let slice = &self.source[start..end];

                if let Some(tok) = is_reserved(slice) {
                    Ok((
                        tok,
                        Span::new(start, end),
                        &self.source[start..end]
                    ))
                } else {
                    Ok((
                        if upper { Token::UpVar } else { Token::Var },
                        Span::new(start, end),
                        &self.source[start..end]
                    ))
                }  
            }
            Some(x) if x.is_ascii_digit() => {

                while let Some(x) = self.next_satisfy(
                    |ch| ch.is_ascii_digit()) {}

                if self.peek_char() != Some('.') {
                    let end = self.pos;
                    return Ok((
                        Token::Int,
                        Span::new(start, end),
                        &self.source[start..end]
                    ));
                } else {
                    self.next_char();
                }

                if let Some(x) = self.next_satisfy(
                    |ch| ch.is_ascii_digit()) {
                    // nothing
                } else {
                    return Err("Real number without fractional part!".to_string())
                }

                while let Some(x) = self.next_satisfy(
                    |ch| ch.is_ascii_digit()) {}

                let end = self.pos;
                Ok((
                    Token::Real,
                    Span::new(start, end),
                    &self.source[start..end]
                ))
            }
            None => {
                if self.is_end {
                    return Err("file is ended!".to_string());
                } else {
                    self.is_end = true;
                }
                let end = self.pos;
                Ok((
                    Token::EndOfFile,
                    Span::new(start, end),
                    &self.source[start..end]
                ))
            }
            _ => {
                Err("lexing failed!".to_string())
            }
        }
    }
    
}


#[test]
fn lexer_test() {
    let string = "fn f x => (f 42 (true) 3.1415)";
    //let string = "fn f g x => (f x) g x";
    let mut lex = Lexer::new(string);

    while let Ok(tok) = lex.next_token() {
        println!("{:?}", tok);
    }
}