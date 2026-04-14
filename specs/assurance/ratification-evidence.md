# Trellis Ratification Evidence Registry (Draft)

## Purpose

Provide evidence references for each completed gate in `specs/assurance/ratification-checklist.md`.

## Evidence status key

- `PROSE`: satisfied by normative text currently present in spec drafts.
- `PENDING-AUTO`: requires executable check artifact in follow-up implementation.

## Global gate evidence

| Gate ID | Gate | Status | Evidence |
|---|---|---|---|
| G-1 | Core/companion boundaries stable | PROSE | `specs/core/trellis-core.md` out-of-scope and cross-repo boundaries sections |
| G-2 | Trellis/Formspec/WOS boundaries explicit | PROSE | `specs/README.md` ownership boundaries + `core/trellis-core.md` cross-repo section |
| G-3 | MUST-level requirements mapped to traceability | PROSE | `specs/assurance/assurance-traceability.md` traceability matrix |
| G-4 | Native/WASM vectors exist | PENDING-AUTO | Required by `assurance/assurance-traceability.md` CI expectation #2 |
| G-5 | Offline verifier reproducibility across implementations | PENDING-AUTO | Required by `export/export-verification-package.md` cross-implementation requirement |

## Per-document gate evidence

| Gate ID | Document | Status | Evidence |
|---|---|---|---|
| C-1 | `core/trellis-core.md` append/order/hash/idempotency gates | PROSE | In-scope normative section + core invariants |
| B-1 | `core/shared-ledger-binding.md` field/version/rejection gates | PROSE | family matrix + compatibility policy + rejection codes |
| T-1 | `trust/trust-profiles.md` declarations/metadata/honesty gates | PROSE | declaration schema + honesty rule + audit hooks |
| K-1 | `trust/key-lifecycle-operating-model.md` transition/grace/recovery gates | PROSE | lifecycle states + grace-period rule + evidence requirements |
| P-1 | `projection/projection-runtime-discipline.md` watermark/rebuild/purge gates | PROSE | watermark contract + rebuild verification + purge rule |
| E-1 | `export/export-verification-package.md` manifest/readability/offline gates | PROSE | manifest minimums + readability declaration + cross-implementation requirement |
| D-1 | `export/disclosure-manifest.md` taxonomy/provenance/no-rewrite gates | PROSE | claim-class taxonomy + canonical-reference no-rewrite rule |
| M-1 | `operations/monitoring-witnessing.md` publication/growth/anti-equivocation gates | PROSE | normative seam focus + testability hooks |
| A-1 | `assurance/assurance-traceability.md` invariant mapping/retention/cadence gates | PROSE | traceability matrix + minimum CI expectations + retention policy |

## Follow-up required to move PENDING-AUTO to complete

1. Add shared native/WASM vector fixtures for canonical hash/serialization checks.
2. Add at least two verifier implementations against shared export package fixtures.
3. Update this registry with artifact paths, run IDs, and dates.
