# Derivation — `tamper/019-erasure-catalog-digest-mismatch`

Starts from `export/009-erasure-evidence-inline`, changes `evidence_id` in the first
catalog row (so catalog bytes change) while leaving `000-manifest.cbor` unchanged.
The verifier must fail with `erasure_evidence_catalog_digest_mismatch`.
