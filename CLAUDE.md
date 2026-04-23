# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working on Trellis — the **cryptographic integrity substrate** of the three-spec stack.

## HIGH PRIORITY — Writing backlog / TODO / task items

**Every backlog entry, TODO, or task description MUST carry its own context.** A reader (human or agent) opening the item cold — no surrounding conversation, no memory of the session that produced it — must know *what the work is*, *why it matters*, and *what "done" looks like*, from the words on the page alone.

Write dense, not verbose. The model is a poem or a well-contextualized meme: few words, heavy payload, still easy to read. Every sentence pulls weight — if a phrase can be cut without losing meaning, cut it; if a phrase that looks redundant is actually the anchor that makes the rest make sense, keep it. No orphan pronouns, no "see above", no "the thing we discussed" — name the thing.

**The test:** if this item sat untouched for six weeks and a different agent picked it up, could they act on it without asking a clarifying question? If no, rewrite until yes.

Applies to `TODO.md`, plan files in `thoughts/plans/`, ADR follow-ups, ratification-gate backlog entries, fixture-vector stubs, Companion-declaration TODOs, and any inline `// TODO` comments that escape a single session.

## Project Overview

Trellis is the integrity layer beneath **Formspec** (intake) and **WOS** (governance). It specifies the envelope, chain, checkpoint, and export-bundle format by which a Formspec response and its downstream WOS governance events become a single append-only, signed, offline-verifiable record.

Trellis does not replace Formspec or WOS. It concretely answers two already-written deferrals: the Respondent Ledger §13 `LedgerCheckpoint` seam and the WOS `custodyHook` (§10.5). **What survives when the system, the vendor, and the years go away is the Trellis export.**

At stack end state, Trellis proves the portable case record, not only standalone export bytes. Export-first remains the near-term integrity posture, but the full stack proof must also carry the evidence bindings, WOS custody hooks, signature affirmations, amendment links, statutory clock state, and migration pins needed for a verifier to understand the case without the original runtime.

Trellis has two current normative specs, tagged `v1.0.0` to mark a coherent state — NOT to freeze further change:

- [`specs/trellis-core.md`](specs/trellis-core.md) — Phase 1 byte protocol (envelope, canonical encoding, signature profile, chain, checkpoint, export, verification).
- [`specs/trellis-operational-companion.md`](specs/trellis-operational-companion.md) — Phase 2+ operator obligations (custody models, projection discipline, metadata budgets, delegated-compute honesty, sidecars).

**Nothing in this repo is released.** There are zero production deployments, zero users, zero backwards-compatibility obligations. "Ratified," "v1.0," and "tagged" mean *this is our current best understanding* — they never justify refusing to change the wire shape, the CDDL, the signature profile, the chain topology, or anything else if a better design prevents future architectural debt. Coding is cheap, time is cheap, processing is cheap; architectural debt that blocks future changes is the only expensive debt. If changing a tagged surface prevents that debt, change it.

## Operating Context — READ THESE BEFORE DECIDING

Trellis is one spec in a three-spec stack. Architectural decisions frequently cross spec boundaries. Consult in this order before any non-trivial decision:

