# Derivation - `append/014-reinstatement`

ADR 0066 mode 5 reinstatement event on the same chain.

## Inputs

- Issuer key: `_keys/issuer-001.cose_key` (Ed25519 / suite-id 1).
- `ledger_scope` = `wos-case:adr0066-fixture-primary`
- `sequence` = `3`
- `prev_hash` = `77e0d5c74dc9f6817cfb101eed28352f826de0a0db51f5f82417500667b145ec`
- `event_type` = `wos.governance.reinstated`
- WOS/Formspec-owned payload bytes: `input-adr0066-record.cbor`.

The payload record is dCBOR-encoded as the inline ciphertext marker. Trellis
does not interpret the WOS governance fields in this positive append vector;
the envelope binds them through `content_hash`, `author_event_hash`, the COSE
signature, and `canonical_event_hash`.

## Pinned hashes

- `content_hash` = `f2f4c6c32dfa930212d98656567fe988838699a1ea3120e9cf0d5b9cb3e05f4b`
- `author_event_hash` = `92e9095ba9caa2215684b63e727e7bd7bd40987b4c892a2f738bbd6663977e3a`
- `canonical_event_hash` = `4de23e6fd5982d4cadb65d4ff561486fbb46060d27d7999b9c368b72cf0cfef7`

Generator: `fixtures/vectors/_generator/gen_append_011_to_015.py`.
