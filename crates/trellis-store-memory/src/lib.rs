// Rust guideline compliant 2026-02-21
//! In-memory `LedgerStore` for the Phase-1 append scaffold and conformance corpus.
//!
//! Provides a parity surface to [`trellis_store_postgres`]: a buffered
//! "transaction" type and an [`append_event_in_tx`] free function so
//! conformance / cross-store parity tests can target both stores through
//! the same composition shape (canonical write + caller-supplied side
//! effects atomically committed or discarded).

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

    /// Begins a buffered "transaction" against this store.
    ///
    /// The returned [`MemoryTransaction`] buffers events; callers issue
    /// [`append_event_in_tx`] against it (and optionally their own side
    /// effects), then call [`MemoryTransaction::commit`] to flush. Drop
    /// without commit discards the buffer — the parity surface for
    /// `trellis_store_postgres::append_event_in_tx` rollback semantics.
    pub fn begin(&mut self) -> MemoryTransaction<'_> {
        MemoryTransaction {
            store: self,
            buffered: Vec::new(),
            committed: false,
        }
    }
}

/// Buffered transaction over [`MemoryStore`].
///
/// On [`Self::commit`] the buffered events flush to the store; on drop
/// without commit the buffer discards. Mirrors the rollback-on-drop
/// semantic of `postgres::Transaction`.
pub struct MemoryTransaction<'a> {
    store: &'a mut MemoryStore,
    buffered: Vec<StoredEvent>,
    committed: bool,
}

impl<'a> MemoryTransaction<'a> {
    /// Number of events buffered (not yet committed).
    pub fn buffered_len(&self) -> usize {
        self.buffered.len()
    }

    /// Commits buffered events to the underlying store.
    ///
    /// Returns `Result<(), Infallible>` for parity with
    /// `postgres::Transaction::commit() -> Result<(), postgres::Error>`:
    /// cross-store generic test bodies can share `tx.commit()?` across both
    /// adapters. The `Infallible` arm encodes statically that this in-memory
    /// path cannot fail; callers can `.unwrap()` or `?`-chain identically.
    ///
    /// # Errors
    /// Never fails. The `Result` shape is for parity only.
    pub fn commit(mut self) -> Result<(), Infallible> {
        self.store.events.extend(std::mem::take(&mut self.buffered));
        self.committed = true;
        Ok(())
    }
}

impl Drop for MemoryTransaction<'_> {
    fn drop(&mut self) {
        // Uncommitted buffer drops on the floor — explicit rollback semantic.
        if !self.committed {
            self.buffered.clear();
        }
    }
}

/// Appends one event into the supplied buffered transaction.
///
/// Parity counterpart to `trellis_store_postgres::append_event_in_tx` so
/// cross-store composition tests can write a single generic body. The
/// `idempotency_key` parameter is accepted for shape parity; memory
/// enforces only the Core §6.1 length bound and reports a duplicate
/// `(scope, key)` collision through [`MemoryAppendError::IdempotencyKeyConflict`].
///
/// # Errors
/// - [`MemoryAppendError::IdempotencyKeyTooLong`] when `key` violates the
///   Core §6.1 `bstr .size (1..64)` bound.
/// - [`MemoryAppendError::IdempotencyKeyConflict`] when `(scope, key)`
///   collides with a previously-committed event in the underlying store
///   or a buffered event in the same transaction.
pub fn append_event_in_tx(
    tx: &mut MemoryTransaction<'_>,
    event: &StoredEvent,
    idempotency_key: Option<&[u8]>,
) -> Result<(), MemoryAppendError> {
    if let Some(key) = idempotency_key {
        if key.is_empty() || key.len() > IDEMPOTENCY_KEY_MAX_LEN {
            return Err(MemoryAppendError::IdempotencyKeyTooLong(key.len()));
        }
        let collides_in_store = tx
            .store
            .events
            .iter()
            .any(|stored| stored.scope() == event.scope() && stored_key(stored) == Some(key));
        let collides_in_buffer = tx
            .buffered
            .iter()
            .any(|buffered| buffered.scope() == event.scope() && stored_key(buffered) == Some(key));
        if collides_in_store || collides_in_buffer {
            return Err(MemoryAppendError::IdempotencyKeyConflict);
        }
    }

    // `StoredEvent::idempotency_key` now carries the parsed Core §6.1 / §17.2
    // wire-contract identity through (item #2 closed Wave 24). The collision
    // check above is therefore live — both `stored_key` and the buffered
    // events return the threaded value.
    tx.buffered.push(event.clone());
    Ok(())
}

/// Maximum byte length of `idempotency_key` per Core §6.1 / §17.2.
pub const IDEMPOTENCY_KEY_MAX_LEN: usize = 64;

