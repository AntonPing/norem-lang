use crate::utils::*;
use crate::lexer::Token;
use crate::parser::{Parsable, Parser};
use crate::checker::*;

use super::*;

pub mod lit;
pub mod var;
pub mod lam;
pub mod app;
pub mod lett;

impl Parsable for Expr {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        match par.peek()? {
            Token::Int | Token::Real | Token::Char | Token::Bool => {
                let res = par.parse::<ExprLit>()?;
                Ok(Box::new(Expr::Lit(res)))
            }
            Token::Var => {
                let res = par.parse::<ExprVar>()?;
                Ok(Box::new(Expr::Var(res)))
            }
            Token::Fn => {
                let res = par.parse::<ExprLam>()?;
                Ok(Box::new(Expr::Lam(res)))
            }
            Token::LParen => {
                par.next()?;
                let res = par.parse::<ExprApp>()?;
                par.match_next(Token::RParen)?;
                Ok(Box::new(Expr::App(res)))
            }
            _ => {
                Err("Can't parse expression!".to_string())
            }
        }
    }
}

/*
impl Expr {
    pub fn expr_start(tok: Token) -> bool {
        match tok {
            Token::Int |
            Token::Real | 
            Token::Char |
            Token::Bool |
            Token::Var |
            Token::Fn |
            Token::LParen => {
                true
            }
            _ => {
                false
            }
        }
    }
}
*/

impl Typable for Expr {
    fn infer(&self, chk: &mut Checker) -> Result<Type,String> {
        match self {
            Expr::Lit(x) => x.infer(chk),
            Expr::Var(x) => x.infer(chk),
            Expr::Lam(x) => x.infer(chk),
            Expr::App(x) => x.infer(chk),
            _ => unimplemented!(),
        }
    }
}
