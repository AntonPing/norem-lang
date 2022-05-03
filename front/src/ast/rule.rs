use crate::utils::*;
use crate::lexer::Token;
use crate::parser::{Parsable, Parser};
use crate::checker::*;

use super::*;


impl Parsable for Rule {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        par.match_next(Token::Bar)?;

        let pat: Pattern = par.parse()?;

        par.match_next(Token::EArrow)?;
        
        let body: Expr = par.parse()?;

        Ok(Box::new(Rule { pat, body }))
    }
}

