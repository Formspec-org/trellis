# TR-CORE-050 model-check evidence

Date: 2026-04-21

Code artifact:
- `crates/trellis-conformance/src/model_checks.rs`
- property test `tr_core_050_idempotency_keys_are_stable_across_retries`

Claim exercised:
- Same-key same-payload retries replay the same canonical reference, while same-key different-payload retries are rejected.
