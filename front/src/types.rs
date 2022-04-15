use std::collections::HashMap;
use crate::lexer::*;
use crate::parser::*;
use crate::utils::*;
use crate::ast::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum LitType {
    Int,
    Real,
    Bool,
    Char,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeVar {
    Lit(LitType),
    Var(usize),
    Arr(Box<TypeVar>, Box<TypeVar>),
    App(Symbol, Vec<TypeVar>),
    //Cons(Symbol),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Forall(usize,Box<Type>),
    Lit(LitType),
    Var(usize),
    Arr(Box<Type>, Box<Type>),
    
    //App(Box<TypeVar>, Box<TypeVar>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Scheme {
    Mono(TypeVar),
    Poly(Vec<usize>,TypeVar),
}

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

impl TypeVar {
    pub fn ftv(&self) -> MultiSet<usize> {
        let mut set = MultiSet::new();
        let mut stack = Vec::<&TypeVar>::new();
        stack.push(self);

        while let Some(elem) = stack.pop() {
            match elem {
                TypeVar::Lit(_) => {}
                TypeVar::Var(x) => {
                    set.insert(*x);
                }
                TypeVar::Arr(ty1,ty2) => {
                    stack.push(ty1);
                    stack.push(ty2);
                }
                TypeVar::App(cons, args) => {
                    for arg in args {
                        stack.push(arg);
                    }
                }
            }
        }
        set
    }


    pub fn subst(&self, sub: &HashMap<usize,TypeVar>) -> TypeVar {
        match self {
            TypeVar::Lit(_) => {
                self.clone()
            }
            TypeVar::Var(x) => {
                if let Some(t) = sub.get(&x) {
                    t.clone()
                } else {
                    self.clone()
                }
            }
            TypeVar::Arr(t1,t2) => {
                TypeVar::Arr(
                    Box::new(t1.subst(sub)),
                    Box::new(t2.subst(sub)))
            }
            TypeVar::App(cons, args) => {
                let mut new_args = Vec::new();
                for arg in args {
                    new_args.push(arg.subst(sub));
                }
                TypeVar::App(cons.clone(), new_args)
            }
        }
    }
}

impl Parsable for TypeVar {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        let mut vec = Vec::new();

        loop {
            match par.peek()? {
                Token::LitType => {
                    vec.push(*parse_type_lit(par)?);
                }
                Token::UpVar => {
                    vec.push(*parse_type_app(par)?);
                }
                Token::LParen => {
                    vec.push(par.parse::<TypeVar>()?);
                    par.match_next(Token::RParen)?;
                }
                _ => { break; }
            }
        }

        match vec.len() {
            0 => { Err("can't parse type!".to_string()) }
            1 => { }
        }


        Err("can't parse type!".to_string())
    }
}


fn parse_type_lit(par: &mut Parser) -> Result<Box<TypeVar>,String> {
    par.match_next(Token::LitType)?;
    Ok(Box::new(match par.text(0)? {
        "Int" => TypeVar::Lit(LitType::Int),
        "Real" => TypeVar::Lit(LitType::Real),
        "Bool" => TypeVar::Lit(LitType::Bool),
        "Char" => TypeVar::Lit(LitType::Char),
        _ => { panic!("not a constant type!"); }
    }))
}

/*
fn parse_type_var(par: &mut Parser) -> Result<Box<TypeVar>,String> {
    par.match_next(Token::UpVar)?;
    Ok(Box::new(TypeVar::Var(Symbol::new(par.text(0)?))))
}
*/

fn parse_type_app(par: &mut Parser) -> Result<Box<TypeVar>,String> {
    par.match_next(Token::UpVar)?;
    
    let cons = Symbol::new(par.text(0)?);

    let mut args: Vec<TypeVar> = par.many(|p| 
        p.peek() == Ok(Token::UpVar) 
        || p.peek() == Ok(Token::LitType));

    Ok(Box::new(TypeVar::App(cons, args)))
}


fn parse_arr_list(par: &mut Parser) -> Result<Box<TypeVar>,String> {

    //let mut vec = Vec::new();

    unimplemented!()

}