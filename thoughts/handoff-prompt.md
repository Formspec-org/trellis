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

## Working norms

Retrospective-derived; keep ceremony proportional to risk.

- **Inline by default for work under ~30 minutes.** Subagents earn their keep via isolation (preserving main-session context), parallelism (multiple independent chunks running at once), or genuinely large scope. For "write one template file" or "update three references," just do it. A 400-line subagent prompt for a 50-line job is waste.
- **Plans: 3–5 tasks max.** If a plan wants 12 tasks, it's probably two plans or the tasks are too small. Each task should be a meaningful chunk, not a bite-sized step. The G-3 scaffold's 12 tasks in retrospect would collapse cleanly to 4.
- **Doc updates travel with the thing they document.** If a commit adds a new Core §N, it also adds the corresponding `TR-CORE-NNN` row. If it closes a gate, it updates the checklist in the same commit. If it renames a concept, it propagates. Separate reconciliation passes accumulate drift that costs more later.
- **Skill ceremony is optional when design is settled.** Invoke `superpowers:brainstorming` only when design space is genuinely open. If the user already signaled the design ("directory-per-vector, TOML manifest, just build it"), write the plan and go.
- **Single-reviewer sweeps after meaningful chunks, not per-task.** Two-stage (spec + code) review is worth it for large scope. For small diffs, one post-hoc review finds the same issues at a third the cost. Always keep semi-formal review after anything that'll be read by outside implementors.
- **Parallel by default for independent work.** Reviews + implementation can run concurrently when the implementer's output isn't gated on the review. Same for running 3+ follow-on vector batches in parallel subagents. Use `run_in_background: true` freely.
- **Trust opus; prompt less.** Give the subagent the goal, the constraints, and the escalation rules. Let it figure out the plumbing. Long step-by-step prompts imply distrust that slows the agent and costs tokens.
- **`git log` before `Read` when orienting.** 10 commit messages usually tell you 80% of what 5 spec files would. Drop into the files when the log points at something specific.
- **Preserve unconditionally:** the escalation discipline (`NEEDS_CONTEXT` over fabrication — the T10 call that surfaced the three Core gaps was the highest-value moment of the prior session); semi-formal review after meaningful chunks; commit-per-logical-unit for anything a reviewer will read.

## Before you edit

Skim the tail of `git log --oneline` (last ~20 commits) to see the recent decision
trail — commit messages carry rationale, not just diffs. Don't revert uncommitted
user edits flagged by system reminders.
