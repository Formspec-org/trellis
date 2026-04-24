# Decision memo — Core §9.4 HPKE-freshness for `append/004-hpke-wrapped-inline`

**Date:** 2026-04-19
**Author role:** Orchestrator decision, Stream / G-3 critical path
**Blocks:** authoring of `fixtures/vectors/append/004-hpke-wrapped-inline`
**Status:** Recommendation ready to adopt
**Precedent for amendment pattern:** T10 gaps addressed inline during G-3 scaffold (§7.4 / §9.1 / §6.1 amendments landed together with vectors — see `thoughts/specs/2026-04-18-trellis-g3-first-batch-brainstorm.md` §3)

---

## 1 — Ambiguity statement

### 1.1 The exact §9.4 wording

Core §9.4 (`specs/trellis-core.md`, lines 542–564) defines the key bag and Phase 1 HPKE wrap. The load-bearing sentences for this decision are:

- CDDL (line 553): `ephemeral_pubkey: bstr .size 32,       ; X25519 ephemeral public key, **unique per wrap**`
- Prose (line 562): "Every wrap MUST use a **fresh ephemeral X25519 keypair**, generated, used once, and destroyed. The `ephemeral_pubkey` is persisted in the envelope so the recipient can perform ECDH. **Reusing an ephemeral keypair across wraps is a non-conformance.**"

§8.6 (line 473) reinforces the same model for `LedgerServiceWrapEntry`: "fresh X25519 ephemeral public key."

### 1.2 Two latent ambiguities, stacked

**A. Scope of "wrap."** The text says "per wrap" and "across wraps," but the term *wrap* is not defined. Three defensible readings:

1. **Per-`KeyBagEntry`** — one ephemeral per `(event, recipient)` pair. The strictest reading, and the reading most consistent with RFC 9180 §5 ("single-shot encryption to a single recipient"). Under this reading an event with two recipients has two ephemerals; two events with the same single recipient have two distinct ephemerals.
2. **Per-event** — one ephemeral shared across all `KeyBagEntry` rows within a single event, then never reused on any later event. Uncommon but not excluded by the bare word "wrap."
3. **Per-chain / per-envelope-lifetime** — one ephemeral per ledger scope or per author session. Clearly the weakest reading, and clearly outside what the prose intends, but not textually ruled out by §9.4 alone.

The §8.6 phrasing ("fresh X25519 ephemeral public key" per `LedgerServiceWrapEntry`) tilts toward reading (1), but the term is not quoted consistently across §§7, 8, 9.

**B. Reconciling "used once, and destroyed" with byte-reproducible fixtures.** Core §4 (line 182) and §29 mandate that every fixture vector "MUST reproduce byte-for-byte on a second, independent implementation." Byte-reproducibility of an HPKE wrap requires pinning the ephemeral private key (otherwise the `wrapped_dek` bstr varies on every regeneration). A pinned keypair is reused on every regeneration of the fixture — which reads as the "reusing an ephemeral keypair across wraps is a non-conformance" clause, verbatim. §9.4 provides no carve-out for test vectors.

These two ambiguities combine multiplicatively: the fixture author must pick a scope (A) *and* justify pinning (B) in the derivation.md, with neither choice traceable to pinned Core prose.

### 1.3 Downstream matrix rows that depend on §9.4 semantics

`specs/trellis-requirements-matrix.md` row **TR-CORE-038** (matrix line 92) binds invariant #7 (`key_bag` / `author_event_hash` immutable under rotation) to `§9.4` + `§8.6` via a test-vector check. This row's byte-testability is exactly what `append/004` is being authored to satisfy. Whatever is picked here becomes the precedent cited by that test-vector row.

Operational rows TR-OP-010…014 (matrix lines 225–229) reference Companion §9.4, which is a different §9.4 (Custody-Model declaration rules). Those are unaffected.

---

## 2 — Option analysis

### 2.1 Option (a) — Amend Core §9.4 to pin normative HPKE-freshness behavior

**What it binds.**

