# Derivation — `tamper/029-uca-chain-position-mismatch`

3-event chain on `ledger_scope = b'trellis-uca-tamper:029-chain-pos'`:

* seq 0: identity-attestation event resolving `"urn:trellis:principal:applicant-029"`
* seq 1: host event with `canonical_event_hash` = `6bdf2d27ba75948dfb3635a3ccb30e3755c7f816192df47cd16ece6004b3fadc`
* seq 2: user-content-attestation event with `attested_event_position = 1`
  but `attested_event_hash` set to `4417ac380368075b2c3a09f265a9f63a11021e6ff092aca1ddbf4de35153acaf` (≠ host's actual hash).

Per ADR 0010 §"Verifier obligations" step 3, the verifier MUST resolve
`attested_event_position` to a chain-present event in scope and confirm
its `canonical_event_hash` equals `attested_event_hash`. This vector's
disagreement flips `chain_position_resolved = false` and emits
`user_content_attestation_chain_position_mismatch` with `failing_event_id`
= `ed46881143b8b72c912766ed0e0d896462c31063165968b74a5ebf92effc2897`.

Generator: `_generator/gen_tamper_028_to_034.py`.
