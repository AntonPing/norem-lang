use std::fmt;

use crate::symbol::Symbol;
use crate::utils::Span;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Lit(ExprLit),
    Prim(ExprPrim),
    Var(ExprVar),
    Lam(ExprLam),
    App(ExprApp),
    Let(ExprLet),
    Case(ExprCase),
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum LitVal {
    Int(i64),
    Real(f64),
    Bool(bool),
    Char(char),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Prim {
    IAdd,
    ISub,
    IMul,
    IDiv,
    INeg,
    BNot,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprPrim {
    pub prim: Prim,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLit {
    pub lit: LitVal,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprVar {
    pub ident: Symbol,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLam {
    pub args: Vec<Symbol>,
    pub body: Box<Expr>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprApp {
    pub func: Box<Expr>,
    pub args: Vec<Expr>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLet {
    pub decls: Vec<Decl>,
    pub body: Box<Expr>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprCase {
    pub expr: Box<Expr>,
    pub rules: Vec<Rule>,
    pub span: Span,
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
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclData {
    pub name: Symbol,
    pub args: Vec<Symbol>,
    pub vars: Vec<Variant>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variant {
    pub cons: Symbol,
    pub args: Vec<Type>,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclType {
    pub name: Symbol,
    pub args: Vec<Symbol>,
    pub typ: Type,
    pub span: Span,
}

/// Rule of a case expression
#[derive(Clone, Debug, PartialEq)]
pub struct Rule {
    pub pat: Pattern,
    pub body: Expr,
    pub span: Span,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    /// Algebraic datatype constructor, along with binding pattern
    App(Symbol, Vec<Pattern>),
    /// Constant
    Lit(LitVal),
    /// List pattern [pat1, ... patN]
    // List(Vec<Pat>),
    /// Record pattern { label1, label2 }, and whether it's flexible or not
    // Record(Vec<Row<Pat>>, bool),
    /// Variable binding
    Var(Symbol),
    Wild,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum LitType {
    Int,
    Real,
    Bool,
    Char,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Cons(Symbol),
    Lit(LitType),
    Var(Symbol),
    Arr(Box<Type>, Box<Type>),
    App(Box<Type>, Box<Type>),
    //Poly(Vec<Symbol>, Box<Type>),
    Temp(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Scheme {
    Mono(Type),
    Poly(Vec<Symbol>, Type),
}

impl Expr {
    pub fn is_prim(&self) -> bool {
        if let Expr::Prim(_) = self {
            true
        } else {
            false
        }
    }
}

impl Prim {
    pub fn get_arity(&self) -> usize {
        match self {
            Prim::IAdd => 2,
            Prim::ISub => 2,
            Prim::IMul => 2,
            Prim::IDiv => 2,
            Prim::INeg => 1,
            Prim::BNot => 1,
        }
    }   
}

impl fmt::Display for Prim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{self:?}")
    }
}



/*
#[derive(Debug, Clone, PartialEq)]
pub struct TypeVar {
    pub name: Symbol,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeArr {
    pub ty1: Box<Type>,
    pub ty2: Box<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeApp {
    pub ty1: Box<Type>,
    pub ty2: Box<Type>,
}
*/

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


*/
