# Derivation - `append/012-amendment`

ADR 0066 mode 2 determination-changing amendment event on the same chain.

## Inputs

- Issuer key: `_keys/issuer-001.cose_key` (Ed25519 / suite-id 1).
- `ledger_scope` = `wos-case:adr0066-fixture-primary`
- `sequence` = `1`
- `prev_hash` = `0a18e130f3c5f47957860066ddffcc0ffb2e1c138ce951c6cb0a7c3c20197634`
- `event_type` = `wos.governance.determinationAmended`
- WOS/Formspec-owned payload bytes: `input-adr0066-record.cbor`.

The payload record is dCBOR-encoded as the inline ciphertext marker. Trellis
does not interpret the WOS governance fields in this positive append vector;
the envelope binds them through `content_hash`, `author_event_hash`, the COSE
signature, and `canonical_event_hash`.

## Pinned hashes

- `content_hash` = `d6cb0b7a5147516c3d4f13c80525b8eed37fb9f4bd96ae83f8c6c61d35e666ac`
- `author_event_hash` = `c564517ca11cef44d303197507b0a8acf01a051936c5567cad4f397989252076`
- `canonical_event_hash` = `6af990a8cadb34f4deb57fcf39c0b03d09382222750f05609a7277804ad66acd`

Generator: `fixtures/vectors/_generator/gen_append_011_to_015.py`.
