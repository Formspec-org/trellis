# Derivation — `tamper/023-cert-attestation-malformed`

Starts from `append/028-certificate-of-completion-minimal-pdf`. Truncates
`attestations[0].signature` from 64 to 63 bytes (Ed25519 signatures are
fixed-size 64-byte values per RFC 8032). Phase-1 reference verifier checks
structural shape only — it does not crypto-verify attestation signatures
(see `finalize_certificates_of_completion` step 3 docstring in
`crates/trellis-verify/src/lib.rs`). The structural check is `signature.len()
== 64`; truncation flips `attestation_signatures_well_formed = false`,
yielding `attestation_insufficient` per ADR 0007 §"Verifier obligations"
step 3 (existing Core §19.1 tamper_kind reused).

A single-byte flip on a 64-byte signature would NOT trigger the Phase-1
structural failure mode — the length stays 64, so the verifier would admit
the malformed signature pending Phase-2+ crypto verification. Truncation
exercises the operative Phase-1 path.

Failing canonical_event_hash: `b82aec24029e4f502213ad60af2a67d83f2a00539a3bb13078efe048371dd8a0`.

Generator: `_generator/gen_tamper_021_023_025_026.py`.
