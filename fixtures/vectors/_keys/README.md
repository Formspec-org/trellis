# Pinned Test Keys

COSE_Key CBOR-encoded Ed25519 signing keys. Bytes are authoritative — no derivation procedure. Keys are generated once, committed, and referenced by vector manifests via `../../_keys/<name>.cose_key` paths.

| File | Role | Added |
|------|------|-------|
| issuer-001.cose_key | Primary issuer — append happy-path vectors | 2026-04-18 |
| issuer-002.cose_key | Successor issuer — signing-key rotation (§8) fixture `append/002-rotation-signing-key` | 2026-04-19 |
| recipient-004-ledger-service.cose_key | X25519 HPKE recipient for `append/004-hpke-wrapped-inline` | 2026-04-20 |
| ephemeral-004-recipient-001.cose_key | Fixture-only pinned X25519 HPKE ephemeral key for `append/004-hpke-wrapped-inline` | 2026-04-20 |
