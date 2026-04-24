# Trellis O-3 Projection Conformance — Design

**Date:** 2026-04-18
**Scope:** conformance-fixture design for the four obligations O-3 names — watermark attestation, rebuild equivalence, snapshot cadence, purge-cascade verification.
**Closes:** O-3 (design); instantiates Companion §27.2 (Projection Rebuild Tests) and §27.3 (Crypto-Shred Cascade Tests) in executable form.
**Unblocks:** authoring of the O-3 fixtures themselves; G-4 Rust impl conformance runner for operational-tier deployments.
**Does not cover:** authoring the fixtures; re-normatizing Companion prose; O-4 declaration-doc check (Stream C) or O-5 posture-transition events (Stream D). Those are separate streams; the matrix rows they anchor (TR-OP-031, §10 transitions) are out of scope here.

## Context

Ratification bar per `ratification/ratification-checklist.md` O-3: watermark contract, rebuild equivalence, snapshot cadence, and purge-cascade verification MUST have conformance fixtures. The normative obligations live in `specs/trellis-operational-companion.md` §14 (Derived-Artifact Discipline), §15 (Projection Runtime Rules), §16 (Snapshot-from-Day-One), §17 (Staff-View Integrity), and §27 (Operational Conformance Tests). The byte-level anchor is `specs/trellis-core.md` §15 (Snapshot and Watermark Discipline), which fixes the `Watermark` CDDL shape `(scope, tree_size, tree_head_hash, checkpoint_ref, built_at, rebuild_path)`.

O-3 is a **different kind of conformance** from G-3. G-3 vectors are closed-form input/output byte checks: one `append` call, one expected canonical event, byte-compare, done. O-3 tests are temporal and cross-artifact — a chain grows, a projection is built at some height, plaintext is destroyed, downstream state must transition. The fixture format has to carry a chain, a derived view, a procedure, and an expected post-state. The runner has to execute, not just compare.

This design resolves four decisions: fixture location in the repo, manifest schema (TOML vs richer), runner contract per test, and coverage-enforcement shape. It deliberately inherits as much of G-3 as possible — the principle that the stranger test is "match spec, not match impl" applies identically.

## Structural options considered

### Option A — Reuse `fixtures/vectors/<op>/` with new ops `projection/` and `shred/`

Treat O-3 tests as two more operation dirs alongside `append/verify/export/tamper/`. Manifests use the G-3 TOML schema with op-specific `[inputs]` / `[expected]` fields. Runners extend the existing `op` dispatch.

- **Pro:** zero new infrastructure. Existing lint rules, `_pending-invariants.toml`, coverage accounting, and slug/deprecation discipline apply unchanged. One walker, one manifest parser, one place implementors look.
- **Pro:** keeps the stranger test's surface area minimal — an implementor already reading `fixtures/vectors/` doesn't discover a second fixture system.
- **Con:** G-3 vectors are pure data with byte-compare outputs. O-3 tests execute a rebuild procedure and compare post-state flags. The `op` enum stops meaning "function under test" and starts meaning "execution mode." That's a real stretch of the G-3 contract.
- **Con:** chains in O-3 are multi-event and grow over time. Encoding a 30-event chain as a flat `input-ledger.cbor` is viable but ugly; the `input-*.cbor` convention assumes one input per field.

### Option B — Sibling tree `fixtures/projections/` with a richer manifest (TOML with nested chain + view + procedure sections)

A parallel directory outside `fixtures/vectors/`. Manifest stays TOML but grows structure: `[chain]` declares event files in order, `[view]` declares the derived artifact + its watermark, `[procedure]` names the rebuild algorithm the runner invokes, `[expected]` declares post-state assertions (flags, hashes, cascade outcomes).

- **Pro:** honest about the shape difference. O-3 is operational conformance, not byte conformance — giving it its own tree signals that to readers and keeps the G-3 corpus tight.
- **Pro:** runner contract is cleanly distinct. G-3 runner byte-compares; O-3 runner executes a named procedure against a canonical chain and checks post-state. Mixing them pressures the G-3 runner to grow branches it doesn't want.
- **Pro:** TOML still scales — nested tables handle chain ordering and expected-state trees without escape hatches. CBOR/JSON buys nothing here; the canonical bytes already live in sibling `.cbor` files (chain events, derived-view blobs, expected post-state blobs), and the manifest's role is narrative + pointers, exactly as in G-3.
- **Con:** two lint rule sets, two walkers, two `_pending-*.toml` registries to maintain.
- **Con:** some implementors may miss `fixtures/projections/` if they only follow the G-3 path. Mitigation: `README.md` at `fixtures/` top level enumerates both trees.

