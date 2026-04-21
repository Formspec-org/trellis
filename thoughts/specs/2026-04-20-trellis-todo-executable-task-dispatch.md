# Trellis TODO — Executable Task Dispatch

**Date:** 2026-04-20
**Scope:** Explode [`TODO.md`](../../TODO.md) into concrete tasks that can be
dispatched without re-deriving the backlog each session.
**Rule:** `TODO.md` remains the tactical index. This doc is the executable
breakdown.

## Status legend

- **COMPLETED** — closed in the current repo state.
- **IN-PROGRESS** — partially landed; follow-on work remains in the same row.
- **READY-NOW** — can be executed immediately in the current repo state.
- **READY-AFTER-GATE** — blocked only on acceptance of
  [`2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md`](2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md).
- **BLOCKED-EXTERNAL** — requires owner validation, sibling-repo coordination,
  or a genuinely independent implementor.
- **NOT-HONEST-FOR-THIS-AGENT** — this thread has already read material that
  would contaminate the task's independence claim.

## Gating reality

This was true when the dispatch doc was first written. Gate 0 is now closed:
the principles/ADR doc is accepted, `vision-model.md` is synced to it, and
`TODO.md` points at the accepted posture. Everything still blocked below is
blocked for its own dependency reasons, not because the architecture is open.

## Gate 0 — principles/ADR validation

| ID | Status | Task | Acceptance |
|---|---|---|---|
| G0-01 | COMPLETED | Owner validates Principles 1-7 in [`2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md`](2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md). | Checklist item 1 ticked. |
| G0-02 | COMPLETED | Owner validates ADR 0001 (DAG envelope, Phase-1 length-1 runtime). | Checklist item 2 ticked. |
| G0-03 | COMPLETED | Owner validates ADR 0002 (list-form anchors, single-anchor default semantics). | Checklist item 3 ticked. |
| G0-04 | COMPLETED | Owner validates ADR 0003 (§22/§24 reservations, Phase-1 MUST-NOT-populate). | Checklist item 4 ticked. |
| G0-05 | COMPLETED | Owner validates ADR 0004 (Rust byte authority, Python retained as cross-check). | Checklist item 5 ticked. |
| G0-06 | COMPLETED | Owner validates all ADR re-open triggers. | Checklist item 6 ticked. |
| G0-07 | COMPLETED | Flip the principles doc from `Draft` to `Accepted`; tick the checklist; tighten any wording changed during validation. | Accepted status and checklist landed. |
| G0-08 | COMPLETED | Update [`/Users/mikewolfd/Work/formspec/.claude/vision-model.md`](/Users/mikewolfd/Work/formspec/.claude/vision-model.md) so Trellis no longer contradicts the accepted principles doc. | Trellis section reflects the accepted posture. |
| G0-09 | COMPLETED | Collapse `TODO.md`'s gate pointers to the accepted doc so future sessions do not re-litigate the decision. | `TODO.md` points at the accepted principles doc as the authority. |
| G0-10 | COMPLETED | Re-run repo validation after the doc edits. | `python3 scripts/check-specs.py` passes; script pytest stayed green earlier in the session. |

## Stream 1 — G-4 Rust reference implementation

### Milestone 1 — `append/001` byte match

