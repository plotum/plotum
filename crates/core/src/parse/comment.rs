use crate::error::{ParseError, Span};

#[derive(Default)]
pub(crate) struct Comments {
    open: Option<Span>,
}

impl Comments {
    pub(crate) fn skip(&mut self, line: &str, line_number: usize) -> bool {
        let mut text = line.trim_start();

        if self.open.is_none() && text.starts_with("/*") {
            self.open = Some(Span {
                line: line_number,
                column: line.chars().count() - text.chars().count() + 1,
                length: 2,
            });
            text = &text[2..];
        }

        if self.open.is_some() {
            if text.contains("*/") {
                self.open = None;
            }
            return true;
        }

        text.starts_with("//")
    }

    pub(crate) fn finish(self) -> Result<(), ParseError> {
        match self.open {
            Some(span) => Err(ParseError::UnclosedComment { span }),
            None => Ok(()),
        }
    }
}
