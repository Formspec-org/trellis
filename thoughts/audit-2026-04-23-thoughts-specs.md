---
date: 2026-04-23
scope: thoughts/specs/
source_count: 9
archive_candidates: 8
skill: squashing-specs
investigator: spec-investigator
batches: 2
batch_size: 5
---

# Audit — `thoughts/specs/` — 2026-04-23

Walked 9 documents newest-to-oldest in 2 batches (5 + 4) via the `spec-investigator` agent. No inter-document supersessions were declared by any file in the folder — the batch-walk's supersession set stayed empty. Every document investigated exhaustively; zero deferrals. Seven of nine docs were individual design briefs for specific ratification gates (G-3, G-4, O-3, O-4, O-5, HPKE decision, custody-hook wire-format), all of whose technical substance has landed in the Rust crates, Python cross-check, fixture corpus, Companion, and lint rules. Eight of nine are archive candidates; the ninth (`2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md`) is load-bearing reference cited by `TODO.md`, `CLAUDE.md`, and `.claude/vision-model.md` — it needs rehoming before archive, not blind `git mv`.

## Summary table

| # | Document | Frontmatter status | Verdict | Key finding |
|---|----------|--------------------|---------|-------------|
| 1 | `2026-04-21-trellis-wos-custody-hook-wire-format.md` | Accepted | **FULLY RESOLVED** | Every follow-on landed; `append/010` regenerated with dCBOR + TypeID; Companion §24.9 aligned; WOS ADR 0061 accepted. |
| 2 | `2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md` | Accepted | **FULLY RESOLVED** | All four ADRs (0001–0004) ratified into `v1.0.0`; G-4 + G-5 closed. **Reference-of-record** — cited by TODO.md, CLAUDE.md, vision-model. Rehome before archive. |
| 3 | `2026-04-19-trellis-hpke-freshness-decision.md` | Recommendation ready | **FULLY RESOLVED** | Option (a) adopted; §4 drop-in text live at `specs/trellis-core.md:569-571`; `append/004-hpke-wrapped-inline` landed with pinned key. |
| 4 | `2026-04-18-trellis-o5-posture-transition-schemas.md` | (Closes: O-5 at design) | **FULLY RESOLVED** | All 6 decisions landed; disclosure-profile verifier extended; `tamper/016` landed; **G-O-5 re-closed on checklist** (investigator saw stale snapshot — correction below). |
| 5 | `2026-04-18-trellis-o4-declaration-doc-template.md` | (Closes: O-4 at design) | **MOSTLY RESOLVED** | O-4 gate closed; reference declaration corpus live; Lint Rule 11 shipped. **Rules 14 + 15 still open** (TODO.md Stream 5). |
| 6 | `2026-04-18-trellis-o3-projection-conformance.md` | (Closes: O-3) | **FULLY RESOLVED** | 5 projection + 2 shred fixtures landed; O-3 closed; `_pending-projection-drills.toml` retired. |
| 7 | `2026-04-18-trellis-g4-rust-workspace-plan.md` | (Closes: G-4) | **MOSTLY RESOLVED** | 10-crate split shipped; G-4 + G-5 closed. **Stale directory prose** (`trellis/rust/` → actual `crates/`) + Rust HPKE wrap/unwrap path still outstanding (TODO.md). |
| 8 | `2026-04-18-trellis-g3-fixture-system-design.md` | (Closes: G-3) | **FULLY RESOLVED** | Fixture system shipped verbatim; F6 lifecycle in use; coverage lint (R5/R7/R8/R11) landed; allowlists all closed. |
| 9 | `2026-04-18-trellis-g3-first-batch-brainstorm.md` | (Scope: first 5 vectors) | **FULLY RESOLVED** | Every predicted vector shipped; corrected invariant mapping is what landed; generator Option B absorbed as `_generator/_lib/`. **Not superseded by #8** — design pins system, brainstorm picks contents (orthogonal). |

**Taxonomy:** 7 × FULLY RESOLVED, 2 × MOSTLY RESOLVED, 0 × PARTIAL / STALE / SUPERSEDED / NOT STARTED.

