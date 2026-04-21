# TR-OP-061 model-check evidence

Date: 2026-04-21

Code artifact:
- `crates/trellis-conformance/src/model_checks.rs`
- property test `tr_op_061_conflicts_stay_scoped_to_affected_facts`

Claim exercised:
- Conflict handling remains local to the affected scope and fact category; unrelated scopes keep admitting records.
