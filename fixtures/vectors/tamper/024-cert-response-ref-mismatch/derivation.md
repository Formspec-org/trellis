# Derivation — `tamper/024-cert-response-ref-mismatch`

Starts from `export/010-certificate-of-completion-inline`. The certificate event's `chain_summary.response_ref` is XOR-flipped (`b ^ 0xAA` for each byte) so its 32-byte digest disagrees with the SignatureAffirmation record's `data.formspecResponseRef = "sha256:1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"`.

`finalize_certificates_of_completion` runs step 7 (response_ref equivalence): walks the chain-resolved SignatureAffirmation events, parses each `data.formspecResponseRef` via `parse_sha256_text` (succeeds because the source vector ships a `sha256:<hex>` text, not a URL), records `had_resolvable_response = true`, finds no match, and emits `response_ref_mismatch` localized to the certificate's canonical_event_hash.

Generator: `_generator/gen_export_010_certificate_of_completion.py`.
