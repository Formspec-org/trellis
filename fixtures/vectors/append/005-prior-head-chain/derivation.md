# Derivation — `append/005-prior-head-chain`

## Header

**What this vector exercises.** This is the first non-genesis append — `sequence = 1` — extending the ledger scope `test-response-ledger` that `append/001-minimal-inline-payload` initialized. It reproduces, byte-for-byte, the three Core constructions that genesis cannot reach: (a) the `prev_hash` linkage of §10.2 under `sequence > 0`; (b) the Phase-1 envelope preimage under a non-null `prev_hash`, which is the first byte shape under which the §10 strict-superset claim (invariant #10, TR-CORE-080) has non-trivial content; and (c) the idempotency identity rule of §17.3 — 005's `(ledger_scope, idempotency_key)` pair is distinct from 001's under the *same* ledger scope, so §17.3's identity rule admits 005 as a new canonical position rather than resolving it to 001's. Every other construction (authored / canonical / signed event surfaces per §6.8; `author_event_hash` per §9.5; COSE_Sign1 per §7.4; `canonical_event_hash` per §9.2; AppendHead per §10.6) is structurally identical to 001 and is recomputed here only to demonstrate that the `prev_hash`-propagated bytes *do* flow through the subsequent digests without extra handling.

**Scope of this vector.** Structural-only for the payload layer, like 001. The `PayloadInline.ciphertext` bstr carries the pinned 64-byte plaintext from `../../_inputs/sample-payload-001.bin` opaquely; `KeyBag.entries` is the empty list. No HPKE wrap. The issuer signing key is unchanged — 005 is chain-linkage, not key-rotation (rotation is vector 002). Consequently every byte of 005 that is not downstream of `prev_hash`, `sequence`, `authored_at`, or `idempotency_key` is either literally identical to 001 (the `kid`, `content_hash`, protected-header bytes, the issuer pubkey) or derived identically from identical inputs (the `PayloadInline` submap).

**Pinned inputs.**

| Input | Value | Source |
|---|---|---|
| `signing_key` | Ed25519 COSE_Key (unchanged vs 001) | `../../_keys/issuer-001.cose_key` |
| `payload` | 64 bytes, ASCII `"Trellis fixture payload #001"` + `0x00` padding | `../../_inputs/sample-payload-001.bin` |
| `ledger_scope` | bstr `"test-response-ledger"` (equal to 001's — §10.2 linkage is per-scope) | §10.4; §10.6 `AppendHead.scope` |
| `sequence` | `1` | §10.2: `sequence > 0` demands non-null `prev_hash` |
| `prev_hash` | 32-byte digest `ef2622f1…e724ddb` — read from `input-prior-append-head.cbor` | §10.2; §10.6 AppendHead |
| `timestamp` (`authored_at`) | `1745000001` (+1s vs 001) | §12.1; narrative-only — §10.2 pins by `prev_hash`, not wall-clock |
| `event_type` | bstr `"x-trellis-test/append-minimal"` (inherited from 001) | §14.6 |
| `classification` | bstr `"x-trellis-test/unclassified"` (inherited from 001) | §14.6 |
| `retention_tier` | `0` | §12.1 |
| `idempotency_key` | bstr `"idemp-append-005"` (16 bytes) | §6.1; §17.2 — **distinct from 001's** under the same ledger scope |
| `suite_id` | `1` (unchanged vs 001) | §7.1 |
| `PayloadInline.nonce` | 12 bytes of `0x00` (unchanged vs 001) | §6.4 |

**Core § roadmap (in traversal order).**

1. §10.6 — read `AppendHead` of the prior event from `input-prior-append-head.cbor`; extract `canonical_event_hash`.
2. §10.2 — propagate that digest as 005's `prev_hash`.
3. §17.3 — confirm 005's `(ledger_scope, idempotency_key)` is distinct from 001's; pin a new `idempotency_key`.
4. §9.3 + §9.1 — `content_hash`; recomputed for completeness; identical to 001's.
5. §6.8 (authored) + §9.5 — build `AuthorEventHashPreimage` for `sequence = 1`; serialize as dCBOR; commit as `input-author-event-hash-preimage.cbor`.
6. §9.5 + §9.1 — `author_event_hash` over the authored preimage.
7. §6.8 (canonical) + §6.1 — build `EventPayload` for `sequence = 1`; serialize; commit as `expected-event-payload.cbor`.
8. §7.4 + RFC 9052 §4.4 — protected header (unchanged vs 001) and `Sig_structure` (now carrying 005's canonical payload).
9. §7.1 — Ed25519 signature; deterministic under RFC 8032 for fixed seed + fixed message.
10. §6.1 + §6.8 (signed) + §7.4 — COSE_Sign1 tag-18 envelope; commit as `expected-event.cbor`.
11. §9.2 + §9.1 — `canonical_event_hash`.
12. §10.6 — `AppendHead` for the post-append state at `sequence = 1`.

Steps 4, 8 (protected-header portion), and the envelope assembly procedure of step 10 are identical to the corresponding steps of `append/001-minimal-inline-payload`. Rather than re-quote their load-bearing sentences, this derivation cites 001's derivation.md by reference for those sub-operations and shows the bytes; the load-bearing normative prose is quoted only for the steps where 005 introduces a new construction over 001 (steps 1, 2, 3, 5, 7, and the post-signing deltas in 9–12 that ride the new `EventPayload`).

---

## Body

### Step 1: Read the prior `AppendHead`

**Core § citation:** §10.6 Append head artifact.

**Load-bearing sentence:**

> "Every successful `append` operation returns a structured **append head** describing the post-append state of the targeted ledger scope. This artifact is the structural companion to the `prev_hash` invariant (§10.2): the next event's `prev_hash` is equal to the previous append's `AppendHead.canonical_event_hash` field, by construction."

**Inputs:** `input-prior-append-head.cbor` — a byte-identical copy of `append/001-minimal-inline-payload/expected-append-head.cbor` (93 bytes; sha256 `dc0fc83406bc87364b8beeebb4b8c867e68e9e5a025e24817c542f91da3772db`). It is committed alongside 005 so the stranger test does not require cross-vector lookups: everything 005 needs to verify its own `prev_hash` linkage is present in 005's directory.

**Operation:** dCBOR-decode. The 3-key map `{scope, sequence, canonical_event_hash}` parses to:

| Field | Value |
|---|---|
| `scope` | `b"test-response-ledger"` (20 bytes) |
| `sequence` | `0` |
| `canonical_event_hash` | `ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb` (32 bytes) |

**Committed as:** `input-prior-append-head.cbor` (copy of the prior vector's head; bytes verbatim).

---

### Step 2: Propagate `prev_hash`

**Core § citation:** §10.2 `prev_hash` requirements.

**Load-bearing sentences:**

> "For `sequence == 0`: `prev_hash` MUST be `null`."

> "For `sequence == N > 0`: `prev_hash` MUST equal the `canonical_event_hash` (§9.2) of the event with `sequence == N-1` in the same ledger."

> "A Canonical Append Service MUST reject any submission whose `prev_hash` does not satisfy this constraint."

> "A Verifier MUST verify the chain by recomputing each event's `canonical_event_hash` and checking that it appears as `prev_hash` in the next event."

**Reading of §10.2 used by this vector.** §10.2 places the `prev_hash`-production obligation on the Fact Producer (the event's author builds the field into `AuthorEventHashPreimage` and `EventPayload` before signing) and the `prev_hash`-check obligation on the Canonical Append Service (at admission) and on any Verifier (at replay). §10.6 closes the loop: the Fact Producer reads the hash from the prior call's `AppendHead.canonical_event_hash` return value. This is unambiguous for the genesis→non-genesis transition in a single-author single-scope setting, which is exactly what 005 exercises. No Core gap is claimed; see the "Core-gap notes" note at the bottom of this file for the one consideration a more-than-one-author deployment would need to revisit.

**Inputs:**

- `sequence = 1` (so §10.2 requires a non-null digest).
- Prior `canonical_event_hash` from Step 1: `ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb`.
- Prior `scope` from Step 1, which equals this event's `ledger_scope` (§10.2 requires the linkage be in the same ledger).

**Operation:** set `prev_hash = ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb`. This bstr is embedded in the authored preimage (Step 5) and the canonical payload (Step 7); its bytes are the single load-bearing delta that distinguishes 005's preimages from 001's.

**Committed as:** embedded in `input-author-event-hash-preimage.cbor` and `expected-event-payload.cbor`; the bstr is identifiable in the footer hex dumps as the 32 bytes following the `69 70 72 65 76 5f 68 61 73 68 58 20` (key `"prev_hash"` + bstr length 32).

---

### Step 3: Pick `idempotency_key` distinct from 001's

**Core § citation:** §17.2 `idempotency_key`; §17.3 Append idempotency contract.

**Load-bearing sentence (§17.3):**

> "For a given `idempotency_key` within a declared ledger scope, a Canonical Append Service MUST resolve every successful retry to exactly one of: [...] 3. **Reject on conflict.** A retry that shares `idempotency_key` but whose payload would produce a different `content_hash`, `author_event_hash`, or `canonical_event_hash` MUST be rejected with the structured error `IdempotencyKeyPayloadMismatch` (§17.5)."

**Why this step is load-bearing for 005.** 005 shares `ledger_scope` with 001 and has a different payload (only trivially different — `prev_hash`, `sequence`, `authored_at`, and `idempotency_key` differ — but §17.3 keys on `content_hash` / `author_event_hash` / `canonical_event_hash`, and 005's `author_event_hash` and `canonical_event_hash` are provably distinct from 001's because the preimage maps differ at the `prev_hash` field). If 005 were authored under 001's `idempotency_key = b"idemp-append-001"`, §17.3 would require the service to reject 005 with `IdempotencyKeyPayloadMismatch`. 005 is therefore obligated to carry a distinct key.

**Pinned choice:** `idempotency_key = b"idemp-append-005"` (16 bytes, §6.1 `.size (1..64)`). No construction — the key is opaque to Core §17.2. The only constraint the fixture honours is the distinctness obligation of §17.3 under the shared scope.

**Committed as:** embedded in `input-author-event-hash-preimage.cbor` and `expected-event-payload.cbor` at the `idempotency_key` key.

---

### Step 4: `content_hash` over `PayloadInline.ciphertext`

**Core § citation:** §9.3 Content hash; §9.1 Domain separation discipline.

**Inherits from 001** — identical payload bytes, identical tag, therefore identical output.

**Inputs:**

- `tag = "trellis-content-v1"` (UTF-8 length 18).
- `component = PayloadInline.ciphertext` = 64 bytes from `../../_inputs/sample-payload-001.bin`.

**Operation:** `content_hash = SHA-256(u32be(18) || "trellis-content-v1" || u32be(64) || <64 ciphertext bytes>)`. Procedure is Step 3 of `append/001-minimal-inline-payload/derivation.md`; not re-expanded here.

**Result (32 bytes, identical to 001's):**

```
bcdced2dfaf5342cd2baca0560a9d384473fd45e202555f0654cccaa32b4f812
```

**Committed as:** embedded in the preimages committed below.

---

### Step 5: Build `AuthorEventHashPreimage` (authored form per §6.8)

**Core § citation:** §6.8 Authored form; §9.5 `author_event_hash` construction; Appendix A (§28) CDDL.

**Load-bearing sentences:**

(From 001's derivation.md Step 4 — the authored-form CDDL and §6.8 load-bearing sentence apply unchanged. 005's delta is field-level, not structural.)

**Field population (delta vs 001 highlighted).**

| Field | 001 value | 005 value | Core citation |
|---|---|---|---|
| `version` | `1` | `1` | §6.1 |
| `ledger_scope` | `"test-response-ledger"` | `"test-response-ledger"` | §6.1, §10.4 |
| `sequence` | `0` | **`1`** | §10.2 |
| `prev_hash` | `null` | **32-byte digest from Step 2** | §10.2 |
| `causal_deps` | `null` | `null` | §10.3 |
| `content_hash` | (Step 4 output) | same bytes | §9.3 |
| `header.authored_at` | `1745000000` | **`1745000001`** | §12.1 — narrative-only |
| `header.*` (all other keys) | see 001 | same values | §12.1 |
| `commitments` | `null` | `null` | §13.3 |
| `payload_ref` | `PayloadInline{…}` | same submap | §6.4 |
| `key_bag` | `{entries:[]}` | same submap | §9.4 |
| `idempotency_key` | `"idemp-append-001"` | **`"idemp-append-005"`** | §6.1; §17.2 |
| `extensions` | `null` | `null` | §6.5 |

Three load-bearing deltas (`sequence`, `prev_hash`, `idempotency_key`) and one narrative delta (`authored_at`). Nothing else changes.

**Operation:** dCBOR-encode the map (§5.1 byte-wise lex key ordering; identical procedure to 001 Step 4). The 12 top-level keys sort identically to 001 because the key set is identical.

**Result (571 bytes):** 33 bytes longer than 001's 538-byte preimage. The size delta is exactly the size of swapping `prev_hash: null` (1 byte: `0xf6`) for `prev_hash: h'<32-byte digest>'` (34 bytes: `0x58 0x20` bstr prefix + 32 digest bytes), i.e. +33 bytes. The `authored_at` uint upgrade from `1745000000` to `1745000001` does not change encoded length. `idempotency_key` bstr length is unchanged at 16 bytes. The map-header byte stays `0xac` (still 12 entries).

**Committed as:** `input-author-event-hash-preimage.cbor` (571 bytes). Full hex in the footer.

---

### Step 6: `author_event_hash`

**Core § citation:** §9.5 `author_event_hash` construction; §9.1.

**Inherits the procedure from 001 Step 5**; only the component bytes change.

**Inputs:**

- `tag = "trellis-author-event-v1"` (UTF-8 length 23).
- `component = dCBOR(AuthorEventHashPreimage)` = 571 bytes of Step 5 output.

**Operation:** `preimage = u32be(23) || "trellis-author-event-v1" || u32be(571) || <571 bytes>`; `author_event_hash = SHA-256(preimage)`.

**Result (preimage, 602 bytes):** committed as `author-event-preimage.bin`. Bytes 0..3 are `00 00 00 17`; bytes 4..26 are `"trellis-author-event-v1"`; bytes 27..30 are `00 00 02 3b` (length-prefix, 571); bytes 31..601 reproduce `input-author-event-hash-preimage.cbor` verbatim.

**Result (digest, 32 bytes):**

```
f25c40a7fe073206b7d2e6c6136a435e892640fc6ffc0ccdbdeccd42d6b7f266
```

**Committed as:** preimage bytes as `author-event-preimage.bin`; digest as `author-event-hash.bin`.

---

### Step 7: Build `EventPayload` (canonical form per §6.8)

**Core § citation:** §6.8 Canonical form; §6.1.

**Inherits the CDDL and procedure from 001 Step 6.** Field population is identical to Step 5 above **plus** `author_event_hash` set to the 32-byte digest from Step 6.

**Operation:** dCBOR-encode (§5.1).

**Result (623 bytes):** 33 bytes longer than 001's 590-byte canonical payload, for the same reason as Step 5 (prev_hash switch from `null` to a 32-byte digest). Map header is `0xad` (13 entries, same as 001). Final 35 bytes carry `"author_event_hash"` + the Step 6 digest.

**Committed as:** `expected-event-payload.cbor` (623 bytes). These are the bytes that become the COSE_Sign1 `payload` bstr.

---

### Step 8: COSE protected header + `Sig_structure`

**Core § citation:** §7.4; RFC 9052 §4.4; §6.6.

**Protected header.** The three-key map `{alg: -8, kid: …, suite_id: 1}` is **byte-identical** to 001's because every input is identical: same issuer pubkey → same §8.3 `kid` derivation → same 16-byte `kid`; suite and alg are Phase-1 pins shared with 001. Concretely:

```
inner bytes (27): a301270450af9dff525391faa75c8e8da4808b17433a0001000001
wrapped bstr (29): 581b a301270450af9dff525391faa75c8e8da4808b17433a0001000001
```

Identical to 001's Step 7.

**`Sig_structure` (RFC 9052 §4.4):** the 4-element array `["Signature1", <27-byte inner protected bstr>, h'', <623-byte EventPayload bstr>]` serialized as dCBOR. Procedure identical to 001 Step 8; only the embedded payload differs.

**Result (668 bytes):** array prefix `0x84`; then `0x6a "Signature1"`; then `0x58 0x1b <27 bytes>`; then `0x40` (empty external_aad); then `0x59 0x02 0x6f <623-byte EventPayload>`. The length prefix differs from 001's `0x59 0x02 0x4e` because 623 vs 590 (diff = 33, matching the Step 5/7 size delta).

**Committed as:** `sig-structure.bin` (668 bytes).

---

### Step 9: Ed25519 signature

**Core § citation:** §7.1.

Procedure from 001 Step 9. Signing seed unchanged; message is the 668-byte `Sig_structure` from Step 8.

**Result (64 bytes):**

```
2b014298d95d43b7d3f03dab2af664733c7c8c7f6905874eed8890ff3fbfa37c
cb7d50e126cfa901948668e74341cb73629dc4a15b0fd3a413787f11c7c7f90f
```

Deterministic under RFC 8032 for fixed seed + fixed message. Because 005's `Sig_structure` differs from 001's at the embedded `EventPayload` bytes (differing in `prev_hash`, `sequence`, `authored_at`, and `idempotency_key`), 005's signature necessarily differs from 001's.

**Committed as:** embedded in `expected-event.cbor`.

---

### Step 10: Assemble the COSE_Sign1 tag-18 envelope (signed form per §6.8)

**Core § citation:** §6.8 Signed form; §6.1; §7.4; RFC 9052 §4.2.

Procedure from 001 Step 10. Envelope fields:

- `protected_bstr` = 29-byte wrapped protected header from Step 8 (identical bytes to 001).
- `unprotected_map` = `{}` (empty).
- `payload_bstr` = 623-byte `EventPayload` from Step 7.
- `signature_bstr` = 64-byte signature from Step 9.

Tag 18 (`0xd2`) applied.

**Result (724 bytes):** first 4 bytes are `0xd2 0x84 0x58 0x1b` (tag 18, array of 4, bstr length 27). Full dump in the footer.

**Committed as:** `expected-event.cbor` (724 bytes).

---

### Step 11: `canonical_event_hash`

**Core § citation:** §9.2; §9.1.

Procedure from 001 Step 11. `CanonicalEventHashPreimage = {version: 1, ledger_scope: "test-response-ledger", event_payload: <Step 7 EventPayload>}`; dCBOR-encode; domain-separate under `"trellis-event-v1"` per §9.1; SHA-256.

**Result (32 bytes):**

```
3d3d5aeb5d4b8d972adbddfe0f339a94fffe01bf90ac1648be2eb98d4acc9f17
```

Differs from 001's `ef2622f1…e724ddb` because the embedded `EventPayload` bytes differ. This is the hash that, by §10.2, a hypothetical `sequence = 2` successor would carry in its `prev_hash` field.

**Committed as:** embedded in `expected-append-head.cbor` (Step 12).

---

### Step 12: Build `AppendHead`

**Core § citation:** §10.6.

Procedure from 001 Step 12. Fields:

| Field | Value |
|---|---|
| `scope` | `"test-response-ledger"` (equal to 001's) |
| `sequence` | `1` — **the load-bearing delta vs 001's AppendHead** |
| `canonical_event_hash` | 32 bytes from Step 11 |

**Operation:** dCBOR-encode (§5.1).

**Result (93 bytes):** same length as 001's AppendHead (`sequence = 0` and `sequence = 1` encode in the same 1 byte).

**Committed as:** `expected-append-head.cbor` (93 bytes).

---

## Footer — full hex dumps

Each block below is the byte-exact content of the named sibling file.

### `input-prior-append-head.cbor` (93 bytes, sha256 `dc0fc83406bc87364b8beeebb4b8c867e68e9e5a025e24817c542f91da3772db`)

Copy of `append/001-minimal-inline-payload/expected-append-head.cbor`, verbatim.

```
a36573636f706554746573742d726573706f6e73652d6c656467657268736571
75656e6365007463616e6f6e6963616c5f6576656e745f686173685820ef2622
f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb
```

### `input-author-event-hash-preimage.cbor` (571 bytes, sha256 `d5701c0e3aa6fcb79ac4e4627e86f0774b907488b10ddbdb01845b1786d49442`)

```
ac66686561646572a96a6576656e745f74797065581d782d7472656c6c69732d
746573742f617070656e642d6d696e696d616c6a657874656e73696f6e73f66b
617574686f7265645f61741a680296416b7769746e6573735f726566f66e636c
617373696669636174696f6e581b782d7472656c6c69732d746573742f756e63
6c61737369666965646e726574656e74696f6e5f74696572006e7461675f636f
6d6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74f6767375
626a6563745f7265665f636f6d6d69746d656e74f6676b65795f626167a16765
6e7472696573806776657273696f6e016873657175656e63650169707265765f
686173685820ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77a
ae068e724ddb6a657874656e73696f6e73f66b63617573616c5f64657073f66b
636f6d6d69746d656e7473f66b7061796c6f61645f726566a3656e6f6e63654c
000000000000000000000000687265665f7479706566696e6c696e656a636970
6865727465787458405472656c6c69732066697874757265207061796c6f6164
20233030310000000000000000000000000000000000000000000000000000
0000000000000000000000006c636f6e74656e745f686173685820bcdced2dfa
f5342cd2baca0560a9d384473fd45e202555f0654cccaa32b4f8126c6c656467
65725f73636f706554746573742d726573706f6e73652d6c65646765726f6964
656d706f74656e63795f6b6579506964656d702d617070656e642d303035
```

Byte-level diff vs `append/001-minimal-inline-payload/input-author-event-hash-preimage.cbor`:

- `authored_at` uint value changes from `1a68029640` to `1a68029641` (last byte `0x40` → `0x41` = `1745000000` → `1745000001`).
- `prev_hash` bstr changes from `f6` (CBOR `null`, 1 byte) to `5820 <32 bytes>` (CBOR bstr-32, 34 bytes) — the 33-byte size delta.
- `sequence` uint changes from `00` to `01`.
- `idempotency_key` bstr trailing byte changes from `0x31` (ASCII `'1'`) to `0x35` (ASCII `'5'`) — the `"idemp-append-001"` → `"idemp-append-005"` swap.

No other bytes differ.

### `author-event-preimage.bin` (602 bytes, sha256 `f25c40a7fe073206b7d2e6c6136a435e892640fc6ffc0ccdbdeccd42d6b7f266`)

```
000000177472656c6c69732d617574686f722d6576656e742d76310000023bac
66686561646572a96a6576656e745f74797065581d782d7472656c6c69732d74
6573742f617070656e642d6d696e696d616c6a657874656e73696f6e73f66b61
7574686f7265645f61741a680296416b7769746e6573735f726566f66e636c61
7373696669636174696f6e581b782d7472656c6c69732d746573742f756e636c
61737369666965646e726574656e74696f6e5f74696572006e7461675f636f6d
6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74f676737562
6a6563745f7265665f636f6d6d69746d656e74f6676b65795f626167a167656e
7472696573806776657273696f6e016873657175656e63650169707265765f68
6173685820ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae
068e724ddb6a657874656e73696f6e73f66b63617573616c5f64657073f66b63
6f6d6d69746d656e7473f66b7061796c6f61645f726566a3656e6f6e63654c00
0000000000000000000000687265665f7479706566696e6c696e656a63697068
65727465787458405472656c6c69732066697874757265207061796c6f616420
2330303100000000000000000000000000000000000000000000000000000000
00000000000000006c636f6e74656e745f686173685820bcdced2dfaf5342cd2
baca0560a9d384473fd45e202555f0654cccaa32b4f8126c6c65646765725f73
636f706554746573742d726573706f6e73652d6c65646765726f6964656d706f
74656e63795f6b6579506964656d702d617070656e642d303035
```

Bytes 0..3 are `00 00 00 17` (length-prefix, 23); bytes 4..26 are `"trellis-author-event-v1"`; bytes 27..30 are `00 00 02 3b` (length-prefix, 571); bytes 31..601 reproduce `input-author-event-hash-preimage.cbor` verbatim.

### `author-event-hash.bin` (32 bytes, sha256 `3593dbaea18e63f04bc218f6c1385fb0a9c64f0fa7d8dbe1e558c12ae4407669`)

```
f25c40a7fe073206b7d2e6c6136a435e892640fc6ffc0ccdbdeccd42d6b7f266
```

This is the SHA-256 of `author-event-preimage.bin`; the file *is* the `author_event_hash` value. The file's own SHA-256 (shown above) is a second-order integrity check only, following the same convention as 001's `author-event-hash.bin`.

### `expected-event-payload.cbor` (623 bytes, sha256 `28d90fee4061c7bb75984e7a180b6b3debac5a201cf72840fec00744d63850b9`)

```
ad66686561646572a96a6576656e745f74797065581d782d7472656c6c69732d
746573742f617070656e642d6d696e696d616c6a657874656e73696f6e73f66b
617574686f7265645f61741a680296416b7769746e6573735f726566f66e636c
617373696669636174696f6e581b782d7472656c6c69732d746573742f756e63
6c61737369666965646e726574656e74696f6e5f74696572006e7461675f636f
6d6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74f6767375
626a6563745f7265665f636f6d6d69746d656e74f6676b65795f626167a16765
6e7472696573806776657273696f6e016873657175656e63650169707265765f
686173685820ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77a
ae068e724ddb6a657874656e73696f6e73f66b63617573616c5f64657073f66b
636f6d6d69746d656e7473f66b7061796c6f61645f726566a3656e6f6e63654c
000000000000000000000000687265665f7479706566696e6c696e656a636970
6865727465787458405472656c6c69732066697874757265207061796c6f6164
20233030310000000000000000000000000000000000000000000000000000
0000000000000000000000006c636f6e74656e745f686173685820bcdced2dfa
f5342cd2baca0560a9d384473fd45e202555f0654cccaa32b4f8126c6c656467
65725f73636f706554746573742d726573706f6e73652d6c65646765726f6964
656d706f74656e63795f6b6579506964656d702d617070656e642d3030357161
7574686f725f6576656e745f686173685820f25c40a7fe073206b7d2e6c6136a
435e892640fc6ffc0ccdbdeccd42d6b7f266
```

Map header is `0xad` (13 entries — 12 from the authored preimage plus `author_event_hash`). Final 35 bytes, `71 617574686f725f6576656e745f68617368 5820 <32-byte digest>`, carry the text key `"author_event_hash"` and its digest bstr value.

### `sig-structure.bin` (668 bytes, sha256 `d73aa881b46ae39553fe92f7d3d442a4c103d446726beb87fb33ae6dd02b4b58`)

```
846a5369676e617475726531581ba301270450af9dff525391faa75c8e8da480
8b17433a00010000014059026fad66686561646572a96a6576656e745f747970
65581d782d7472656c6c69732d746573742f617070656e642d6d696e696d616c
6a657874656e73696f6e73f66b617574686f7265645f61741a680296416b7769
746e6573735f726566f66e636c617373696669636174696f6e581b782d747265
6c6c69732d746573742f756e636c61737369666965646e726574656e74696f6e
5f74696572006e7461675f636f6d6d69746d656e74f6726f7574636f6d655f63
6f6d6d69746d656e74f6767375626a6563745f7265665f636f6d6d69746d656e
74f6676b65795f626167a167656e7472696573806776657273696f6e01687365
7175656e63650169707265765f686173685820ef2622f1470ba3d9c24b47c056
6cab8902b6500fbb3d47bdd77aae068e724ddb6a657874656e73696f6e73f66b
63617573616c5f64657073f66b636f6d6d69746d656e7473f66b7061796c6f61
645f726566a3656e6f6e63654c000000000000000000000000687265665f7479
706566696e6c696e656a6369706865727465787458405472656c6c6973206669
7874757265207061796c6f616420233030310000000000000000000000000000
000000000000000000000000000000000000000000000000006c636f6e74656e
745f686173685820bcdced2dfaf5342cd2baca0560a9d384473fd45e202555f0
654cccaa32b4f8126c6c65646765725f73636f706554746573742d726573706f
6e73652d6c65646765726f6964656d706f74656e63795f6b6579506964656d70
2d617070656e642d30303571617574686f725f6576656e745f686173685820f2
5c40a7fe073206b7d2e6c6136a435e892640fc6ffc0ccdbdeccd42d6b7f266
```

Structure (per RFC 9052 §4.4): `0x84` (array of 4); `0x6a "Signature1"`; `0x58 0x1b <27-byte protected header>`; `0x40` (zero-length external_aad); `0x59 0x02 0x6f <623-byte EventPayload>`. The `0x59 0x02 0x6f` length prefix differs from 001's `0x59 0x02 0x4e` by the 33-byte preimage size delta derived in Step 5.

### `expected-event.cbor` (724 bytes, sha256 `416d5e6190d0ec8ad791437f7e4bdb369f751b11dcb3597a5f2911421529aac9`)

```
d284581ba301270450af9dff525391faa75c8e8da4808b17433a0001000001a0
59026fad66686561646572a96a6576656e745f74797065581d782d7472656c6c
69732d746573742f617070656e642d6d696e696d616c6a657874656e73696f6e
73f66b617574686f7265645f61741a680296416b7769746e6573735f726566f6
6e636c617373696669636174696f6e581b782d7472656c6c69732d746573742f
756e636c61737369666965646e726574656e74696f6e5f74696572006e746167
5f636f6d6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74f6
767375626a6563745f7265665f636f6d6d69746d656e74f6676b65795f626167
a167656e7472696573806776657273696f6e016873657175656e636501697072
65765f686173685820ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47
bdd77aae068e724ddb6a657874656e73696f6e73f66b63617573616c5f646570
73f66b636f6d6d69746d656e7473f66b7061796c6f61645f726566a3656e6f6e
63654c000000000000000000000000687265665f7479706566696e6c696e656a
6369706865727465787458405472656c6c69732066697874757265207061796c
6f61642023303031000000000000000000000000000000000000000000000000
0000000000000000000000000000006c636f6e74656e745f686173685820bcdc
ed2dfaf5342cd2baca0560a9d384473fd45e202555f0654cccaa32b4f8126c6c
65646765725f73636f706554746573742d726573706f6e73652d6c6564676572
6f6964656d706f74656e63795f6b6579506964656d702d617070656e642d3030
3571617574686f725f6576656e745f686173685820f25c40a7fe073206b7d2e6
c6136a435e892640fc6ffc0ccdbdeccd42d6b7f26658402b014298d95d43b7d3
f03dab2af664733c7c8c7f6905874eed8890ff3fbfa37ccb7d50e126cfa90194
8668e74341cb73629dc4a15b0fd3a413787f11c7c7f90f
```

Structure: `0xd2` (tag 18); `0x84` (array of 4); `0x58 0x1b <27-byte protected>`; `0xa0` (empty unprotected map); `0x59 0x02 0x6f <623-byte payload>`; `0x58 0x40 <64-byte signature>`.

### `expected-append-head.cbor` (93 bytes, sha256 `1210c7ba9469f94bc29961fbe3e01a7910a6f2dc6bcdb107041f153f26f8a912`)

```
a36573636f706554746573742d726573706f6e73652d6c656467657268736571
75656e6365017463616e6f6e6963616c5f6576656e745f686173685820 3d3d
5aeb5d4b8d972adbddfe0f339a94fffe01bf90ac1648be2eb98d4acc9f17
```

Structure: `0xa3` (map of 3); `0x65 "scope" 0x54 <20-byte ledger_scope>`; `0x68 "sequence" 0x01`; `0x74 "canonical_event_hash" 0x58 0x20 <32-byte digest>`. The `sequence` byte is `0x01` (vs 001's `0x00`); the digest is 005's canonical_event_hash from Step 11.

---

## Invariant → byte mapping

| Invariant | Where in 005's bytes |
|---|---|
| #5 (canonical order named, TR-CORE-020) | `expected-event-payload.cbor` carries `sequence = 1` and `prev_hash` = 001's `canonical_event_hash`, in a shared `ledger_scope`. Together with 001's `sequence = 0` they establish exactly one canonical order across two positions. |
| #5 (order independent of wall-clock, TR-CORE-023) | `authored_at` changes (`1745000001` vs 001's `1745000000`) but no byte of the `prev_hash` / sequence / canonical_event_hash chain depends on it — §10.2 pins by `canonical_event_hash`, not timestamp. |
| #10 (Phase-1 envelope = Phase-3 case-ledger event, TR-CORE-080) | `expected-event-payload.cbor` is the first EventPayload in this corpus with a non-null `prev_hash` — the first byte shape under which the §10 "strict superset" claim has non-trivial content. Phase 3 is obligated by §10 to accept this byte shape unchanged. |
| #13 (idempotency_key stable in wire contract, TR-CORE-050) | `idempotency_key = b"idemp-append-005"` is present in `input-author-event-hash-preimage.cbor` and `expected-event-payload.cbor`; it is distinct from 001's `idemp-append-001` under the same `ledger_scope`, satisfying §17.3's identity rule. |

## Core-gap notes

§10.2 is unambiguous for the single-author single-scope case that this vector exercises: the Fact Producer produces `prev_hash`; the Canonical Append Service rejects bad values; the Verifier re-checks. No gap is claimed against 005's scope. A broader-deployment clarification — for example, whether a multi-author ledger scope requires a lock-acquire step before the producer reads `AppendHead.canonical_event_hash` for use as `prev_hash` — is outside what 005 exercises and is not claimed to surface a Core gap from this vector. The fixture-system design's prior-head propagation contract (`AppendHead` in, `prev_hash` out) is what 005 demonstrates; it is the shape §10.6 explicitly names as "the contract between `append` and its caller."
