use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::*;
use crate::symbol::*;
use crate::parser::*;
use crate::pretty::*;
use crate::ast::*;


pub struct Checker<'src> {
    table: Rc<RefCell<SymTable<'src>>>,
}


impl<'src> Checker<'src> {
    pub fn check_expr(&mut self, expr: &Expr) -> Result<(),String> {
        match expr {
            _ => unimplemented!()
        }
    }
}
