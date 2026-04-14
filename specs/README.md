# Trellis Specs (Draft Family)

This folder is the active normalization target for Trellis specifications.

## Normative dependency order (draft)

1. `trellis-core.md`
2. `shared-ledger-binding.md`
3. `trust-profiles.md`
4. `key-lifecycle-operating-model.md`
5. `projection-runtime-discipline.md`
6. `export-verification-package.md`
7. `disclosure-manifest.md`
8. `monitoring-witnessing.md`
9. `assurance-traceability.md`
10. `ratification-checklist.md`
11. `ratification-evidence.md`

## Ownership boundaries (draft)

- **Formspec owns:** authored/spec/runtime semantics for form responses.
- **WOS owns:** workflow/governance meaning and runtime envelope.
- **Trellis owns:** canonical ledger semantics, append/attestation, trust/custody/disclosure semantics, export/verifier semantics, and cross-system projection discipline.

## Current maturity markers

- `trellis-core.md`: constitutional scope + core invariants/conformance roles draft
- `shared-ledger-binding.md`: family-binding structure + canonization matrix draft
- `trust-profiles.md`: trust posture + metadata budget + honesty rule draft
- `key-lifecycle-operating-model.md`: lifecycle state-machine + grace-period draft
- `projection-runtime-discipline.md`: derived-system boundary + watermark contract draft
- `export-verification-package.md`: offline verification package + manifest minimums draft
- `disclosure-manifest.md`: selective disclosure release draft
- `monitoring-witnessing.md`: seam-only minimal draft
- `assurance-traceability.md`: invariant-to-assurance mapping draft
- `ratification-checklist.md`: draft readiness/risk gates for ratification
- `ratification-evidence.md`: gate-to-evidence registry with pending automation markers

## Immediate next extraction passes

1. Pull precise MUST/SHALL language from `DRAFTS/unified_ledger_core.md` into `trellis-core.md`.
2. Pull companion-grade normative language from `DRAFTS/unified_ledger_companion.md` into each companion draft.
3. Expand assurance traceability matrix into executable checks and artifact retention policy.
4. Keep `ratification-checklist.md` updated as sections move from draft to normative-ready.

## Phase status

- Ratification checklist gates are now evidence-linked via `ratification-evidence.md`.
- Two global auto-evidence gates remain open: native/WASM vectors and cross-implementation verifier reproducibility.
- Initial migrations from `DRAFTS/unified_ledger_core.md` have been extracted into companion-specific “Migrated requirements” sections.
