use core::slice;
use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

use crate::ast::{Prim, LitType};
use norem_utils::interner::{InternStr, intern};
use norem_utils::position::{Position, Span};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    /// Parens
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    
    // Delims
    Comma,
    Semicolon,
    Bar,
    Wild,
    Equal,
    
    // Fixities
    Infixl,
    Infixr,
    Nonfix,

    // Keywords
    Let,
    In,
    End,
    Val,
    Data,
    Type,
    Case,
    Of,
    Do,
    Return,
    Fn,
    EArrow,
    Arrow,
    BArrow,

    // Literal Values
    Int(i64),
    Real(f64),
    Bool(bool),
    Char(char),
    Str(InternStr),

    // Literal Prims and Types
    Prim(Prim),
    LitType(LitType),
    
    // Symbols
    Var(InternStr),
    UpVar(InternStr),
    Opr(InternStr),

    // Special
    StartOfFile,
    EndOfFile,
    BadToken(&'static str),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Token::Int(x) => write!(f, "Int({x})"),
            Token::Real(x) => write!(f, "Real({x})"),
            Token::Bool(x) => write!(f, "Bool({x})"),
            Token::Char(x) => write!(f, "Char({x})"),
            Token::Str(x) => write!(f, "Str({x})"),
            Token::Prim(x) => write!(f, "Prim({x})"),
            Token::LitType(x) => write!(f, "LitType({x})"),
            Token::Var(x) => write!(f, "Var({x})"),
            Token::UpVar(x) => write!(f, "UpVar({x})"),
            Token::Opr(x) => write!(f, "Opr({x})"),
            Token::BadToken(msg) => write!(f, "BadToken:\n {msg}"),
            other => write!(f, "{:?}", other),
        }
    }
}

pub fn as_keyword(str: &str) -> Option<Token> {
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
        "do" => Token::Do,
        "return" => Token::Return,
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "Int" => Token::LitType(LitType::Int),
        "Real" => Token::LitType(LitType::Real),
        "Bool" => Token::LitType(LitType::Bool),
        "Char" => Token::LitType(LitType::Char),
        _ => { return None; }
    };
    Some(tok)
}

pub fn as_primitive(str: &str) -> Option<Token> {
    let tok = match str {
        "_iadd" => Token::Prim(Prim::IAdd),
        "_isub" => Token::Prim(Prim::ISub),
        "_imul" => Token::Prim(Prim::IMul),
        "_idiv" => Token::Prim(Prim::IDiv),
        "_ineg" => Token::Prim(Prim::INeg),
        "_bnot" => Token::Prim(Prim::BNot),
        _ => { return None; }
    };
    Some(tok)
}

pub fn is_opr_char(ch: char) -> bool {
    match ch {
        ':' | '!' | '#' | '$' | '%' |
        '&' | '*' | '+' | '.' | '/' |
        '<' | '=' | '>' | '?' | '@' |
        '\\' | '^' | '|' | '-' | '~' => true,
        _ => false,
    }
}

pub struct Lexer<'src> {
    source: &'src str,
    stream: Peekable<Chars<'src>>,
    is_end: bool,
    row: usize,
    col: usize,
    abs: usize,
}

impl<'src> Lexer<'src> {
    pub fn new(s: &'src str) -> Self {
        Lexer {
            source: s,
            stream: s.chars().peekable(),
            is_end: false,
            row: 0,
            col: 0,
            abs: 0,
        }
    }

    fn position(&self) -> Position {
        Position::new(self.col, self.row, self.abs)
    }

