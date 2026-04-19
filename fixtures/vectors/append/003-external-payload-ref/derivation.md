# Derivation — `append/003-external-payload-ref`

## Header

**What this vector exercises.** This vector is a genesis (`sequence = 0`) append to the ledger scope `test-response-ledger` carrying a **`PayloadExternal`** payload reference (Core §6.4) — the off-graph variant of the `PayloadRef` tagged union. Where `append/001-minimal-inline-payload` pinned a `PayloadInline` envelope that embeds the ciphertext as a bstr, `003` pins a `PayloadExternal` envelope that carries `{ref_type, content_hash, availability, retrieval_hint}` and leaves the ciphertext bytes external to the event. It exercises, byte-for-byte, every append-critical construction already covered by 001 (the authored / canonical / signed surfaces of §6.8; the `author_event_hash` of §9.5; the `content_hash` over ciphertext of §9.3; the COSE_Sign1 profile of §7.4 + §7.1; the `canonical_event_hash` of §9.2; the `AppendHead` of §10.6) plus one new Core surface: §6.4's `PayloadExternal` CDDL, including the §6.4 equality rule "`EventPayload.content_hash` MUST equal `PayloadExternal.content_hash`".

**Scope of this vector.** This is a **structural-only** vector (same convention as 001): the 64-byte external payload bytes pinned at `../../_inputs/sample-external-payload-003.bin` are used directly as the ciphertext bytes named by `payload_ref` — no AEAD encryption is performed. `KeyBag.entries` is the empty list. The vector does **not** exercise HPKE wrap; 004 is the first vector to do so. The load-bearing claim 003 makes is that the `content_hash` construction of §9.3 does not depend on payload location: running the `trellis-content-v1` domain-separated SHA-256 over the bytes named by `payload_ref` produces the same shape of digest regardless of whether `payload_ref` is `PayloadInline` (bytes embedded) or `PayloadExternal` (bytes external with `availability = InExport`). This is the structural side of invariant #4 (hashes over ciphertext) and the reserved-slot side of invariant #8 (commitment slot populated with a real `PayloadExternal` struct, not empty).

