# Derivation — `tamper/015-intake-handoff-catalog-digest-mismatch`

This fixture starts from `export/007-intake-handoffs-public-create`, mutates
`063-intake-handoffs.cbor`, and leaves the signed `000-manifest.cbor`
unchanged. The verifier must localize the failure to the intake-handoff catalog
digest bound by `trellis.export.intake-handoffs.v1.intake_catalog_digest`.
