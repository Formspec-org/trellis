# Trellis — handoff prompt

You are continuing work on Trellis, the cryptographic integrity substrate for the
Formspec (intake) + WOS (governance) stack. Repo: `/Users/mikewolfd/Work/formspec/trellis`.
Branch: `main`.

## Orient in this order

1. `README.md` — one-page framing, pointers to everything else.
2. `thoughts/product-vision.md` — authoritative roadmap. Read "Phase 1 envelope
   invariants (non-negotiable)" (#1–#15), "Terminology — ledger vs log," and
   Tracks A–E. This is the source of truth for what's being built and why.
3. `specs/trellis-agreement.md` — non-normative decision gate.
4. `specs/trellis-core.md` — normative Phase 1 byte protocol (~16k words).
5. `specs/trellis-operational-companion.md` — normative Phase 2+ operator
   obligations (~14k words).
6. `specs/trellis-requirements-matrix.md` — traceability (79 TR-CORE + 49 TR-OP
   rows; prose wins on conflict).
7. `ratification/ratification-checklist.md` — open gates are the to-do list.

Skip `thoughts/specs/2026-04-17-trellis-normalization-handoff.md` unless you are
archeologizing a Core section. The handoff is closed.

## Current state

- Specs converged on two normative W3C-style documents: Core + Operational
  Companion. Previous 8-spec family is in `specs/archive/`. Don't cite it as
  normative.
- Ratification gates green: G-1 (handoff), G-6 (lint), C-1..C-8 (Core byte
  protocol), O-1/O-2 (Companion cross-refs + custody-model identifiers), M-1..M-3
  (matrix factual + gap-log + invariant coverage).
- Ratification gates open: **G-2** (final invariant coverage audit), **G-3**
  (~50 byte-exact test vectors under `fixtures/vectors/{append,verify,export,tamper}/`),
  **G-4** (`trellis-*` Rust crate workspace), **G-5** (independent second
  implementation byte-matching), **O-3/O-4/O-5** (Companion conformance fixtures).
- Respondent Ledger Track E §21 binding closed: §6.2.1 MUST-promotion,
  §13A case ledger, §13B agency log, §15A.4 three-namespace disambiguation.
- Upstream spec lives at `thoughts/formspec/specs/respondent-ledger-spec.md`.

## Conventions

- **Greenfield.** No backwards compat, no production legacy. Prefer clean rewrite
  over carrying a weak compromise. Architecture matters more than code or time.
- **Product vision is authoritative.** Every design decision should trace back to
  an invariant or a Track step. If you find a conflict between the vision and
  any other doc, the vision wins.
- **Lint.** `python3 scripts/check-specs.py` must stay green. It enforces:
  canonical CBOR (not JCS), no custom signature-zero-fill prose, no stale version
  strings, no `DRAFTS/` paths, Profile-namespace hygiene (three orthogonal
  namespaces: Respondent Ledger `Profile A/B/C` = posture; Core Conformance
  Classes; Companion Custody Models).
- **Vocabulary.** Event / Response ledger / Case ledger / Agency log /
  Federation log. Nested scopes. "Ledger" is always scope-qualified. "Log" is a
  higher-order structure whose entries are other ledgers' heads.
- **Normative authority.** Only `trellis-core.md` and `trellis-operational-companion.md`
  impose implementor obligations. The agreement is a sign-off gate. The matrix
  is traceability.

## Most useful next work

In rough dependency order:

1. **Author test vectors** (`fixtures/vectors/…`). Every byte-level claim in
   `trellis-core.md` should resolve to at least one vector. Closes G-3 after
   population. Unblocks G-4 and G-5.
2. **Stand up `trellis-*` Rust crate workspace.** Per Core-spec Track A step 7.
   Public API: `append`, `verify`, `export`. Closes G-4.
3. **Commission a second implementation** (Python or Go) read-only-from-spec.
   Byte-identical vector match is the stranger-test / ratification bar.
   Closes G-5.
4. **Operational Companion conformance fixtures** for projections, delegated-compute
   declarations, posture transitions. Closes O-3/O-4/O-5.
5. **Final G-2 invariant-coverage audit** across matrix §3.1 once the above have
   shaken out any drift.

Tracks B (WOS runtime + Formspec coprocessor), C (FedRAMP/SOC2/GSA calendar-gated
certs), and D (reviewer dashboard, doc storage, webhooks, notifications) in the
product vision run in parallel with Track A and are not blocked by Trellis
ratification.

## Before you edit

Skim the tail of `git log --oneline` (last ~15 commits) to see the recent
decision trail. Read commit messages, not just diffs — they carry rationale.
Don't revert uncommitted user edits flagged by system reminders.
