//! This context stores the tokens and their spans.

use crate::data::token::Token;
use std::rc::Rc;

//TODO: this has become somewhat ridiculously small, maybe TokenStream = Vec<Token>

#[derive(Debug, Default)]
pub struct TokenStream {
    tokens: Vec<Rc<Token>>,
}

impl TokenStream {
    pub fn tokens(&self) -> &[Rc<Token>] {
        &self.tokens
    }

    pub fn push_token(&mut self, token: Token) {
        self.tokens.push(Rc::new(token));
    }
}
