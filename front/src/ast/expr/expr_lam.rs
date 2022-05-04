use super::*;

impl fmt::Display for ExprLam {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"fn")?;
        for arg in &self.args {
            write!(f," {}", arg)?;
        }
        write!(f," => {}", self.body)?;
        Ok(())
    }
}

impl Parsable for ExprLam {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {

        par.match_next(Token::Fn)?;

        let args = par.parse_many1::<Symbol>(
            &vec![Token::Var]
        )?;

        par.match_next(Token::EArrow)?;

        let body = par.parse::<ExprApp>()?;

        let body = Box::new(Expr::App(body));
        
        Ok(Box::new(ExprLam { args, body }))
    }
}

impl Typable for ExprLam {
    fn infer(&self, chk: &mut Checker) -> Result<Type,String> {
        
        let mut record = Vec::new();
        
        let mut args_ty = Vec::new();
        for arg in &self.args {
            let new_ty = Type::Temp(chk.newvar());
            args_ty.push(new_ty.clone());
            
            if let Some(old) = chk.environment
                .insert(arg.clone(), Scheme::Mono(new_ty)) {    
                record.push((arg.clone(),old));
            }
        }

        let body_ty = self.body.infer(chk)?;

        let res_ty = args_ty
            .into_iter().rev()
            .fold(body_ty, |ty1,ty2| {
                Type::Arr(Box::new(ty2), Box::new(ty1))
            });

        Ok(res_ty)

    }
}