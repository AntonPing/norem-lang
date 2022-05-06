use super::*;

impl fmt::Display for ExprLet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"let ")?;
        for decl in &self.decls {
            writeln!(f,"{};", decl)?;
        }
        write!(f,"in {} end", self.body)?;
        Ok(())
    }
}

impl Parsable for ExprLet {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {

        par.match_next(Token::Let)?;

        let decls = par.parse_many1::<Decl>(&vec![
            Token::Val, Token::Data, Token::Type,
        ])?;

        par.match_next(Token::In)?;

        let body = par.parse::<Expr>()?;

        par.match_next(Token::End)?;

        let body = Box::new(body);

        Ok(Box::new(ExprLet { decls, body }))

    }
}

impl Typable for ExprLet {
    fn infer(&self, chk: &mut Checker) -> Result<Type,String> {

        let mark1 = chk.var_env().backup();
        let mark2 = chk.cons_env().backup();
        let mark3 = chk.type_env().backup();

        for decl in self.decls {
            match self {
                Decl::Val(x) => {
                    let var = Type::Temp(chk.newvar());
                    chk.var_env().update(x.name, var);
                }
                Decl::Data(x) => { &x.name }
                Decl::Type(x) => { &x.name }
            }
        }

        let body_ty = self.body.infer(chk)?;

        chk.var_env().recover(mark1);
        chk.cons_env().recover(mark2);
        chk.type_env().recover(mark3);

        Ok(body_ty)
    }
}



#[test]
fn parser_test() {
    let text = "
        let
            val x = 42
            type MyInt = Int
        in
            x
        end
    ";
    let mut par = Parser::new(text);
    let res = par.parse::<Expr>().unwrap();
    println!("{}", res);
}