use crate::utils::*;
use crate::lexer::Token;
use crate::parser::{Parsable, Parser};
use crate::checker::*;

use super::*;
pub mod decl_val;
pub mod decl_data;
pub mod decl_type;


impl Parsable for Decl {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        match par.peek()? {
            Token::Val => {
                let res = par.parse::<DeclVal>()?;
                Ok(Box::new(Decl::Val(res)))
            }
            Token::Data => {
                let res = par.parse::<DeclData>()?;
                Ok(Box::new(Decl::Data(res)))
            }
            Token::Type => {
                let res = par.parse::<DeclType>()?;
                Ok(Box::new(Decl::Type(res)))
            }
            _ => {
                Err("Can't parse expression!".to_string())
            }
        }
    }
}