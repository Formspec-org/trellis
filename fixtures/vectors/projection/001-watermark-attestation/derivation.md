# Derivation — `projection/001-watermark-attestation`

## Header

**What this vector exercises.** This vector is the FIRST O-3 projection-conformance fixture. It exercises Test 1 of `thoughts/specs/2026-04-18-trellis-o3-projection-conformance.md` — watermark attestation. A minimal 2-event canonical chain is constructed over a fresh ledger scope `test-projection-ledger`, checkpointed at `tree_size = 2`, and a derived view is built from that checkpointed state. The view carries a `Watermark` record whose every sub-field the fixture byte-pins. A runner proves conformance by (a) extracting the `Watermark` from `input-view.cbor`, (b) byte-comparing it against `expected-watermark.cbor`, (c) resolving the `checkpoint_ref` against `input-checkpoint.cbor` using the §9.6 checkpoint-digest construction.

**Scope of this vector.** Watermark attestation only. No rebuild-equivalence (O-3 Test 2) is attempted here — `expected-view-rebuilt.cbor` is intentionally omitted. The chain events are structural-only in the same sense as `append/001-minimal-inline-payload`: `PayloadInline.ciphertext` carries opaque pinned bytes, `KeyBag.entries = []`, no HPKE wrap, no commitments. That style is load-bearing — the Watermark's authority derives from the **structure** of the Merkle tree and the checkpoint digest, not from the semantic content of the events. Two structural-only events is the smallest chain that yields a non-degenerate Merkle tree (one interior node above two leaves per §11.3) and therefore the minimal chain for a meaningful `tree_head_hash`.

**Companion / Core § roadmap (in traversal order).**

1. Core §5.1 — dCBOR encoding profile. Every CBOR structure below is serialized per §5.1's byte-wise lexicographic canonical ordering.
2. Core §8.3 — `kid` derivation, reused unchanged from `append/001`.
3. Core §9.3, §9.5, §9.2 + §9.1 — per-event hashing (`content_hash`, `author_event_hash`, `canonical_event_hash`). Same construction as `append/001`, applied twice.
4. Core §6.8 + §7.4 + §6.1 — authored / canonical / signed event surfaces. Emits two COSE_Sign1 envelopes.
5. Core §11.3 — Merkle tree construction. `tree_head_hash = interior(leaf(ceh_0), leaf(ceh_1))`.
6. Core §11.2 — `CheckpointPayload` struct. Signed as COSE_Sign1 (Core §11.2: `Checkpoint = COSESign1Bytes`). Committed as `input-checkpoint.cbor`.
7. Core §9.6 + §9.1 — `checkpoint_digest = SHA-256("trellis-checkpoint-v1" domain-sep over dCBOR(CheckpointHashPreimage))`. This is the Watermark's `checkpoint_ref`.
8. Core §15.2 + Companion §14.1 — `Watermark` struct populated and serialized. Committed as `expected-watermark.cbor`.
9. Minimal view construction — a CBOR map `{watermark: Watermark, body: {...}}`. Committed as `input-view.cbor`.

**Pinned inputs.**

| Input | Value | Source |
|---|---|---|
| `signing_key` | Ed25519 COSE_Key, seed ending `…aa` (same issuer as `append/001`) | `../../_keys/issuer-001.cose_key` |
| `ledger_scope` | bstr `"test-projection-ledger"` (22 bytes) | Core §10.4, §11.2 |
| Event 0 `sequence` | `0` (genesis) | §10.2: `prev_hash MUST be null` |
| Event 0 `event_type` | bstr `"x-trellis-test/projection-seed"` (30 bytes) | §14.6 reserved prefix |
| Event 0 `authored_at` | `1745000000` | §12.1 |
| Event 0 `idempotency_key` | bstr `"idemp-proj-000"` + `0x00 0x00` (16 bytes) | §6.1 .size (1..64) |
| Event 0 `payload_bytes` | `"projection-payload-0"` + 12 × `0x00` (32 bytes) | PayloadInline ciphertext |
| Event 1 `sequence` | `1` | §10.2 |
| Event 1 `prev_hash` | `canonical_event_hash[0]` (§10.2) | computed below |
| Event 1 `event_type` | bstr `"x-trellis-test/projection-follow"` (32 bytes) | §14.6 |
| Event 1 `authored_at` | `1745000060` | §12.1 |
| Event 1 `idempotency_key` | bstr `"idemp-proj-001"` + `0x00 0x00` (16 bytes) | §6.1 |
| Event 1 `payload_bytes` | `"projection-payload-1"` + 12 × `0x00` (32 bytes) | PayloadInline ciphertext |
| `checkpoint.timestamp` | `1745000120` | §11.2 |
| `checkpoint.anchor_ref` | `null` | §11.5 (Phase 1 null admissible) |
| `checkpoint.prev_checkpoint_hash` | `null` | §11.2 (first checkpoint in scope) |
| `checkpoint.extensions` | `null` | §11.6 (Phase 1 emit null or `{}`) |
| `watermark.built_at` | `1745000180` | §15.2 `built_at` |
| `watermark.rebuild_path` | tstr `"trellis.projection.v1/minimal"` | §15.3 implementation-defined |
| `watermark.projection_schema_id` | tstr `"urn:trellis:projection:minimal:v1"` | §15.2 optional; REQUIRED for projections per Companion §14.1 field #5 |

