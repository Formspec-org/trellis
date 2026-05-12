# Derivation - `append/013-rescission`

ADR 0066 mode 4 determination-rescinded event on the same chain.

## Inputs

- Issuer key: `_keys/issuer-001.cose_key` (Ed25519 / suite-id 1).
- `ledger_scope` = `wos-case:adr0066-fixture-primary`
- `sequence` = `2`
- `prev_hash` = `948cb5ec7f4afcd1842c66de18c02ed5d34ff2499e28daea6588390ee3910e97`
- `event_type` = `wos.governance.determination_rescinded`
- WOS/Formspec-owned payload bytes: `input-adr0066-record.cbor`.

The payload record is dCBOR-encoded as the inline ciphertext marker. Trellis
does not interpret the WOS governance fields in this positive append vector;
the envelope binds them through `content_hash`, `author_event_hash`, the COSE
signature, and `canonical_event_hash`.

## Pinned hashes

- `content_hash` = `ebe9a8f6cae1689f433d607ed8f04903c3c0a301ae146c8c12b05535564d9d35`
- `author_event_hash` = `2e93f28f82fce0be79b1a2ec0322b0c7eff70b6e341ae3676a667d9cc23eff91`
- `canonical_event_hash` = `7d3677f5dad1a17e3e5c8cb498cd2ac04ecd033c207b9285c1614d67087aa649`

Generator: `fixtures/vectors/_generator/gen_append_011_to_015.py`.
