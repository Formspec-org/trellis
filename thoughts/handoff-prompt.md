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
   implementation plan. **All 12 tasks complete.** First reference vector
   `fixtures/vectors/append/001-minimal-inline-payload/` committed at `e1ab065`.

Skip `thoughts/specs/2026-04-17-trellis-normalization-handoff.md` unless you are
archeologizing a Core section. The handoff is closed.

## Current state (as of 2026-04-18, 227b212)

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
- **G-3 fixture scaffold plan: closed.** All 12 tasks committed. Infrastructure
  in place: directory layout, TOML manifest format, narrative derivation
  template, lint test harness (7 tests), four coverage rules (testable-row,
  declared-vs-derived, invariant coverage, generator-import AST scan), pinned
  issuer-001 COSE_Key + sample payload, plus the first reference vector
  `fixtures/vectors/append/001-minimal-inline-payload/` (commit `e1ab065`)
  exercising authored/canonical/signed event surfaces (§6.8), author_event_hash
  (§9.5), COSE_Sign1 with pinned -65537 suite_id label (§7.4), canonical_event_hash
  (§9.2), and AppendHead (§10.6). Vector is deterministic — regeneration
  produces byte-identical output.
- **Core amendments:** B1 (§7.4 COSE header labels `-65537/-65538`),
  B2 (§6.8 three event surfaces), B3 (§10.6 AppendHead struct), plus S1/S2
  (§14.6 `x-trellis-test/` prefix), S3 (§6.4 nonce `.size 12`), S5 (§8.3 kid
  derivation) all landed in commits `6ad24ab / 1b66eed / a844e4a / e1895ae`.
  Core gap resolution trail at `thoughts/specs/2026-04-18-trellis-core-gaps-surfaced-by-g3.md`.
- **Lint state:** with `TRELLIS_SKIP_COVERAGE=1`, green. Without bypass,
  meaningful gap list: 61 `TR-CORE-*` rows uncovered (down from 65 pre-T10)
  + 7 byte-testable invariants uncovered (down from 11 pre-T10). Invariants
  newly covered by the first reference vector: #1 (dCBOR), #2 (COSE_Sign1),
  #4 (canonical hash), #5 (scoped vocabulary). Bypass remains transitional
  pending the per-invariant allowlist (`_pending-invariants.toml`)
  follow-on from design F5.

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

G-3 scaffold is closed. Four candidates for the next session, in rough priority:

1. **Replace `TRELLIS_SKIP_COVERAGE=1` with `_pending-invariants.toml`
   allowlist.** Design F5 committed to this. Small Python change to
   `scripts/check-specs.py` (remove the three bypass early-returns;
   load a TOML file at `fixtures/vectors/_pending-invariants.toml`
   enumerating invariants not yet covered; emit errors both for missing-
   and-not-listed *and* for listed-but-now-covered). New harness scenario.
   Preserves ratification signal during rollout. ~1 focused session.

2. **Next vector batch: append/002..00N.** Targets the remaining byte-
   testable invariants `{3, 6, 7, 8, 10, 12, 13}`. Candidate batch (rough):
   - `append/002-rotation-signing-key` — invariant #8 (key rotation).
   - `append/003-external-payload-ref` — invariant #6 (external payload).
   - `append/004-hpke-wrapped-inline` — a real HPKE wrap with pinned
     ephemeral X25519 keypair (first vector deferred this per S4).
   - `append/005-multi-signer` — invariant around co-signing if applicable.
   - `append/006-prior-head-chain` — explicit `prev_hash` linkage (non-
     genesis), invariant #7.
   - A tamper vector exercising signature-invalid detection.
   Each batch is its own plan per the scaffold plan's "Follow-on signals"
   section; brainstorm before writing.

3. **G-4: Rust reference impl.** Per Core-spec Track A step 7. Public API
   `append` / `verify` / `export`. Consumes `fixtures/vectors/` as its test
   corpus. Independent of G-3's remaining vector batches — byte-matching
   the first vector alone is a legitimate G-4 milestone.

4. **Operational Companion conformance fixtures** for O-3/O-4/O-5 (projections,
   delegated-compute declarations, posture transitions). Separate fixture
   system; design precedes implementation.

Parallel low-risk work (does NOT block on Core amendments):

- **Replace `TRELLIS_SKIP_COVERAGE=1` with `_pending-invariants.toml`
  allowlist.** Design committed to this via F5. Small Python change, new test
  scenario, update of the plan's "Follow-ons" tracking. Improves ratification
  signal during rollout.
- **Secondary Core gaps (S1–S5).** `event_type` / `classification`
  registration, `PayloadInline.nonce` size pin, HPKE-roundtrip vs structural
  latitude for `key_bag`, `kid` construction byte encoding. Independent of
  B1–B3; could be batched with them.

After Task 10 lands: Tasks 11–12 (link fixture scaffold from top-level
`README.md` + update G-3 evidence in `ratification/ratification-checklist.md`,
then final verification). Then follow-on plans author the remaining ~49
vectors in batches.

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
