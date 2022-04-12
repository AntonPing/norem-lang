use crate::utils::*;
use crate::lexer::Token;
use crate::parser::*;
use crate::expr::*;

use crate::ast::*;

impl Parsable for ExprLam {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {

        match par.next()? {

            Token::Fn => {
                let start = par.span(0)?.start;

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

                let body = expr_app::parse_app_list(par)?;

                let end = par.span(0)?.end;
                let span = Span { start, end };
                
                Ok(Box::new(ExprLam { args, body, span }))
            }
            _ => { Err("parsing variable failed!".to_string())}
        }
    }
}