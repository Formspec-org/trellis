# Derivation - `append/012-amendment`

ADR 0066 mode 2 determination-changing amendment event on the same chain.

## Inputs

- Issuer key: `_keys/issuer-001.cose_key` (Ed25519 / suite-id 1).
- `ledger_scope` = `wos-case:adr0066-fixture-primary`
- `sequence` = `1`
- `prev_hash` = `cdc659202e1bcd425cf18e36202793e7300491e0043f5d7907106dbd51421ef0`
- `event_type` = `wos.governance.determination_amended`
- WOS/Formspec-owned payload bytes: `input-adr0066-record.cbor`.

The payload record is dCBOR-encoded as the inline ciphertext marker. Trellis
does not interpret the WOS governance fields in this positive append vector;
the envelope binds them through `content_hash`, `author_event_hash`, the COSE
signature, and `canonical_event_hash`.

## Pinned hashes

- `content_hash` = `d6cb0b7a5147516c3d4f13c80525b8eed37fb9f4bd96ae83f8c6c61d35e666ac`
- `author_event_hash` = `84d40e7c0ccaccc0cd4345a867fa58076bd37eec8f656664e89e204dc5c20742`
- `canonical_event_hash` = `948cb5ec7f4afcd1842c66de18c02ed5d34ff2499e28daea6588390ee3910e97`

Generator: `fixtures/vectors/_generator/gen_append_011_to_015.py`.
