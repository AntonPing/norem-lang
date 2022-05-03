
use crate::types::*;
use crate::utils::*;
use crate::lexer::Token;
use crate::parser::{Parsable, Parser};
use crate::checker::*;

use super::*;


impl Parsable for DeclData {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        par.match_next(Token::Data)?;

        par.match_next(Token::UpVar)?;
        let name = par.parse::<Symbol>()?;

        let args = par.parse_many::<Symbol>(|p|
            p.peek() == Ok(Token::Var));

        par.match_next(Token::Equal)?;

        let vars: Vec<Variant> = par.parse_sepby(|par| {
            par.match_peek()
        });

        Ok(Box::new(DeclData { name, args, vars }))
    }
}

