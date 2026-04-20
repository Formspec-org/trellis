# Derivation — `tamper/005-chain-truncation`

## Header

**What this vector exercises.** Core §19 step 4.h — `prev_hash` linkage. The
tampered ledger is a one-element dCBOR array carrying only
`append/005-prior-head-chain`'s byte-exact COSE_Sign1 envelope (`sequence = 1`).
The genesis event (`append/001-minimal-inline-payload`, `sequence = 0`) has
been dropped. No event bytes are mutated: the survivor is signed cleanly
under issuer-001, its `author_event_hash` and `canonical_event_hash`
recomputations both pass, and its protected-header kid resolves against the
one-entry signing-key registry. The tamper is the *set* of events in the
ledger, not the contents of any event. Core §19 step 4.h is the failure
site because the survivor's `sequence = 1` forces the verifier to look up
`events[0]`, and `events[0]` is the survivor itself — whose
`canonical_event_hash` does NOT equal its own `prev_hash` (which still
references the now-omitted genesis event's canonical hash). Per Core §19's
"Failure classes" prose this is a *localizable* failure — step 4.k records
it in `event_failures`, step 9's `integrity_verified` conjunction drops to
false via "prev_hash links ... valid", and `structure_verified` stays true
because every remaining byte decodes cleanly through step 4.c.

**Scope.**

1. Verification-side only. No fact-producer surfaces. No new events are
   authored; all bytes are inherited from the pinned `append/001` /
   `append/005` chain.
2. No ExportManifest, no checkpoint, no HPKE wrap. The tamper surfaces at
   §19 step 4.h, reachable from just `(ledger, signing_key_registry)` —
   identical scope to `tamper/001`.
3. Dropping the genesis event (rather than a middle event of a longer
   chain) is deliberate: the pinned corpus has exactly two non-genesis /
   genesis events under the `test-response-ledger` scope, so genesis-drop
   is the only truncation shape authorable without adding new signed events
   to the corpus. The §19 step 4.h failure surface is identical regardless
   of which non-head event is removed.

**Core §19 roadmap (in traversal order).**

| §19 step | Outcome on this vector | Reason |
|---|---|---|
| 4.a (kid resolution) | passes | `input-signing-key-registry.cbor` carries the issuer-001 kid that appears in the survivor's protected header (`af9dff525391faa75c8e8da4808b1743`). |
| 4.b (signature verify) | passes | The survivor's bytes are byte-exact `append/005/expected-event.cbor`; its Ed25519 signature was produced by `gen_append_005.py` and remains valid. |
| 4.c (payload decode) | passes | Unchanged bytes round-trip through dCBOR decode. |
| 4.d (author_event_hash recomputation) | passes | Unchanged bytes; recomputation equals `payload.author_event_hash`. |
| 4.e (canonical_event_hash recomputation) | passes | Unchanged bytes; recomputation equals the survivor's pinned canonical hash `3d3d5aeb…9f17`. |
| 4.f (ledger_scope match) | not exercised | No ExportManifest in this vector; this is a runner-level no-op for tamper fixtures (same as tamper/001). |
| 4.g (payload_ref digest) | passes | The survivor's `PayloadInline.ciphertext` is unchanged; its `content_hash` recomputation matches. |
| 4.h (prev_hash linkage) | **FAILS** | `payload.sequence = 1`, so verifier checks `payload.prev_hash == canonical_event_hash(events[0])`. `events` has length 1; `events[0]` IS the survivor; the survivor's canonical hash (`3d3d5aeb…9f17`) ≠ the survivor's `prev_hash` (`ef2622f1…4ddb`). |
| 4.i (causal_deps) | passes | Survivor's `causal_deps = null` (inherited from append/005's Phase-1 shape). |
| 4.j (registry binding) | passes | No RegistryBinding in this vector; step is a no-op for tamper fixtures. |
| 4.k (record failure, continue) | records `event_failures[0]` | `VerificationFailure.location` populated with the survivor's `canonical_event_hash`. |
| 9 (step-9 conjunction) | `integrity_verified = false` | The "prev_hash links ... valid" conjunct is false; the remaining conjuncts would otherwise pass. |

---

## Body

### Step 1 — Read the pinned two-event chain

**Core § citation:** §10.1 (strict linear canonical order); §10.2
(`prev_hash` requirements); §18.4 (`010-events.cbor` shape).

**Load-bearing sentence (§10.2):**

> "For `sequence == N > 0`: `prev_hash` MUST equal the `canonical_event_hash`
> (§9.2) of the event with `sequence == N-1` in the same ledger."

**Inputs:**

