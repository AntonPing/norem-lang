use crate::checker::*;
use crate::types::*;
use crate::utils::*;
use crate::lexer::Token;
use crate::parser::*;

use super::*;

impl Parsable for ExprApp {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        let func = Expr::parse(par)?;

        let mut args = Vec::new();
        while let Ok(expr) = Expr::parse(par) {
            args.push(expr);
        }

        if args.len() == 0 {
            return Err(
                "application should have at least one argument!"
            .to_string())
        }

        Ok(Box::new(ExprApp { func, args }))
    }
}

pub fn parse_app_list(par: &mut Parser) -> Result<Box<Expr>,String> {
    let func = Expr::parse(par)?;

    let mut args = Vec::new();

    while let Ok(expr) = Expr::parse(par) {
        args.push(expr);
    }

    if args.len() == 0 {
        Ok(func)
    } else {
        Ok(Box::new(Expr::App(ExprApp { func, args })))
    }
}



impl Typable for ExprApp {
    fn infer(&self, chk: &mut Checker) -> Result<TypeVar,String> {

        let func_ty = self.func.infer(chk)?;

        let mut args_ty = Vec::new();
        for arg in &self.args {
            let arg_ty = arg.infer(chk)?;
            args_ty.push(arg_ty);
        }

        let res_ty = TypeVar::Var(chk.newvar());
        
        let func_ty_2 = args_ty
            .into_iter().rev()
            .fold(res_ty.clone(), |ty1, ty2| {
                TypeVar::Arr(Box::new(ty2), Box::new(ty1))
            });
        
        
        chk.unify(&func_ty, &func_ty_2)?;

        Ok(res_ty)
    }
}
