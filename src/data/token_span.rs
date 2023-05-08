use crate::data::span::Span;
use crate::data::token::Token;
use crate::errors::error_message::Spanned;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct TokSpan {
    pub start: Rc<Token>,
    pub end: Rc<Token>,
}

impl TokSpan {
    pub fn new(start: Rc<Token>, end: Rc<Token>) -> Self {
        Self { start, end }
    }

    pub fn span(&self) -> Span {
        Span::new(self.start.span().start, self.end.span().end)
    }
}

impl fmt::Display for TokSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.span().fmt(f)
    }
}

impl Spanned for TokSpan {
    fn span(&self) -> Span {
        self.span()
    }
}
