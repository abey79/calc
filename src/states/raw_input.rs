//! Model of the raw input state.
//!
//! This is basically just some raw text stored in a `String`.

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
}
