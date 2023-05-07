//! This context stores the tokens and their spans.

use crate::data::span::Span;
use crate::data::token::{Token, TokenId};
use crate::data::token_span::TokSpan;
use std::collections::BTreeMap;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct TokenStream {
    tokens: Vec<Rc<Token>>,
    tokens_by_id: BTreeMap<TokenId, Rc<Token>>,
    token_spans: BTreeMap<TokenId, Span>,
}

impl TokenStream {
    pub fn tokens(&self) -> &[Rc<Token>] {
        &self.tokens
    }

    pub fn token_from_id(&self, id: TokenId) -> Option<&Token> {
        self.tokens_by_id.get(&id).map(|t| &**t)
    }

    pub fn span_from_id(&self, id: TokenId) -> Option<Span> {
        self.token_spans.get(&id).copied()
    }

    pub fn push_token(&mut self, token: Token, span: Span) {
        self.token_spans.insert(token.id, span);
        let token = Rc::new(token);
        self.tokens.push(token.clone());
        self.tokens_by_id.insert(token.id, token);
    }

    pub fn span_from_tok_span(&self, tok_span: TokSpan) -> Option<Span> {
        let start = self.span_from_id(tok_span.start)?;
        let end = self.span_from_id(tok_span.end)?;
        Some(Span::new(start.start, end.end))
    }
}