1. **[`../.claude/user_profile.md`](../.claude/user_profile.md)** — Owner's operating preferences. Economic model (minutes-not-days × Imp × Debt); design philosophy (opinionated, closed taxonomies, named seams); communication style (terse, opinionated, hedges labeled); and the **maximalist one-shot delivery** rule — no stubs, no `TODO: implement later`, no placeholder returns. If AI builds it, it ships complete and working in one pass; iterate on working code, not half-built code. Surface blockers instead of papering over with stubs.
2. **[`../.claude/vision-model.md`](../.claude/vision-model.md)** — Stack-wide vision captured 2026-04-20. The **Trellis section is fully populated**: Phase-1 posture (zero records before G-5 — now closed; Rust is byte authority; maximalist envelope / restrictive Phase-1 runtime; stranger-test is integrity anchor), the four format ADRs (0001–0004), the 7 ratification gates, active uncertainties (anchor substrate ε, Python maintenance burden, `custodyHook` contract, SCITT strictness β, Federation Profile), and Trellis-specific decision heuristics (phase-check, cheap-revision-window, byte-authority, reservation discipline).
3. **[`../STACK.md`](../STACK.md)** — Public-facing integrative doc covering the three-spec stack and the five cross-layer contracts. Canonical source for how Formspec + WOS + Trellis compose.
4. **[`../thoughts/specs/2026-04-22-platform-decisioning-forks-and-options.md`](../thoughts/specs/2026-04-22-platform-decisioning-forks-and-options.md)** — Active platform decision register for end-state commitments, implementation leans, forks, kill criteria, and organizational/product constraints. Consult before changing proof posture, custody semantics, witness/storage assumptions, signing/certificate export semantics, migration semantics, or stack-level verifier claims.
5. **[`../CLAUDE.md`](../CLAUDE.md)** — Parent repo guide. Filemap conventions, worktree rules, and parent-tree conventions apply wherever cross-spec work touches the parent.

**Conflict resolution:** direct owner signals in the current conversation > these docs > this CLAUDE.md > generic defaults. If any of these docs conflicts with owner signals, update the doc — don't work around it.

## Foundational answers (stack Q1-Q4, specialized for Trellis)

From the vision model, refined for Trellis:

- **Q1 First adopter:** SBA PoC is the concrete Phase 1 customer (single-agency intake with offline-verifiable export); public SaaS is the phase-sequenced successor (cooperative federation in Phase 4). "DocuSign replacement via our ledger" is the stack-level claim Trellis makes verifiable.
- **Q2 Spec-runtime authority:** Co-authoritative at byte level, with a sharper rule — **Rust is the byte authority** (ADR 0004). For decisions spec prose can't pin (CBOR ordering, COSE headers, ZIP metadata, Merkle steps), the Rust reference implementation is canonical. Python (`trellis-py/`) is a cross-check that updates to match; disagreement becomes a spec-clarification trigger.
- **Q3 Opinionated:** Maximalist envelope, restrictive Phase-1 runtime. Reserve capacity in the wire shape now (DAG `priorEventHash: [Hash]`, list-form `anchor_refs`, §22/§24 reservation slots); enforce Phase-1 scope with lint and runtime constraints rather than by omitting capacity from the envelope.
- **Q4 Verifiability threshold:** Byte-exact reproducibility across two independent implementations. G-5 stranger test is the integrity anchor — a second implementor reads Core prose only and byte-matches every vector. G-5 is closed (`trellis-py/` passes 45/45).

## Development Philosophy — READ THIS FIRST

**Compute is cheap. Time is cheaper. Development is near-free next to architectural debt.** Expensive mistakes here are architectural — data model, crypto boundaries, event taxonomy, sync contracts — not editorial. Prefer clean rethink over carrying a weak compromise forward. Phase 1 must name byte-exact decisions now because each is cheap to include and wire-breaking to retrofit.

**Write code for humans first.** Every crate, module, and function should be immediately legible. Names reveal intent. Comments explain *why*, never *what*. Byte-level code is hard enough without clever abstractions; be boringly clear.

**Prioritize by value added.** Before spending effort, ask: does this close a ratification gate, unlock a phase transition, or harden the stranger test? If not, deprioritize.

**All code is ephemeral. So is every current spec surface.** Nothing issues production records today. The Phase 1 shape is our current best answer, not a pinned artifact. Change it freely whenever the change prevents architectural debt; the only cost we care about is future-change cost, and today that cost is low. If a deployment later issues records, *that* deployment's compatibility story is a future concern — don't pre-impose it on ourselves now.

