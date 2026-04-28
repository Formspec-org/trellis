# Derivation — `tamper/033-uca-intent-malformed`

3-event chain on `ledger_scope = b'trellis-uca-tamper:033-bad-intent'`:

* seq 0: identity-attestation event
* seq 1: host event
* seq 2: user-content-attestation event with `signing_intent` =
  `"not-a-uri-just-some-string"` (not a URI — no scheme separator).

Per ADR 0010 §"Verifier obligations" step 2, `signing_intent` MUST be
a syntactically valid URI per RFC 3986. The reference verifier's
`is_syntactically_valid_uri` rejects this string. Step 2 is an
intra-payload-invariant check (post-CDDL-decode), so the failure flips
`integrity_verified = false` only — `structure_verified` and
`readability_verified` stay `true`. The decoder records a deferred
`step_2_failure` marker on `UserContentAttestationDetails`;
`finalize_user_content_attestations` raises it as an `event_failure`
with kind `user_content_attestation_intent_malformed` and skips
remaining per-event checks for this attestation.

`failing_event_id` = `f062c08349cb038426a74b0f31f4cf80669d88cd6206c910ceaeb6359bc0c06b` (the offending UCA event).

Generator: `_generator/gen_tamper_028_to_034.py`.
