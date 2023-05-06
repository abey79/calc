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