- **Architecture over code** — byte-level decisions (canonical encoding, signature profile, hash construction, chain topology, anchor shape) are where the thinking goes. Implementations are cheap to redo; wire shape is expensive to retrofit.
- **Delete, don't preserve** — no users, no legacy, no released surface. Historical drafts live under `specs/archive/` and `thoughts/archive/` labeled non-normative; current normative specs can themselves be rewritten whenever a better design appears.
- **KISS always** — fewer bytes = fewer verification paths = fewer adversary surfaces.
- **Right-sized files** — one coherent concept per file. `trellis-core.md` is monolithic by design (§1–§24 belong together); crate modules split by conformance class.
- **Extensibility where the spec demands it** — named seams only: `SigningKeyEntry` registry, `suite_id` registry (ML-DSA / SLH-DSA / hybrid reserved), `anchor_refs` list, Respondent Ledger §13 `LedgerCheckpoint`, WOS `custodyHook` §10.5, Track E §21 case-ledger / agency-log extension. New seams require ADRs.
- **The spec is the source of truth** — every normative MUST has a traceable row in [`specs/trellis-requirements-matrix.md`](specs/trellis-requirements-matrix.md) and (where testable) a byte-exact fixture in [`fixtures/vectors/`](fixtures/vectors/). Prose in Core and the Companion wins on matrix conflict.
- **No "defer" on greenfield** — audit finds something wrong, fix it. All phases are greenfield; the `v1.0.0` tag is a snapshot, not a lock.
- **Maximalist one-shot delivery** — ship complete. Stubs / `unimplemented!()` / `todo!()` / `NotImplementedError` are forbidden unless the blocker is an unresolved architectural decision (e.g., anchor substrate choice), in which case STOP and surface it.

## Three-spec layering — what Trellis owns vs. doesn't

| Concern | Layer | Owner |
|---|---|---|
| Form fields, FEL, validation, canonical response | Intake | **Formspec** |
| Respondent Ledger event shape (declared) | Intake add-on | **Formspec**; Trellis concretizes §13 seam |
| Workflow governance, deontic rules, provenance emission | Governance | **WOS** |
| `custodyHook` record construction | Governance | **WOS** emits; Trellis consumes |
| Event envelope (CDDL, CBOR canonical) | Integrity | **Trellis** |
| Signature suite (Ed25519 / COSE_Sign1 / suite_id) | Integrity | **Trellis** |
| Signing-key registry, rotation lifecycle | Integrity | **Trellis** |
| Hash construction (encrypt-then-hash, domain separation, crypto-shredding) | Integrity | **Trellis** |
| Chain topology (Phase 1 linear `prev_hash`; DAG `causal_deps` reserved) | Integrity | **Trellis** |
| Checkpoint seals, anchor targets, transparency-log patterns | Integrity | **Trellis** |
| Export bundle ZIP layout, offline verification | Integrity | **Trellis** |
| Certificate-of-completion (signed PDF artifact) | Integrity | **Trellis** |
| Federation Profile (Phase 4 cooperative trust-anchor network) | Integrity | **Trellis** (deferred) |

Trellis does NOT own: workflow semantics, signer-role workflow patterns, consent capture, form rendering, intake validation. Those live upstream in WOS and Formspec.

Trellis also does NOT own case initiation semantics. Per accepted [ADR 0073](../thoughts/adr/0073-stack-case-initiation-and-intake-handoff.md), WOS owns governed case identity and `case.created`; Formspec owns the intake session and `IntakeHandoff` evidence. Trellis anchors and exports that evidence path without deciding whether a case exists.

## Phase arc

Four-phase delivery arc from [`thoughts/product-vision.md`](thoughts/product-vision.md):