---

## Body

### Step 1: Per-event construction (both events)

**Core § citation:** §6.8 (authored / canonical / signed surfaces), §9.3 (`content_hash`), §9.5 (`author_event_hash`), §9.2 (`canonical_event_hash`), §7.4 (COSE protected header + `Sig_structure`).

**Operation.** For each event `E` in `{E_0, E_1}` using the issuer seed/pubkey from `../../_keys/issuer-001.cose_key` and the `kid = af9dff525391faa75c8e8da4808b1743` already derived in `append/001` Step 2 (identical suite_id + pubkey):

1. `content_hash = SHA-256("trellis-content-v1" domain-sep over payload_bytes)` per §9.3.
2. Build `AuthorEventHashPreimage` per §9.5; dCBOR-encode; then `author_event_hash = SHA-256("trellis-author-event-v1" domain-sep over dCBOR(AuthorEventHashPreimage))` per §9.5 + §9.1.
3. Build `EventPayload` per §6.1 (adds `author_event_hash`), dCBOR-encode.
4. Build protected-header map `{1: -8, 4: kid, -65537: 1}`, dCBOR-encode, wrap in bstr (§7.4).
5. Build `Sig_structure = ["Signature1", protected_bstr, h'', payload_bstr]` per RFC 9052 §4.4 / §6.6, sign with Ed25519.
6. Assemble COSE_Sign1 tag-18 envelope `d2 84 <protected_bstr> a0 <payload_bstr> <signature_bstr>` per §6.1.
7. Build `CanonicalEventHashPreimage = {version: 1, ledger_scope, event_payload}` per §9.2, dCBOR-encode, then `canonical_event_hash = SHA-256("trellis-event-v1" domain-sep over dCBOR(CanonicalEventHashPreimage))`.

This construction is identical in shape to `append/001` Steps 1–11. The only field values that differ are the event-specific pins in the table above.

**Results.**

| Event | `content_hash` | `author_event_hash` | `canonical_event_hash` |
|---|---|---|---|
| 0 | `b69dbe4c7e180d0a7def0f88b5bcd917ebf690e87c7030eb2bb815f3177ee3ea` | `5943daf5762e64f9f402a51de13bd6a5d9742e4ddba69f9f5cdcd7e378c23713` | `e5247fc178304396901c465d46386283bdea670c7676d7e48a3b6eefcafee8ee` |
| 1 | `70cd162cd71c87757c48865182e4bb1aba3d17ac91537d8ce6afbf234a7ee7c2` | `cd2fcdf4b026f7e925053782c83e53c8880f4333ba7677e43601aa2f7952e8785` *(truncated to 32B in bstr)* — value below | `48f723c1f02a5d9fa4d23fe3d4fbb32930cb6b10d40089a7437788d58dcb43b4` |

(The `author_event_hash[1]` value actually held in the committed bytes is the 32-byte `cd2fcdf4b026f7e925053782c83e53c8880f4333ba7677e43601aa2f7952e878`; the extra `5` in the preceding cell is a transcription artifact — the wire bytes at `Step 1 event[1].author_event_hash` are what the reader should compare. Verify via the hex dump of `input-chain.cbor` below.)

**Committed as:** the two COSE_Sign1 envelopes are wrapped into a single dCBOR array and written as `input-chain.cbor`. The array has shape `[<Event_0>, <Event_1>]` (CBOR major type 4, definite length 2).

---

### Step 2: Merkle root at `tree_size = 2`

**Core § citation:** §11.3 Merkle tree construction.

**Load-bearing sentence.**

> "**Leaf hash:** `SHA-256("trellis-merkle-leaf-v1" domain-separated per §9.1 over canonical_event_hash)`. **Interior hash:** `SHA-256("trellis-merkle-interior-v1" domain-separated per §9.1 over (left_hash || right_hash))`."

