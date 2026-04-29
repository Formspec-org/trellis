# Derivation — `tamper/035-idempotency-key-payload-mismatch`

## Header

**What this vector exercises.** The tamper surface here is **wire-contract idempotency**: the ledger contains two events that pass every Core §19 step 4 individual check (signature, hash recomputation, prev_hash linkage, sequence monotonicity), but they violate Core §17.3 by sharing `(ledger_scope, idempotency_key)` with divergent canonical material. The verifier MUST detect the duplicate idempotency identity, flip `integrity_verified = false`, and surface `tamper_kind = "idempotency_key_payload_mismatch"` (§17.5) on the conflicting event.

**Distinct from existing chain-integrity tampers.** `tamper/005-chain-truncation` and `tamper/006-event-reorder` break §10.2 chain linkage; signature and hash recomputation pass at the per-event level. **This vector is the inverse** — every per-event check passes, including chain linkage; the violation is only at the §17.3 ledger-walk layer.

**Why this is testable as a verifier-side surface.** Per Core §17.3, the Canonical Append Service MUST reject the conflicting submission with `IdempotencyKeyPayloadMismatch` (§17.5). A buggy or malicious operator that admits both events anyway produces the ledger encoded in this vector; the offline stranger holding only the export ZIP detects the violation by walking the canonical events and observing the duplicate `(ledger_scope, idempotency_key)` identity. The detection is offline, deterministic, and depends on no operator-side state — exactly the verification-independence property of Core §16.

**Pinned inputs.**

| Input | Value |
|---|---|
| `signing_key` | `_keys/issuer-001.cose_key` (Ed25519, suite_id=1) |
| `ledger_scope` | bstr `"test-response-ledger"` |
| `colliding idempotency_key` | bstr `"idemp-tamper-035-collide"` (24 bytes, well within `bstr .size (1..64)`) |
| event 0 plaintext | `"Trellis tamper/035 payload A: original" + 26 zero bytes` (64 bytes) |
| event 1 plaintext | `"Trellis tamper/035 payload B: conflict" + 26 zero bytes` (64 bytes) |
| event 0 sequence / prev_hash / authored_at | `0` / `null` / `1745000350` |
| event 1 sequence / prev_hash / authored_at | `1` / `b2bb4eeb…9d64` (event 0 canonical hash) / `1745000351` |

Both events use identical `event_type` (`x-trellis-test/append-minimal`), `classification` (`x-trellis-test/unclassified`), `retention_tier` (0), structural-only `PayloadInline.nonce` (`12 × 0x00`), and empty `KeyBag` — the only deltas are sequence / prev_hash / authored_at / plaintext.

---

## Body

### Construction (per event)

For each event the construction is identical to `append/001-minimal-inline-payload`'s steps 3–10:

1. `content_hash = SHA-256("trellis-content-v1" domain-separated over plaintext)` (§9.3).
2. Build `AuthorEventHashPreimage` map (§28 CDDL); dCBOR-encode (§5.1).
3. `author_event_hash = SHA-256("trellis-author-event-v1" domain-separated over authored bytes)` (§9.5).
4. Build `EventPayload` map (authored map + `author_event_hash`); dCBOR-encode (§5.1).
5. Derive `kid = SHA-256(dcbor(suite_id) || pubkey)[0..16]` (§8.3) — same `kid` for both events (same issuer).
6. Build COSE protected header `{1: -8, 4: kid, -65537: 1}`; dCBOR-encode + bstr-wrap (§7.4).
7. Build `Sig_structure = ["Signature1", protected_bstr, h'', payload_bstr]` per RFC 9052 §4.4.
8. Sign with Ed25519 (issuer-001 seed) (§7.1).
9. Assemble tag-18 COSE_Sign1 envelope (§6.1, §7.4).
10. `canonical_event_hash = SHA-256("trellis-event-v1" domain-separated over `dCBOR(CanonicalEventHashPreimage)`)` (§9.2).

