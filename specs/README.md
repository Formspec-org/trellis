# Trellis Specs

This folder contains the normalized Trellis specification set.

## Reading Order

Read the top-level documents first:

1. [`trellis-agreement.md`](trellis-agreement.md) — non-normative decision gate and product invariants.
2. [`trellis-core.md`](trellis-core.md) — normative Phase 1 byte protocol for append, verify, and export.
3. [`trellis-operational-companion.md`](trellis-operational-companion.md) — normative Phase 2+ operator obligations.
4. [`trellis-requirements-matrix.md`](trellis-requirements-matrix.md) — traceability matrix. Prose in Core and the Operational Companion wins on conflict.

## Normative Authority

Only `trellis-core.md` and `trellis-operational-companion.md` are normative prose specifications.

`trellis-agreement.md` is a sign-off gate for scope and invariants. It does not impose implementor conformance obligations.

`trellis-requirements-matrix.md` exists to preserve provenance from mined legacy requirements into the two normative specs. It is not a competing source of truth.

## Archived Inputs

The previous per-family spec drafts are superseded inputs and live under [`archive/`](archive/):

| Archived folder | Prior role |
|---|---|
| `archive/core/` | Constitutional core draft, shared-ledger binding, and legacy matrices. |
| `archive/trust/` | Trust posture and key-lifecycle drafts. |
| `archive/projection/` | Projection runtime discipline draft. |
| `archive/export/` | Export package and disclosure manifest drafts. |
| `archive/operations/` | Monitoring and witnessing draft. |
| `archive/forms/` | Formspec respondent-history draft. |
| `archive/workflow/` | Workflow governance draft. |
| `archive/assurance/` | Assurance traceability draft. |

Treat archived files as historical source material. Do not cite them as normative unless a top-level spec explicitly incorporates a specific requirement.

## Checks

Run the Trellis spec lint after editing these documents:

```bash
python3 scripts/check-specs.py
```

The check enforces stale Core-section references, forbidden legacy terms, requirement-ID sanity, and archived-input placement.
