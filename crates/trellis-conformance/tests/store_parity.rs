// Rust guideline compliant 2026-02-21
//! Cross-store parity: every `append/*` fixture produces byte-identical
//! stored canonical + signed bytes whether the underlying store is
//! `trellis-store-memory` or `trellis-store-postgres`.
//!
//! Conformance-side enforcement of the wos-server commitment that
//! `trellis-store-postgres` is a drop-in canonical-side composition partner
//! for the in-memory test oracle (per VISION.md §III + §V — "Trellis IS
//! the database"; the canonical schema is byte-equivalent across adapters).
//!
//! This is also the first-class integration test exercising the
//! transaction-composition surface ([`trellis_store_postgres::append_event_in_tx`])
//! against the full append corpus.

mod common;

use std::fs;
use std::path::{Path, PathBuf};

use trellis_core::{AuthoredEvent, SigningKeyMaterial, append_event};
use trellis_store_memory::MemoryStore;
use trellis_store_postgres::{PostgresStore, append_event_in_tx};
use trellis_types::StoredEvent;

use common::TestCluster;

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/vectors")
}

fn append_vector_dirs() -> Vec<PathBuf> {
    let mut dirs = fs::read_dir(fixtures_root().join("append"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.is_dir())
        .collect::<Vec<_>>();
    dirs.sort();
    dirs
}

fn manifest_for(dir: &Path) -> toml::Value {
    toml::from_str(&fs::read_to_string(dir.join("manifest.toml")).unwrap()).unwrap()
}

fn signing_key_name(inputs: &toml::value::Table) -> &str {
    inputs
        .get("signing_key")
        .and_then(toml::Value::as_str)
        .or_else(|| inputs.get("signing_key_b").and_then(toml::Value::as_str))
        .expect("append manifest must name a signing key")
}

#[test]
fn append_corpus_byte_parity_memory_vs_postgres() {
    let cluster = TestCluster::start();
    let mut postgres_store = PostgresStore::connect(&cluster.connection_string()).unwrap();

    let mut checked = 0usize;
    for dir in append_vector_dirs() {
        let manifest = manifest_for(&dir);
        let inputs = manifest
            .get("inputs")
            .and_then(toml::Value::as_table)
            .expect("manifest.inputs missing");
        let authored_event_path = inputs
            .get("authored_event")
            .and_then(toml::Value::as_str)
            .expect("inputs.authored_event missing");
        let authored_event = fs::read(dir.join(authored_event_path)).unwrap();
        let signing_key = fs::read(dir.join(signing_key_name(inputs))).unwrap();

        // Run the production append pipeline against memory.
        let mut memory_store = MemoryStore::new();
        let memory_artifacts = append_event(
            &mut memory_store,
            &SigningKeyMaterial::new(signing_key.clone()),
            &AuthoredEvent::new(authored_event.clone()),
        )
        .unwrap();

        assert_eq!(memory_store.events().len(), 1);
        let memory_stored = &memory_store.events()[0];

        // Run again, this time persisting into Postgres via the transaction-
        // composition surface that wos-server's EventStore will use.
        let mut runner = MemoryStore::new();
        let postgres_artifacts = append_event(
            &mut runner,
            &SigningKeyMaterial::new(signing_key),
            &AuthoredEvent::new(authored_event),
        )
        .unwrap();
        let runner_stored = &runner.events()[0];

        // The append corpus reuses scopes across fixtures (e.g. multiple
        // `001-`-style minimal events share `ledger_scope` and `sequence`);
        // truncate before each per-fixture round-trip so Postgres sees a
        // fresh write. Byte parity is the assertion under test, not
        // multi-fixture replay (which the wider conformance suite covers).
        {
            let mut clean = postgres_store.begin().unwrap();
            clean
                .batch_execute("TRUNCATE TABLE trellis_events")
                .unwrap();
            clean.commit().unwrap();
        }
        let mut tx = postgres_store.begin().unwrap();
        append_event_in_tx(
            &mut tx,
            &StoredEvent::new(
                runner_stored.scope().to_vec(),
                runner_stored.sequence(),
                runner_stored.canonical_event().to_vec(),
                runner_stored.signed_event().to_vec(),
            ),
            None,
        )
        .unwrap();
        tx.commit().unwrap();

        let postgres_events = postgres_store
            .load_scope_events(memory_stored.scope())
            .unwrap();
        let postgres_stored = postgres_events
            .iter()
            .find(|e| e.sequence() == memory_stored.sequence())
            .unwrap_or_else(|| panic!("postgres did not persist event for {}", dir.display()));

        // BYTE-EXACT parity contract.
        assert_eq!(
            postgres_stored.canonical_event(),
            memory_stored.canonical_event(),
            "canonical_event byte mismatch memory vs postgres for {}",
            dir.display()
        );
        assert_eq!(
            postgres_stored.signed_event(),
            memory_stored.signed_event(),
            "signed_event byte mismatch memory vs postgres for {}",
            dir.display()
        );
        assert_eq!(
            postgres_artifacts.canonical_event_hash,
            memory_artifacts.canonical_event_hash,
            "canonical_event_hash divergence for {}",
            dir.display()
        );

        checked += 1;
    }

    assert!(
        checked > 0,
        "no append vectors discovered under fixtures/vectors/append"
    );
}

#[test]
fn transaction_composition_atomic_with_caller_projections() {
    // Demonstrate the load-bearing wos-server commitment: a canonical
    // append + a caller-supplied projection update commit atomically OR
    // roll back together. This is the architectural invariant VISION.md
    // §VIII rejection of dual-write rests on.
    let cluster = TestCluster::start();
    let mut store = PostgresStore::connect(&cluster.connection_string()).unwrap();

    {
        // Composition root would create this kind of table per its own schema;
        // we model the seam with a simple side table.
        let mut tx = store.begin().unwrap();
        tx.batch_execute("CREATE TABLE projection_demo (case_id TEXT PRIMARY KEY, status TEXT)")
            .unwrap();
        tx.commit().unwrap();
    }

    // Happy path: canonical + projection commit together.
    {
        let mut tx = store.begin().unwrap();
        append_event_in_tx(
            &mut tx,
            &StoredEvent::new(b"case-1".to_vec(), 0, vec![0xab], vec![0xcd]),
            None,
        )
        .unwrap();
        tx.execute(
            "INSERT INTO projection_demo (case_id, status) VALUES ($1, $2)",
            &[&"case-1", &"open"],
        )
        .unwrap();
        tx.commit().unwrap();
    }
    let events = store.load_scope_events(b"case-1").unwrap();
    assert_eq!(events.len(), 1);

    // Sad path: projection write fails -> canonical event MUST roll back.
    {
        let mut tx = store.begin().unwrap();
        append_event_in_tx(
            &mut tx,
            &StoredEvent::new(b"case-2".to_vec(), 0, vec![0xee], vec![0xff]),
            None,
        )
        .unwrap();
        // PK conflict against case-1 row.
        let projection_err = tx.execute(
            "INSERT INTO projection_demo (case_id, status) VALUES ($1, $2)",
            &[&"case-1", &"duplicate"],
        );
        assert!(projection_err.is_err());
        // Drop without commit -> rollback.
        drop(tx);
    }
    let events = store.load_scope_events(b"case-2").unwrap();
    assert!(
        events.is_empty(),
        "rolled-back tx left canonical event visible"
    );
}
