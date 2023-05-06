//! This is the state after the tokenization step.

use crate::data::token::{Token, TokenKind};
use crate::states::TokenizedInput;
use std::fmt::Write;

impl TokenizedInput {
    pub fn dump<W: Write>(&self, writer: &mut W) -> Result<(), std::fmt::Error> {
        for Token { kind: token, id } in self.token_ctx.tokens() {
            let span = if let Some(span) = self.token_ctx.token_span(*id) {
                format!("{}", span)
            } else {
                "???".to_string()
            };

            let kind = match token {
                TokenKind::Name(ref s) => format!("{:10} {:?}", "Name", s),
                TokenKind::Int(i) => format!("{:10} {}", "Int", i),
                TokenKind::Float(fl) => format!("{:10} {:?}", "Float", fl),
                _ => format!("{:?}", token),
            };

            writeln!(writer, "{:15} {}", span, kind)?;
        }

        Ok(())
    }
}
