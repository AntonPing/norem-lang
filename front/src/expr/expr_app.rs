

use crate::utils::*;
use crate::lexer::Token;
use crate::parser::*;

use crate::ast::*;

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