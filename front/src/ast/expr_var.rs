use crate::checker::Typable;
use crate::types::TypeVar;
use crate::utils::*;
use crate::lexer::Token;
use crate::parser::*;
use crate::checker::*;

use super::*;

impl Parsable for ExprVar {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        match par.next()? {
            Token::Var => {
                let ident = par.text(0)?.to_string();
                Ok(Box::new(ExprVar { ident }))
            }
            _ => { Err("parsing variable failed!".to_string())}
        }
    }
}


impl Typable for ExprVar {
    fn infer(&self, chk: &mut Checker) -> Result<TypeVar,String> {
        let sc = chk.lookup(&self.ident)?;
        let ty = chk.instantiate(&sc);
        Ok(ty)
        // todo: occur check
    }
}