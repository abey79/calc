//! Tokens

use crate::data::meta::Meta;
use crate::data::span::Span;
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
    Comma,

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
            Comma => write!(f, "','"),
            Plus => write!(f, "'+'"),
            Minus => write!(f, "'-'"),
            Star => write!(f, "'*'"),
            Slash => write!(f, "'/'"),
            Print => write!(f, "'print'"),
        }
    }
}

/// Token type.
///
/// This is an RC pointer to a token kind, as token will be passed around a lot.
pub type Token = Meta<TokenKind, Span>;

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, meta: span }
    }

    pub fn span(&self) -> Span {
        self.meta
    }
}
