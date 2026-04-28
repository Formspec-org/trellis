# Derivation — `tamper/030-uca-identity-unresolved`

2-event chain on `ledger_scope = b'trellis-uca-tamper:030-no-identity'`:

* seq 0: host event
* seq 1: user-content-attestation event with `identity_attestation_ref` =
  `bc8e2f7571ff79b284b0fd881a97567ad7027f085a6c9593f7875c03d0824977` (a digest that does NOT resolve to any
  chain-present event).

Per ADR 0010 §"Verifier obligations" step 4, when `identity_attestation_ref`
is non-null the verifier MUST resolve it to a chain-present event of a
registered identity-attestation event type. Failure to resolve flips
`identity_resolved = false` and emits
`user_content_attestation_identity_unresolved` with `failing_event_id` =
the **identity_attestation_ref** digest (the unresolvable target), per
verifier convention for the location-of-failure field.

The default-required posture is in force (no Posture Declaration shipped
with this tamper, so `admit_unverified_user_attestations` defaults to
`false`).

Generator: `_generator/gen_tamper_028_to_034.py`.
