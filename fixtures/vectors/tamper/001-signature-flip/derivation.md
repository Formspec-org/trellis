# Derivation — `tamper/001-signature-flip`

## Header

**What this vector exercises.** This is the first tamper vector. It pins the tamper-op manifest shape (`[expected.report]` sub-table with `tamper_kind` + `failing_event_id`) that every subsequent tamper vector inherits. The tamper itself is the minimum-surface mutation a Core §7.4 signature profile can tolerate: a single-byte flip inside the 64-byte Ed25519 signature bstr of `append/005-prior-head-chain`'s byte-exact `expected-event.cbor`. No payload byte changes. No canonical_event_hash, no author_event_hash, no content_hash, no prev_hash, no kid, no protected header. Only the signature. The verification signal is that Core §19 step 4.b (per-event signature verify over `Sig_structure`, §7.4) fails while every other §19 check passes, and `integrity_verified` drops to `false` via the step-9 conjunction. The failure is *localizable* under §19's "Failure classes" prose — localized to this one event — not fatal, so `structure_verified` stays `true`.

**Scope of this vector.**

1. Verification-side only. No fact-producer surfaces.
2. No HPKE, no posture transitions, no checkpoint chain. Only §19 step 4.b.
3. No full §18 `ExportManifest`. The tamper surfaces at §19 step 4.b, which is reachable from just `(ledger, signing_key_registry)`. Authoring a signed ExportManifest would require computing digests over seven sibling archive members plus a COSE_Sign1 wrap plus a full §20 `PostureDeclaration` — none of that carries signal for a signature-flip tamper. A later tamper vector will exercise §19 step 1-3's manifest-binding path end-to-end; this vector defers it. This is a scope decision, not a gap claim against Core.

**Core § roadmap (in traversal order).**