## Per-source verdicts (newest → oldest)

### 1. `2026-04-21-trellis-wos-custody-hook-wire-format.md`

**Path:** `/Users/mikewolfd/Work/formspec/trellis/thoughts/specs/2026-04-21-trellis-wos-custody-hook-wire-format.md`
**Last touched:** `1e216a1` 2026-04-21 — `docs: accept WOS custodyHook wire-format note with dCBOR + TypeID resolution`

**Verdict: FULLY RESOLVED.** Every follow-on in §"Follow-on on the Trellis side" has landed. `append/010-wos-custody-hook-state-transition` regenerated to carry `input-wos-record.dcbor` and `input-wos-idempotency-tuple.cbor`. Companion §24.9 documents the two-field `(caseId, recordId)` idempotency construction under `trellis-wos-idempotency-v1` (`specs/trellis-operational-companion.md:1075-1081`). `COMPLETED.md:65-68` records the landing. Core §23 deferral preserved (`specs/trellis-core.md:1600-1608, 1645`). WOS ADR 0061 accepted at `../wos-spec/thoughts/adr/0061-custody-hook-trellis-wire-format.md`. All four Follow-on items executed; no forward work remains. **Archive candidate.**

**Open items:** all four Follow-on items closed (fixture regenerated ✓, Companion §24.9 aligned ✓, WOS coordination complete ✓, no Core §23 prose change required ✓).

### 2. `2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md`

**Path:** `/Users/mikewolfd/Work/formspec/trellis/thoughts/specs/2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md`
**Last touched:** `8911d12` — `docs: refine Trellis MVP roadmap and vision`

**Verdict: FULLY RESOLVED.** All four ADRs (0001–0004) ratified into `v1.0.0` per `TODO.md:14-23` and root `CLAUDE.md`. ADR 0001 (list-form `priorEventHash` + length-1 Phase-1 lint), ADR 0002 (list-form `anchor_refs` + single-anchor default), ADR 0003 (§22/§24 reservations MUST NOT populate), ADR 0004 (Rust byte authority + Python cross-check) — all live. G-4 closed (`ratification/ratification-checklist.md:16`), G-5 closed 45/45 (`:17`). Validation checklist `[x]` across all six items.

**Caveat:** this doc functions as **reference-of-record** for the Phase-1 ADR decisions — `TODO.md:15`, `CLAUDE.md`, `.claude/vision-model.md` all cite it as the authority. Archival is not a simple `git mv` — the ADR content needs rehoming (likely to `thoughts/adr/`) with citing docs updated in the same commit, or the reference links break.

**Open items:** none. The inline offer ("I can apply this vision-model.md update after this doc is validated") is satisfied per the CLAUDE.md snapshot.

### 3. `2026-04-19-trellis-hpke-freshness-decision.md`

**Path:** `/Users/mikewolfd/Work/formspec/trellis/thoughts/specs/2026-04-19-trellis-hpke-freshness-decision.md`
**Last touched:** `6b20ef3` 2026-04-19 — `docs(trellis): §9.4 HPKE-freshness decision memo — recommend Core amendment`

**Verdict: FULLY RESOLVED.** Option (a) adopted. The §4 drop-in text is live at `specs/trellis-core.md:569-571` verbatim, including the N-recipients-→-N-ephemerals clarifier and the `(a)/(b)/(c)` test-vector carve-out. `fixtures/vectors/append/004-hpke-wrapped-inline/` authored with pinned `_keys/ephemeral-004-recipient-001.cose_key`. `COMPLETED.md:75-86` records commits `ee57780` (Core amendment) and `4cc9fe8` (fixture + keys). All 5 Next-steps items executed. **Archive candidate.**

**Open items:** all five Next-steps items closed.

### 4. `2026-04-18-trellis-o5-posture-transition-schemas.md`

**Path:** `/Users/mikewolfd/Work/formspec/trellis/thoughts/specs/2026-04-18-trellis-o5-posture-transition-schemas.md`
**Last touched:** `f94342b` 2026-04-18 — `docs(trellis): Stream D design brief — O-5 posture-transition event schemas`

