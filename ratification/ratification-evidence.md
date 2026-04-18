# Trellis Ratification Evidence Registry (Draft)

## Purpose

Provide evidence references for each gate in [`ratification-checklist.md`](ratification-checklist.md).

## Evidence status key

- `PROSE`: satisfied by normative text currently present in Core or the Operational Companion.
- `PENDING-HANDOFF`: satisfied once the corresponding handoff task in [`../thoughts/specs/2026-04-17-trellis-normalization-handoff.md`](../thoughts/specs/2026-04-17-trellis-normalization-handoff.md) is closed.
- `PENDING-AUTO`: requires an executable check artifact (fixtures, CI run, reference-impl binary) before it can land.

## Global gate evidence

| Gate | Description | Status | Evidence / dependency |
|---|---|---|---|
| G-1 | Handoff complete | PENDING-HANDOFF | Handoff Groups A–D. |
| G-2 | Invariant coverage in Core + Matrix | PENDING-HANDOFF | Matrix rows for invariants #1–#15; Core prose MUST statements. Partial today; audited under handoff task 12. |
| G-3 | ~50 byte-exact test vectors | PENDING-AUTO | `fixtures/vectors/{append,verify,export,tamper}/` — not yet created. Required by Core §27. |
| G-4 | Reference implementation passes | PENDING-AUTO | `trellis-*` Rust crates — not yet created. Per product-vision Track A step 7. |
| G-5 | Second implementation byte-matches | PENDING-AUTO | Independent `trellis-py` or `trellis-go`. Per product-vision Track A step 9. |
| G-6 | Lint clean | PENDING-AUTO | `python3 scripts/check-specs.py` on current tree — status depends on handoff closure. |

## Per-document gate evidence

### `../specs/trellis-core.md`

| Gate | Description | Status | Evidence / dependency |
|---|---|---|---|
| C-1 | COSE_Sign1 signature model | PENDING-HANDOFF | Handoff Group A task 1. Currently uses custom signature-zeroing (lines 252, 297–303, 403, 542, 654, 893). |
| C-2 | Explicit hash preimages | PENDING-HANDOFF | Handoff Group A task 2. |
| C-3 | Tagged payload references + 3-way verifier output | PENDING-HANDOFF | Handoff Group A task 3. |
| C-4 | Deterministic export ZIP | PENDING-HANDOFF | Handoff Group A task 4. |
| C-5 | Strict-superset semantics | PENDING-HANDOFF | Handoff Group A task 5. |
| C-6 | Idempotency identity scope-permanent | PENDING-HANDOFF | Handoff Group A task 6. Core §17 partial today. |
| C-7 | Agency-log extension points | PENDING-HANDOFF | Handoff Group A task 7. Core §24 may need §11 CDDL reservation. |
| C-8 | Profile/Custody/Conformance-Class vocabulary | PROSE (partial) | Core §21. Final audit under handoff task 11. |

### `../specs/trellis-operational-companion.md`

| Gate | Description | Status | Evidence / dependency |
|---|---|---|---|
| O-1 | Core §N references resolve | PENDING-HANDOFF | Handoff Group B task 10. Known stale refs: §15 (should be §13), §12 (should be §18), §9 (should be §19), Canonical Append Service (§2, not §7). |
| O-2 | Custody-model identifier set unified | PENDING-HANDOFF | Handoff Group C task 11. |
| O-3 | Projection discipline testable | PROSE (partial) | Companion §§14–17. Conformance fixtures pending (PENDING-AUTO). |
| O-4 | Delegated-compute declarations | PROSE | Companion §19. |
| O-5 | Posture-transition auditability | PROSE | Companion §10. |

### `../specs/trellis-requirements-matrix.md`

| Gate | Description | Status | Evidence / dependency |
|---|---|---|---|
| M-1 | Factual consistency with Core (JCS → dCBOR + row coverage) | PENDING-HANDOFF | Handoff Group C task 13. TR-CORE-032 specifies JCS; must become dCBOR. Cross-reference coverage is handoff task 8. |
| M-2 | Gap-log soundness | PENDING-HANDOFF | Handoff Group C task 12. 23 gap-log entries to audit against invariants #1–#15. |
| M-3 | Invariant coverage | PROSE (partial) | Matrix §3. Full audit under handoff task 12. |

## Follow-up to move gates to complete

1. Close handoff Group A (byte-protocol repair) — cascades into C-1 through C-7, G-2, G-3 design decisions.
2. Close handoff Group B (document hygiene) — closes O-1.
3. Close handoff Group C (vocabulary/traceability) — closes C-8, O-2, M-1, M-2, M-3.
4. Close handoff Group D (automation) — closes G-6 and enables routine re-verification.
5. Create `fixtures/vectors/` hierarchy per Core §27 — closes G-3.
6. Land `trellis-*` Rust reference implementation — closes G-4.
7. Commission independent second implementation — closes G-5.
8. Update this registry with artifact paths, run IDs, and dates as each gate lands.
