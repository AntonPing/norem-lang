use std::collections::HashMap;
use crate::lexer::*;
use crate::parser::*;
use crate::utils::*;
use crate::ast::*;

impl Parsable for TypeLit {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        par.match_next(Token::LitType)?;

        Ok(Box::new(match par.text(0)? {
            "Int" => { TypeLit::Int }
            "Real" => { TypeLit::Real }
            "Bool" => { TypeLit::Bool }
            "Char" => { TypeLit::Char }
            _ => { panic!("Impossible!"); }
        }))
    }
}