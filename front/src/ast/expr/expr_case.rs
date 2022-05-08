use super::*;

impl fmt::Display for ExprCase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"case {} of ", self.expr)?;
        for rule in &self.rules {
            write!(f,"| {} ", rule)?;
        }
        Ok(())
    }
}

impl Parsable for ExprCase {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        par.match_next(Token::Case)?;

        let expr = par.parse::<Expr>()?;
        let expr = Box::new(expr);

        par.match_next(Token::Of)?;

        let rules = par.parse_many1::<Rule>(
            &vec![Token::Bar]
        )?;

        par.match_next(Token::End)?;

        Ok(Box::new(ExprCase { expr, rules }))
    }
}

impl Typable for ExprCase {
    fn infer(&self, chk: &mut Checker) -> Result<Type,String> {

        unimplemented!()
    }
}

impl TransCore for ExprCase {
    fn translate(&self, trs: &mut Translator) -> Result<CoreExpr,String> {
        unimplemented!()
    }
}

#[test]
fn parser_test() {
    let text = "case 42 of | a => 1 | b => 2";
    let mut par = Parser::new(text);
    let res = par.parse::<ExprCase>().unwrap();
    println!("{}", res);
}
