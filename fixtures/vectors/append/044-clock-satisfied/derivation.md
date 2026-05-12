# Derivation - `append/044-clock-satisfied`

ADR 0067 clockResolved record satisfying the opened response clock.

The WOS-owned provenance record is dCBOR-encoded as `input-clock-record.cbor`.
Trellis treats that record as inline payload bytes and binds it through
`content_hash`, `author_event_hash`, the COSE signature, and
`canonical_event_hash`.

## Inputs

- `ledger_scope` = `wos-case:adr0067-fixture`
- `sequence` = `1`
- `prev_hash` = `99a543ba1d0f5ec7832230529b0b9cfbb2ce2c3a564697257ac66536baf2802e`
- `event_type` = `wos.governance.clock_resolved`
- `recordKind` = `clockResolved`

## Pinned hashes

- `content_hash` = `7625b116ebdcc88f2d4f0aea74b89287e45afd0cdcd599926a26d6456b574f95`
- `author_event_hash` = `610b4be98111c81fc2239b12a3c1eace8c4eef0336498b35855b914d30b6989f`
- `canonical_event_hash` = `af934ab5bbd4e704c9ace735e6cfb7f449c2f05fe5231ee03b04a536076cc6a4`

Generator: `fixtures/vectors/_generator/gen_adr0067_clocks.py`.
