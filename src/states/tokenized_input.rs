//! This is the state after the tokenization step.

use crate::data::token::TokenKind;
use crate::errors::error_context::ErrorContext;
use crate::states::{TokSpan, TokenizedInput};
use std::fmt::Write;

impl TokenizedInput {
    pub fn dump<W: Write>(&self, writer: &mut W) -> Result<(), std::fmt::Error> {
        for token in self.token_ctx.tokens() {
            let span = if let Some(span) = self.token_ctx.span_from_id(token.id) {
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

    pub fn error_context(&self, tok_span: TokSpan) -> ErrorContext {
        self.text_ctx
            .error_context(self.token_ctx.span_from_tok_span(tok_span))
    }
}
