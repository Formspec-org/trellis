# Derivation - `append/043-clock-started`

ADR 0067 clockStarted record opening a statutory response clock.

The WOS-owned provenance record is dCBOR-encoded as `input-clock-record.cbor`.
Trellis treats that record as inline payload bytes and binds it through
`content_hash`, `author_event_hash`, the COSE signature, and
`canonical_event_hash`.

## Inputs

- `ledger_scope` = `wos-case:adr0067-fixture`
- `sequence` = `0`
- `prev_hash` = `null`
- `event_type` = `wos.governance.clock_started`
- `recordKind` = `clockStarted`

## Pinned hashes

- `content_hash` = `d4d8294489287eeaca8834c8315c4d53e9c4462b3894b04645d2772ba18009d7`
- `author_event_hash` = `e4ac2f5cd68da37fc8c2d661062ae66733d14e0909480fe6b48cec1270a753ee`
- `canonical_event_hash` = `99a543ba1d0f5ec7832230529b0b9cfbb2ce2c3a564697257ac66536baf2802e`

Generator: `fixtures/vectors/_generator/gen_adr0067_clocks.py`.
