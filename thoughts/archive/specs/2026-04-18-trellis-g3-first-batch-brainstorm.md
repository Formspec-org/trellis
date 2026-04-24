# Trellis G-3 first-batch vector set — brainstorm

**Date:** 2026-04-18
**Scope:** pre-authoring design for the 5 vectors named in `TODO.md` → "First vector batch (G-3)": `append/002-rotation-signing-key`, `append/003-external-payload-ref`, `append/004-hpke-wrapped-inline`, `append/005-prior-head-chain`, and the first `tamper/` signature-flip.
**Does not cover:** authoring the vectors, or the `append/` residue batch (critical-path step 2) that picks up invariants this batch leaves on the table.
**Convention:** invariant numbers below are the Phase 1 envelope invariants as defined in `thoughts/product-vision.md` §"Phase 1 envelope invariants (non-negotiable)" lines 134–162. Byte-testable pending set per `fixtures/vectors/_pending-invariants.toml`: `{3, 6, 7, 8, 10, 11, 13, 14, 15}`.

## The central finding

`TODO.md` near-term list (lines 73–79) tentatively maps:

| Vector | TODO's invariant claim |
|---|---|
| 002-rotation-signing-key | #8 |
| 003-external-payload-ref | #6 |
| 004-hpke-wrapped-inline | "HPKE" (no invariant number) |
| 005-prior-head-chain | #7 |

But the **definitional** numbering (vision §"Phase 1 envelope invariants") says:

- #6 = registry-snapshot binding in the **export manifest** (content-addressed digest of domain registry).
- #7 = `key_bag` / `author_event_hash` immutable under rotation → `LedgerServiceWrapEntry` append-only.
- #8 = redaction-aware commitment **slots** reserved in the envelope header.

The TODO numbers do not match the definitions they purport to exercise. "002 exercises #8" collapses if 002 is a rotation vector — a rotation exercises #7 (key-bag/author-hash immutability), not #8 (reserved commitment slots). "005 exercises #7" collapses equivalently — a `prev_hash` chain vector exercises ordering (#5) and idempotency (#13), not rotation. "003 exercises #6" is the most load-bearing slip: #6 lives in the **export manifest**, not in an append; an external-payload-ref append exercises the §6.4 `PayloadExternal` variant and (through §9.3) the "hashes over ciphertext" discipline of #4. The TODO's own residue-batch bullet (line 95) uses the correct definitions — the near-term bullets were written before the definitions were cross-checked.

The orchestrator's anticipated "canary invariants may get reshuffled" is precisely this. The rest of this brainstorm is the shuffle.

## 1 — Invariant assignment per vector

### Option A: keep TODO's slot assignments, fix the invariant labels

Re-label each vector to the invariants it actually exercises; keep the `<op>/NNN-<slug>` identities the TODO named. Low churn; the on-disk identity is what matters long-term under the F6 "renumbering forbidden" rule.

### Option B: rename slugs to match what they actually exercise

E.g. `002-rotation-signing-key` → `002-key-bag-immutable-under-rotation`. Rejected: adds churn; "rotation-signing-key" is a fine short-hand for "exercises rotation" even if the load-bearing invariant is the immutability claim rather than the rotation act itself. F6 permits slug edits but discourages them.

### Option C: re-partition the batch around the invariants, not the procedures

E.g. fold "rotation" into the residue batch and promote a *pure* #13 idempotency vector into first-batch slot 002. Rejected: loses the TODO's staging rationale. 005 was chosen as the slot to unblock O-5 posture-transition fixtures (`TODO.md` Stream D blocked on "every transition needs a non-genesis chain"); moving head-chaining out of the first batch slows a parallel stream. The batch sequence is a dependency graph, not an invariant checklist.

**Pick Option A.** Final map (invariants to vectors), with the TR-CORE rows each vector should claim in `coverage.tr_core`:

