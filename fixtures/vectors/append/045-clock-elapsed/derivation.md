# Derivation - `append/045-clock-elapsed`

ADR 0067 clockResolved record marking an independent notice clock elapsed.

The WOS-owned provenance record is dCBOR-encoded as `input-clock-record.cbor`.
Trellis treats that record as inline payload bytes and binds it through
`content_hash`, `author_event_hash`, the COSE signature, and
`canonical_event_hash`.

## Inputs

- `ledger_scope` = `wos-case:adr0067-fixture`
- `sequence` = `2`
- `prev_hash` = `b2f462b84293fc98ed54b0af83cb56d2e19142a511bb39332702e02ec0888ce2`
- `event_type` = `wos.clock.resolved`
- `recordKind` = `clockResolved`

## Pinned hashes

- `content_hash` = `0e29240de315dc4d412f91096adffb0683b26578e03f6e61ad7f07be7fedeb00`
- `author_event_hash` = `16f87c2cdc07dec99a06cd8381f0f7823ce01b86b9bab82b1f6102edd7b2e114`
- `canonical_event_hash` = `ab8ccf0eb6614b2228e8b9368513a19a8f27c53d5e3d133e55330b981b6bb44d`

Generator: `fixtures/vectors/_generator/gen_adr0067_clocks.py`.
