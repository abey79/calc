//! Error message
//!
//! This is the part of errors that contain the textual representation of the error location,
//! including a source extract and an underline.
//!
//! Built by [`Source::error_message`].

use crate::context::source::Source;
use crate::data::meta::Meta;
use crate::data::span::Span;
use std::fmt;

/// The context of an error
///
/// This is a ready-to-display string with source extract and underline of the error.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ErrorSpan(String);

impl fmt::Display for ErrorSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n\n{}", self.0)
    }
}

impl From<&str> for ErrorSpan {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for ErrorSpan {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// A trait for object that have a span
///
/// A `Spanned` object somehow have a corresponding [`Span`] which can be used to construct an
/// [`ErrorSpan`], which in turn is used to display nice error messages.
pub trait Spanned {
    fn span(&self) -> Span;

    fn to_error(&self, source: &Source) -> ErrorSpan {
        if source.source().is_empty() {
            return "".into();
        }

        let span = self.span();

        let extract: String = source
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

impl<K, M: Spanned> Spanned for Meta<K, M> {
    fn span(&self) -> Span {
        self.meta.span()
    }
}