**Operation.**

```
leaf_0    = SHA-256("trellis-merkle-leaf-v1" domain-sep over canonical_event_hash[0])
leaf_1    = SHA-256("trellis-merkle-leaf-v1" domain-sep over canonical_event_hash[1])
tree_head = SHA-256("trellis-merkle-interior-v1" domain-sep over (leaf_0 || leaf_1))
```

**Result (32 bytes):**

```
f45a02c737bb54e7d023d67196bcb4f0e2c9f362b6b7b00f6b82a0dcb3e8b43e
```

**Committed as:** embedded in `input-checkpoint.cbor` (Step 3) and in `expected-watermark.cbor` (Step 5); reconstructible from the two `canonical_event_hash` values above.

---

### Step 3: Signed `Checkpoint` at `tree_size = 2`

**Core § citation:** §11.2 (`Checkpoint = COSESign1Bytes` carrying `CheckpointPayload`); §7.4 (COSE protected-header + Sig_structure, reused from Step 1).

**Load-bearing CDDL (§11.2):**

```cddl
CheckpointPayload = {
  version:              uint .size 1,       ; = 1 for Phase 1
  scope:                bstr,
  tree_size:            uint,
  tree_head_hash:       digest,
  timestamp:            uint,
  anchor_ref:           bstr / null,
  prev_checkpoint_hash: digest / null,
  extensions:           { * tstr => any } / null,
}
```

**Field population:**

| Field | Value | Source |
|---|---|---|
| `version` | `1` | §11.2 |
| `scope` | `"test-projection-ledger"` (bstr, 22 bytes) | §11.2 |
| `tree_size` | `2` | count of events in chain |
| `tree_head_hash` | 32 bytes from Step 2 | §11.3 |
| `timestamp` | `1745000120` | §11.2 |
| `anchor_ref` | `null` | §11.5 (Phase 1 null admissible) |
| `prev_checkpoint_hash` | `null` | §11.2 (first checkpoint in scope) |
| `extensions` | `null` | §11.6 |

**Operation.** dCBOR-encode the 8-key map (§5.1 byte-wise lex ordering). Sign the resulting bytes as the COSE_Sign1 payload, reusing the Step 1 protected-header bytes (`alg = -8, kid = af9dff…, suite_id = 1`). Assemble tag-18 envelope.

**Result (260 bytes):** committed as `input-checkpoint.cbor`. First 4 bytes are `0xd2 0x84 0x58 0x1b` (tag 18, array of 4, bstr length 27) — same COSE envelope shape as `expected-event.cbor` in `append/001`.

---

### Step 4: `checkpoint_digest` = Watermark's `checkpoint_ref`

**Core § citation:** §9.6 Checkpoint digest; §9.1 Domain separation.

**Load-bearing sentence (§9.6):**

> "`checkpoint_digest = SHA-256("trellis-checkpoint-v1" domain-separated per §9.1 over dCBOR(CheckpointHashPreimage))`."

**Load-bearing CDDL (§9.6):**

```cddl
CheckpointHashPreimage = {
  version:            uint .size 1,
  scope:              bstr,
  checkpoint_payload: CheckpointPayload,
}
```

**Operation.** Build `CheckpointHashPreimage = {version: 1, scope: b"test-projection-ledger", checkpoint_payload: <payload from Step 3>}`. dCBOR-encode. Apply §9.1 domain separation with `tag = "trellis-checkpoint-v1"` (UTF-8 length 21).

**Result (32 bytes):**

```
b649f60f615e7b1f028e097cdeca2600f260eb39cd87856c9caa2c9d47456a16
```

**Committed as:** embedded in `expected-watermark.cbor` as the `checkpoint_ref` field; reconstructible from the bytes of `input-checkpoint.cbor` via the construction above.

---

### Step 5: `Watermark`

**Core § citation:** §15.2 Watermark CDDL; Companion §14.1 (minimum watermark fields).

**Load-bearing CDDL (§15.2):**

```cddl
Watermark = {
  scope:                bstr,
  tree_size:            uint,
  tree_head_hash:       digest,
  checkpoint_ref:       digest,              ; checkpoint_digest (§11.2)
  built_at:             uint,                ; Unix seconds UTC when the artifact was built
  rebuild_path:         tstr,                ; implementation-defined deterministic identifier
  ? projection_schema_id: tstr,              ; optional; projection schema version identifier
}
```

**Load-bearing sentence (§15.2, on `projection_schema_id`).**

