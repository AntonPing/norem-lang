use std::{iter::Peekable, str::Chars, error::Error};
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

    /*
    #[inline]
    fn next_slice(&mut self) -> Option<Span> {
        self.eat_space();
        let start = self.start();
        match self.next()? {
            '(' | ')' | '[' | ']' | '{' | '}' => {
                return Some(self.end(start));
            }
            '-' | '=' => {
                if self.peek()? == '>' {
                    self.next().unwrap();
                } 
                return Some(self.end(start));
            }
            ch if ch.is_ascii_alphabetic() => {
                while let Some(ch) = self.next()

            }

            _ => { return None }
        }
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

    fn read_punct(&mut self) -> Option<Spanned<Token>> {
        
        let start = self.start();
        let tok;
        
        match self.peek()? {
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
        let start = self.start();
        self.while_satisfy(char::is_numeric);
        let span = self.success(start);
        let str = self.get_slice(span);
        let n = str.parse::<usize>().ok()?;
        Some(Spanned::new(Token::Int(n as i64), span))
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


    #[inline]
    fn valid_symbolic(c: char) -> bool {
        match c {
            '!' | '%' | '&' | '$' | '#' | '+' | '-' | '/' | ':' | '<' | '=' | '>' | '?' | '@'
            | '~' | '`' | '^' | '|' | '*' | '\\' | '.' => true,
            _ => false,
        }
    }
    
    #[inline]
    fn valid_id_char(c: char) -> bool {
        match c {
            x if x.is_alphanumeric() => true,
            '_' | '\'' => true,
            _ => false,
        }
    }

    /*
    /// Lex a reserved keyword or identifier
    fn keyword(&mut self) -> Spanned<Token> {
        let (word, sp) = self.consume_while(Self::valid_id_char);
        let word = self.interner.intern(word);
        let kind = match word {
            S_ABSTYPE => Token::Abstype,
            S_AND => Token::And,
            S_ANDALSO => Token::Andalso,
            S_AS => Token::As,
            S_CASE => Token::Case,
            S_DATATYPE => Token::Datatype,
            S_DO => Token::Do,
            S_ELSE => Token::Else,
            S_END => Token::End,
            S_EXCEPTION => Token::Exception,
            S_FN => Token::Fn,
            S_FUN => Token::Fun,
            S_FUNCTOR => Token::Functor,
            S_HANDLE => Token::Handle,
            S_IF => Token::If,
            S_IN => Token::In,
            S_INFIX => Token::Infix,
            S_INFIXR => Token::Infixr,
            S_LET => Token::Let,
            S_LOCAL => Token::Local,
            S_NONFIX => Token::Nonfix,
            S_OF => Token::Of,
            S_OP => Token::Op,
            S_OPEN => Token::Open,
            S_ORELSE => Token::Orelse,
            S_PRIM => Token::Primitive,
            S_RAISE => Token::Raise,
            S_REC => Token::Rec,
            S_THEN => Token::Then,
            S_TYPE => Token::Type,
            S_VAL => Token::Val,
            S_WITH => Token::With,
            S_WITHTYPE => Token::Withtype,
            S_WHILE => Token::While,
            S_SIG => Token::Sig,
            S_SIGNATURE => Token::Signature,
            S_STRUCT => Token::Struct,
            S_STRUCTURE => Token::Structure,
            _ => Token::Id(word),
        };
        Spanned::new(kind, sp)
    }

    fn string_lit(&mut self) -> Option<Spanned<Token>> {
        self.consume()?;
        let (s, sp) = self.consume_while(|c| c != '"');
        if self.consume().is_none() {
            return Some(Spanned::new(Token::MissingDelimiter('"'), sp));
        }
        Some(Spanned::new(
            Token::Const(Const::String(self.interner.intern(s))),
            sp,
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

impl<'s, 'sym> Iterator for Lexer<'s, 'sym> {
    type Item = Spanned<Token>;
    fn next(&mut self) -> Option<Self::Item> {
        self.lex()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn _span(a: u32, b: u32) -> Span {
        Span::new(Location::new(0, a as u16, a), Location::new(0, b as u16, b))
    }

    #[test]
    fn keywords() {
        let mut int = Interner::with_capacity(64);
        let lex = Lexer::new("andalso and fn ...".chars(), &mut int);

        let tks = lex.collect::<Vec<Spanned<Token>>>();
        assert_eq!(
            tks,
            vec![
                Spanned::new(Token::Andalso, _span(0, 7)),
                Spanned::new(Token::And, _span(8, 11)),
                Spanned::new(Token::Fn, _span(12, 14)),
                Spanned::new(Token::Flex, _span(15, 18)),
            ]
        );
        assert_eq!(
            tks.into_iter().map(|s| s.span).collect::<Vec<_>>(),
            vec![_span(0, 7), _span(8, 11), _span(12, 14), _span(15, 18),]
        )
    }
    */
}

