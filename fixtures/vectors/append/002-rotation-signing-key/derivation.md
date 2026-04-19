# Derivation — `append/002-rotation-signing-key`

## Header

**What this vector exercises.** Phase-1 invariant #7 — "`key_bag` /
`author_event_hash` immutable under rotation" — under a signing-key rotation
of the issuer key. The vector runs a self-contained two-event chain in its own
ledger scope `test-rotation-ledger` (Core §10.4): **Event A** is the genesis
(`sequence = 0`) signed by `issuer-001`; between A and B, the signing-key
registry rotates — `issuer-001` is flipped `Active → Retired` per §8.4 and
`issuer-002` is appended as `Active` with `supersedes = kid(issuer-001)` per
§8.2; **Event B** is `sequence = 1`, chained from A via §10.2 `prev_hash`, and
signed by `issuer-002`. The bytes prove, without re-wrap, that

1. Event A's `author_event_hash` is determined entirely by its
   `AuthorEventHashPreimage` bytes (§9.5 CDDL: no `kid` field, no signature
   material). Nothing in the registry mutation of step (iii) above is a
   preimage input, so `author_event_hash(A)` reproduces byte-identically
   after rotation (TR-CORE-038).
2. Event A's signed COSE_Sign1 (committed as `input-pre-rotation-event.cbor`)
   is still verifiable against the post-rotation registry snapshot
   (`input-signing-key-registry-after.cbor`) because `issuer-001`'s entry
   persists with `status = Retired` (§8.4: `Retired` is terminal for signature
   *issuance*, not for *verification of historical records*) —
   demonstrating Core §7.3's migration obligation, TR-CORE-036.
3. Event B's signed COSE_Sign1 (committed as `expected-event.cbor`) is
   verifiable against the post-rotation registry because `issuer-002` is
   present as `Active` with `valid_from = ROTATION_TIMESTAMP`.
4. Event B's `prev_hash` equals Event A's `canonical_event_hash` per §10.2,
   so the chain is intact across the rotation boundary.

**Scope of this vector.** Structural-only for the payload layer —
rotation-**without-re-wrap**. Both events carry an empty `KeyBag` (§9.4 admits
zero entries) and a `PayloadInline` ciphertext bstr holding the pinned 64-byte
plaintext from `../../_inputs/sample-payload-001.bin` opaquely. Because no
HPKE wrap exists to re-wrap, no §8.6 `LedgerServiceWrapEntry` fires; the
immutability claim lands entirely on the signing-key-registry side of §8.
A later vector (`append/004-hpke-wrapped-inline` residue) will exercise the
`LedgerServiceWrapEntry` append-only mechanics once real HPKE wraps exist to
re-wrap.

**Ledger-scope choice.** `test-rotation-ledger` is deliberately distinct from
`test-response-ledger` (001, 005) to avoid collision at `sequence = 1`: per
§10.1 and invariant #5, each ledger scope admits exactly one canonical event
at each sequence position. 005 already claims `sequence = 1` in the
`test-response-ledger` scope; authoring 002 in a separate scope keeps the
invariant #5 claim of 005 unperturbed.

**Pinned inputs.**

| Input | Value | Source |
|---|---|---|
| `signing_key_a` (pre-rotation) | Ed25519 COSE_Key for `issuer-001` (unchanged vs 001) | `../../_keys/issuer-001.cose_key` |
| `signing_key_b` (post-rotation) | Ed25519 COSE_Key for `issuer-002`, seed `…00cc` | `../../_keys/issuer-002.cose_key` |
| `payload` | 64 bytes, ASCII `"Trellis fixture payload #001"` + `0x00` padding | `../../_inputs/sample-payload-001.bin` |
| `ledger_scope` | bstr `"test-rotation-ledger"` (20 bytes) | §10.4 |
| Event A `sequence` | `0` | §10.2 genesis |
| Event A `authored_at` | `1745100000` | §12.1; narrative-only |
| Event A `idempotency_key` | bstr `"idemp-append-002a"` (17 bytes) | §6.1 |
| Rotation timestamp | `1745100060` — pinned as `valid_to` on issuer-001's post-rotation entry and `valid_from` on issuer-002 | §8.2 |
| Event B `sequence` | `1` | §10.2 `sequence > 0` → non-null `prev_hash` |
| Event B `prev_hash` | 32-byte digest `a1baf9ac…0ff12056` — equals `canonical_event_hash(A)` | §10.2 |
| Event B `authored_at` | `1745100120` (post-rotation) | §12.1; narrative-only |
| Event B `idempotency_key` | bstr `"idemp-append-002b"` (17 bytes; distinct from A per §17.3) | §6.1; §17.3 |
| `suite_id` | `1` (Phase-1 pin, shared by both issuers) | §7.1 |
| `PayloadInline.nonce` | 12 bytes of `0x00` (shared A/B) | §6.4 |

**Core § roadmap (in traversal order).**

1. §9.3 + §9.1 — `content_hash` over the 64-byte `PayloadInline.ciphertext`; identical output across both events because the payload bytes and the tag are identical.
2. §6.8 (authored), §9.5, Appendix A — build Event A's `AuthorEventHashPreimage`; serialize as dCBOR; commit as `input-pre-rotation-author-event-hash-preimage.cbor`.
3. §9.5 + §9.1 — `author_event_hash(A)`; commit as `input-pre-rotation-author-event-hash.bin`.
4. §6.8 (canonical) + §6.1 — Event A's `EventPayload`; commit as `input-pre-rotation-event-payload.cbor`.
5. §7.4 — Event A protected header and `Sig_structure`; §7.1 Ed25519 signature under `issuer-001`; §6.1 + §7.4 tag-18 wire envelope; commit as `input-pre-rotation-event.cbor`.
6. §9.2 + §10.6 — Event A `canonical_event_hash(A)` and `AppendHead(A)`; commit `input-pre-rotation-append-head.cbor`.
7. §8.2 + §8.5 — `signing-key-registry-before.cbor` (one entry: issuer-001 Active).
8. §8.4 + §8.2 — signing-key rotation event: issuer-001 transitions `Active → Retired` with `valid_to = ROTATION_TIMESTAMP`; issuer-002 is added with `status = Active`, `valid_from = ROTATION_TIMESTAMP`, `supersedes = kid(issuer-001)`. Commit `signing-key-registry-after.cbor` (two entries).
9. **Invariant #7 reproduction check (§9.5):** Event A's `AuthorEventHashPreimage` bytes are re-formed from the same inputs and recompute to the same `author_event_hash(A)`. No registry field is in scope.
10. §10.2 — set Event B `prev_hash = canonical_event_hash(A)`.
11. §6.8, §9.5, §9.2, §7.4, §7.1, §6.1, §10.6 — Event B's authored / canonical / signed surfaces, signed under `issuer-002`; commit as primary `expected-*` outputs.

