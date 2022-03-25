use logos::Span;
use crate::symbol::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Lit(LitValue),
    Var(Symbol),
    Lam(Symbol, Box<Expr>),
    App(Box<Expr>, Box<Expr>),
    Let(Vec<DeclKind>,Box<Expr>),
    Case(Box<Expr>, Vec<Rule>),
    Ifte(Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Lit(LitType),
    Var(Symbol),
    Arr(Box<Type>, Box<Type>),
    // Con(Symbol),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LitValue {
    Int(i64),
    Real(f64),
    Bool(bool),
    Char(char),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum LitType {
    Int,
    Real,
    Bool,
    Char,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variant {
    pub constr: Symbol,
    pub args: Vec<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValDecl {
    pub name: Symbol,
    pub args: Vec<Symbol>,
    pub body: Expr,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataDecl {
    pub name: Symbol,
    pub args: Vec<Symbol>,
    pub branches: Vec<Variant>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeDecl {
    pub name: Symbol,
    pub args: Vec<Symbol>,
    pub typ: Type,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DeclKind {
    Val(ValDecl),
    Data(DataDecl),
    Type(TypeDecl),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    /// Algebraic datatype constructor, along with binding pattern
    App(Symbol, Vec<Pattern>),
    /// Constant
    Lit(LitValue),
    /// List pattern [pat1, ... patN]
    // List(Vec<Pat>),
    /// Record pattern { label1, label2 }, and whether it's flexible or not
    // Record(Vec<Row<Pat>>, bool),
    /// Variable binding
    Var(Symbol),
    /// Wildcard
    Wild,
}

/// Rule of a case expression
#[derive(Clone, Debug, PartialEq)]
pub struct Rule {
    pub pat: Pattern,
    pub expr: Expr,
    pub span: Span,
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