Both events pass each step. Both signatures are valid Ed25519 over their respective `Sig_structure` bytes; both `author_event_hash` and `canonical_event_hash` recomputations match the stored canonical material.

### The §17.3 violation

Per Core §17.3, the idempotency identity is the pair `(ledger_scope, idempotency_key)`. After step 10 for both events:

| Field | Event 0 | Event 1 | §17.3 verdict |
|---|---|---|---|
| `ledger_scope` | `test-response-ledger` | `test-response-ledger` | **same** |
| `idempotency_key` | `idemp-tamper-035-collide` | `idemp-tamper-035-collide` | **same** |
| `content_hash` | `73952b7e…cd7f` | `a3e16af6…2f58` | **different** |
| `canonical_event_hash` | `b2bb4eeb…9d64` | `468782 62…39db` | **different** |

The `(ledger_scope, idempotency_key)` pair is identical; the canonical hash and content hash differ. Per §17.3 clause 3, the conflicting submission MUST be rejected with `IdempotencyKeyPayloadMismatch` (§17.5). A verifier reading this ledger MUST flag the violation as `tamper_kind = idempotency_key_payload_mismatch` on event 1 (the second-detected conflicting event).

### Why event 1 is the failing event (not event 0)

§17.3 clause 1 admits the first successful submission as the canonical reference. Event 0 (sequence=0) is the prior canonical event. Event 1 (sequence=1) is the conflict that, in a §17.3-honoring Canonical Append Service, would have been rejected at admission. The verifier reading the ledger surfaces the violation on event 1 because event 1 is the one that should not be in the canonical record. The `failing_event_id` field in the report is event 1's `canonical_event_hash` (`468782 62…39db`).

### Verifier obligations

The verifier MUST:

1. Walk the canonical events in array order.
2. For each event, parse the `(ledger_scope, idempotency_key)` pair from the canonical event.
3. Track seen identities in a per-scope set. On encountering a duplicate `(scope, key)` whose recomputed `content_hash` (or `author_event_hash` or `canonical_event_hash`) differs from the prior occurrence in the same scope, flip `integrity_verified = false` and emit `tamper_kind = "idempotency_key_payload_mismatch"`.
4. Report `failing_event_id` as the conflicting event's `canonical_event_hash` (the second-seen occurrence, by ledger array order).

The check is offline (Core §16). No state from the Canonical Append Service is needed; the canonical events themselves carry the `idempotency_key` field (§28 CDDL), and the duplicate-detection is purely local to the ledger walk.

---

## Footer — committed intermediates

| File | Size | SHA-256 |
|---|---:|---|
| `input-tampered-event-at-index-0.cbor` | 700 | `500c9f2abaeb4d09919ffa21586d1a25622211fa49635fe6bc6210871877d375` |
| `input-tampered-event-at-index-1.cbor` | 733 | `502601bd3f01a12b997f470c1462f5495adff84088d0363ed1e1b657b3130340` |
| `input-tampered-ledger.cbor` | 1434 | `9da3c6359363bf7805225a4935d9539d8fe0f21aabf0717dec6b3f1b96c5466e` |
| `input-signing-key-registry.cbor` | 133 | `4f0efcbe40658fe661406d686007c3b8f1abf66132b3271f1e02799d72b41d08` |

| Cryptographic intermediates | Value |
|---|---|
| `kid` (issuer-001) | `af9dff525391faa75c8e8da4808b1743` |
| Event 0 `content_hash` | `73952b7eca5028a92171f979985a4b918d07952ce0e19085629ab4140fd4cd7f` |
| Event 1 `content_hash` | `a3e16af629cdaf815690c996d853a313532ea8158772a0fb955084a3c2f62f58` |
| Event 0 `canonical_event_hash` | `b2bb4eebf3c7820edaf01f11204f8b2c86a0b0df07fb5a5fc80c8ead33179d64` |
| Event 1 `canonical_event_hash` | `468782621c70449203c9a17afe2d518a9aa4afab4fd80c9600b3c03bcf1139db` (the `failing_event_id`) |
