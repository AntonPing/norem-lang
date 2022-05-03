use crate::utils::*;


pub mod expr;
pub mod typp;
pub mod decl;
pub mod pattern;
pub mod variant;
pub mod rule;

use typp::*;

trait ExprTrait {
    fn span(&self) -> Span;
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Lit(ExprLit),
    Var(ExprVar),
    Lam(ExprLam),
    App(ExprApp),
    Let(ExprLet),
    Case(ExprCase),
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum ExprLit {
    Int(i64),
    Real(f64),
    Bool(bool),
    Char(char),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprVar {
    pub ident: Symbol,
}


#[derive(Clone, Debug, PartialEq)]
pub struct ExprLam {
    pub args: Vec<Symbol>,
    pub body: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprApp {
    pub func: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLet {
    pub decls: Vec<Decl>,
    pub body: Box<Expr>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprCase {
    pub expr: Box<Expr>,
    pub rules: Vec<Rule>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Decl {
    Val(DeclVal),
    Data(DeclData),
    Type(DeclType),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclVal {
    pub name: Symbol,
    pub args: Vec<Symbol>,
    pub body: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclData {
    pub name: Symbol,
    pub args: Vec<Symbol>,
    pub vars: Vec<Variant>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variant {
    pub cons: Symbol,
    pub args: Vec<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclType {
    pub name: Symbol,
    pub args: Vec<Symbol>,
    pub typ: Type,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    /// Algebraic datatype constructor, along with binding pattern
    App(Symbol, Vec<Pattern>),
    /// Constant
    Lit(ExprLit),
    /// List pattern [pat1, ... patN]
    // List(Vec<Pat>),
    /// Record pattern { label1, label2 }, and whether it's flexible or not
    // Record(Vec<Row<Pat>>, bool),
    /// Variable binding
    Var(Symbol),
    Wild,
}

/// Rule of a case expression
#[derive(Clone, Debug, PartialEq)]
pub struct Rule {
    pub pat: Pattern,
    pub body: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Temp(usize),
    Cons(Symbol),
    Lit(TypeLit),
    Var(Symbol),
    Arr(Box<Type>, Box<Type>),
    App(Box<Type>, Box<Type>),
    Poly(Vec<Symbol>, Box<Type>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum TypeLit {
    Int,
    Real,
    Bool,
    Char,
}

/*
#[derive(Clone, Debug, PartialEq)]
pub struct TypeVar {
    pub name: Symbol,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeArr {
    pub ty1: Box<Type>,
    pub ty2: Box<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeApp {
    pub ty1: Box<Type>,
    pub ty2: Box<Type>,
}
*/

#[derive(Clone, Debug, PartialEq)]
pub enum Scheme {
    Mono(Type),
    Poly(Vec<Symbol>,Type),
}

/*
/// Interestingly, MLton immediately desugars tuples during parsing, rather than
/// during elaboration. We do the same
pub fn make_record(v: Vec<Expr>) -> ExprKind {
    ExprKind::Record(
        v.into_iter()
            .enumerate()
            .map(|(idx, ex)| Row {
                label: Symbol::tuple_field(1 + idx as u32),
                span: ex.span,
                data: ex,
            })
            .collect(),
    )
}

pub fn make_record_type(v: Vec<Type>) -> TypeKind {
    TypeKind::Record(
        v.into_iter()
            .enumerate()
            .map(|(idx, ex)| Row {
                label: Symbol::tuple_field(1 + idx as u32),
                span: ex.span,
                data: ex,
            })
            .collect(),
    )
}

pub fn make_record_pat(v: Vec<Pat>, flex: bool) -> PatKind {
    PatKind::Record(
        v.into_iter()
            .enumerate()
            .map(|(idx, ex)| Row {
                label: Symbol::tuple_field(1 + idx as u32),
                span: ex.span,
                data: ex,
            })
            .collect(),
        flex,
    )
}


#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Row<T> {
    pub label: Symbol,
    pub data: T,
    pub span: Span,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Fixity {
    Infixl,
    Infixr,
    Nonfix,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Primitive {
    pub sym: Symbol,
    pub ty: Type,
}
*/

