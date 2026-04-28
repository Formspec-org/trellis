# Derivation — `tamper/032-uca-identity-temporal-inversion`

2-event chain on `ledger_scope = b'trellis-uca-tamper:032-temporal'`:

* seq 0: identity-attestation event with `subject` = `"urn:trellis:principal:applicant-032"`,
  `canonical_event_hash` = `0e89362621972cc804552746ee24ccb29c544308aad8d37e1e701095e6a1de07`.
* seq 1: user-content-attestation event with
  `attested_event_position = 0`, `attested_event_hash` = identity event's
  canonical hash, `identity_attestation_ref` = same digest.

Per ADR 0010 §"Verifier obligations" step 4, the resolved identity-
attestation event's `sequence` MUST be strictly less than
`attested_event_position` (identity proof temporally precedes the
attestation). Here both are 0, so the inequality `0 < 0` fails. Verifier
flips `identity_resolved = false` and emits
`user_content_attestation_identity_temporal_inversion`.

The construction collapses the host event and identity event into one
event for compactness — the attestation references identity-as-host at
position 0 AND as identity_ref. Step 3 (position+hash agreement) passes;
step 4 (temporal precedence) fails. Real deployments would have a
separate host event; the temporal-inversion class still applies whenever
identity event sequence ≥ attested_event_position.

Generator: `_generator/gen_tamper_028_to_034.py`.
