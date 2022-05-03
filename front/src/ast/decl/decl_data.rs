use crate::utils::*;
use crate::lexer::Token;
use crate::parser::{Parsable, Parser};
use crate::checker::*;

use super::*;


impl Parsable for DeclData {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        par.match_next(Token::Data)?;

        par.match_peek(Token::UpVar)?;
        let name = par.parse::<Symbol>()?;

        let args = par.parse_many::<Symbol>(
            &vec![Token::Var, Token::UpVar]
        )?;

        par.match_next(Token::Equal)?;

        let vars = par.parse_sepby::<Variant>(Token::Bar)?;

        Ok(Box::new(DeclData { name, args, vars }))
    }
}

