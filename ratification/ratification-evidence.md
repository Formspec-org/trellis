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
| C-1 | COSE_Sign1 signature model | PROSE | Commit 3a143a1 — specs/trellis-core.md §7, §11, §18, §19, §29; no zero-fill prose remains. |
| C-2 | Explicit hash preimages | PROSE | Commit 3a143a1 — specs/trellis-core.md §9 defines explicit preimage structures with domain separation tags; ledger scope included in signed material. |
| C-3 | Tagged payload references + 3-way verifier output | PROSE | Commit 3a143a1 — specs/trellis-core.md §6 defines tagged PayloadRef / PayloadInline / PayloadExternal; verifier reports `structure_verified` / `integrity_verified` / `readability_verified` independently. |
| C-4 | Deterministic export ZIP | PROSE | Commit 3a143a1 — specs/trellis-core.md §18 pins deterministic ZIP construction (prefix-ordered filenames, pinned local-file-header fields). |
| C-5 | Strict-superset semantics | PROSE | Commit 3a143a1 — specs/trellis-core.md §6.7 + §24 define strict superset as reserved-extension preservation; Phase 1 verifiers reject unknown top-level fields. |
| C-6 | Idempotency identity scope-permanent | PROSE | Commit 3a143a1 — specs/trellis-core.md §17 defines scope-permanent idempotency (same key + same payload → same reference; same key + different payload → deterministic rejection). |
| C-7 | Agency-log extension points | PROSE | Commit 3a143a1 — specs/trellis-core.md §11 reserves CDDL extension slots referenced by §24. |
| C-8 | Profile/Custody/Conformance-Class vocabulary | PROSE | Commit 3a143a1 — specs/trellis-core.md §21 unifies the three-namespace vocabulary. |

### `../specs/trellis-operational-companion.md`

| Gate | Description | Status | Evidence / dependency |
|---|---|---|---|
| O-1 | Core §N references resolve | PROSE | Commit 3a143a1 — all Core §N references in specs/trellis-operational-companion.md resolve to current Core headings. |
| O-2 | Custody-model identifier set unified | PROSE | Commit 3a143a1 — custody-model identifiers unified at CM-A..CM-F across Core §21.3, Companion §9.2, Matrix §2.2/§4.3. |
| O-3 | Projection discipline testable | PROSE (partial) | Companion §§14–17. Conformance fixtures pending (PENDING-AUTO). |
| O-4 | Delegated-compute declarations | PROSE | Companion §19. |
| O-5 | Posture-transition auditability | PROSE | Companion §10. |

### `../specs/trellis-requirements-matrix.md`

| Gate | Description | Status | Evidence / dependency |
|---|---|---|---|
| M-1 | Factual consistency with Core (JCS → dCBOR + row coverage) | PROSE | Commit 3a143a1 — TR-CORE-032 now pins dCBOR; MUST-to-row coverage audited. |
| M-2 | Gap-log soundness | PROSE | Commit 3a143a1 — gap-log audit complete (1 reinstated, 13 corrected, 9 confirmed). |
| M-3 | Invariant coverage | PROSE | Commit 3a143a1 — Matrix §3.1 covers all 15 invariants; reviewer audit of table §3.1 done. |

## Follow-up to move gates to complete

1. Close handoff Group A (byte-protocol repair) — cascades into C-1 through C-7, G-2, G-3 design decisions.
2. Close handoff Group B (document hygiene) — closes O-1.
3. Close handoff Group C (vocabulary/traceability) — closes C-8, O-2, M-1, M-2, M-3.
4. Close handoff Group D (automation) — closes G-6 and enables routine re-verification.
5. Create `fixtures/vectors/` hierarchy per Core §27 — closes G-3.
6. Land `trellis-*` Rust reference implementation — closes G-4.
7. Commission independent second implementation — closes G-5.
8. Update this registry with artifact paths, run IDs, and dates as each gate lands.
