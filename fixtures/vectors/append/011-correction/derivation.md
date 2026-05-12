# Derivation - `append/011-correction`

ADR 0066 mode 1 correction-authorizing act on the existing chain.

## Inputs

- Issuer key: `_keys/issuer-001.cose_key` (Ed25519 / suite-id 1).
- `ledger_scope` = `wos-case:adr0066-fixture-primary`
- `sequence` = `0`
- `prev_hash` = `null`
- `event_type` = `wos.governance.correction_authorized`
- WOS/Formspec-owned payload bytes: `input-adr0066-record.cbor`.

The payload record is dCBOR-encoded as the inline ciphertext marker. Trellis
does not interpret the WOS governance fields in this positive append vector;
the envelope binds them through `content_hash`, `author_event_hash`, the COSE
signature, and `canonical_event_hash`.

## Pinned hashes

- `content_hash` = `75bbb6dde320f6a66f741c46c83826c4b261d4c0e9329167cec1cb1f92bfb7d6`
- `author_event_hash` = `d7b60c423ae6fe6033d8fdbec926943c37970644a60e8527bbbe27126a51b416`
- `canonical_event_hash` = `cdc659202e1bcd425cf18e36202793e7300491e0043f5d7907106dbd51421ef0`

Generator: `fixtures/vectors/_generator/gen_append_011_to_015.py`.