1. §18.4 — the source event shape. Read `append/005/expected-event.cbor` as a COSE_Sign1 tag-18 envelope.
2. §7.4 — locate the signature bstr (4th element of the tag-18 4-array). Record the byte offset at which the final byte of the signature lives in the serialized envelope.
3. §7.1 — the signature is 64 bytes of Ed25519 per [RFC 8032]. The final byte is a byte of that Ed25519 output; flipping any single bit of an Ed25519 signature invalidates it under the RFC 8032 verify function with overwhelming probability (the scheme's forgery resistance).
4. §9.2 — the pre-tamper `canonical_event_hash` is unchanged by the tamper (the hash is over `dCBOR(CanonicalEventHashPreimage)`, whose `event_payload` field contains the payload bytes but not the signature bytes). This is why the flipped-signature event retains its pre-tamper identity as a "failing event id" — its own canonical hash still matches §19 step 4.e's recomputation, only the signature check at step 4.b fails.
5. §8.5 / §8.2 — build a minimal registry so §19 step 4.a kid resolution succeeds. Failure at step 4.a would abort with `report.unresolvable_manifest_kid` (a fatal failure per §19's "Failure classes" prose), which is NOT the signal this vector targets.
6. §19 — assemble the `[expected.report]` sub-table.

---

## Body

### Step 1: Read the source event

**Core § citation:** §18.4 `010-events.cbor`.

**Load-bearing sentence:**

> "A dCBOR array of `Event` COSE_Sign1 records in canonical order, starting at `sequence = 0` up to `sequence = tree_size - 1`."

**Inputs:** `../../append/005-prior-head-chain/expected-event.cbor` (724 bytes, sha256 `416d5e6190d0ec8ad791437f7e4bdb369f751b11dcb3597a5f2911421529aac9`). This file is the tag-18 COSE_Sign1 envelope built by `append/005`'s derivation Step 10; it is the byte-exact source of the tamper.

**Operation:** byte-copy into the working set. No decode is necessary for the tamper itself — the byte offset can be computed from the envelope's dCBOR structure at Step 2.

---

### Step 2: Locate the signature byte to flip

**Core § citation:** §7.4 COSE protected headers and Sig_structure; RFC 9052 §4.2 COSE_Sign1 structure.

**Load-bearing sentence (§7.4):**

> "Every Trellis COSE_Sign1 artifact is the CBOR tag-18 4-array `[protected, unprotected, payload, signature]` of [RFC 9052] §4.2, with the payload bstr carried at array position 3 (i.e., embedded)."

**Reading of §7.4 / RFC 9052 §4.2 used by this vector.** The 4-array serializes under dCBOR (Core §5.1) with no trailing bytes after the final element. The final element is the signature bstr. Therefore the last byte of the serialized envelope is the last byte of the signature bstr's *value* (not the bstr's length-prefix bytes, which precede the value). For a 64-byte signature encoded under dCBOR, the length prefix is `0x58 0x40` (major type 2, length 64) and sits at offset 658 in the 724-byte envelope; the signature value occupies bytes 660..723 inclusive. Byte 723 is the last byte of the signature.

**Operation:** compute `offset = len(source_bytes) - 1 = 723`.

**Result:**

| Item | Value |
|---|---|
| Source envelope length | 724 bytes |
| Byte offset of target | **723** (= `len - 1`) |
| Original byte value | **`0x0f`** |
| Mutated byte value | **`0x0e`** (= original XOR `0x01`) |

Flipping the low bit of the final byte was chosen over flipping a higher-order bit for three reasons: (a) the verification signal is identical — any single-bit flip of an Ed25519 signature invalidates it under RFC 8032 (§7.1), (b) the low-bit flip is trivially reproducible by hand from the hex dump of the source envelope, and (c) flipping the *last* byte keeps the byte offset invariant under any future Ed25519-length-preserving refactor of the envelope shape (so long as §7.4's "signature is the final element" invariant holds).

Cross-reference: derivation of the offset does not require CBOR parsing if the stranger-test reader notes that the source envelope ends with the 64-byte signature bstr. The `expected-event.cbor` hex dump in `append/005/derivation.md` Footer shows the tail as `…c7c7f90f`; byte 723 is the final `0x0f`.

---

### Step 3: Apply the byte flip

**Core § citation:** §7.1 Pinned Phase 1 suite; [RFC 8032] Ed25519 verify.

**Load-bearing sentence (§7.1):**

> "Phase 1 pins `suite_id = 1` to Ed25519-over-COSE_Sign1. Concretely: the signature is COSE_Sign1 ([RFC 9052]) with `alg = -8` (EdDSA) and the signing key a 32-byte Ed25519 public key ([RFC 8032])."

**Why a single-bit flip is sufficient.** RFC 8032 Ed25519 verify rejects any signature that does not satisfy the group-equation check over the given message and public key. Perturbing any single bit of a valid signature yields a signature that satisfies the check with probability negligibly distinct from 2^−253 (the order of the curve's torsion-free subgroup). No implementation that conforms to RFC 8032 §5.1.7 will accept the mutated signature.

**Operation:** `tampered_bytes = source_bytes[:723] || 0x0e || source_bytes[724:]` (the tail slice is empty). Length is preserved: 724 bytes in, 724 bytes out.

**Result (724 bytes, sha256 `b09659c5a09be8353f78689df3c6bf9f2d837ec95c5d9a0fe9bcfd8dec035182`):** byte-identical to `append/005/expected-event.cbor` except at offset 723. Committed as `input-tampered-event.cbor`.

---

### Step 4: Wrap the tampered event in a one-element ledger

**Core § citation:** §18.4 `010-events.cbor`.

**Operation:** dCBOR-encode a Python list containing the CBOR-decoded form of the tampered event. cbor2 preserves the tag-18 wrapping under re-serialization when the input round-trips cleanly. The result is an array-of-1 where element 0 is the 724-byte tag-18 envelope; total serialized length is 725 bytes (1-byte array header `0x81` + 724-byte envelope).

**Result (725 bytes, sha256 `429c954b4751cc6778ecd47ebfb0a68a438e8d8af6509e0816f6dd0fea5e4ef0`):** committed as `input-tampered-ledger.cbor`. The one-event shape is the smallest ledger that round-trips through §19 step 4's `for each Event in 010-events.cbor` loop.

---

### Step 5: Build the signing-key registry

**Core § citation:** §8.2 `SigningKeyEntry`; §8.5 Registry snapshot in every export.

**Load-bearing sentence (§8.5):**

> "A verifier encountering a `kid` that cannot be resolved against the embedded registry MUST reject the export."

**Operation:** construct a one-element dCBOR array whose sole entry is a `SigningKeyEntry` for the issuer-001 key used to sign the pre-tamper event. Field population:

| Field | Value | Source |
|---|---|---|
| `kid` | 16 bytes: `af9dff525391faa75c8e8da4808b1743` | §8.3 derived-kid construction over the issuer-001 pubkey with `suite_id = 1`; identical to the `kid` embedded in the tampered event's protected header (see `append/005/derivation.md` Step 8). |
| `pubkey` | 32 bytes: raw Ed25519 public key from `../../_keys/issuer-001.cose_key` COSE_Key label `-2` | §7.1 |
| `suite_id` | `1` | §7.2 |
| `status` | `0` (Active) | §8.4 |
| `valid_from` | `1745000000` | §8.2; narrative-only for this vector |
| `valid_to` | `null` | §8.2: null = currently-active |
| `supersedes` | `null` | §8.2; no predecessor |
| `attestation` | `null` | §8.2; optional |

**Result (133 bytes, sha256 `4f0efcbe40658fe661406d686007c3b8f1abf66132b3271f1e02799d72b41d08`):** committed as `input-signing-key-registry.cbor`.

This is the minimum registry that lets §19 step 4.a resolve the tampered event's `kid` to a pubkey. Step 4.a succeeds; step 4.b fails; that is the entire signal.

---

### Step 6: Pin the pre-tamper `failing_event_id`

**Core § citation:** §9.2 Canonical event hash; §19 Verification Algorithm step 4.e / 4.k.

**Load-bearing sentence (§9.2):**

> "`event_payload` is the decoded `EventPayload` carried as the COSE payload. It contains `author_event_hash`, but it contains no signature bytes."

**Why the canonical hash is unchanged by the signature tamper.** `CanonicalEventHashPreimage` carries the `EventPayload` map (Appendix A §28), which contains `author_event_hash`, `content_hash`, `prev_hash`, `sequence`, `ledger_scope`, `header`, `payload_ref`, `key_bag`, `idempotency_key`, `commitments`, `causal_deps`, `extensions`, and `version`. None of those fields carries the signature bytes. The signature is in the COSE_Sign1 envelope *outside* the payload, not in the payload. Therefore flipping a byte of the signature does not change the payload, does not change the preimage, and does not change `canonical_event_hash`.

**Value:** `3d3d5aeb5d4b8d972adbddfe0f339a94fffe01bf90ac1648be2eb98d4acc9f17` (= `append/005`'s `canonical_event_hash` per that vector's derivation.md Step 11).

**Operation:** pin this hex string as `[expected.report].failing_event_id`. §19's per-event `VerificationFailure` carries a `location` field; the runner convention is to populate `location` with the failing event's `canonical_event_hash` hex. The pre-tamper hash is the correct identity because §19 step 4.e *does* pass on the tampered bytes (the payload is unchanged, so the recomputed canonical hash still equals `payload.author_event_hash`'s implied preimage, and it equals the identity the verifier would naturally surface in the failure record).

---

### Step 7: Pin `[expected.report]`

**Core § citation:** §19 Verification Algorithm step 9 (boolean conjunctions); §19's "Failure classes" prose block.

**Load-bearing sentences (§19 step 9):**

> "`structure_verified` = manifest signature valid AND every COSE/CBOR/CDDL structure decoded and signed AND no unknown top-level Phase 1 fields were accepted."
>
> "`integrity_verified` = archive digests valid AND event hashes, prev_hash links, checkpoint roots, inclusion proofs, consistency proofs, and every available ciphertext hash valid AND report.omitted_payload_checks is empty AND no entry in report.posture_transitions has continuity_verified = false AND no entry in report.posture_transitions has attestations_verified = false."
>
> "`readability_verified` = every payload required by the export scope was decrypted and schema-validated under the bound registry and upstream Formspec/WOS semantics."

**Load-bearing sentences (§19 "Failure classes"):**

> "**Fatal failures.** The verifier MUST abort immediately and return a report with `structure_verified = false`. Fatal failures are: invalid manifest structure, manifest signature invalid, archive-member digest mismatch against the manifest, signing-key-registry resolution failure for any manifest or checkpoint `kid`, and checkpoint signature invalid."
>
> "**Localizable failures.** The verifier MUST continue, accumulate the failure in the report's per-artifact failure list, and report on a per-event or per-proof basis. Localizable failures are: individual event hash mismatches, individual event signature failures, individual payload integrity failures (including ciphertext-hash mismatches), individual inclusion-proof failures, and individual consistency-proof failures between non-head checkpoints."

**Assignment for this vector.**

| Field | Value | Reasoning |
|---|---|---|
| `structure_verified` | `true` | The tampered envelope decodes cleanly through §19 step 4.c. The signature-verify failure at step 4.b is *localizable* per the "Failure classes" prose — enumerated in `event_failures`, not treated as an abort path. §19 step 9's conjunction for `structure_verified` reads "every COSE/CBOR/CDDL structure decoded" — this vector's tampered event DOES decode (the bytes are still a valid tag-18 4-array of the right shape); only the signature-check conjunct in `integrity_verified` fails. |
| `integrity_verified` | `false` | §19 step 4.b's signature verify fails (§7.1). Step 4.k records it in `event_failures`. Step 9's AND-conjunction for `integrity_verified` includes "event hashes, prev_hash links, checkpoint roots, inclusion proofs, consistency proofs, and every available ciphertext hash valid" — per the "Failure classes" prose's enumeration of localizable failures, "individual event signature failures" are part of the integrity-verified conjunction. |
| `readability_verified` | `true` | §19 step 9's readability conjunct runs only over payloads "decrypted and schema-validated". The Phase 1 append/005 event carries a `PayloadInline` ciphertext opaque to decryption (no HPKE wrap, structural-only — see 005/derivation.md Step 7). No payload decryption is attempted, so readability has nothing to check. The `ciphertext` bytes themselves hash cleanly under §9.3 because the tamper was in the signature, not the payload. |
| `tamper_kind` | `"signature_invalid"` | The pinned tamper-kind enum value for this class. See "Tamper-kind enum" below. |
| `failing_event_id` | `3d3d5aeb5d4b8d972adbddfe0f339a94fffe01bf90ac1648be2eb98d4acc9f17` | Pre-tamper canonical_event_hash per Step 6. |

---

## Tamper-kind enum (full proposal)

This vector pins `tamper_kind = "signature_invalid"`. For every subsequent tamper vector to reach a consistent naming, the full enum is fixed here so later tamper authors cite one source instead of drifting names independently. The enum covers the thirteen tamper classes identified in the G-3 first-batch brainstorm §1 (source: `thoughts/specs/2026-04-18-trellis-g3-first-batch-brainstorm.md`) plus the expanded tamper TODO list and the O-5 posture-transition fixture names.

| Value | Mutation | Detected at Core §19 step | Notes |
|---|---|---|---|
| `signature_invalid` | COSE_Sign1 signature does not verify under the registered kid's pubkey | step 4.b | **This vector.** Minimum-surface mutation; single bit flip of a signature byte suffices. |
| `hash_mismatch` | `author_event_hash` or `canonical_event_hash` recomputation disagrees with the payload's recorded value | step 4.d / 4.e | Payload-field tamper that the author forgot to re-sign; §19's recomputation of the hash catches it. |
| `prev_hash_break` | Event's `prev_hash` does not equal the prior event's recomputed `canonical_event_hash` | step 4.h | Chain-linkage tamper; the chain no longer forms a strict linear order per §10.2. |
| `event_truncation` | A middle event of a chain is absent, and subsequent events' `prev_hash` values do not link to their recomputed predecessors | step 4.h | Distinguished from `prev_hash_break` by intent; structurally detected the same way. |
| `event_reorder` | Two adjacent events are swapped; the later event's `prev_hash` no longer matches the now-earlier event's canonical hash | step 4.h | Variant of `prev_hash_break` with the additional property that `sequence` monotonicity is also broken. |
| `missing_head` | A chain missing its final head (the `tree_size - 1` event) | step 5.c / step 7.b | Manifests as Merkle root mismatch at checkpoint verification and as inclusion-proof failure. |
| `malformed_cose` | COSE_Sign1 envelope is structurally invalid (wrong array length, wrong tag, malformed bstr length prefix, absent or nil payload) | step 4.c (and step 4.b if decode proceeds) | Fatal-classification candidate; §19's "Failure classes" prose names decode failures as aborts if they corrupt `structure_verified` globally. |
| `wrong_scope` | `EventPayload.ledger_scope` does not equal `manifest.scope` | step 4.f | Caught by §19 step 4.f's equality check; detected whether or not the signature is valid. |
| `registry_snapshot_swap` | A `RegistryBinding`'s `registry_digest` does not equal the SHA-256 of the corresponding `050-registries/<digest>.cbor` file | step 3.f | Fatal; aborts with `report.archive_integrity_failure`. |
| `checkpoint_divergence` | Two checkpoints at the same `tree_size` with inconsistent `tree_head_hash` values, or a `Checkpoint.prev_checkpoint_hash` mismatch | step 5.c / 5.d / 5.e | Localizable (per-checkpoint). |
| `state_continuity_mismatch` | A posture-transition event's `from_*` state does not match the most recent prior transition's `to_*` state (or the initial declaration) | step 6.b | Localizable; recorded in `report.posture_transitions[i].continuity_verified = false`. |
| `attestation_insufficient` | Posture-transition event fails the Companion §10 attestation-count check, or an attestation signature does not verify | step 6.d | Localizable. |
| `posture_declaration_digest_mismatch` | Posture-declaration document is present but its recomputed digest (under `trellis-posture-declaration-v1`) does not equal the event's `declaration_doc_digest` | step 6.c | Localizable; §19 step 6.c names this explicitly as tamper evidence (sets BOTH `declaration_resolved = false` AND `continuity_verified = false`). |

**Enum governance.** This enum lives in `derivation.md` of the first tamper vector and is cited by subsequent vectors. A pending orchestrator step (flagged at the bottom of this file) will canonicalize it into `thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md` as a design-doc amendment. Future tamper classes added in later batches MUST be registered here (or in the amended design doc once the orchestrator lands it) before a vector uses them.

---

## Footer — full hex dumps

### `input-tampered-event.cbor` (724 bytes, sha256 `b09659c5a09be8353f78689df3c6bf9f2d837ec95c5d9a0fe9bcfd8dec035182`)

Byte-identical to `append/005-prior-head-chain/expected-event.cbor` except at offset 723 (the final byte), which changes from `0x0f` to `0x0e`.

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
8668e74341cb73629dc4a15b0fd3a413787f11c7c7f90e
```

Final byte is `0x0e` (vs 005's `0x0f`). Every other byte is unchanged.

### `input-tampered-ledger.cbor` (725 bytes, sha256 `429c954b4751cc6778ecd47ebfb0a68a438e8d8af6509e0816f6dd0fea5e4ef0`)

One-element dCBOR array wrapping the tampered event: leading byte `0x81` (array of 1, major type 4) followed by the 724-byte envelope from `input-tampered-event.cbor` verbatim.

```
81d284581ba301270450af9dff525391faa75c8e8da4808b17433a0001000001
a059026fad66686561646572a96a6576656e745f74797065581d782d7472656c
6c69732d746573742f617070656e642d6d696e696d616c6a657874656e73696f
6e73f66b617574686f7265645f61741a680296416b7769746e6573735f726566
f66e636c617373696669636174696f6e581b782d7472656c6c69732d74657374
2f756e636c61737369666965646e726574656e74696f6e5f74696572006e7461
675f636f6d6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74
f6767375626a6563745f7265665f636f6d6d69746d656e74f6676b65795f6261
67a167656e7472696573806776657273696f6e016873657175656e6365016970
7265765f686173685820ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d
47bdd77aae068e724ddb6a657874656e73696f6e73f66b63617573616c5f6465
7073f66b636f6d6d69746d656e7473f66b7061796c6f61645f726566a3656e6f
6e63654c000000000000000000000000687265665f7479706566696e6c696e65
6a6369706865727465787458405472656c6c69732066697874757265207061
796c6f6164202330303100000000000000000000000000000000000000000000
00000000000000000000000000000000006c636f6e74656e745f686173685820
bcdced2dfaf5342cd2baca0560a9d384473fd45e202555f0654cccaa32b4f812
6c6c65646765725f73636f706554746573742d726573706f6e73652d6c656467
65726f6964656d706f74656e63795f6b6579506964656d702d617070656e642d
30303571617574686f725f6576656e745f686173685820f25c40a7fe073206b7
d2e6c6136a435e892640fc6ffc0ccdbdeccd42d6b7f26658402b014298d95d43
b7d3f03dab2af664733c7c8c7f6905874eed8890ff3fbfa37ccb7d50e126cfa9
01948668e74341cb73629dc4a15b0fd3a413787f11c7c7f90e
```

### `input-signing-key-registry.cbor` (133 bytes, sha256 `4f0efcbe40658fe661406d686007c3b8f1abf66132b3271f1e02799d72b41d08`)

One-element dCBOR array carrying one `SigningKeyEntry` (§8.2): leading byte `0x81` (array of 1), then the 8-key map encoding the entry's fields.

```
81a86363616e78206b6964506166306466663239316661613735636538656461
343830386231373433 ... <rendered for orientation only; the actual
bytes are the dCBOR encoding of the field table in Step 5>.
```

The authoritative bytes are on disk; this dump block is orientation only. The hex above is truncated because the tail carries the 32-byte pubkey bstr whose exact hex is traceable via the COSE_Key file at `../../_keys/issuer-001.cose_key` (label `-2`). A stranger-test reader computes the exact bytes by dCBOR-encoding the field table in Step 5.

---

## Invariant → byte mapping

| Invariant / capability | Where in this vector's bytes |
|---|---|
| §19 step 4.b per-event signature verify | `input-tampered-event.cbor` byte 723 flipped from `0x0f` to `0x0e`. All other bytes unchanged, including the kid, the protected header, the payload, and the preceding 63 signature bytes. |
| §19 step 4.a kid resolution succeeds | `input-signing-key-registry.cbor` contains a `SigningKeyEntry` whose `kid` equals the kid in the tampered event's protected header (`af9dff525391faa75c8e8da4808b1743`). |
| §19 step 4.d / 4.e hash recomputation passes | Payload bytes are unchanged; `canonical_event_hash` recomputation still equals `3d3d5aeb…4acc9f17` (the pre-tamper value from 005/Step 11). |
| §19 step 9 `integrity_verified = false` conjunction | Entered via step 4.k's accumulated event-failure entry for the failed signature check. |
| §19 step 9 `structure_verified = true` | The tampered envelope decodes cleanly (§19 step 4.c reads it as a well-formed 4-array with decodable payload); §19's "Failure classes" prose names signature failures as *localizable*, not fatal. |

## Core-gap notes

No Core gap is claimed against Core §19 from this vector. The verification algorithm's step-4.b signature-verify branch is unambiguous: the signature fails, step 4.k records the failure, step 9's conjunction flips `integrity_verified` to `false`. §19 step 9's conjunction for `integrity_verified` enumerates "event hashes, prev_hash links, checkpoint roots, inclusion proofs, consistency proofs, and every available ciphertext hash" but does not *literally* list "event signature valid" in the step-9 prose — the "Failure classes" prose block does list "individual event signature failures" as a localizable failure, and §19's intent is that the set of localizable failures all participate in the `integrity_verified` conjunction. A future editorial pass on §19 step 9 could make this more explicit without changing semantics; this is flagged as a candidate editorial clarification, not a gap. The Core-gaps thoughts doc carries the note append-only.

**Orchestrator return item: tamper-kind enum pinning.** The full thirteen-value `tamper_kind` enum proposed in this derivation's "Tamper-kind enum (full proposal)" section should be canonicalized into `thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md` as a design-doc amendment, so later tamper vectors cite one authoritative source. This vector does NOT edit that design doc.
