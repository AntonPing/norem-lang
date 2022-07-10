use std::fmt::{self, write};
use core::fmt::Display;

use norem_utils::position::Span;


#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum LitVal {
    Int(i64),
    Real(f64),
    Bool(bool),
    Char(char),
}

impl Display for LitVal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LitVal::Int(x) => write!(f, "{}", x),
            LitVal::Real(x) => write!(f, "{}", x),
            LitVal::Bool(x) => write!(f, "{}", x),
            LitVal::Char(x) => write!(f, "{}", x),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum LitType {
    Int,
    Real,
    Bool,
    Char,
}

impl Display for LitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
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

impl Display for Prim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<Ident,Extra = ()> {
    Lit(ExprLit<Extra>),
    Prim(ExprPrim<Extra>),
    Var(ExprVar<Ident,Extra>),
    Lam(ExprLam<Ident,Extra>),
    App(ExprApp<Ident,Extra>),
    // chain will be desugared
    // Chain(ExprChain),
    // Let(ExprLet),
    // Case(ExprCase),
    // Block(ExprBlock),
    // Rec(Vec<Row<Expr>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLit<Extra>  {
    pub lit: LitVal,
    pub ext: Extra,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprPrim<Extra> {
    pub prim: Prim,
    pub ext: Extra,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprVar<Ident, Extra> {
    pub name: Ident,
    pub ext: Extra,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLam<Ident,Extra> {
    pub args: Vec<Ident>,
    pub body: Box<Expr<Ident,Extra>>,
    pub ext: Extra,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprApp<Ident, Extra> {
    pub func: Box<Expr<Ident,Extra>>,
    pub args: Vec<Expr<Ident,Extra>>,
    pub ext: Extra
}
/*
#[derive(Clone, Debug, PartialEq)]
pub struct ExprChain {
    pub head: Box<Expr>,
    pub tail: Vec<(Symbol,Expr)>,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLet {
    pub decls: Vec<Decl>,
    pub body: Box<Expr>,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprCase {
    pub expr: Box<Expr>,
    pub rules: Vec<Rule>,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub enum Decl {
    Val(DeclVal),
    Data(DeclData),
    Type(DeclType),
    Opr(DeclOpr),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclVal {
    pub name: Symbol,
    pub args: Vec<Symbol>,
    pub body: Expr,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclData {
    pub name: Symbol,
    pub args: Vec<Symbol>,
    pub vars: Vec<Variant>,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variant {
    pub cons: Symbol,
    pub args: Vec<Type>,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclType {
    pub name: Symbol,
    pub args: Vec<Symbol>,
    pub typ: Type,
    pub ext: Extra
}

/// Rule of a case expression
#[derive(Clone, Debug, PartialEq)]
pub struct Rule {
    pub pat: Pattern,
    pub body: Expr,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclOpr {
    pub name: Symbol,
    pub fixity: Fixity,
    pub prec: u8,
    pub ext: Extra
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Fixity {
    Infixl,
    Infixr,
    Nonfix,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprBlock {
    pub stats: Vec<Stat>, 
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub enum Stat {
    // let foo = bar;
    Let(StatLet),
    // let foo <- bar;
    Bind(StatBind),
    // bar;
    Drop(StatDrop),
    // return foo;
    Ret(StatRet),
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatLet {
    pub name: Symbol,
    pub body: Box<Expr>,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatBind {
    pub name: Symbol,
    pub body: Box<Expr>,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatDrop {
    pub body: Box<Expr>,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub struct StatRet {
    pub body: Box<Expr>,
    pub ext: Extra
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





#[derive(Debug, Clone, PartialEq)]
pub enum Type<Ident> {
    Lit(LitType),
    Var(Ident),
    Arr(Box<Type<Ident>>, Box<Type<Ident>>),
    App(Box<Type<Ident>>, Box<Type<Ident>>),
    Rec(Vec<Row<Type<Ident>>>),
    Temp(usize),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Row<T> {
    pub name: Symbol,
    pub data: Box<T>,
    pub ext: Extra
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

*/

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
    pub ext: Extra
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Fixity {
    Infixl,
    Infixr,
    Nonfix,
}


*/
