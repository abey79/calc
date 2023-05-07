//! This is the state after the tokenization step.

use crate::context::source::Source;
use crate::context::token_stream::TokenStream;
use crate::data::token::{Token, TokenKind};
use crate::errors::ParserError;
use crate::pipeline;
use crate::states::ParsedState;
use std::fmt::Write;
use std::rc::Rc;

pub struct TokenizedState {
    pub(crate) source: Source,
    pub(crate) token_stream: TokenStream,
}

impl TokenizedState {
    pub fn parse(self) -> Result<ParsedState, ParserError> {
        pipeline::parser::parse(self)
    }

    pub fn tokens(&self) -> &[Rc<Token>] {
        self.token_stream.tokens()
    }
}

impl TokenizedState {
    pub fn dump<W: Write>(&self, writer: &mut W) -> Result<(), std::fmt::Error> {
        for token in self.token_stream.tokens() {
            let span = format!("{}", token.span());

            let kind_str = match token.kind {
                TokenKind::Name(ref s) => format!("{:10} {:?}", "Name", s),
                TokenKind::Int(i) => format!("{:10} {}", "Int", i),
                TokenKind::Float(fl) => format!("{:10} {:?}", "Float", fl),
                _ => format!("{:?}", token.kind),
            };

            writeln!(writer, "{:15} {}", span, kind_str)?;
        }

        Ok(())
    }
}
