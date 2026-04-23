# Derivation — `verify/015-export-007-intake-response-hash-mismatch`

This fixture starts from `export/007-intake-handoffs-public-create`, mutates
the embedded Response bytes in `063-intake-handoffs.cbor`, recomputes the
catalog digest, and re-signs `000-manifest.cbor`. The ZIP remains structurally
valid, but the Formspec handoff's `responseHash` no longer matches the carried
Response bytes.
