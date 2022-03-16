use std::{iter::Peekable, str::Chars, error::Error};
use std::char;
use std::char::*;

use crate::utils::*;

#[test]
fn test_lexer() {
    println!("Hello, world!");
}

pub enum Token {
    LParen, RParen, // ( )
    LBracket, RBracket, // [ ]
    LBrace, RBrace, // { }
    Comma, Semicolon, // , ;
    Let,In,End, // let in end
    Fn, EArrow, MArrow, // fn => ->
    Add,Sub,Mul,Div, // + - * /
    Equal,
    Int(i64),
    Real(f64),
    Bool(bool),
    String(String),
    Var(String),
}

pub struct Lexer<'src> {
    pub source: &'src str,
    pub input: Peekable<Chars<'src>>,
    pub current: Position,
}

impl<'src> Lexer<'src> {
    pub fn new(string: &'src String) -> Lexer<'src> {
        Lexer {
            source: string.as_str(),
            input: string.chars().peekable(),
            current: Position {
                line: 0,
                col: 0,
                abs: 0,
            },
        }
    }

    /// Peek at the next [`char`] in the input stream
    #[inline]
    fn peek(&mut self) -> Option<char> {
        self.input.peek().copied()
    }

    /// Consume the next [`char`] and advance internal source position
    #[inline]
    fn next(&mut self) -> Option<char> {
        match self.input.next() {
            Some('\n') => {
                self.current.line += 1;
                self.current.col = 0;
                self.current.abs += 1;
                Some('\n')
            }
            Some(ch) => {
                self.current.col += 1;
                self.current.abs += 1;
                Some(ch)
            }
            None => None,
        }
    }

    #[inline]
    fn spanned<T>(&mut self, f: fn(&mut Self) -> Option<T>)
        -> Option<Spanned<T>> {
        let start = self.current;
        if let Some(res) = f(self) {
            let end = self.current;
            Some(Spanned::new(res,Span::new(start,end)))
        } else {
            self.current = start;
            None;
        }
    }

    /*
    #[inline]
    fn start(&self) -> Position {
        self.current
    }

    #[inline]
    fn success(&self, start: Position) -> Span {
        Span::new(start,self.current)
    }

    #[inline]
    fn failed<T>(&mut self, start: Position) -> Option<T> {
        self.current = start;
        None
    }

    #[inline]
    fn get_slice(&self, span: Span) -> &'src str {
        &self.source[span.start.abs .. span.end.abs]
    }
    */

    #[inline]
    fn satisfy<F: Fn(char) -> bool>(&mut self, pred: F) -> Option<()> {
        let ch = self.peek()?;
        if pred(ch) {
            self.next().unwrap();
            Some(())
        } else {
            None
        }
    }

    /// Consume characters from the input stream while pred(peek()) is true,
    /// collecting the characters into a string.
    #[inline]
    fn while_satisfy<F: Fn(char) -> bool>(&mut self, pred: F) {
        while let Some(n) = self.peek() {
            if pred(n) {
                match self.next() {
                    Some(_) => continue,
                    None => break,
                }
            } else {
                break;
            }
        }
    }

    /// Eat whitespace
    #[inline]
    fn eat_space(&mut self) {
        self.while_satisfy(char::is_whitespace);
    }

    fn tokenize(&mut self) -> Option<Spanned<Token>> {
        
        let start = self.start();
        let tok;
        
        match self.peek()? {
            '\n' => { self.next(); }
            ';' => { self.next(); tok = Token::Semicolon; }
            ',' => { self.next(); tok = Token::Comma; }
            '(' => { self.next(); tok = Token::LParen; }
            ')' => { self.next(); tok = Token::RParen; }
            '[' => { self.next(); tok = Token::LBracket; }
            ']' => { self.next(); tok = Token::RBracket; }
            '{' => { self.next(); tok = Token::LBrace; }
            '}' => { self.next(); tok = Token::RBrace; }
            '-' => {
                self.next();
                if self.next()? == '>' {
                    tok = Token::MArrow;
                } else {
                    tok = Token::Sub;
                }
            }
            '=' => {
                if self.next()? == '>' {
                    tok = Token::EArrow;
                } else {
                    tok = Token::Equal;
                }
            }
            x if char::is_numeric(x) => {
                return self.read_nat();
            }
            x if char::is_alphabetic(x) => {
                return self.read_keyword();
            }
            _ => { return None }
        }

        let span = self.success(start);

        Some(Spanned::new(tok,span))
    }

    /// Lex a natural number
    fn read_nat(&mut self) -> Option<Spanned<Token>> {
        // Since we peeked at least one numeric char, we should always
        // have a string containing at least 1 single digit, as such
        // it is safe to call unwrap() on str::parse<u32>
        self.spanned(|lex| {
            let mut ch = lex.next()?;
            let n = 0;
            while char::is_numeric(ch) {
                n *= 10;
                n += ch - '0';
            }
            if char::is_whitespace(ch) {
                Some(n as i64)
            } else if ch == '.' {
                ch = lex.next()?;
                let mut x = 0;
                while char::is_numeric(ch) {
                    x *= 10;
                    x += ch - '0';
                }
                if char::is_whitespace(ch) {
                    Some(n as i64)
                }


            } else {
                None
            }
        })
    }

    fn read_keyword(&mut self) -> Option<Spanned<Token>> {
        let start = self.start();
        self.while_satisfy(char::is_alphabetic);
        let span = self.success(start);
        self.satisfy(char::is_whitespace)?;
        let str = self.get_slice(span);
        let tok;
        match str {
            "let" => { tok = Token::Let; }
            "in" => { tok = Token::In; }
            "end" => { tok = Token::End; }
            _ => { return self.failed(start) }
        }
        let span = self.success(start);
        Some(Spanned::new(tok,span))
    }

    fn read_ident(&mut self) -> Option<Spanned<Token>> {
        unimplemented!()
    }

    fn string_lit(&mut self) -> Option<Spanned<Token>> {
        let start = self.start();
        self.satisfy(|c| c != '"');
        self.satisfy_while(|c| c != '"');
        let span = self.success(start);
        Some(Spanned::new(
            Token::Const(Const::String(self.interner.intern(s))),
            span,
        ))
    }

    fn char_lit(&mut self) -> Option<Spanned<Token>> {
        let sp = self.current;
        match self.consume()? {
            '"' => {}
            c => return Some(Spanned::new(Token::Invalid(c), Span::new(sp, self.current))),
        }
        // TODO: return invalid on fail
        let ch = self.consume()?;
        match self.consume() {
            Some('"') => Some(Spanned::new(
                Token::Const(Const::Char(ch)),
                Span::new(sp, self.current),
            )),
            Some(c) => Some(Spanned::new(Token::Invalid(c), Span::new(sp, self.current))),
            None => Some(Spanned::new(
                Token::MissingDelimiter('\''),
                Span::new(sp, self.current),
            )),
        }
    }

    fn comment(&mut self) -> Option<Spanned<Token>> {
        let (_, sp) = self.consume_while(|ch| ch != '*');
        self.consume()?;
        match self.peek() {
            Some(')') => {
                self.consume();
                self.lex()
            }
            Some(_) => self.comment(),
            None => Some(Spanned::new(Token::MissingDelimiter('*'), sp)),
        }
    }

    pub fn lex(&mut self) -> Option<Spanned<Token>> {
        self.consume_delimiter();
        let sp = self.current;

        macro_rules! eat {
            ($kind:expr) => {{
                self.consume().unwrap();
                Some(Spanned::new($kind, Span::new(sp, self.current)))
            }};
        }

        match self.peek()? {
            ';' => eat!(Token::Semi),
            ',' => eat!(Token::Comma),
            '\'' => eat!(Token::Apostrophe),
            '_' => eat!(Token::Wildcard),
            '(' => {
                let alt = eat!(Token::LParen);
                if let Some('*') = self.peek() {
                    self.comment()
                } else {
                    alt
                }
            }
            ')' => eat!(Token::RParen),
            '{' => eat!(Token::LBrace),
            '}' => eat!(Token::RBrace),
            '[' => eat!(Token::LBracket),
            ']' => eat!(Token::RBracket),
            'λ' => eat!(Token::Fn),
            '∀' => eat!(Token::Forall),
            '#' => {
                self.consume();
                match self.peek() {
                    Some('"') => self.char_lit(),
                    Some(_) => Some(Spanned::new(Token::Selector, Span::new(sp, self.current))),
                    _ => None,
                }
            }
            '"' => self.string_lit(),
            x if x.is_ascii_alphabetic() => Some(self.keyword()),
            x if x.is_numeric() => self.number(),
            x if Self::valid_symbolic(x) => Some(self.symbolic()),
            ch => {
                self.consume();
                Some(Spanned::new(
                    Token::Invalid(ch),
                    Span::new(self.current, self.current),
                ))
            }
        }
    }
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Spanned<Token>;
    fn next(&mut self) -> Option<Self::Item> {
        self.lex()
    }
}


#[cfg(test)]
fn keyword_test() {
    let lex = Lexer::new(&"fn(1234]let->".to_string());

    let tks = lex.collect::<Vec<Spanned<Token>>>();
    assert_eq!(
        tks.into_iter().map(|s| s.data),
        vec![
            Token::Fn,
            Token::LParen,
            Token::Int(1234),
            Token::Let,
            Token::MArrow,
        ]
    );
}

