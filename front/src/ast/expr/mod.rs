
use crate::types::*;
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
        match par.token(1)? { // peek one
            Token::Int |
            Token::Real | 
            Token::Char |
            Token::Bool => {
                let res = ExprLit::parse(par)?;
                Ok(Box::new(Expr::Lit(*res)))
            }
            Token::Var => {
                let res = ExprVar::parse(par)?;
                Ok(Box::new(Expr::Var(*res)))
            }
            Token::Fn => {
                let res = ExprLam::parse(par)?;
                Ok(Box::new(Expr::Lam(*res)))
            }
            Token::LParen => {
                par.next()?;
                let res = ExprApp::parse(par)?;
                if let Token::RParen = par.next()? {
                    Ok(Box::new(Expr::App(*res)))
                } else {
                    Err("paranthesis is not closed!".to_string())
                }
            }
            _ => {
                Err("Can't parse expression!".to_string())
            }
        }
    }
}

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