**Verdict: FULLY RESOLVED** (corrected from investigator's MOSTLY RESOLVED — see cross-ref delta #1 below). All six decisions landed: event-type strings registered, six fixtures cover attestation + state-continuity rules, matrix rows `TR-OP-042..045` added, `trellis-verify` extended with `decode_disclosure_profile_transition` at `crates/trellis-verify/src/lib.rs:1602`. G-O-5 was reopened 2026-04-23 and **re-closed 2026-04-23** per `ratification/ratification-checklist.md:39` (`[x]`) — the investigator's read of `[ ]` reflects a stale snapshot; the line has been updated. `tamper/016-disclosure-profile-from-mismatch` is the negative oracle. Verified by `cargo test -p trellis-conformance` and `python3 -m trellis_py.conformance` (63 vectors, 0 failures). **Archive candidate.**

**Open items (verbatim from source):**

> - **Reason-code registry governance** — propose `ReasonCode` as append-only per Core §6.7; `Other = 255` reserved catch-all. Warrants confirmation before landing as CDDL.
> - **Attestation signature scope** — Decision 3's `Attestation.signature` COSE_Sign1 countersignature needs a precise `Sig_structure` preimage. Propose: `Sig_structure` payload = `dCBOR([transition_id, effective_at, authority_class])` with domain-separation tag `trellis-transition-attestation-v1`.
> - **Disclosure-profile scope granularity** — brief assumes deployment-scope; finer-grained (per-case) profile transitions force new deployment declaration. Warrants review.
> - **Interaction with `trellis.external_anchor.v1`** (Core §6.7 Phase 2 reservation) — Phase 2 deployment may want transition events anchored with higher priority. Non-blocking for Phase 1.

These are upstream-posture open items, not blockers on this doc's archival.

### 5. `2026-04-18-trellis-o4-declaration-doc-template.md`

**Path:** `/Users/mikewolfd/Work/formspec/trellis/thoughts/specs/2026-04-18-trellis-o4-declaration-doc-template.md`
**Last touched:** `b40e8a4` 2026-04-18 — `docs(trellis): Stream C design brief — O-4 declaration-doc template`

**Verdict: MOSTLY RESOLVED.** O-4 gate closed (`ratification/ratification-checklist.md:38` `[x]`), Companion A.6 normative text landed (`8069062`, `65090f8`), reference SSDI-intake-triage declaration corpus live at `fixtures/declarations/ssdi-intake-triage/`, Lint Rule 11 implemented at `scripts/check-specs.py:1551`, frontmatter schema enforced with `supersedes` recognition. **Rules 14 and 15 still outstanding** per `TODO.md:125-131`: Rule 14 (signing-key structure validation without crypto) is partial; Rule 15 (`supersedes` chain acyclicity) is unimplemented. Ledger-replay Rules 7–13 are Phase-1-admissible-as-follow-on. Most substance landed; two named lint rules remain.

**Open items (verbatim from source):**

> - Declaration-doc lint rules 1–6, 14–15 in `scripts/check-specs.py` — static checks only; 1–6 landed, 14 partial, 15 not implemented.
> - Ledger-replay lint rules 7–13 in Rust conformance crate — Phase-1-admissible-deferral.
> - Appendix A status — resolved (Companion A.6 exists).
> - Event-type registry ownership — partially resolved via O-5 brief + Companion §24.9; not audited for full coverage here.
> - `decide` as authorized-action — disposition not independently verified.

**Archive caveat:** if archived, TODO.md Stream 5 cites this brief's Rule 14/15 surface by path — archiving requires updating that citation in the same commit.

### 6. `2026-04-18-trellis-o3-projection-conformance.md`

**Path:** `/Users/mikewolfd/Work/formspec/trellis/thoughts/specs/2026-04-18-trellis-o3-projection-conformance.md`
**Last touched:** `e895920` 2026-04-18 — `docs(trellis): Stream B design brief — O-3 projection conformance fixtures`

**Verdict: FULLY RESOLVED.** All 4 test shapes, op-dir layout, and coverage-enforcement contract landed. `fixtures/vectors/projection/` holds 5 fixtures matching Tests 1–3, `fixtures/vectors/shred/` holds 2 matching Test 4 (cascade + backup-refusal). Manifest schema matches design §Manifest verbatim. O-3 closed (`ratification/ratification-checklist.md:37`). `COMPLETED.md:307-314` records every item landed. `_pending-projection-drills.toml` retired as part of O-3 close. **Archive candidate.**

**Open items:** all five Follow-ons delivered (fixtures ✓, lint additions ✓, G-2 audit-path row integrated ✓, matrix cadence row delivered ✓, Core §15.3 canonical-encoding tightening delivered per `COMPLETED.md:307`). Three "Ambiguity" review-before-citing items resolved during implementation.

### 7. `2026-04-18-trellis-g4-rust-workspace-plan.md`

**Path:** `/Users/mikewolfd/Work/formspec/trellis/thoughts/specs/2026-04-18-trellis-g4-rust-workspace-plan.md`
**Last touched:** `30e76d7` 2026-04-18 — `docs(trellis): Wave 3D plan — G-4 Rust workspace`

**Verdict: MOSTLY RESOLVED.** G-4 gate closed (`ratification/ratification-checklist.md:16`, `COMPLETED.md:287-295`). Architectural commitments landed: 10-crate split matches plan exactly (`trellis-cddl`, `trellis-cose`, `trellis-types`, `trellis-core`, `trellis-verify`, `trellis-export`, `trellis-store-memory`, `trellis-store-postgres`, `trellis-conformance`, `trellis-cli` all present in `crates/`). Public API (`append`/`verify`/`export`) confirmed. Fixture runner walks `append/verify/export/tamper` and scaled cleanly to include `projection/shred`. Open Items #1–#8 all landed, resolved, or made moot.

**Staleness:** directory-layout §Stranger-test isolation proposed `trellis/rust/` but actual workspace is at `trellis/crates/` per parent CLAUDE.md convention. Stranger-test isolation mechanism that actually worked is documented in `ratification/g5-package/` (not the proposed `.g5-readlist`). Plan's §Stranger-test-isolation diagram is wrong relative to ground truth.

**Outstanding against plan discipline:** `TODO.md:144-150` flags "HPKE wrap/unwrap in Rust" as open — `trellis-core` has no Rust wrap/unwrap path, only round-trips committed bytes for `append/004`. Not a G-4 closure requirement (G-4 vectors still byte-matched via round-trip) but a latent gap against plan M1.a/M1.c discipline.

**Archive caveat:** same as #5 — safe to archive but note the HPKE-Rust gap in TODO.md before the `git mv` so the trail remains navigable.

### 8. `2026-04-18-trellis-g3-fixture-system-design.md`

**Path:** `/Users/mikewolfd/Work/formspec/trellis/thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md`
**Last touched:** `73234f7` — `docs(trellis): amend G-3 fixture-system design with F6 vector-lifecycle` (sequence: `68f53a2` → `64af7cc` → `73234f7`)

**Verdict: FULLY RESOLVED.** Every decision in force and producing fixtures. `fixtures/vectors/` laid out exactly per design §Directory layout: `append/` (15), `verify/` (16), `export/` (9), `tamper/` (16), plus O-3's `projection/` (5) and `shred/` (2). Underscored scaffolding (`_generator/`, `_keys/`, `_inputs/`, `_templates/`) excluded from runner walks. TOML manifest schema landed verbatim with per-vector tables. Coverage enforcement lint (R5/R7/R8/R11) landed (`ratification-checklist.md:14`). F6 lifecycle amendment (slug convention, renumbering forbidden, deprecation via status field) is the de-facto regime. Generator Python follows the allowed-import fence via `_generator/_lib/byte_utils.py`. `TRELLIS_SKIP_COVERAGE=1` replacement to per-invariant allowlists landed — `_pending-invariants.toml`, `_pending-projection-drills.toml`, `_pending-matrix-rows.toml` all closed (`ratification-checklist.md:15`). G-3 closed. **Top-tier archive candidate** — sibling scaffold plan already archived at `thoughts/archive/specs/2026-04-18-trellis-g3-fixture-scaffold-plan.md`.

**Open items:** all resolved. F6 lint enforcement (deprecation-field check, renumbering pre-merge guard) landed as part of `check-specs.py` hardening.

### 9. `2026-04-18-trellis-g3-first-batch-brainstorm.md`

**Path:** `/Users/mikewolfd/Work/formspec/trellis/thoughts/specs/2026-04-18-trellis-g3-first-batch-brainstorm.md`
**Last touched:** `50e9361` 2026-04-18 — `docs(trellis): Wave 3A brainstorm — first-batch G-3 vectors`

**Verdict: FULLY RESOLVED.** Every predicted vector shipped: `append/{002,003,004,005}` + `tamper/001` all present and byte-matched. The brainstorm's corrected invariant mapping (flagging TODO's mislabels: `002↔#7` not `#8`, `003↔#4` not `#6`, `005↔#5/#10/#13` not `#7`) is what landed per `COMPLETED.md:250-270`. Serial order Option C honored. All three predicted Core gaps (§10.2 prev_hash, §9.4 HPKE freshness, §8 rotation hash binding) surfaced and closed. Generator Option B ("narrow byte-level utilities only") absorbed as `_generator/_lib/byte_utils.py`. All 5 §6 Unknowns resolved.

