use super::*;

impl fmt::Display for ExprLit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ExprLit::Int(x) => { write!(f,"{}",*x) }
            ExprLit::Real(x) => { write!(f,"{}",*x) }
            ExprLit::Bool(x) => { write!(f,"{}",*x) }
            ExprLit::Char(x) => { write!(f,"{}",*x) }
        }
    }
}

impl Parsable for ExprLit {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        match par.next()? {
            Token::Int => {
                let value = par.text(0)?.parse().unwrap();
                Ok(Box::new(ExprLit::Int(value)))
            }
            Token::Real => {
                let value = par.text(0)?.parse().unwrap();
                Ok(Box::new(ExprLit::Real(value)))
            }
            Token::Bool => {
                let value = par.text(0)?.parse().unwrap();
                Ok(Box::new(ExprLit::Bool(value)))
            }
            Token::Char => {
                let value = par.text(0)?.parse().unwrap();
                Ok(Box::new(ExprLit::Char(value)))
            }
            _ => { Err("parsing variable failed!".to_string())}
        }
    }
}

impl Typable for ExprLit {
    fn infer(&self, _chk: &mut Checker) -> Result<Type,String> {
        match self {
            ExprLit::Int(_) => Ok(Type::Lit(TypeLit::Int)),
            ExprLit::Real(_) => Ok(Type::Lit(TypeLit::Real)),
            ExprLit::Char(_) => Ok(Type::Lit(TypeLit::Char)),
            ExprLit::Bool(_) => Ok(Type::Lit(TypeLit::Bool)),
        }
    }
}