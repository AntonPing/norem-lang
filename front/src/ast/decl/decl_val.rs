use super::*;

impl fmt::Display for DeclVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"val {}", self.name)?;
        for arg in &self.args {
            write!(f," {}", arg)?;
        }
        write!(f," = {};", self.body)?;
        Ok(())
    }
}

impl Parsable for DeclVal {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        par.match_next(Token::Val)?;

        par.match_peek(Token::Var)?;
        let name = par.parse::<Symbol>()?;

        let args = par.parse_many::<Symbol>(
            &vec![Token::Var]
        )?;

        par.match_next(Token::Equal)?;

        let body = par.parse::<Expr>()?;

        Ok(Box::new(DeclVal { name, args, body }))
    }
}

#[test]
fn parser_test() {
    let text = "val x y = 42";
    let mut par = Parser::new(text);
    let res = par.parse::<DeclVal>().unwrap();
    println!("{}", res);
}
