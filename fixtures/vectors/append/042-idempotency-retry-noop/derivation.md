# Derivation — `append/042-idempotency-retry-noop`

## Header

**What this vector exercises.** This vector pins the byte-equal retry surface of Core §17.3 clauses 1 + 2: for a fixed `(ledger_scope, idempotency_key)` and byte-identical authored inputs, every successful retry MUST resolve to the byte-identical canonical event. Concretely the vector commits one full append pipeline (authored / canonical / signed / append-head) and the in-vector generator asserts that re-running the pipeline against the same inputs produces byte-identical intermediates at each step (`payload_nonce`, `ciphertext`, `content_hash`, `author_event_hash`, `canonical_event_hash`).

**Relationship to `append/041-aead-retry-determinism`.** The two vectors share the §9.4 deterministic-nonce + retry-byte-equality property. They differ only on `idempotency_key` (`idemp-append-042` vs `idemp-append-041`) and `event_type` (`x-trellis-test/idempotency-retry-noop` vs `x-trellis-test/aead-retry-determinism`). The pair demonstrates that the byte-equal retry property is not an artifact of one specific key — it holds across the `idempotency_key` value space, as §17.3 / §9.4 require.

**Pinned inputs.**

| Input | Value | Source |
|---|---|---|
| `signing_key` | Ed25519 COSE_Key, seed ending `…aa` | `../../_keys/issuer-001.cose_key` (shared with `append/041`) |
| `payload` | 64 bytes, ASCII `"Trellis fixture payload #001"` + `0x00` padding | `../../_inputs/sample-payload-001.bin` (shared with `append/041`) |
| `ledger_scope` | bstr `"test-response-ledger"` | §10.4 ledger scope |
| `sequence` | `0` (genesis) | §10.2: `prev_hash` MUST be `null` for `sequence == 0` |
| `timestamp` (`authored_at`) | `1745000042` | §12.1 `authored_at: uint` |
| `event_type` | bstr `"x-trellis-test/idempotency-retry-noop"` | §14.6 reserved test-identifier prefix |
| `classification` | bstr `"x-trellis-test/unclassified"` | §14.6 reserved test-identifier prefix |
| `retention_tier` | `0` | §12.1 |
| `idempotency_key` | bstr `"idemp-append-042"` (16 bytes) | §6.1 `idempotency_key: bstr .size (1..64)` |
| `suite_id` | `1` | §7.1 Phase 1 pin |
| `recipient` | `"ledger-service"` | shared with `append/041`'s recipient X25519 key |
| `ephemeral seed` (per-recipient) | distinct from `041` (one-byte flip in tail) | `../../_keys/ephemeral-042-recipient-001.cose_key` |

**Core § roadmap (in traversal order).** Identical to `append/041`'s — see that vector's derivation for the per-step construction. The byte-level differences flow purely from the `idempotency_key` and `event_type` substitutions: distinct `idempotency_key` ⇒ distinct salt for the §9.4 HKDF nonce derivation ⇒ distinct `payload_nonce` ⇒ distinct `ciphertext` ⇒ distinct `content_hash` ⇒ distinct `author_event_hash` ⇒ distinct `canonical_event_hash` ⇒ distinct signed envelope and append head. None of these distinctions weaken the §17.3 byte-equal retry property; they re-prove it under a different key.

---

## Body

### Step 1: §9.4 deterministic AEAD nonce

`payload_nonce = HKDF-SHA256(salt = dCBOR("idemp-append-042"), ikm = SHA-256(plaintext), info = "trellis-payload-nonce-v1", length = 12)`. The dCBOR encoding of the 16-byte text-string-shaped bstr `"idemp-append-042"` is `0x50 + idemp-append-042`. The HKDF-SHA256 output is byte-determined by these inputs alone.

**Result:** `payload_nonce = 90e46949e504f6cea008554b` (12 bytes; see footer dump for full intermediate trace via the generator output).

### Step 2: ChaCha20-Poly1305 encryption

`ciphertext = ChaCha20Poly1305(DEK).encrypt(payload_nonce, plaintext, aad=h'')` where `DEK = bytes(range(32))` (shared with `append/041`).

### Step 3: §9.3 `content_hash` over ciphertext

`content_hash = SHA-256("trellis-content-v1" domain-separated over ciphertext) = 8cdeeafa07b3f836a12cff277969b6f81c4d354489b226b25c76a17d21306fc7`.

### Step 4–12: identical construction to `append/041`

Authored map → `author_event_hash` (`trellis-author-event-v1` domain) → canonical `EventPayload` (with `idempotency_key` as the twelfth lex-sorted text key, encoding `0x6f 6964656d706f74656e63795f6b6579`) → `kid`-derived COSE protected header → RFC 9052 `Sig_structure` → Ed25519 signature → tag-18 COSE_Sign1 → `canonical_event_hash` over `dCBOR(CanonicalEventHashPreimage)` → `AppendHead`. All twelve steps are byte-deterministic from the pinned inputs.

### Retry-noop property (§17.3 clauses 1 + 2)

The generator runs the full pipeline twice and asserts byte equality of every intermediate (`payload_nonce`, `ciphertext`, `content_hash`, `authored_bytes`, `author_event_hash`, `event_payload_bytes`, `canonical_event_hash`). A producer that re-runs this generator (which is exactly what a Phase-1 retry under §17.3 amounts to: same inputs → same outputs) MUST observe byte equality at every level. Failure of any one assertion in the generator is a producer-side determinism break and a §17.3 violation.

---

## Footer — committed intermediates

| File | Size | SHA-256 |
|---|---:|---|
| `input-author-event-hash-preimage.cbor` | 708 | `b15b86057efc4c497cf97e18715835f4690d2cc5636cbd0cac6c1c44513464b4` |
| `author-event-preimage.bin` | 739 | `0e8a55162d09ee4fed060890f1182a371bc19ebe1f0963097a0cea5c780e9a81` (same as `author-event-hash.bin` content) |
| `author-event-hash.bin` | 32 | `0e8a55162d09ee4fed060890f1182a371bc19ebe1f0963097a0cea5c780e9a81` |
| `expected-event-payload.cbor` | 760 | `0c07eb6e5e9621c18477477edb961d045ba4295585c00a952a997c10d4c42798` |
| `sig-structure.bin` | 805 | `caaae4370fe5b9e7c9216d8aa340de153182f8fc5b82ace7e3a8daee8141c8b7` |
| `expected-event.cbor` | 861 | `6b484f35097378e6de044f917342095567e24f361009c21c87dd2946e80638fa` |
| `expected-append-head.cbor` | 93 | `61cb5aa343335c42f42c64f1cc853b70347c4526f395f5f42d8a4f140907d77e` |

**Map prefix witnesses (TR-CORE-161):** `input-author-event-hash-preimage.cbor[0] = 0xac` (12-entry map); `expected-event-payload.cbor[0] = 0xad` (13-entry map = 12 authored + `author_event_hash`). The `idempotency_key` field rides in both maps; per §5.1 it sorts last in the authored map (encoding `0x6f 6964656d706f74656e63795f6b6579 + 0x50 + idemp-append-042`) and second-to-last in the canonical map (with `author_event_hash` sorting last).
