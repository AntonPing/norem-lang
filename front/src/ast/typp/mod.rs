use crate::lexer::*;
use crate::parser::*;
use crate::utils::*;
use crate::ast::*;

pub mod lit;
pub mod var;
pub mod arr;
pub mod app;

/*
impl Scheme {
    pub fn ftv(&self) -> MultiSet<usize> {
        match self {
            Scheme::Mono(ty) => { ty.ftv() }
            Scheme::Poly(args, ty) => {
                let mut set = ty.ftv();
                for arg in args {
                    set.remove_all(arg);
                }
                set
            }
        }
    }
}

impl Type {
    pub fn ftv(&self) -> MultiSet<usize> {
        let mut set = MultiSet::new();
        let mut stack = Vec::<&Type>::new();
        stack.push(self);

        while let Some(elem) = stack.pop() {
            match elem {
                Type::Lit(_) => {}
                Type::Var(x) => {
                    set.insert(*x);
                }
                Type::Arr(ty1,ty2) => {
                    stack.push(ty1);
                    stack.push(ty2);
                }
                Type::App(cons, args) => {
                    for arg in args {
                        stack.push(arg);
                    }
                }
            }
        }
        set
    }


    pub fn subst(&self, sub: &HashMap<usize,Type>) -> Type {
        match self {
            Type::Lit(_) => {
                self.clone()
            }
            Type::Var(x) => {
                if let Some(t) = sub.get(&x) {
                    t.clone()
                } else {
                    self.clone()
                }
            }
            Type::Arr(t1,t2) => {
                Type::Arr(
                    Box::new(t1.subst(sub)),
                    Box::new(t2.subst(sub)))
            }
            Type::App(cons, args) => {
                let mut new_args = Vec::new();
                for arg in args {
                    new_args.push(arg.subst(sub));
                }
                Type::App(cons.clone(), new_args)
            }
        }
    }
}
*/

impl Parsable for Type {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        let mut vec: Vec<Type> = par.parse_sepby1(Token::Arrow)?;

        let mut init = vec.pop().unwrap();

        for ty in vec.into_iter() {
            let ty1 = Box::new(ty);
            let ty2 = Box::new(init);
            init = Type::Arr(TypeArr{ ty1 , ty2 });
        }

        Ok(Box::new(init))
    }
}

fn parse_single_type(par: &mut Parser) -> Result<Box<Type>,String> {
    /*
    match par.peek()? {
        Token::LitType => {
            let res = match par.text(0)? {
                "Int" => { Type::Lit(TypeLit::Int) }
                "Real" => { Type::Lit(TypeLit::Real) }
                "Bool" => { Type::Lit(TypeLit::Bool) }
                "Char" => { Type::Lit(TypeLit::Char) }
                _ => { panic!("Impossible!"); }
            };
            Ok(Box::new(res))
        }
        Token::UpVar => {
            par.match_next(Token::UpVar)?;

            let cons: Symbol = par.parse()?;

            let args: Vec<Type> = par.parse_many1(
                &vec![Token::LitType, Token::UpVar,Token::LParen])?;



            Ok(Box::new(Type::App(TypeApp { cons, args })))
        }
        Token::LParen => {
            par.match_next(Token::LParen)?;
            let res: Type = par.parse()?;
            par.match_next(Token::RParen)?;
            Ok(Box::new(res))
        }
        _ => {
            Err("Can't parse type".to_string())
        }
    }
    */
}