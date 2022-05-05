use super::*;

impl fmt::Display for DeclType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"type {}", self.name)?;
        for arg in &self.args {
            write!(f," {}", arg)?;
        }
        write!(f," = {};", self.typ)?;
        Ok(())
    }
}


impl Parsable for DeclType {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        par.match_next(Token::Type)?;

        par.match_peek(Token::UpVar)?;
        let name = par.parse::<Symbol>()?;

        let args = par.parse_many::<Symbol>(
            &vec![Token::Var]
        )?;

        par.match_next(Token::Equal)?;

        let typ = par.parse::<Type>()?;

        Ok(Box::new(DeclType { name, args, typ }))
    }
}

