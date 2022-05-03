use crate::utils::*;
use crate::lexer::Token;
use crate::parser::*;
use crate::checker::*;

use super::*;

impl Parsable for ExprLet {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {

        par.match_next(Token::Let)?;

        let decls = par.parse_many1::<Decl>(&vec![
            Token::Val, Token::Data, Token::Type,
        ])?;

        par.match_next(Token::In)?;

        let body = par.parse::<Expr>()?;

        let body = Box::new(body);

        Ok(Box::new(ExprLet { decls, body }))

    }
}

/*
impl Typable for ExprLet {
    fn infer(&self, chk: &mut Checker) -> Result<TypeVar,String> {
        
        let mut record = Vec::new();
        
        let mut args_ty = Vec::new();
        for arg in &self.args {
            let new_ty = TypeVar::Var(chk.newvar());
            args_ty.push(new_ty.clone());
            
            if let Some(old) = chk.environment
                .insert(arg.clone(), Scheme::Mono(new_ty)) {    
                record.push((arg.clone(),old));
            }
        }

        let body_ty = self.body.infer(chk)?;

        let res_ty = args_ty
            .into_iter().rev()
            .fold(body_ty, |ty1,ty2| {
                TypeVar::Arr(Box::new(ty2), Box::new(ty1))
            });

        Ok(res_ty)

    }
}
*/