# TR-CORE-001 model-check evidence

Date: 2026-04-21

Code artifact:

- `crates/trellis-conformance/src/model_checks.rs`
- replay test `tr_core_001_append_fixture_replay_is_identical_across_memory_and_indexed_stores`
- supporting contract-state tests `tr_core_046_prerequisites_gate_attestation` and `tr_core_050_idempotency_keys_are_stable_across_retries`

Claim exercised:

- For the committed `append/001` fixture, the Phase-1 append contract yields the same canonical append artifacts and the same ordered event sequence when replayed through the `MemoryStore` and `IndexedStore` test harnesses wired in that test. This is a **cross-adapter byte parity** check on two in-memory implementations, not a claim about I/O durability, crash recovery, or other storage semantics.
