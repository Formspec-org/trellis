# Derivation - `append/011-correction`

ADR 0066 mode 1 correction-authorizing act on the existing chain.

## Inputs

- Issuer key: `_keys/issuer-001.cose_key` (Ed25519 / suite-id 1).
- `ledger_scope` = `wos-case:adr0066-fixture-primary`
- `sequence` = `0`
- `prev_hash` = `null`
- `event_type` = `wos.governance.correctionAuthorized`
- WOS/Formspec-owned payload bytes: `input-adr0066-record.cbor`.

The payload record is dCBOR-encoded as the inline ciphertext marker. Trellis
does not interpret the WOS governance fields in this positive append vector;
the envelope binds them through `content_hash`, `author_event_hash`, the COSE
signature, and `canonical_event_hash`.

## Pinned hashes

- `content_hash` = `75bbb6dde320f6a66f741c46c83826c4b261d4c0e9329167cec1cb1f92bfb7d6`
- `author_event_hash` = `f1e7b4742838181f0fa883c4de8403bb04ea27aeab1bceb469dd708a1b9f8d3e`
- `canonical_event_hash` = `0a18e130f3c5f47957860066ddffcc0ffb2e1c138ce951c6cb0a7c3c20197634`

Generator: `fixtures/vectors/_generator/gen_append_011_to_015.py`.
