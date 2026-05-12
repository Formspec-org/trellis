# Derivation - `append/046-clock-paused-resumed`

ADR 0067 residual clockStarted segment after a pause/resume boundary.

The WOS-owned provenance record is dCBOR-encoded as `input-clock-record.cbor`.
Trellis treats that record as inline payload bytes and binds it through
`content_hash`, `author_event_hash`, the COSE signature, and
`canonical_event_hash`.

## Inputs

- `ledger_scope` = `wos-case:adr0067-fixture`
- `sequence` = `3`
- `prev_hash` = `ca3ed2d7c6b81438489893edae75f0d2d1e58f19f437b634a823bf056854a66e`
- `event_type` = `wos.governance.clock_started`
- `recordKind` = `clockStarted`

## Pinned hashes

- `content_hash` = `8214cf499d4275f9fd0c479453ded66d41bcf852d7e7ac8b271d160154fec932`
- `author_event_hash` = `0cc7d44156a47b819de29a2be7b47129c47ea523616094f3d4484c18eca684b9`
- `canonical_event_hash` = `b8daaf7bd2abd639c04b9e8aba12a14241883cb3e96b3f722625432cbb519d39`

Generator: `fixtures/vectors/_generator/gen_adr0067_clocks.py`.
