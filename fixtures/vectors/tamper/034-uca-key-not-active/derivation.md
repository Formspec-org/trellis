# Derivation — `tamper/034-uca-key-not-active`

3-event chain on `ledger_scope = b'trellis-uca-tamper:034-key-retired'`:

* seq 0: identity-attestation event
* seq 1: host event
* seq 2: user-content-attestation event signed under kid
  `af9dff525391faa75c8e8da4808b1743` (registered with `status = 2` Retired per Core §28
  `SigningKeyStatus`).

Per ADR 0010 §"Verifier obligations" step 6, only `Active`
(SigningKeyStatus = 0) is admitted in Phase 1; `Rotating` (1) rides
ratified rotation grace per ADR 0010 open question 4. The verifier flips
`key_active = false` and emits
`user_content_attestation_key_not_active` with `failing_event_id` =
`92776371d61fd0bf6a7b2637185d7ec507e8f39988776ab1f0eb18c45819ace2`.

This is the first fixture corpus to exercise the SigningKeyStatus
distinction at the user-content-attestation step 6 surface — prior
fixtures use the COSE-envelope-only key-state path which only gates on
Revoked (status = 3).

Generator: `_generator/gen_tamper_028_to_034.py`.
