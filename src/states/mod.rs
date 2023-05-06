pub mod input;

use crate::data::ast::{Block, NodeId};
use crate::data::span::Span;
use crate::data::token::{TokSpan, Token, TokenId};
use crate::errors::TokenizerError;
use crate::pipeline;
use std::collections::BTreeMap;

// =================================================================================================
// CONTEXTS
//
// Contexts are state structures, with "state" as in "state data". I use the term "context" to avoid
// confusion with the "state" in "state machine".

pub struct TextContext(String);

#[derive(Debug, Default)]
pub struct TokenContext {
    pub(crate) tokens: Vec<Token>,
    pub(crate) token_spans: BTreeMap<TokenId, Span>,
}

#[derive(Debug)]
pub struct AstContext {
    pub(crate) nodes: Block,
    pub(crate) node_spans: BTreeMap<NodeId, TokSpan>,
}

// =================================================================================================
// STATES
//
// "State" as in "state machine".
//
// These are global states at various stages of the pipeline. Each state implement functions that
// allows it to be converted to the next state.

pub struct RawInput {
    pub(crate) text_ctx: TextContext,
}

impl RawInput {
    pub fn tokenize(self) -> Result<TokenizedInput, TokenizerError> {
        pipeline::tokenizer::tokenize(self)
    }
}

impl AsRef<str> for RawInput {
    fn as_ref(&self) -> &str {
        &self.text_ctx.0
    }
}

pub struct TokenizedInput {
    pub(crate) text_ctx: TextContext,
    pub(crate) token_ctx: TokenContext,
}

impl TokenizedInput {
    pub fn tokens(&self) -> &[Token] {
        &self.token_ctx.tokens
    }
}

pub struct ParsedInput {
    pub(crate) text_ctx: TextContext,
    pub(crate) token_ctx: TokenContext,
    pub(crate) node_ctx: AstContext,
}