### Option C — `fixtures/vectors/projection/` as a new op-dir with richer per-vector manifest shape

Keep the tree unified (implementor finds one root) but let `projection/` vectors use a superset manifest — same common fields (`id`, `op`, `description`, `status`, `[coverage]`, `[derivation]`) plus new sections (`[chain]`, `[view]`, `[procedure]`, `[expected.post_state]`) that only exist when `op = "projection"` or `op = "shred"`.

- **Pro:** one root, one lint walker, one coverage registry. Implementor discoverability of Option A + schema honesty of Option B.
- **Pro:** the G-3 manifest schema is already a tagged union on `op` — extending the union is idiomatic, not invasive.
- **Con:** the G-3 lint has to learn that op-specific schema shapes diverge more than they do today. Small code change, but the rule "a vector is self-describing" now admits more shape diversity.
- **Con:** `fixtures/vectors/_pending-invariants.toml` is scoped to byte-testable Core invariants. O-3 coverage is `TR-OP-*` rows with `Verification = projection-rebuild-drill`, not `test-vector` — so a parallel `_pending-projection-drills.toml` (or equivalent) is needed anyway.

### Decision — Option C

Pick C. Rationale: one tree, one implementor path, and the G-3 manifest is already a tagged union on `op` — adding `projection` and `shred` ops is the smallest structural change that doesn't misrepresent what the fixtures do. The objection that O-3 executes rather than byte-compares is answered by giving each op its own runner contract (below): dispatch on `op` already means "this test shape is different"; that's what tagged unions are for. Option B's honesty about shape difference is real but overpriced — the cost of splitting the tree (two walkers, two lints, two registries, two READMEs, two discoverability paths) exceeds the signal value of "these are different in kind." Option A is rejected outright: `op` must mean something more specific than "execution mode," or the dispatch loses its type-checking value.

## Fixture contract

Each O-3 fixture is a directory `fixtures/vectors/<op>/NNN-slug/` where `<op>` is `projection` or `shred`. It carries:

- `manifest.toml` — common fields (`id`, `op`, `description`, optional `status` / `deprecated_at`, `[coverage]`, `[derivation]`) plus op-specific sections.
- `derivation.md` — prose cite-chain to Companion §§14–17 / §20 / §27; same template as G-3 but the Core §-roadmap paragraph is replaced with a Companion §-roadmap.
- `chain/` — an ordered sequence of committed canonical events `000-event.cbor`, `001-event.cbor`, … plus `checkpoints/NNN.cbor` at declared heights. Ordering and naming mirror Core §18's export ZIP layout.
- `view/` — the derived artifact under test, as bytes. One or more files (e.g., `view.cbor`, `view-rebuilt.cbor`) depending on op.
- `expected/` — post-state assertion blobs. For rebuild-equivalence, this is the byte-exact rebuilt view. For shred, this is a flag table plus optionally a post-cascade view state.

### Manifest schema additions

```toml
id          = "projection/001-watermark-attestation"
op          = "projection"   # or "shred" for purge-cascade tests
description = "Derived artifact carries a conforming Watermark per Core §15.2."

[coverage]
tr_op       = ["TR-OP-001", "TR-OP-002"]                     # canonical for O-3
tr_core     = ["TR-CORE-090"]                                 # optional; watermark CDDL anchor
companion_sections = ["§14.1", "§15.2", "§17.2"]              # optional; lint-verified against tr_op
# invariants = [14]                                           # optional commentary, warning-only

[derivation]
document = "derivation.md"

# Op-specific — see per-test sections below.
[chain]
events      = ["chain/000-event.cbor", "chain/001-event.cbor", "chain/002-event.cbor"]
checkpoints = ["chain/checkpoints/002.cbor"]

[view]
artifact    = "view/view.cbor"
schema_id   = "trellis.staff-view.v1"

[expected.watermark]
scope          = "case-0001"
tree_size      = 3
tree_head_hash = "hex:aabbccdd…"
checkpoint_ref = "hex:112233…"
rebuild_path   = "trellis.staff-view.v1/default"
```

