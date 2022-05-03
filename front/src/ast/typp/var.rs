use std::collections::HashMap;
use crate::lexer::*;
use crate::parser::*;
use crate::utils::*;
use crate::ast::*;

impl Parsable for TypeVar {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        par.match_next(Token::TypeLit)?;

        Ok(match par.text(0)? {
            "Int" => { Type::Lit(TypeLit::Int) }
            "Real" => { Type::Lit(TypeLit::Real) }
            "Bool" => { Type::Lit(TypeLit::Bool) }
            "Char" => { Type::Lit(TypeLit::Char) }
            _ => { panic!("Impossible!"); }
        })
    }
}