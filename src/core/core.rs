use crate::common::lit::{LitType, LitVal};
use crate::common::name::Name;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Expr {
    Lit {
        lit: LitVal,
    },
    /// x
    Var {
        var: Name,
    },
    /// let x = E1 in E2
    Let {
        bind: Name,
        expr: Box<Expr>,
        cont: Box<Expr>,
    },
    /// fn(x1: T1, ..., xn: Tn) => E
    Func {
        pars: Vec<(Name, Type)>,
        body: Box<Expr>,
    },
    /// E0(E1, ..., En)
    App {
        func: Box<Expr>,
        args: Vec<Expr>,
    },
    /// (E1, ..., En)
    Tup {
        flds: Vec<Expr>,
    },
    /// E.i
    Sel {
        expr: Box<Expr>,
        idx: usize,
    },
    /// letrec D1 ... Dn in E end
    Letrec {
        decls: Vec<Decl>,
        cont: Box<Expr>,
    },
    /// E[T1, ..., Tn]
    Inst {
        expr: Box<Expr>,
        typs: Vec<Type>,
    },
    /// pack E as [X1=T1, ..., Xn=Tn](U1, ..., Un)
    Pack {
        expr: Box<Expr>,
        seals: Vec<(Name, Type)>,
        flds: Vec<Type>,
    },
    /// unpack x[X1, ..., Xn] = E1 in E2
    Unpack {
        bind: Name,
        opens: Vec<Name>,
        expr: Box<Expr>,
        cont: Box<Expr>,
    },
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub enum Type {
    Lit {
        lit: LitType,
    },
    /// x
    Var {
        var: Name,
    },
    /// fn(T1, ..., Tn) -> U
    Func {
        pars: Vec<Type>,
        res: Box<Type>,
    },
    /// (T1, ..., Tn)
    Tup {
        flds: Vec<Type>,
    },
    /// fn[X1, ..., Xn](T1, ..., Tn) -> U
    Forall {
        gens: Vec<Name>,
        pars: Vec<Type>,
        res: Box<Type>,
    },
    /// [X1, ..., Xn](T1, ..., Tn)
    Exist {
        seals: Vec<Name>,
        flds: Vec<Type>,
    },
}

/// function f[X1, ..., Xn](x1: T1, ..., xn: Tn) -> U
/// begin
///     E
/// end
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Decl {
    pub name: Name,
    pub gens: Option<Vec<Name>>,
    pub pars: Vec<(Name, Type)>,
    pub res: Type,
    pub body: Expr,
}

/// toplevel program
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Program {
    pub decls: Vec<Decl>,
}