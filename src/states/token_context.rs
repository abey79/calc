//! This context stores the tokens and their spans.

use crate::data::span::Span;
use crate::data::token::{Token, TokenId};
use crate::states::TokenContext;

impl TokenContext {
    pub fn tokens(&self) -> &[Token] {
        &self.tokens
    }
    pub fn token_span(&self, id: TokenId) -> Option<Span> {
        self.token_spans.get(&id).copied()
    }

    pub fn push_token(&mut self, token: Token, span: Span) {
        self.token_spans.insert(token.id, span);
        self.tokens.push(token);
    }
}
