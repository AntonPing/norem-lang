use std::collections::HashMap;
use crate::lexer::*;
use crate::parser::*;
use crate::utils::*;
use crate::ast::*;

impl Parsable for TypeApp {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {

        par.match_next(Token::UpVar)?;

        let cons: Symbol = par.parse()?;

        let mut vec: Vec<Type> = par.parse_many1(
            &vec![Token::LitType, Token::UpVar,Token::LParen])?;

        let mut init = Type::Cons(cons);

        for ty in vec.into_iter() {
            let ty1 = Box::new(init);
            let ty2 = Box::new(ty);
            init = Type::App(TypeApp{ ty1 , ty2 });
        }

        if let Type::App(app) = init {
            Ok(Box::new(app))
        } else {
            panic!("Impossible");
        }
    }
}