use std::collections::VecDeque;

use crate::lexer::{Lexer, Token, self};
use crate::utils::*;
use crate::ast::*;

#[macro_export]
macro_rules! catch {
    ( $par:expr, $res:expr, $( $msg:expr ),* ) => {
        {
            if let Some(value) = res {
                value
            } else {
                $(
                    par.error.push($msg);
                )*
                return None;
            }
        }
    };
}


pub trait Parsable {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String>;
}

pub struct Parser<'src> {
    lexer: Lexer<'src>,
    buffer: VecDeque<(Token,Span,&'src str)>,
    is_end: bool,
    //is_err: Option<(Token,Span,&'src str)>,
    error: Vec<String>,
}

impl<'src> Parser<'src> {
    pub fn new(str: &'src str) -> Parser<'src> {
        let mut par = Parser {
            lexer: Lexer::new(str),
            buffer: VecDeque::new(),
            is_end: false,
            error: Vec::new(),
        };
        
        par.buffer.push_back(
            (Token::StartOfFile,Span::zero(),&str[0..0])
        );
        par
    }

    pub fn reload(&mut self, n: usize) -> Result<(),String> {
        while n >= self.buffer.len() {
            let res = self.lexer.next_token()?;
            self.buffer.push_back(res);
        }
        Ok(())
    }

    pub fn token(&mut self, n: usize) -> Result<Token,String> {
        if let Some(res) = self.buffer.get(n) {
            Ok(res.0)
        } else {
            self.reload(n)?;
            self.token(n)
        }
    }

    pub fn span(&mut self, n: usize) -> Result<Span,String> {
        if let Some(res) = self.buffer.get(n) {
            Ok(res.1)
        } else {
            self.reload(n)?;
            self.span(n)
        }
    }

    pub fn text(&mut self, n: usize) -> Result<&'src str,String> {
        if let Some(res) = self.buffer.get(n) {
            Ok(res.2)
        } else {
            self.reload(n)?;
            self.text(n)
        }
    }

    pub fn pass(&mut self, n: usize) -> Result<(),String> {
        assert_ne!(n,0);
        for _ in 0..n {
            if self.buffer.is_empty() {
                self.lexer.next_token()?;
            } else {
                self.buffer.pop_front().unwrap();
            }
        }
        Ok(())
    }

    pub fn next(&mut self) -> Result<Token,String> {
        self.pass(1)?;
        self.token(0)
    }

    pub fn peek(&mut self) -> Result<Token,String> {
        self.token(1)
    }

    pub fn match_this(&mut self, tok: Token) -> Result<Token,String> {
        if let Ok(tok) = self.token(0) {
            Ok(tok)
        } else {
            Err("Can't match this token!".to_string())
        }
    }

    pub fn match_peek(&mut self, tok: Token) -> Result<Token,String> {
        if let Ok(tok) = self.peek() {
            Ok(tok)
        } else {
            Err("Can't match peek token!".to_string())
        }
    }

    pub fn match_next(&mut self, tok: Token) -> Result<Token,String> {
        if let Ok(tok) = self.next() {
            Ok(tok)
        } else {
            Err("Can't match next token!".to_string())
        }
    }

    pub fn eof(&mut self) -> Result<(),String> {
        let tok = self.next()?;
        if tok == Token::EndOfFile {
            Ok(())
        } else {
            Err("expected EndOfFile!".to_string())
        }
    }

    pub fn parse_many<T: Parsable>(&mut self, first: &Vec<Token>) -> Result<Vec<T>,String> {
        let mut vec: Vec<T> = Vec::new();

        while let Ok(tok) = self.peek() {
            if first.contains(&tok) {
                vec.push(self.parse()?);
            } else {
                break;
            }
        }
        
        Ok(vec)
    }

    pub fn parse_many1<T: Parsable>(&mut self, first: &Vec<Token>) -> Result<Vec<T>,String> {
        let vec = self.parse_many(first)?;

        if vec.len() >= 1 {
            Ok(vec)
        } else {
            Err("Except at least one!".to_string())
        }
    }

    pub fn parse_sepby<T: Parsable>(&mut self, delim: Token) -> Result<Vec<T>,String> {
        let mut vec: Vec<T> = Vec::new();

        if let Ok(first) = self.parse() {
            vec.push(first);
        } else {
            return Ok(vec);
        }

        while let Ok(tok) = self.peek() {
            if tok == delim {
                self.next();
                vec.push(*T::parse(self)?);
            } else {
                break;
            }
        }
        
        Ok(vec)
    }

    pub fn parse_sepby1<T: Parsable>(&mut self, delim: Token) -> Result<Vec<T>,String> {
        let vec = self.parse_sepby(delim)?;

        if vec.len() >= 1 {
            Ok(vec)
        } else {
            Err("Except at least one!".to_string())
        }
    }

    pub fn parse<T: Parsable>(&mut self) -> Result<T,String> {
        let res = T::parse(self)?;
        Ok(*res)
    }

}

#[test]
fn parser_test() {
    let text = "fn f g x => (f x) g x";
    let mut par = Parser::new(text);
    let res = par.parse::<Expr>().unwrap();
    println!("{}", res);

    let text = "
        let
            val x = 42
            type MyInt = Int
            data Color = Red | Blue | Green
        in
            case c of
            | Red => 1
            | Blue => 2
            | Green => 3
            end
        end
    ";
    let mut lex = Lexer::new(text);
    while let Ok((tok,span,txt)) = lex.next_token() {
        println!("{:?}, {:?}, {}",tok,span,txt);
    }
    let mut par = Parser::new(text);
    let res = par.parse::<Expr>().unwrap();
    println!("{}", res);
}