/// Failure cases for [`append_event_in_tx`].
#[derive(Debug, PartialEq, Eq)]
pub enum MemoryAppendError {
    /// `idempotency_key` length outside the Core §6.1 bound.
    IdempotencyKeyTooLong(usize),
    /// Same `(scope, idempotency_key)` already appended.
    IdempotencyKeyConflict,
}

impl std::fmt::Display for MemoryAppendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdempotencyKeyTooLong(n) => {
                write!(
                    f,
                    "idempotency_key length {n} outside Core §6.1 bound 1..=64"
                )
            }
            Self::IdempotencyKeyConflict => write!(
                f,
                "Core §17.3 idempotency-key conflict on (ledger_scope, idempotency_key)"
            ),
        }
    }
}

impl std::error::Error for MemoryAppendError {}

/// Reads the Core §6.1 `idempotency_key` from the threaded `StoredEvent`.
fn stored_key(stored: &StoredEvent) -> Option<&[u8]> {
    stored.idempotency_key()
}

impl LedgerStore for MemoryStore {
    type Error = MemoryAppendError;

    fn append_event(&mut self, event: StoredEvent) -> Result<(), Self::Error> {
        // Compose the `LedgerStore` trait method through the buffered-tx
        // surface so the §6.1 length bound and the §17.3 unique-`(scope,
        // key)` collision check both fire against the same code path
        // exercised by `append_event_in_tx`.
        let key = event.idempotency_key().map(<[u8]>::to_vec);
        let mut tx = self.begin();
        append_event_in_tx(&mut tx, &event, key.as_deref())?;
        // `MemoryTransaction::commit` returns `Result<(), Infallible>`;
        // the `?` here is structurally unreachable but kept for parity with
        // `trellis-store-postgres`.
        let _ = tx.commit();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_append_event_path_persists() {
        let mut store = MemoryStore::new();
        store
            .append_event(StoredEvent::new(
                b"scope".to_vec(),
                0,
                vec![0x01],
                vec![0x02],
            ))
            .unwrap();
        assert_eq!(store.events().len(), 1);
    }

    #[test]
    fn buffered_transaction_commits_events() {
        let mut store = MemoryStore::new();
        let mut tx = store.begin();
        append_event_in_tx(
            &mut tx,
            &StoredEvent::new(b"scope".to_vec(), 0, vec![0x01], vec![0x02]),
            None,
        )
        .unwrap();
        assert_eq!(tx.buffered_len(), 1);
        tx.commit().unwrap();
        assert_eq!(store.events().len(), 1);
    }

    #[test]
    fn buffered_transaction_rolls_back_on_drop() {
        let mut store = MemoryStore::new();
        {
            let mut tx = store.begin();
            append_event_in_tx(
                &mut tx,
                &StoredEvent::new(b"scope".to_vec(), 0, vec![0x01], vec![0x02]),
                None,
            )
            .unwrap();
            // Drop without commit — events MUST NOT land.
        }
        assert!(store.events().is_empty(), "uncommitted tx leaked events");
    }

    /// Pins the parity rationale for the `Result<(), Infallible>` return:
    /// a generic test body can `?`-chain `tx.commit()` and the shape works
    /// against both this adapter and `trellis-store-postgres` whose
    /// `Transaction::commit` returns `Result<(), postgres::Error>`.
    #[test]
    fn commit_supports_question_mark_chaining() {
        fn drive(store: &mut MemoryStore) -> Result<(), Infallible> {
            let mut tx = store.begin();
            append_event_in_tx(
                &mut tx,
                &StoredEvent::new(b"scope".to_vec(), 0, vec![0x01], vec![0x02]),
                None,
            )
            .unwrap();
            tx.commit()?;
            Ok(())
        }

        let mut store = MemoryStore::new();
        drive(&mut store).unwrap();
        assert_eq!(store.events().len(), 1);
    }

    #[test]
    fn idempotency_key_too_long_rejected() {
        let mut store = MemoryStore::new();
        let mut tx = store.begin();
        let oversize = vec![0u8; IDEMPOTENCY_KEY_MAX_LEN + 1];
        let err = append_event_in_tx(
            &mut tx,
            &StoredEvent::new(b"s".to_vec(), 0, vec![], vec![]),
            Some(&oversize),
        )
        .unwrap_err();
        assert_eq!(err, MemoryAppendError::IdempotencyKeyTooLong(65));

        let empty: Vec<u8> = Vec::new();
        let err = append_event_in_tx(
            &mut tx,
            &StoredEvent::new(b"s".to_vec(), 0, vec![], vec![]),
            Some(&empty),
        )
        .unwrap_err();
        assert_eq!(err, MemoryAppendError::IdempotencyKeyTooLong(0));
    }
}
