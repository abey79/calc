//! Structures representing the state (as in "state data") of the compiler pipeline.
//!
//! Using the "context" terminology to disambiguate from the "state machine" states.

use crate::context::ast::Ast;
use crate::context::source::Source;
use crate::context::token_stream::TokenStream;
use crate::errors::CheckerError;
use crate::pipeline;
use crate::states::CheckedState;
use std::fmt;

pub struct ParsedState {
    pub(crate) source: Source,
    pub(crate) token_stream: TokenStream,
    pub(crate) ast: Ast,
}

impl ParsedState {
    pub fn format<W: fmt::Write>(&self, w: &mut W) -> Result<String, fmt::Error> {
        pipeline::formatter::format(self, w)
    }

    pub fn check(self) -> Result<CheckedState, CheckerError> {
        pipeline::checker::check(self)
    }
}
