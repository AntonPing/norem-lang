
use crate::utils::*;
use crate::lexer::Token;
use crate::parser::*;
use crate::checker::*;

use super::*;
use crate::types::*;

impl Parsable for ExprLam {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {

        par.match_next(Token::Fn)?;

        let args = par.many1::<Symbol>(|p| 
            p.peek() == Ok(Token::Var))?;

        par.match_next(Token::EArrow)?;

        let body = par.parse::<ExprApp>()?;

        let body = Box::new(Expr::App(body));
        
        Ok(Box::new(ExprLam { args, body }))
    }
}

impl Typable for ExprLam {
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