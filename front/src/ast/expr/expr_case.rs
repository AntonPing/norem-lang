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

        Ok(Box::new(ExprCase { expr, rules }))
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

impl Typable for ExprCase {
    fn infer(&self, chk: &mut Checker) -> Result<Type,String> {

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
