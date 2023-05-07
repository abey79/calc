//! Structures representing the state (as in "state data") of the compiler pipeline.
//!
//! Using the "context" terminology to disambiguate from the "state machine" states.

use crate::context::ast::Ast;
use crate::context::source::Source;
use crate::context::token_stream::TokenStream;
use crate::data::token_span::TokSpan;
use crate::errors::CheckerError;
use crate::pipeline;
use crate::states::CheckedState;

pub struct ParsedState {
    pub(crate) source: Source,
    pub(crate) token_stream: TokenStream,
    pub(crate) raw_ast: Ast<TokSpan>,
}

impl ParsedState {
    pub fn check(self) -> Result<CheckedState, CheckerError> {
        pipeline::checker::check(self)
    }
}
