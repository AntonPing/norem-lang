
type Symbol = String;
/*
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Lit(LitValue),
    Var(Symbol),
    Cons(Symbol),
    Lam(Vec<Symbol>, Box<Expr>),
    App(Box<Expr>,Vec<Box<Expr>>),
    Let(Vec<Spanned<Decl>>,Spanned<Expr>),
    Case(Spanned<Expr>, Vec<Spanned<Rule>>),
    Ifte(Spanned<Expr>, Spanned<Expr>, Spanned<Expr>),
    //Do(Vec<Statment>,Spanned<Expr>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Lit(LitType),
    Var(Symbol),
    Arr(Ptr<Type>, Ptr<Type>),
    App(Symbol, Vec<Ptr<Type>>),
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

pub fn lit_value_type(val: LitValue) -> LitType {
    match val {
        LitValue::Bool(_) => LitType::Bool,
        LitValue::Char(_) => LitType::Char,
        LitValue::Int(_) => LitType::Int,
        LitValue::Real(_) => LitType::Real,
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Decl {
    Val(Spanned<ValDecl>),
    Data(Spanned<DataDecl>),
    Type(Spanned<TypeDecl>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ValDecl {
    pub name: Symbol,
    pub args: Vec<Symbol>,
    pub body: Expr,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DataDecl {
    pub name: Symbol,
    pub args: Vec<Symbol>,
    pub vars: Vec<Spanned<Variant>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Variant {
    pub cons: Symbol,
    pub args: Vec<Type>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypeDecl {
    pub name: Symbol,
    pub args: Vec<Symbol>,
    pub typ: Type,
}

/*
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Row<T> {
    pub label: Symbol,
    pub data: T,
}
*/

#[derive(Clone, Debug, PartialEq)]
pub enum Pattern {
    /// Algebraic datatype constructor, along with binding pattern
    App(Symbol, Vec<Spanned<Pattern>>),
    /// Constant
    Lit(LitValue),
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
    pub pat: Spanned<Pattern>,
    pub expr: Spanned<Expr>,
}

/*
#[derive(Clone, Debug, PartialEq)]
pub enum Statment {
    Assign(Symbol,Expr),
    Ignore(Expr),
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

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Primitive {
    pub sym: Symbol,
    pub ty: Type,
}
*/

*/
