use std::fmt;

use crate::utils::*;
use crate::lexer::Token;
use crate::parser::{Parsable, Parser};
use crate::checker::*;

use super::*;

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{} => {}", self.pat, self.body)?;
        Ok(())
    }
}

impl Parsable for Rule {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        par.match_next(Token::Bar)?;

        let pat = par.parse::<Pattern>()?;

        par.match_next(Token::EArrow)?;
        
        let body = par.parse::<Expr>()?;

        Ok(Box::new(Rule { pat, body }))
    }
}

#[test]
fn parser_test() {
    let text = "| a => 1";
    let mut par = Parser::new(text);
    let res = par.parse::<Rule>().unwrap();
    println!("{}", res);
}