**Critical finding: not superseded by #8 (fixture-system design).** The design pinned the *system* (layout, manifest, coverage lint, F6 lifecycle). The brainstorm chose the *first 5 vector identities*, corrected mislabels, picked order, predicted Core gaps — an authoring plan, not a system spec. Design §Non-goals explicitly excludes "authoring the ~50 vectors themselves." **Archive candidate** on its own merit.

**Open items:** all 5 §6 Unknowns resolved.

## Open items rollup

Deduplicated across sources. Tagged with source doc(s). All items below are upstream-posture items — no doc is blocked on them for archive purposes.

### Lint-rule follow-ons (Phase 1)

- [ ] **Rule 14** — declaration-doc signing-key structure validation without crypto verification — partial implementation. Source: `2026-04-18-trellis-o4-declaration-doc-template.md` §Lint rule surface. Tracked: `TODO.md:126-131` (Stream 5).
- [ ] **Rule 15** — declaration-doc `supersedes` chain acyclicity — unimplemented. Source: `2026-04-18-trellis-o4-declaration-doc-template.md` §Lint rule surface. Tracked: `TODO.md:126-131`.
- [ ] **Ledger-replay lint Rules 7–13** — declaration-doc checks that require ledger replay, deferred to Rust conformance crate. Not Phase-1-required. Source: `2026-04-18-trellis-o4-declaration-doc-template.md` §Follow-ons.

