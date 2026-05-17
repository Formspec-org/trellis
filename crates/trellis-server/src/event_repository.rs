// Rust guideline compliant 2026-02-21
//! Durable and in-memory [`EventRepository`] implementations for the Trellis HTTP service.

use std::collections::HashMap;

use async_trait::async_trait;
use sqlx::PgPool;
use stack_common_error::StackError;
use tokio::sync::Mutex;
use trellis_server_ports::ArtifactRef;
use trellis_store_postgres_async::{
    AppendError, BundlePublicationError, BundlePublicationIdentity, BundlePublicationRecord,
};
use trellis_types::StoredEvent;

use crate::artifacts::{BundleIdentity, BundleIndexPort, BundleRecord};

/// Core §17.3 identity for idempotent retries: same canonical + signed bytes, independent of
/// optional `canonical_event_hash` materialization on [`StoredEvent`].
fn same_event_bytes(left: &StoredEvent, right: &StoredEvent) -> bool {
    left.canonical_event() == right.canonical_event() && left.signed_event() == right.signed_event()
}

fn map_postgres_append_error(error: AppendError) -> StackError {
    match error {
        AppendError::IdempotencyKeyPayloadMismatch | AppendError::PkCollisionMismatch => {
            StackError::conflict(format!("trellis append rejected: {error}"))
        }
        AppendError::SequenceGap(_) => {
            StackError::conflict(format!("trellis append rejected: {error}"))
        }
        AppendError::IdempotencyKeyTooLong(len) => StackError::internal(format!(
            "idempotency key length invariant violated at append: {len}"
        )),
        AppendError::DomainViolation(seq) => StackError::internal(format!(
            "sequence does not fit store domain at append: {seq}"
        )),
        AppendError::Sqlx(sqlx_err) => {
            if let Some(db_err) = sqlx_err.as_database_error()
                && db_err.code().as_deref() == Some("23505")
            {
                return StackError::conflict(format!("trellis append rejected: {sqlx_err}"));
            }
            StackError::unavailable(format!("trellis append failed: {sqlx_err}"))
        }
    }
}

fn map_postgres_bundle_error(error: BundlePublicationError) -> StackError {
    match error {
        BundlePublicationError::Conflict(message) => {
            StackError::conflict(format!("trellis bundle publication rejected: {message}"))
        }
        BundlePublicationError::DomainViolation(seal_version) => StackError::internal(format!(
            "seal version does not fit store domain at bundle publication: {seal_version}"
        )),
        BundlePublicationError::Sqlx(sqlx_err) => {
            if let Some(db_err) = sqlx_err.as_database_error()
                && db_err.code().as_deref() == Some("23505")
            {
                return StackError::conflict(format!(
                    "trellis bundle publication rejected: {sqlx_err}"
                ));
            }
            StackError::unavailable(format!("trellis bundle publication failed: {sqlx_err}"))
        }
    }
}

fn publication_identity(scope: &[u8], identity: &BundleIdentity) -> BundlePublicationIdentity {
    BundlePublicationIdentity {
        scope: scope.to_vec(),
        checkpoint_digest: identity.checkpoint_digest.clone(),
        seal_version: identity.seal_version,
        export_attempt_id: identity.export_attempt_id.clone(),
    }
}

fn publication_record(scope: &[u8], record: &BundleRecord) -> BundlePublicationRecord {
    BundlePublicationRecord {
        identity: publication_identity(scope, &record.identity()),
        artifact_ref: record.artifact_ref.uri.clone(),
    }
}

fn bundle_record_from_publication(record: BundlePublicationRecord) -> BundleRecord {
    BundleRecord {
        checkpoint_digest: record.identity.checkpoint_digest,
        seal_version: record.identity.seal_version,
        export_attempt_id: record.identity.export_attempt_id,
        artifact_ref: ArtifactRef::new(record.artifact_ref),
    }
}

