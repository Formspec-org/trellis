# Derivation — `tamper/041-timestamp-backwards`

Mutates `append/005-prior-head-chain`'s `authored_at` from 1745000001 to 1744999999 (1 second before the genesis event's 1745000000), recomputes `author_event_hash` from the modified preimage, and re-signs under issuer-001's real seed.

The hash chain is valid: event[1]'s `prev_hash` still points at event[0]'s canonical_event_hash. Signatures verify. The tamper surfaces exclusively at the ADR 0069 D-3 temporal-order check: `authored_at[1] < authored_at[0]` is an integrity failure classified as `timestamp_order_violation`.

§19 verifier walk:

- Step 4.a — kid resolves. PASS (both events).
- Step 4.b — signatures verify (genesis unchanged, chain event re-signed with real key). PASS.
- Step 4.c — payloads decode. PASS.
- Step 4.d — `author_event_hash` recomputes from the modified preimage and matches the stored value (we updated it). PASS.
- Step 4.e — `canonical_event_hash` recomputes from the re-encoded payload bytes. PASS.
- Step 4.h — `prev_hash` linkage: event[1]'s `prev_hash` = event[0]'s canonical_event_hash. PASS.
- Step 4.h-temporal (ADR 0069 D-3) — event[1].`authored_at` (1744999999) < event[0].`authored_at` (1745000000). **FAIL** — `timestamp_order_violation`.
- Step 9 — `integrity_verified` = false.

Distinguishes from prior tamper cases:

- tamper/005 = chain truncation; step 4.h `prev_hash` fails.
- tamper/006 = event reorder; step 4.h `prev_hash` fails.
- tamper/007 = `author_event_hash` mismatch; step 4.d fails.
- tamper/041 = THIS vector; hash chain + signatures valid; timestamps decrease; ADR 0069 D-3 step fails.