### Core amendments still queued

- [ ] **Core §17.5 `tamper_kind` enum promotion** — currently stable across 16 tamper vectors (`"signature_invalid"`, `"prev_hash_break"`, etc.) but not pinned as a Core-level registry. Not load-bearing for G-3; tracked: `TODO.md:213-216`. Source: `2026-04-18-trellis-g3-first-batch-brainstorm.md` §6 Unknown #1.
- [ ] **ReasonCode registry governance** — `ReasonCode` append-only with `Other = 255` reserved catch-all. Needs confirmation before landing as CDDL. Source: `2026-04-18-trellis-o5-posture-transition-schemas.md` §Open items.
- [ ] **Attestation signature scope tightening** — `Sig_structure` preimage for `Attestation.signature` needs precise pinning: `dCBOR([transition_id, effective_at, authority_class])` + domain tag `trellis-transition-attestation-v1`. Source: `2026-04-18-trellis-o5-posture-transition-schemas.md` §Open items.

### Phase-2+ posture questions (not blocking Phase 1)

- [ ] **Disclosure-profile scope granularity** — whether per-case profile transitions are allowed or forced to deployment-scope. Source: `2026-04-18-trellis-o5-posture-transition-schemas.md` §Open items.
- [ ] **`trellis.external_anchor.v1` priority** — Phase 2 interaction with posture-transition events. Source: `2026-04-18-trellis-o5-posture-transition-schemas.md` §Open items.

