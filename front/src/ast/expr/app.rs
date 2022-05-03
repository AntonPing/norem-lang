use crate::checker::*;
use crate::utils::*;
use crate::lexer::Token;
use crate::parser::*;

use super::*;

impl Parsable for ExprApp {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        let func = Expr::parse(par)?;

        let args = par.parse_many::<Expr>(&vec![
            Token::Int, Token::Real, Token::Char, Token::Bool,
            Token::Var, Token::Fn, Token::LParen,
        ])?;

        Ok(Box::new(ExprApp { func, args }))
    }
}



/*
struct ExprAppList(ExprApp);

impl From<ExprApp> for ExprAppList {
    fn from(item: ExprApp) -> Self {
        ExprAppList(item)
    }
}

impl From<ExprAppList> for ExprApp {
    fn from(item: ExprAppList) -> Self {
        item.0
    }
}

impl Parsable for ExprAppList {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        let func = Expr::parse(par)?;

        let args = par.parse_many1::<Expr>(|p|
            p.peek().is_ok() && Expr::expr_start(p.peek().unwrap()))?;

        Ok(Box::new(ExprApp { func, args }))
    }
}
*/

/*
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
*/

impl Typable for ExprApp {
    fn infer(&self, chk: &mut Checker) -> Result<Type,String> {

        let func_ty = self.func.infer(chk)?;

        let mut args_ty = Vec::new();
        for arg in &self.args {
            let arg_ty = arg.infer(chk)?;
            args_ty.push(arg_ty);
        }

        let res_ty = Type::Temp(chk.newvar());
        
        let func_ty_2 = args_ty
            .into_iter().rev()
            .fold(res_ty.clone(), |ty1, ty2| {
                Type::Arr(Box::new(ty2), Box::new(ty1))
            });
        
        
        chk.unify(&func_ty, &func_ty_2)?;

        Ok(res_ty)
    }
}