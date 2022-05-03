use crate::utils::*;
use crate::lexer::Token;
use crate::parser::{Parsable, Parser};
use crate::checker::*;

use super::*;


impl Parsable for DeclType {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        par.match_next(Token::Type)?;

        par.match_peek(Token::UpVar)?;
        let name = par.parse::<Symbol>()?;

        let args = par.parse_many::<Symbol>(
            &vec![Token::Var]
        )?;

        par.match_next(Token::Equal)?;

        let typ = par.parse::<Type>()?;

        Ok(Box::new(DeclType { name, args, typ }))
    }
}

