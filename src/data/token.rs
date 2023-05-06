//! Tokens

use crate::data::identified::{new_id, Identified};
use std::fmt;

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

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;
        match self {
            Name(ref s) => write!(f, "'{}'", s),
            Int(i) => write!(f, "'{}'", i),
            Float(fl) => write!(f, "'{:?}'", fl),
            Semi => write!(f, "';'"),
            Assign => write!(f, "'='"),
            LParen => write!(f, "'('"),
            RParen => write!(f, "')'"),
            Plus => write!(f, "'+'"),
            Minus => write!(f, "'-'"),
            Star => write!(f, "'*'"),
            Slash => write!(f, "'/'"),
            Print => write!(f, "'print'"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TokenId(usize);

impl TokenId {
    pub fn new() -> Self {
        Self(new_id())
    }
}

impl fmt::Display for TokenId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
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