| Vector | Invariants exercised | TR-CORE claims (net-new vs `_pending-invariants.toml`) | Why |
|---|---|---|---|
| **append/002-rotation-signing-key** | #7 (key-bag/author-hash immutable under rotation), #2 (suite identified), #3 (signing-key registry entries are `Active` / `Revoked`-capable) | TR-CORE-036 (§8 migration obligation, spec-cross-ref + test-vector), TR-CORE-038 (#7 immutability + LedgerServiceWrapEntry append-only), TR-CORE-050 or 053 (idempotency — if the vector uses an idempotency_key a verifier could check) | Rotation without re-wrap: `author_event_hash` of an event signed under key v1 MUST reproduce after the registry adds key v2. Structural-only wrap is fine; what the bytes prove is that rotation did not perturb the historical hash. A second event in the vector (or a sibling vector) must demonstrate the new-key signature validates under the rotated registry. |
| **append/003-external-payload-ref** | #4 (hashes over ciphertext, not plaintext) — but *externalised*, so the ciphertext-reference discipline is now observably byte-stable; #13 (idempotency, piggybacks); ancillary: #8 (commitment slots stay reserved even when payload is off-graph) | TR-CORE-031 (content_hash over ciphertext — already claimed by 001, but 003 strengthens the claim by showing content_hash does not depend on payload location), TR-CORE-071 (#8 — slots reserved when payload is external; CDDL `PayloadExternal` still carries the envelope header), TR-CORE-050/053 (idempotency) | `PayloadExternal` swaps in for `PayloadInline`; `content_hash` still runs over the named ciphertext bytes named by `payload_ref`, regardless of whether those bytes live inline or at a URL-addressed location. The vector proves that by recomputing `author_event_hash` byte-for-byte against a fixture that pins the external bytes. Registry-snapshot binding (#6) is **not** exercised — that is an export manifest concern and belongs to an `export/` vector. |
| **append/004-hpke-wrapped-inline** | #4 (hashes over ciphertext — now real ciphertext, not structural-only), #8 (reserved commitment slot `key_bag` carries a real `KeyBagEntry`); latent: #11 (plaintext-vs-committed header — exercised via the distinction between plaintext routing fields and the HPKE-wrapped payload key) | TR-CORE-031 (content_hash over real ciphertext), TR-CORE-071 (#8 — `key_bag` carries one entry with a real HPKE wrap; commitment slot populated, not empty), new TR-CORE row iff §9.4 ends up clarifying the `kem_id` / `kdf_id` / `aead_id` cross-check (see §3) | Real HPKE wrap with pinned X25519 ephemeral keypair under `_keys/`. The point of 004 is to close the scope hole in 001's derivation.md — 001 explicitly deferred HPKE as "can't reproduce without pinned ephemerals"; 004 pins them. Freshness obligation (§9.4) is relaxed by fixture convention: pinned ephemeral keys are admissible *in a test vector* provided the vector declares the relaxation and derivation.md cites §9.4 to say so. This is a fixture-authoring convention; it does not amend Core. |
| **append/005-prior-head-chain** | #5 (ordering model named — `prev_hash` is strict linear Phase 1), #13 (idempotency across retries), #10 (Phase-1 envelope = Phase-3 case-ledger event — this is the first vector where `sequence > 0`, so it is the first byte shape under which the strict-superset claim has non-trivial content) | TR-CORE-020 (#5 exactly-one canonical order), TR-CORE-023 (canonical order independent of wall-clock), TR-CORE-080 (#10 strict superset), TR-CORE-050 (#13 idempotency — if 005 is authored with an idempotency key) | Non-genesis append: `sequence = 1`, `prev_hash = canonical_event_hash` of the 001 event. This vector is also a **prerequisite for Stream D** (O-5 posture transitions) per `TODO.md` dispatch notes — every transition needs a non-genesis chain. |
| **tamper/001-signature-flip** (first tamper) | — (no new invariants; exercises the **verification** side of #1 / #2 — structure/integrity verification produces a negative result) | TR-CORE-060 (verifier role), TR-CORE-061 (verification independence), TR-CORE-067 (verifier capability matrix) — all three via the `integrity_verified = false` flip | First tamper establishes the tamper-op shape (`expected.report.integrity_verified = false`, `tamper_kind = "signature_invalid"`). Minimal surface: flip a single byte in the COSE_Sign1 signature bstr. No payload mutation. |

**What this batch does NOT cover (pushed to the `append/` residue batch, critical-path step 2):**

- **#3** (signing-key registry Active/Revoked lifecycle) — only fully exercised in an `export/` vector that bundles a registry snapshot. An `append/` vector can claim TR-CORE-037 only weakly (registry entries *referenced* by the signing `kid`). Residue batch should include a rotation-then-revoke chain.
- **#6** (registry-snapshot digest in manifest) — export concern. Residue batch or first `export/` vector.
- **#8 fully** — 004 populates one `KeyBagEntry`; a full exercise of "reserved slots" means a vector with multiple commitment types in the envelope header, per-field commitments represented, which Phase 1 defers implementation of. A structural-reservation vector in the residue batch suffices (empty-but-schema-valid commitment slots).
- **#11** (plaintext-vs-committed header policy) — hybrid (model-check + test-vector per Wave-2 allowlist). 004 touches it latently; a dedicated vector in the residue batch makes the plaintext/committed split explicit.
- **#14** (snapshots + watermarks) — projection concern; Stream B.
- **#15** (honest posture claims) — declaration-doc + hybrid; Stream C / E or Wave-2 O-5 fixtures.

**Invariants this batch closes byte-testably:** {#7, #8 (partial), #10, #13}. Residue batch picks up {#3, #6, #8 (full), #11 structural, and the non-byte-testable remnants through Wave 2 audit paths}.

## 2 — Dependency order within the batch

### Option A: author in numerical order (002 → 003 → 004 → 005 → tamper)

Natural-feeling; matches the slug numbering. **Rejected.** 005 requires 001's `canonical_event_hash` as its `prev_hash`; 005 is also the upstream dependency of Stream D. 004's HPKE wrap does not depend on rotation state. Numeric order buries the only chain dependency at the end.

### Option B: 005 first (to unblock Stream D), then 004, 003, 002, tamper

Optimises for parallel-stream unblock. **Rejected.** Authoring 005 before 002 means 005 must be re-authored if 002 surfaces a rotation-triggered Core amendment that changes how `signing-key-registry.cbor` is referenced from an event's `kid`. Stream D can start on 001 + a stub for 005; the cost of forcing 005 first is inheriting rework risk.

### Option C (recommended): 005 → 003 → 004 → 002 → tamper

Rationale:

1. **005 first.** Cheapest derivation — it is 001's derivation with three bytes changed (`sequence`, `prev_hash`, `idempotency_key`), plus one new Core surface (the `prev_hash` linkage rule of §10.2 exercised non-trivially). It produces the non-genesis chain that Stream D needs immediately. Risk: 005's final bytes will drift if 002 forces a registry-cross-reference change, but that is a two-byte edit, not a rewrite.
2. **003 next.** Exercises `PayloadExternal` — a Core §6.4 surface 001 and 005 do not touch. Structural scope similar to 001 (pinned payload bytes, no HPKE). Isolates the `PayloadExternal` construction before adding the HPKE dimension in 004.
3. **004.** HPKE wrap lands on a payload shape already exercised (inline, like 001) but now carries a real `KeyBagEntry`. Authoring 003 first de-risks the payload-location axis so 004 only introduces one new dimension (wrap) rather than two. Requires a second pinned COSE_Key under `_keys/` (X25519 ephemeral) — this is the first vector in the batch that adds to `_keys/`.
4. **002 last of the appends.** Rotation is the highest-risk derivation: it may surface Core prose gaps in §8 (migration obligation, key lifecycle) analogous to the T10 gaps in §7.4 / §9.1 / §6.1. Authoring 002 with 003/004/005 already green means the generator, manifest, and derivation conventions are settled; only the rotation-specific constructions are new.
5. **tamper-001 after 005.** Strictly, the first tamper could ride 001. But 005 is a richer base (non-genesis, `prev_hash` live) — a sig-flip on 005 proves the verifier rejects a chained event with a broken signature, which is a stronger verification-independence claim than flipping 001 (which only proves verifier rejects a bad genesis). Small cost; material evidence gain. Alternatively, cut tamper-001 from 001 for minimality and add a tamper-002 from 005 in the residue batch. Recommend **tamper from 005** in this first batch — one tamper, exercised against the richest base available.

**Serial order picked: 005 → 003 → 004 → 002 → tamper-001 (from 005).**

## 3 — Core-spec gaps likely to surface

Authoring each vector load-tests new Core prose. By the T10 precedent (§7.4 / §9.1 / §6.1 amendments landed inside the scaffold plan), expect 1–2 gaps per vector. Highest-probability sites:

- **§10.2 prev_hash linkage (005).** Core currently says `prev_hash = canonical_event_hash(N-1)` for `sequence > 0`. Unclear whether the linkage is verified against the *locally held* prior event or against an *externally supplied* prior head (`AppendHead`) bytes. 005 will force a choice: is `prev_hash` a claim by the author or a check by the service? The answer governs whether the tamper vector from 005 produces `integrity_verified = false` or a distinct `chain_break` tamper kind. **Most likely gap site.**
- **§9.4 HPKE freshness obligation (004).** Core §9.4 mandates a fresh X25519 ephemeral keypair *per recipient, per wrap*. A reproducible fixture must pin the ephemeral keypair. 001's derivation.md flagged this as deferred; 004 must resolve it by a fixture-level convention ("pinned ephemerals admissible in test vectors, not in production") plus a Core prose clarification naming the deferral. **Second most likely gap.**
- **§8 rotation + §9.5 author_event_hash binding (002).** Core §9.5 says `author_event_hash` binds `key_bag` at the moment of signing; §8 says rotation produces a new registry entry, not a re-wrap. The load-bearing claim (invariant #7) is that rotation does not invalidate a historical `author_event_hash`. 002 must prove this by recomputing `author_event_hash` byte-identically under a rotated registry. If Core's §8 prose does not explicitly state "the historical `author_event_hash` MUST reproduce byte-identically after registry mutation," 002's derivation.md will expose the gap. Expect a §8 clarification analogous to the §7.4 suite_id-label clarification.

Lower-probability but watchable:

- **§6.4 `PayloadExternal` integrity reference shape (003).** `PayloadExternal` carries a reference (URL + digest?) to the ciphertext. If Core does not pin the shape of that reference byte-exactly, 003's `PayloadExternal` bytes are indeterminate. Likely landmine.
- **§17.5 `IdempotencyKeyPayloadMismatch` structured error (any append with idempotency).** Core §17.5 defines the error name; unclear whether it defines a byte-level error artifact. 005 and 002 both carry idempotency keys and therefore ship under the retry contract of TR-CORE-050/053. If a verifier is expected to produce a byte-identical error artifact on mismatch, that artifact's shape needs pinning.
- **§9.1 length-prefix uniformity re-test (003 / 004).** T10 already clarified §9.1 for single-component hashes; 003 and 004 introduce multi-component hashes (external payload reference + nonce in 003; wrap components in 004). The T10 clarification may or may not generalise — worth re-reading §9.1 against 003's and 004's preimage constructions.

## 4 — Generator work required

`gen_append_001.py` is a single-file, self-contained script. Five more vectors at that style is ~5× 500-line scripts with duplicated KeyBag, COSE_Sign1, preimage, and dCBOR plumbing. Two extraction strategies:

### Option A: extract shared helpers into `_generator/_lib/` before authoring 005

Move pure building blocks (dCBOR wrapper, domain-separation preimage builder, COSE protected-header assembler, Sig_structure assembler, kid derivation) into importable modules. Each vector generator then imports them. **Risk:** violates the G-5 isolation boundary in spirit — if the generators look like a library, authors stop quoting Core and start quoting the library. The fixture-system design explicitly names this: "Spec-interpretive code ... is hand-written in the generator with inline Core § citations." A shared lib erases the citations. **Rejected at library level.**

### Option B: extract narrow byte-level utilities only, keep spec-interpretive logic inline

Acceptable extractions: dCBOR wrapper (trivial `cbor2.dumps(..., canonical=True)`), big-endian length-prefix builder, Ed25519 signing wrapper, HPKE Base-mode wrap wrapper (004's new dependency). **Not** extracted: preimage structure, domain-tag strings, CDDL field ordering, COSE label choices, any construction that cites a Core §. Those stay inline with `# §N.M` citations in each generator. This keeps every generator self-contained as a reading of Core, while avoiding trivially-redundant boilerplate. **Recommended.**

### Option C: no extraction — duplicate fully across five generators

Simplest; matches current state. **Rejected.** The `cbor2.dumps(..., canonical=True)` and the length-prefix helper appear in every preimage computation. Duplicating 5× increases the probability of an author accidentally writing a non-canonical `cbor2.dumps(..., canonical=False)` in one of five files and not catching it until a stranger-test mismatch. Narrow helpers exist to eliminate that class of error; that is the only reason to extract.

**Proposed pre-work before authoring 005:** add `fixtures/vectors/_generator/_lib/bytes.py` carrying: `dcbor(value) -> bytes`, `length_prefixed(u32, bytes) -> bytes`, `domain_separated_digest(tag: str, *components: bytes) -> bytes`. Each function is ≤10 LOC; each has a docstring citing the Core § it serves. Every generator imports from there; no other extraction lands before 005 authoring begins. **Allowed-import AST scan** in `check-specs.py` must be extended to permit imports from `_generator/_lib/` (currently it walks `_generator/**/*.py` uniformly per F5 — verify the scan recognises the subdir).

HPKE wrap (004) introduces `cryptography.hazmat.primitives.asymmetric.x25519` and `cryptography.hazmat.primitives.kdf.hkdf`. Both are stdlib + `cryptography` ∴ within the allowed-import fence. No new top-level dependency.

## 5 — First tamper vector shape

### Option A: tamper ships only an input + an expected.report, no regeneration

The tamper vector's directory holds: `input-signed-event.cbor` (bytes of `expected-event.cbor` from the base vector, with byte N flipped), `manifest.toml`, `derivation.md`. No `expected-event-payload.cbor` — that concept doesn't apply to a verify/tamper op. The manifest's `[expected.report]` carries `integrity_verified = false`, `tamper_kind = "signature_invalid"`, `failing_event_id` pointing back to the base vector's event id.

### Option B: tamper ships a full ledger (base events + tampered event)

Bundles the whole ledger state including the unmutated prior events. Matches `verify/` op shape. Closer to a real verifier's input. **Recommended for 005-based tamper** because 005 has a `prev_hash`, and the verifier must walk the chain to find the broken signature at `sequence = 1`. A lone tampered event has no chain context.

### Option C: tamper from 001 (minimal)

Genesis-only; no chain walk. Simpler derivation but weaker evidence. **Rejected** per §2's ordering argument.

**Pick Option B.** Tamper vector layout:

```
tamper/001-signature-flip/
├── input-ledger.cbor            # {events: [001-event, 005-event-with-flipped-sig], head}
├── manifest.toml                # op = "tamper"; [expected.report] integrity_verified=false
├── derivation.md                # 2 sections: (a) construction; (b) why-it-fails-verification
└── derivation-byte-diff.txt     # hex-diff vs base `005/expected-event.cbor` showing flipped byte
```

**Derivation for a tamper differs from a valid vector in kind:** a valid vector's derivation.md reproduces a Core construction. A tamper vector's derivation.md (a) reproduces the base construction, then (b) describes the mutation and explains via Core §7.4 / §7.1 why the RFC 8032 Ed25519 verification equation no longer holds after the flip. Both halves cite Core. Section (b) is the new template surface this first tamper establishes; subsequent tampers (truncation, reorder, missing head, registry swap) will each need their own (b) tied to a different Core § failure mode.

**Byte selected for flip:** the last byte of the 64-byte Ed25519 signature. Justification in derivation.md: flipping any bit in any of the 64 signature bytes causes the RFC 8032 §5.1.7 verification equation `[S]B = R + [k]A` to fail; the last byte is the simplest to name ("signature_bytes[63] ^= 0x01"). A reviewer can verify by hex-diffing `input-ledger.cbor` against the base `005/expected-event.cbor` and seeing exactly one byte changed.

**The tamper-op runner contract** (per fixture-system design §Conformance runner): the runner invokes the local `verify` API on `input-ledger.cbor`, compares the returned `VerificationReport.integrity_verified` against `expected.report.integrity_verified = false`, and checks `tamper_kind` and `failing_event_id`. No byte comparison of the report — these are small-data fields compared field-wise. That matches the design's "inline `[expected.report]`" decision for structured small-data outputs.

## 6 — Unknowns flagged for orchestrator

1. **Does the orchestrator want `tamper_kind` values pre-enumerated before 5 tampers get authored?** The fixture-system design names `tamper_kind = "signature_invalid"` as an illustrative value but does not enumerate the set. If multiple tampers land without a pinned enum, implementers will diverge on names (`"sig_invalid"` vs `"signature_invalid"` vs `"invalid_signature"`). Recommend: pin the enum in `specs/trellis-core.md` §17.5 (alongside the existing error-name registry) as part of the residue batch's expanded tamper suite. First tamper can proceed with `"signature_invalid"` as a provisional choice.

2. **Fixture-level HPKE freshness relaxation — Core amendment or fixture-doc convention?** 004 needs a pinned ephemeral X25519 keypair. Core §9.4 requires freshness. Two paths: (a) amend §9.4 to say "fresh per production wrap; test vectors MAY pin for reproducibility"; (b) leave §9.4 alone and declare the relaxation in the G-3 fixture-system design as a fixture-authoring convention. (a) is a load-bearing change to §9.4 prose; (b) preserves Core but risks the stranger reader inferring that Trellis allows pinned ephemerals in production. Recommend (a). Needs an orchestrator decision before 004 authoring.

3. **`prev_hash` linkage check locus** (see §3 above). Is 005's `prev_hash` checked against an `AppendHead` the service holds, or against the author's claimed prior bytes? This controls the tamper-001 report shape. Orchestrator should pick before 005 authoring; a residue tamper (`chain_break`) may be needed depending on the answer.

4. **Does the generator pre-work go in its own commit before 005?** Or bundled with 005's first authoring commit? Recommend separate commit — makes the shared-helper extraction reviewable independent of 005's per-vector derivation, and future vectors point to it as a reference.

5. **Stream D's `append/005` dependency:** Stream D blocked on 005 per `TODO.md` Dispatch notes. If Stream D authoring starts in parallel the moment 005 lands, any post-hoc edit to 005's bytes (triggered by a Core amendment from 002 or 004) breaks Stream D work. Two options: (a) freeze 005 bytes before starting 002/004 (risks rework if 002/004 force a prev-hash-shape amendment); (b) let Stream D consume a `status = "deprecated"`-able stub that is replaced once 002/004 are green. Recommend (a) — 005 is the simplest derivation and least likely to surface Core gaps; accept the small rework risk.
