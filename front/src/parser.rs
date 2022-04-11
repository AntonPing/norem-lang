use std::collections::VecDeque;

use crate::lexer::{Lexer, Token, self};
use crate::utils::*;

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
        Parser {
            lexer: Lexer::new(str),
            buffer: VecDeque::new(),
            is_end: false,
        }
    }

    pub fn free_buffer(&mut self, n: usize) -> Result<(),String> {
        while n >= self.buffer.len() {
            let res = self.lexer.next_token()?;
            self.buffer.push_back(res);
        }
        Ok(())
    }

    pub fn load_buffer(&mut self, n: usize) -> Result<(),String> {
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
            self.load_buffer(n)?;
            self.token(n)
        }
    }

    pub fn span(&mut self, n: usize) -> Result<Span,String> {
        if let Some(res) = self.buffer.get(n) {
            Ok(res.1)
        } else {
            self.load_buffer(n)?;
            self.span(n)
        }
    }

    pub fn text(&mut self, n: usize) -> Result<&'src str,String> {
        if let Some(res) = self.buffer.get(n) {
            Ok(res.2)
        } else {
            self.load_buffer(n)?;
            self.text(n)
        }
    }

    pub fn next(&mut self) -> Result<(Token,Span,&'src str),String> {
        if self.buffer.is_empty() {
            self.lexer.next_token()
        } else {
            Ok(self.buffer.pop_front().unwrap())
        }
    }

    pub fn peek(&mut self) -> Result<(Token,Span,&'src str),String> {
        let res = self.lexer.next_token()?;
        self.buffer.push_back(res);
        Ok(res)
    }

}