    fn get_slice(&self, start: Position, end: Position) -> &'src str {
        &self.source[start.abs .. end.abs]
    }

    fn peek_char(&mut self) -> Option<char> {
        let ch = self.stream.peek()?;
        Some(*ch)
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.stream.next()?;
        self.abs += 1;
        if ch == '\n' {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    fn next_satisfy(&mut self, f: fn(char) -> bool) -> Option<char> {
        let ch = self.peek_char()?;
        if f(ch) {
            self.next_char()
        } else {
            None
        }
    }

    fn skip_satisfy(&mut self, f: fn(char) -> bool) {
        while let Some(ch) = self.peek_char() {
            if f(ch) {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn comment_block(&mut self) -> Result<(), ()> {
        // an automat of 3 states and a variable "level"
        let mut level = 1;
        let mut state = 0;

        while level > 0 {
            let ch = self.next_char().ok_or(())?;

            match state {
                0 => match ch {
                    '*' => state = 1,
                    '/' => state = 2,
                    _ => {}
                },
                1 => match ch {
                    '*' => state = 1,
                    '/' => {
                        level -= 1;
                        state = 0
                    }
                    _ => state = 0,
                },
                2 => match ch {
                    '*' => {
                        level += 1;
                        state = 0
                    }
                    '/' => state = 2,
                    _ => state = 0,
                },
                _ => {
                    unreachable!()
                }
            }
        }

        Ok(())
    }

    pub fn lex_opr(&mut self, start: Position) -> (Token, Span) {
        self.skip_satisfy(is_opr_char);
        let end = self.position();
        let span = Span::new(start, end);
        let slice = self.get_slice(start, end);
        


        let sym = intern(slice);
        (Token::Opr(sym), span)
    }

    pub fn next_token(&mut self) -> (Token, Span) {
        // skip all whitespaces
        self.skip_satisfy(|ch| ch.is_ascii_whitespace());

        let start = self.position();

        match self.next_char() {
            Some('/') => match self.peek_char() {
                Some('/') => {
                    self.next_char();
                    self.skip_satisfy(|ch| ch != '\n');
                    self.next_token()
                }
                Some('*') => {
                    self.next_char();
                    if self.comment_block().is_ok() {
                        self.next_token()
                    } else {
                        let end = self.position();
                        let span = Span::new(end, end);
                        (Token::BadToken("comment block not closed!"), span)
                    }
                }
                _ => {
                    self.lex_opr(start)
                }
            },
            Some('_') => match self.peek_char() {
                Some(ch) if ch.is_alphabetic() => {
                    self.next_char();
                    self.skip_satisfy(|ch| ch.is_alphabetic());
                    
                    let end = self.position();
                    let span = Span::new(start,end);
                    let slice = self.get_slice(start, end);

                    if let Some(tok) = as_primitive(slice) {
                        (tok, span)
                    } else {
                        (Token::BadToken("no such a primitive!"), span)
                    }
                }
                _ => {
                    let end = self.position();
                    (Token::Wild, Span::new(start, end))
                }
            },
            Some('=') => match self.peek_char() {
                Some('>') => {
                    self.next_char();
                    let end = self.position();
                    (Token::EArrow, Span::new(start, end))
                }
                _ => {
                    let end = self.position();
                    (Token::Equal, Span::new(start, end))
                }
            },
            Some('-') => match self.peek_char() {
                Some('>') => {
                    self.next_char();
                    let end = self.position();
                    (Token::Arrow, Span::new(start, end))
                }
                _ => {
                    self.lex_opr(start)
                }
            },
            Some('<') => match self.peek_char() {
                Some('-') => {
                    self.next_char();
                    let end = self.position();
                    (Token::BArrow, Span::new(start, end))
                }
                _ => {
                    self.lex_opr(start)
                }
            },
            Some('(') => {
                let end = self.position();
                (Token::LParen, Span::new(start, end))
            }
            Some(')') => {
                let end = self.position();
                (Token::RParen, Span::new(start, end))
            }
            Some('[') => {
                let end = self.position();
                (Token::LBracket, Span::new(start, end))
            }
            Some(']') => {
                let end = self.position();
                (Token::RBracket, Span::new(start, end))
            }
            Some('{') => {
                let end = self.position();
                (Token::LBrace, Span::new(start, end))
            }
            Some('}') => {
                let end = self.position();
                (Token::RBrace, Span::new(start, end))
            }
            Some(',') => {
                let end = self.position();
                (Token::Comma, Span::new(start, end))
            }
            Some(';') => {
                let end = self.position();
                (Token::Semicolon, Span::new(start, end))
            }
            Some('|') => {
                let end = self.position();
                (Token::Bar, Span::new(start, end))
            }
            Some(ch) if is_opr_char(ch) => {
                self.lex_opr(start)
            }
            Some(ch) if ch.is_ascii_alphabetic() => {
                let upper = ch.is_uppercase();

                self.skip_satisfy(|ch| ch.is_alphanumeric());

                let end = self.position();
                let span = Span::new(start, end);
                let slice = self.get_slice(start, end);

                if let Some(res) = as_keyword(slice) {
                    (res, span)
                } else {
                    let sym = intern(slice);
                    if upper {
                        (Token::UpVar(sym), span)
                    } else {
                        (Token::Var(sym), span)
                    }
                }
            }

            Some(ch) if ch.is_ascii_digit() => {
                self.skip_satisfy(|ch| ch.is_ascii_digit());

                if self.peek_char() != Some('.') {
                    let end = self.position();
                    let span = Span::new(start, end);
                    let slice = self.get_slice(start, end);
                    let val: i64 = slice.parse().unwrap();
                    return (Token::Int(val), span);
                } else {
                    self.next_char();
                }

                // fractional part
                if self.next_satisfy(|ch| ch.is_ascii_digit()).is_some() {
                    // at least one digit
                    self.skip_satisfy(|ch| ch.is_ascii_digit());
                } else {
                    let end = self.position();
                    let span = Span::new(start, end);
                    // ignore all char until a whitespace
                    self.skip_satisfy(|ch| !ch.is_ascii_whitespace());
                    
                    return (Token::BadToken(
                        "fractional part needs at least one digit!"
                    ), span);
                }

                let end = self.position();
                let span = Span::new(start, end);
                let slice = self.get_slice(start, end);
                let val: f64 = slice.parse().unwrap();

                (Token::Real(val), span)
            }
            Some(_) => {
                self.next_char();
                let end = self.position();
                let span = Span::new(start, end);
                // ignore all char until a whitespace
                self.skip_satisfy(|ch| !ch.is_ascii_whitespace());
                (Token::BadToken("unknown character!"), span)
            }
            None => {
                let end = self.position();
                let span = Span::new(start, end);

                if self.is_end {
                    (Token::BadToken("file ended!"), span)
                } else {
                    self.is_end = true;
                    (Token::EndOfFile, span)
                }
            }
        }
    }
}

#[test]
fn lexer_test() {
    //let string = "fn f x => (f 42 (true) 3.1415)";
    //let string = "fn f g x => (f x) g x";

    let string = "
        let
            val x = 42
            type MyInt = Int
            data Color = Red | Blue | Green
        in
            x
        end
    ";

    let mut lex = Lexer::new(string);

    loop {
        let (tok,span) = lex.next_token();
        
        println!("{} {}", tok, span);
        if tok == Token::EndOfFile {
            break;   
        }
    }
}
