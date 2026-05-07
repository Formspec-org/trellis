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
- `event_type` = `wos.clock.started`
- `recordKind` = `clockStarted`

## Pinned hashes

- `content_hash` = `d4d8294489287eeaca8834c8315c4d53e9c4462b3894b04645d2772ba18009d7`
- `author_event_hash` = `4f30bc97931021d7eac412ee28adaa87a87d0b5c800432afa066aafd7398742f`
- `canonical_event_hash` = `c80f41246b960971c2f451538a0208c9824fdde8c90bf86a0b9f1ad5e883fc47`

Generator: `fixtures/vectors/_generator/gen_adr0067_clocks.py`.
