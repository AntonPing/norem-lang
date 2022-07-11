use std::fmt;
use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum LitVal {
    Int(i64),
    Real(f64),
    Bool(bool),
    Char(char),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum LitType {
    Int,
    Real,
    Bool,
    Char,
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

#[derive(Clone, Debug, PartialEq)]
pub enum Expr<Ident,Extra = ()> {
    Lit(ExprLit<Extra>),
    Prim(ExprPrim<Extra>),
    Var(ExprVar<Ident,Extra>),
    Lam(ExprLam<Ident,Extra>),
    App(ExprApp<Ident,Extra>),
    // chain will be desugared
    Chain(ExprChain<Ident,Extra>),
    Let(ExprLet<Ident,Extra>),
    Case(ExprCase<Ident,Extra>),
    // Block(ExprBlock),
    // Rec(Vec<Row<Expr>>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLit<Extra = ()>  {
    pub lit: LitVal,
    pub ext: Extra,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprPrim<Extra = ()> {
    pub prim: Prim,
    pub ext: Extra,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprVar<Ident, Extra = ()> {
    pub name: Ident,
    pub ext: Extra,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLam<Ident,Extra = ()> {
    pub args: Vec<Ident>,
    pub body: Box<Expr<Ident,Extra>>,
    pub ext: Extra,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprApp<Ident, Extra = ()> {
    pub func: Box<Expr<Ident,Extra>>,
    pub args: Vec<Expr<Ident,Extra>>,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprChain<Ident, Extra = ()> {
    pub head: Box<Expr<Ident,Extra>>,
    pub tail: Vec<(Ident,Expr<Ident,Extra>)>,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprLet<Ident,Extra = ()> {
    pub decls: Vec<Decl<Ident,Extra>>,
    pub body: Box<Expr<Ident,Extra>>,
    pub ext: Extra,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExprCase<Ident,Extra = ()> {
    pub expr: Box<Expr<Ident,Extra>>,
    pub rules: Vec<Rule<Ident,Extra>>,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub enum Decl<Ident,Extra = ()> {
    Val(DeclVal<Ident,Extra>),
    Data(DeclData<Ident,Extra>),
    Type(DeclType<Ident,Extra>),
    Opr(DeclOpr<Ident,Extra>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclVal<Ident,Extra = ()> {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub body: Expr<Ident,Extra>,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclData<Ident,Extra = ()> {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub vars: Vec<Variant<Ident,Extra>>,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variant<Ident,Extra = ()> {
    pub cons: Ident,
    pub args: Vec<Type<Ident>>,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeclType<Ident,Extra = ()> {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub typ: Type<Ident>,
    pub ext: Extra
}

#[derive(Clone, Debug, PartialEq)]
pub struct Rule<Ident,Extra = ()> {
    pub pat: Pattern<Ident>,
    pub body: Expr<Ident,Extra>,
    pub ext: Extra
}


#[derive(Clone, Debug, PartialEq)]
pub struct DeclOpr<Ident,Extra = ()> {
    pub fixity: Fixity,
    pub prec: u8,
    pub name: Ident,
    pub func: Ident,
    pub ext: Extra
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub enum Fixity {
    Infixl,
    Infixr,
    Nonfix,
}

/*
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
*/

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern<Ident> {
    /// Constant
    Lit(LitVal),
    /// Variable binding
    Var(Ident),
    /// Algebraic datatype constructor, along with binding pattern
    App(Ident, Vec<Pattern<Ident>>),
    /// List pattern [pat1, ... patN]
    // List(Vec<Pat>),
    /// Record pattern { label1, label2 }, and whether it's flexible or not
    // Record(Vec<Row<Pat>>, bool),
    Wild,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type<Ident> {
    Lit(LitType),
    Var(Ident),
    Arr(Box<Type<Ident>>, Box<Type<Ident>>),
    App(Box<Type<Ident>>, Box<Type<Ident>>),
    // Rec(Vec<Row<Type<Ident>>>),
    // Temp(usize),
}
/*
#[derive(Clone, Debug, PartialEq)]
pub struct Row<T> {
    pub name: Symbol,
    pub data: Box<T>,
    pub ext: Extra
}
*/

#[derive(Debug, Clone, PartialEq)]
pub enum Scheme<Ident> {
    Mono(Type<Ident>),
    Poly(Vec<Ident>, Type<Ident>),
}
/*
impl Expr {
    pub fn is_prim(&self) -> bool {
        if let Expr::Prim(_) = self {
            true
        } else {
            false
        }
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
