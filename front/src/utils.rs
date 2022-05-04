use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, self};
use std::hash::Hash;
use std::ops::Deref;

use crate::parser::Parsable;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// A span in the source, with a start and end location
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span { start , end }
    }
    pub const fn zero() -> Span {
        Span {
            start: 0,
            end: 0,
        }
    }
}

/// A Box with span position message
pub struct Spanned<T> {
    pub data: Box<T>,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(data: T, span: Span) -> Self {
        Spanned { data: Box::new(data), span }
    }
    
    pub fn data(self) -> T {
        *self.data
    }

    pub fn span(self) -> Span {
        self.span
    }

    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> Spanned<U> {
        Spanned {
            data: Box::new(f(*self.data)),
            span: self.span,
        }
    }
    
    pub fn zero(data: T) -> Self {
        Spanned {
            data: Box::new(data),
            span: Span::zero(),
        }
    }
}

impl<T: PartialEq> PartialEq for Spanned<T> {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<T: PartialOrd> PartialOrd for Spanned<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.data.partial_cmp(&other.data)
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl<T, E> Spanned<Result<T, E>> {
    pub fn flatten(self) -> Result<Spanned<T>, E> {
        match *self.data {
            Ok(t) => Ok(Spanned::new(t, self.span)),
            Err(e) => Err(e),
        }
    }
}

impl<T> Spanned<Option<T>> {
    pub fn flatten(self) -> Option<Spanned<T>> {
        match *self.data {
            Some(t) => Some(Spanned::new(t, self.span)),
            None => None,
        }
    }
}

//impl<T: Copy> Copy for Spanned<T> {}
impl<T: Clone> Clone for Spanned<T> {
    fn clone(&self) -> Self {
        Spanned { data: self.data.clone(), span: self.span }
    }
}

impl<T: Debug> Debug for Spanned<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        (*self.data).fmt(f)
    }
}

impl<T: Display> Display for Spanned<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        (*self.data).fmt(f)
    }
}


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Symbol(String);

impl Symbol {
    pub fn new(str: &str) -> Self {
        Symbol(str.to_string())
    }
}

impl From<String> for Symbol {
    fn from(item: String) -> Self {
        Symbol(item)
    }
}

impl From<Symbol> for String {
    fn from(item: Symbol) -> Self {
        item.0
    }
}

use crate::parser::*;
use crate::lexer::*;

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.0)?;
        Ok(())
    }
}

impl Parsable for Symbol {
    fn parse(par: &mut Parser) -> Result<Box<Self>,String> {
        let tok = par.next()?;
        match tok {
            Token::Var | Token::UpVar => {
                Ok(Box::new(Symbol::new(par.text(0)?)))
            }
            _ => {
                Err("can't parse symbol!".to_string())
            }
        }
    }

}

pub struct MultiSet<T>(HashMap<T,usize>);

impl<T> MultiSet<T> where T: Hash + Eq {

    pub fn new() -> MultiSet<T> {
        MultiSet(HashMap::new())
    }

    pub fn insert(&mut self, elem: T) {
        if let Some((k,v)) = self.0.remove_entry(&elem) {
            self.0.insert(k, v + 1);
        } else {
            self.0.insert(elem, 1);
        }
    }

    pub fn remove(&mut self, elem: &T) {
        if let Some((k,v)) = self.0.remove_entry(elem) {
            if v > 1 {
                self.0.insert(k, v - 1);
            }
        }
    }

    pub fn remove_all(&mut self, elem: &T) {
        self.0.remove(elem);
    }

    pub fn contains(&self, value: &T) -> bool {
        self.0.get(value).is_some()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn to_vec(self) -> Vec<T> {
        let mut vec = Vec::new();
        for (k,_) in self.0 {
            vec.push(k);
        }
        vec
    }
}

/*
impl<T> IntoIterator for MultiSet<T> {
    type Item = (T,usize) ;
    type IntoIter = hashmap::IntoIter<T,usize>;

    fn into_iter(self) -> Self::IntoIter {
        let keys = self.0.keys().into_iter();
        MultiSetIntoIter { keys }
    }
}

pub struct MultiSetIntoIter<'a, T> {
    keys: Keys<'a, T, usize>,
}

impl<'a, T> Iterator for MultiSetIntoIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Item> {
        let result = match self.index {
            0 => self.pixel.r,
            1 => self.pixel.g,
            2 => self.pixel.b,
            _ => return None,
        };
        self.index += 1;
        Some(result)
    }
}




*/