**Why TOML stays.** The bytes that matter — events, checkpoints, views, rebuilt views — are already sibling `.cbor` / `.bin` files, same as G-3. The manifest's job is narrative + pointers + small-data expected state (the watermark tuple, flag tables). JSON loses comments; CBOR creates a circular dependency (a Trellis fixture manifest encoded in the format the fixture tests); YAML's indentation traps are a worse failure mode than nested TOML tables. Nested `[expected.watermark]` / `[expected.post_state.view_stale]` tables scale fine.

**Deprecation and slug lifecycle.** Inherit G-3's F6 rules verbatim — `status = "deprecated"` tombstones, renumbering forbidden after merge, overlap encouraged, boolean coverage.

## Runner contract

Inherit G-3's data-only principle: vectors are pure data, each implementation writes its own runner. The G-3 rationale applies identically — a shared protocol would dilute the stranger test by making "did I implement the runner right?" compete with "did I implement Companion right?"

What changes from G-3 is the per-op runner **responsibilities**. Dispatch on `op` already carries shape information; the runner's job is different for each.

### Test 1 — Watermark attestation (op = "projection", subtype = "watermark")

**Fixture shape.** A canonical chain of `N` events, one checkpoint at `tree_size = N`, and a derived view artifact. The view artifact MUST embed a `Watermark` record per Core §15.2.

**Runner contract.** The runner loads `view/view.cbor`, extracts its `Watermark` field, and compares each sub-field to the values declared in `[expected.watermark]`. Additionally, the runner verifies that `checkpoint_ref` resolves to one of the files under `chain/checkpoints/` (presence check), and that the checkpoint at that ref has matching `(tree_size, tree_head_hash)`.

**Pass/fail.** Pass iff (a) every required field in §14.1 is present in the view's watermark, (b) every field value matches `[expected.watermark]` byte-for-byte, (c) `checkpoint_ref` resolves within the fixture. Fail on any missing field, any mismatch, or any unresolvable reference.

**Coverage anchor.** `tr_op` includes `TR-OP-001`, `TR-OP-002`. Covers Companion §14.1 (watermark fields), §15.2 (watermark display — presence subset), §17.2 (staff-view watermark propagation) for staff-view subtype. OC-32, OC-38, OC-49.

### Test 2 — Rebuild equivalence (op = "projection", subtype = "rebuild")

**Fixture shape.** A canonical chain of `N` events, a checkpoint at `tree_size = N`, a derived view `view/view.cbor` declared to have been built at that checkpoint under declared configuration, and an expected `expected/view-rebuilt.cbor`. `[procedure]` names the rebuild path (e.g., `trellis.staff-view.v1/default`) and lists the declared-deterministic fields (Companion §15.3 OC-40).

**Runner contract.** The runner re-executes the rebuild procedure: replay events `chain/000..(N-1).cbor` in canonical order, apply the projection transform named by `[procedure].rebuild_path`, emit a rebuilt view, and byte-compare against `expected/view-rebuilt.cbor`. The runner also byte-compares against `view/view.cbor` when the fixture declares `[expected].rebuild_matches_original = true` (the happy path — fixture-author states that `view.cbor` and the rebuild agree).

**Pass/fail.** Pass iff the rebuilt bytes equal `expected/view-rebuilt.cbor` across every declared-deterministic field. Non-deterministic fields (per §15.3 OC-40) are declared in `[procedure].non_deterministic_fields` and stripped before comparison. Fail on any mismatch in a deterministic field, or on presence of a field not declared in either set.

**Pass/fail — deliberate gap.** "Replaying the canonical chain from genesis must reproduce the derived view byte-for-byte" is Companion §15.3's promise for **declared-deterministic fields only**. A blanket byte-for-byte equivalence promise is overclaim — Companion §15.3 explicitly permits non-deterministic fields. The fixture MUST declare which fields are which; the runner enforces the split.

