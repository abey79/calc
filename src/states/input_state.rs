//! Model of the raw input state.
//!
//! This is basically just some raw text stored in a `String`.

use crate::context::source::Source;

use crate::errors::TokenizerError;
use crate::pipeline;
use crate::states::TokenizedState;
use std::io;
use std::io::Read;
use std::path::PathBuf;

pub struct InputState {
    pub(crate) source: Source,
}

impl InputState {
    pub fn tokenize(self) -> Result<TokenizedState, TokenizerError> {
        pipeline::tokenizer::tokenize(self)
    }
}

impl From<String> for InputState {
    fn from(text: String) -> Self {
        Self {
            source: Source::new(text),
        }
    }
}

impl AsRef<str> for InputState {
    fn as_ref(&self) -> &str {
        self.source.source()
    }
}

impl InputState {
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
        self.source.source()
    }
}
