use crate::data::token::TokenId;

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
