use super::*;

impl fmt::Display for ExprVar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.ident)?;
        Ok(())
    }
}

impl Parsable for ExprVar {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        let ident = par.parse::<Symbol>()?;
        Ok(Box::new(ExprVar{ ident }))
    }
}

impl Typable for ExprVar {
    fn infer(&self, chk: &mut Checker) -> Result<Type,String> {
        let sc = chk.lookup(&self.ident)?;
        let ty = chk.instantiate(&sc);
        if chk.occur_check(&self.ident, &ty) {
            Err("Occur check failed!".to_string())
        } else {
            Ok(ty)
        }  
    }
}

impl TransCore for ExprVar {
    fn translate(&self, trs: &mut Translator) -> Result<CoreExpr,String> {
        Ok(CoreExpr::Var(self.ident.clone()))
    }
}