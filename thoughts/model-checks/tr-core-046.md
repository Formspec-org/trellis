# TR-CORE-046 model-check evidence

Date: 2026-04-21

Code artifact:
- `crates/trellis-conformance/src/model_checks.rs`
- property test `tr_core_046_prerequisites_gate_attestation`

Claim exercised:
- The model never issues an append attestation before all declared dependencies are admitted, and succeeds once the prerequisites hold.
