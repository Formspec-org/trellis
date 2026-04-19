# Trellis ratification (draft)

Process gates and evidence for moving the two normative Trellis specs under [`../specs/`](../specs/) toward ratification. These documents are **not** protocol specs; they track readiness and proof artifacts.

Scope:

- [`../specs/trellis-core.md`](../specs/trellis-core.md) — Phase 1 byte protocol.
- [`../specs/trellis-operational-companion.md`](../specs/trellis-operational-companion.md) — Phase 2+ operator obligations.

Companion documents:

- [`../specs/trellis-agreement.md`](../specs/trellis-agreement.md) is non-normative and ratified by product-strategy sign-off (see §11 of that document). It is **not** in scope for these gates.
- [`../specs/trellis-requirements-matrix.md`](../specs/trellis-requirements-matrix.md) is traceability. Drift from the normative prose is a lint bug (`scripts/check-specs.py`), not a ratification gate.

Files:

- [`ratification-checklist.md`](ratification-checklist.md) — global and per-document gates. Evidence-of-record: each gate row carries inline commit SHAs and artifact pointers.

Tactical work to close open gates lives in [`../TODO.md`](../TODO.md). A parallel `ratification-evidence.md` registry existed briefly; it was removed (commit `617f9ae`) because the inline evidence pointers in the checklist made it redundant and it had drifted.

Archived per-family-spec gates (C-1, B-1, T-1, K-1, P-1, E-1, D-1, M-1, A-1) were superseded when the eight-spec family was consolidated into Core + Operational Companion. They live in the git history of this directory and in [`../specs/archive/`](../specs/archive/) but are not reinstated.