| ID | Status | Task | Acceptance |
|---|---|---|---|
| R1-01 | COMPLETED | Scaffold the Cargo workspace root and crate directories named in [`2026-04-18-trellis-g4-rust-workspace-plan.md`](2026-04-18-trellis-g4-rust-workspace-plan.md). | Crate set exists and the Trellis packages compile/test. |
| R1-02 | COMPLETED | Add workspace manifests and minimal compile targets for `trellis-core`, `trellis-cose`, `trellis-store-memory`, `trellis-store-postgres`, `trellis-verify`, `trellis-cli`, and `trellis-conformance`. | The Trellis package set compiles and tests cleanly. |
| R1-03 | COMPLETED | Implement `trellis-types` minimal structs/constants required by `append/001`. | Types/constants are live and used by downstream crates. |
| R1-04 | COMPLETED | Implement `trellis-cddl` minimal dCBOR encode/decode for the `append/001` event payload. | A round-trip test re-encodes `expected-event-payload.cbor` byte-identically. |
| R1-05 | COMPLETED | Add a fixed-point/property test for the minimal payload encoder. | Property test passes under `cargo test -p trellis-cddl`. |
| R1-06 | COMPLETED | Implement `trellis-cose` Sig_structure construction for the pinned COSE profile. | `sig-structure.bin` matches the fixture bytes. |
| R1-07 | COMPLETED | Implement Ed25519 signing/verification for the minimal event path. | `expected-event.cbor` byte-matches the fixture. |
| R1-08 | COMPLETED | Export the Core §9.8 domain-tag constants from Rust. | Constants exist and are referenced by hashing code/tests. |
| R1-09 | COMPLETED | Implement author-event hash preimage construction and digest calculation. | `author-event-hash.bin` matches the fixture. |
| R1-10 | COMPLETED | Define the `LedgerStore` seam in `trellis-core` and implement `trellis-store-memory`. | `append` persists and retrieves the single-event scope in memory. |
| R1-11 | COMPLETED | Implement the `append` skeleton for `append/001`. | Returned `AppendHead` and stored canonical bytes match the fixture. |
| R1-12 | COMPLETED | Implement the minimal `trellis-conformance` runner that executes `append/001`. | `cargo test -p trellis-conformance` passes on `append/001`. |
| R1-13 | COMPLETED | Implement the minimal single-event happy-path verifier. | `VerificationReport` returns all three booleans true for `append/001`. |
| R1-14 | COMPLETED | Implement deterministic ZIP export logic with a reproducibility test. | Two serializations of the same logical package are byte-identical. |
| R1-15 | COMPLETED | Add a CLI that exercises `append`, `verify`, and `export` for the current fixture path. | CLI commands run and report real artifact sizes / verification booleans. |
| R1-16 | COMPLETED | Add a Postgres adapter seam with real backend behavior or surface the blocker explicitly. | `trellis-store-postgres` now has a real schema/init/append/read path with a temporary-cluster integration test. |

### Milestone 2 — full committed corpus parity

