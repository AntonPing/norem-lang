use crate::checker::Typable;
use crate::types::Type;
use crate::utils::*;
use crate::lexer::Token;
use crate::parser::*;
use crate::checker::*;

use super::*;

impl Parsable for ExprVar {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        let ident = par.parse::<Symbol>()?;
        Ok(Box::new(ExprVar{ ident }))
    }
}


impl Typable for ExprVar {
    fn infer(&self, chk: &mut Checker) -> Result<Type,String> {
        let sc = chk.lookup(&self.ident)?;
        let ty = chk.instantiate(&sc);
        Ok(ty)
        // todo: occur check
    }
}