- Defines "wrap" explicitly as a single `KeyBagEntry` construction (reading A.1).
- Pins the obligation at `(event, recipient)` scope: one ephemeral X25519 keypair per `KeyBagEntry`, unique across the full ledger scope's event sequence.
- Adds an explicit *test-vector carve-out*: pinned ephemeral keys are admissible in language-neutral test vectors under `fixtures/vectors/**` only, and only when the fixture's `manifest.toml` declares the pinned key as a test artifact. Production implementations MUST generate fresh ephemerals in-process and destroy them after single use.
- Bounds "destroyed" to private-key material — the `ephemeral_pubkey` is persisted in-envelope by construction and is not subject to the destruction clause.

**What it leaves open.**

- LAK-key freshness in §8.6 is already stated separately; this amendment touches §9.4 only, not §8.6. A parallel one-sentence §8.6 alignment may be needed but does not block `append/004` (which does not exercise `LedgerServiceWrapEntry`).
- "Unique across the ledger scope" is stated but not proven; a duplicate-detection lint is deferred to G-2 invariant audits.

**Costs.**

- Load-bearing spec change to a cryptographic section; requires cross-reference check against §§7, 8, and Companion (the string "§9.4" appears 11 times in the requirements matrix, all operational rows to the Companion's §9.4 — structurally disjoint, no collision).
- Bumps `trellis-core.md` and requires the §29 domain-tag registry (§9.8) to be re-checked for new tags (none introduced by this amendment — verified: the amendment is prose + CDDL comment tightening, not a new construction).
- Adds a normative sentence about fixtures to Core, which mildly blurs Core/fixture separation. Mitigant: the carve-out is *enabling*, not constraining — it names the already-mandatory byte-reproducibility requirement of §29 and reconciles it explicitly.

### 2.2 Option (b) — Relax in fixture documentation

**What it binds.**

- `append/004`'s derivation.md states explicitly: "this fixture pins the ephemeral X25519 keypair under `_keys/ephemeral-004-recipient-001.cose_key` to satisfy byte-reproducibility (§4, §29); in production, §9.4 line 562 forbids ephemeral reuse."
- Carves the fixture to a single `KeyBagEntry` so the per-wrap vs per-event ambiguity is literally untestable from this vector alone. One recipient, one event, one ephemeral — unambiguous under any of readings A.1/A.2/A.3.
- Adds one row to `fixtures/vectors/_pending-invariants.toml` declaring the "cross-wrap ephemeral-uniqueness" claim as G-2 audit-only (non-byte-testable), pending a Core clarification.

**What it leaves open.**

- Scope-of-"wrap" (A.1 vs A.2 vs A.3) remains unpinned in Core prose.
- Any future multi-recipient fixture (there is no immediate demand) will re-raise the question.
- The fixture-to-Core traceability fragment under TR-CORE-038 gains an asterisk: "the vector exercises `key_bag` wrap structure under a pinned-ephemeral simplification; rotation-immutability is the tested invariant, ephemeral-freshness is not."
- The fixture declares a convention Core never blesses — `append/004` becomes a precedent that implicitly extends §9.4. This is the "kick the can" trade-off: every subsequent author will need to rediscover this decision or find it in the derivation.md.

**Costs.**

- Very low cost today.
- Future Core §9.4 amendment is still likely; when it lands, `append/004`'s derivation.md must be revised to align, and the relaxation language retired. Net spec-amendment work is ≥ Option (a), just deferred.

---

## 3 — Recommendation

**Pick: Option (a) — Amend Core §9.4.**

**Rationale (one sentence):** The ambiguity has two independent axes (scope of "wrap" + fixture-reproducibility carve-out), both of which `append/004` will cite in prose regardless of option — so doing that work in Core prose costs the same as doing it in a fixture preamble, while producing normative text that every future HPKE-bearing vector can cite instead of re-derive; per the T10 / §7.4 / §9.1 / §6.1 precedent landed during G-3 scaffold, gaps surfaced by fixture authoring are resolved in Core, not around Core.

**Supporting reasons:**

1. **No legitimate users to protect.** The greenfield / no-backwards-compat principle applies: fixing now costs less than every future deferred fix.
2. **The matrix already leans on it.** TR-CORE-038's test-vector status is the byte-level check for invariant #7. Publishing `append/004` without a pinned §9.4 reading means TR-CORE-038 inherits the ambiguity, and the first G-4 Rust implementation will re-raise it as a blocker.
3. **RFC 9180 is already definitive on reading A.1.** The "wrap = per-recipient-single-shot" reading is the RFC's mainline SealBase call site. Codifying it in §9.4 is pinning what HPKE already says, not inventing a rule.
4. **The test-vector carve-out is independently needed.** Even if the scope ambiguity were not present, §9.4's "generated, used once, and destroyed" clause collides with §§4 / 29 byte-reproducibility. A carve-out must land somewhere; Core prose is the right home for a reconciliation clause, since it is a Core-vs-Core tension.
5. **Option (b)'s deferral grows landmines.** The brainstorm doc's own language ("004 must resolve it by a fixture-level convention *plus a Core prose clarification*") already acknowledges a Core amendment is coming; delaying it beyond `append/004` means the first Rust impl of `append/004` ships against a derivation.md that Core will later overrule.

---

## 4 — Proposed §9.4 amendment text (drop-in ready)

The amendment replaces the single prose paragraph at `specs/trellis-core.md` line 562. It does not change the CDDL (line 553's `unique per wrap` comment stands; its meaning is now pinned below). It does not introduce a new domain tag (§9.8 unaffected). It does not alter §8.6.

**Replace the current line-562 paragraph with:**

> Every `KeyBagEntry` (hereafter "wrap") MUST use a fresh X25519 ephemeral keypair, unique across every wrap in the containing ledger scope. For the avoidance of doubt: in an event with N recipients the `key_bag` contains N `KeyBagEntry` rows with N distinct `ephemeral_pubkey` values, generated from N distinct ephemeral private keys; no `ephemeral_pubkey` value produced by any author in any event in the same ledger scope may recur in any later event. The `ephemeral_pubkey` is persisted in the envelope so the recipient can perform ECDH; the corresponding ephemeral *private* key MUST be used exactly once and destroyed after the wrap is sealed. Reusing an ephemeral private key across wraps — within the same event, across events in the same ledger scope, or across ledger scopes — is a non-conformance.
>
> **Test-vector carve-out.** Language-neutral byte-level test vectors under `fixtures/vectors/**` MAY pin the ephemeral private key as a fixture artifact, because §4 / §29 require every fixture to reproduce byte-for-byte across independent implementations and that requirement would otherwise be unsatisfiable for HPKE wraps. A fixture that pins an ephemeral private key MUST: (a) commit the pinned private key under `fixtures/vectors/_keys/` with a filename that names the owning vector, (b) declare in its `manifest.toml` that the pinned key is a test artifact, and (c) state in its `derivation.md` that a production implementation MUST generate the ephemeral in-process and destroy it after single use. The carve-out applies to fixtures only; no production `Fact Producer`, `Canonical Append Service`, or `Verifier` may rely on it.

**Rationale fields the new text settles:**

- "Wrap" == single `KeyBagEntry` (reading A.1). Reading A.2 and A.3 are explicitly ruled out by the "N recipients → N distinct ephemerals" clarifying sentence.
- Scope of uniqueness is pinned to *the containing ledger scope* (not just the event), which is stronger than the prior text and is directly implementable as a dup-detection pass against the existing event log during any `append`.
- "Used once and destroyed" is tightened to apply to the *private key*, which removes any implication that `ephemeral_pubkey` (which §9.4 itself persists) must also be destroyed — it cannot be, because it is covered by `author_event_hash` per §9.5.
- The test-vector carve-out is procedurally constrained (the (a)/(b)/(c) clauses) so that fixtures cannot silently sneak production-relaxed behavior past Core.

---

## 5 — Implications for `append/004` fixture authoring

With Option (a) adopted:

1. **`_keys/` gains one artifact.** A pinned X25519 COSE_Key at `fixtures/vectors/_keys/ephemeral-004-recipient-001.cose_key`, named per the amendment's (a) clause. This is the first X25519 key in `_keys/`; the naming convention (`ephemeral-<vector>-<recipient-slug>.cose_key`) establishes the pattern for every future HPKE fixture.
2. **Single-recipient simplification survives.** The amendment does not force a multi-recipient `append/004`. A single-recipient fixture remains the cheapest derivation and is sufficient to exercise the per-wrap byte path. A future `append/004b-multi-recipient` vector (optional) would test the "N recipients → N distinct ephemerals" prose; it is not required for Phase 1 closure.
3. **`manifest.toml` adds one declaration row.** Per the amendment's (b) clause: `test_artifact_keys = ["_keys/ephemeral-004-recipient-001.cose_key"]` (or equivalent schema-aligned field — exact key name to be fixed during authoring).
4. **`derivation.md` gains one paragraph.** The production-vs-test distinction per the amendment's (c) clause; no novel prose beyond quoting the amendment.
5. **`_pending-invariants.toml` is unchanged.** The HPKE path becomes byte-testable now; no new pending row is needed. (Option (b) would have added one.)
6. **Downstream unblock.** `append/004` is ready to author as soon as the amendment lands. No other G-3 vector is blocked on §9.4. `append/002` rotation and `append/005` chain-linkage are orthogonal.
7. **Cross-spec sweep.** One pre-authoring check: grep the Companion + Agreement for `ephemeral`, `HPKE`, `§9.4` (Core) to confirm no cross-reference drift. (From the context read: matrix hits are all Companion-§9.4, structurally disjoint. Expected clean.)

---

## 6 — If Option (b) were adopted instead (for completeness only)

*Included to make the trade-off legible; NOT the recommendation.*

**Fixture-doc relaxation language** to place in `append/004/derivation.md` under a new `### Scope note — HPKE ephemeral freshness` section:

> This vector pins the X25519 ephemeral keypair at `../../_keys/ephemeral-004-recipient-001.cose_key` to satisfy the byte-reproducibility requirement of Core §4 / §29. Core §9.4 (line 562) states that "every wrap MUST use a fresh ephemeral X25519 keypair, generated, used once, and destroyed" and that "reusing an ephemeral keypair across wraps is a non-conformance." The present fixture reproduces the pinned `wrapped_dek` bytes on every regeneration; a production implementation MUST generate a fresh ephemeral private key per wrap and destroy it after use. The semantic gap between "fresh per wrap" and "pinned for byte-reproducibility" is a latent ambiguity in §9.4's current text; its resolution is deferred to a future Core amendment. This vector constrains itself to a single `KeyBagEntry` (one recipient, one event) so that the per-wrap-vs-per-event-vs-per-chain scope question is untestable from this vector alone; the bytes herein are consistent with any of the three readings.

**Scope carve-out for `_pending-invariants.toml`** (new row):

```toml
[[pending]]
invariant = "hpke-ephemeral-cross-wrap-uniqueness"
reason = "Core §9.4 line 562 forbids ephemeral reuse across wraps, but the scope of 'wrap' (per-KeyBagEntry vs per-event vs per-ledger-scope) is not pinned. append/004 uses one KeyBagEntry and cannot distinguish scopes. G-2 audit row only until Core §9.4 is amended."
kind = "non-byte-testable"
```

**Cost under (b):** `append/004` ships, but TR-CORE-038's byte-status gains an asterisk, the G-2 audit queue gains a row, and the first G-4 Rust implementation either adopts `append/004`'s implicit convention or re-raises the decision — in both cases at higher cost than landing the amendment now.

---

## 7 — Next steps

1. Adopt Option (a). Land the §9.4 amendment text from §4 above into `specs/trellis-core.md` in a Core-amendment commit, separate from the `append/004` fixture commit.
2. Run the cross-reference sweep per §5 step 7.
3. Author `append/004-hpke-wrapped-inline` per the per-wrap single-recipient shape; `_keys/ephemeral-004-recipient-001.cose_key` is the first X25519 fixture key.
4. Cross-link TR-CORE-038 in the requirements matrix to the updated §9.4 prose (no row edit needed; the `§9.4` reference already points there).
5. Unblock task #15 (Author vector: `append/004-hpke-wrapped-inline`).