**Coverage anchor.** `tr_op` includes `TR-OP-005`, `TR-OP-006`. Covers OC-39, OC-40, OC-44.

### Test 3 — Snapshot cadence (op = "projection", subtype = "cadence")

**Fixture shape.** A canonical chain spanning a declared cadence window — e.g., for height-based cadence "every 10 events," a chain of 30 events with expected checkpoints at heights 10, 20, 30. `[procedure]` declares the cadence rule (`kind = "height-based"`, `interval = 10` | `kind = "time-based"`, `interval_seconds = 3600` | `kind = "event-driven"`, `trigger_event_types = […]` | `kind = "hybrid"`, composition). `[expected.checkpoints]` lists the heights/events at which checkpoints are required.

**Runner contract.** The runner walks `chain/checkpoints/` and asserts that for every required checkpoint declared in `[expected.checkpoints]`, a matching checkpoint file exists with the declared `(tree_size, tree_head_hash)`. The runner also asserts **no gaps** — under the declared cadence, absent checkpoints where the cadence requires them is NON-CONFORMANT per Companion §16.2 OC-46.

**Negative variant.** A `status = "active"`, `[expected].fail = true` fixture carries a chain with a deliberate cadence gap (e.g., checkpoint missing at height 20 when cadence is 10). The runner MUST detect the gap and report the missing-checkpoint failure. This closes §16.2's "Absent snapshots where the cadence requires them is a conformance violation" as a testable assertion.

**Pass/fail.** Pass iff every required checkpoint is present with correct `(tree_size, tree_head_hash)` (positive fixtures) OR iff the runner reports the declared gap (negative fixtures). Fail on any unexpected absence, any unexpected presence, or any `(tree_size, tree_head_hash)` mismatch.

**Open cadence question.** Companion §16.2 lets each deployment declare its own cadence — there is no single normative cadence number. The fixture therefore ships one fixture per cadence kind (time / height / event-driven / hybrid) at a declared interval, and the runner checks conformance to that fixture's declared interval. This is sufficient for O-3: the obligation is "declared cadence is met," not "a specific cadence is universally required." If a future revision of Companion §16 pins a minimum cadence floor, add a lint rule that rejects fixtures declaring below-floor intervals — not a fixture change.

**Coverage anchor.** `tr_op` includes `TR-OP-006` (and a new `TR-OP-*` row if the matrix does not yet enumerate cadence compliance — flagged under Core/Companion gaps below). Covers OC-45, OC-46, OC-47.

### Test 4 — Purge-cascade verification (op = "shred")

**Fixture shape.** A canonical chain of `N` events including at least one `PayloadInline` event whose DEK is later destroyed by a canonical erasure fact (an event declaring crypto-shred of a named payload per Companion §20.3). Plus: a pre-shred derived view `view/view-pre.cbor` that materialized the plaintext, and an expected post-shred view `expected/view-post.cbor` that has been invalidated or flagged stale. `[procedure]` names the cascade policy (Companion §20.5 — which derived-artifact classes are in scope).

**Runner contract.** The runner replays the chain up to the shred event, materializes `view-pre.cbor`, then applies the declared cascade procedure (§20.4 OC-76 "invalidated, purged, or otherwise made unusable"), and checks the post-state against `[expected.post_state]`:

- `view_invalidated: bool` — the view is removed or flagged unusable.
- `plaintext_residue_absent: bool` — no plaintext from the shredded payload remains in the post-state view.
- `cascade_completeness: list[str]` — each derived-artifact class declared in `[procedure].cascade_scope` (§20.5) MUST appear in the post-state report with `invalidated = true` OR `plaintext_absent = true`.

**Pass/fail.** Pass iff every declared scope class reports invalidation or plaintext-absent, AND no plaintext bytes from the shredded payload appear anywhere in the post-state artifacts. Fail if any scope class is missing, if any class reports plaintext residue, or if `view-post.cbor` disagrees with `expected/view-post.cbor` on the declared flags.

**Backup variant.** A second fixture carries a backup snapshot containing the plaintext and asserts that the post-cascade state MUST NOT restore the backup into a live derived artifact (Companion §16.5, §20.5). The runner simulates a recovery attempt and asserts it is refused.

