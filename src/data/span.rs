//! A span is a range withing a text stream.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Loc {
    pub line: usize,
    pub col: usize,
}

impl Default for Loc {
    fn default() -> Self {
        Self { line: 1, col: 0 }
    }
}

impl Loc {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

/// A span of source code
///
/// Spans are inclusive of both start and end.
#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub start: Loc,
    pub end: Loc,
}

impl Span {
    pub fn new(start: Loc, end: Loc) -> Self {
        Self { start, end }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.start.line, self.start.col)?;

        if self.start.line == self.end.line {
            write!(f, "-{}", self.end.col)
        } else {
            write!(f, "-{}:{}", self.end.line, self.end.col)
        }
    }
}
