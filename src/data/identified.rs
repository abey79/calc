//! Generic type for things that have an ID, such as AST nodes.
//!
//! This type as both a generic type for the "kind" (the content of the structure) and the ID. This
//! ensure that NodeIds are not used to look up, e.g., tokens.

use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Identified<K, I> {
    pub kind: K,
    pub id: I,
}

impl<K, I> AsRef<K> for Identified<K, I> {
    fn as_ref(&self) -> &K {
        &self.kind
    }
}

pub fn new_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}
