# TR-CORE-025 model-check evidence

Date: 2026-04-21

Code artifact:
- `crates/trellis-conformance/src/model_checks.rs`
- property test `tr_core_025_concurrency_uses_deterministic_tie_breaking`

Claim exercised:
- Concurrent ready records collapse to one deterministic total order via the model's explicit `(tie_breaker, id)` rule.
