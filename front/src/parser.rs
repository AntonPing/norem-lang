use std::collections::VecDeque;

use crate::lexer::{Lexer, Token, self};
use crate::utils::*;
use crate::ast::*;

pub trait Parsable {
    fn parse<'src>(par: &mut Parser<'src>) -> Result<Box<Self>,String>;
}

pub struct Parser<'src> {
    lexer: Lexer<'src>,    
    buffer: VecDeque<(Token,Span,&'src str)>,
    is_end: bool,
    //is_err: Option<(Token,Span,&'src str)>,
}

impl<'src> Parser<'src> {
    pub fn new(str: &'src str) -> Parser<'src> {
        let mut par = Parser {
            lexer: Lexer::new(str),
            buffer: VecDeque::new(),
            is_end: false,
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
}

#[test]
fn parser_test() {
    let text = "fn f g x => (f x) g x";
    let mut par = Parser::new(text);
    let res = Expr::parse(&mut par);
    println!("{:?}", res);

}