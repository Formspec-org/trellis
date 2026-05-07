# Derivation - `tamper/051-clock-calendar-mismatch`

Generator: `fixtures/vectors/_generator/gen_adr0067_clocks.py`.

The export ZIP is deterministic per Core section 18.1. The manifest signs and
digest-binds `010-events.cbor`, `020-inclusion-proofs.cbor`,
`025-consistency-proofs.cbor`, `030-signing-key-registry.cbor`,
`040-checkpoints.cbor`, and the registry snapshot. The positive vector also
digest-binds `open-clocks.json` through
`trellis.export.open-clocks.v1.open_clocks_digest`.

The third event is the resumed `clockStarted` segment. It keeps `clockId` and `clockKind` from the paused segment but changes `calendarRef`; verifiers localize `clock_calendar_mismatch` at `a8c4f033277dc239da74741066fb60fa38e1e03028b3b32efb6dd32113272a5a`.

