# Task 10 — Removed ULCR rows

Working file consumed by Task 14 (cross-reference-map). Delete after Task 14 completes.

Removed from `unified-ledger-requirements-matrix.md` in Plan 3 (2026-04-15). These IDs are permanently retired per §4.1 "IDs are stable across revisions"; removed IDs are not reused.

| ULCR-ID | Concept | Upstream home |
|---|---|---|
| ULCR-063 | Disclosure posture vs assurance level taxonomy (MUST distinguish; MUST NOT require identity disclosure for higher assurance; MAY support subject continuity) | [WOS Assurance §2 assurance taxonomy], [WOS Assurance §4 Invariant 6], [WOS Assurance §3 subject continuity], [Formspec Respondent Ledger §6.6.1 assuranceLevel], [Formspec Respondent Ledger §6.6A Subject Continuity] |
| ULCR-080 | User-Held Record Reuse Profile (reusable prior records, binding reused content into canonical truth) | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCR-081 | Respondent History Profile (respondent-originated/visible history, timelines as derived artifacts) | [Formspec Respondent Ledger §6.6A, §6.7] |
| ULCR-112 | Legacy Invariant 6 — Disclosure posture and assurance posture MUST remain distinct and MUST NOT be conflated | [WOS Assurance §4 Invariant 6] |

## Rescoped rows (kept in matrix, scope narrowed)

| ULCR-ID | Concept (narrowed to ledger-specific) | Upstream home for the generic portion |
|---|---|---|
| ULCR-091 | Cryptographic lifecycle facts — narrowed to ledger-specific cryptographic operations (key destruction, export issuance). Generic lifecycle operations (retention, legal hold, archival, sealing, schema upgrade) delegated upstream. | [WOS Governance §2.9 Schema Upgrade], [WOS Governance §7.15 Legal Hold] |
