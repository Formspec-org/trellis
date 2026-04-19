# Wave 1 Consolidation Plan — Applying Design-Brief Outputs

**Date:** 2026-04-18
**Scope:** Merge the four Wave 1 design briefs (G-2 / O-3 / O-4 / O-5) back into Core + Companion + Matrix + `check-specs.py`.
**Inputs:**
- [`2026-04-18-trellis-g2-invariant-audit-paths.md`](2026-04-18-trellis-g2-invariant-audit-paths.md)
- [`2026-04-18-trellis-o3-projection-conformance.md`](2026-04-18-trellis-o3-projection-conformance.md)
- [`2026-04-18-trellis-o4-declaration-doc-template.md`](2026-04-18-trellis-o4-declaration-doc-template.md)
- [`2026-04-18-trellis-o5-posture-transition-schemas.md`](2026-04-18-trellis-o5-posture-transition-schemas.md)

**Does not cover:** authoring the O-3 / O-4 / O-5 fixtures themselves, or building the conformance runners that consume them. Those are follow-on plans that load from the edited specs.

## Cross-stream resolutions landed

| Issue | Resolution |
|---|---|
| C's lint rule 7 depends on a registry of operator event-type strings | D's Core §6.7 follow-on is the registry (reject-if-unknown-at-version). C's rule 7 can cite §6.7. |
| A classified invariant #14 as `projection-rebuild-drill` only; B adds byte-testable coverage via rebuild-equivalence | Once B's TR-OP-005/006 rows flip to `Verification=test-vector` (below), A's hybrid rule auto-promotes #14 to hybrid. A's brief stands. |
| B's backup-refusal (Test 4 negative variant) — fixture-testable or declaration-doc-check? | Keep in B as fixture-testable. It is observed behavior, not a static declaration. C's template is unchanged. |

## Spec-edit backlog (merge into Core + Companion + Matrix)

Grouped by file. Each item cites the source brief. Order within a file is not load-bearing.

### `specs/trellis-core.md`

- **§6.7 — register two event-type identifiers.** Add `trellis.custody-model-transition.v1` and `trellis.disclosure-profile-transition.v1` to the Phase 1 reject-if-unknown-at-version registry. Source: D §"Event-type strings." **XS**.
- **§9.8 — register new domain tag.** Add `trellis-posture-declaration-v1` for `declaration_doc_digest` preimages. Source: D follow-ons. **XS**.
- **§15.3 — pin canonical encoding for rebuilt artifacts.** Require dCBOR for rebuild output so two conforming implementations produce byte-equal rebuilds. Source: B gap #1. **XS**.
- **§19 — extend verification algorithm.** Add step 5.5 (state-continuity + attestation-count checks for transition events); extend `VerificationReport` CDDL with `posture_transitions: [* PostureTransitionOutcome]`. Source: D follow-ons. **S**.
- **§15.2 vs Companion §14.1 — reconcile `projection_schema_id`.** The Companion view-watermark carries a field absent from the Core CDDL. Either add to Core CDDL or clarify the Companion field as a superset. Source: B ambiguity #3. **XS**.

### `specs/trellis-operational-companion.md`

- **§16.2 — add TR-OP row for OC-46 snapshot cadence.** The obligation exists in prose but has no matrix anchor, which blocks B's Test 3 coverage lint. Source: B gap #2. **XS**.
- **§20.5 — promote cascade-scope vocabulary to Appendix A enum.** The six-class list is prose-only; make it machine-checkable. Source: B gap #3. **XS**.
- **Appendix A.5 — rewrite as "Posture Transition Event Families."** Preserve existing generic 8-field `PostureTransition` parent shape (it maps to §10.3). Add A.5.1 (custody-model) and A.5.2 (disclosure-profile) carrying D's CDDL. Generic fields `prior_posture_ref` / `new_posture_ref` URI are realized by `declaration_doc_digest` + implicit `from_*` / `to_*` in subtypes. Source: D Appendix A.5 disposition. **S**.
- **§27 — cite O-3 / O-5 fixtures.** Once fixtures exist, §27 (Operational Conformance Tests) references them by ID. Source: D follow-ons. **XS** (wait until fixtures land).

### `specs/trellis-requirements-matrix.md`

- **Add TR-OP-042..045.** Transition schema (custody), transition schema (disclosure), verifier rule, co-publish rule. All `Verification=test-vector`. Source: D follow-ons. **XS**.
- **Add TR-OP row for snapshot cadence (OC-46 anchor).** Needed for B Test 3. `Verification=test-vector`. Source: B gap #2. **XS** (paired with Companion §16.2 edit above).
- **Flip TR-OP-005/006 Verification cells.** Add `test-vector` alongside existing `projection-rebuild-drill` so invariant #14 auto-promotes to hybrid per A's rule. Source: A/B cross-stream resolution. **XS**.
- **Add TR-CORE row for invariant #1's forward-looking registry clause?** A flagged this as optional. Defer until a registry-companion spec exists. **n/a**.