| ID | Status | Task | Acceptance |
|---|---|---|---|
| R2-01 | COMPLETED | Extend `trellis-conformance` to walk the committed `append/` corpus from manifest data. | Runner discovers and checks every committed append fixture directory. |
| R2-02 | COMPLETED | Add Rust support for `append/002-rotation-signing-key`. | Vector passes in `cargo test -p trellis-conformance`. |
| R2-03 | COMPLETED | Add Rust support for `append/003-external-payload-ref`. | Vector passes. |
| R2-04 | COMPLETED | Add Rust support for `append/004-hpke-wrapped-inline`. | Vector passes. |
| R2-05 | COMPLETED | Add Rust support for `append/005-prior-head-chain`. | Vector passes. |
| R2-06 | COMPLETED | Add Rust support for `append/006-custody-transition-cm-b-to-cm-a`. | Vector passes. |
| R2-07 | COMPLETED | Add Rust support for `append/007-custody-transition-cm-c-narrowing`. | Vector passes. |
| R2-08 | COMPLETED | Add Rust support for `append/008-disclosure-profile-transition-a-to-b`. | Vector passes. |
| R2-09 | COMPLETED | Add Rust support for `append/009-signing-key-revocation`. | Vector passes. |
| R2-10 | COMPLETED | Add Rust support for `tamper/001-signature-flip`. | Vector passes. |
| R2-11 | COMPLETED | Add Rust support for `tamper/002-transition-from-mismatch`. | Vector passes. |
| R2-12 | COMPLETED | Add Rust support for `tamper/003-transition-missing-dual-attestation`. | Vector passes. |
| R2-13 | COMPLETED | Add Rust support for `tamper/004-transition-declaration-digest-mismatch`. | Vector passes. |
| R2-14 | COMPLETED | Add Rust support for `tamper/005-chain-truncation`. | Vector passes. |
| R2-15 | COMPLETED | Add Rust support for `tamper/006-event-reorder`. | Vector passes. |
| R2-16 | COMPLETED | Add Rust support for `tamper/007-hash-mismatch`. | Vector passes. |
| R2-17 | COMPLETED | Add Rust support for `tamper/008-malformed-cose`. | Vector passes. |
| R2-18 | COMPLETED | Add Rust support for `projection/001-watermark-attestation`. | Vector passes. |
| R2-19 | COMPLETED | Add Rust support for `projection/002-rebuild-equivalence-minimal`. | Vector passes. |
| R2-20 | COMPLETED | Add Rust support for `projection/003-cadence-positive-height`. | Vector passes. |
| R2-21 | COMPLETED | Add Rust support for `projection/004-cadence-gap`. | Vector passes. |
| R2-22 | COMPLETED | Add Rust support for `projection/005-watermark-staff-view-decision-binding`. | Vector passes. |
| R2-23 | COMPLETED | Add Rust support for `shred/001-purge-cascade-minimal`. | Vector passes. |
| R2-24 | COMPLETED | Add Rust support for `shred/002-backup-refusal`. | Vector passes. |
| R2-25 | COMPLETED | Add Rust support for `export/001-two-event-chain`. | Vector passes. |
| R2-26 | COMPLETED | Add Rust support for `verify/001-export-001-two-event-chain`. | Vector passes. |
| R2-27 | COMPLETED | Add Rust support for `verify/002-export-001-manifest-sigflip`. | Vector passes. |
| R2-28 | COMPLETED | Add Rust support for `verify/003-export-001-missing-registry-snapshot`. | Vector passes. |
| R2-29 | COMPLETED | Add Rust support for `verify/004-export-001-unsupported-suite`. | Vector passes. |
| R2-30 | COMPLETED | Add Rust support for `verify/005-export-001-unresolvable-manifest-kid`. | Vector passes. |
| R2-31 | COMPLETED | Add Rust support for `verify/006-export-001-checkpoint-root-mismatch`. | Vector passes. |
| R2-32 | COMPLETED | Add Rust support for `verify/007-export-001-inclusion-proof-mismatch`. | Vector passes. |

## Stream 2 — G-5 stranger implementation

| ID | Status | Task | Acceptance |
|---|---|---|---|
| S2-01 | NOT-HONEST-FOR-THIS-AGENT | Implement the stranger test in this thread. | Do not do this here; the thread has already read plans, generators, and repo internals. |
| S2-02 | READY-AFTER-GATE | Draft a clean commission brief for an independent `trellis-py` or `trellis-go` implementor that names allowed and forbidden inputs. | Brief exists and cites only the three normative specs plus the vector corpus. |
| S2-03 | BLOCKED-EXTERNAL | Commission the independent implementor. | A genuinely separate implementor accepts the brief. |
| S2-04 | BLOCKED-EXTERNAL | Run the stranger implementation against the corpus and capture byte-match evidence. | G-5 evidence is recorded in `ratification/ratification-checklist.md`. |

## Stream 3 — vector authoring still missing from G-3 breadth

