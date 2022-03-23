use crate::{ast::*, symbol::Symbol};

use std::fmt;



/*
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Position {
    pub line: usize,
    pub col: usize,
    pub abs: usize,
}

impl Position {
    pub const fn new(line: usize, col: usize, abs: usize) -> Position {
        Position { line, col, abs }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}({})", self.line, self.col, self.abs)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// A span in the source, with a start and end location
pub struct Span {
    pub start: Position,
    pub end: Position,
}

/// Data with associated code span
pub struct Spanned<T> {
    pub data: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(data: T, span: Span) -> Self {
        Spanned { data, span }
    }
    
    pub fn data(self) -> T {
        self.data
    }

    pub fn fmap<S, F: FnMut(T) -> S>(self, mut f: F) -> Spanned<S> {
        Spanned {
            span: self.span,
            data: f(self.data),
        }
    }

    pub fn smap<S, F: FnMut(T, Span) -> S>(self, mut f: F) -> Spanned<S> {
        Spanned {
            span: self.span,
            data: f(self.data, self.span),
        }
    }
    
    /*
    pub const fn zero(data: T) -> Self {
        Spanned {
            span: Span::zero(),
            data,
        }
    }
    */
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

impl<T> std::ops::Deref for Spanned<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T, E> Spanned<Result<T, E>> {
    pub fn flatten(self) -> Result<Spanned<T>, E> {
        match self.data {
            Ok(t) => Ok(Spanned::new(t, self.span)),
            Err(e) => Err(e),
        }
    }
}

impl<T> Spanned<Option<T>> {
    pub fn flatten(self) -> Option<Spanned<T>> {
        match self.data {
            Some(t) => Some(Spanned::new(t, self.span)),
            None => None,
        }
    }
}

impl<T: Copy> Copy for Spanned<T> {}
impl<T: Clone> Clone for Spanned<T> {
    fn clone(&self) -> Self {
        Spanned::new(self.data.clone(), self.span)
    }
}

impl<T: fmt::Debug> fmt::Debug for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(f)
    }
}

impl<T: fmt::Display> fmt::Display for Spanned<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.data.fmt(f)
    }
}

impl Span {
    pub fn new(start: Position, end: Position) -> Span {
        Span { start, end }
    }
    /*

    pub const fn zero() -> Span {
        let max = Location {
            line: 0,
            col: 0,
            abs: 0,
        };
        Span {
            start: max,
            end: max,
        }
    }

    pub const fn dummy() -> Span {
        let max = Location::new(std::u16::MAX, std::u16::MAX, 0);
        Span {
            start: max,
            end: max,
        }
    }
    */
}


impl std::ops::Add<Span> for Span {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Span {
            start: self.start,
            end: rhs.end,
        }
    }
}

impl std::ops::Add<Location> for Span {
    type Output = Self;
    fn add(self, rhs: Location) -> Self::Output {
        Span {
            start: self.start,
            end: rhs,
        }
    }
}

impl std::ops::AddAssign<Span> for Span {
    fn add_assign(&mut self, rhs: Self) {
        self.end = rhs.end;
    }
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn spanned_size() {
        //assert_eq!(std::mem::size_of::<Span>(), 16);
    }
}
*/

pub fn lit_value_type(val: LitValue) -> LitType {
    match val {
        LitValue::Bool(_) => LitType::Bool,
        LitValue::Char(_) => LitType::Char,
        LitValue::Int(_) => LitType::Int,
        LitValue::Real(_) => LitType::Real,
    }
}


pub fn unfold_lam(expr: &Expr) -> (Vec<&Symbol>,&Expr) {
    unimplemented!()
}