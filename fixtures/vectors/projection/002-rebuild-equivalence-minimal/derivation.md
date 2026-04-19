# Derivation — `projection/002-rebuild-equivalence-minimal`

## Header

**What this vector exercises.** This vector is the SECOND O-3 projection-conformance fixture, and the FIRST to exercise Core §15.3's new "Rebuild-output encoding" pin landed in Wave 2 of the Trellis normalization effort. It implements Test 2 of `thoughts/specs/2026-04-18-trellis-o3-projection-conformance.md` — rebuild equivalence. A minimal 2-event canonical chain is checkpointed at `tree_size = 2`, and a 2-field projection view is built from it at checkpoint-time. A fresh rebuild of that view from the same canonical chain and the same procedure configuration produces output that is **byte-equal** to the original view — this is the executable form of Core §15.3's promise that "two conforming implementations that rebuild the same derived artifact from the same canonical events and configuration history MUST produce byte-equal output."

**Scope of this vector.** Rebuild equivalence only. Every field of the view is declared rebuild-deterministic per Core §15.3 + Companion §15.3 OC-40, so the byte-compare covers the full view — there is no non-deterministic portion to strip before comparison. Non-deterministic-field handling (per §15.3's "Fields whose determinism depends on external state … MUST be declared in the rebuild-path identifier as non-deterministic") is intentionally deferred to a later fixture; see the Core-gap note below.

**Why this view shape.** A 2-field view over a 2-event chain is the smallest construct that meaningfully exercises rebuild equivalence:

- `event_count: uint` — a rebuild-deterministic reduction over the chain (the replay length).
- `last_canonical_event_hash: digest` — a rebuild-deterministic pointer into the chain (the last canonical-event hash). This field also proves the rebuild agrees on *which* chain was replayed, not merely how long it was.

Any 1-field view would collapse to either the trivial `event_count` (which tells the runner nothing about chain identity) or the trivial `last_canonical_event_hash` (which tells the runner nothing about replay count). The 2-field shape is minimal-but-non-degenerate.

**Relationship to `projection/001-watermark-attestation`.** The two fixtures are stylistically parallel — both use a 2-event structural-only chain over the `x-trellis-test/` reserved event-type prefix (§14.6), both checkpoint at `tree_size = 2`, both use the `issuer-001` key from `../../_keys/issuer-001.cose_key`. They differ in:

- **Ledger scope.** `test-rebuild-ledger` vs. `test-projection-ledger`, so neither fixture's canonical bytes can be confused with the other's.
- **Event-type / idempotency / payload pins.** Distinct strings so the canonical-event hashes are distinct.
- **View shape.** `{event_count, last_canonical_event_hash}` vs. projection/001's `{body, watermark}` wrapper. projection/001 pins the Watermark subfields; projection/002 pins the rebuild procedure and the rebuilt-view bytes.
- **What the runner verifies.** projection/001's runner byte-compares a Watermark extracted from the view; projection/002's runner byte-compares a freshly-rebuilt view against the original.

**Companion / Core § roadmap (in traversal order).**

1. Core §5.1 — dCBOR encoding profile. Every CBOR structure below is serialized per §5.1's byte-wise lexicographic canonical ordering. The rebuild-equivalence claim rests on dCBOR: two implementations that agree on §5.1 produce identical bytes from identical inputs.
2. Core §8.3 — `kid` derivation, reused unchanged from `append/001`.
3. Core §9.3, §9.5, §9.2 + §9.1 — per-event hashing. Same construction as `projection/001`, applied twice with this fixture's distinct field pins.
4. Core §6.8 + §7.4 + §6.1 — authored / canonical / signed event surfaces. Emits two COSE_Sign1 envelopes.
5. Core §11.3 — Merkle tree construction at `tree_size = 2`.
6. Core §11.2 — `CheckpointPayload` struct, signed as COSE_Sign1. Committed as `input-checkpoint.cbor`.
7. Procedure configuration — pinned as `input-procedure-config.cbor`. Core §15.3 names "the declared configuration history of the derived processor" as a rebuild input but does NOT pin its byte-level shape; this fixture commits to a minimal 3-field configuration (see Step 4 below and the Core-gap note).
8. Original view construction — `dCBOR(View)` where `View = {event_count, last_canonical_event_hash}`. Committed as `input-view.cbor`.
9. Core §15.3 "Rebuild-output encoding" — the fresh rebuild. Consumes the chain + procedure-config, emits a View whose bytes are byte-equal to the original. Committed as `expected-view-rebuilt.cbor`.

**Pinned inputs.**

| Input | Value | Source |
|---|---|---|
| `signing_key` | Ed25519 COSE_Key, seed ending `…aa` (same issuer as `append/001`, `projection/001`) | `../../_keys/issuer-001.cose_key` |
| `ledger_scope` | bstr `"test-rebuild-ledger"` (19 bytes) | Core §10.4, §11.2 |
| Event 0 `sequence` | `0` (genesis) | §10.2: `prev_hash MUST be null` |
| Event 0 `event_type` | bstr `"x-trellis-test/rebuild-seed"` (27 bytes) | §14.6 reserved prefix |
| Event 0 `authored_at` | `1745100000` | §12.1 |
| Event 0 `idempotency_key` | bstr `"idemp-rebld-000"` + `0x00` (16 bytes) | §6.1 .size (1..64) |
| Event 0 `payload_bytes` | `"rebuild-payload-0"` + 15 × `0x00` (32 bytes) | PayloadInline ciphertext |
| Event 1 `sequence` | `1` | §10.2 |
| Event 1 `prev_hash` | `canonical_event_hash[0]` (§10.2) | computed below |
| Event 1 `event_type` | bstr `"x-trellis-test/rebuild-follow"` (29 bytes) | §14.6 |
| Event 1 `authored_at` | `1745100060` | §12.1 |
| Event 1 `idempotency_key` | bstr `"idemp-rebld-001"` + `0x00` (16 bytes) | §6.1 |
| Event 1 `payload_bytes` | `"rebuild-payload-1"` + 15 × `0x00` (32 bytes) | PayloadInline ciphertext |
| `checkpoint.timestamp` | `1745100120` | §11.2 |
| `checkpoint.anchor_ref` | `null` | §11.5 (Phase 1 null admissible) |
| `checkpoint.prev_checkpoint_hash` | `null` | §11.2 (first checkpoint in scope) |
| `checkpoint.extensions` | `null` | §11.6 (Phase 1 emit null or `{}`) |
| `procedure.rebuild_path` | `"trellis.projection.v1/rebuild-minimal"` | §15.3 |
| `procedure.view_schema_id` | `"urn:trellis:view:rebuild-minimal:v1"` | Companion §14.1 field #5 (URI-shaped) |
| `procedure.deterministic_fields` | `["event_count", "last_canonical_event_hash"]` | §15.3 + Companion §15.3 OC-40 |

---

## Body

### Step 1: Per-event construction (both events)

**Core § citation:** §6.8 (authored / canonical / signed surfaces), §9.3 (`content_hash`), §9.5 (`author_event_hash`), §9.2 (`canonical_event_hash`), §7.4 (COSE protected header + `Sig_structure`).

**Operation.** Identical in shape to `projection/001` Step 1 — the same `kid = af9dff525391faa75c8e8da4808b1743` (derived from `SUITE_ID = 1` and the `issuer-001` pubkey per §8.3) signs both events. Each event is constructed as:

1. `content_hash = SHA-256("trellis-content-v1" domain-sep over payload_bytes)` per §9.3.
2. Build `AuthorEventHashPreimage` per §9.5; dCBOR-encode; then `author_event_hash = SHA-256("trellis-author-event-v1" domain-sep over dCBOR(AuthorEventHashPreimage))` per §9.5 + §9.1.
3. Build `EventPayload` per §6.1 (adds `author_event_hash`), dCBOR-encode.
4. Build protected-header map `{1: -8, 4: kid, -65537: 1}`, dCBOR-encode, wrap in bstr (§7.4).
5. Build `Sig_structure = ["Signature1", protected_bstr, h'', payload_bstr]` per RFC 9052 §4.4 / §6.6, sign with Ed25519.
6. Assemble COSE_Sign1 tag-18 envelope `d2 84 <protected_bstr> a0 <payload_bstr> <signature_bstr>` per §6.1.
7. Build `CanonicalEventHashPreimage = {version: 1, ledger_scope, event_payload}` per §9.2, dCBOR-encode, then `canonical_event_hash = SHA-256("trellis-event-v1" domain-sep over dCBOR(CanonicalEventHashPreimage))`.

**Results.**

| Event | `canonical_event_hash` |
|---|---|
| 0 | `379fa66950e4aee8f92abcde3465b84a9b29f0905ca54b1b9746fedbfec996bf` |
| 1 | `9ad0556334071a0d40050c61ba4601506b87dbc4847d808fb3693b364af5090c` |

**Committed as:** the two COSE_Sign1 envelopes are wrapped into a single dCBOR array and written as `input-chain.cbor` (1348 bytes, CBOR major type 4, definite length 2).

---

### Step 2: Merkle root at `tree_size = 2`

**Core § citation:** §11.3 Merkle tree construction.

**Operation.**

```
leaf_0    = SHA-256("trellis-merkle-leaf-v1" domain-sep over canonical_event_hash[0])
leaf_1    = SHA-256("trellis-merkle-leaf-v1" domain-sep over canonical_event_hash[1])
tree_head = SHA-256("trellis-merkle-interior-v1" domain-sep over (leaf_0 || leaf_1))
```

**Result (32 bytes):**

```
715a0d5e26ac092dfca5015fbf53f8779080bbbef64cc70aa3d5546e2d5e88a6
```

**Committed as:** embedded in `input-checkpoint.cbor` (Step 3); reconstructible from the two `canonical_event_hash` values above.

---

### Step 3: Signed `Checkpoint` at `tree_size = 2`

**Core § citation:** §11.2 (`Checkpoint = COSESign1Bytes` carrying `CheckpointPayload`); §7.4 (COSE protected-header + Sig_structure, reused from Step 1).

**Field population** (same CDDL shape as `projection/001` Step 3, distinct scope / timestamp):

| Field | Value | Source |
|---|---|---|
| `version` | `1` | §11.2 |
| `scope` | `"test-rebuild-ledger"` (bstr, 19 bytes) | §11.2 |
| `tree_size` | `2` | count of events in chain |
| `tree_head_hash` | 32 bytes from Step 2 | §11.3 |
| `timestamp` | `1745100120` | §11.2 |
| `anchor_ref` | `null` | §11.5 (Phase 1 null admissible) |
| `prev_checkpoint_hash` | `null` | §11.2 (first checkpoint in scope) |
| `extensions` | `null` | §11.6 |

**Operation.** dCBOR-encode the 8-key `CheckpointPayload` (§5.1 byte-wise lex ordering). Sign the resulting bytes as the COSE_Sign1 payload, reusing the Step 1 protected-header bytes (`alg = -8, kid = af9dff…, suite_id = 1`). Assemble tag-18 envelope.

**Result (257 bytes):** committed as `input-checkpoint.cbor`. First 4 bytes are `0xd2 0x84 0x58 0x1b` (tag 18, array of 4, bstr length 27) — same COSE envelope shape as `projection/001`'s checkpoint; bytes differ because scope / timestamp / tree_head_hash differ.

---

### Step 4: Rebuild-procedure configuration

**Core § citation:** §15.3 Rebuild path — "`rebuild_path` is a deterministic identifier that, combined with the canonical events up to `tree_size` and with the declared configuration history of the derived processor, allows a recipient to rebuild the derived artifact …"

**What Core pins.** The `rebuild_path` identifier is "implementation-defined." Core §15.3 names "the declared configuration history of the derived processor" as a rebuild input, but does NOT pin the byte-level shape of that configuration history. This fixture commits to a minimal 3-field record as a reference shape; see the Core-gap note below for why the shape is fixture-local rather than spec-normative.

**Load-bearing CDDL (this fixture's local shape):**

```cddl
ProcedureConfig = {
  rebuild_path:         tstr,              ; matches Core §15.3 identifier
  view_schema_id:       tstr,              ; URI per Companion §14.1 field #5
  deterministic_fields: [* tstr],          ; §15.3 + Companion §15.3 OC-40
}
```

**Field population:**

| Field | Value | Source |
|---|---|---|
| `rebuild_path` | `"trellis.projection.v1/rebuild-minimal"` (37 bytes) | §15.3 |
| `view_schema_id` | `"urn:trellis:view:rebuild-minimal:v1"` (35 bytes) | Companion §14.1 (URI) |
| `deterministic_fields` | `["event_count", "last_canonical_event_hash"]` (2 entries) | §15.3 OC-40 |

**Operation.** dCBOR-encode the 3-key map (§5.1). Canonical key order by byte-wise length-then-lex: `rebuild_path` (12 chars, `0x6c`), `view_schema_id` (14 chars, `0x6e`), `deterministic_fields` (20 chars, `0x74`).

**Result (166 bytes):** first byte `0xa3` (map with 3 entries). Committed as `input-procedure-config.cbor`.

**Full hex (166 bytes, sha256 `ab71c507ae45fc847cd18dac099fefd75bb594cb360d9426146b69cf5e94ecda`):**

```
a36c726562756964... (full dump below in the footer)
```

---

### Step 5: Original `View` at checkpoint-time

**Core § citation:** §15.3 Rebuild path (Rebuild-output encoding — "Rebuilt derived artifacts MUST use dCBOR (§5) as their canonical encoding whenever the artifact shape admits CBOR serialization").

**Load-bearing CDDL:**

```cddl
View = {
  event_count:               uint,
  last_canonical_event_hash: digest,     ; digest = bstr .size 32
}
```

**Why these two fields.** Both are pure functions of the canonical chain at `tree_size = 2`: `event_count = 2` is the replay length; `last_canonical_event_hash = canonical_event_hash[1]` is the tail hash. Neither depends on wall-clock time, per-implementation resource IDs, or any state outside the canonical chain + the pinned procedure config. Both are therefore **rebuild-deterministic** per Core §15.3 + Companion §15.3 OC-40.

**Field population:**

| Field | Value | Source |
|---|---|---|
| `event_count` | `2` | Step 1 — count of events in chain |
| `last_canonical_event_hash` | `9ad0556334071a0d40050c61ba4601506b87dbc4847d808fb3693b364af5090c` | Step 1 — `canonical_event_hash[1]` |

**Operation.** dCBOR-encode the 2-key map (§5.1). Canonical key order by byte-wise length-then-lex: `event_count` (11 chars, `0x6b`) < `last_canonical_event_hash` (25 chars, `0x78 0x19`).

**Result (75 bytes):** first byte `0xa2` (map with 2 entries). Committed as `input-view.cbor`.

**Full hex (75 bytes, sha256 `72e0ad532f7533d00631188a4403f0f00f751cc65fe0302b982c91d48fffa6da`):**

```
a26b6576656e745f636f756e7402
78196c6173745f63616e6f6e6963616c5f6576656e745f68617368
58209ad0556334071a0d40050c61ba4601506b87dbc4847d808fb3693b364af5090c
```

---

### Step 6: Fresh rebuild → `expected-view-rebuilt.cbor`

**Core § citation:** §15.3 "Rebuild-output encoding":

> "Rebuilt derived artifacts MUST use dCBOR (§5) as their canonical encoding whenever the artifact shape admits CBOR serialization. Two conforming implementations that rebuild the same derived artifact from the same canonical events and configuration history MUST produce byte-equal output. Fields whose determinism depends on external state (wall-clock timestamps, per-implementation resource IDs) MUST be declared in the rebuild-path identifier as non-deterministic; byte-equality is required over the declared-deterministic portion only (Companion §15.3 OC-40)."

**Companion citation:** §15.3 OC-39 (rebuilt output MUST yield semantically equivalent output for every rebuild-deterministic field), OC-40 (producer MUST declare which fields are rebuild-deterministic).

**Operation.** Re-execute the rebuild procedure from first principles:

1. Load `input-chain.cbor` → CBOR-decode → array of two COSE_Sign1 envelopes.
2. For each envelope, extract the `EventPayload`, compute `canonical_event_hash` per §9.2 (same Step 1 construction). Result is `[ceh_0, ceh_1]`.
3. Load `input-procedure-config.cbor` → assert `view_schema_id == "urn:trellis:view:rebuild-minimal:v1"`.
4. Build `View = {event_count: 2, last_canonical_event_hash: ceh_1}` — the two fields are pure reductions over the `canonical_event_hash` sequence.
5. dCBOR-encode. Per Core §15.3 "Rebuild-output encoding" this encoding MUST be dCBOR; the canonical-key ordering is what produces byte-identical output across implementations.

**Result (75 bytes, sha256 `72e0ad532f7533d00631188a4403f0f00f751cc65fe0302b982c91d48fffa6da`):** byte-identical to `input-view.cbor`. Committed as `expected-view-rebuilt.cbor`.

**Why bit-equal, not field-equal.** This fixture declares every view field as rebuild-deterministic (there are no non-deterministic fields to strip before comparison), so "byte-equality over the declared-deterministic portion" (Core §15.3) reduces to "byte-equality over the whole artifact" — the bytes of `input-view.cbor` and `expected-view-rebuilt.cbor` are identical modulo zero intermediate steps. A runner can verify by `sha256(input-view.cbor) == sha256(expected-view-rebuilt.cbor)`.

---

### Runner contract recap

Per O-3 design §"Runner contract / Test 2", a conforming runner:

1. Replays `input-chain.cbor` in canonical order.
2. Loads `input-procedure-config.cbor` and applies the rebuild procedure named by `procedure.rebuild_path` (here: `trellis.projection.v1/rebuild-minimal`).
3. dCBOR-encodes the rebuilt view.
4. Byte-compares against `expected-view-rebuilt.cbor` across every field in `deterministic_fields = ["event_count", "last_canonical_event_hash"]`.
5. Because this fixture declares NO non-deterministic fields, the byte-compare covers the full view and is equivalent to `sha256(rebuilt) == sha256(expected)`.
6. The runner MAY additionally byte-compare the rebuild against `input-view.cbor` — this fixture asserts the original and the rebuild agree, which they do byte-for-byte.

**Pass/fail.** Pass iff the rebuilt bytes equal `expected-view-rebuilt.cbor` across every declared-deterministic field. Fail on any mismatch in a deterministic field, or on presence of any field not declared in `deterministic_fields`.

---

## Core-gap note (informational)

Core §15.3's new "Rebuild-output encoding" paragraph says:

> "Fields whose determinism depends on external state (wall-clock timestamps, per-implementation resource IDs) MUST be declared in the rebuild-path identifier as non-deterministic."

The **normative data structure** for that declaration is NOT pinned in §15.3. Two candidate shapes:

1. Encode the non-deterministic-field list **inside** the `rebuild_path` string itself (e.g., `"trellis.projection.v1/default?nd=built_at,resource_id"` — URL-query-style). Keeps the declaration inside the identifier as §15.3's prose literally says, at the cost of smuggling structured data through an opaque `tstr`.
2. Extend the rebuild-procedure configuration record (Core §15.3 "the declared configuration history") with a top-level `non_deterministic_fields` list, paralleling this fixture's `deterministic_fields`. Cleaner as data but stretches §15.3's "declared in the rebuild-path identifier" phrasing.

For this fixture the gap does NOT bite: every view field is deterministic, so no declaration is needed. But a future rebuild fixture with a `build_timestamp`, a `resource_uri`, or any other externally-dependent field will have to pick a shape before the runner knows which bytes to strip before comparing. Escalated to `thoughts/specs/2026-04-18-trellis-core-gaps-surfaced-by-g3.md` as an append-only note.

---

## Footer — full hex dumps

### `input-view.cbor` (75 bytes, sha256 `72e0ad532f7533d00631188a4403f0f00f751cc65fe0302b982c91d48fffa6da`)

```
a26b6576656e745f636f756e7402 78196c6173745f63616e6f6e6963
616c5f6576656e745f686173685820 9ad0556334071a0d40050c61ba46
01506b87dbc4847d808fb3693b364af5090c
```

Structure: `0xa2` (map of 2); `event_count` (key `0x6b` + 11 bytes) = `0x02`; `last_canonical_event_hash` (key `0x78 0x19` + 25 bytes) = `0x58 0x20 <32-byte digest>`.

### `expected-view-rebuilt.cbor` (75 bytes, sha256 `72e0ad532f7533d00631188a4403f0f00f751cc65fe0302b982c91d48fffa6da`)

Byte-identical to `input-view.cbor` — this is the rebuild-equivalence promise.

### `input-procedure-config.cbor` (166 bytes, sha256 `ab71c507ae45fc847cd18dac099fefd75bb594cb360d9426146b69cf5e94ecda`)

```
a36c72656275696c645f7061746878257472656c6c69732e70726f6a6563
74696f6e2e76312f72656275696c642d6d696e696d616c6e766965775f73
6368656d615f6964782375726e3a7472656c6c69733a766965773a726562
75696c642d6d696e696d616c3a7674646574657276696e69737469635f66
69656c6473826b6576656e745f636f756e7478196c6173745f63616e6f6e
6963616c5f6576656e745f68617368
```

Structure: `0xa3` (map of 3); three keys in dCBOR-canonical order `rebuild_path`, `view_schema_id`, `deterministic_fields`.

### `input-checkpoint.cbor` (257 bytes, sha256 `ca1370c780cf1219e081a100051d918eac74c2b76c0722bc8099939fd97b38b5`)

```
d284581ba301270450af9dff525391faa75c8e8da4808b17433a0001000001a0
589da86573636f706553746573742d72656275696c642d6c65646765726776
657273696f6e016974696d657374616d701a68041d5869747265655f73697a
65026a616e63686f725f726566f66a657874656e73696f6e73f66e74726565
5f686561645f686173685820715a0d5e26ac092dfca5015fbf53f8779080bb
bef64cc70aa3d5546e2d5e88a674707265765f636865636b706f696e745f68
617368f6584078fce7361e9cc64e93685e1a927db795a5cf677fc8ae957528
7f4e794c2425c6773d75fd684cc2ef4b2000afa0abb68960d394db8146392c
4480cbe146473c02
```

Structure: `0xd2` (tag 18); `0x84` (array of 4); `0x58 0x1b <27-byte protected header>` (same bytes as `projection/001`'s checkpoint protected header — `kid`/`alg`/`suite_id` unchanged); `0xa0` (empty unprotected); `0x58 0x9d <157-byte CheckpointPayload>`; `0x58 0x40 <64-byte Ed25519 signature>`.

### `input-chain.cbor` (1348 bytes, sha256 `1602d17f2477e9f151755a3fc46cf577d711b4037742e33a093ac58f15455912`)

CBOR array of two COSE_Sign1 envelopes (`0x82` = array of 2, then `0xd2 …` for each envelope). The two envelopes share the same 27-byte protected-header bstr as `projection/001` (same `kid`, `alg`, `suite_id`). Event 0's `EventPayload` uses `ledger_scope = "test-rebuild-ledger"`, `event_type = "x-trellis-test/rebuild-seed"`, `idempotency_key = "idemp-rebld-000\x00"`, `authored_at = 1745100000`; event 1's `EventPayload` uses `event_type = "x-trellis-test/rebuild-follow"`, `idempotency_key = "idemp-rebld-001\x00"`, `authored_at = 1745100060`, and `prev_hash = canonical_event_hash[0] = 379fa66950e4aee8f92abcde3465b84a9b29f0905ca54b1b9746fedbfec996bf`.

Reader-verifiable intermediates (embedded in the chain bytes):

- Event 0 `canonical_event_hash` = `379fa66950e4aee8f92abcde3465b84a9b29f0905ca54b1b9746fedbfec996bf`
- Event 1 `prev_hash` = Event 0 `canonical_event_hash` (§10.2).
- Event 1 `canonical_event_hash` = `9ad0556334071a0d40050c61ba4601506b87dbc4847d808fb3693b364af5090c`

These two values feed Step 2 (Merkle root) and Step 5/6 (`last_canonical_event_hash` in both views).