/// Durable event repository used by the service composition root.
#[async_trait]
pub trait EventRepository: Send + Sync {
    /// Lists committed events for `scope` in ascending sequence order.
    ///
    /// # Errors
    ///
    /// Returns [`StackError::unavailable`] when a durable store read fails.
    async fn list_scope(&self, scope: &[u8]) -> Result<Vec<StoredEvent>, StackError>;

    /// Persists one ledger event as the next sequence for its scope.
    ///
    /// # Errors
    ///
    /// Returns [`StackError::conflict`] when idempotency, uniqueness, or chain
    /// invariants reject the append. Returns [`StackError::unavailable`] when the
    /// store cannot complete I/O.
    async fn append_event(&self, event: StoredEvent) -> Result<(), StackError>;
}

/// In-memory repository for tests and explicitly requested local runs.
#[derive(Default)]
pub struct InMemoryEventRepository {
    events: Mutex<HashMap<Vec<u8>, Vec<StoredEvent>>>,
}

impl InMemoryEventRepository {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl EventRepository for InMemoryEventRepository {
    async fn list_scope(&self, scope: &[u8]) -> Result<Vec<StoredEvent>, StackError> {
        let events = self.events.lock().await;
        Ok(events.get(scope).cloned().unwrap_or_default())
    }

    async fn append_event(&self, event: StoredEvent) -> Result<(), StackError> {
        let mut events = self.events.lock().await;
        let scope_events = events.entry(event.scope().to_vec()).or_default();
        let expected = u64::try_from(scope_events.len())
            .map_err(|_| StackError::internal("event count exceeds u64"))?;
        if event.sequence() != expected {
            return Err(StackError::conflict(format!(
                "sequence {} does not match next sequence {expected}",
                event.sequence()
            )));
        }
        if let Some(idempotency_key) = event.idempotency_key() {
            if let Some(existing) = scope_events
                .iter()
                .find(|stored| stored.idempotency_key() == Some(idempotency_key))
            {
                if same_event_bytes(existing, &event) {
                    return Ok(());
                }
                return Err(StackError::conflict(
                    "idempotency key already committed with a different payload",
                ));
            }
        }
        scope_events.push(event);
        Ok(())
    }
}

/// Postgres repository backed by the Trellis async store schema.
#[derive(Clone)]
pub struct PostgresEventRepository {
    pool: PgPool,
}

impl PostgresEventRepository {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EventRepository for PostgresEventRepository {
    async fn list_scope(&self, scope: &[u8]) -> Result<Vec<StoredEvent>, StackError> {
        let rows = sqlx::query_as::<
            _,
            (
                Vec<u8>,
                i64,
                Vec<u8>,
                Vec<u8>,
                Option<Vec<u8>>,
                Option<Vec<u8>>,
            ),
        >(
            "\
SELECT scope, sequence, canonical_event, signed_event, idempotency_key, canonical_event_hash
FROM trellis_events
WHERE scope = $1
ORDER BY sequence",
        )
        .bind(scope)
        .fetch_all(&self.pool)
        .await
        .map_err(|error| StackError::unavailable(format!("trellis event read failed: {error}")))?;

        rows.into_iter()
            .map(
                |(scope, sequence, canonical, signed, idempotency_key, canonical_hash)| {
                    let sequence = u64::try_from(sequence)
                        .map_err(|_| StackError::internal("stored Trellis sequence is negative"))?;
                    let mut event = if let Some(idempotency_key) = idempotency_key {
                        StoredEvent::with_idempotency_key(
                            scope,
                            sequence,
                            canonical,
                            signed,
                            idempotency_key,
                        )
                    } else {
                        StoredEvent::new(scope, sequence, canonical, signed)
                    };
                    if let Some(hash) = canonical_hash {
                        let hash = hash.as_slice().try_into().map_err(|_| {
                            StackError::internal("stored canonical_event_hash is not 32 bytes")
                        })?;
                        event = event.with_canonical_event_hash(Some(hash));
                    }
                    Ok(event)
                },
            )
            .collect()
    }