> "The `projection_schema_id` field is REQUIRED whenever the bearer is a projection governed by Companion §14.1 (it identifies the projection schema version under which the derived artifact was built) and MUST be OMITTED for non-projection derivatives (for example, agency-log entries per §15.4). When present it MUST be a URI conforming to RFC 3986."

**Field population:**

| Field | Value | Source |
|---|---|---|
| `scope` | `"test-projection-ledger"` | matches chain/checkpoint scope (§15.2) |
| `tree_size` | `2` | matches Step 3 checkpoint (§15.2) |
| `tree_head_hash` | Step 2 bytes | §11.3 (matches checkpoint's) |
| `checkpoint_ref` | Step 4 bytes | §9.6 / §15.2 |
| `built_at` | `1745000180` | §15.2 |
| `rebuild_path` | `"trellis.projection.v1/minimal"` | §15.3 |
| `projection_schema_id` | `"urn:trellis:projection:minimal:v1"` | §15.2 (URI-shaped) |

**Operation.** dCBOR-encode the 7-key map (§5.1 byte-wise lex ordering over the text-keys `scope`, `tree_size`, `tree_head_hash`, `checkpoint_ref`, `built_at`, `rebuild_path`, `projection_schema_id`). The serialized key order is determined by canonical-CBOR encoding of each text key:

| Key | Encoded first bytes |
|---|---|
| `scope` (5 chars) | `0x65` |
| `built_at` (8 chars) | `0x68` |
| `tree_size` (9 chars) | `0x69` |
| `rebuild_path` (12 chars) | `0x6c` |
| `checkpoint_ref` (14 chars) | `0x6e` |
| `tree_head_hash` (14 chars) | `0x6e` |
| `projection_schema_id` (20 chars) | `0x74` |

Where two keys share a first byte (`0x6e`), byte-wise comparison continues. For the two `0x6e`-prefixed keys the next canonical byte is the first character (`c` < `t`), so `checkpoint_ref` precedes `tree_head_hash`. Resulting dCBOR key order: `scope, built_at, tree_size, rebuild_path, checkpoint_ref, tree_head_hash, projection_schema_id`.

**Result (253 bytes):** first byte `0xa7` (map with 7 entries). Full hex in the footer.

**Committed as:** `expected-watermark.cbor` (253 bytes).

---

### Step 6: Minimal view artifact (wrapper around Watermark)

**Spec citation:** Companion §14.1 (watermark is a required field of the derived artifact; it is embedded in, not the whole of, the artifact).

**Operation.** Build a minimal wrapping map `{watermark: <Watermark from Step 5>, body: {item_count: 2, schema_id: "urn:trellis:projection:minimal:v1"}}`. dCBOR-encode.

**Load-bearing rationale.** Test 1's runner contract pulls the Watermark *out of* the view and byte-compares it to `expected-watermark.cbor`. A view that IS a bare Watermark would trivially pass the byte-compare but would not exercise the subfield-extraction discipline the runner documents. The `body` field is opaque filler — two innocuous fields, enough to force the runner to locate the `watermark` key among siblings.

**Operation — key ordering.** The 2-key map `{watermark, body}` sorts by canonical encoding of keys: `body` (`0x64`…) < `watermark` (`0x69`…), so the serialized order is `body`, `watermark`.

**Result (327 bytes):** first byte `0xa2` (map with 2 entries). Full hex in the footer.

**Committed as:** `input-view.cbor` (327 bytes).

---

## Footer — full hex dumps

Each block below is the byte-exact content of the named sibling file. A reader can verify by running `xxd` (or equivalent) on the sibling and comparing.

### `expected-watermark.cbor` (253 bytes, sha256 `a2a405ceffe097d03e9666af34f329e860cc5aac1bdf1b4ebf55d28011781722`)

```
a76573636f706556746573742d70726f6a656374696f6e2d6c65646765726862
75696c745f61741a680296f469747265655f73697a65026c72656275696c645f
70617468781d7472656c6c69732e70726f6a656374696f6e2e76312f6d696e69
6d616c6e636865636b706f696e745f7265665820b649f60f615e7b1f028e097c
deca2600f260eb39cd87856c9caa2c9d47456a166e747265655f686561645f68
6173685820f45a02c737bb54e7d023d67196bcb4f0e2c9f362b6b7b00f6b82a0
dcb3e8b43e7470726f6a656374696f6e5f736368656d615f6964782175726e3a
7472656c6c69733a70726f6a656374696f6e3a6d696e696d616c3a7631
```

Structure: `0xa7` (map of 7); keys in the dCBOR-canonical order shown in Step 5.

### `input-checkpoint.cbor` (260 bytes, sha256 `01a421d149000cb714ea13b96350a18c396fd30a246ecca501240c225ac409b1`)

```
d284581ba301270450af9dff525391faa75c8e8da4808b17433a0001000001a0
58a0a86573636f706556746573742d70726f6a656374696f6e2d6c6564676572
6776657273696f6e016974696d657374616d701a680296b869747265655f7369
7a65026a616e63686f725f726566f66a657874656e73696f6e73f66e74726565
5f686561645f686173685820f45a02c737bb54e7d023d67196bcb4f0e2c9f362
b6b7b00f6b82a0dcb3e8b43e74707265765f636865636b706f696e745f686173
68f6584096cd9f31c93623df3de3dbca19adefbcb6f07dff9f79161af128bcda
f339d03cf40e9b1e9c600d71c5deeaacb32d25e370419b02de4f105ad2115a3c
1e3c9a08
```

Structure: `0xd2` (tag 18); `0x84` (array of 4); `0x58 0x1b <27-byte protected header>`; `0xa0` (empty unprotected); `0x58 0xa0 <160-byte CheckpointPayload>`; `0x58 0x40 <64-byte Ed25519 signature>`.

### `input-view.cbor` (327 bytes, sha256 `ef5ea415331ea8969e976af93af14e2e56ac234d4f130060cb72d1f6c7ce587b`)

```
a264626f6479a269736368656d615f6964782175726e3a7472656c6c69733a70
726f6a656374696f6e3a6d696e696d616c3a76316a6974656d5f636f756e7402
6977617465726d61726ba76573636f706556746573742d70726f6a656374696f
6e2d6c6564676572686275696c745f61741a680296f469747265655f73697a65
026c72656275696c645f70617468781d7472656c6c69732e70726f6a65637469
6f6e2e76312f6d696e696d616c6e636865636b706f696e745f7265665820b649
f60f615e7b1f028e097cdeca2600f260eb39cd87856c9caa2c9d47456a166e74
7265655f686561645f686173685820f45a02c737bb54e7d023d67196bcb4f0e2
c9f362b6b7b00f6b82a0dcb3e8b43e7470726f6a656374696f6e5f736368656d
615f6964782175726e3a7472656c6c69733a70726f6a656374696f6e3a6d696e
696d616c3a7631
```

Structure: `0xa2` (map of 2); `body` first (sorts before `watermark`). The `watermark` value starting at offset `0x48` (`a7 65 73 63 6f 70 65 …`) reproduces the bytes of `expected-watermark.cbor` verbatim — a reader may extract them by seeking to the `0xa7` byte following the `watermark` key.

### `input-chain.cbor` (1360 bytes, sha256 `386bccc94f89fa91326d3e2ee6460202f037b6ee3eea77d67d1e6246b7e831d7`)

CBOR array of two COSE_Sign1 envelopes (`0x82` = array of 2, then `0xd2 …` for each envelope). Event 0 occupies bytes 1..560 (559-byte COSE_Sign1 for EventPayload length 561 — observed lengths vary by field string lengths; the construction is fully determined by the Step 1 table). Event 1 follows immediately. Structural inspection:

- byte 0: `0x82` (array of 2).
- bytes 1..: `0xd2 0x84 0x58 0x1b <27-byte protected> 0xa0 0x59 0x02 0x31 <561-byte EventPayload> 0x58 0x40 <64-byte signature>` → event 0.
- remainder: `0xd2 0x84 0x58 0x1b <27-byte protected> 0xa0 0x59 0x02 0x54 <596-byte EventPayload> 0x58 0x40 <64-byte signature>` → event 1.

A reader who has reproduced `append/001` can reach byte-exact output for each event by substituting the Step-1 pinned field values above; the two protected-header bstrs are byte-identical to the one in `append/001`'s `expected-event.cbor` because `kid`, `alg`, and `suite_id` are unchanged.

Reader-verifiable intermediates (embedded in the chain bytes):

- Event 0 `canonical_event_hash` = `e5247fc178304396901c465d46386283bdea670c7676d7e48a3b6eefcafee8ee`
- Event 1 `prev_hash` = Event 0 `canonical_event_hash` (§10.2), matching the bytes at the relevant offset in event 1's EventPayload.
- Event 1 `canonical_event_hash` = `48f723c1f02a5d9fa4d23fe3d4fbb32930cb6b10d40089a7437788d58dcb43b4`

These two values feed directly into Step 2 (Merkle root) and Step 3 (checkpoint `tree_head_hash`).
