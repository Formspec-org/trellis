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

## Current state (as of 2026-04-18, e1895ae)

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
  infrastructure committed and post-review-hardened. Scaffold, test harness
  (7 tests, up from 3), four coverage rules (now with correct `#N` / `#N, #M`
  invariant parsing), pinned issuer-001 COSE_Key, and pinned sample payload
  all in place. Design + plan amended per review findings F1/F2/F4/F5.
  First reference vector (`fixtures/vectors/append/001-minimal-inline-payload/`)
  **blocked — see Core gap list below.**
- Lint bypass `TRELLIS_SKIP_COVERAGE=1` is transitional; the amended design
  commits to replacing it with a per-invariant allowlist
  (`_pending-invariants.toml`) — a separate follow-on plan. With bypass:
  `python3 scripts/check-specs.py` → green. Without bypass: meaningful gap
  list — specific uncovered `TR-CORE-*` rows + 11 uncovered byte-testable
  invariants (not all 15 — non-byte-testable invariants are audited via
  separate G-2 work per amended design F2).
- **Task 10 UNBLOCKED.** The three blocking Core gaps (B1/B2/B3) are closed
  in commits `6ad24ab` (§7.4 COSE labels), `1b66eed` (§6.8 three event
  surfaces), `a844e4a` (§10.6 AppendHead struct). Four secondary gaps
  (S1/S2/S3/S5) closed in `e1895ae`. Resolution detail at
  `thoughts/specs/2026-04-18-trellis-core-gaps-surfaced-by-g3.md`. The
  ratification bar worked as intended: G-3 surfaced specific Core
  under-specifications before they became G-5 interop failures, and Core
  is now tighter for it.
- **Plan Task 10 citations need updating before re-dispatch.** Plan cites
  §§6/7/8/11 for constructions that now live at §§6.1+6.8 / §7.4 / §9.5 /
  §9.2 / §10.6. The gap-list doc carries the correct mapping.

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

**Top priority: re-dispatch Task 10** (author `append/001-minimal-inline-payload`
end-to-end). Core is now unblocked. Before re-dispatching:

1. **Update plan Task 10 citations** to reflect the post-amendment Core:
   - AuthoredEvent / CanonicalEvent naming → §6.1 + §6.8
   - `author_event_hash` preimage + domain separation → §9.5 + §9.1
   - COSE_Sign1 procedure + header labels → §7.4
   - `canonical_event_hash` → §9.2
   - `kid` derivation → §8.3
   - Chain invariant → §10.2
   - `AppendHead` return artifact → §10.6
   - Reserved test identifiers → §14.6
2. **Consider renaming** `expected-next-head.cbor` → `expected-append-head.cbor`
   to match the new Core term `AppendHead`. Minor; either works.
3. **Re-dispatch the T10 implementer subagent.** Same prompt as before but
   with updated section citations. The vector author now has enough in Core
   to produce byte-exact output: pinned header labels, named surfaces, pinned
   nonce size, pinned kid derivation, pinned reserved test identifiers, and
   an explicit return artifact.

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
