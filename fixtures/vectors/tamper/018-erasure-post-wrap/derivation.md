# tamper/018-erasure-post-wrap — derivation

Authority: ADR 0005 §Verifier obligations step 8 (subject-class `post_erasure_wrap`).

## Construction

From `gen_tamper_017_to_018.py` (`gen_tamper_018`). Event 0 hosts `trellis.erasure-evidence.v1` with opaque `kid_destroyed` (16× `0xb8`) and `key_class = subject`. Event 1 is signed under the issuer `kid` but includes a `key_bag` entry whose `recipient` equals `kid_destroyed`, with `authored_at > destroyed_at`.

## Expected report

| Field | Value |
|-------|--------|
| `structure_verified` | `true` |
| `integrity_verified` | `false` |
| `readability_verified` | `true` |
| `tamper_kind` | `post_erasure_wrap` |
| `failing_event_id` | `fbf9d93dddf1c62e40e0d5538f4a41256dfa1e83f62f534b8c41b930f8bca0f9` |
