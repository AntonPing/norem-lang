
use crate::types::*;
use crate::utils::*;
use crate::lexer::Token;
use crate::parser::{Parsable, Parser};
use crate::checker::*;

use super::*;


impl Parsable for Variant {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        par.match_next(Token::Data)?;
        par.match_next(Token::UpVar)?;
        let name = par.text(0)?.to_string();

        let mut args = Vec::new();
        while par.match_next(Token::Var).is_ok() {
            let arg = par.text(0)?.to_string();
            args.push(arg);
        }

        par.match_this(Token::Equal)?;

        let body = *Expr::parse(par)?;

        Ok(Box::new(DeclVal { name, args, body }))
    }
}

