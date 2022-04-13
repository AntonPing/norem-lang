use crate::checker::Typable;
use crate::types::TypeVar;
use crate::utils::*;
use crate::lexer::Token;
use crate::parser::*;
use crate::checker::*;

use crate::ast::*;

impl Parsable for ExprVar {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        match par.next()? {
            Token::Var => {
                let ident = par.text(0)?.to_string();
                let span = par.span(0)?;
                Ok(Box::new(ExprVar { ident, span }))
            }
            _ => { Err("parsing variable failed!".to_string())}
        }
    }
}

impl Typable for ExprVar {
    fn infer(&self, chk: &mut Checker) -> Result<TypeVar,String> {
        chk.lookup(&self.ident)
    }
}