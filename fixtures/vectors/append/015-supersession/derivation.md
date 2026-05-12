# Derivation - `append/015-supersession`

ADR 0066 mode 3 new-chain supersession event with registered Trellis linkage extension.

## Inputs

- Issuer key: `_keys/issuer-001.cose_key` (Ed25519 / suite-id 1).
- `ledger_scope` = `wos-case:adr0066-fixture-superseding`
- `sequence` = `0`
- `prev_hash` = `null`
- `event_type` = `wos.kernel.supersession_started`
- WOS/Formspec-owned payload bytes: `input-adr0066-record.cbor`.

The payload record is dCBOR-encoded as the inline ciphertext marker. Trellis
does not interpret the WOS governance fields in this positive append vector;
the envelope binds them through `content_hash`, `author_event_hash`, the COSE
signature, and `canonical_event_hash`.

## Supersession extension

`EventPayload.extensions["trellis.supersedes-chain-id.v1"]` carries
`SupersedesChainIdPayload`:

- `chain_id` = `wos-case:adr0066-fixture-primary`
- `checkpoint_hash` = `fce9e813193b1a0bb1f5568da9190ad38bd6926e17b74f39b62f922842cdda85`

This pins Core section 6.7 / section 28 and TR-CORE-169 at the fixture layer.

## Pinned hashes

- `content_hash` = `664844849cf23dd192b8f417259472980393649479a453bb0834f1f23e02d002`
- `author_event_hash` = `fe961a0cb52e4e87419fea6122774a7cd9bcdfa3dead8210fc8b9bac164ae4e3`
- `canonical_event_hash` = `d20ed042de676cc31d2772b269878261635ac6faf372e93a2bd44369b6a6b4d0`

Generator: `fixtures/vectors/_generator/gen_append_011_to_015.py`.
