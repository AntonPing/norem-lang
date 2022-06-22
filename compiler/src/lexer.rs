use std::fmt;
use std::fmt::write;
use std::iter::Peekable;
use std::str::Chars;

use crate::symbol::*;
use crate::utils::*;

#[derive(Debug, PartialEq, Clone, Copy)]
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

    Int(i64),
    Real(f64),
    Bool(bool),
    Char(char),
    Str(Symbol),

    LitType(Symbol),
    Prim(Symbol),
    Var(Symbol),
    UpVar(Symbol),

    StartOfFile,
    EndOfFile,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            Token::Int(x) => write!(f, "Int({x})"),
            Token::Real(x) => write!(f, "Real({x})"),
            Token::Bool(x) => write!(f, "Bool({x})"),
            Token::Char(x) => write!(f, "Char({x})"),
            Token::Str(x) => write!(f, "Str({x})"),
            Token::LitType(x) => write!(f, "LitType({x})"),
            Token::Prim(x) => write!(f, "Opr({x})"),
            Token::Var(x) => write!(f, "Var({x})"),
            Token::UpVar(x) => write!(f, "UpVar({x})"),
            other => write!(f, "{other:?}"),
        }
    }
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
        "true" => Token::Bool(true),
        "false" => Token::Bool(false),
        "Int" => Token::LitType(S_TY_INT),
        "Real" => Token::LitType(S_TY_REAL),
        "Bool" => Token::LitType(S_TY_BOOL),
        "Char" => Token::LitType(S_TY_CHAR),
        _ => {
            return None;
        }
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
    pub fn new(s: &'src str) -> Self {
        Lexer {
            source: s,
            stream: s.chars().peekable(),
            is_end: false,
            pos: 0,
            row: 0,
            col: 0,
        }
    }

    fn position(&self) -> Position {
        Position::new(self.pos, self.row, self.col)
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
        } else {
            self.col += 1;
        }
        Some(ch)
    }

    fn peek_satisfy(&mut self, f: fn(char) -> bool) -> Option<char> {
        let ch = self.peek_char()?;
        if f(ch) {
            Some(ch)
        } else {
            None
        }
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

    fn comment_block(&mut self) -> Result<(), String> {
        let mut level = 1;

        // an automat of 3 states
        let mut state = 0;

        while level > 0 {
            let ch = self
                .next_char()
                .ok_or("comment block not closed!".to_string())?;

            match state {
                0 => match ch {
                    '*' => state = 1,
                    '/' => state = 2,
                    _ => {}
                },
                1 => match ch {
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
                    _ => state = 0,
                },
                _ => {
                    unreachable!()
                }
            }
        }

        Ok(())
    }

    pub fn next_token(&mut self) -> Result<(Token, Span), String> {
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
                    self.comment_block()?;
                    self.next_token()
                }
                Some(_) => {
                    let end = self.position();
                    Ok((Token::Prim(S_IDIV), Span::new(start, end)))
                }
                None => Err("lexing failed!".to_string()),
            },
            Some('=') => match self.peek_char() {
                Some('>') => {
                    self.next_char();
                    let end = self.position();
                    Ok((Token::EArrow, Span::new(start, end)))
                }
                _ => {
                    let end = self.position();
                    Ok((Token::Equal, Span::new(start, end)))
                }
            },
            Some('-') => match self.peek_char() {
                Some('>') => {
                    self.next_char();
                    let end = self.position();
                    Ok((Token::Arrow, Span::new(start, end)))
                }
                _ => {
                    let end = self.position();
                    Ok((Token::Prim(S_ISUB), Span::new(start, end)))
                }
            },
            Some('+') => {
                let end = self.position();
                Ok((Token::Prim(S_IADD), Span::new(start, end)))
            }
            Some('*') => {
                let end = self.position();
                Ok((Token::Prim(S_IMUL), Span::new(start, end)))
            }
            Some('(') => {
                let end = self.position();
                Ok((Token::LParen, Span::new(start, end)))
            }
            Some(')') => {
                let end = self.position();
                Ok((Token::RParen, Span::new(start, end)))
            }
            Some('[') => {
                let end = self.position();
                Ok((Token::LBracket, Span::new(start, end)))
            }
            Some(']') => {
                let end = self.position();
                Ok((Token::RBracket, Span::new(start, end)))
            }
            Some('{') => {
                let end = self.position();
                Ok((Token::LBrace, Span::new(start, end)))
            }
            Some('}') => {
                let end = self.position();
                Ok((Token::RBrace, Span::new(start, end)))
            }
            Some(',') => {
                let end = self.position();
                Ok((Token::RBrace, Span::new(start, end)))
            }
            Some(';') => {
                let end = self.position();
                Ok((Token::Semicolon, Span::new(start, end)))
            }
            Some('|') => {
                let end = self.position();
                Ok((Token::Bar, Span::new(start, end)))
            }
            Some('_') => {
                let end = self.position();
                Ok((Token::Wild, Span::new(start, end)))
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

                let end = self.position();
                let slice = &self.source[start.pos..end.pos];

                let tok;
                if let Some(res) = is_reserved(slice) {
                    tok = res;
                } else {
                    let sym = newvar(slice);
                    if upper {
                        tok = Token::UpVar(sym);
                    } else {
                        tok = Token::Var(sym);
                    }
                }

                Ok((tok, Span::new(start, end)))
            }

            Some(x) if x.is_ascii_digit() => {
                self.skip_satisfy(|ch| ch.is_ascii_digit());

                if self.peek_char() != Some('.') {
                    let end = self.position();
                    let val: i64 = self.source[start.pos..end.pos].parse().unwrap();
                    return Ok((Token::Int(val), Span::new(start, end)));
                } else {
                    self.next_char();
                }

                // fractional part
                if let Some(_) = self.next_satisfy(|ch| ch.is_ascii_digit()) {
                    // at least one digit
                    self.skip_satisfy(|ch| ch.is_ascii_digit());
                } else {
                    return Err("Real number without fractional part!".to_string());
                }

                let end = self.position();
                let val: f64 = self.source[start.pos..end.pos].parse().unwrap();
                Ok((Token::Real(val), Span::new(start, end)))
            }
            None => {
                if self.is_end {
                    return Err("file is ended!".to_string());
                } else {
                    self.is_end = true;
                }
                let end = self.position();
                Ok((Token::EndOfFile, Span::new(start, end)))
            }
            Some(ch) => Err(format!("lexing failed at {ch}!")),
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

    while let Ok(tok) = lex.next_token() {
        println!("{} {}", tok.0, tok.1);
    }
}