    async fn append_event(&self, event: StoredEvent) -> Result<(), StackError> {
        let mut tx = self.pool.begin().await.map_err(|error| {
            StackError::unavailable(format!("trellis tx begin failed: {error}"))
        })?;
        trellis_store_postgres_async::append_event_in_tx(&mut tx, &event)
            .await
            .map_err(map_postgres_append_error)?;
        tx.commit().await.map_err(|error| {
            StackError::unavailable(format!("trellis tx commit failed: {error}"))
        })?;
        Ok(())
    }
}

/// Postgres-backed bundle publication index.
#[derive(Clone)]
pub(crate) struct PostgresBundleIndex {
    pool: PgPool,
}

impl PostgresBundleIndex {
    #[must_use]
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BundleIndexPort for PostgresBundleIndex {
    async fn reserve_publishable(
        &self,
        scope: &[u8],
        identity: &BundleIdentity,
    ) -> Result<(), StackError> {
        trellis_store_postgres_async::reserve_bundle_publication(
            &self.pool,
            &publication_identity(scope, identity),
        )
        .await
        .map_err(map_postgres_bundle_error)
    }

    async fn get_by_digest(
        &self,
        scope: &[u8],
        checkpoint_digest: &str,
    ) -> Result<Option<BundleRecord>, StackError> {
        trellis_store_postgres_async::get_bundle_publication_by_digest(
            &self.pool,
            scope,
            checkpoint_digest,
        )
        .await
        .map(|record| record.map(bundle_record_from_publication))
        .map_err(map_postgres_bundle_error)
    }

    async fn insert_published_record(
        &self,
        scope: &[u8],
        record: &BundleRecord,
        _update_head: bool,
    ) -> Result<(), StackError> {
        trellis_store_postgres_async::publish_bundle_publication(
            &self.pool,
            &publication_record(scope, record),
        )
        .await
        .map_err(map_postgres_bundle_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn in_memory_appends_sequential_events() {
        let repo = InMemoryEventRepository::new();
        let scope = b"scope-a".to_vec();
        let e0 = StoredEvent::with_idempotency_key(
            scope.clone(),
            0,
            vec![0x01],
            vec![0x02],
            b"key-a".to_vec(),
        );
        repo.append_event(e0).await.expect("first append");
        let e1 = StoredEvent::with_idempotency_key(
            scope.clone(),
            1,
            vec![0x03],
            vec![0x04],
            b"key-b".to_vec(),
        );
        repo.append_event(e1).await.expect("second append");
        let list = repo.list_scope(scope.as_slice()).await.expect("list");
        assert_eq!(list.len(), 2);
        assert_eq!(list[0].sequence(), 0);
        assert_eq!(list[1].sequence(), 1);
    }

    #[tokio::test]
    async fn in_memory_rejects_same_idempotency_key_with_different_payload() {
        let repo = InMemoryEventRepository::new();
        let scope = b"scope-b".to_vec();
        let e0 = StoredEvent::with_idempotency_key(
            scope.clone(),
            0,
            vec![0x01],
            vec![0x02],
            b"shared-key".to_vec(),
        );
        repo.append_event(e0).await.unwrap();
        let e1 = StoredEvent::with_idempotency_key(
            scope.clone(),
            1,
            vec![0xff],
            vec![0xfe],
            b"shared-key".to_vec(),
        );
        let err = repo.append_event(e1).await.expect_err("expected conflict");
        let msg = err.to_string();
        assert!(
            msg.contains("idempotency key already committed"),
            "unexpected message: {msg}"
        );
    }

    #[tokio::test]
    async fn in_memory_rejects_sequence_gap() {
        let repo = InMemoryEventRepository::new();
        let scope = b"scope-c".to_vec();
        let e1 = StoredEvent::new(scope.clone(), 1, vec![1], vec![2]);
        let err = repo
            .append_event(e1)
            .await
            .expect_err("sequence must start at 0");
        assert!(err.to_string().contains("sequence"));
    }
}
