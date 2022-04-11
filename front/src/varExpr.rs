use std::env::VarError;

use crate::utils::*;
use crate::parser::*;


pub struct VarExpr {
    ident: String,
    span: Span,
}

impl Parsable for VarExpr {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        if let par.peek()

    }
}