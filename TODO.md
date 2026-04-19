# Trellis — TODO

Tactical work list. Concrete, near-term, actionable.

**This file is for:** current tactical state + "next thing we could pick up."
One-liners, each pointing to where the real context lives.

**This file is not for:** strategy (→ [`thoughts/product-vision.md`](thoughts/product-vision.md)),
ratification scope (→ [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md)),
or implementation plans (→ [`thoughts/specs/`](thoughts/specs/)).

When a TODO grows into a spec-sized effort, move its substance to
`thoughts/specs/…` and replace the entry here with a pointer.

Size tags: **XS** (≤1h) · **S** (≤1 session) · **M** (≤3 sessions) · **L** (multi-session).

---

## Current state (as of 2026-04-18, post-`c346313` unstaged)

- **Gates:** 14 closed (G-1/G-6, C-1..C-8, O-1/O-2, M-1..M-3); 7 open — see table below.
- **Lint:** green; `fixtures/vectors/_pending-invariants.toml` allowlist tracks 61 `TR-CORE-*`/`TR-OP-*` rows + 6 byte-testable invariants pending vector coverage. `TRELLIS_SKIP_COVERAGE=1` retired. Batched vector rollout drives the allowlist to zero.
- **End-state = Trellis Phase 1 stranger test passes** ([`thoughts/product-vision.md`](thoughts/product-vision.md) §"Phase 1 success criterion"): a stranger writes a second impl from Core + Companion + Agreement alone and byte-matches every vector. Closes when all 7 open gates close + Track A steps 6–9 done + Track E bindings landed. Phase 2–4 explicitly out of scope.

---

## Open ratification gates

Tracked in [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md).

| Gate | State | What closes it |
|------|-------|----------------|
| **G-2** Invariant coverage | partial | Byte-testable invariants audited via G-3 lint (`check_invariant_coverage`); non-byte-testable (model-check / declaration-doc-check / spec-cross-ref) still need a dedicated audit pass. |
| **G-3** Byte-exact vectors | partial | ~49 more vectors across `{append, verify, export, tamper}/`. First vector `append/001-minimal-inline-payload` committed. |
| **G-4** Rust reference impl | open | Cargo workspace + `append`/`verify`/`export` API + byte-match on all fixtures. |
| **G-5** Second implementation | open | Independent stranger-test impl (Python or Go) byte-matching every vector, written by someone who read only the specs. |
| **O-3** Projection discipline | open | Conformance fixtures for watermark, rebuild equivalence, snapshot cadence, purge-cascade verification. |
| **O-4** Delegated-compute honesty | open | Declaration documents per Companion §19 for every agent-in-the-loop deployment. |
| **O-5** Posture-transition audit | open | Canonical events recorded for custody-model / disclosure-profile changes per Companion §10. |

---

## Near-term — Sprint queue

### Lint / fixture infrastructure

- [ ] **Lint-enforce F6 deprecation fields** — **XS**.
      `check-specs.py` should validate `status` / `deprecated_at` manifest fields per
      the F6 amendment and exclude `status = "deprecated"` vectors from byte-testable
      coverage. Also: pre-merge guard on the renumbering-forbidden rule. Both flagged
      as Follow-ons in the fixture-system design.
- [ ] **Close review nits from 2026-04-18 patch set** — **XS**.
      Background review findings: F1 (lint silently skips lists in manifest values —
      fail-loud on unknown shape), F2 (absolute/empty-path bypass in
      `check_vector_manifest_paths`), F4 ("ratification branch" wording in F6 amendment —
      Trellis is single-branch on `main`), F5 (rename `pending_tr_core` →
      `pending_matrix_rows`; holds 20 `TR-OP-*` IDs), F3 (§7.4 tag-18 nit),
      F8 (missing tests: unknown-TR-row in allowlist, bad-type in `pending_invariants`).

### First vector batch (G-3)

Each batch is its own plan under `thoughts/specs/…`; brainstorm the set before starting.

- [ ] **`append/002-rotation-signing-key`** — invariant #8 (key rotation). **S**.
- [ ] **`append/003-external-payload-ref`** — invariant #6 (external payload via `PayloadExternal`). **S**.
- [ ] **`append/004-hpke-wrapped-inline`** — real HPKE wrap with pinned X25519 ephemeral
      keypair committed under `_keys/`. Task 10 deferred this per S4. **S**.
- [ ] **`append/005-prior-head-chain`** — non-genesis append, explicit `prev_hash` linkage, invariant #7. **S**.
- [ ] **First tamper vector** — signature-invalid flip in COSE_Sign1 signature bytes →
      verifier reports `integrity_verified=false`. Establishes the tamper-op shape. **S**.

---

## Parallelization plan — how we close Phase 1

The gate table above is the end-state coverage map. This section is the execution sequencing: what blocks what, what runs concurrently, where the merge points are. Each task was enumerated by gate in prior versions; here it is re-grouped by dependency stream.

**The frame:** the byte stack (G-3 → G-4 → G-5) is the critical path. Everything else (G-2 non-byte audit, O-3, O-4, O-5, Track E) is *off* the critical path and should run in parallel starting now, not after G-3. Waiting for G-3 before opening parallel tracks adds ~3–4 months to the total with no compensating signal benefit.

