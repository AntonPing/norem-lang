//mod utils;
mod symbol;
mod ast;
mod pretty;
mod lexer;
mod parser;
mod repl;

fn main() {
    println!("Hello, world!");
    repl::run_repl();
}
