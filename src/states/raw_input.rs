//! Model of the raw input state.
//!
//! This is basically just some raw text stored in a `String`.

use crate::data::span::Span;
use crate::errors::error_context::ErrorContext;
use crate::states::{RawInput, TextContext};
use std::io;
use std::io::Read;
use std::path::PathBuf;

impl From<String> for RawInput {
    fn from(text: String) -> Self {
        Self {
            text_ctx: TextContext(text),
        }
    }
}

impl AsRef<str> for RawInput {
    fn as_ref(&self) -> &str {
        &self.text_ctx.0
    }
}

impl RawInput {
    pub fn from_file(path: PathBuf) -> io::Result<Self> {
        let text = std::fs::read_to_string(path)?;
        Ok(Self::from(text))
    }

    pub fn from_stdin() -> io::Result<Self> {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        Ok(Self::from(buffer))
    }

    pub fn source(&self) -> &str {
        &self.text_ctx.0
    }

    pub fn error_context(&self, span: Span) -> ErrorContext {
        if self.source().is_empty() {
            return "".into();
        }

        let extract: String = self
            .source()
            .split('\n')
            .skip(span.start.line - 1)
            .take(span.end.line - span.start.line + 1)
            .enumerate()
            .map(|(i, line)| {
                let cur_line = i + span.start.line;
                let start = if cur_line == span.start.line {
                    span.start.col
                } else {
                    1
                };
                let end = if cur_line == span.end.line {
                    span.end.col
                } else {
                    line.len()
                };

                let underline = " ".repeat(start - 1) + &"^".repeat(end - start + 1);
                format!("{:>4} | {}\n     | {}\n", cur_line, line, underline)
            })
            .collect();

        extract.into()
    }
}