### Critical path (serial, gates the stranger test)

Each step's output feeds the next; cannot be parallelized within the path.

1. **First vector batch (G-3 start)** — `append/002..005` + first tamper. See "Near-term → First vector batch" above.
2. **`append/` residue batch (G-3)** — **M**.
      Close the 6 uncovered byte-testable invariants: #3 signing-key registry Active/Revoked lifecycle; #6 registry-snapshot binding (manifest digest of domain registry); #7 `key_bag` immutable under rotation (`LedgerServiceWrapEntry` append-only); #8 redaction-aware commitment slots reserved; #10 Phase 1 envelope = Phase 3 case-ledger event format (structural superset); #13 append idempotency retry semantics. First-batch 5 cover #6/#7/#8/#13 partially; this closes the residue. Target: `pending_invariants = []`.
3. **`verify/` suite (G-3)** — **M**.
      Happy-path (structure/integrity/readability all pass) + negative-non-tamper (expired key, suite-unsupported, missing registry snapshot). `verify/success/` vs `verify/negative/` split deferred per fixture-system design.
4. **`export/` suite (G-3)** — **M**.
      Deterministic ZIP layout, manifest shape, key-material handling, inclusion-proof shape. Per Core §18. Byte-exact ZIP is the acceptance gate.
5. **Expanded `tamper/` suite (G-3 close)** — **S** per case.
      Beyond first sig-flip: truncation, event reorder, missing head, malformed COSE, wrong-scope, stale `prev_hash`, registry-snapshot swap, checkpoint divergence.
6. **Rust workspace plan (G-4)** — **S** (plan only).
      Can start in parallel with step 2 once step 1 is done. Crate split: `trellis-core`, `trellis-cose`, `trellis-store-memory`, `trellis-store-postgres`, `trellis-verify`, `trellis-conformance`, `trellis-cli`.
7. **Rust workspace: first-vector byte-match (G-4)** — **L**.
      Build `append`/`verify`/`export` APIs; byte-match `append/001`. Can start once step 6 lands; does not need the full corpus.
8. **Rust workspace: full corpus match (G-4 closed)** — **L**.
      Byte-match every vector in `fixtures/vectors/`. Blocked by step 5.
9. **Commission stranger second impl (G-5)** — **L**.
      `trellis-py` or `trellis-go`. Implementor reads only Core + Companion + Agreement (never `_generator/`, never the Rust impl). Byte-matches every vector. Can begin reading specs mid-way through the corpus authoring (steps 2–5) but cannot finish until the corpus is frozen. Closes G-5.
10. **Ratification close-out** — **XS** (mechanical).
      Once all 7 gates flip to `[x]` and `python3 scripts/check-specs.py` reports zero violations, update `ratification/ratification-checklist.md` with final evidence SHAs, strike "(Draft)" from `specs/trellis-core.md` and `specs/trellis-operational-companion.md` titles, cut a version tag. Per `ratification/ratification-checklist.md` §"Natural stopping point."

### Parallel streams (start now — each closes an independent gate)

Off the critical path. Nothing here blocks G-3/G-4/G-5; nothing here is blocked by them. All five streams can open concurrently — ideal work for parallel agent dispatch on the design-brief phase.

Wave 1 design briefs all landed. Consolidated follow-up plan: [`thoughts/specs/2026-04-18-trellis-wave1-consolidation-plan.md`](thoughts/specs/2026-04-18-trellis-wave1-consolidation-plan.md). Remaining authoring tasks per stream below.

#### Stream A — G-2 non-byte-testable invariant audit

Design landed at [`thoughts/specs/2026-04-18-trellis-g2-invariant-audit-paths.md`](thoughts/specs/2026-04-18-trellis-g2-invariant-audit-paths.md). Hybrid classification; 11 byte-testable, 4 non-byte-only, 5 hybrid invariants. No remaining authoring — G-2 closes when (a) consolidation-plan lint rules land and (b) the audit-path table stays green.

#### Stream B — O-3 projection discipline

Design landed at [`thoughts/specs/2026-04-18-trellis-o3-projection-conformance.md`](thoughts/specs/2026-04-18-trellis-o3-projection-conformance.md). `fixtures/vectors/projection/` + `fixtures/vectors/shred/` new op-dirs.

- [ ] **Author O-3 fixtures** — **M**. 6–8 vectors covering watermark / rebuild / cadence / purge-cascade. Blocked by consolidation-plan Core §15.3 + Companion §16.2 + §20.5 edits.

#### Stream C — O-4 delegated-compute honesty

Design landed at [`thoughts/specs/2026-04-18-trellis-o4-declaration-doc-template.md`](thoughts/specs/2026-04-18-trellis-o4-declaration-doc-template.md). TOML-frontmatter-in-Markdown; 6-table schema.

- [ ] **Author reference declaration doc** — **S**. SSDI intake triage worked example. Blocked by consolidation-plan Appendix A housing decision.

#### Stream D — O-5 posture-transition audit

