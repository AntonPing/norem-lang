use std::collections::HashMap;

use crate::lexer::*;
use crate::parser::*;
use crate::utils::*;
use crate::ast::*;

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
*/

impl Type {
    /*
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
                Type::Cons(_) => todo!(),
                Type::Temp(_) => todo!(),
            }
        }
        set
    }
    */

    pub fn subst(&self, sub: &HashMap<Symbol,Type>) -> Type {
        match self {
            Type::Lit(_) => {
                self.clone()
            }
            Type::Cons(_) => {
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
                    Box::new(t2.subst(sub))
                )
            }
            Type::App(func, arg) => {
                let new_arg = arg.subst(sub);
                Type::App(func.clone(), Box::new(new_arg))
            }
            Type::Temp(n) => {
                self.clone()
            }
        }
    }
}


impl Parsable for Type {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        let vec0: Vec<SingleType> = par.parse_sepby1(Token::Arrow)?;
        let mut vec: Vec<Type> = vec0.into_iter().map(|x| x.into()).collect();

        let mut init = vec.pop().unwrap();

        for ty in vec.into_iter() {
            let ty1 = Box::new(ty);
            let ty2 = Box::new(init);
            init = Type::Arr(ty1,ty2);
        }

        Ok(Box::new(init))
    }
}

struct SingleType(Type);

impl From<Type> for SingleType {
    fn from(item: Type) -> Self {
        SingleType(item)
    }
}

impl From<SingleType> for Type {
    fn from(item: SingleType) -> Self {
        item.0
    }
}

impl Parsable for SingleType {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
       let res = match par.peek()? {
            Token::LitType => {
                let res = match par.text(0)? {
                    "Int" => { Type::Lit(TypeLit::Int) }
                    "Real" => { Type::Lit(TypeLit::Real) }
                    "Bool" => { Type::Lit(TypeLit::Bool) }
                    "Char" => { Type::Lit(TypeLit::Char) }
                    _ => { panic!("Impossible!"); }
                };
                res
            }
            Token::UpVar => {
                par.match_next(Token::UpVar)?;

                let cons: Symbol = par.parse()?;

                let args: Vec<Type> = par.parse_many(
                    &vec![Token::LitType, Token::UpVar,Token::LParen])?;
                
                let mut res = Type::Cons(cons);
                
                for arg in args {
                    res = Type::App(Box::new(res), Box::new(arg));
                }

                res
            }
            Token::LParen => {
                par.match_next(Token::LParen)?;
                let res: Type = par.parse()?;
                par.match_next(Token::RParen)?;
                res
            }
            _ => {
                return Err("Can't parse type".to_string());
            }
        };
        Ok(Box::new(SingleType(res)))
    }
}
