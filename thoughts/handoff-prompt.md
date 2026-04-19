# Trellis — handoff prompt

You are continuing work on Trellis, the cryptographic integrity substrate for the
Formspec (intake) + WOS (governance) stack. Repo: `/Users/mikewolfd/Work/formspec/trellis`.
Branch: `main`.

## Orient in this order

1. [`README.md`](../README.md) — one-page framing, pointers to everything else.
2. [`TODO.md`](../TODO.md) — **current tactical state + next-work list.** Read this
   before starting anything; it supersedes any ad-hoc "what should I do next" list.
3. [`thoughts/product-vision.md`](product-vision.md) — authoritative roadmap.
   "Phase 1 envelope invariants (non-negotiable)" #1–#15, "Terminology — ledger
   vs log," Tracks A–E. Source of truth for what's being built and why.
4. [`specs/trellis-agreement.md`](../specs/trellis-agreement.md) — non-normative decision gate.
5. [`specs/trellis-core.md`](../specs/trellis-core.md) — normative Phase 1 byte protocol (~16k words).
6. [`specs/trellis-operational-companion.md`](../specs/trellis-operational-companion.md) —
   normative Phase 2+ operator obligations (~14k words).
7. [`specs/trellis-requirements-matrix.md`](../specs/trellis-requirements-matrix.md) —
   traceability (79 TR-CORE + 49 TR-OP rows; prose wins on conflict).
8. [`ratification/ratification-checklist.md`](../ratification/ratification-checklist.md) —
   open gates (G-2..G-5, O-3..O-5). Summary in `TODO.md`.

**Active design docs under `thoughts/specs/`:**

- `2026-04-18-trellis-g3-fixture-system-design.md` — fixture-system design (scope, manifest, coverage lint, generator discipline, pinned keys).
- `2026-04-18-trellis-g3-fixture-scaffold-plan.md` — 12-task plan; complete.
- `2026-04-18-trellis-core-gaps-surfaced-by-g3.md` — Core gaps surfaced by T10; all resolved.
- Older handoffs live under `thoughts/archive/specs/` — archaeology only.

## Conventions

- **Greenfield.** No backwards compat, no production legacy. Prefer clean rewrite
  over carrying a weak compromise. Architecture matters more than code or time.
- **Product vision is authoritative.** Every design decision traces to an invariant
  or a Track step. If you find a conflict between the vision and any other doc,
  the vision wins.
- **Lint stays green.** `python3 scripts/check-specs.py` (with `TRELLIS_SKIP_COVERAGE=1`
  until the per-invariant allowlist lands — see TODO.md). Test harness at
  `scripts/test_check_specs.py` — 7 tests, all pass.
- **Vocabulary.** Event / Response ledger / Case ledger / Agency log / Federation log.
  Nested scopes. "Ledger" is always scope-qualified. "Log" is a higher-order structure
  whose entries are other ledgers' heads.
- **Normative authority.** Only `trellis-core.md` and `trellis-operational-companion.md`
  impose implementor obligations. The agreement is a sign-off gate. The matrix is
  traceability. Design docs under `thoughts/specs/` are not normative.

## Before you edit

Skim the tail of `git log --oneline` (last ~20 commits) to see the recent decision
trail — commit messages carry rationale, not just diffs. Don't revert uncommitted
user edits flagged by system reminders.