### `specs/trellis-operational-companion.md` — Appendix A.6 (new)

- **Delegated-Compute Declaration Document template.** C proposes housing the new artifact adjacent to Appendix A rather than inside it. WG call. If WG prefers Appendix A housing, transplant C's schema as A.6. Source: C flagged review #1. **S** (WG decision + mechanical transplant).

## `check-specs.py` backlog (bundle into one lint refactor plan)

Rather than 4 independent PRs, bundle all lint additions into a single refactor that extends `check-specs.py` once. Shared plumbing (`tr_op` coverage, `companion_sections` verification, TOML-allowlist loader) shows up in every stream.

### Extensions

- **`tr_op` coverage.** Mirror `tr_core` but for `TR-OP-*` rows. Feeds O-3 / O-4 / O-5. Source: B / C / D.
- **`companion_sections`.** Mirror `core_sections` for Companion. Feeds O-3 / O-4 / O-5. Source: B / C / D.
- **`_pending-projection-drills.toml`.** Parallel to `_pending-invariants.toml` for projection-rebuild-drill coverage during rollout. Source: B.
- **`projection` / `shred` op dispatch.** Extend `vector_manifests()` to walk two new op-dirs. Source: B.
- **Declaration-doc schema validator.** Load declaration docs; validate against C's schema; cross-check posture_declaration_ref → posture doc. Source: C lint rules 1–6 + 14–15.
- **Event-type registry check.** Verify every emitted event-type in test vectors appears in Core §6.7 registry. Source: D.
- **CDDL cross-ref check.** Verify event-type strings in vectors match the CDDL types they claim. Source: D.
- **Fixture-naming guard.** Enforce `append/NNN-slug` / `tamper/NNN-slug` conventions; reject drift. Source: D.
- **Spec-cross-ref row resolution.** For TR rows with `Verification=spec-cross-ref`, verify the referenced §N exists. Source: A.
- **Projection-rebuild-drill coverage.** For TR rows with this Verification, verify a projection fixture references the row. Source: A/B.
- **Model-check evidence assertion.** For TR rows with `Verification=model-check`, verify an evidence artifact (TLA+ spec, Alloy model, etc.) exists at a declared path. Source: A.

### Plan shape

One `thoughts/specs/2026-04-18-trellis-wave1-lint-extension-plan.md` (to be written) that sequences: (a) shared plumbing first; (b) per-stream lint rules in the same order as the spec edits above; (c) test fixtures under `scripts/check-specs-fixtures/` following the existing pattern. Size: **M** total.

## Execution sequencing

Proposed order — each step unblocks later ones:

1. **Spec edits in Core + Companion** (this plan, §"Spec-edit backlog"). **S** total.
2. **Matrix edits** (this plan, §"trellis-requirements-matrix.md"). **XS**. Can land in the same commit as the spec edits it depends on.
3. **Lint extension plan** (`thoughts/specs/2026-04-18-trellis-wave1-lint-extension-plan.md`). **S** (design brief). Then **M** (implementation).
4. **Fixture authoring** — O-3 (**M**), O-4 reference declaration (**S**), O-5 six fixtures (**S**). Start after (3) lands so lint catches authoring drift in real time. Can partially overlap with Rust G-4 work.

Item (1) is the hard blocker. Items (3)–(4) are parallelizable with G-3 critical-path work.

## Open items for WG review before execution

Flagged by the agents; orchestrator/WG to resolve before these briefs are treated as authoritative.

- **Stream A:** invariant #14 ownership boundary once B's matrix edits land (likely auto-resolves to hybrid, but confirm).
- **Stream A:** invariant #1 spec-cross-ref forward-looking clause — close it today, or defer until a registry-companion spec exists?
- **Stream A:** non-byte-channel audit signer — default is the Agreement §11 signer; WG may designate a dedicated role.
- **Stream B:** cadence-violation self-detection — implementation obligation or auditor-only?
- **Stream C:** declaration-doc housing — standalone artifact or Appendix A.6?
- **Stream C:** drop `scope.authorized_actions = "decide"` for Phase 1?
- **Stream D:** reason-code registry governance — `Other = 255` as append-only catch-all acceptable?
- **Stream D:** disclosure-profile transition granularity — deployment-scope only, or per-case?

## Non-goals

- Authoring O-3 / O-4 / O-5 fixtures (follow-on plans per stream).
- Writing the Rust conformance-crate replay rules C lists as follow-ons 7–13 (blocked by G-4).
- Building O-3 / O-5 runners (separate plans; vectors are data).

## Consumers

Once landed:
1. G-3 critical-path vector authoring picks up the new domain tags (§9.8) and event-type registry (§6.7) mid-flight.
2. O-3 / O-4 / O-5 fixture authoring plans are unblocked.
3. G-2 closure becomes mechanical — the audit-path table lives in the G-2 brief; lint enforces it.
4. G-4 Rust workspace plan inherits the ~10 lint rules as its conformance-runner test scaffold.
