use crate::utils::*;
use crate::lexer::Token;
use crate::parser::*;

use crate::ast::*;

impl Parsable for ExprLit {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        match par.next()? {
            Token::Int => {
                let value = par.text(0)?.parse().unwrap();
                Ok(Box::new(ExprLit::Int(value)))
            }
            Token::Real => {
                let value = par.text(0)?.parse().unwrap();
                Ok(Box::new(ExprLit::Real(value)))
            }
            Token::Bool => {
                let value = par.text(0)?.parse().unwrap();
                Ok(Box::new(ExprLit::Bool(value)))
            }
            Token::Char => {
                let value = par.text(0)?.parse().unwrap();
                Ok(Box::new(ExprLit::Char(value)))
            }
            _ => { Err("parsing variable failed!".to_string())}
        }
    }
}