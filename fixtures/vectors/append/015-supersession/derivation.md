# Derivation - `append/015-supersession`

ADR 0066 mode 3 new-chain supersession event with registered Trellis linkage extension.

## Inputs

- Issuer key: `_keys/issuer-001.cose_key` (Ed25519 / suite-id 1).
- `ledger_scope` = `wos-case:adr0066-fixture-superseding`
- `sequence` = `0`
- `prev_hash` = `null`
- `event_type` = `wos.case.supersessionStarted`
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
- `author_event_hash` = `93dd707574bd3dca9664acbdfa66dde24feb298f5dae0e370f96e7894911f2ec`
- `canonical_event_hash` = `fe36ecae153d6d430a07b06ef57ddfe718d33cd579978c65125cf50882258278`

Generator: `fixtures/vectors/_generator/gen_append_011_to_015.py`.
