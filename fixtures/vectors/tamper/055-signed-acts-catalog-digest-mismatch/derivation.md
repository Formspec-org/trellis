# Derivation — `tamper/055-signed-acts-catalog-digest-mismatch`

This fixture starts from `export/006-signature-affirmations-inline`, mutates
`066-signed-acts.cbor`, and leaves the signed `000-manifest.cbor` unchanged.
The verifier must localize the failure to the SignedAct projection catalog
digest bound by `trellis.export.signed-acts.v1.catalog_digest`.
