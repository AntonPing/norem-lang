use crate::{ast::*, symbol::Symbol};

use std::cell::RefCell;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
/// A span in the source, with a start and end location
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }
    pub const fn zero() -> Span {
        Span { start: 0, end: 0 }
    }
}

/// A Box with span position message
pub struct Spanned<T> {
    pub data: Box<T>,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(data: T, span: Span) -> Self {
        Spanned {
            data: Box::new(data),
            span,
        }
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
        Spanned {
            data: self.data.clone(),
            span: self.span,
        }
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

//
// Pointers
//

pub type Ptr<T> = Rc<T>;

#[allow(non_snake_case)]
pub fn Ptr<T>(val: T) -> Ptr<T> {
    Ptr::new(val)
}

pub struct Mut<T>(Rc<RefCell<T>>);
pub struct MutWeak<T>(Weak<RefCell<T>>);

impl<T> Mut<T> {
    pub fn new(val: T) -> Mut<T> {
        Mut(Rc::new(RefCell::new(val)))
    }

    pub fn weak(&self) -> MutWeak<T> {
        MutWeak(Rc::downgrade(&self.0))
    }

    pub fn take_inner(this: Self) -> Result<T, Mut<T>> {
        Rc::try_unwrap(this.0).map(|x| x.into_inner()).map_err(Mut)
    }
}

impl<T> Deref for Mut<T> {
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Deref for MutWeak<T> {
    type Target = Weak<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for Mut<T> {
    fn clone(&self) -> Self {
        Mut(self.0.clone())
    }
}

impl<T> Debug for Mut<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.borrow().fmt(f)
    }
}

impl<T> std::fmt::Display for Mut<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&*self.0.borrow(), f)
    }
}

impl<T> Debug for MutWeak<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub fn unfold_lam(expr: &Expr) -> (Vec<&Symbol>, &Expr) {
    unimplemented!()
}
