
use std::env::VarError;

use crate::types::*;
use crate::utils::*;
use crate::lexer::Token;
use crate::parser::{Parsable, Parser};
use crate::checker::*;

use super::*;


impl Parsable for Variant {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {

        par.match_next(Token::UpVar)?;
        let cons: Symbol = par.parse()?;

        let args: Vec<Type> = par.parse_many(start_of_type);

        Ok(Box::new(Variant { cons, args }))
    }
}