| File | Size | SHA-256 | Role |
|---|---|---|---|
| `../../append/001-minimal-inline-payload/expected-event.cbor` | 691 B | `8d18bcd820945b4c5575a44823d79685858914ee5893ac3c9e4b8ec183273815` | Genesis (`sequence = 0`, `prev_hash = null`). **OMITTED** from the tampered ledger. |
| `../../append/005-prior-head-chain/expected-event.cbor` | 724 B | `416d5e6190d0ec8ad791437f7e4bdb369f751b11dcb3597a5f2911421529aac9` | Survivor (`sequence = 1`, `prev_hash = ef2622f1…4ddb`). Committed byte-exact. |

The two events form the entire `test-response-ledger` chain in the pinned
corpus. Genesis's `canonical_event_hash` is
`ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb`
(read from `append/001/expected-append-head.cbor`); survivor's
`canonical_event_hash` is
`3d3d5aeb5d4b8d972adbddfe0f339a94fffe01bf90ac1648be2eb98d4acc9f17`
(read from `append/005/expected-append-head.cbor`). Survivor's
`EventPayload.prev_hash` equals genesis's `canonical_event_hash` by
construction (see `append/005/derivation.md` — that is the defining
property of the 005 vector).

---

### Step 2 — Apply the truncation

**Core § citation:** §10.2 (chain-linkage invariant); §19 step 4.h
(verifier-side check).

**Operation:** omit the genesis event from the ledger array. The survivor's
bytes are not touched; only the *composition* of the ledger changes.

| | Pre-tamper ledger | Tampered ledger |
|---|---|---|
| Length | 2 events | 1 event |
| `events[0]` | genesis (`sequence = 0`) | survivor (`sequence = 1`) |
| `events[1]` | survivor (`sequence = 1`) | — |

