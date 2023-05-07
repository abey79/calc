//! This is the state after the tokenization step.

use crate::context::source::Source;
use crate::context::token_stream::TokenStream;
use crate::data::token::{Token, TokenKind};
use crate::data::token_span::TokSpan;
use crate::errors::error_message::ErrorMessage;
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
            let span = if let Some(span) = self.token_stream.span_from_id(token.id) {
                format!("{}", span)
            } else {
                "???".to_string()
            };

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

    pub fn error_context(&self, tok_span: TokSpan) -> ErrorMessage {
        self.source
            .error_message(self.token_stream.span_from_tok_span(tok_span))
    }
}
