# Derivation - `append/013-rescission`

ADR 0066 mode 4 determination-rescinded event on the same chain.

## Inputs

- Issuer key: `_keys/issuer-001.cose_key` (Ed25519 / suite-id 1).
- `ledger_scope` = `wos-case:adr0066-fixture-primary`
- `sequence` = `2`
- `prev_hash` = `6af990a8cadb34f4deb57fcf39c0b03d09382222750f05609a7277804ad66acd`
- `event_type` = `wos.governance.determinationRescinded`
- WOS/Formspec-owned payload bytes: `input-adr0066-record.cbor`.

The payload record is dCBOR-encoded as the inline ciphertext marker. Trellis
does not interpret the WOS governance fields in this positive append vector;
the envelope binds them through `content_hash`, `author_event_hash`, the COSE
signature, and `canonical_event_hash`.

## Pinned hashes

- `content_hash` = `ebe9a8f6cae1689f433d607ed8f04903c3c0a301ae146c8c12b05535564d9d35`
- `author_event_hash` = `a99a4f479af57c938b72eaae023bacbc7a377b8d278aa26151ead79c1d38d4d3`
- `canonical_event_hash` = `77e0d5c74dc9f6817cfb101eed28352f826de0a0db51f5f82417500667b145ec`

Generator: `fixtures/vectors/_generator/gen_append_011_to_015.py`.
