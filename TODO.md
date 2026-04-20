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

## Current state (as of 2026-04-19, HEAD = `6b20ef3`)

- **Gates:** 14 closed (G-1/G-6, C-1..C-8, O-1/O-2, M-1..M-3); 7 open — see table below. G-2 / O-3 / O-4 / O-5 all have normative spec anchors (Wave 2) + meaningful fixture coverage (Waves 4-6); closure blocked only on remaining fixture batches, lint commits 5-6, the G-2 audit sign-off, and a small batch of Wave-6-surfaced spec amendments (§9.4 HPKE freshness, staff-view extension registry, tamper_kind enum).
- **Lint:** green; 89/89 pytest. `fixtures/vectors/_pending-invariants.toml` allowlist: 2 byte-testable invariants (#3, #6) + 54 `TR-*` rows; `_pending-projection-drills.toml` 5 rows; `_pending-model-checks.toml` 8 rows awaiting G-4 evidence. Pre-merge vector-renumbering guard green (18 base prefixes preserved).
- **Fixture corpus:** 18 vectors across `append/{001,002,003,005,006,007,008}`, `projection/{001,002,003,004}`, `shred/{001,002}`, `tamper/{001,002,003,004,005}`. Reference `fixtures/declarations/ssdi-intake-triage/` O-4 artifact also landed.
- **End-state = Trellis Phase 1 stranger test passes** ([`thoughts/product-vision.md`](thoughts/product-vision.md) §"Phase 1 success criterion"): a stranger writes a second impl from Core + Companion + Agreement alone and byte-matches every vector. Closes when all 7 open gates close + Track A steps 6–9 done + Track E bindings landed. Phase 2–4 explicitly out of scope.
- **Review discipline:** Wave 2 + Wave 3C passed 4 interleaved opus-model `/semi-formal-code-review` cycles (Core / Companion / cross-spec / WOS binding); 25 findings fixed in-patch total.

---

## Open ratification gates

Tracked in [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md).

| Gate | State | What closes it |
|------|-------|----------------|
| **G-2** Invariant coverage | partial | Byte-testable invariants audited via G-3 lint (`check_invariant_coverage`). R6 spec-cross-ref + R7 projection-drill + R8 model-check lint rules landed (Wave 5). Remaining: R9/R10 (lint commit 5), R11 declaration-doc (lint commit 6), and G-4 evidence artifacts to flush `_pending-model-checks.toml`. |
| **G-3** Byte-exact vectors | partial | ~31 more vectors across `{append, verify, export, tamper}/`. 18 committed. Pending allowlist down to 2 invariants + 54 rows. |
| **G-4** Rust reference impl | open | Cargo workspace + `append`/`verify`/`export` API + byte-match on all fixtures. |
| **G-5** Second implementation | open | Independent stranger-test impl (Python or Go) byte-matching every vector, written by someone who read only the specs. |
| **O-3** Projection discipline | open | Conformance fixtures for watermark, rebuild equivalence, snapshot cadence, purge-cascade verification. |
| **O-4** Delegated-compute honesty | open | Declaration documents per Companion §19 for every agent-in-the-loop deployment. |
| **O-5** Posture-transition audit | open | Canonical events recorded for custody-model / disclosure-profile changes per Companion §10. |

---

## Near-term — Sprint queue

### Lint / fixture infrastructure

Wave 1 lint-refactor plan landed at [`thoughts/specs/2026-04-18-trellis-wave1-lint-extension-plan.md`](thoughts/specs/2026-04-18-trellis-wave1-lint-extension-plan.md): 6 S-sized commits total. Commits 1-2 landed (shared plumbing + R1 fixture-naming + R3 projection-drills loader). Remaining:

- [x] **Lint-refactor commit 3** — **S**. R4-R5: projection/shred op dispatch + `tr_op` / `companion_sections` coverage lint. Landed in Wave 5 working tree.
- [x] **Lint-refactor commit 4** — **S**. R6-R8: G-2 non-byte verification channels. R7 landed Wave 5 working tree; R6 spec-cross-ref (warning-not-error on uncited rows; non-resolving cites remain hard errors) + R8 model-check evidence (new `_pending-model-checks.toml` + `thoughts/model-checks/evidence.toml` path convention) landed in `d9f228a`.
- [ ] **Lint-refactor commit 5** — **S**. R9-R10: O-5 event-type registry check + CDDL cross-ref.
- [ ] **Lint-refactor commit 6** — **S**. R11: O-4 declaration-doc Phase 1 (6 static cross-checks; 7 ledger-replay checks deferred to G-4 Rust).
- [x] **Pre-merge renumbering guard** — **XS**. F6 amendment's complementary rule at merge time: `scripts/check-specs.py` enforces lifecycle fields, and `scripts/check-vector-renumbering.py` compares the current tree to a ratification/base ref to reject deleted or renumbered `<op>/NNN-*` vector prefixes. Landed in Wave 5 working tree with CLI/git-path tests.

### First vector batch (G-3) — per [`thoughts/specs/2026-04-18-trellis-g3-first-batch-brainstorm.md`](thoughts/specs/2026-04-18-trellis-g3-first-batch-brainstorm.md)

Brainstorm corrected TODO's prior invariant mislabels. Serial order: 005 → 003 → 004 → 002 → tamper/001 (from 005, not 001). Each vector its own plan under `thoughts/specs/…`.

- [x] **`append/005-prior-head-chain`** — invariants #5, #10, #13; TR-CORE-020/023/050/080. Landed (`060a547`).
- [x] **`append/003-external-payload-ref`** — invariants #4 + #8 partial + #13. `PayloadExternal` variant. Claims TR-CORE-031, -071. Landed (`905668b`). **S**.
- [ ] **`append/004-hpke-wrapped-inline`** — invariants #4 real + #8 populated + #11 latent. Real HPKE wrap with pinned X25519 ephemeral keypair under `_keys/`. **S**. Blocked on Core §9.4 amendment (decision memo landed `6b20ef3` at `thoughts/specs/2026-04-19-trellis-hpke-freshness-decision.md`; recommends Option (a) — pin per-`KeyBagEntry` freshness + fixture-reproducibility carve-out, drop-in paragraph included).
- [x] **`append/002-rotation-signing-key`** — invariant #7 (key-bag immutable under rotation; not "key rotation" writ large). Claims TR-CORE-036, -038. Landed (`4585646`, rebased onto main from worktree). **S**.
- [x] **`tamper/001-signature-flip`** (derived from `append/005`, not 001) — verification side; claims TR-CORE-061. Landed (`905668b`). **S**.

---

## Parallelization plan — how we close Phase 1

The gate table above is the end-state coverage map. This section is the execution sequencing: what blocks what, what runs concurrently, where the merge points are. Each task was enumerated by gate in prior versions; here it is re-grouped by dependency stream.

**The frame:** the byte stack (G-3 → G-4 → G-5) is the critical path. Everything else (G-2 non-byte audit, O-3, O-4, O-5, Track E) is *off* the critical path and should run in parallel starting now, not after G-3. Waiting for G-3 before opening parallel tracks adds ~3–4 months to the total with no compensating signal benefit.

### Critical path (serial, gates the stranger test)

Each step's output feeds the next; cannot be parallelized within the path.

1. **First vector batch (G-3 start)** — `append/002..005` + first tamper. See "Near-term → First vector batch" above.
2. **`append/` residue batch (G-3)** — **S** (reduced from M post-Wave-5).
      Close the 2 remaining uncovered byte-testable invariants per current `_pending-invariants.toml`: #3 signing-key registry Active/Revoked lifecycle; #6 registry-snapshot binding (manifest digest of domain registry). Wave 5 closed #7/#10/#13 fully and #8 to the lint's satisfaction via `append/002`/`003`/`005`. Target: `pending_invariants = []`.
3. **`verify/` suite (G-3)** — **M**.
      Happy-path (structure/integrity/readability all pass) + negative-non-tamper (expired key, suite-unsupported, missing registry snapshot). `verify/success/` vs `verify/negative/` split deferred per fixture-system design.
4. **`export/` suite (G-3)** — **M**.
      Deterministic ZIP layout, manifest shape, key-material handling, inclusion-proof shape. Per Core §18. Byte-exact ZIP is the acceptance gate.
5. **Expanded `tamper/` suite (G-3 close)** — **S** per case.
      Beyond first sig-flip: `tamper/005-chain-truncation` landed (`7fd5bed`, TR-CORE-020, Core §19 step 4.h); remaining cases: event reorder, missing head, malformed COSE, wrong-scope, stale `prev_hash` residuals, registry-snapshot swap, checkpoint divergence.
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

Wave 1 design briefs + Wave 2 spec edits + Wave 4 first fixtures all landed. Consolidated follow-up plan: [`thoughts/specs/2026-04-18-trellis-wave1-consolidation-plan.md`](thoughts/specs/2026-04-18-trellis-wave1-consolidation-plan.md). Per-stream state + remaining authoring below.

#### Stream A — G-2 non-byte-testable invariant audit

Design at [`thoughts/specs/2026-04-18-trellis-g2-invariant-audit-paths.md`](thoughts/specs/2026-04-18-trellis-g2-invariant-audit-paths.md). Hybrid classification; 11 byte-testable, 4 non-byte-only, 5 hybrid invariants. No remaining authoring — G-2 closes when (a) Wave 1 lint-refactor commits 4 (R6-R8) lands and (b) the audit-path table stays green.

#### Stream B — O-3 projection discipline

Design at [`thoughts/specs/2026-04-18-trellis-o3-projection-conformance.md`](thoughts/specs/2026-04-18-trellis-o3-projection-conformance.md). Spec anchors landed Wave 2; first 2 fixtures landed Wave 4 (`projection/001-watermark-attestation` Test 1, `shred/001-purge-cascade-minimal` Test 4).

- [ ] **Author remaining O-3 fixtures** — **S** (reduced from M). Priority order per Wave 4C handoff:
      (a) [x] `projection/003-cadence-positive-height` + `004-cadence-gap` (Test 3 / TR-OP-008) landed (`905668b`);
      (b) [x] `shred/002-backup-refusal` (Test 4 backup variant) landed (`334bb75`);
      (c) [ ] `projection/005-watermark-staff-view-decision-binding` (TR-OP-006 + §17.4 Staff-View) — last O-3 breadth item. **BLOCKED** on spec decision: Core §6.7 forbids Phase-1 `EventPayload.extensions` outside a registered set, and no registered identifier carries a `Watermark`. Authoring agent correctly halted. Fix queued: register `trellis.staff-view-decision-binding.v1` in Core §6.7 + CDDL pin to §15.2 `Watermark`. **S** amendment + **S** fixture (~1 session each).

- [x] **`projection/002-rebuild-equivalence-minimal`** — **S**. Test 2 / TR-OP-005; first fixture exercising Core §15.3's new dCBOR rebuild pin. Landed (`905668b`).

#### Stream C — O-4 delegated-compute honesty

Design at [`thoughts/specs/2026-04-18-trellis-o4-declaration-doc-template.md`](thoughts/specs/2026-04-18-trellis-o4-declaration-doc-template.md). Spec anchors landed Wave 2. SSDI intake triage reference declaration landed Wave 4 (`fixtures/declarations/ssdi-intake-triage/`). Three schema ambiguities flagged by authoring:

- [x] **Companion A.6 amendment to pin ambiguities** — **XS**. Pinned key-absence-as-null for TOML nullable fields, `[signature] = {cose_sign1_b64, signer_kid, alg}` shape, and optional `audit.registry_ref`. Landed in Wave 5 working tree.

#### Stream D — O-5 posture-transition audit

Design at [`thoughts/specs/2026-04-18-trellis-o5-posture-transition-schemas.md`](thoughts/specs/2026-04-18-trellis-o5-posture-transition-schemas.md). Spec anchors landed Wave 2. `append/005-prior-head-chain` landed Wave 4 — Stream D fixture authoring now fully unblocked.

- [x] **Author O-5 fixtures** — **S**. All 6 cases landed: `append/006` CM-B→CM-A (TR-OP-042/045), `append/007` CM-C narrowing, `append/008` disclosure-profile A→B (TR-OP-043/045 + invariant #11), `tamper/002` from-state mismatch (TR-OP-044), `tamper/003` missing dual-attestation, `tamper/004` declaration-digest mismatch. Commits `dbdfe0a` + `814b2fe`. Semi-formal review (Wave 6) returned REQUEST CHANGES — minor, single blocking finding.
- [ ] **Reconcile `tamper_kind` enum naming** — **XS**. Review finding: `tamper/001` canonical enum + design brief use `posture_declaration_digest_mismatch`; `tamper/004` manifest uses `declaration_digest_mismatch` (matches Core §19 step 6.c `failures[]` short form). Pick one and land a single edit.

#### Stream E — Track E cross-cutting bindings

Not Phase 1 gates, but named in vision §"Next steps → Track E" as closing conditions for the three-tier claim. Core already reserves §22 (Composition with Respondent Ledger), §23 (Composition with WOS `custodyHook`), §24 (Agency Log extension points) as anchor sections.

- [x] **WOS `custodyHook` ↔ Trellis binding** (vision item 22). Core §23 (4→8 subsections) + Companion §24 (OC-113a/b/c/d/e) + Appendix B.2 extensions landed Wave 3C + Wave 4E (10 opus-review findings applied). Committed as `248781f`.
- [ ] **Respondent Ledger ↔ Trellis binding** (vision item 21) — **M**.
      Three parts: (a) promote Formspec Respondent Ledger §6.2 `eventHash`/`priorEventHash` SHOULD → MUST when wrapped by a Trellis envelope; (b) define the **case ledger** as a top-level object composing sealed response-ledger heads with WOS governance events (Core §22); (c) define the **agency log** as the operator-maintained log of case-ledger heads (Core §24 reserves the extension points). Spec extension across Core §22 + new Core §24 content, not a nesting note. Requires Formspec spec edits — coordinate with the Formspec side before authoring.

### Dispatch notes

- **Wave 1 (done):** 4 parallel design-brief agents landed Streams A/B/C/D briefs; consolidated in `thoughts/specs/2026-04-18-trellis-wave1-consolidation-plan.md`.
- **Wave 2 (done):** spec-edit execution landed as 3 commits (Core/Companion/Matrix). 15 blockers+warnings closed across 3 interleaved opus-review cycles.
- **Wave 3 (done):** 5 parallel agents + 1 opus review. 3A brainstorm (first-batch vector invariants), 3B lint-refactor plan (6 S-commits phased), 3C WOS custodyHook binding (Core §23 + Companion §24, reviewed by opus, 10 findings flagged), 3D G-4 Rust workspace plan (10 crates in 5 layers), 3E lint code fixes (F1/F2/F5/F8 nits + F6 deprecation enforcement, 30→52 pytest).
- **Wave 4 (done):** 5 parallel agents. 4A `append/005-prior-head-chain` (closes #10/#13), 4B SSDI intake-triage reference declaration, 4C first O-3 projection + shred fixtures (+ lint manifest-skip extension), 4D Wave 1 lint refactor commits 1-2 (shared plumbing + R1/R3), 4E applied all 10 WOS review findings. 6 commits total.
- **Wave 5 (done, 9 commits `334bb75..552c142`):** 4 parallel code-scout agents (run under `isolation: "worktree"`) landed `shred/002-backup-refusal`, lint commit 4 (R6+R8), Stream D O-5 bundle (3 append + 3 tamper), and `append/002-rotation-signing-key` (worktree rebased onto main after Wave 5 working-tree advances). Working-tree queue committed in 4 follow-on commits: lint commit 3 (R4-R5/R7) + renumbering guard, Companion A.6 amendment, Wave 5 fixture batch (`append/003`, `projection/002-004`, `tamper/001`), TODO refresh. Allowlist: 2 invariants + 54 rows + 5 projection-drill + 8 model-check; pytest 74→89.
- **Wave 6 (in progress, 2 commits `7fd5bed..6b20ef3`):** 4 parallel agents dispatched in-tree (no worktrees — strict no-git + no-allowlist discipline held cleanly). Landed: `tamper/005-chain-truncation` (first expanded-tamper case, TR-CORE-020, step 4.h, `7fd5bed`); §9.4 HPKE-freshness decision memo recommending Option (a) amendment (`6b20ef3`). Surfaced 3 new spec-amendment tasks: `tamper_kind` enum reconcile (XS), register `trellis.staff-view-decision-binding.v1` Phase-1 extension (S), land Core §9.4 HPKE amendment (S). Stream D semi-formal review completed. `projection/005` authoring halted cleanly on spec blocker (as designed).
- **Wave 6 remaining tail:**
      (a) Land Core §9.4 amendment → unblocks `append/004`;
      (b) Land staff-view-decision-binding registry amendment → unblocks `projection/005`;
      (c) Reconcile `tamper_kind` enum;
      (d) Lint-refactor commits 5 (R9-R10) + 6 (R11);
      (e) G-3 `append/` residue batch (**S**, only #3/#6 left);
      (f) Stream E Respondent Ledger ↔ Trellis binding (needs Formspec-side coordination);
      (g) Review checkpoints per task list (first-batch / O-3 / lint-4 combined).
- **Wave 7:** G-4 Rust workspace execution per the plan. Not agent-friendly at L scale. Vector corpus (`verify/`, `export/`, expanded `tamper/`) continues in parallel sessions as Rust progresses.
- **Wave 8:** commission G-5 stranger test once corpus is frozen. Parallel streams should all have closed by this point or be in fixture-authoring tail.
- **Merge:** ratification close-out (step 10) is trivial once all streams merge back.

**Velocity estimate:** serial execution ≈ 7–9 months wall-clock. Parallelized per above ≈ 4–6 months, bounded by the critical path (G-3 corpus → G-4 full match → G-5 stranger). Parallel streams finish inside the G-3/G-4 window with weeks to spare.

---

## Parallel tracks (not blocked by Trellis ratification)

Tracks B (WOS runtime + Formspec coprocessor), C (FedRAMP / SOC 2 / GSA / WCAG certification clocks), and D (reviewer dashboard, document storage, webhooks, notifications) run independently of Track A. Detail in [`thoughts/product-vision.md`](thoughts/product-vision.md).

---

## Recently closed

Prune aggressively — `git log` is the real record.

- **Wave 6 (2 commits `7fd5bed..6b20ef3`):** 4 parallel in-tree agents (no worktrees). Landed `tamper/005-chain-truncation` (TR-CORE-020, Core §19 step 4.h pin; first expanded-tamper case, `7fd5bed`) and §9.4 HPKE-freshness decision memo recommending Option (a) amendment (`6b20ef3`, `thoughts/specs/2026-04-19-trellis-hpke-freshness-decision.md`). Stream D O-5 semi-formal review completed (verdict REQUEST CHANGES, 1 blocking-style finding + 1 nit + 6 notes; Finding 1 is the `tamper_kind` enum-name drift between `tamper/001` and `tamper/004`). `projection/005` authoring agent halted cleanly on a Phase-1 extension-registry spec blocker. 3 new spec-amendment tasks surfaced: `tamper_kind` reconcile, register `trellis.staff-view-decision-binding.v1`, land Core §9.4 amendment. 89/89 pytest; check-specs + renumbering guard green (18 base prefixes).
- **Wave 5 (9 commits `334bb75..552c142`):** 4 parallel code-scout agents + working-tree commit-split. Landed: `shred/002-backup-refusal` (`334bb75`), lint commit 4 R6+R8 including new `_pending-model-checks.toml` scaffolding (`d9f228a`), Stream D O-5 bundle `append/006..008` + `tamper/002..004` (`dbdfe0a` + `814b2fe`), `append/002-rotation-signing-key` rebased from worktree (`4585646`), lint commit 3 R4-R5/R7 fixture corpora + pre-merge renumbering guard (`0fcee6f`), Companion A.6 ambiguity amendment (`65090f8`), Wave 5 fixture batch `append/003` + `projection/002-004` + `tamper/001` (`905668b`), TODO refresh (`552c142`). Allowlist 2 invariants + 54 rows + 5 projection-drill + 8 model-check; 89/89 pytest; `check-specs.py` + `check-vector-renumbering.py --base-ref HEAD` green.
- **Wave 4 (6 commits `248781f..00042c4`):** `append/005-prior-head-chain` vector (closes #10/#13, TR-CORE-020/023/050/080); SSDI intake-triage reference O-4 declaration at `fixtures/declarations/`; first O-3 projection + shred fixtures (Test 1 watermark + Test 4 purge-cascade); Wave 1 lint refactor commits 1-2 (shared plumbing + R1 fixture-naming guard + R3 projection-drills loader, 30→52 pytest); 10 WOS-binding review findings applied.
- **Wave 3 (5 commits + 1 review):** `append/005` brainstorm (corrected TODO invariant mislabels; pinned serial order 005→003→004→002→tamper); Wave 1 lint-refactor plan (6 S-commits phased); WOS custodyHook ↔ Trellis binding (Core §23 4→8 subsections + Companion §24 OC-113a/b/c/d/e + Appendix B.2 extensions); G-4 Rust workspace plan (10 crates, 5 layers, M1 six sub-milestones); F6 deprecation-field lint + F1/F2/F5/F8 review-nits cleanup.
- **Wave 2 spec edits (`cfd587b..1233e02`):** Core §§6.5/6.7/9.8/15.2/15.3/19 (Posture-transition registry, `trellis-posture-declaration-v1` + `trellis-transition-attestation-v1` domain tags, `projection_schema_id` reconciliation, dCBOR rebuild encoding, verification algorithm step 5.5 + `PostureTransitionOutcome`); Companion §§10.3/16.2/19.9/20.5 + Appendix A.5 (shared `Attestation` rule + A.5.1/A.5.2/A.5.3) + A.6 (Delegated-Compute Declaration + OC-70a mandates) + A.7 (Cascade-scope enum); Matrix TR-OP-008/042..045 + TR-OP-005/006 flipped; allowlist promotes #11/#14/#15 to hybrid. Validated through 3 opus-model `/semi-formal-code-review` cycles; 15 blockers+warnings closed in-patch.
- Wave 1 design briefs (G-2 / O-3 / O-4 / O-5) landed; consolidation plan at `thoughts/specs/2026-04-18-trellis-wave1-consolidation-plan.md` surfaces ~9 spec edits + ~10 lint rules across Core / Companion / Matrix.
- Core clarifications from T10 — §6.1 (`idempotency_key` uniqueness scope + UUIDv7 construction), §7.4 (COSE_Sign1 embedded payload, verifier MUST reject `payload == nil`), §9.1 (length-prefix form applies uniformly including single-component).
- Allowlist rollout — `TRELLIS_SKIP_COVERAGE=1` bypass removed; `_pending-invariants.toml` allowlist drives batched vector coverage (F5). `check_vector_manifest_paths` lint rule added (F7). 20/20 pytest green.
- Vector-lifecycle policy (F6) — renumbering-forbidden, `status = "deprecated"` tombstones, overlap-encouraged-as-boolean. Landed as F6 amendment in `thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md` under "Vector lifecycle" + "Manifest schema"; lint enforcement deferred to the separate `check-specs.py` follow-on plan.
- Matrix drift for Core §6.8 / §10.6 / §14.6 closed; `append/001` coverage updated (`475b064`, `a1eb41f`).
- Working norms encoded in the handoff prompt (`c346313`).
- Ratification-evidence removed; normalization handoff archived (`617f9ae`, `28f551c`).
- G-3 scaffold plan (12 tasks, `880ebdd..18c72c8`), Core amendments B1..S5 (`6ad24ab..e1895ae`), first reference vector (`e1ab065`).
