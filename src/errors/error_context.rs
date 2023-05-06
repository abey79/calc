//! Error context
//!
//! An error context strictly refers to the data that is used for error display. Currently, this
//! consists of only a string with the relevant highlighted source extract.

use std::fmt;

/// The context of an error
///
/// This is a ready-to-display string with source extract and underline of the error.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ErrorContext(String);

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n\n{}", self.0)
    }
}

impl From<&str> for ErrorContext {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for ErrorContext {
    fn from(s: String) -> Self {
        Self(s)
    }
}
