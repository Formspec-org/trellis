# TR-CORE-023 model-check evidence

Date: 2026-04-21

Code artifact:
- `crates/trellis-conformance/src/model_checks.rs`
- property test `tr_core_023_order_is_independent_of_operational_accidents`

Claim exercised:
- Canonical order depends only on the modeled specification inputs, not on receipt order, worker identity, or queue-depth accidents.
