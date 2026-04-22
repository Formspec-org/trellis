# Derivation — `verify/014-export-006-signature-row-mismatch`

This fixture starts from `export/006-signature-affirmations-inline`, mutates the
`signer_id` in `062-signature-affirmations.cbor`, recomputes the catalog digest,
and re-signs `000-manifest.cbor`. The ZIP remains structurally valid and all
manifest digests match the archive contents, but the signature catalog no longer
matches the chain-authored WOS `SignatureAffirmation` payload.

`manifest.toml` pins `first_failure_kind` and `failing_event_id` under
`[expected.report]`. Both **`trellis-conformance`** (Rust) and **`trellis_py.conformance`**
assert those fields against this ZIP so the two verifiers stay aligned on the
first observable failure for export extension checks.
