# Derivation - `append/014-reinstatement`

ADR 0066 mode 5 reinstatement event on the same chain.

## Inputs

- Issuer key: `_keys/issuer-001.cose_key` (Ed25519 / suite-id 1).
- `ledger_scope` = `wos-case:adr0066-fixture-primary`
- `sequence` = `3`
- `prev_hash` = `7d3677f5dad1a17e3e5c8cb498cd2ac04ecd033c207b9285c1614d67087aa649`
- `event_type` = `wos.governance.reinstated`
- WOS/Formspec-owned payload bytes: `input-adr0066-record.cbor`.

The payload record is dCBOR-encoded as the inline ciphertext marker. Trellis
does not interpret the WOS governance fields in this positive append vector;
the envelope binds them through `content_hash`, `author_event_hash`, the COSE
signature, and `canonical_event_hash`.

## Pinned hashes

- `content_hash` = `f2f4c6c32dfa930212d98656567fe988838699a1ea3120e9cf0d5b9cb3e05f4b`
- `author_event_hash` = `f32f4ec6d0bdf0b3f8a00d2aa1ab9e1f824bccf6e09cc8a60ff7d46ef964b967`
- `canonical_event_hash` = `6db7fe19a9c4b17698a15281004031b949bb3cac598c2fe3324822e43f74699b`

Generator: `fixtures/vectors/_generator/gen_append_011_to_015.py`.
