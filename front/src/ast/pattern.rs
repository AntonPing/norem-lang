use crate::utils::*;
use crate::lexer::Token;
use crate::parser::{Parsable, Parser};
use crate::checker::*;

use super::*;

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"pattern")?;
        Ok(())
    }
}

impl Parsable for Pattern {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {

        match par.peek()? {
            Token::Int | Token::Real | Token::Bool | Token::Char => {
                let lit = par.parse::<ExprLit>()?;
                Ok(Box::new(Pattern::Lit(lit)))
            }
            Token::Var => {
                let sym = par.parse::<Symbol>()?;
                Ok(Box::new(Pattern::Var(sym)))
            }
            Token::UpVar => {
                let con = par.parse::<Symbol>()?;
                let args : Vec<Pattern> = par.parse_many(&vec![
                    Token::Int, Token::Real, Token::Bool , Token::Char,
                    Token::Var, Token::UpVar, Token::LParen
                ])?;
                Ok(Box::new(Pattern::App(con, args)))
            }
            Token::LParen => {
                let pat: Pattern = par.parse()?;
                par.match_next(Token::RParen)?;
                Ok(Box::new(pat))
            }
            _ => {
                Err("Can't parse pattern!".to_string())
            }
        }
    }
}

/*
fn start_of_pattern(par: &mut Parser) -> bool {
    if let Ok(tok) = par.peek() {
        match tok {
            Token::Int | Token::Real | Token::Bool | Token::Char |
            Token::Var | Token::UpVar | Token::LParen => {
                true
            }
            _ => false
        }

    } else {
        false
    }
}
*/

#[test]
fn parser_test() {
    let text = "a";
    let mut par = Parser::new(text);
    let res = par.parse::<Pattern>().unwrap();
    println!("{:?}", res);
}
