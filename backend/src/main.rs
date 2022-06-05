#[macro_use]
extern crate lazy_static;

mod core;
mod symbol;
mod pretty;
mod lexer;
mod parser;
mod cps_trans;
mod visitor;

fn main() {
    println!("Hello, world!");
}
