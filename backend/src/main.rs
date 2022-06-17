#![feature(fmt_internals)]
#[macro_use]
extern crate lazy_static;

mod core;
mod symbol;
mod print;
mod lexer;
mod parser;
mod cps_trans;
mod visitor;

fn main() {
    println!("Hello, world!");
}
