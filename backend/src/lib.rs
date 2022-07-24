use norem_utils::symbol::Symbol;
use norem_utils::position::Span;

pub mod ast;
pub mod visitor;
pub mod dead_elim;
pub mod const_fold;