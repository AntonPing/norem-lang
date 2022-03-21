use rustyline::{Editor, Result};
use rustyline::error::ReadlineError;
use rustyline::validate::{
    MatchingBracketValidator,
    ValidationContext,
    ValidationResult,
    Validator,
};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter};

use crate::symbol::{self, SymTable};
use crate::parser;
use crate::pretty::{self, Print};

#[derive(Completer, Helper, Highlighter, Hinter)]
struct InputValidator {
    brackets: MatchingBracketValidator,
}

impl Validator for InputValidator {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult> {
        self.brackets.validate(ctx)
    }
}

pub fn run_repl() {

    let h = InputValidator {
        brackets: MatchingBracketValidator::new(),
    };
    let mut rl = Editor::new();
    rl.set_helper(Some(h));
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let input = String::from(line);
                command_line(input.as_str());
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
    
}

fn command_line<'src>(input: &'src str) {
    let mut table: SymTable<'src> = symbol::SymTable::new();
    let mut p = parser::Parser::new(
        input, &mut table);
    let mut pp = pretty::PrettyPrinter::new(120,p.table());
    if let Some(expr) = p.read_app() {
        expr.print(&mut pp);
    }
    println!("cmd {}",input);
}