Under the tampered ledger, §19 step 4.h's lookup `events[payload.sequence - 1] =
events[0]` resolves to the survivor itself. The survivor's recomputed
`canonical_event_hash` is `3d3d5aeb…9f17`; its `payload.prev_hash` is
`ef2622f1…4ddb`. The two are structurally different 32-byte values; step
4.h records `prev_hash_mismatch` per §19's enumerated failure codes (§19
prose defines `prev_hash_mismatch` as "`prev_hash` does not match the
predecessor's canonical event hash"; the spec does NOT require the
predecessor to exist, only that the recorded `prev_hash` agree with the
indexed lookup).

---

### Step 3 — Commit the survivor bytes

**Core § citation:** §18.4 `010-events.cbor`; §7.4 COSE_Sign1 envelope.

**Operation:** `input-tampered-event.cbor` = byte-exact copy of
`append/005/expected-event.cbor`.

**Result (724 bytes, sha256 `416d5e6190d0ec8ad791437f7e4bdb369f751b11dcb3597a5f2911421529aac9`):**
byte-identical to append/005's signed event. Convenience artifact for
runners that process a single event at a time without unwrapping a ledger
array.

---

### Step 4 — Wrap the survivor in a one-element ledger

**Core § citation:** §18.4 `010-events.cbor`.

**Load-bearing sentence (§18.4):**

> "A dCBOR array of `Event` COSE_Sign1 records in canonical order, starting
> at `sequence = 0` up to `sequence = tree_size - 1`."

**Reading used by this vector.** The §18.4 clause names a shape that the
tampered ledger violates: the tampered array does not start at
`sequence = 0`. §19's enumerated verification steps do NOT include an
explicit "the first event in `010-events.cbor` MUST have sequence = 0"
check distinct from step 4.h — §19's prev_hash linkage check at step 4.h
catches the same class of violation because a sequence-1 event placed at
index 0 has a `prev_hash` that does not match `canonical_event_hash(events[0])`
(= itself). A future §19 tightening could add a dedicated "array-index
monotonicity" step; under current §19 text, step 4.h is the normative
failure site. This vector does NOT claim a Core gap on that point (see
"Core-gap notes" at the Footer); it names the reading as the path §19
currently takes.

**Operation:** `input-tampered-ledger.cbor = dcbor([<tag-18 survivor>])`.
Leading byte `0x81` (CBOR major type 4, array length 1) followed by the
724-byte envelope verbatim.

**Result (725 bytes, sha256 `e8af09ae3a757bb25bb1bc6d6051c4bdab5d6dadbf5a4fd501dc230ebb20c8e5`):**
committed as `input-tampered-ledger.cbor`.

Cross-reference: this byte shape is byte-identical to `tamper/001`'s
`input-tampered-ledger.cbor` except at offset 724 (the final signature byte
that tamper/001 flipped from `0x0f` to `0x0e`). The two vectors demonstrate
the minimum tamper surface at two different §19 steps — tamper/001 at step
4.b (signature), tamper/005 at step 4.h (prev_hash linkage) — through the
nearly-identical byte shape.

---

### Step 5 — Build the signing-key registry

**Core § citation:** §8.2 `SigningKeyEntry`; §8.5 registry snapshot
requirement; §19 step 4.a kid resolution.

**Load-bearing sentence (§8.5):**

> "A verifier encountering a `kid` that cannot be resolved against the
> embedded registry MUST reject the export."

**Operation:** identical to `tamper/001` Step 5 — construct a one-entry
dCBOR array carrying the issuer-001 `SigningKeyEntry` so §19 step 4.a
resolves the survivor's kid. The registry bytes are byte-identical to
`tamper/001/input-signing-key-registry.cbor` by construction (same
derivation, same issuer key, same suite, same field values).

**Result (133 bytes, sha256 `4f0efcbe40658fe661406d686007c3b8f1abf66132b3271f1e02799d72b41d08`):**
committed as `input-signing-key-registry.cbor`.

---

### Step 6 — Pin the survivor's `failing_event_id`

**Core § citation:** §9.2 (canonical event hash); §19 step 4.k
(`VerificationFailure.location`).

**Why the survivor's own canonical hash is the correct anchor.** Step 4.h
fails *on the survivor event*. §19's per-event failure record
(`VerificationFailure`) carries a `location` field populated with the
failing event's `canonical_event_hash`. Since step 4.e passes for the
survivor (its bytes are unchanged and the recomputation matches), the
survivor has a well-defined canonical identity that the verifier surfaces
in the failure record.

Contrast with `tamper/001`: there the payload is unchanged, and the
"failing event" is still the survivor (the one with the flipped
signature), so `failing_event_id` again equals the survivor's
canonical_event_hash. Both tamper/001 and tamper/005 pin the *same* hash
value (`3d3d5aeb…9f17`) as `failing_event_id` because they both tamper
append/005 in different ways. This is coincidental — in tamper/001 the
payload is unchanged so the pre-tamper identity survives; in tamper/005
no bytes change at all.

**Value:** `3d3d5aeb5d4b8d972adbddfe0f339a94fffe01bf90ac1648be2eb98d4acc9f17`.

---

### Step 7 — Pin the dangling pointer (`omitted_event_id`)

**Core § citation:** §10.2 (`prev_hash` = predecessor's canonical hash);
§19 step 4.h (linkage check).

**Why surface a non-§19-named field.** §19's `VerificationReport` CDDL does
NOT prescribe a field for "the canonical hash the survivor pointed at". A
runner cross-checking the tamper can derive it from the tampered event's
payload (`payload.prev_hash`), but giving the expected value a named pin
in `[expected.report]` lets a stranger-test runner compare directly without
re-decoding the event. Precedent: `tamper/001` surfaces `failing_event_id`
as a non-§19-normative convenience pin.

**Value:** `ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb`
(= `append/001`'s `canonical_event_hash`).

---

### Step 8 — Pin `[expected.report]`

**Core § citation:** §19 step 9 (conjunctions); §19 "Failure classes" prose.

**Load-bearing sentences (§19 step 9):**

> "`integrity_verified` = archive digests valid AND event hashes, prev_hash
> links, checkpoint roots, inclusion proofs, consistency proofs, and every
> available ciphertext hash valid AND report.omitted_payload_checks is
> empty AND no entry in report.posture_transitions has continuity_verified
> = false AND no entry in report.posture_transitions has
> attestations_verified = false."

**Load-bearing sentences (§19 "Failure classes"):**

> "**Localizable failures.** The verifier MUST continue, accumulate the
> failure in the report's per-artifact failure list, and report on a
> per-event or per-proof basis. Localizable failures are: individual event
> hash mismatches, individual event signature failures, individual payload
> integrity failures (including ciphertext-hash mismatches), individual
> inclusion-proof failures, and individual consistency-proof failures
> between non-head checkpoints."

**Reading used by this vector.** "Localizable failures" enumerates "individual
event hash mismatches" — step 4.h's prev_hash-linkage check is a hash
comparison between `payload.prev_hash` and `canonical_event_hash(events[sequence-1])`.
The enumeration does not name "prev_hash mismatch" explicitly, but the
step-9 conjunction for `integrity_verified` names "prev_hash links ...
valid" as an explicit conjunct, and the "Failure classes" prose treats
individual per-event hash checks as localizable. A `prev_hash_mismatch`
here therefore fails `integrity_verified` without aborting
`structure_verified`. (An editorial clarification to §19's "Failure
classes" prose to spell "prev_hash mismatch" explicitly would be
desirable; see Core-gap notes.)

**Assignment for this vector.**

| Field | Value | Reasoning |
|---|---|---|
| `structure_verified` | `true` | The survivor's bytes decode cleanly through §19 step 4.c. Step 4.h is *localizable* per "Failure classes" prose — it does not abort `structure_verified`. |
| `integrity_verified` | `false` | §19 step 9 names "prev_hash links ... valid" as a conjunct; step 4.k's recorded failure violates it. |
| `readability_verified` | `true` | No payload decryption is attempted on the Phase 1 `PayloadInline` structural-only bytes (identical to tamper/001's reasoning; the survivor's `ciphertext` hashes cleanly under §9.3 because no bytes were mutated). |
| `tamper_kind` | `"event_truncation"` | Pinned row from `tamper/001`'s tamper-kind enum. "Middle event of a chain absent" generalizes to "non-head event absent"; genesis-drop is the minimum authorable case under the pinned two-event corpus. |
| `failing_event_id` | `3d3d5aeb…9f17` | Survivor's canonical_event_hash (Step 6). |
| `omitted_event_id` | `ef2622f1…4ddb` | Dangling pointer (Step 7). |

---

## Invariant → byte mapping

| Invariant / capability | Where in this vector's bytes |
|---|---|
| §19 step 4.h prev_hash linkage check | `input-tampered-ledger.cbor` contains exactly one event, `events[0]` = the survivor. Survivor's `payload.sequence = 1` (fourth field of its EventPayload map, reachable via the inner dCBOR payload decode) forces `events[0]` lookup; the lookup resolves to itself. |
| §19 step 4.a kid resolution succeeds | `input-signing-key-registry.cbor` carries a `SigningKeyEntry` whose `kid` equals the kid in the survivor's protected header (`af9dff525391faa75c8e8da4808b1743`). |
| §19 step 4.b signature verify succeeds | Survivor bytes are byte-exact `append/005/expected-event.cbor`; no signature byte is mutated (contrast tamper/001). |
| §19 step 4.e canonical_event_hash recomputation succeeds | Survivor payload bytes unchanged; recomputation = `3d3d5aeb…9f17`. |
| §19 step 9 `structure_verified = true` | Envelope decodes; step 4.h is *localizable*. |
| §19 step 9 `integrity_verified = false` | Entered via step 4.k's accumulated event-failure entry for the prev_hash mismatch. |
| TR-CORE-020 (inv #5) | The tampered ledger no longer exhibits a single canonical order — genesis's position is empty; the survivor's claim of `sequence = 1` is unanchored. The verifier detects this at §19 step 4.h. |

---

## Core-gap notes

No Core gap is claimed from this vector against §19 step 4.h. The step is
unambiguous: `payload.prev_hash` must equal
`canonical_event_hash(events[payload.sequence - 1])`, the tampered ledger
violates that equality, and step 4.k enumerates the failure. Two editorial
clarifications would sharpen §19 without changing semantics, both flagged
here as candidate editorial passes (not gaps):

1. §19's "Failure classes" prose enumerates "individual event hash
   mismatches" as localizable. Spelling this as "individual event hash
   mismatches *including prev_hash linkage*" would make the mapping from
   step 4.h failures to step 9's `integrity_verified` conjunction literal
   instead of inferred. tamper/001's derivation.md flagged the same
   editorial surface for "individual event signature failures"; both
   belong in one editorial pass.
2. §18.4's "starting at `sequence = 0` up to `sequence = tree_size - 1`"
   clause names a shape the verifier enforces at step 4.h but does not
   enforce explicitly at step 4's entry. If §19 added an "array-index
   monotonicity" check distinct from step 4.h, tamper/005 would fail at
   that earlier step instead; under current §19, step 4.h is the
   normative failure site. This is a scope decision inside §19, not a
   gap against the §18.4 clause.

---

## Footer — digest summary

| File | Bytes | SHA-256 |
|---|---|---|
| `input-tampered-event.cbor` | 724 | `416d5e6190d0ec8ad791437f7e4bdb369f751b11dcb3597a5f2911421529aac9` |
| `input-tampered-ledger.cbor` | 725 | `e8af09ae3a757bb25bb1bc6d6051c4bdab5d6dadbf5a4fd501dc230ebb20c8e5` |
| `input-signing-key-registry.cbor` | 133 | `4f0efcbe40658fe661406d686007c3b8f1abf66132b3271f1e02799d72b41d08` |

The `input-tampered-event.cbor` digest equals `append/005/expected-event.cbor`
(drift alarm in the generator asserts this). The
`input-signing-key-registry.cbor` digest equals
`tamper/001/input-signing-key-registry.cbor` (same issuer key, same suite,
same derivation). The `input-tampered-ledger.cbor` digest differs from
tamper/001's at exactly one byte — offset 724, the final signature byte
that tamper/001 flipped.