Design landed at [`thoughts/specs/2026-04-18-trellis-o5-posture-transition-schemas.md`](thoughts/specs/2026-04-18-trellis-o5-posture-transition-schemas.md). Two CDDL event families; co-publish declaration rule; 6 fixtures planned.

- [ ] **Author O-5 fixtures** — **S**. 3 append + 3 tamper under existing G-3 layout. Blocked by consolidation-plan Core §6.7 + §9.8 + §19 edits + `append/005-prior-head-chain` (critical-path step 2).

#### Stream E — Track E cross-cutting bindings

Not Phase 1 gates, but named in vision §"Next steps → Track E" as closing conditions for the three-tier claim. Core already reserves §22 (Composition with Respondent Ledger), §23 (Composition with WOS `custodyHook`), §24 (Agency Log extension points) as anchor sections.

- [ ] **WOS `custodyHook` ↔ Trellis binding** (vision item 22) — **S**.
      Flesh out Core §23 + Companion §24 (Workflow Governance Sidecar). Document how a WOS runtime uses Trellis as its custody backend without redefining either spec. Small because both seams are already named — this is text, not design.
- [ ] **Respondent Ledger ↔ Trellis binding** (vision item 21) — **M**.
      Three parts: (a) promote Formspec Respondent Ledger §6.2 `eventHash`/`priorEventHash` SHOULD → MUST when wrapped by a Trellis envelope; (b) define the **case ledger** as a top-level object composing sealed response-ledger heads with WOS governance events (Core §22); (c) define the **agency log** as the operator-maintained log of case-ledger heads (Core §24 reserves the extension points). Spec extension across Core §22 + new Core §24 content, not a nesting note. Requires Formspec spec edits — coordinate with the Formspec side before authoring.

### Dispatch notes

- **Wave 1 (done):** 4 parallel design-brief agents returned clean. Streams A/B/C/D briefs landed under `thoughts/specs/2026-04-18-trellis-{g2,o3,o4,o5}-*.md`. Consolidated follow-ups in `thoughts/specs/2026-04-18-trellis-wave1-consolidation-plan.md`.
- **Wave 2 (next):** execute the consolidation plan — Core + Companion + Matrix edits unblock O-3 / O-4 / O-5 fixture authoring; `check-specs.py` lint refactor bundles the ~10 rules surfaced across streams. Stream E WOS binding fits here too. First-batch G-3 vectors can run in parallel human sessions.
- **Wave 3:** G-4 Rust workspace execution. Not agent-friendly at L scale; sit with it. Vector corpus continues in parallel human sessions as Rust progresses.
- **Wave 4:** commission G-5 stranger test once corpus is frozen. Parallel streams should all have closed by this point or be in fixture-authoring tail.
- **Merge:** ratification close-out (step 10) is trivial once all streams merge back.

**Velocity estimate:** serial execution ≈ 7–9 months wall-clock. Parallelized per above ≈ 4–6 months, bounded by the critical path (G-3 corpus → G-4 full match → G-5 stranger). Parallel streams finish inside the G-3/G-4 window with weeks to spare.

---

## Parallel tracks (not blocked by Trellis ratification)

Tracks B (WOS runtime + Formspec coprocessor), C (FedRAMP / SOC 2 / GSA / WCAG certification clocks), and D (reviewer dashboard, document storage, webhooks, notifications) run independently of Track A. Detail in [`thoughts/product-vision.md`](thoughts/product-vision.md).

---

## Recently closed

Prune aggressively — `git log` is the real record.

- Wave 1 design briefs (G-2 / O-3 / O-4 / O-5) landed; consolidation plan at `thoughts/specs/2026-04-18-trellis-wave1-consolidation-plan.md` surfaces ~9 spec edits + ~10 lint rules across Core / Companion / Matrix.
- Core clarifications from T10 — §6.1 (`idempotency_key` uniqueness scope + UUIDv7 construction), §7.4 (COSE_Sign1 embedded payload, verifier MUST reject `payload == nil`), §9.1 (length-prefix form applies uniformly including single-component).
- Allowlist rollout — `TRELLIS_SKIP_COVERAGE=1` bypass removed; `_pending-invariants.toml` allowlist drives batched vector coverage (F5). `check_vector_manifest_paths` lint rule added (F7). 20/20 pytest green.
- Vector-lifecycle policy (F6) — renumbering-forbidden, `status = "deprecated"` tombstones, overlap-encouraged-as-boolean. Landed as F6 amendment in `thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md` under "Vector lifecycle" + "Manifest schema"; lint enforcement deferred to the separate `check-specs.py` follow-on plan.
- Matrix drift for Core §6.8 / §10.6 / §14.6 closed; `append/001` coverage updated (`475b064`, `a1eb41f`).
- Working norms encoded in the handoff prompt (`c346313`).
- Ratification-evidence removed; normalization handoff archived (`617f9ae`, `28f551c`).
- G-3 scaffold plan (12 tasks, `880ebdd..18c72c8`), Core amendments B1..S5 (`6ad24ab..e1895ae`), first reference vector (`e1ab065`).
