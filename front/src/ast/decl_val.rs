
use crate::types::*;
use crate::utils::*;
use crate::lexer::Token;
use crate::parser::{Parsable, Parser};
use crate::checker::*;

use super::*;


impl Parsable for DeclVal {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        par.match_next(Token::Val)?;

        par.match_peek(Token::Var)?;
        let name = par.parse::<Symbol>()?;

        let args = par.many::<Symbol>(|p|
            p.peek() == Ok(Token::Var));

        par.match_next(Token::Equal)?;

        let body = par.parse::<Expr>()?;

        Ok(Box::new(DeclVal { name, args, body }))
    }
}

