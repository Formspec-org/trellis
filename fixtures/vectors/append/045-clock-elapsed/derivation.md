# Derivation - `append/045-clock-elapsed`

ADR 0067 clockResolved record marking an independent notice clock elapsed.

The WOS-owned provenance record is dCBOR-encoded as `input-clock-record.cbor`.
Trellis treats that record as inline payload bytes and binds it through
`content_hash`, `author_event_hash`, the COSE signature, and
`canonical_event_hash`.

## Inputs

- `ledger_scope` = `wos-case:adr0067-fixture`
- `sequence` = `2`
- `prev_hash` = `af934ab5bbd4e704c9ace735e6cfb7f449c2f05fe5231ee03b04a536076cc6a4`
- `event_type` = `wos.governance.clock_resolved`
- `recordKind` = `clockResolved`

## Pinned hashes

- `content_hash` = `0e29240de315dc4d412f91096adffb0683b26578e03f6e61ad7f07be7fedeb00`
- `author_event_hash` = `3e99618b4d65bff7ff5410be7c7a67a2ef62a65e3ec16a35765e9e8e43e65a41`
- `canonical_event_hash` = `ca3ed2d7c6b81438489893edae75f0d2d1e58f19f437b634a823bf056854a66e`

Generator: `fixtures/vectors/_generator/gen_adr0067_clocks.py`.
