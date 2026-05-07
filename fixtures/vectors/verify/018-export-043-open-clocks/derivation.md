# Derivation - `verify/018-export-043-open-clocks`

Generator: `fixtures/vectors/_generator/gen_adr0067_clocks.py`.

The export ZIP is deterministic per Core section 18.1. The manifest signs and
digest-binds `010-events.cbor`, `020-inclusion-proofs.cbor`,
`025-consistency-proofs.cbor`, `030-signing-key-registry.cbor`,
`040-checkpoints.cbor`, and the registry snapshot. The positive vector also
digest-binds `open-clocks.json` through
`trellis.export.open-clocks.v1.open_clocks_digest`.

