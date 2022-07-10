use super::position::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DiagLevel {
    Error,
    Warn,
    Info,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Description {
    verbosity: u8,
    message: String,
    span: Option<Span>,
}

impl Description {
    pub fn message<S: Into<String>>(msg: S) -> Description {
        Description {
            verbosity: 1,
            message: msg.into(),
            span: None,
        }
    }

    pub fn with_span<S: Into<String>>(
        span: Span,
        msg: S
    ) -> Description {
        Description {
            verbosity: 1,
            message: msg.into(),
            span: Some(span),
        }
    }

}

#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    level: DiagLevel,
    title: String,
    descriptions: Vec<Description>,
}

impl Diagnostic {
    pub fn error<S: Into<String>>(title: S) -> Diagnostic {
        Diagnostic {
            level: DiagLevel::Error,
            title: title.into(),
            descriptions: Vec::new(),
        }
    }

    pub fn line<S: Into<String>>(mut self, msg: S) -> Diagnostic {
        self.descriptions.push(Description::message(msg.into()));
        self
    }

    pub fn span<S: Into<String>>(
        mut self,
        span: Span,
        msg: S,
    ) -> Diagnostic {
        self.descriptions.push(Description::with_span(span, msg.into()));
        self
    }

    pub fn report(&self, source: &str, verbosity: u8) -> String {
        if verbosity == 0 {
            self.minimal_report()
        } else {
            self.normal_report(source, verbosity)
        }
    }
    
    pub fn minimal_report(&self) -> String {
        let mut output = format!("{:?}: {}\n", self.level, &self.title);

        for descr in &self.descriptions {
            if descr.verbosity > 1 {
                // ignore those description with higher than 1
                continue;
            }
            match &descr.span {
                Some(span) => {
                    output.push_str(&format!(
                        "from [line {}: col {}] to [line {} : col {}]\n{}\n",
                        span.start.row,
                        span.start.col,
                        span.end.row,
                        span.end.col,
                        descr.message,
                    ));
                }
                None => {
                    output.push_str(&descr.message);
                    output.push('\n');
                }
            }
        }
        output
    }

    pub fn normal_report(&self, source: &str, verbosity: u8) -> String {
        let mut output = format!("{:?}: {}\n", self.level, &self.title);

        let text = source.lines().collect::<Vec<&str>>();

        for descr in &self.descriptions {
            if descr.verbosity > verbosity {
                // ignore those description with higher verbosity
                continue;
            }
            match &descr.span {
                Some(span) => {
                    let row_range = std::ops::Range {
                        start: span.start.row,
                        end: span.end.row + 1,
                    };
                    //println!("range = {:?}",range);
                    let head_width = (1 + span.end.row).to_string().len();
    
                    for row in row_range {
                        // print header "xxx | ", where xxx is the line number
                        output.push_str(
                            &format!("{:>.*} | {}\n",
                            head_width, row + 1, text[row]
                        ));
    
                        if row == span.start.row {
                            let empty = (0..span.start.col + head_width + 3)
                                .map(|_| ' ')
                                .collect::<String>();
                            let tilde = (2..span.end.col.saturating_sub(span.start.col))
                                .map(|_| '~')
                                .collect::<String>();
                            output.push_str(&format!("{}^{}^ {}\n", empty, tilde, descr.message))
                        }
                    }
                }
                None => {
                    output.push_str(&descr.message);
                    output.push('\n');
                }
            }
        }
        output
    }
}
