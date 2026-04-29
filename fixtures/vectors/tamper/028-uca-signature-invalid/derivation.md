# Derivation — `tamper/028-uca-signature-invalid`

3-event chain on `ledger_scope = b'trellis-uca-tamper:028-sig-invalid'`:

* seq 0: identity-attestation event whose payload `extensions."x-trellis-test/identity-attestation/v1".subject` equals `"urn:trellis:principal:applicant-028"`
* seq 1: host event (event_type `trellis.test.host-event.v1`)
* seq 2: user-content-attestation event whose `attested_event_hash`
  resolves to seq 1 (the host), `identity_attestation_ref` resolves to
  seq 0 (the identity event). Step 4 temporal precedence: identity
  sequence 0 < attested_event_position 1 — passes.

Per ADR 0010 §"Verifier obligations" step 5, the signature on
`UserContentAttestationPayload.signature` MUST be computed under domain
tag `trellis-user-content-attestation-v1` (Core §9.8). This vector
computes the signature under `trellis-transition-attestation-v1`
(Companion §A.5's operator-actor posture-transition tag). The Phase-1
verifier flips `signature_verified = false` and emits
`user_content_attestation_signature_invalid` with `failing_event_id`
= `32f293be28b3145c06ab5687b3bc9d9af596be3af24fd622b07aeff4c0796ccd`.

Adversary intent: cross-family signature confusion — present an A.5
Attestation byte slug as a user-content attestation, hoping the verifier
admits the byte shape. The domain-separation tag is what blocks this.

Generator: `_generator/gen_tamper_028_to_034.py`.
