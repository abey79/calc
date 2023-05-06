//! This module models the compiler pipeline states
//!
//! Two types of structures are introduced:
//! - Contexts: these are structures containing state, as in "state data".
//! - States: these are structures capturing the current state (as in "state machine") of the
//!   processed data.
//!
//! To disambiguate both "states", I introduce the term "context" to refer to the former.
//!
//! States can transform into other states thanks to pipeline functions ("steps").
//!
//! For clarity, this file contains only the structure definitions and the very high-level API, such
//! as state transition. Most of the implementation details are in the submodules.

pub mod ast_context;
pub mod raw_input;
pub mod text_context;
pub mod token_context;
pub mod tokenized_input;

use crate::data::ast::{Block, NodeId};
use crate::data::span::Span;
use crate::data::token::{Token, TokenId};
use crate::errors::{ParserError, TokenizerError};
use crate::pipeline;
use std::collections::BTreeMap;
use std::fmt;
use std::rc::Rc;

// =================================================================================================
// CONTEXTS

pub struct TextContext(String);

#[derive(Debug, Default)]
pub struct TokenContext {
    tokens: Vec<Rc<Token>>,
    tokens_by_id: BTreeMap<TokenId, Rc<Token>>,
    token_spans: BTreeMap<TokenId, Span>,
}

#[derive(Debug)]
pub struct AstContext {
    nodes: Block,
    node_spans: BTreeMap<NodeId, TokSpan>,
}

// =================================================================================================
// STATES

pub struct RawInput {
    pub(crate) text_ctx: TextContext,
}

impl RawInput {
    pub fn tokenize(self) -> Result<TokenizedInput, TokenizerError> {
        pipeline::tokenizer::tokenize(self)
    }
}

pub struct TokenizedInput {
    pub(crate) text_ctx: TextContext,
    pub(crate) token_ctx: TokenContext,
}

impl TokenizedInput {
    pub fn parse(self) -> Result<ParsedInput, ParserError> {
        pipeline::parser::parse(self)
    }

    pub fn tokens(&self) -> &[Rc<Token>] {
        &self.token_ctx.tokens
    }
}

pub struct ParsedInput {
    pub(crate) text_ctx: TextContext,
    pub(crate) token_ctx: TokenContext,
    pub(crate) ast_ctx: AstContext,
}

impl ParsedInput {
    pub fn format<W: fmt::Write>(&self, w: &mut W) -> Result<String, fmt::Error> {
        pipeline::formatter::format(self, w)
    }
}

// =================================================================================================
// MISC

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokSpan {
    pub start: TokenId,
    pub end: TokenId,
}

impl TokSpan {
    pub fn new(start: TokenId, end: TokenId) -> Self {
        Self { start, end }
    }
}