| ID | Status | Task | Acceptance |
|---|---|---|---|
| V3-01 | READY-AFTER-GATE | Pin Core §19 language for Revoked/`valid_to` enforcement before authoring the corresponding verify vectors. | Spec prose lands; no ambiguity remains about the check point. |
| V3-02 | READY-AFTER-GATE | Author a verify vector for the remaining §19 step-4 event-level negative-non-tamper obligation. | New `verify/` fixture lands with derivation and claims. |
| V3-03 | READY-AFTER-GATE | Author a verify vector for §19 step 5.d `prev_checkpoint_hash` mismatch. | New `verify/` fixture lands and lints clean. |
| V3-04 | READY-AFTER-GATE | Author a verify vector for §19 step 5.e consistency-proof mismatch between non-head checkpoints. | New `verify/` fixture lands and lints clean. |
| V3-05 | READY-AFTER-GATE | Author a verify vector for §19 step 6 posture-transition verification in the non-tamper path. | New `verify/` fixture lands and lints clean. |
| V3-06 | READY-AFTER-GATE | Author a verify vector for §19 step 8 external-anchor handling. | New `verify/` fixture lands and lints clean. |
| V3-07 | READY-AFTER-GATE | Expand the `export/` suite for ZIP determinism edge cases. | At least one new `export/` vector lands with deterministic ZIP evidence. |
| V3-08 | READY-AFTER-GATE | Expand the `export/` suite for manifest-variant coverage. | At least one new `export/` vector lands with claims/derivation. |
| V3-09 | READY-AFTER-GATE | Expand the `export/` suite for key-material handling. | At least one new `export/` vector lands with claims/derivation. |
| V3-10 | READY-AFTER-GATE | Expand the `export/` suite for larger inclusion/consistency-proof sets. | At least one new `export/` vector lands with claims/derivation. |
| V3-11 | READY-AFTER-GATE | Author `tamper/prev_hash_break` (mutated bytes + re-sign). | New `tamper/` vector lands and lints clean. |
| V3-12 | READY-AFTER-GATE | Author `tamper/missing_head` (checkpoint-aware). | New `tamper/` vector lands and lints clean. |
| V3-13 | READY-AFTER-GATE | Author `tamper/wrong_scope`. | New `tamper/` vector lands and lints clean. |
| V3-14 | READY-AFTER-GATE | Author `tamper/registry_snapshot_swap`. | New `tamper/` vector lands and lints clean. |
| V3-15 | READY-AFTER-GATE | After each new vector batch, update `ratification/ratification-checklist.md` G-3 evidence text. | G-3 evidence block names the newly landed vectors. |

## Stream 4 — Respondent Ledger ↔ Trellis binding

| ID | Status | Task | Acceptance |
|---|---|---|---|
| B4-01 | BLOCKED-EXTERNAL | Confirm with Formspec that Respondent Ledger §6.2 `eventHash` / `priorEventHash` become MUST when Trellis-wrapped. | Cross-repo agreement exists. |
| B4-02 | READY-AFTER-GATE | Draft the Trellis-side spec amendment that references the Formspec MUST promotion. | Trellis prose patch is ready or landed. |
| B4-03 | BLOCKED-EXTERNAL | Land the Formspec-side MUST promotion. | Formspec spec change is committed in the sibling repo. |
| B4-04 | READY-AFTER-GATE | Add Trellis-side conformance/lint coverage for the promoted requirement once Formspec lands it. | New or amended checks fail when the fields are absent. |
| B4-05 | BLOCKED-EXTERNAL | Phase-4 only: define the semantic contents of Core §22 case ledger and §24 agency log. | Separate Phase-4 design brief exists; not Phase-1 work. |

## Stream 5 — G-2 model-check flush

| ID | Status | Task | Acceptance |
|---|---|---|---|
| M5-01 | COMPLETED | Create `thoughts/model-checks/evidence.toml` as the evidence registry consumed by lint rule R8. | File exists and lint resolves it. |
| M5-02 | COMPLETED | Add model-check evidence for `TR-CORE-001` (Canonical Append Contract). | Row removed from `_pending-model-checks.toml`; evidence file points at a real artifact. |
| M5-03 | COMPLETED | Add model-check evidence for `TR-CORE-020` (single canonical order per governed scope). | Row removed from allowlist; evidence recorded. |
| M5-04 | COMPLETED | Add model-check evidence for `TR-CORE-023` (order independent of operational accidents). | Row removed from allowlist; evidence recorded. |
| M5-05 | COMPLETED | Add model-check evidence for `TR-CORE-025` (deterministic tie-breaking under concurrency). | Row removed from allowlist; evidence recorded. |
| M5-06 | COMPLETED | Add model-check evidence for `TR-CORE-046` (no append attestation before prerequisites hold). | Row removed from allowlist; evidence recorded. |
| M5-07 | COMPLETED | Add model-check evidence for `TR-CORE-050` (idempotency key semantics). | Row removed from allowlist; evidence recorded. |
| M5-08 | COMPLETED | Add model-check evidence for `TR-OP-061` (conflict handling scoped to affected facts/scope). | Row removed from allowlist; evidence recorded. |
| M5-09 | COMPLETED | Add model-check evidence for `TR-OP-111` (operational testing guidance exercised concretely). | Row removed from allowlist; evidence recorded. |
| M5-10 | COMPLETED | Empty [`fixtures/vectors/_pending-model-checks.toml`](../../fixtures/vectors/_pending-model-checks.toml). | Allowlist is empty and lint stays green. |

