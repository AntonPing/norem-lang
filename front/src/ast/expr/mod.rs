
use super::*;

pub mod expr_lit;
pub mod expr_var;
pub mod expr_lam;
pub mod expr_app;
pub mod expr_let;
pub mod expr_case;

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Lit(x) => { write!(f,"{}",*x)? }
            Expr::Var(x) => { write!(f,"{}",*x)? }
            Expr::Lam(x) => { write!(f,"{}",*x)? }
            Expr::App(x) => {write!(f,"({})",*x)? }
            Expr::Let(x) => { write!(f,"{}",*x)? }
            Expr::Case(x) => { write!(f,"{}",*x)? }
        }
        Ok(())
    }
}

impl Parsable for Expr {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        match par.peek()? {
            Token::Int | Token::Real | Token::Char | Token::Bool => {
                let res = par.parse::<ExprLit>()?;
                Ok(Box::new(Expr::Lit(res)))
            }
            Token::Var => {
                let res = par.parse::<ExprVar>()?;
                Ok(Box::new(Expr::Var(res)))
            }
            Token::Fn => {
                let res = par.parse::<ExprLam>()?;
                Ok(Box::new(Expr::Lam(res)))
            }
            Token::Let => {
                let res = par.parse::<ExprLet>()?;
                Ok(Box::new(Expr::Let(res)))
            }
            Token::LParen => {
                par.next()?;
                let res = par.parse::<ExprApp>()?;
                par.match_next(Token::RParen)?;
                if res.args.len() == 0 {
                    Ok(res.func)
                } else {
                    Ok(Box::new(Expr::App(res)))
                }
            }
            Token::Case => {
                let res = par.parse::<ExprCase>()?;
                Ok(Box::new(Expr::Case(res)))
            }
            tok => {
                Err(format!(
                    "parse expression faile!\n
                    unexcepted token {:?}",
                &tok
                ))
            }
        }
    }
}

impl Typable for Expr {
    fn infer(&self, chk: &mut Checker) -> Result<Type,String> {
        match self {
            Expr::Lit(x) => x.infer(chk),
            Expr::Var(x) => x.infer(chk),
            Expr::Lam(x) => x.infer(chk),
            Expr::App(x) => x.infer(chk),
            _ => unimplemented!(),
        }
    }
}

impl TransCore for Expr {
    fn translate(&self, trs: &mut Translator) -> Result<CoreExpr,String> {
        match self {
            Expr::Lit(x) => x.translate(trs),
            Expr::Var(x) => x.translate(trs),
            Expr::Lam(x) => x.translate(trs),
            Expr::App(x) => x.translate(trs),
            _ => unimplemented!(),
        }
    }
}

#[test]
fn parser_test() {
    let text = "42";
    let mut par = Parser::new(text);
    let res = par.parse::<Expr>().unwrap();
    println!("{}", res);
}