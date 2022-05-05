use super::*;


impl fmt::Display for DeclData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"data {}", self.name)?;
        for arg in &self.args {
            write!(f," {}", arg)?;
        }
        write!(f,"= {}", &self.vars[0])?;
        for var in &self.vars[1..] {
            write!(f," | {}", var)?;
        }
        Ok(())
    }
}

impl Parsable for DeclData {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        par.match_next(Token::Data)?;

        par.match_peek(Token::UpVar)?;
        let name = par.parse::<Symbol>()?;

        let args = par.parse_many::<Symbol>(
            &vec![Token::Var, Token::UpVar]
        )?;

        par.match_next(Token::Equal)?;

        let vars = par.parse_sepby1::<Variant>(Token::Bar)?;

        Ok(Box::new(DeclData { name, args, vars }))
    }
}

#[test]
fn parser_test() {
    let text = "data Color = Red | Blue | Green";
    let mut par = Parser::new(text);
    let res = par.parse::<DeclData>().unwrap();
    println!("{}", res);
}