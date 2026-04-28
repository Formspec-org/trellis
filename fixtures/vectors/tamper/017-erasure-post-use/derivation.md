# tamper/017-erasure-post-use — derivation

Authority: ADR 0005 §Verifier obligations step 8 (signing-class `post_erasure_use`).

## Construction

Deterministic bytes from `fixtures/vectors/_generator/gen_tamper_017_to_018.py` (`gen_tamper_017`). Event 0 is a genesis `trellis.erasure-evidence.v1` host whose extension declares `kid_destroyed` equal to the issuer COSE `kid` and `key_class = signing`, with `destroyed_at` equal to the host `authored_at`. Event 1 is a follow-up event with `authored_at > destroyed_at` still signed under that issuer `kid`.

## Expected report

| Field | Value |
|-------|--------|
| `structure_verified` | `true` |
| `integrity_verified` | `false` |
| `readability_verified` | `true` |
| `tamper_kind` | `post_erasure_use` |
| `failing_event_id` | `fde5712aa169c6689d97628c2435e4229d828c5a31ca577cf2c12f3635052ee8` (event 1 `canonical_event_hash`) |