## Stream 6 — WOS `custodyHook` joint ADR

| ID | Status | Task | Acceptance |
|---|---|---|---|
| C6-01 | BLOCKED-EXTERNAL | Confirm the joint-design boundary with WOS so Trellis does not invent WOS-side primitives. | Cross-submodule scope is agreed. |
| C6-02 | READY-AFTER-GATE | Draft the Trellis-side half of the joint ADR: envelope composition, hash surface, and anchor-target shape. | Trellis draft exists with explicit open questions. |
| C6-03 | BLOCKED-EXTERNAL | Draft the WOS-side half: recordKind/lifecycle-reference semantics. | WOS draft exists in the sibling repo. |
| C6-04 | BLOCKED-EXTERNAL | Land a mirrored ADR in both repos and link it from each TODO. | Same wire-format ADR is committed in both submodules. |

## Stream 7 — O-gate close-out tasks

| ID | Status | Task | Acceptance |
|---|---|---|---|
| O7-01 | COMPLETED | Author O-3 fixture inputs under `fixtures/vectors/_inputs/projection/` once the Rust conformance path exists. | Inputs are committed and consumed by the Rust runner. |
| O7-02 | READY-AFTER-GATE | Land O-4 declaration docs per Companion §19 for every delegated-compute deployment fixture. | Declaration docs exist and pass the current validator. |
| O7-03 | READY-AFTER-GATE | Author the remaining O-5 canonical events for custody/disclosure posture changes if new gaps remain beyond `append/006..008` and `tamper/002..004`. | O-5 gap list is empty. |
| O7-04 | READY-AFTER-GATE | Record evidence SHAs for O-3 in [`ratification/ratification-checklist.md`](../../ratification/ratification-checklist.md). | O-3 gate has concrete evidence pointers. |
| O7-05 | READY-AFTER-GATE | Record evidence SHAs for O-4 in [`ratification/ratification-checklist.md`](../../ratification/ratification-checklist.md). | O-4 gate has concrete evidence pointers. |
| O7-06 | READY-AFTER-GATE | Record evidence SHAs for O-5 in [`ratification/ratification-checklist.md`](../../ratification/ratification-checklist.md). | O-5 gate has concrete evidence pointers. |

## Ratification close-out

| ID | Status | Task | Acceptance |
|---|---|---|---|
| Z-01 | READY-AFTER-GATE | When all seven gates flip, update the ratification checklist with final evidence SHAs. | Checklist is the evidence-of-record with no placeholders. |
| Z-02 | READY-AFTER-GATE | Strike `(Draft)` from Core and Companion titles. | Normative docs reflect ratified status. |
| Z-03 | READY-AFTER-GATE | Cut the version tag. | Version tag exists and matches the ratified surface. |

## Suggested remaining execution order

1. `G0-07` through `G0-10` — make the accepted architecture authoritative.
2. `R1-01` through `R1-16` — establish the Rust workspace and first byte match.
3. `R2-01` through `R2-32` in fixture-directory order — close G-4 against the committed corpus.
4. `V3-01` through `V3-15` — finish the residual G-3 breadth that the current corpus still lacks.
5. `O7-01` through `O7-06` — close the remaining O-gates.
6. `Z-01` through `Z-03` — ratify.

## Deliberate non-dispatches

- Do not dispatch G-5 implementation work to this thread. The stranger test
  requires a cleaner epistemic boundary than this session can honestly claim.
- Do not treat Phase-4 §22/§24 substance as Phase-1 work. The reservation
  decision may land now; the semantics do not.
- Do not start Rust code before the principles/ADR gate is accepted or
  redirected. Otherwise the workspace risks calcifying around a decision the
  project still marks as open.
