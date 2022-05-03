use crate::utils::*;
use crate::lexer::Token;
use crate::parser::{Parsable, Parser};
use crate::checker::*;

use super::*;


impl Parsable for Variant {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {

        par.match_next(Token::UpVar)?;
        let cons = par.parse::<Symbol>()?;

        let args: Vec<Type> = par.parse_many(&vec![
            Token::LitType, Token::UpVar, Token::LParen
        ])?;

        Ok(Box::new(Variant { cons, args }))
    }
}

