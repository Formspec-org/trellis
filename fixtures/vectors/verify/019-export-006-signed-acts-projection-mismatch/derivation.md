# Derivation — `verify/019-export-006-signed-acts-projection-mismatch`

This fixture starts from `export/006-signature-affirmations-inline`, mutates
`066-signed-acts.cbor`, recomputes the `catalog_digest` under
`trellis.export.signed-acts.v1`, and re-signs `000-manifest.cbor`. The ZIP
remains structurally valid and all manifest digests match archive contents, but
the SignedAct projection no longer matches the signed WOS
`SignatureAffirmation` payload.
