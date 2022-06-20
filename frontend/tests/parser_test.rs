use std::{cell::RefCell, rc::Rc};

use norem_frontend::parser::*;
use norem_frontend::symbol::*;

#[test]
pub fn parser_func() {
    let string = "fn f g x => f x (g x)";
    let table = Rc::new(RefCell::new(SymTable::new()));
    let mut par = Parser::new(string, table);

    let expr = par.read_app();
    assert!(expr.is_some());
    println!("{:?}", expr);
}

#[test]
pub fn parse_case() {
    let string = "
        case Red of
        | Red => 1
        | Green => 2
        | Blue => 3
    ";

    let table = Rc::new(RefCell::new(SymTable::new()));
    let mut par = Parser::new(string, table);
    let expr = par.read_app();
    assert!(expr.is_some());
    println!("{:?}", expr);
}
#[test]
pub fn parse_let_data() {
    let string = "
        let data RGB = Red | Green | Blue
        in 1 end
    ";

    let table = Rc::new(RefCell::new(SymTable::new()));
    let mut par = Parser::new(string, table);
    let expr = par.read_app();
    assert!(expr.is_some());
    println!("{:?}", expr);
}

#[test]
pub fn parser_let_many() {
    let string = "
        let
            val x = 42
            data RGB = Red | Green | Blue
            type Color = RGC
        in
            case color of
            | Red => 1
            | Green => 2
            | Blue => 3
        end
    ";

    let table = Rc::new(RefCell::new(SymTable::new()));
    let mut par = Parser::new(string, table);
    let expr = par.read_app();
    assert!(expr.is_some());
    println!("{:?}", expr);
}
