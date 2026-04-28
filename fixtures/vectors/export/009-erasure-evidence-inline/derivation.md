# Derivation — `export/009-erasure-evidence-inline`

Single-event export: `010-events.cbor` contains only `tamper/017-erasure-post-use` chain
event 0 (genesis erasure host). Domain registry admits `trellis.erasure-evidence.v1`.
`064-erasure-evidence.cbor` is a one-row chain-derived catalog; the manifest extension
`trellis.export.erasure-evidence.v1` binds its digest and `entry_count`.

Generator: `fixtures/vectors/_generator/gen_export_009_erasure_evidence.py`.
