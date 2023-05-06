//! Tokens

use crate::data::identified::{new_id, Identified};

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Name(String),
    Int(i32),
    Float(f64),

    // misc
    Semi,
    Assign,
    LParen,
    RParen,

    // operators
    Plus,
    Minus,
    Star,
    Slash,

    // keywords
    Print,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TokenId(usize);

impl TokenId {
    pub fn new() -> Self {
        Self(new_id())
    }
}

pub type Token = Identified<TokenKind, TokenId>;

impl Token {
    pub fn new(kind: TokenKind) -> Self {
        Self {
            kind,
            id: TokenId::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokSpan {
    pub start: TokenId,
    pub end: TokenId,
}
