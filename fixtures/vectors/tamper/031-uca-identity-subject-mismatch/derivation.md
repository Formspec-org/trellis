# Derivation — `tamper/031-uca-identity-subject-mismatch`

3-event chain on `ledger_scope = b'trellis-uca-tamper:031-subj-mismatch'`:

* seq 0: identity-attestation event with `subject` = `"urn:trellis:principal:somebody-else-031"`
* seq 1: host event
* seq 2: user-content-attestation event with `attestor` = `"urn:trellis:principal:applicant-031"`,
  `identity_attestation_ref` = seq 0's `canonical_event_hash`
  (`6e7f4e3d8e18574a541d28a5ad99af919a2e7274959eef66398b1f0add5bf3bd`).

Per ADR 0010 §"Verifier obligations" step 4, the resolved identity-
attestation event's payload subject MUST equal `attestor`. This vector's
mismatch flips `identity_resolved = false` and emits
`user_content_attestation_identity_subject_mismatch` with location
pointing at the resolved-but-wrong-subject identity event hash.

Generator: `_generator/gen_tamper_028_to_034.py`.
