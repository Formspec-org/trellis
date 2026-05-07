# Derivation - `append/044-clock-satisfied`

ADR 0067 clockResolved record satisfying the opened response clock.

The WOS-owned provenance record is dCBOR-encoded as `input-clock-record.cbor`.
Trellis treats that record as inline payload bytes and binds it through
`content_hash`, `author_event_hash`, the COSE signature, and
`canonical_event_hash`.

## Inputs

- `ledger_scope` = `wos-case:adr0067-fixture`
- `sequence` = `1`
- `prev_hash` = `c80f41246b960971c2f451538a0208c9824fdde8c90bf86a0b9f1ad5e883fc47`
- `event_type` = `wos.clock.resolved`
- `recordKind` = `clockResolved`

## Pinned hashes

- `content_hash` = `10b01ca337ef397d7cb20c4b84f39a7d09dc2ba449e895c285e72c1ed710afb4`
- `author_event_hash` = `36659000d13614db53e25316bd612dd492897b141a87e94de982f69424a1e88b`
- `canonical_event_hash` = `b2f462b84293fc98ed54b0af83cb56d2e19142a511bb39332702e02ec0888ce2`

Generator: `fixtures/vectors/_generator/gen_adr0067_clocks.py`.
