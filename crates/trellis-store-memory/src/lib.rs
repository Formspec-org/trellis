// Rust guideline compliant 2026-02-21
//! In-memory store for the append scaffold.

#![forbid(unsafe_code)]

use std::convert::Infallible;

use trellis_core::LedgerStore;
use trellis_types::StoredEvent;

/// Stores appended events in memory for conformance tests.
#[derive(Default, Debug)]
pub struct MemoryStore {
    events: Vec<StoredEvent>,
}

impl MemoryStore {
    /// Creates an empty in-memory store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns all stored events in append order.
    pub fn events(&self) -> &[StoredEvent] {
        &self.events
    }
}

impl LedgerStore for MemoryStore {
    type Error = Infallible;

    fn append_event(&mut self, event: StoredEvent) -> Result<(), Self::Error> {
        self.events.push(event);
        Ok(())
    }
}