**Coverage anchor.** `tr_op` includes `TR-OP-004`. Covers OC-75, OC-76, OC-77.

## Coverage enforcement

Mirror G-3's `coverage.tr_core` pattern with an `tr_op` lane:

1. Every matrix row where `Verification` contains `projection-rebuild-drill` AND Scope is `operational` MUST have ≥1 non-deprecated vector whose `coverage.tr_op` contains that row's ID. Anchor rows: `TR-OP-001..007`, plus `TR-OP-031` for evaluator discipline (out of O-3 scope — track under Stream C or a later op batch, not here).
2. A vector's `companion_sections`, if declared, is lint-verified equal to the set derived from its `tr_op` list via matrix lookup (error on mismatch, matching G-3's `core_sections` rule).
3. A vector's `invariants`, if declared, is commentary only — warning on mismatch with the matrix-derived set, never error. Invariant #14 is the anchor for O-3.

A parallel allowlist `fixtures/vectors/_pending-projection-drills.toml` tracks `TR-OP-*` rows not yet covered by a fixture. Same mechanics as `_pending-invariants.toml` (F5): fail-closed once emptied, drives batched authoring to zero.

**Why `projection-rebuild-drill` and not `test-vector` in the matrix `Verification` column.** These fixtures *are* test vectors in a generalized sense, but they execute a drill (replay + rebuild + cascade simulation), not a pure byte-compare. Keeping the matrix's existing verification taxonomy honest — `test-vector` = byte-compare, `projection-rebuild-drill` = this design — preserves the G-2 non-byte-testable audit channel signal. An alternative is to broaden `test-vector` to cover both; rejected because it collapses a useful distinction and makes the G-3 lint's "byte-testable coverage" count meaningless.

## Core/Companion gaps surfaced

These surfaced while working out the runner contracts. Flagged for review, not fixed here.

1. **Rebuild determinism is under-specified.** Core §15.3 says `rebuild_path` is "implementation-defined"; Companion §15.3 OC-39 requires "semantically equivalent output" for declared-deterministic fields. Neither pins a canonical encoding for the rebuilt artifact at byte level. Without that pin, two conforming implementations could produce byte-different rebuilt views that both satisfy §15.3 (e.g., map-key order, optional-field presence). For O-3 fixtures to be implementation-portable, either (a) Core §15.3 must require dCBOR canonical encoding for the rebuilt artifact (consistent with Core §5), or (b) the rebuilt-view byte-match is restricted to a declared canonical projection in Companion §15. Recommendation: option (a) — extend Core §15.3 with a one-sentence requirement that rebuilt artifacts use the same canonical encoding (dCBOR) as the chain they replay. Minor spec edit; unblocks byte-match across implementations.

2. **Snapshot cadence has no matrix row.** `TR-OP-006` covers "Projection conformance tests MUST validate watermark presence and stale-status behavior" — it does not name cadence. Companion §16.2 OC-46 ("declared cadence … comparable to observed behavior … absent snapshots … is a conformance violation") lacks a matching `TR-OP-*` row. Recommendation: add a new `TR-OP-NNN` row for OC-46 with `Verification = projection-rebuild-drill`, invariant #14. Needed to give Test 3 (cadence) a matrix anchor for the coverage lint. Tracked as a matrix-consistency follow-on.

3. **Cascade scope enumeration is not machine-checkable.** Companion §20.5 OC-77 lists six cascade-scope classes in prose. The fixture's `[procedure].cascade_scope` field uses these names but there's no canonical vocabulary — different implementations could use different spellings. Recommendation: promote the §20.5 list to a named enumeration in Companion Appendix A (Declaration Template) and reference it from `[procedure].cascade_scope`. Lint-enforceable once enumerated.

## Non-goals

- Authoring the O-3 fixtures. Tracked as "Author O-3 fixtures (M)" in `TODO.md` Stream B.
- O-4 declaration-doc conformance (Stream C) — declaration-doc-check is a different audit channel.
- O-5 posture-transition events (Stream D) — posture transitions are canonical events, tested via `append/` G-3 vectors, not projection drills.
- Companion §27.5 (Metadata-Budget Compliance), §27.6 (Auditor Workflow), §27.7 (Transition-Auditability), §27.8 (Idempotency Replay) — each has its own testing discipline and belongs to a separate design.
- Integrity sampling cadence (§15.5 OC-42 / OC-43) — sampling is an *operational* policy surface, not a byte/drill check. Conformance here is "declared policy exists and meets §15.5"; belongs to declaration-doc-check, not to O-3 fixtures.