---

## Body

### Step 1: `content_hash` over `PayloadInline.ciphertext`

**Core § citation:** §9.3 Content hash; §9.1 Domain separation discipline.

**Inherited from 001 verbatim.** Payload bytes, domain tag, and length-prefix
construction are identical; only the ledger scope differs, and `content_hash`
is scope-independent (§9.3 preimage is the ciphertext bytes only).

**Inputs:**

- `tag = "trellis-content-v1"` (UTF-8 length 18).
- `component = PayloadInline.ciphertext` = 64 bytes from `../../_inputs/sample-payload-001.bin`.

**Operation:** `content_hash = SHA-256(u32be(18) || "trellis-content-v1" || u32be(64) || <64 ciphertext bytes>)`. Procedure reproduced in full at
`append/001-minimal-inline-payload/derivation.md` Step 3; not re-expanded here.

**Result (32 bytes, identical across events A and B and to 001's):**

```
bcdced2dfaf5342cd2baca0560a9d384473fd45e202555f0654cccaa32b4f812
```

---

### Step 2: Event A `AuthorEventHashPreimage` (§6.8 authored form)

**Core § citation:** §6.8 Authored form; §9.5 `author_event_hash` construction; Appendix A (§28) CDDL.

**Load-bearing sentence (§9.5):**

> "`author_event_hash` binds the envelope payload, payload reference, and key
> bag at the moment of signing. It excludes itself and all signature material
> by construction: `AuthorEventHashPreimage` has no `author_event_hash` field
> and no COSE signature field. It is immutable under rotation because none of
> its inputs is altered by service-side re-wraps (§8.6); re-wraps produce
> append-only `LedgerServiceWrapEntry` records outside the author-event scope."

**Load-bearing reading for 002.** The CDDL of `AuthorEventHashPreimage`
(§9.5) has 12 fields: `version`, `ledger_scope`, `sequence`, `prev_hash`,
`causal_deps`, `content_hash`, `header`, `commitments`, `payload_ref`,
`key_bag`, `idempotency_key`, `extensions`. **None of those fields is a
`kid`, a signing key identifier, or a signing-key-registry reference.** The
registry mutation between Event A signing (Step 5) and Event B signing (Step
11) changes none of these 12 fields for Event A. Therefore
`author_event_hash(A)` — the SHA-256 of the domain-separated dCBOR of those
bytes — is invariant under the rotation.

**Field population for Event A:**

| Field | Value | Core citation |
|---|---|---|
| `version` | `1` | §6.1 |
| `ledger_scope` | `"test-rotation-ledger"` | §10.4 |
| `sequence` | `0` | §10.2 genesis |
| `prev_hash` | `null` | §10.2: null iff `sequence == 0` |
| `causal_deps` | `null` | §10.3 (Phase 1) |
| `content_hash` | (Step 1 output) | §9.3 |
| `header.event_type` | `"x-trellis-test/append-minimal"` | §14.6 |
| `header.authored_at` | `1745100000` | §12.1 |
| `header.retention_tier` | `0` | §12.1 |
| `header.classification` | `"x-trellis-test/unclassified"` | §14.6 |
| `header.outcome_commitment` | `null` | §12.2 |
| `header.subject_ref_commitment` | `null` | §12.2 |
| `header.tag_commitment` | `null` | §12.2 |
| `header.witness_ref` | `null` | §12.1 |
| `header.extensions` | `null` | §12.3 |
| `commitments` | `null` | §13.3 |
| `payload_ref` | `PayloadInline{ref_type:"inline", ciphertext:<64 bytes>, nonce:<12×0x00>}` | §6.4 |
| `key_bag` | `{entries: []}` | §9.4 |
| `idempotency_key` | `"idemp-append-002a"` (17 bytes) | §6.1 |
| `extensions` | `null` | §6.5 |

**Operation:** dCBOR-encode the 12-key map (§5.1 byte-wise lex key ordering).

**Result (539 bytes):** committed as
`input-pre-rotation-author-event-hash-preimage.cbor` (sha256 `44967770…7944`).
Full hex in the footer.

---

### Step 3: Event A `author_event_hash`

**Core § citation:** §9.5; §9.1.

**Inputs:**

- `tag = "trellis-author-event-v1"` (UTF-8 length 23).
- `component = dCBOR(AuthorEventHashPreimage_A)` = 539 bytes of Step 2 output.

**Operation:** `preimage_A = u32be(23) || "trellis-author-event-v1" || u32be(539) || <539 bytes>`;
`author_event_hash(A) = SHA-256(preimage_A)`.

**Result (32 bytes):**

```
1a27bd4e1c4da0845bcb23a9d167ce2034a42290f33945a7cc09a4bc485e7d51
```

**Committed as:** `input-pre-rotation-author-event-hash.bin`.

This 32-byte value is the load-bearing artifact for invariant #7. Steps 7 and
8 below mutate the signing-key registry without touching Step 2's preimage
bytes; Step 9 re-executes Steps 2–3 and obtains the same 32 bytes.

---

### Step 4: Event A `EventPayload` (§6.8 canonical form)

**Core § citation:** §6.8 Canonical form; §6.1.

Field population identical to Step 2 **plus** `author_event_hash` set to the
Step 3 digest.

**Operation:** dCBOR-encode (§5.1). Map header becomes `0xad` (13 entries; 12
from the authored preimage plus `author_event_hash`).

**Result (591 bytes):** committed as `input-pre-rotation-event-payload.cbor`
(sha256 `f667e8c8…b30d4f`). Full hex in the footer.

---

### Step 5: Event A COSE_Sign1 signed envelope (§6.8 signed form)

**Core § citations:** §7.4 protected header; RFC 9052 §4.4 `Sig_structure`;
§6.6 Signature scope; §7.1 Ed25519 suite; §8.3 derived `kid`; §6.1 + §6.8
signed form.

**Protected header.** Three-key map `{alg: -8, kid: kid(issuer-001), suite_id: 1}`.
Per §8.3, `kid(issuer-001) = SHA-256(dCBOR(1) || pubkey_001)[0..16] = af9dff52 5391faa7 5c8e8da4 808b1743`.
The serialized protected header bytes are byte-identical to those in 001 and
005 because issuer-001 and the Phase-1 suite are unchanged.

```
inner bytes (27):  a301270450af9dff525391faa75c8e8da4808b17433a0001000001
wrapped bstr (29): 581b a301270450af9dff525391faa75c8e8da4808b17433a0001000001
```

**`Sig_structure`:** `["Signature1", <27-byte inner protected bstr>, h'',
<591-byte EventPayload_A bstr>]` dCBOR-encoded per RFC 9052 §4.4 and §6.6.

**Ed25519 signature:** `sign(issuer-001.seed, Sig_structure_A)` (RFC 8032,
deterministic under fixed seed + fixed message).

**COSE_Sign1 envelope** (RFC 9052 §4.2, tag 18, payload embedded per §6.1):
`d284 581b <27-byte protected> a0 5902 4f <591-byte payload> 5840 <64-byte signature>`.

**Result (692 bytes):** committed as `input-pre-rotation-event.cbor`
(sha256 `a9adbdd6…ab9b1122`). Full hex in the footer.

---

### Step 6: Event A `canonical_event_hash` and `AppendHead`

**Core § citations:** §9.2; §9.1; §10.6.

`CanonicalEventHashPreimage_A = {version: 1, ledger_scope: "test-rotation-ledger", event_payload: <Step 4 EventPayload>}`;
dCBOR-encode; domain-separate under `"trellis-event-v1"` per §9.1; SHA-256.

**Result (`canonical_event_hash(A)`, 32 bytes):**

```
a1baf9ac2b009be8d6c49c05bda2cf6cf6055b2900fd381c296299100ff12056
```

**`AppendHead(A)` (§10.6):** `{scope: "test-rotation-ledger", sequence: 0, canonical_event_hash: <32 bytes>}`;
dCBOR-encoded; 93 bytes. Committed as `input-pre-rotation-append-head.cbor`
(sha256 `306f4c16…d77b06b1b`).

---

### Step 7: `signing-key-registry-before.cbor` snapshot (§8.2, §8.5)

**Core § citations:** §8.2 `SigningKeyEntry`; §8.5 Registry snapshot in every
export.

**Load-bearing sentence (§8.5):**

> "Every export package (§18) MUST include a complete registry snapshot
> resolvable for every `kid` referenced by any event, checkpoint, or
> `LedgerServiceWrapEntry` in the export. […] A verifier encountering a
> `kid` that cannot be resolved against the embedded registry MUST reject
> the export."

**Reading used by this vector.** The registry-before snapshot is the single
committed witness to the service state at the moment Event A is signed. It
contains exactly one entry for `issuer-001`, `status = Active`, `valid_to =
null`, `supersedes = null`. A verifier presented with Event A and this
before-snapshot can resolve Event A's kid, select the Ed25519/COSE_Sign1
suite via `suite_id = 1`, and verify the signature.

**Entry shape** (§8.2 CDDL order — dCBOR canonical order sorts by byte-wise
lex of the UTF-8 keys: `attestation`, `kid`, `pubkey`, `status`, `suite_id`,
`supersedes`, `valid_from`, `valid_to`).

| Field | Value |
|---|---|
| `attestation` | `null` (optional per §8.2) |
| `kid` | 16 bytes: `af9dff525391faa75c8e8da4808b1743` |
| `pubkey` | 32 bytes: Ed25519 public key of issuer-001 |
| `status` | `0` (Active, §8.2 `SigningKeyStatus` enum) |
| `suite_id` | `1` |
| `supersedes` | `null` |
| `valid_from` | `1745100000` (= Event A `authored_at`) |
| `valid_to` | `null` |

**Operation:** dCBOR-encode a one-element array of the map.

**Result (133 bytes):** committed as
`input-signing-key-registry-before.cbor` (sha256 `da06d7e0…b38775`).

---

### Step 8: Signing-key rotation → `signing-key-registry-after.cbor`

**Core § citations:** §8.4 Lifecycle; §8.2 supersedes linkage; §8.5 complete-snapshot requirement.

**Load-bearing sentences (§8.4):**

> "`Active → Rotating`: a successor key has been provisioned; both keys accept
> signatures."
>
> "`Rotating → Retired`: successor is fully deployed; the old key accepts no
> new signatures but remains verifiable for historical material."
>
> "`Retired` is terminal for signature issuance but not for verification of
> historical records."

**Load-bearing sentence (§8.2, `supersedes`):**

> "`supersedes: bstr / null, ; kid this entry replaces, if any`"

**Reading used by this vector.** 002 pins the simplest lifecycle path that
preserves historical verifiability: issuer-001 transitions directly to
`Retired` (§8.4 admits `Active → Rotating → Retired`; the intermediate
`Rotating` state is a production-operations nicety and is elided in this
fixture to keep the after-snapshot to two entries). The after-snapshot:

- **Entry for issuer-001** — same kid/pubkey/suite_id/valid_from as the
  before-snapshot; `status` flips `Active (0) → Retired (2)`; `valid_to` is
  set to `ROTATION_TIMESTAMP = 1745100060`. All other fields unchanged.
- **Entry for issuer-002** — new row with `kid = 3d05ee9ced8f29b60ef84b17d4712e24`,
  `pubkey` = issuer-002 raw public key, `status = 0` (Active), `suite_id = 1`,
  `valid_from = 1745100060`, `valid_to = null`, `supersedes = <16-byte
  kid(issuer-001)>`, `attestation = null`.

**Operation:** dCBOR-encode a two-element array of the two maps.

**Result (285 bytes):** committed as
`input-signing-key-registry-after.cbor` (sha256 `2eae7097…b94d0c158`).

**Reproduction under rotation claim (TR-CORE-036).** Both pre-rotation
(`input-pre-rotation-event.cbor`) and post-rotation
(`expected-event.cbor`) COSE_Sign1 envelopes carry kids that resolve against
the after-snapshot:

- Event A's protected-header kid `af9dff52…8b1743` resolves to the first row
  of the after-snapshot, `status = Retired`. Per §8.4, `Retired` is valid
  for historical verification. The verifier loads the pubkey, selects the
  suite via `suite_id = 1`, runs the Ed25519 verification equation — **it
  holds**, because Event A's bytes have not changed.
- Event B's protected-header kid `3d05ee9c…712e24` resolves to the second
  row, `status = Active`. Verification succeeds under issuer-002's pubkey.

This is the byte-level content of §7.3's multi-decade migration obligation.

---

### Step 9: Invariant #7 reproduction check (TR-CORE-038)

**Core § citations:** §9.5 (`author_event_hash` immutability under rotation);
§8.6 (`LedgerServiceWrapEntry` mechanics — *not* exercised here; this vector
is rotation-without-re-wrap).

**Claim.** Event A's `author_event_hash` — the 32 bytes
`1a27bd4e…485e7d51` committed as
`input-pre-rotation-author-event-hash.bin` in Step 3 — is the SHA-256 of the
domain-separated dCBOR of Event A's `AuthorEventHashPreimage`. After Steps 7
and 8 mutate the signing-key registry, Steps 2 and 3 are re-executed against
the same Event A inputs (`ledger_scope`, `sequence`, `prev_hash`, payload
bytes, header fields, empty `key_bag`, `idempotency_key`). None of those
inputs is a registry reference. The dCBOR output of Step 2 reproduces
byte-identically; the SHA-256 of Step 3 reproduces byte-identically.

**In-generator assertion.** `gen_append_002.py` calls `build_event(...)` for
Event A twice — once before, and once after, Steps 7 and 8 produce the
registry snapshots — and asserts both calls return the same
`author_event_hash` and the same `signed_envelope_bytes`. This is a
generator-side mechanical restatement of the Core §9.5 claim; it is
authoritative only in that a failing assertion would block the generator
from emitting the vector. The normative claim is Core §9.5's last sentence
of that paragraph ("It is immutable under rotation because none of its
inputs is altered by service-side re-wraps").

**Why `LedgerServiceWrapEntry` does not appear in this vector.** §8.6's
`LedgerServiceWrapEntry` is the append-only record of a service-side re-wrap
of a payload DEK under a new LAK. Both Event A and Event B carry an empty
`KeyBag` (§9.4 admits `entries: []`) because both are structural-only
vectors; there is no HPKE-wrapped DEK to re-wrap. A production deployment
that rotated a LAK against events with populated `key_bag`s would emit one
`LedgerServiceWrapEntry` per re-wrapped event; a future vector
(`append/004-hpke-wrapped-inline` or a residue-batch vector) will exercise
that path once real wraps exist. 002 exercises the immutability half of
invariant #7 — the claim that the original `Event` bytes (including
`key_bag` and `author_event_hash`) are untouched by rotation — via the empty
`KeyBag` construction.

---

### Step 10: Event B `prev_hash` propagation

**Core § citation:** §10.2 `prev_hash` requirements.

**Load-bearing sentence:**

> "For `sequence == N > 0`: `prev_hash` MUST equal the `canonical_event_hash`
> (§9.2) of the event with `sequence == N-1` in the same ledger."

**Operation:** set Event B `prev_hash = canonical_event_hash(A) = a1baf9ac2b009be8d6c49c05bda2cf6cf6055b2900fd381c296299100ff12056`.

The rotation does **not** affect `prev_hash`: §10.2 pins the linkage by
`canonical_event_hash`, and `canonical_event_hash(A)` — the SHA-256 of the
§9.2 preimage over Event A's `EventPayload` — is determined entirely by
Event A's bytes and is thus unchanged.

---

### Step 11: Event B authored / canonical / signed surfaces (§6.8)

**Core § citations:** §6.8; §9.5; §7.4; §7.1; §6.1; §9.2; §10.6.

Event B's construction is procedurally identical to Event A's Steps 2–6
**plus** §10.2 `prev_hash` population (Step 10 above) and with the following
field-level deltas:

| Field | A | B |
|---|---|---|
| `sequence` | `0` | `1` |
| `prev_hash` | `null` | 32-byte `canonical_event_hash(A)` |
| `header.authored_at` | `1745100000` | `1745100120` |
| `idempotency_key` | `"idemp-append-002a"` | `"idemp-append-002b"` (§17.3 distinctness) |
| Protected-header `kid` | `kid(issuer-001) = af9dff52…8b1743` | `kid(issuer-002) = 3d05ee9c…712e24` |
| Signing key | `issuer-001.seed` | `issuer-002.seed` |

**Computed outputs.**

- **Authored preimage** (`input-author-event-hash-preimage.cbor`, 572 bytes,
  sha256 `10b0978a…4636cd7`). 33 bytes longer than A's 539-byte preimage —
  the exact size delta of swapping `prev_hash: null` (1 byte, `0xf6`) for
  `prev_hash: h'<32>'` (34 bytes, `0x5820 <32 bytes>`). `authored_at` and
  `idempotency_key` size-neutral.
- **Author-event preimage** (`author-event-preimage.bin`, 603 bytes) —
  `u32be(23) || "trellis-author-event-v1" || u32be(572) || <572 bytes>`.
- **`author_event_hash(B)`** (`author-event-hash.bin`, 32 bytes):

  ```
  9064382099e70ae4edd919b1ff0d01e0f9b53a27b9195e84dfc12639917e8af0
  ```

- **Canonical `EventPayload_B`** (`expected-event-payload.cbor`, 624 bytes,
  sha256 `e42c99c1…63f4e95a`).
- **Protected-header map** (`{alg: -8, kid: kid(issuer-002), suite_id: 1}`,
  inner 27 bytes = `a3012704503d05ee9ced8f29b60ef84b17d4712e243a00010000 01`).
- **`Sig_structure_B`** (`sig-structure.bin`, 669 bytes, sha256
  `9b0365f4…ece52831b77`). `0x84 0x6a "Signature1" 0x58 0x1b <27 bytes
  protected> 0x40 0x59 0x02 0x70 <624-byte EventPayload_B>`. The `0x59
  0x02 0x70` payload-length prefix is `624 = 0x0270`; exactly 33 bytes
  larger than A's `0x59 0x02 0x4f` (591), matching the preimage size delta
  of Step 2.
- **Ed25519 signature under issuer-002** (64 bytes):

  ```
  47aafc9f0fb298ea1efc6eac71655b3945bfb62a93e8fc9326eb4f2102bab546
  cb89efb6f26588250625c9f827eaf54d02d3f37e1a525e64612c91ad07551701
  ```

  Determined by RFC 8032 Ed25519 for fixed seed (`issuer-002.d`) + fixed
  message (`Sig_structure_B`).

- **Signed envelope** (`expected-event.cbor`, 725 bytes, sha256
  `008adf47…802d49cf6e`). COSE_Sign1 tag 18 (`0xd2`), 4-element array,
  protected bstr / empty unprotected map / payload bstr / signature bstr.

- **`canonical_event_hash(B)`** — SHA-256 of the domain-separated dCBOR of
  `CanonicalEventHashPreimage_B` (§9.2, §9.1), 32 bytes:

  ```
  1aa3dec81e044676aaa193fcb53b8fa83add0622711dee69977052995846fd1d
  ```

- **`AppendHead(B)`** (`expected-append-head.cbor`, 93 bytes):
  `{scope: "test-rotation-ledger", sequence: 1, canonical_event_hash:
  <32 bytes>}`.

---

## Footer — full hex dumps

Each block below is the byte-exact content of the named sibling file.

### `input-pre-rotation-author-event-hash-preimage.cbor` (539 bytes, sha256 `44967770dc6322da8dcbbcda8a0f6d9c12275da24781c80cc6803a7c9f077944`)

```
ac66686561646572a96a6576656e745f74797065581d782d7472656c6c69732d
746573742f617070656e642d6d696e696d616c6a657874656e73696f6e73f66b
617574686f7265645f61741a68041ce06b7769746e6573735f726566f66e636c
617373696669636174696f6e581b782d7472656c6c69732d746573742f756e63
6c61737369666965646e726574656e74696f6e5f74696572006e7461675f636f
6d6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74f6767375
626a6563745f7265665f636f6d6d69746d656e74f6676b65795f626167a16765
6e7472696573806776657273696f6e016873657175656e63650069707265765f
68617368f66a657874656e73696f6e73f66b63617573616c5f64657073f66b63
6f6d6d69746d656e7473f66b7061796c6f61645f726566a3656e6f6e63654c00
0000000000000000000000687265665f7479706566696e6c696e656a63697068
65727465787458405472656c6c69732066697874757265207061796c6f616420
2330303100000000000000000000000000000000000000000000000000000000
00000000000000006c636f6e74656e745f686173685820bcdced2dfaf5342cd2
baca0560a9d384473fd45e202555f0654cccaa32b4f8126c6c65646765725f73
636f706554746573742d726f746174696f6e2d6c65646765726f6964656d706f
74656e63795f6b6579516964656d702d617070656e642d30303261
```

Map header `0xac` (12 entries). The `0x51` before `"idemp-append-002a"` is
the bstr-17 length prefix; A's idempotency key is 17 bytes (one byte longer
than 001's 16-byte key), accounting for a +2-byte length-prefix-plus-byte
delta relative to 001's authored preimage structure.

### `input-pre-rotation-author-event-hash.bin` (32 bytes, sha256 `5092317be4379c39c38275c5ade9be0c4a82e390abdd97aad3801fb83248e086`)

```
1a27bd4e1c4da0845bcb23a9d167ce2034a42290f33945a7cc09a4bc485e7d51
```

This IS `author_event_hash(A)`. Its file-level sha256 (the second-order
integrity digest printed by `write_bytes`) is the shown `5092317b…86`; the
file's 32-byte content is the load-bearing value.

### `input-pre-rotation-event-payload.cbor` (591 bytes, sha256 `f667e8c8d9be9757725734d20c577c4b1fc5b0040a2122057e6d79ab20b30d4f`)

```
ad66686561646572a96a6576656e745f74797065581d782d7472656c6c69732d
746573742f617070656e642d6d696e696d616c6a657874656e73696f6e73f66b
617574686f7265645f61741a68041ce06b7769746e6573735f726566f66e636c
617373696669636174696f6e581b782d7472656c6c69732d746573742f756e63
6c61737369666965646e726574656e74696f6e5f74696572006e7461675f636f
6d6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74f6767375
626a6563745f7265665f636f6d6d69746d656e74f6676b65795f626167a16765
6e7472696573806776657273696f6e016873657175656e63650069707265765f
68617368f66a657874656e73696f6e73f66b63617573616c5f64657073f66b63
6f6d6d69746d656e7473f66b7061796c6f61645f726566a3656e6f6e63654c00
0000000000000000000000687265665f7479706566696e6c696e656a63697068
65727465787458405472656c6c69732066697874757265207061796c6f616420
2330303100000000000000000000000000000000000000000000000000000000
00000000000000006c636f6e74656e745f686173685820bcdced2dfaf5342cd2
baca0560a9d384473fd45e202555f0654cccaa32b4f8126c6c65646765725f73
636f706554746573742d726f746174696f6e2d6c65646765726f6964656d706f
74656e63795f6b6579516964656d702d617070656e642d303032617161757468
6f725f6576656e745f6861736858201a27bd4e1c4da0845bcb23a9d167ce2034
a42290f33945a7cc09a4bc485e7d51
```

Map header `0xad` (13 entries). Final 35 bytes `71 "author_event_hash" 5820
<32-byte digest>` carry Event A's `author_event_hash` value.

### `input-pre-rotation-event.cbor` (692 bytes, sha256 `a9adbdd6ba80ee3d4c1d266c867d1bb4989a62ae998392a900918579ab9b1122`)

```
d284581ba301270450af9dff525391faa75c8e8da4808b17433a0001000001a0
59024fad66686561646572a96a6576656e745f74797065581d782d7472656c6c
69732d746573742f617070656e642d6d696e696d616c6a657874656e73696f6e
73f66b617574686f7265645f61741a68041ce06b7769746e6573735f726566f6
6e636c617373696669636174696f6e581b782d7472656c6c69732d746573742f
756e636c61737369666965646e726574656e74696f6e5f74696572006e746167
5f636f6d6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74f6
767375626a6563745f7265665f636f6d6d69746d656e74f6676b65795f626167
a167656e7472696573806776657273696f6e016873657175656e636500697072
65765f68617368f66a657874656e73696f6e73f66b63617573616c5f64657073
f66b636f6d6d69746d656e7473f66b7061796c6f61645f726566a3656e6f6e63
654c000000000000000000000000687265665f7479706566696e6c696e656a63
69706865727465787458405472656c6c69732066697874757265207061796c6f
6164202330303100000000000000000000000000000000000000000000000000
00000000000000000000006c636f6e74656e745f686173685820bcdced2dfaf5
342cd2baca0560a9d384473fd45e202555f0654cccaa32b4f8126c6c65646765
725f73636f706554746573742d726f746174696f6e2d6c65646765726f696465
6d706f74656e63795f6b6579516964656d702d617070656e642d303032617161
7574686f725f6576656e745f6861736858201a27bd4e1c4da0845bcb23a9d167
ce2034a42290f33945a7cc09a4bc485e7d515840d5a82e22de425183064fd052
e37952d76592b83e04a8f21abe3bdfa1e0a15f568c6705e38bd97c0eaba525f2
af51ae39847441ca09c3e8913bba7061c56faa04
```

Structure: `0xd2` (tag 18); `0x84` (array of 4); `0x58 0x1b <27-byte
protected>`; `0xa0` (empty unprotected map); `0x59 0x02 0x4f <591-byte
payload>`; `0x58 0x40 <64-byte signature under issuer-001>`.

### `input-pre-rotation-append-head.cbor` (93 bytes, sha256 `306f4c16da698a255a7a314a2640f4624f821d7ebbbc3b1aae609a9d77b06b1b`)

```
a36573636f706554746573742d726f746174696f6e2d6c656467657268736571
75656e6365007463616e6f6e6963616c5f6576656e745f686173685820a1baf9
ac2b009be8d6c49c05bda2cf6cf6055b2900fd381c296299100ff12056
```

Structure: `0xa3` (map of 3); `"scope" : "test-rotation-ledger"`;
`"sequence" : 0`; `"canonical_event_hash" : <32 bytes>`.

### `input-signing-key-registry-before.cbor` (133 bytes, sha256 `da06d7e0a1f35358b7809cf3ceea94c16e07157ecec1d61c30b193ec47b38775`)

```
81a8636b696450af9dff525391faa75c8e8da4808b1743667075626b65795820
71de3b4e933aa718a6f5c45845ee83af8000a450f9572d4cf393e681c8144191
66737461747573006873756974655f6964016876616c69645f746ff66a737570
65727365646573f66a76616c69645f66726f6d1a68041ce06b61747465737461
74696f6ef6
```

Structure: `0x81` (array of 1) containing a `0xa8` (map of 8) — the single
`SigningKeyEntry` for issuer-001. dCBOR canonical map order is byte-wise lex
of the 8 text keys: `attestation`, `kid`, `pubkey`, `status`, `suite_id`,
`supersedes`, `valid_from`, `valid_to`.

| Field | Encoded | Decoded |
|---|---|---|
| `attestation` | `6b "attestation" f6` | null |
| `kid` | `63 "kid" 50 <16 bytes>` | `af9dff52…8b1743` |
| `pubkey` | `66 "pubkey" 58 20 <32 bytes>` | Ed25519 x for issuer-001 |
| `status` | `66 "status" 00` | 0 = Active |
| `suite_id` | `68 "suite_id" 01` | 1 |
| `supersedes` | `6a "supersedes" f6` | null |
| `valid_from` | `6a "valid_from" 1a 68041ce0` | 1745100000 |
| `valid_to` | `68 "valid_to" f6` | null |

### `input-signing-key-registry-after.cbor` (285 bytes, sha256 `2eae70978f40a8b14d6d10f29c8aa58b8beb8015222f5d73e8d2f5cb94d0c158`)

```
82a8636b696450af9dff525391faa75c8e8da4808b1743667075626b65795820
71de3b4e933aa718a6f5c45845ee83af8000a450f9572d4cf393e681c8144191
66737461747573026873756974655f6964016876616c69645f746f1a68041d1c
6a73757065727365646573f66a76616c69645f66726f6d1a68041ce06b617474
6573746174696f6ef6a8636b6964503d05ee9ced8f29b60ef84b17d4712e2466
7075626b657958205378d2ffa05793e8bd39f3c5fa7f49889e25ecf81450ec21
6d6e44d5274339e866737461747573006873756974655f6964016876616c6964
5f746ff66a7375706572736564657350af9dff525391faa75c8e8da4808b1743
6a76616c69645f66726f6d1a68041d1c6b6174746573746174696f6ef6
```

Structure: `0x82` (array of 2) — first entry is the rotated issuer-001
(`status: 02` Retired; `valid_to: 1a 68041d1c` = 1745100060), second is
issuer-002 (new kid `3d05ee9c…712e24`; `supersedes: 50 <16-byte
kid(issuer-001)>`; `valid_from: 1a 68041d1c` = 1745100060).

Byte-level diff vs `signing-key-registry-before.cbor`:

- Array prefix: `0x81` → `0x82` (two entries vs one).
- issuer-001 row: `status` uint value `00` → `02`; `valid_to` bstr-null
  `f6` → uint `1a 68041d1c`. Same kid, same pubkey, same suite_id, same
  supersedes=null, same valid_from.
- issuer-002 row: new 147-byte entry at the tail.

### `input-author-event-hash-preimage.cbor` (572 bytes, sha256 `10b0978aa2476ed3862739f05219f97628ec5090b2e4505dd4e27b5d54636cd7`)

```
ac66686561646572a96a6576656e745f74797065581d782d7472656c6c69732d
746573742f617070656e642d6d696e696d616c6a657874656e73696f6e73f66b
617574686f7265645f61741a68041d586b7769746e6573735f726566f66e636c
617373696669636174696f6e581b782d7472656c6c69732d746573742f756e63
6c61737369666965646e726574656e74696f6e5f74696572006e7461675f636f
6d6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74f6767375
626a6563745f7265665f636f6d6d69746d656e74f6676b65795f626167a16765
6e7472696573806776657273696f6e016873657175656e63650169707265765f
686173685820a1baf9ac2b009be8d6c49c05bda2cf6cf6055b2900fd381c2962
99100ff120566a657874656e73696f6e73f66b63617573616c5f64657073f66b
636f6d6d69746d656e7473f66b7061796c6f61645f726566a3656e6f6e63654c
000000000000000000000000687265665f7479706566696e6c696e656a636970
6865727465787458405472656c6c69732066697874757265207061796c6f6164
2023303031000000000000000000000000000000000000000000000000000000
0000000000000000006c636f6e74656e745f686173685820bcdced2dfaf5342c
d2baca0560a9d384473fd45e202555f0654cccaa32b4f8126c6c65646765725f
73636f706554746573742d726f746174696f6e2d6c65646765726f6964656d70
6f74656e63795f6b6579516964656d702d617070656e642d30303262
```

Byte-level diff vs `input-pre-rotation-author-event-hash-preimage.cbor`:

- `authored_at`: `1a 68041ce0` → `1a 68041d58` (1745100000 → 1745100120, +120).
- `sequence`: `00` → `01`.
- `prev_hash`: `f6` (null, 1 byte) → `5820 a1baf9ac…0ff12056` (bstr-32, 34
  bytes). +33 bytes size delta.
- `idempotency_key`: `…303261` (`"…002a"`, trailing `0x61`) → `…303262`
  (`"…002b"`, trailing `0x62`). Size-neutral — both keys are 17 bytes.

No other bytes differ. Map header byte `0xac` unchanged (12 entries).

### `author-event-preimage.bin` (603 bytes, sha256 `9064382099e70ae4edd919b1ff0d01e0f9b53a27b9195e84dfc12639917e8af0`)

```
000000177472656c6c69732d617574686f722d6576656e742d76310000023cac
66686561646572a96a6576656e745f74797065581d782d7472656c6c69732d74
6573742f617070656e642d6d696e696d616c6a657874656e73696f6e73f66b61
7574686f7265645f61741a68041d586b7769746e6573735f726566f66e636c61
7373696669636174696f6e581b782d7472656c6c69732d746573742f756e636c
61737369666965646e726574656e74696f6e5f74696572006e7461675f636f6d
6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74f676737562
6a6563745f7265665f636f6d6d69746d656e74f6676b65795f626167a167656e
7472696573806776657273696f6e016873657175656e63650169707265765f68
6173685820a1baf9ac2b009be8d6c49c05bda2cf6cf6055b2900fd381c296299
100ff120566a657874656e73696f6e73f66b63617573616c5f64657073f66b63
6f6d6d69746d656e7473f66b7061796c6f61645f726566a3656e6f6e63654c00
0000000000000000000000687265665f7479706566696e6c696e656a63697068
65727465787458405472656c6c69732066697874757265207061796c6f616420
2330303100000000000000000000000000000000000000000000000000000000
00000000000000006c636f6e74656e745f686173685820bcdced2dfaf5342cd2
baca0560a9d384473fd45e202555f0654cccaa32b4f8126c6c65646765725f73
636f706554746573742d726f746174696f6e2d6c65646765726f6964656d706f
74656e63795f6b6579516964656d702d617070656e642d30303262
```

Bytes 0..3 are `00 00 00 17` (u32be length 23); bytes 4..26 are
`"trellis-author-event-v1"`; bytes 27..30 are `00 00 02 3c` (u32be length
572); bytes 31..602 reproduce `input-author-event-hash-preimage.cbor`
verbatim.

### `author-event-hash.bin` (32 bytes, sha256 `b6cdeffdfef68146b030d67ac59ec01e10fa3a7c816e7d37832ecba92a0b44f9`)

```
9064382099e70ae4edd919b1ff0d01e0f9b53a27b9195e84dfc12639917e8af0
```

This IS `author_event_hash(B)`. The file's sha256 (`b6cdeffd…b44f9`) is a
second-order integrity check.

### `expected-event-payload.cbor` (624 bytes, sha256 `e42c99c1f0263de0c7cb9a2d04a9f7813d0bfd15e1b4b76d60d9cd3563f4e95a`)

```
ad66686561646572a96a6576656e745f74797065581d782d7472656c6c69732d
746573742f617070656e642d6d696e696d616c6a657874656e73696f6e73f66b
617574686f7265645f61741a68041d586b7769746e6573735f726566f66e636c
617373696669636174696f6e581b782d7472656c6c69732d746573742f756e63
6c61737369666965646e726574656e74696f6e5f74696572006e7461675f636f
6d6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74f6767375
626a6563745f7265665f636f6d6d69746d656e74f6676b65795f626167a16765
6e7472696573806776657273696f6e016873657175656e63650169707265765f
686173685820a1baf9ac2b009be8d6c49c05bda2cf6cf6055b2900fd381c2962
99100ff120566a657874656e73696f6e73f66b63617573616c5f64657073f66b
636f6d6d69746d656e7473f66b7061796c6f61645f726566a3656e6f6e63654c
000000000000000000000000687265665f7479706566696e6c696e656a636970
6865727465787458405472656c6c69732066697874757265207061796c6f6164
2023303031000000000000000000000000000000000000000000000000000000
0000000000000000006c636f6e74656e745f686173685820bcdced2dfaf5342c
d2baca0560a9d384473fd45e202555f0654cccaa32b4f8126c6c65646765725f
73636f706554746573742d726f746174696f6e2d6c65646765726f6964656d70
6f74656e63795f6b6579516964656d702d617070656e642d3030326271617574
686f725f6576656e745f6861736858209064382099e70ae4edd919b1ff0d01e0
f9b53a27b9195e84dfc12639917e8af0
```

Map header `0xad` (13 entries). Final 35 bytes carry
`"author_event_hash" : <author_event_hash(B)>`.

### `sig-structure.bin` (669 bytes, sha256 `9b0365f4a530d2d6e9db251ae495456e427a53e73c2918f3705e7ece52831b77`)

```
846a5369676e617475726531581ba3012704503d05ee9ced8f29b60ef84b17d4
712e243a000100000140590270ad66686561646572a96a6576656e745f747970
65581d782d7472656c6c69732d746573742f617070656e642d6d696e696d616c
6a657874656e73696f6e73f66b617574686f7265645f61741a68041d586b7769
746e6573735f726566f66e636c617373696669636174696f6e581b782d747265
6c6c69732d746573742f756e636c61737369666965646e726574656e74696f6e
5f74696572006e7461675f636f6d6d69746d656e74f6726f7574636f6d655f63
6f6d6d69746d656e74f6767375626a6563745f7265665f636f6d6d69746d656e
74f6676b65795f626167a167656e7472696573806776657273696f6e01687365
7175656e63650169707265765f686173685820a1baf9ac2b009be8d6c49c05bd
a2cf6cf6055b2900fd381c296299100ff120566a657874656e73696f6e73f66b
63617573616c5f64657073f66b636f6d6d69746d656e7473f66b7061796c6f61
645f726566a3656e6f6e63654c000000000000000000000000687265665f7479
706566696e6c696e656a6369706865727465787458405472656c6c6973206669
7874757265207061796c6f616420233030310000000000000000000000000000
000000000000000000000000000000000000000000006c636f6e74656e745f68
6173685820bcdced2dfaf5342cd2baca0560a9d384473fd45e202555f0654ccc
aa32b4f8126c6c65646765725f73636f706554746573742d726f746174696f6e
2d6c65646765726f6964656d706f74656e63795f6b6579516964656d702d6170
70656e642d3030326271617574686f725f6576656e745f686173685820906438
2099e70ae4edd919b1ff0d01e0f9b53a27b9195e84dfc12639917e8af0
```

RFC 9052 §4.4: `0x84 0x6a "Signature1" 0x58 0x1b <27-byte protected
carrying kid(issuer-002)> 0x40 (empty external_aad) 0x59 0x02 0x70
<624-byte EventPayload_B>`. The `0x3d05ee9c…712e24` kid bytes appear at
offset 8..23 in the inner protected header.

### `expected-event.cbor` (725 bytes, sha256 `008adf47b669a9a17ada93524846716f152dcb8e4300df4e333082802d49cf6e`)

```
d284581ba3012704503d05ee9ced8f29b60ef84b17d4712e243a0001000001a0
590270ad66686561646572a96a6576656e745f74797065581d782d7472656c6c
69732d746573742f617070656e642d6d696e696d616c6a657874656e73696f6e
73f66b617574686f7265645f61741a68041d586b7769746e6573735f726566f6
6e636c617373696669636174696f6e581b782d7472656c6c69732d746573742f
756e636c61737369666965646e726574656e74696f6e5f74696572006e746167
5f636f6d6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74f6
767375626a6563745f7265665f636f6d6d69746d656e74f6676b65795f626167
a167656e7472696573806776657273696f6e016873657175656e636501697072
65765f68617368 5820 a1baf9ac2b009be8d6c49c05bda2cf6cf6055b2900fd38
1c296299100ff120566a657874656e73696f6e73f66b63617573616c5f646570
73f66b636f6d6d69746d656e7473f66b7061796c6f61645f726566a3656e6f6e
63654c000000000000000000000000687265665f7479706566696e6c696e656a
6369706865727465787458405472656c6c69732066697874757265207061796c
6f61642023303031000000000000000000000000000000000000000000000000
0000000000000000000000006c636f6e74656e745f686173685820bcdced2dfa
f5342cd2baca0560a9d384473fd45e202555f0654cccaa32b4f8126c6c656467
65725f73636f706554746573742d726f746174696f6e2d6c65646765726f6964
656d706f74656e63795f6b6579516964656d702d617070656e642d3030326271
617574686f725f6576656e745f6861736858209064382099e70ae4edd919b1ff
0d01e0f9b53a27b9195e84dfc12639917e8af0 5840 47aafc9f0fb298ea1efc6e
ac71655b3945bfb62a93e8fc9326eb4f2102bab546cb89efb6f26588250625c9
f827eaf54d02d3f37e1a525e64612c91ad07551701
```

(The spacing around `5820` before the prev_hash bstr and `5840` before the
signature bstr is editorial — the bytes are contiguous in the file.)

Structure: `0xd2` (tag 18); `0x84` (array of 4); `0x58 0x1b <27-byte
protected carrying kid(issuer-002)>`; `0xa0` (empty unprotected map);
`0x59 0x02 0x70 <624-byte payload>`; `0x58 0x40 <64-byte signature under
issuer-002>`.

### `expected-append-head.cbor` (93 bytes, sha256 `1308a30f02b3e127ed51e9bec857d1eedfa16b6853a62ce4074ce5d3a7c92703`)

```
a36573636f706554746573742d726f746174696f6e2d6c656467657268736571
75656e6365017463616e6f6e6963616c5f6576656e745f6861736858201aa3de
c81e044676aaa193fcb53b8fa83add0622711dee69977052995846fd1d
```

Structure: `0xa3` (map of 3); `"scope" : "test-rotation-ledger"`;
`"sequence" : 1`; `"canonical_event_hash" : <canonical_event_hash(B)>`.

---

## Invariant → byte mapping

| Invariant / TR-CORE row | Where in 002's bytes |
|---|---|
| #7 key_bag / author_event_hash immutable under rotation (TR-CORE-038) | `input-pre-rotation-author-event-hash.bin` (32 bytes) is the SHA-256 of the domain-separated dCBOR of Event A's `AuthorEventHashPreimage`. The registry mutation from `input-signing-key-registry-before.cbor` to `input-signing-key-registry-after.cbor` changes zero bytes of Event A's preimage (§9.5 CDDL has no kid/signature field); re-running the §9.5 preimage construction after rotation yields byte-identical output. The generator asserts this in-script; Core §9.5's penultimate sentence ("It is immutable under rotation because none of its inputs is altered by service-side re-wraps") is the normative source. |
| #2 signature-suite migration obligation (TR-CORE-036, §7.3) | Both `input-pre-rotation-event.cbor` (kid = `af9dff52…8b1743`) and `expected-event.cbor` (kid = `3d05ee9c…712e24`) resolve their protected-header kids against `input-signing-key-registry-after.cbor`; `issuer-001`'s entry survives with `status = Retired` so Event A remains verifiable after rotation (§8.4 "Retired is terminal for signature issuance but not for verification of historical records"), and `issuer-002` is present as `Active` for Event B verification. |
| §8.2 supersession linkage | `input-signing-key-registry-after.cbor` entry 2 carries `supersedes = af9dff52…8b1743` — the kid of entry 1. The linkage establishes the transitive-resolution chain §8.5 names as required for "complete" snapshots. |
| §10.2 prev_hash linkage across rotation | `input-author-event-hash-preimage.cbor` and `expected-event-payload.cbor` both carry `prev_hash = a1baf9ac…0ff12056` — the 32-byte `canonical_event_hash(A)`. The linkage is preserved across the rotation boundary because §10.2 pins prev_hash to the canonical event hash (not the signing kid). |

## Core-gap notes

The load-bearing Core claim for this vector — §9.5's immutability sentence —
is unambiguous and reproduces byte-identically with the generator-side
assertion described in Step 9. No Core gap is claimed from 002.

Two observations worth logging for the residue batch (not gaps, just
context):

- **§8.4 Rotating state elision.** 002 rotates directly from `Active` to
  `Retired`, skipping the intermediate `Rotating` state. §8.4 admits both
  `Active → Rotating → Retired` and `Active → Revoked` / `Active → Rotating
  → Revoked` paths. A separate residue-batch vector that pins the
  `Rotating` state with both keys accepting signatures would exercise the
  "both keys accept signatures" prose in §8.4's first bullet; 002 does not
  — it focuses on the terminal `Retired` case where the historical signature
  verifiability claim is most load-bearing.
- **LedgerServiceWrapEntry path.** §8.6's `LedgerServiceWrapEntry` fires on
  LAK rotation against non-empty `KeyBag`s. 002's structural-only
  construction (empty `KeyBag` in both events) means no `LedgerServiceWrapEntry`
  is emitted. The full immutability claim — that a LAK re-wrap produces an
  append-only record *and* does not mutate the original `Event` — is
  exercised by a future vector once a real HPKE wrap exists to re-wrap.
  The second half of TR-CORE-038's prose ("re-wrapping MUST produce an
  append-only `LedgerServiceWrapEntry`") is therefore covered only
  negatively by 002 (by the absence of one).
