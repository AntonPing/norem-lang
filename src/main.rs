pub mod utils;
pub mod symbol;
pub mod ast;
pub mod pretty;
pub mod lexer;
pub mod parser;
pub mod check;
pub mod infer;
pub mod repl;

fn main() {
    println!("Hello, world!");
    repl::run_repl();
}
