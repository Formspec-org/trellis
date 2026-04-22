# Derivation — `tamper/014-signature-catalog-digest-mismatch`

This fixture starts from `export/006-signature-affirmations-inline`, mutates
`062-signature-affirmations.cbor`, and leaves the signed `000-manifest.cbor`
unchanged. The verifier must localize the failure to the signature catalog
digest bound by `trellis.export.signature-affirmations.v1.signature_catalog_digest`.