### Implementation gaps (tracked in TODO.md)

- [ ] **Rust HPKE wrap/unwrap path** — `trellis-core` has no Rust wrap/unwrap path for `append/004`; only round-trips committed bytes. Tracked: `TODO.md:144-150`. Source: `2026-04-18-trellis-g4-rust-workspace-plan.md` (plan's M1.a/M1.c discipline, not a G-4 closure requirement).
- [ ] **`trellis-cli` surface expansion** — currently scaffold-only per plan §Non-goals. Intended forward work not yet scheduled. Source: `2026-04-18-trellis-g4-rust-workspace-plan.md`.

## Cross-ref deltas

High-value findings where this audit diverges from other indices.

1. **Batch-1 investigator read a stale O-5 checklist snapshot.** The batch-1 verdict for `2026-04-18-trellis-o5-posture-transition-schemas.md` reported `[ ]` for O-5 in `ratification/ratification-checklist.md:39`. Ground truth at HEAD is `[x]` — **re-closed 2026-04-23** per commit `5a6c9d5 docs(ratification): re-close G-O-5 after disclosure-profile verifier fix`. The verdict has been corrected to FULLY RESOLVED in this audit's summary table and §4 block. **Action for the skill:** investigators should re-read ratification-checklist.md at the top of each batch if the batch spans a known-reopened gate. The cross-ref lookup cost is cheap; a stale gate-status claim in the output is expensive.

2. **`_pending-projection-drills.toml` prose drift.** `2026-04-18-trellis-o3-projection-conformance.md:164` still cites `_pending-projection-drills.toml` as the allowlist registry. That file was removed when O-3 closed per `ratification/ratification-checklist.md:15`. Doc prose drifts from repo reality — archive-with-caveat (the historical reference stands, but readers should not assume the file exists).

3. **G-4 plan directory layout.** `2026-04-18-trellis-g4-rust-workspace-plan.md` §Directory layout diagram says `trellis/rust/`; repo uses `trellis/crates/`. Plan prose is wrong relative to ground truth. Again: archive-with-caveat, not a blocker.

4. **Brainstorm-vs-design is NOT a supersession pattern here.** `2026-04-18-trellis-g3-fixture-system-design.md` (#8) and `2026-04-18-trellis-g3-first-batch-brainstorm.md` (#9) look like the canonical "brainstorm superseded by formal design" pattern but are orthogonal: design = system, brainstorm = first-batch content. Squasher correctly did not collapse them. Worth noting for future `squashing-specs` runs — filename keywords ("brainstorm", "plan", "design") are a hint, not a reliable supersession signal.

5. **TODO.md / COMPLETED.md citations will break on naive archive.** Docs #2, #5, and #7 are cited by path in `TODO.md` streams. A blind `git mv` breaks those links. Archive-move commands below include the caveat; migrate citations in the same commit.

## Archive-move checklist

**Review each line before executing.** Eight of nine docs are archive candidates; one (`#2`) should not be archived without first rehoming its ADR content. Execute in batches, not all at once — run the lint after each to catch broken citations.

```bash
# Group A — clean archives (no citation update needed)
git mv thoughts/specs/2026-04-21-trellis-wos-custody-hook-wire-format.md \
       thoughts/archive/specs/2026-04-21-trellis-wos-custody-hook-wire-format.md

git mv thoughts/specs/2026-04-19-trellis-hpke-freshness-decision.md \
       thoughts/archive/specs/2026-04-19-trellis-hpke-freshness-decision.md

git mv thoughts/specs/2026-04-18-trellis-o3-projection-conformance.md \
       thoughts/archive/specs/2026-04-18-trellis-o3-projection-conformance.md

git mv thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md \
       thoughts/archive/specs/2026-04-18-trellis-g3-fixture-system-design.md

git mv thoughts/specs/2026-04-18-trellis-g3-first-batch-brainstorm.md \
       thoughts/archive/specs/2026-04-18-trellis-g3-first-batch-brainstorm.md

git mv thoughts/specs/2026-04-18-trellis-o5-posture-transition-schemas.md \
       thoughts/archive/specs/2026-04-18-trellis-o5-posture-transition-schemas.md

# Group B — archive WITH citation migration (update TODO.md in same commit)
# These docs are cited by path in TODO.md / streams. Before or in the same commit:
#   1. Update TODO.md:126-131 (Stream 5) to remove the path reference to the
#      O-4 template doc, or update to archive path.
#   2. Confirm no broken links via: grep -rn "thoughts/specs/2026-04-18-trellis-o4" . --include="*.md"

git mv thoughts/specs/2026-04-18-trellis-o4-declaration-doc-template.md \
       thoughts/archive/specs/2026-04-18-trellis-o4-declaration-doc-template.md

# Before archiving: confirm no TODO.md line still cites the plan by its pre-archive
# path. grep: grep -rn "thoughts/specs/2026-04-18-trellis-g4" . --include="*.md"
git mv thoughts/specs/2026-04-18-trellis-g4-rust-workspace-plan.md \
       thoughts/archive/specs/2026-04-18-trellis-g4-rust-workspace-plan.md

# Group C — DO NOT ARCHIVE yet (reference-of-record)
# 2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md is cited by
# TODO.md:15, CLAUDE.md, and .claude/vision-model.md as the authority for
# ADRs 0001-0004. Before archiving:
#   1. Decide whether to rehome ADR content to thoughts/adr/ (0070-0073 range
#      available — check thoughts/README.md for next free ID).
#   2. Update all citing docs to the new path in the same commit.
#   3. Then archive this file as historical.
# Skipping this git mv intentionally.
```

## Recommendations

1. **Run the Group A archive block today.** Six docs, zero external-citation risk, all ratified. Frees 6 / 9 of the active `thoughts/specs/` set and matches the repo's archival convention (`thoughts/archive/specs/`). Total diff: 6 `git mv` commands.

2. **Run Group B in a second commit** after grep-confirming no broken links (the grep commands are inline in the checklist). Two docs, each cited by TODO.md; a one-line TODO update plus the `git mv` in the same commit keeps the trail navigable.

3. **Defer Group C pending a rehoming decision.** The 2026-04-20 Phase-1-MVP-principles doc is load-bearing in three places (TODO.md, CLAUDE.md, vision-model.md). Archival requires either (a) promoting the ADR content to `thoughts/adr/` with proper ADR numbering (next free is 0074 per root CLAUDE.md), or (b) leaving the doc in `thoughts/specs/` permanently as "reference material, not a spec-in-flight." Neither is a bad call; pick one before archiving.

4. **Harden the squasher against stale checklist reads.** Cross-ref delta #1 above is a real bug: batch-1's investigator read O-5 as `[ ]` when HEAD has `[x]`. Either the agent cached an earlier Read result or the investigator ran before a shared-context refresh. Add to the spec-investigator system prompt: "When reporting a gate's status, the checklist line MUST be re-read from disk at the moment the verdict is written, not from batch-top shared context." Low-cost fix; catches a systematic false-stale-flag.

5. **Land the Rule 14/15 lint pair before archiving O-4.** Doc #5 is archive-ready except for these two named rules. They're small (both are static checks). Landing them closes the `MOSTLY RESOLVED` → `FULLY RESOLVED` gap and makes Group B a clean single-commit move.

6. **Document that brainstorm-design orthogonality is real.** The G-3 brainstorm (#9) stands on its own even next to the G-3 fixture-system design (#8). Future `squashing-specs` runs should expect this pattern where a "brainstorm" picks content and a "design" pins the system — and not auto-collapse them. Worth adding to the skill's edge-cases section.

---

*Generated by `squashing-specs` skill v1 (batched mode, batch_size=5). Investigator: `spec-investigator`. Walked 9 docs in 2 batches. Zero deferrals. See `thoughts/audit-2026-04-23-design-docs-vs-specs-and-code.md` for the prior, broader-scope audit whose rollup this complements.*
