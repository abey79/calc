//! Generic type for things that have an ID, such as AST nodes.
//!
//! This type as both a generic type for the "kind" (the content of the structure) and the ID. This
//! ensure that NodeIds are not used to look up, e.g., tokens.

use std::fmt;
use std::fmt::{Debug, Display};

#[derive(Clone, PartialEq, Default)]
pub struct Meta<K, I> {
    pub kind: K,
    pub meta: I,
}

impl<K: Display, I> Display for Meta<K, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.kind.fmt(f)
    }
}

impl<K: Debug, I: Display> Debug for Meta<K, I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] ", self.meta)?;
        self.kind.fmt(f)
    }
}

impl<K, I> AsRef<K> for Meta<K, I> {
    fn as_ref(&self) -> &K {
        &self.kind
    }
}