## Open items

- **Op name.** `projection` vs `projection-rebuild` vs `op-projection`. Favor `projection` for brevity; formalize at fixture-authoring time.
- **Shred op name.** `shred` vs `purge-cascade` vs `erasure`. Favor `shred` to match Companion §20.3 / §27.3 "crypto-shred" naming. Formalize at authoring.
- **Subtype field.** Test 1 and Test 2 share `op = "projection"` but are distinct runner shapes. Options: (a) introduce `subtype = "watermark" | "rebuild" | "cadence"` in the manifest, (b) split into three ops (`projection-watermark`, `projection-rebuild`, `projection-cadence`). (a) is terser; (b) is more honest about dispatch. Decide at fixture-authoring when the first vector is concrete enough to exercise the choice.
- **Chain encoding.** Whether `chain/NNN-event.cbor` stores raw `Event` records or `SignedEnvelope` COSE_Sign1 outputs. The §18 export layout stores COSE_Sign1 records; consistency argues for the same. Formalize at authoring; the G-3 `append/` vectors already pin the choice.

## Follow-ons for the orchestrator

1. **Author O-3 fixtures batch** — Stream B "Author O-3 fixtures (M)". Produces ~6–8 fixtures: two watermark (inc. one staff-view subtype), two rebuild-equivalence (positive + non-deterministic-field split), two cadence (one positive per kind, one negative), two shred (in-scope cascade + backup-refusal). Each fixture a separate sub-plan.
2. **Lint additions to `scripts/check-specs.py`** — add `tr_op` coverage rule, `companion_sections` lint-verification, `_pending-projection-drills.toml` allowlist, op-dispatch for `projection` / `shred`. Coordinate with the ongoing `check-specs.py` work flagged in the G-3 design's Follow-ons ("Another agent is extending `check-specs.py` concurrently — do not modify it as part of this amendment").
3. **Conformance-runner extension** — when the G-4 Rust impl grows an operational-tier runner, it consumes these fixtures. Runner is per-impl; this design does not normatize it, same as G-3.
4. **Matrix follow-on** — add the missing cadence row (gap #2 above) and cascade-scope enumeration (gap #3 above). Coordinate with the Stream A audit-path assignment so these land together.
5. **Core §15.3 canonical-encoding tightening** — gap #1. Small spec edit. Unblocks cross-implementation rebuild byte-match. Tracked as a spec amendment, not a fixture work item.

## Ambiguities flagged for review before downstream citation

- **Cadence obligation shape.** Test 3's pass/fail depends on "no gaps under declared cadence." Companion §16.2 says "Absent snapshots where the cadence requires them is a conformance violation." That reads clearly, but the fixture-negative variant (deliberate gap → runner must detect) tests a second-order property: "the implementation can detect its own cadence violations." If the Auditor rather than the implementation is the detector, the test shape changes. Recommend confirming with the spec author that implementations owe self-detection, not just Auditors.
- **Backup-refusal runner shape.** Test 4's backup variant simulates a recovery attempt and asserts refusal. Companion §16.5 / §20.5 say backups "MUST NOT be used to resurrect destroyed plaintext," which is a policy statement, not an API contract. Whether this is testable at fixture level or belongs to declaration-doc-check is a judgment call. Recommend validating with the Stream C design before the Test 4 backup fixture is authored.
- **`projection_schema_id` semantics.** Core §15.2's `Watermark` CDDL does not include `projection_schema_id`; Companion §14.1 does (as field #5). The gap is intentional — the Core record is the byte-level core, the Companion record extends it. But Test 1's runner has to compare the Companion view's watermark *including* `projection_schema_id`. Clarify that the view artifact's watermark is a superset of the Core `Watermark` CDDL when appearing in a Companion-conformant projection. Possibly a third Core/Companion gap worth surfacing.
