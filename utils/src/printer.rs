use std::cell::RefCell;
use std::fmt::{self, Display};



/*
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pr = Printer::new(50);
        pr.print_expr(f, self)
    }
}
*/

struct NewLine;
struct Indent;
struct Dedent;

static NEWLINE: NewLine = NewLine;
static INDENT: Indent = Indent;
static DEDENT: Dedent = Dedent;

thread_local! {
    static INDENT_COUNT: RefCell<usize>  = RefCell::new(0);
}

impl Display for NewLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        INDENT_COUNT.with(|n| {
            write!(f, "\n")?;
            for _ in 0..*n.borrow() {
                write!(f, "{}", ' ')?;
            }
            Ok(())
        })
    }
}

impl Display for Indent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        INDENT_COUNT.with(|n| {
            *n.borrow_mut() += 2;
            NEWLINE.fmt(f)
        })
    }
}

impl Display for Dedent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        INDENT_COUNT.with(|n| {
            *n.borrow_mut() -= 2;
            NEWLINE.fmt(f)
        })
    }
}



#[test]
pub fn printer_test() {
    println!("\
        hello{INDENT}\
            hey!{NEWLINE}\
            hey!{NEWLINE}\
            hey!{DEDENT}\
        world{NEWLINE}
    ");
}