//! Error message
//!
//! This is the part of errors that contain the textual representation of the error location,
//! including a source extract and an underline.
//!
//! Built by [`Source::error_message`].

use std::fmt;

/// The context of an error
///
/// This is a ready-to-display string with source extract and underline of the error.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ErrorMessage(String);

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n\n{}", self.0)
    }
}

impl From<&str> for ErrorMessage {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for ErrorMessage {
    fn from(s: String) -> Self {
        Self(s)
    }
}
