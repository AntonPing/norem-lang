use crate::utils::*;
use crate::lexer::Token;
use crate::parser::{Parsable, Parser};
use crate::checker::*;

use super::*;

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}", self.cons)?;
        for arg in &self.args {
            write!(f," {}", arg)?;
        }
        Ok(())
    }
}

impl Parsable for Variant {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {

        par.match_peek(Token::UpVar)?;
        let cons = par.parse::<Symbol>()?;

        let args = par.parse_many::<Type>(&vec![
            Token::LitType, Token::UpVar, Token::LParen
        ])?;

        Ok(Box::new(Variant { cons, args }))
    }
}

