# Derivation - `append/046-clock-paused-resumed`

ADR 0067 residual clockStarted segment after a pause/resume boundary.

The WOS-owned provenance record is dCBOR-encoded as `input-clock-record.cbor`.
Trellis treats that record as inline payload bytes and binds it through
`content_hash`, `author_event_hash`, the COSE signature, and
`canonical_event_hash`.

## Inputs

- `ledger_scope` = `wos-case:adr0067-fixture`
- `sequence` = `3`
- `prev_hash` = `ab8ccf0eb6614b2228e8b9368513a19a8f27c53d5e3d133e55330b981b6bb44d`
- `event_type` = `wos.clock.started`
- `recordKind` = `clockStarted`

## Pinned hashes

- `content_hash` = `8214cf499d4275f9fd0c479453ded66d41bcf852d7e7ac8b271d160154fec932`
- `author_event_hash` = `ea63329e0c5f87ba26891b29181de826dbbdeede2ff4ed354682ebeef953539c`
- `canonical_event_hash` = `47ef7cacb82f861a222a88c142db08aa735b39a04d0db3b3af37b14b1f83c747`

Generator: `fixtures/vectors/_generator/gen_adr0067_clocks.py`.
