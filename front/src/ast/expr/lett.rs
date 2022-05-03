
use crate::utils::*;
use crate::lexer::Token;
use crate::parser::*;
use crate::checker::*;

use super::*;
use crate::types::*;

impl Parsable for ExprLet {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {

        match par.next()? {

            Token::Fn => {

                let mut args = Vec::new();
                while let Token::Var = par.next()? {
                    args.push(par.text(0)?.to_string());
                }

                if args.len() == 0 {
                    return Err(
                        "function should have at least one argument!"
                    .to_string());
                }

                if Token::EArrow != par.token(0)? {
                    return Err(format!(
                        "excepted token '=>' ! found {:?}", par.token(0)?
                    ).to_string());
                }

                let body = app::parse_app_list(par)?;
                
                Ok(Box::new(ExprLam { args, body}))
            }
            _ => { Err("parsing variable failed!".to_string())}
        }
    }
}

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