# TR-OP-111 model-check evidence

Date: 2026-04-21

Code artifact:
- `crates/trellis-conformance/src/model_checks.rs`
- replay test `tr_op_111_replay_and_property_battery_are_live`
- property tests `tr_core_020_single_canonical_order_per_scope`, `tr_core_023_order_is_independent_of_operational_accidents`, `tr_core_025_concurrency_uses_deterministic_tie_breaking`, `tr_core_046_prerequisites_gate_attestation`, `tr_core_050_idempotency_keys_are_stable_across_retries`, `tr_op_061_conflicts_stay_scoped_to_affected_facts`
- corpus replay test `tests::committed_vectors_match_the_rust_runtime`

Claim exercised:
- The Rust reference implementation now ships concrete replay and property-based operational test coverage rather than a pending placeholder.
