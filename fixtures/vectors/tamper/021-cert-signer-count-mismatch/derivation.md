# Derivation — `tamper/021-cert-signer-count-mismatch`

Starts from `append/029-certificate-of-completion-dual-signer-pdf`, rewrites
`chain_summary.signer_count` from `2` to `3` while `signing_events` length
stays `2`. Per ADR 0007 §"Verifier obligations" step 2 first invariant
(`signer_count == len(signing_events)`), `decode_certificate_payload`
returns `Err(VerifyError::with_kind(..., "certificate_chain_summary_mismatch"))`,
which `_verify_event_set` surfaces as a fatal `tamper_kind`.

Failing canonical_event_hash: `b7817db6d47eacc152f577b7e1b7a84ea9fc8e0f4142a19e27dbe33f074c579d`.

Generator: `_generator/gen_tamper_021_023_025_026.py`.
