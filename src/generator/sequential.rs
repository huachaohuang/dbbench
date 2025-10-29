use std::sync::atomic::{AtomicU64, Ordering};

use crate::generator::Generator;

pub struct SequentialGenerator {
    current: AtomicU64,
}

impl SequentialGenerator {
    pub fn new() -> Self {
        Self {
            current: AtomicU64::new(0),
        }
    }
}

impl Generator for SequentialGenerator {
    fn next(&self) -> u64 {
        self.current.fetch_add(1, Ordering::Relaxed)
    }
}