**Genesis choice.** 003 is genesis (`sequence = 0`, `prev_hash = null`) rather than non-genesis. This keeps the vector focused on the single variation it claims to test — the `PayloadExternal` wire shape. Non-genesis chain-linkage (invariant #5, §10.2 `prev_hash` linkage) is already exercised by `append/005-prior-head-chain`; co-exercising it here would dilute 003's single-variation discipline and force a byte-level re-derivation every time 005's canonical event hash changes. The `sequence = 0` / `prev_hash = null` preimage mirrors 001 exactly, so a reviewer diffing 001 and 003 sees the `PayloadExternal` swap in isolation.

**`PayloadExternal` variant pinning.** §6.4 defines `AvailabilityHint` as the enum `{InExport=0, External=1, Withheld=2, Unavailable=3}`. This vector pins:

- `availability = 0` (`InExport`). Rationale: cleanest test of the structure-verified + integrity-verified path. The ciphertext bytes are present in the fixture bundle at `../../_inputs/sample-external-payload-003.bin`, so §19 step 4g's "`PayloadExternal` and `060-payloads/<content_hash>.bin` exists → check `SHA-256(file bytes) under §9.3 == payload.content_hash`" branch runs to success. No `omitted_payload_checks` surface in this vector; `External` / `Withheld` / `Unavailable` are reserved for later tamper / verify vectors that exercise the omitted-checks branch of §19.
- `retrieval_hint = null`. Rationale: `retrieval_hint` is `tstr / null` per §6.4 CDDL; `null` is shorter and sufficient when `availability = InExport` (the retrieval locus is the export bundle itself, there is nothing to hint at). A future vector exercising `availability = External` (bytes retrievable from a content-addressed external store) will populate `retrieval_hint` with a URL-like text string.

**Pinned inputs.**

| Input | Value | Source |
|---|---|---|
| `signing_key` | Ed25519 COSE_Key, same as 001 | `../../_keys/issuer-001.cose_key` |
| `payload` | 64 bytes, ASCII `"Trellis fixture external payload #003 (InExport)"` + `0x00` padding | `../../_inputs/sample-external-payload-003.bin` |
| `ledger_scope` | bstr `"test-response-ledger"` | §10.4 ledger scope; same as 001/005 |
| `sequence` | `0` (genesis) | §10.2: `prev_hash` MUST be `null` for `sequence == 0` |
| `timestamp` (`authored_at`) | `1745000003` | §12.1 `authored_at: uint` (Unix seconds UTC) |
| `event_type` | bstr `"x-trellis-test/append-external"` | §14.6 reserved test-identifier prefix |
| `classification` | bstr `"x-trellis-test/unclassified"` | §14.6 reserved test-identifier prefix; inherited from 001 |
| `retention_tier` | `0` | §12.1 `retention_tier: uint .size 1` |
| `idempotency_key` | bstr `"idemp-append-003"` (16 bytes) | §6.1 `.size (1..64)`; §17.3 identity distinct from 001's `"idemp-append-001"` and 005's `"idemp-append-005"` |
| `suite_id` | `1` | §7.1 Phase 1 pin: Ed25519 / COSE_Sign1 |
| `PayloadExternal.availability` | `0` (`InExport`) | §6.4 `AvailabilityHint` enum |
| `PayloadExternal.retrieval_hint` | `null` | §6.4 `tstr / null`; `null` selected for `InExport` |

**Core § roadmap (in traversal order).**

1. §5.1 — dCBOR encoding profile. Every CBOR structure below is serialized per §5.1's byte-wise lexicographic canonical ordering.
2. §8.3 — `kid` derivation: `SHA-256(dCBOR_encode_uint(suite_id) || pubkey_raw)[0..16]`. Same inputs as 001/005 → same 16 bytes.
3. §9.3 + §9.1 — `content_hash` over the pinned external ciphertext bytes, domain-separated by `"trellis-content-v1"`. **The external bytes are the "exact ciphertext bytes named by `payload_ref`" of §9.3 — location (inline vs external) is not load-bearing to the hash.**
4. §6.4 — Build `PayloadExternal` CDDL struct. Pin `content_hash` to equal the value computed in step 3 (and the sibling `EventPayload.content_hash`).
5. §6.8 (authored form) + §9.5 — `AuthorEventHashPreimage` dCBOR-serialized with the `PayloadExternal` payload_ref; committed as `input-author-event-hash-preimage.cbor`.
6. §9.5 + §9.1 — `author_event_hash` preimage bytes (`trellis-author-event-v1` domain separation over step 5); SHA-256 digest.
7. §6.8 (canonical form) + §6.1 — `EventPayload` dCBOR-serialized; committed as `expected-event-payload.cbor`. This bstr is the COSE_Sign1 payload. §6.4 equality `EventPayload.content_hash == PayloadExternal.content_hash` is load-bearing here.
8. §7.4 — COSE protected-header map with `alg`, `kid`, `suite_id`, dCBOR-serialized, wrapped in a bstr. Identical to 001/005.
9. §7.4 + RFC 9052 §4.4 — `Sig_structure = ["Signature1", protected_bstr, external_aad, payload_bstr]` with `external_aad = h''` per §6.6.
10. §7.1 — Ed25519 signature over the `Sig_structure` bytes.
11. §6.1 + §6.8 (signed form) + §7.4 — COSE_Sign1 tag-18 envelope; committed as `expected-event.cbor`.
12. §9.2 + §9.1 — `canonical_event_hash` over `dCBOR(CanonicalEventHashPreimage)` under `"trellis-event-v1"`.
13. §10.6 — `AppendHead = {scope, sequence, canonical_event_hash}` dCBOR-serialized; committed as `expected-append-head.cbor`.

---

## Body

### Step 1: dCBOR encoding profile

**Core § citation:** §5.1 Pinned encoding: dCBOR.

**Load-bearing sentence:**

> "All Trellis byte-level structures … are serialized as **deterministic CBOR (dCBOR)**, which for this specification means the Core Deterministic Encoding profile of [RFC 8949] §4.2.2: Integers encoded in the smallest possible representation (no leading zero-length prefixes). Map keys sorted in byte-wise lexicographic order of their canonical CBOR encoding; duplicate keys rejected. No indefinite-length items (all arrays, maps, byte strings, text strings use definite-length encoding)."

**Operation:** Every CBOR encode operation below MUST satisfy §5.1. The 12 text-string keys common to `AuthorEventHashPreimage` (Step 5) and the 13 keys of `EventPayload` (Step 7) sort in byte-wise lex order exactly as in 001's derivation.md Step 1 — the key-set is identical; the payload_ref sub-map is what differs. `PayloadExternal` contributes four text-string keys whose canonical CBOR encodings sort as follows:

```
  68 72 65 66 5f 74 79 70 65                         ref_type        (len 8 ,  major-3)
  6c 61 76 61 69 6c 61 62 69 6c 69 74 79             availability    (len 12,  major-3)
  6c 63 6f 6e 74 65 6e 74 5f 68 61 73 68             content_hash    (len 12,  major-3)
  6e 72 65 74 72 69 65 76 61 6c 5f 68 69 6e 74       retrieval_hint  (len 14,  major-3)
```

Where the leading byte ties (the two `0x6c`-prefixed keys `availability` / `content_hash`), byte-wise comparison continues: `0x61 < 0x63`, so `availability` sorts before `content_hash`. This produces the 4-key map prefix `a4 ref_type … availability … content_hash … retrieval_hint`.

---

### Step 2: Derive `kid` from `suite_id` and the issuer public key

**Core § citation:** §8.3 `kid` format → Derived `kid` construction (pinned).

**Load-bearing sentence:**

> "When a `kid` is derived, it MUST be the first 16 bytes of: `SHA-256( dCBOR_encode_uint(suite_id_integer) || pubkey_raw )` where: `dCBOR_encode_uint(x)` is the canonical dCBOR encoding of the unsigned integer `x` per §5.1 …, `pubkey_raw` is the raw public-key bytes for the suite …, `||` denotes byte-string concatenation."

**Inputs:**

- `suite_id = 1` (§7.1). Its dCBOR encoding per §5.1 is the single byte `0x01`.
- `pubkey_raw` is the 32-byte Ed25519 public key from the pinned COSE_Key at `../../_keys/issuer-001.cose_key` (COSE_Key label `-2`):

```
71de3b4e933aa718a6f5c45845ee83af8000a450f9572d4cf393e681c8144191
```

**Operation:** `kid = SHA-256(0x01 || pubkey_raw)[0..16]`.

**Result (16 bytes):**

```
af9dff525391faa75c8e8da4808b1743
```

**Committed as:** embedded in the COSE protected header (Step 8) and ultimately in `expected-event.cbor`. Same kid as 001 / 005 because the same issuer key + same suite_id are in use (no rotation here; rotation is 002's scope).

---

### Step 3: `content_hash` over the external ciphertext bytes

**Core § citation:** §9.3 Content hash (`trellis-content-v1`, over ciphertext); §9.1 Domain separation discipline.

**Load-bearing sentence (§9.3):**

> "`content_hash = SHA-256( "trellis-content-v1" domain-separated per §9.1 over the exact ciphertext bytes named by payload_ref )`. `content_hash` is over **ciphertext**, never plaintext."

**Load-bearing sentence (§9.1):**

> "Every hash computation in Trellis is domain-separated by a length-prefixed UTF-8 byte tag. The generic form is: `digest = SHA-256( len(tag) as 4-byte big-endian unsigned || tag as UTF-8 bytes || len(component) as 4-byte big-endian unsigned || component as raw bytes )`."

**The load-bearing observation for this vector.** §9.3 names "the exact ciphertext bytes **named by** `payload_ref`", not "the ciphertext bytes **embedded in** `payload_ref`". For `PayloadInline` (§6.4) the named bytes are the `ciphertext` bstr field; for `PayloadExternal` the named bytes live at a location identified by `retrieval_hint` (or, when `availability = InExport`, at the export's `060-payloads/<content_hash>.bin` slot per §19 step 4g). The hash construction is identical. This is what 003 proves byte-exactly: swap `PayloadInline` for `PayloadExternal` and the `content_hash` construction is unchanged — only the 32-byte output differs because the ciphertext bytes differ.

**Inputs:**

- `tag = "trellis-content-v1"` (§9.8 domain-tag registry, §9.3). UTF-8 length 18.
- `component` = the 64 bytes of `../../_inputs/sample-external-payload-003.bin` (ASCII `"Trellis fixture external payload #003 (InExport)"`, 49 bytes, followed by 15 bytes of `0x00`):

```
5472656c6c69732066697874757265206578746572
6e616c207061796c6f61642023303033202849
6e4578706f7274290000000000000000000000
000000
```

(rendered wrapped; bytes laid out linearly in the Footer dump of the input file is not re-dumped here because the input bytes are an external `_inputs/` artifact; see `../../_inputs/sample-external-payload-003.bin`).

**Operation:**

```
preimage =   0x00 0x00 0x00 0x12        ; len(tag) = 18 as u32be
          || "trellis-content-v1"        ; UTF-8 bytes
          || 0x00 0x00 0x00 0x40        ; len(component) = 64 as u32be
          || <64 external-ciphertext bytes>

content_hash = SHA-256(preimage)
```

**Result (32 bytes):**

```
aa9135f8c9231952dadd257bbaf61055edcbb938331b384859aa86d96ef7d0f6
```

**Committed as:** embedded in the `author_event_hash` preimage, in the `EventPayload`, and — per §6.4 — in the `PayloadExternal` struct itself (twice-over, at `EventPayload.content_hash` and at `PayloadExternal.content_hash`; these MUST be byte-equal per §6.4).

---

### Step 4: Build `PayloadExternal` (authored + canonical `payload_ref`)

**Core § citation:** §6.4 `PayloadRef = PayloadInline / PayloadExternal`; §19 step 4g (verification's `PayloadExternal` branch).

**Load-bearing sentence (§6.4 CDDL):**

```cddl
PayloadExternal = {
  ref_type:       "external",
  content_hash:   digest,
  availability:   AvailabilityHint,
  retrieval_hint: tstr / null,
}

AvailabilityHint = &(
  InExport:     0,
  External:     1,
  Withheld:     2,
  Unavailable:  3,
)
```

**Load-bearing sentence (§6.4 equality rule):**

> "For `PayloadExternal`, `EventPayload.content_hash` MUST equal `PayloadExternal.content_hash`; if ciphertext bytes are not present in the export, an offline verifier reports that payload integrity and readability checks could not run (§19) rather than pretending they succeeded."

**Field population:**

| Field | Value |
|---|---|
| `ref_type` | text string `"external"` (8 bytes: `0x68 6578 7465 726e 616c`) |
| `content_hash` | 32 bytes from Step 3 |
| `availability` | `0` (the `InExport` enum value) |
| `retrieval_hint` | `null` |

**Operation:** dCBOR-encode the 4-key map (§5.1). Per Step 1's sort analysis the 4 keys serialize in the order `ref_type, availability, content_hash, retrieval_hint`.

**Result (inner bytes, as serialized inside `AuthorEventHashPreimage` / `EventPayload`):**

```
a4                                                        ; map(4)
  68 7265665f74797065        68 6578 7465 726e 616c         ; "ref_type" → "external"
  6c 617661696c6162696c697479  00                           ; "availability" → 0 (InExport)
  6c 636f6e74656e745f68617368 5820 <32-byte content_hash>   ; "content_hash" → bstr(32)
  6e 72657472696576616c5f68696e74 f6                       ; "retrieval_hint" → null
```

**Note on `ref_type` variant tagging.** §6.4 uses CDDL group-choice discrimination on the literal `"inline"` / `"external"` text strings rather than a CBOR tag. The byte shape is therefore a plain map whose `ref_type` key carries the discriminator text. A verifier reading `payload_ref.ref_type` decides which branch of `PayloadRef` to process (001's `"inline"` → `ciphertext` + `nonce`; 003's `"external"` → `content_hash` + `availability` + `retrieval_hint`).

**Committed as:** embedded inside `input-author-event-hash-preimage.cbor` (Step 5) and `expected-event-payload.cbor` (Step 7). Not written to a standalone sibling file — the inner bytes are fully determined by this step's procedure and visible in the Footer dumps below.

---

### Step 5: Build `AuthorEventHashPreimage` (authored form per §6.8)

**Core § citation:** §6.8 Three event surfaces (authored form); §9.5 `author_event_hash` construction; Appendix A (§28) CDDL.

**Load-bearing CDDL (§9.5 / Appendix A):**

```cddl
AuthorEventHashPreimage = {
  version:         uint .size 1,
  ledger_scope:    bstr,
  sequence:        uint,
  prev_hash:       digest / null,
  causal_deps:     [* digest] / null,
  content_hash:    digest,
  header:          EventHeader,
  commitments:     [* Commitment] / null,
  payload_ref:     PayloadRef,
  key_bag:         KeyBag,
  idempotency_key: bstr .size (1..64),
  extensions:      { * tstr => any } / null,
}
```

**Field population** (per the pinned inputs above):

| Field | Value | Core citation |
|---|---|---|
| `version` | `1` | §6.1 wire-format version |
| `ledger_scope` | `"test-response-ledger"` (bstr, 20 bytes) | §6.1, §10.4 |
| `sequence` | `0` | §10.2 (genesis) |
| `prev_hash` | `null` | §10.2: "For `sequence == 0`: `prev_hash` MUST be `null`." |
| `causal_deps` | `null` | §10.3: "Phase 1 events MUST emit `causal_deps` as `null` or `[]`." |
| `content_hash` | 32 bytes from Step 3 | §9.3 |
| `header` | `EventHeader` map; see below | §12.1 |
| `commitments` | `null` | §13.3: "Phase 1 producers MUST emit `commitments` as `null` or `[]`." |
| `payload_ref` | `PayloadExternal` map from Step 4 | §6.4 |
| `key_bag` | `KeyBag` map with `entries: []` | §9.4 (empty admissible under `[* KeyBagEntry]`) |
| `idempotency_key` | `"idemp-append-003"` (16 bytes) | §6.1 `.size (1..64)`; §17.3 distinct from 001/005 |
| `extensions` | `null` | §6.5 Phase 1 emission rule |

**`EventHeader` sub-map (§12.1):** same shape as 001; only the `event_type` and `authored_at` values change.

| Field | Value |
|---|---|
| `event_type` | `"x-trellis-test/append-external"` (bstr, 30 bytes) — §14.6 |
| `authored_at` | `1745000003` — §12.1 |
| `retention_tier` | `0` — §12.1 |
| `classification` | `"x-trellis-test/unclassified"` (bstr, 27 bytes) — §14.6 |
| `outcome_commitment` | `null` |
| `subject_ref_commitment` | `null` |
| `tag_commitment` | `null` |
| `witness_ref` | `null` — reserved for Phase 4 |
| `extensions` | `null` — §12.3 |

**`KeyBag` sub-map (§9.4):** `{ "entries": [] }`. Structural-only; HPKE wrap is exercised by 004, not here.

**Operation:** dCBOR-encode the map (§5.1 byte-wise lex key ordering).

**Result (522 bytes):** See the Footer dump for `input-author-event-hash-preimage.cbor`. The first byte is `0xac` (map with 12 text-keyed entries — same 12 keys as 001's `AuthorEventHashPreimage`). The 12 top-level keys appear in byte-wise lex order identical to 001: `header`, `key_bag`, `version`, `sequence`, `prev_hash`, `extensions`, `causal_deps`, `commitments`, `payload_ref`, `content_hash`, `ledger_scope`, `idempotency_key`. The load-bearing delta vs 001 is inside `payload_ref`: where 001's sub-map is `a3` (3 keys: `ciphertext`, `nonce`, `ref_type`), 003's is `a4` (4 keys: `ref_type`, `availability`, `content_hash`, `retrieval_hint`).

**Committed as:** `input-author-event-hash-preimage.cbor` (522 bytes).

---

### Step 6: `author_event_hash`

**Core § citation:** §9.5 `author_event_hash` construction; §9.1 Domain separation discipline.

**Load-bearing sentence:**

> "`author_event_hash = SHA-256( "trellis-author-event-v1" domain-separated per §9.1 over dCBOR(AuthorEventHashPreimage) )`. `author_event_hash` binds the envelope payload, payload reference, and key bag at the moment of signing."

**Inputs:**

- `tag = "trellis-author-event-v1"` (UTF-8 length 23, per §9.8).
- `component = dCBOR(AuthorEventHashPreimage)` = the 522 bytes of Step 5.

**Operation:**

```
preimage =   0x00 0x00 0x00 0x17        ; len(tag) = 23
          || "trellis-author-event-v1"   ; UTF-8
          || 0x00 0x00 0x02 0x0a        ; len(component) = 522
          || <522 bytes of Step 5 output>

author_event_hash = SHA-256(preimage)
```

**Result (preimage, 553 bytes):** Committed as `author-event-preimage.bin`. First 8 bytes are `0x00 00 00 17 74 72 65 6c` = `[len=23][t r e l`.

**Result (digest, 32 bytes):**

```
ae3c92194521b67d41081a37738369f05e46b1ce063177b5652d75485463f4df
```

**Committed as:** preimage bytes as `author-event-preimage.bin`; digest as `author-event-hash.bin`.

---

### Step 7: Build `EventPayload` (canonical form per §6.8)

**Core § citation:** §6.8 Three event surfaces (canonical form); §6.1 Normative structure; §6.4 equality rule; Appendix A (§28) CDDL.

**Load-bearing CDDL (§6.1 / Appendix A):** identical to 001's Step 6 — `EventPayload` has the same 13 keys regardless of `payload_ref` variant.

**Field population:** identical to Step 5's `AuthorEventHashPreimage` *plus* the `author_event_hash` field set to the 32-byte digest from Step 6. All other fields — including the `PayloadExternal` sub-map — are byte-identical to Step 5.

**§6.4 equality check:**

```
EventPayload.content_hash          = aa9135f8c9231952dadd257bbaf61055edcbb938331b384859aa86d96ef7d0f6
EventPayload.payload_ref.content_hash = aa9135f8c9231952dadd257bbaf61055edcbb938331b384859aa86d96ef7d0f6
                                     equal ✓
```

**Operation:** dCBOR-encode (§5.1).

**Result (574 bytes):** See Footer dump for `expected-event-payload.cbor`. The first byte is `0xad` (map with 13 text-keyed entries — 12 from Step 5 plus `author_event_hash`). These are the bytes that will become the COSE_Sign1 `payload` bstr in Step 11.

**Committed as:** `expected-event-payload.cbor` (574 bytes).

---

### Step 8: Build the COSE protected-header bstr

**Core § citation:** §7.4 COSE protected headers and Sig_structure.

**Load-bearing sentence:**

> "For every Trellis COSE_Sign1 artifact, the protected header MUST contain: `alg` — Label (integer key) `1` … . `kid` — Label (integer key) `4` … . `suite_id` — Label (integer key) `-65537` … ."

**Operation:** Build the map `{1: -8, 4: <kid-16B>, -65537: 1}`, serialize as dCBOR (§5.1), then wrap the resulting bytes in a CBOR bstr. Identical to 001 / 005 — the protected header does not depend on payload_ref.

**Inner protected-header map bytes (27 bytes):**

```
a301270450af9dff525391faa75c8e8da4808b17433a0001000001
```

**Wrapped as bstr (29 bytes):** `581b a301270450af9dff525391faa75c8e8da4808b17433a0001000001`.

**Committed as:** embedded in `sig-structure.bin` and `expected-event.cbor`; fully reconstructible from Step 2's `kid` and this step's procedure.

---

### Step 9: Build `Sig_structure` per RFC 9052 §4.4

**Core § citation:** §7.4 step 5; §6.6 Signature scope.

**Load-bearing sentence (§7.4 step 5):**

> "Sign the RFC 9052 `Sig_structure` array `["Signature1", protected, external_aad, payload]`, with `external_aad` equal to the zero-length byte string for Phase 1."

**Operation:** Construct the 4-element CBOR array:

1. Text string `"Signature1"`.
2. The protected bstr from Step 8 (27-byte inner map wrapped as a 29-byte bstr).
3. `external_aad = h''` (zero-length bstr).
4. The `payload` bstr = `dCBOR(EventPayload)` bytes from Step 7 wrapped as a bstr.

Serialize the array as dCBOR (§5.1). Arrays are not key-sorted; their element order is fixed by RFC 9052.

**Result (619 bytes):** Array prefix `0x84` (array of 4). First element `0x6a 5369676e61747572 6531` (text string `"Signature1"`, major type 3, length 10). Second element is the 27-byte inner protected bstr wrapped (`0x58 0x1b` + 27 bytes). Third element is `0x40` (zero-length bstr). Fourth element is the 574-byte `EventPayload` bstr wrapped (`0x59 0x02 0x3e` + 574 bytes).

**Committed as:** `sig-structure.bin` (619 bytes). Full dump in the Footer.

---

### Step 10: Ed25519 signature

**Core § citation:** §7.1 Pinned Phase 1 suite.

**Inputs:**

- Signing seed: the 32 bytes under COSE_Key label `-4` at `../../_keys/issuer-001.cose_key` (same as 001 / 005).
- Message: the 619-byte `Sig_structure` bytes from Step 9.

**Operation:** `signature = Ed25519-Sign(seed, Sig_structure_bytes)` per [RFC 8032] §5.1.6. Ed25519 is pure (no pre-hash).

**Result (64 bytes):**

```
336a8ea3a2befa6df19762d1100a741fb514dec1becbbe089167b4fd7f44d075
f3e32015966c2e27907fc43b6779d6a531ac3695601a75d64ed2fe119fe7da0d
```

**Committed as:** embedded in `expected-event.cbor`; deterministic under RFC 8032 for fixed seed + fixed message.

---

### Step 11: Assemble the COSE_Sign1 tag-18 envelope (signed form per §6.8)

**Core § citation:** §6.8 Three event surfaces (signed form); §6.1; §7.4.

**Operation:** Build the tagged 4-array per RFC 9052 §4.2:

```
COSE_Sign1 = 18([ protected_bstr, unprotected_map, payload_bstr, signature_bstr ])
```

- `protected_bstr` — 29-byte wrapped protected header from Step 8.
- `unprotected_map` — empty CBOR map `{}` (no unprotected headers).
- `payload_bstr` — 574-byte `EventPayload` from Step 7 wrapped as a bstr.
- `signature_bstr` — 64-byte Ed25519 signature from Step 10 wrapped as a bstr.

Apply CBOR tag 18 (`0xd2`) to the 4-array. Serialize as dCBOR.

**Result (675 bytes):** First bytes `0xd2 0x84 0x58 0x1b` (tag 18, array of 4, bstr length 27). Full dump in the Footer.

**Committed as:** `expected-event.cbor` (675 bytes).

---

### Step 12: `canonical_event_hash`

**Core § citation:** §9.2 Canonical event hash (`trellis-event-v1`); §9.1.

**Load-bearing sentence:**

> "`canonical_event_hash = SHA-256( "trellis-event-v1" domain-separated per §9.1 over dCBOR(CanonicalEventHashPreimage) )`."

**Load-bearing CDDL (§9.2):**

```cddl
CanonicalEventHashPreimage = {
  version:       uint .size 1,
  ledger_scope:  bstr,
  event_payload: EventPayload,
}
```

**Inputs:**

- `tag = "trellis-event-v1"` (UTF-8 length 16, per §9.8).
- `component = dCBOR({ "version": 1, "ledger_scope": b"test-response-ledger", "event_payload": <EventPayload from Step 7> })`.

**Operation:**

```
preimage = 0x00 0x00 0x00 0x10            ; len(tag) = 16
        || "trellis-event-v1"              ; UTF-8
        || <4-byte BE len(component)>
        || <component bytes>

canonical_event_hash = SHA-256(preimage)
```

**Result (32 bytes):**

```
09c2b491e71049b555d985d29d863cc69f316e001968d05444a083a29f2f3267
```

**Committed as:** embedded in `expected-append-head.cbor` (Step 13); reconstructible from Step 7 and this procedure.

---

### Step 13: Build `AppendHead`

**Core § citation:** §10.6 Append head artifact; Appendix A (§28) CDDL.

**Field population:**

| Field | Value | Source |
|---|---|---|
| `scope` | `"test-response-ledger"` (bstr, 20 bytes) | equal to `EventPayload.ledger_scope` per §10.6 |
| `sequence` | `0` | equal to `EventPayload.sequence` per §10.6 |
| `canonical_event_hash` | 32 bytes from Step 12 | §9.2 |

**Operation:** dCBOR-encode the 3-key map (§5.1).

**Result (93 bytes):** See Footer dump for `expected-append-head.cbor`. First byte is `0xa3` (map with 3 entries).

**Committed as:** `expected-append-head.cbor` (93 bytes).

---

## Footer — full hex dumps

Each block below is the byte-exact content of the named sibling file. A reader can verify by running `xxd` (or equivalent) on the sibling and comparing.

### `input-author-event-hash-preimage.cbor` (522 bytes, sha256 `59547682d1f2b96aae1151cea517a1c62027f99c1f35cac2154a0f22fe5375a0`)

```
ac66686561646572a96a6576656e745f74797065581e782d7472656c6c69732d
746573742f617070656e642d65787465726e616c6a657874656e73696f6e73f6
6b617574686f7265645f61741a680296436b7769746e6573735f726566f66e63
6c617373696669636174696f6e581b782d7472656c6c69732d746573742f756e
636c61737369666965646e726574656e74696f6e5f74696572006e7461675f63
6f6d6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74f67673
75626a6563745f7265665f636f6d6d69746d656e74f6676b65795f626167a167
656e7472696573806776657273696f6e016873657175656e6365006970726576
5f68617368f66a657874656e73696f6e73f66b63617573616c5f64657073f66b
636f6d6d69746d656e7473f66b7061796c6f61645f726566a4687265665f7479
70656865787465726e616c6c617661696c6162696c697479006c636f6e74656e
745f686173685820aa9135f8c9231952dadd257bbaf61055edcbb938331b3848
59aa86d96ef7d0f66e72657472696576616c5f68696e74f66c636f6e74656e74
5f686173685820aa9135f8c9231952dadd257bbaf61055edcbb938331b384859
aa86d96ef7d0f66c6c65646765725f73636f706554746573742d726573706f6e
73652d6c65646765726f6964656d706f74656e63795f6b6579506964656d702d
617070656e642d303033
```

The `0xa4 …` block that appears twice — once embedded inside the top-level map at key `payload_ref` (starting `a4 68 72 65 66 5f 74 79 70 65 …`), and once as the four-field sub-map proper — is the `PayloadExternal` struct of Step 4.

### `author-event-preimage.bin` (553 bytes, sha256 `ae3c92194521b67d41081a37738369f05e46b1ce063177b5652d75485463f4df`)

```
000000177472656c6c69732d617574686f722d6576656e742d76310000020aac
66686561646572a96a6576656e745f74797065581e782d7472656c6c69732d74
6573742f617070656e642d65787465726e616c6a657874656e73696f6e73f66b
617574686f7265645f61741a680296436b7769746e6573735f726566f66e636c
617373696669636174696f6e581b782d7472656c6c69732d746573742f756e63
6c61737369666965646e726574656e74696f6e5f74696572006e7461675f636f
6d6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74f6767375
626a6563745f7265665f636f6d6d69746d656e74f6676b65795f626167a16765
6e7472696573806776657273696f6e016873657175656e63650069707265765f
68617368f66a657874656e73696f6e73f66b63617573616c5f64657073f66b63
6f6d6d69746d656e7473f66b7061796c6f61645f726566a4687265665f747970
656865787465726e616c6c617661696c6162696c697479006c636f6e74656e74
5f686173685820aa9135f8c9231952dadd257bbaf61055edcbb938331b384859
aa86d96ef7d0f66e72657472696576616c5f68696e74f66c636f6e74656e745f
686173685820aa9135f8c9231952dadd257bbaf61055edcbb938331b384859aa
86d96ef7d0f66c6c65646765725f73636f706554746573742d726573706f6e73
652d6c65646765726f6964656d706f74656e63795f6b6579506964656d702d61
7070656e642d303033
```

Note: bytes 0..3 are `00 00 00 17` (length-prefix, 23) and bytes 4..26 are `"trellis-author-event-v1"`; bytes 27..30 are `00 00 02 0a` (length-prefix, 522) and bytes 31..552 reproduce `input-author-event-hash-preimage.cbor` verbatim.

### `author-event-hash.bin` (32 bytes, sha256 `2ca087149a4eda05d1f64bbf2c5412c4765b61316d3a7624b0df81827d918adc`)

```
ae3c92194521b67d41081a37738369f05e46b1ce063177b5652d75485463f4df
```

This is the SHA-256 of `author-event-preimage.bin`; i.e., the file *is* the author_event_hash value. The file's own SHA-256 (shown above) is a second-order integrity check only.

### `expected-event-payload.cbor` (574 bytes, sha256 `d1b43dd1a2933af92ef61d46597c2e07590f9bd9b092cb175116f487430b2b3f`)

```
ad66686561646572a96a6576656e745f74797065581e782d7472656c6c69732d
746573742f617070656e642d65787465726e616c6a657874656e73696f6e73f6
6b617574686f7265645f61741a680296436b7769746e6573735f726566f66e63
6c617373696669636174696f6e581b782d7472656c6c69732d746573742f756e
636c61737369666965646e726574656e74696f6e5f74696572006e7461675f63
6f6d6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74f67673
75626a6563745f7265665f636f6d6d69746d656e74f6676b65795f626167a167
656e7472696573806776657273696f6e016873657175656e6365006970726576
5f68617368f66a657874656e73696f6e73f66b63617573616c5f64657073f66b
636f6d6d69746d656e7473f66b7061796c6f61645f726566a4687265665f7479
70656865787465726e616c6c617661696c6162696c697479006c636f6e74656e
745f686173685820aa9135f8c9231952dadd257bbaf61055edcbb938331b3848
59aa86d96ef7d0f66e72657472696576616c5f68696e74f66c636f6e74656e74
5f686173685820aa9135f8c9231952dadd257bbaf61055edcbb938331b384859
aa86d96ef7d0f66c6c65646765725f73636f706554746573742d726573706f6e
73652d6c65646765726f6964656d706f74656e63795f6b6579506964656d702d
617070656e642d30303371617574686f725f6576656e745f686173685820ae3c
92194521b67d41081a37738369f05e46b1ce063177b5652d75485463f4df
```

(The final 35 bytes, `71 617574686f725f6576656e745f68617368 5820 <32-byte digest>`, carry the text key `"author_event_hash"` (CBOR major-type-3, length 17) and its digest bstr value. The map header is `0xad` (major type 5, length 13) rather than the `0xac` (length 12) of `input-author-event-hash-preimage.cbor` because `EventPayload` has this one additional key.)

### `sig-structure.bin` (619 bytes, sha256 `3a81a277b0a43d386d2df4c957f17de629cbec20f934aadd58a7af6daea190ad`)

```
846a5369676e617475726531581ba301270450af9dff525391faa75c8e8da480
8b17433a00010000014059023ead66686561646572a96a6576656e745f747970
65581e782d7472656c6c69732d746573742f617070656e642d65787465726e61
6c6a657874656e73696f6e73f66b617574686f7265645f61741a680296436b77
69746e6573735f726566f66e636c617373696669636174696f6e581b782d7472
656c6c69732d746573742f756e636c61737369666965646e726574656e74696f
6e5f74696572006e7461675f636f6d6d69746d656e74f6726f7574636f6d655f
636f6d6d69746d656e74f6767375626a6563745f7265665f636f6d6d69746d65
6e74f6676b65795f626167a167656e7472696573806776657273696f6e016873
657175656e63650069707265765f68617368f66a657874656e73696f6e73f66b
63617573616c5f64657073f66b636f6d6d69746d656e7473f66b7061796c6f61
645f726566a4687265665f747970656865787465726e616c6c617661696c6162
696c697479006c636f6e74656e745f686173685820aa9135f8c9231952dadd25
7bbaf61055edcbb938331b384859aa86d96ef7d0f66e72657472696576616c5f
68696e74f66c636f6e74656e745f686173685820aa9135f8c9231952dadd257b
baf61055edcbb938331b384859aa86d96ef7d0f66c6c65646765725f73636f70
6554746573742d726573706f6e73652d6c65646765726f6964656d706f74656e
63795f6b6579506964656d702d617070656e642d30303371617574686f725f65
76656e745f686173685820ae3c92194521b67d41081a37738369f05e46b1ce06
3177b5652d75485463f4df
```

Structure (per RFC 9052 §4.4): `0x84` (array of 4); `0x6a "Signature1"` (text string, 10 bytes); `0x58 0x1b <27-byte protected header>`; `0x40` (zero-length external_aad bstr); `0x59 0x02 0x3e <574-byte EventPayload>`.

### `expected-event.cbor` (675 bytes, sha256 `302bf6cb4d1b8e2a2f59ed903b11ef6381e46062d9c6188a5d139a36dc6a56df`)

```
d284581ba301270450af9dff525391faa75c8e8da4808b17433a0001000001a0
59023ead66686561646572a96a6576656e745f74797065581e782d7472656c6c
69732d746573742f617070656e642d65787465726e616c6a657874656e73696f
6e73f66b617574686f7265645f61741a680296436b7769746e6573735f726566
f66e636c617373696669636174696f6e581b782d7472656c6c69732d74657374
2f756e636c61737369666965646e726574656e74696f6e5f74696572006e7461
675f636f6d6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74
f6767375626a6563745f7265665f636f6d6d69746d656e74f6676b65795f6261
67a167656e7472696573806776657273696f6e016873657175656e6365006970
7265765f68617368f66a657874656e73696f6e73f66b63617573616c5f646570
73f66b636f6d6d69746d656e7473f66b7061796c6f61645f726566a468726566
5f747970656865787465726e616c6c617661696c6162696c697479006c636f6e
74656e745f686173685820aa9135f8c9231952dadd257bbaf61055edcbb93833
1b384859aa86d96ef7d0f66e72657472696576616c5f68696e74f66c636f6e74
656e745f686173685820aa9135f8c9231952dadd257bbaf61055edcbb938331b
384859aa86d96ef7d0f66c6c65646765725f73636f706554746573742d726573
706f6e73652d6c65646765726f6964656d706f74656e63795f6b657950696465
6d702d617070656e642d30303371617574686f725f6576656e745f6861736858
20ae3c92194521b67d41081a37738369f05e46b1ce063177b5652d75485463f4
df5840336a8ea3a2befa6df19762d1100a741fb514dec1becbbe089167b4fd7f
44d075f3e32015966c2e27907fc43b6779d6a531ac3695601a75d64ed2fe119f
e7da0d
```

Structure: `0xd2` (tag 18); `0x84` (array of 4); `0x58 0x1b <27-byte protected>`; `0xa0` (empty unprotected map); `0x59 0x02 0x3e <574-byte payload>`; `0x58 0x40 <64-byte signature>`.

### `expected-append-head.cbor` (93 bytes, sha256 `fb9c963ab9c9d5a0e093f1227185f234ca7e2c90ca0d578aeb6d84647678ef8b`)

```
a36573636f706554746573742d726573706f6e73652d6c656467657268736571
75656e6365007463616e6f6e6963616c5f6576656e745f68617368582009c2b4
91e71049b555d985d29d863cc69f316e001968d05444a083a29f2f3267
```

Structure: `0xa3` (map of 3); `0x65 "scope" 0x54 <20-byte ledger_scope>`; `0x68 "sequence" 0x00`; `0x74 "canonical_event_hash" 0x58 0x20 <32-byte digest>`.

### External-input sibling: `../../_inputs/sample-external-payload-003.bin` (64 bytes)

```
5472656c6c69732066697874757265206578746572
6e616c207061796c6f61642023303033202849
6e4578706f7274290000000000000000000000
000000
```

(shown wrapped; file bytes are laid out as a flat 64-byte ASCII-then-`0x00`-padding payload — see the file on disk for the canonical byte layout). SHA-256 of this file is NOT the `content_hash` of Step 3 — the file's own SHA-256 is a second-order integrity check; the `content_hash` is the SHA-256 of the §9.1 length-prefixed `trellis-content-v1` preimage over these 64 bytes.
