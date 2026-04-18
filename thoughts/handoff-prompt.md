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
8. **`thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md`** — design
   for the fixture system (directory layout, manifest schema TOML, derivation-
   evidence convention, matrix-driven coverage lint, data-only runner contract,
   disciplined Python generator, RFC-style pinned keys).
9. **`thoughts/specs/2026-04-18-trellis-g3-fixture-scaffold-plan.md`** — 12-task
   implementation plan. **Resume at Task 10.** Tasks 1–9 are committed.

Skip `thoughts/specs/2026-04-17-trellis-normalization-handoff.md` unless you are
archeologizing a Core section. The handoff is closed.

## Current state (as of 2026-04-18, ec5e391)

- Specs converged on two normative W3C-style documents: Core + Operational
  Companion. Previous 8-spec family is in `specs/archive/`. Don't cite it as
  normative.
- Ratification gates green: G-1 (handoff), G-6 (lint), C-1..C-8 (Core byte
  protocol), O-1/O-2 (Companion cross-refs + custody-model identifiers), M-1..M-3
  (matrix factual + gap-log + invariant coverage).
- Ratification gates open: **G-2** (final invariant coverage audit — folded into
  G-3 lint), **G-3** (~50 byte-exact test vectors), **G-4** (`trellis-*` Rust
  crate workspace), **G-5** (independent second implementation byte-matching),
  **O-3/O-4/O-5** (Companion conformance fixtures).
- **G-3 progress:** fixture system design + implementation plan + lint
  infrastructure committed. Scaffold, test harness, four coverage rules, pinned
  issuer-001 COSE_Key, and pinned sample payload all in place. First reference
  vector (`fixtures/vectors/append/001-minimal-inline-payload/`) **pending —
  Task 10 of the scaffold plan.**
- Lint bypass `TRELLIS_SKIP_COVERAGE=1` is active for rules 1–3. Remove it in
  Task 10 once the first vector covers enough rows/invariants to pass without
  it. With bypass: `python3 scripts/check-specs.py` → green. Without bypass:
  65 "no vector covers TR-CORE-XXX" errors + 15 "invariant has no vector"
  errors (the expected gap list).

## Conventions

- **Greenfield.** No backwards compat, no production legacy. Prefer clean rewrite
  over carrying a weak compromise. Architecture matters more than code or time.
- **Product vision is authoritative.** Every design decision should trace back to
  an invariant or a Track step. If you find a conflict between the vision and
  any other doc, the vision wins.
- **Lint.** `python3 scripts/check-specs.py` must stay green (with
  `TRELLIS_SKIP_COVERAGE=1` until Task 10 completes). New checks since last
  handoff: `check_vector_coverage`, `check_vector_declared_coverage`,
  `check_invariant_coverage`, `check_generator_imports`. Test harness at
  `scripts/test_check_specs.py` — all 3 tests pass.
- **Vocabulary.** Event / Response ledger / Case ledger / Agency log /
  Federation log. Nested scopes. "Ledger" is always scope-qualified. "Log" is a
  higher-order structure whose entries are other ledgers' heads.
- **Normative authority.** Only `trellis-core.md` and
  `trellis-operational-companion.md` impose implementor obligations. The
  agreement is a sign-off gate. The matrix is traceability. The fixture design
  spec is a design doc, not normative.

## Most useful next work

**Top priority: Task 10 of the fixture scaffold plan** — author
`fixtures/vectors/append/001-minimal-inline-payload/` end-to-end. This is the
load-bearing test of the design ("reproducible from Core prose alone"):

- Pinned inputs already committed: `issuer-001.cose_key`, `sample-payload-001.bin`.
- Constructions to derive, each citing Core prose: AuthoredEvent encoding
  (§6), `author_event_hash` preimage with domain separation (§7), COSE_Sign1
  via RFC 9052 `Sig_structure` (§8), CanonicalEvent, and
  `canonical_event_hash` / `next_head` chaining (§11).
- Deliverable: `manifest.toml` + `derivation.md` (cites Core prose only, never
  the generator) + 6 sibling `.cbor`/`.bin` files (inputs, intermediates,
  expected outputs) + a new `gen_append_001.py` in `_generator/`.
- Success: deterministic — running the generator twice produces identical
  bytes. Lint-with-bypass continues to pass. Lint-without-bypass shows a
  shrunken gap list (rows/invariants the vector covers now drop off).

Then Tasks 11–12: link fixture scaffold from top-level `README.md` + update
G-3 evidence in `ratification/ratification-checklist.md`, and run final
verification (test harness + lint with and without bypass + determinism diff).

After the scaffold plan closes, follow-on plans author the remaining ~49
vectors in batches (likely one plan per op-dir: a batch of ~15 append, ~15
verify, ~10 export, ~10 tamper). Each follow-on plan consumes the design spec
and the scaffold as its substrate.

## Tracks running in parallel

Tracks B (WOS runtime + Formspec coprocessor), C (FedRAMP/SOC2/GSA calendar-
gated certs), and D (reviewer dashboard, doc storage, webhooks, notifications)
in the product vision are independent of G-3 and can proceed without Trellis
ratification.

## Before you edit

Skim the tail of `git log --oneline` (last ~20 commits) to see the recent
decision trail. Read commit messages, not just diffs — they carry rationale.
Don't revert uncommitted user edits flagged by system reminders.

For Task 10 specifically, the engineer must read Core §§6, 7, 8, 11 carefully
before writing a single byte. The plan has detailed per-step instructions but
deliberately does NOT pre-compute expected bytes — those are what the task
produces, and they must come from reading Core, not from consulting any
other source (including prior generator drafts).
