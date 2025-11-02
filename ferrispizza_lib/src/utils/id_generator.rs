//! Generates unique incremental IDs for orders, invoices, etc.

use std::sync::atomic::{AtomicU64, Ordering};

/// A thread-safe ID generator.
///
/// Usage:
/// ```
/// use ferrispizza_lib::utils::IdGenerator;
/// let gen1 = IdGenerator::new();
/// assert_eq!(gen1.next_id(), "1");
/// ```
pub struct IdGenerator {
    counter: AtomicU64,
    prefix: Option<String>,
}

impl IdGenerator {
    /// Creates a new generator starting at 1 with no prefix.
    pub const fn new() -> Self {
        Self {
            counter: AtomicU64::new(1),
            prefix: None,
        }
    }

    /// Creates a new generator with a string prefix.  
    /// Example: `IdGenerator::with_prefix("PZ")` → `PZ-1`, `PZ-2`
    pub fn with_prefix(prefix: &str) -> Self {
        Self {
            counter: AtomicU64::new(1),
            prefix: Some(prefix.to_string()),
        }
    }

    /// Generate the next incremental ID as a string.
    pub fn next_id(&self) -> String {
        let value = self.counter.fetch_add(1, Ordering::SeqCst);

        match &self.prefix {
            Some(p) => format!("{}-{}", p, value),
            None => format!("{}", value),
        }
    }

    /// Reset ID counter back to 1 (mainly for tests)
    pub fn reset(&self) {
        self.counter.store(1, Ordering::SeqCst);
    }
}
