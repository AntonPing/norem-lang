use crate::utils::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DiagLevel {
    Error,
    Warn,
    Info,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    pub span: Span,
    pub info: String,
}

impl Annotation {
    pub fn new<S: Into<String>>(span: Span, message: S) -> Annotation {
        Annotation {
            span,
            info: message.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub level: DiagLevel,
    pub title: String,
    pub description: Vec<String>,
    pub annos: Vec<Annotation>,
}
/*
trait Logger : Sized {
    fn into_msg(vec: Vec<Self>) -> Diagnostic;
}
*/

impl Diagnostic {
    pub fn error<S: Into<String>>(title: S) -> Diagnostic {
        Diagnostic {
            level: DiagLevel::Error,
            title: title.into(),
            description: Vec::new(),
            annos: Vec::new(),
        }
    }

    pub fn desc<S: Into<String>>(mut self, desc: S) -> Diagnostic {
        self.description.push(desc.into());
        self
    }

    pub fn anno<S: Into<String>>(
        mut self,
        span: Span,
        info: S
    ) -> Diagnostic {
        self.annos.push(Annotation{ span, info: info.into() });
        self
    }

    pub fn report(&self, verbosity: u8, source: &str) -> String {
        if verbosity > 1 {
            self.verbose(source)
        } else {
            self.minimal(source)
        }
    }
    
    pub fn minimal(&self, _: &str) -> String {
        let mut output = format!("{:?}: ", self.level);
        output.push_str(&self.title);
        output.push('\n');

        for line in &self.description {
            output.push_str("  ");
            output.push_str(line);
            output.push('\n');
        }

        for anno in &self.annos {
            if anno.span == Span::dummy() {
                output.push_str(&anno.info);
                
            } else {
                output.push_str(&format!(
                    "from [line {}: col {}] to [line {} : col {}]\n  {}",
                    anno.span.start.row,
                    anno.span.start.col,
                    anno.span.end.row,
                    anno.span.end.col,
                    anno.info,
                ));
            }
            output.push('\n');
        }
        output
    }

    pub fn verbose(&self, source: &str) -> String {
        
        let mut output = format!("{:?}: ", self.level);
        output.push_str(&self.title);
        output.push('\n');

        for line in &self.description {
            output.push_str("  ");
            output.push_str(line);
            output.push('\n');
        }

        let text = source.lines().collect::<Vec<&str>>();

        for anno in &self.annos {
            if anno.span == Span::dummy() {
                output.push_str(&anno.info);
            } else {
                let range = std::ops::Range {
                    start: anno.span.start.row.saturating_sub(1),
                    end: anno.span.end.row + 1,
                };
                let width = (1 + anno.span.end.row).to_string().len();
                for l in range {
                    output.push_str(&format!("{:>.*} | {}\n", width, l + 1, text[l]));
                    if l == anno.span.start.row {
                        let empty = (0..anno.span.start.col + 3 + width)
                            .map(|_| ' ')
                            .collect::<String>();
                        let tilde = (1..anno.span.end.col.saturating_sub(anno.span.start.col))
                            .map(|_| '~')
                            .collect::<String>();
                        output.push_str(&format!("{}^{}^ {}\n", empty, tilde, anno.info))
                    }
                }
            }
            output.push('\n');
        }
        output
    }
    
}
