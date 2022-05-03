use std::collections::HashMap;
use crate::lexer::*;
use crate::parser::*;
use crate::utils::*;
use crate::ast::*;

impl Parsable for TypeArr {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        let mut vec: Vec<Type> = par.parse_sepby1(Token::Arrow)?;

        let mut init = vec.pop().unwrap();

        for ty in vec.into_iter().rev() {
            let ty1 = Box::new(ty);
            let ty2 = Box::new(init);
            init = Type::Arr(TypeArr{ ty1 , ty2 });
        }

        if let Type::Arr(arr) = init {
            Ok(Box::new(arr))
        } else {
            panic!("Impossible");
        }
    }
}