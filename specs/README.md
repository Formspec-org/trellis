# Trellis Specs (Draft Family)

This folder is the active normalization target for Trellis specifications.

## Layout

| Folder | Role |
|--------|------|
| [`core/`](core/) | Constitutional core + family binding to Formspec/WOS (read first). |
| [`trust/`](trust/) | Trust profiles and key lifecycle operating model. |
| [`projection/`](projection/) | Derived projections, watermarking, rebuild and purge discipline. |
| [`export/`](export/) | Offline verification packages and selective disclosure manifests. |
| [`operations/`](operations/) | Monitoring and witnessing seams. |
| [`assurance/`](assurance/) | Invariant-to-assurance traceability matrix and CI expectations. |
| [`ratification/`](ratification/) | Readiness checklist and evidence registry for normative ratification. |

## Normative dependency order (draft)

Paths are relative to `specs/`.

1. `core/trellis-core.md`
2. `core/shared-ledger-binding.md`
3. `trust/trust-profiles.md`
4. `trust/key-lifecycle-operating-model.md`
5. `projection/projection-runtime-discipline.md`
6. `export/export-verification-package.md`
7. `export/disclosure-manifest.md`
8. `operations/monitoring-witnessing.md`
9. `assurance/assurance-traceability.md`
10. `ratification/ratification-checklist.md`
11. `ratification/ratification-evidence.md`

## Ownership boundaries (draft)

- **Formspec owns:** authored/spec/runtime semantics for form responses.
- **WOS owns:** workflow/governance meaning and runtime envelope.
- **Trellis owns:** canonical ledger semantics, append/attestation, trust/custody/disclosure semantics, export/verifier semantics, and cross-system projection discipline.

## Current maturity markers

- `core/trellis-core.md`: constitutional scope + core invariants/conformance roles draft
- `core/shared-ledger-binding.md`: family-binding structure + canonization matrix draft
- `trust/trust-profiles.md`: trust posture + metadata budget + honesty rule draft
- `trust/key-lifecycle-operating-model.md`: lifecycle state-machine + grace-period draft
- `projection/projection-runtime-discipline.md`: derived-system boundary + watermark contract draft
- `export/export-verification-package.md`: offline verification package + manifest minimums draft
- `export/disclosure-manifest.md`: selective disclosure release draft
- `operations/monitoring-witnessing.md`: seam-only minimal draft
- `assurance/assurance-traceability.md`: invariant-to-assurance mapping draft
- `ratification/ratification-checklist.md`: draft readiness/risk gates for ratification
- `ratification/ratification-evidence.md`: gate-to-evidence registry with pending automation markers

## Immediate next extraction passes

1. Pull precise MUST/SHALL language from `DRAFTS/unified_ledger_core.md` into `core/trellis-core.md`.
2. Pull companion-grade normative language from `DRAFTS/unified_ledger_companion.md` into each companion draft.
3. Expand assurance traceability matrix into executable checks and artifact retention policy.
4. Keep `ratification/ratification-checklist.md` updated as sections move from draft to normative-ready.

## Phase status

- Ratification checklist gates are now evidence-linked via `ratification/ratification-evidence.md`.
- Two global auto-evidence gates remain open: native/WASM vectors and cross-implementation verifier reproducibility.
- Initial migrations from `DRAFTS/unified_ledger_core.md` have been extracted into companion-specific “Migrated requirements” sections.