- **Phase 1 — Single-agency intake (ratified `v1.0.0`).** Envelope, chain, checkpoint, export bundle. Single-anchor deployment default. Byte-exact reproducibility via stranger test. SBA PoC reference fixture. **Current scope.**
- **Phase 2 — Consolidation.** HLC / DAG chain (via reserved `causal_deps` slot), multi-anchor, commitment slots (Pedersen/Merkle per-field).
- **Phase 3 — Unified case ledger.** Phase 1 envelope IS the Phase 3 event format (strict-superset guarantee from invariant #10 + #12).
- **Phase 4 — Sovereign / federation.** Cooperative trust-anchor network; respondent-held keys; threshold custody; equivocation-proof via witness.

**On the `v1.0.0` tag:** it marks a coherent snapshot where G-4 and G-5 both passed and the Core + Companion describe a working byte protocol. It is NOT a freeze. If a wire-shape change prevents architectural debt, make it and retag. Don't carry forward a known-weak choice just because it happens to be tagged today.

## Phase-1 principles + format ADRs

Gate: [`thoughts/specs/2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md`](thoughts/specs/2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md) — accepted and ratified into `v1.0.0`.

- **ADR 0001 — DAG-capable event topology, Phase-1 single-parent runtime.** `priorEventHash: [Hash]` in the envelope; Phase-1 lint requires array length = 1. Revisit on Phase-3 consolidation or real Phase-1 consolidation use case.
- **ADR 0002 — List-form anchors, single-anchor deployment default.** `anchor_refs: [AnchorRef]` in the envelope; Phase-1 requires ≥1 entry, operators normally populate one. Substrate choice is adapter-tier (see ε in vision model).
- **ADR 0003 — Core §22/§24 reservations kept in envelope, locked off in Phase 1.** Reserve the optional fields now; Phase-1 lint requires they remain absent. Revisit on Phase-4 scoping.
- **ADR 0004 — Rust is the byte authority, Python retained as cross-check.** Python generators in `fixtures/vectors/_generator/` stay live. Rust wins unresolved byte-level ambiguity; Python updates to match; disagreement becomes a spec-clarification trigger.

## Repo structure

- **`specs/`** — Normative markdown (ratified `v1.0.0`) plus supporting non-normative docs:
  - `trellis-core.md` — normative Phase 1 byte protocol.
  - `trellis-operational-companion.md` — normative operator obligations.
  - `trellis-agreement.md` — non-normative sign-off gate.
  - `trellis-requirements-matrix.md` — 79 TR-CORE + 49 TR-OP rows; traceability.
  - `cross-reference-map.md` — upstream-rehoming map (Formspec Respondent Ledger, WOS).
  - `archive/` — the former 8-spec family; retained for provenance, NOT normative.
- **`crates/`** — Rust workspace (10 crates; the reference implementation that won G-4):
  - `trellis-core` — envelope + canonical encoding + signature + chain + checkpoint + verification.
  - `trellis-types` — shared types / CDDL-bound models.
  - `trellis-cddl` — CDDL parsing / validation.
  - `trellis-cose` — COSE_Sign1 / Ed25519 signature profile.
  - `trellis-export` — deterministic ZIP export bundle layout.
  - `trellis-verify` — offline verifier; verification independence contract (no derived artifacts, no mutable DBs).
  - `trellis-store-memory`, `trellis-store-postgres` — append-only storage adapters (center-vs-adapter).
  - `trellis-conformance` — full committed-corpus replay (G-4 oracle).
  - `trellis-cli` — operator CLI.
- **`trellis-py/`** — Python stranger implementation. Closed G-5 (45/45). See [`trellis-py/ATTESTATION.md`](trellis-py/ATTESTATION.md) and [`trellis-py/BYTE-MATCH-REPORT.json`](trellis-py/BYTE-MATCH-REPORT.json).
- **`fixtures/vectors/`** — Byte-exact test vectors. Directory-per-vector layout with TOML manifest, narrative derivation evidence citing Core prose only, and committed cryptographic intermediates. Governed by [`thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md`](thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md).
- **`fixtures/declarations/`** — Delegated-compute honesty declarations (Companion §19).
- **`ratification/`** — Ratification gates, evidence, and the 7-gate checklist.
- **`scripts/check-specs.py`** — Lint enforcing forbidden patterns (signature zero-fill prose, JCS references, stale version strings, unarchived per-family paths, Profile-namespace hygiene) plus fixture coverage rules (testable-row-has-vector, declared-vs-derived, invariant coverage, generator-import discipline). `TRELLIS_SKIP_COVERAGE=1` bypasses coverage during batched vector rollout (planned replacement is a per-invariant allowlist).
- **`thoughts/`** — Product vision, research, reviews, ADRs, plans, specs-in-flight. Mirrors parent convention.
- **`REFERENCE.md`** — Heading-level inventory of every active document.
- **`TODO.md`** / **[`COMPLETED.md`](COMPLETED.md)** — Tactical work list + closed items.

## Architecture

### Logic ownership — Rust is the byte authority

Trellis business logic lives in Rust crates. Per ADR 0004, for any decision where spec prose can't pin the bytes (CBOR map ordering, COSE header layout, ZIP metadata, Merkle sibling ordering, hash domain separation tags), **Rust wins**. Python (`trellis-py/`) is a parallel implementation maintained as a cross-check: internal Rust/Python agreement catches typos and intra-team ambiguity; the G-5 stranger test catches spec ambiguity for an outside implementor.

If a byte-level question arises that the spec doesn't pin:
1. Check Rust behavior. Rust is canonical.
2. Update spec prose if the behavior is load-bearing and under-specified.
3. Update Python to match.
4. Add a new vector if coverage was missing.

### Canonical encoding + signature suite

- **dCBOR** (RFC 8949 §4.2.2 deterministic profile) pinned with CDDL. Invariant #1.
- **Ed25519 over COSE_Sign1** (`alg = -8`). `suite_id` registry reserves ML-DSA / SLH-DSA / hybrid codepoints for post-quantum migration. Invariant #2.
- **SHA-256** hash construction with domain separation tags. Encrypt-then-hash semantics for crypto-shredding. Invariant #4.
- **HPKE Base-mode** payload-key wrap for encrypted events.
- **COSE_Sign1 checkpoints** over `(tree_size, tree_head_hash, suite_id, timestamp, anchor_ref?)`. OpenTimestamps slot reserved.

### Center-vs-adapter discipline

- **Center:** `trellis-core` (semantics + byte protocol) + `trellis-types` + `trellis-cddl` + `trellis-cose` + `trellis-verify`. Declares the shape.
- **Traits:** storage (append-only), KMS, anchor target.
- **Adapters:** `trellis-store-memory` (dev/test), `trellis-store-postgres` (production default per user preference); anchor substrates TBD (see ε in vision model — OpenTimestamps default, Trillian / Rekor also in scope; adapter-tier choice, spike before pinning).

The **verification independence contract** (§16) is load-bearing: verifiers MUST NOT depend on derived artifacts, workflow runtime, or mutable DBs. Keep `trellis-verify` free of non-essential dependencies.

## Spec authoring contract

- **Use the `formspec-specs:wos-expert` / `formspec-specs:wos-spec-author` skill family** when a decision crosses into WOS territory (the stack has no dedicated "trellis-expert" skill; the spec is self-contained enough that reading `trellis-core.md` + `trellis-operational-companion.md` is the authoritative path). Do not guess from Rust code.
- Structural truth lives in the CDDL inside `trellis-core.md` (authoritative) and the Rust type definitions (which MUST match). When they disagree, Rust wins per ADR 0004; update the CDDL.
- Behavioral semantics that CDDL cannot encode live in the normative prose of `trellis-core.md` and `trellis-operational-companion.md`.
- Historical material under `specs/archive/` and `thoughts/archive/` is labeled non-normative. **Do not cite as normative.**
- Every testable normative row in [`specs/trellis-requirements-matrix.md`](specs/trellis-requirements-matrix.md) SHOULD have a byte-exact fixture in [`fixtures/vectors/`](fixtures/vectors/). `check-specs.py` enforces coverage.

## Build & test commands

```bash
# Targeted crate tests (pick what you touched)
cargo check --workspace
cargo test -p trellis-core
cargo test -p trellis-verify
cargo test -p trellis-conformance   # full-corpus replay (G-4 oracle)

# Full workspace
cargo test --workspace

# Python stranger cross-check (G-5 oracle)
cd trellis-py && python3 -m pytest -q

# Lint enforcing spec discipline + fixture coverage
python3 scripts/check-specs.py

# During batched vector rollout only
TRELLIS_SKIP_COVERAGE=1 python3 scripts/check-specs.py
```

## The 7 ratification gates

Canonically tracked in [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md). Summary:

- **G-2 Invariant coverage** — model-check flush; depends on G-4 evidence.
- **G-3 Byte-exact vectors** — under ADR 0004, corpus is Rust-generated from declarative inputs; acceptance reduces to lint-passing coverage.
- **G-4 Rust reference implementation** — full-corpus byte-match. **Closed.**
- **G-5 Stranger second implementation** — `trellis-py/` byte-matches every vector reading spec prose only. **Closed (45/45).**
- **O-3 Projection discipline** — Phase-1 fixtures per Companion §12.
- **O-4 Delegated-compute honesty** — declaration docs per Companion §19.
- **O-5 Posture-transition audit** — canonical events for custody / disclosure changes.

## Trellis-specific decision heuristics

Apply after stack-wide heuristics (see [`../.claude/vision-model.md`](../.claude/vision-model.md) § "Stack-wide decision heuristics"):

1. **Phase-check.** Phase 1 (single-agency intake — e.g., SBA PoC) or Phase 2+ (consolidation, federation, multi-anchor)? Phase 2+ defers; Phase 4 version-bumps the envelope regardless of Phase 1.
2. **Architectural-debt check.** Would keeping the current shape make a future change more expensive than making the change now? If yes, change it — tag or no tag. The `v1.0.0` label does not close the revision window; only real production records would, and there are none.
3. **Byte-authority check.** Byte-level question (CBOR encoding, COSE headers, hash algorithms, ZIP metadata, Merkle steps)? Rust is the oracle (ADR 0004); spec prose loses on byte-level details unless it's the more precise authority.
4. **Reservation discipline.** Proposing an envelope field, hash slot, or extension hook for Phase 2+? Default YES at the envelope layer if it preserves Phase-1 runtime restriction cleanly and avoids a later format break. Runtime semantics still default NO until the relevant phase opens.

## Development Workflow — Red-Green-Refactor

Every feature or bugfix follows this loop. Do NOT write implementation before a failing test exists.

1. **Red** — Write one minimal failing test (Rust unit, conformance vector, or `check-specs.py` rule — whichever layer the behavior lives at). Run it, confirm it fails for the right reason.
2. **Green** — Make it pass with the simplest change that works.
3. **Expand** — Add tests / vectors for edge cases and the full requirement.
4. **Verify** — Run `cargo test --workspace` + `python3 scripts/check-specs.py` + (if Python touched) `trellis-py` pytest.

**Test locations:**

- `crates/*/src/**/*.rs` — Rust unit tests colocated with code.
- `crates/trellis-conformance/tests/` — full-corpus byte-match (G-4).
- `fixtures/vectors/<op>/<name>/` — per-vector byte-exact cases.
- `fixtures/declarations/` — Companion §19 delegated-compute declarations.
- `trellis-py/src/` — stranger implementation tests.
- `scripts/check-specs.py` + `scripts/test_check_specs.py` — spec-discipline lint.

## Code Review Workflow — Test Before Fix

When review identifies a bug: write a failing test FIRST, then fix, then expand coverage around the bug site, then verify full suite. This applies to every bug — correctness, safety, byte-level drift, off-by-one. The test is proof the bug existed and proof it's fixed. A fix without a test is an unverified claim.

## Commit Convention

Use semantic prefixes: `feat:`, `fix:`, `build:`, `docs:`, `test:`, `refactor:`. Commit at logical stopping points — a passing suite, a complete bugfix, a self-contained refactor, a full vector-set. Not mid-refactor, not after one file of a multi-file change. Each commit is a meaningful, self-contained unit.

**Co-Author footer when AI-authored:**

```
Co-Authored-By: Claude <noreply@anthropic.com>
```

**Never** use `--amend`, `--force`, or `--no-verify` unless explicitly sanctioned by the owner. No commits on behalf of someone else.

**Submodule awareness:** this repo is checked out as `formspec/trellis/` inside the parent Formspec repo. Commits here are separate from parent commits. Remember to bump the parent submodule pointer when landing meaningful work.
