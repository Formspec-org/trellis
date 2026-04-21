# Trellis — TODO

Forward-looking tactical work only. Priority = `Imp × Debt`; size tags are
scheduling hints, never priority inputs. Streams run concurrently once you
validate the principles doc. State, history, and gate tracking live
elsewhere (see bottom).

Size: **XS** (≤1h) · **S** (≤1 session) · **M** (≤3 sessions) · **L** (multi-session).

---

## Gate — validate principles + ADRs

[`thoughts/specs/2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md`](thoughts/specs/2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md)
holds 7 principles and 4 decided ADRs: **single-parent chain** (ADR 0001),
**single-slot anchor** (ADR 0002), **no §22/§24 reservations** (ADR 0003),
**Rust is byte authority + Python generators retire** (ADR 0004). Status:
**Draft — pending the validation checklist at the bottom of the doc.**

Every stream below is blocked on this tick-or-redirect.

---

## Streams (ordered by Imp × Debt)

### 1. Rust reference implementation

Rust is the byte authority (ADR 0004). Every week without it, Python
generators remain the de facto reference impl — accumulating structural
Debt.

- **Workspace scaffolding + `append/001` byte-match** — **M**.
  Per [`thoughts/specs/2026-04-18-trellis-g4-rust-workspace-plan.md`](thoughts/specs/2026-04-18-trellis-g4-rust-workspace-plan.md).
  Goal: `cargo test` produces byte-identical `append/001` output from a
  declarative input. Does not require the rest of the corpus.

- **Incremental op coverage + Python retirement per-op** — **L**.
  For each op: byte-match every committed fixture; commit declarative
  input under `fixtures/vectors/_inputs/<op>/`; add Rust CLI; delete
  `_generator/gen_<op>_*.py` (per ADR 0004). Retire
  `_generator/_lib/` when the last generator falls.

- **Full-corpus parity** — **L**.
  Closes G-4. Rust trails the latest vectors by hours, not months, while
  the impl stays healthy.

### 2. G-5 stranger implementation

Pure Imp, zero contributor cost — wall-clock runs in parallel.

- **Commission `trellis-py` or `trellis-go`** — **L** (elapsed).
  Implementor reads
  [`specs/trellis-core.md`](specs/trellis-core.md) +
  [`specs/trellis-operational-companion.md`](specs/trellis-operational-companion.md) +
  [`specs/trellis-agreement.md`](specs/trellis-agreement.md) only — never
  `fixtures/vectors/_generator/`, never the Rust impl. Builds against
  whatever vectors exist; adds coverage as the corpus grows. Closes when
  byte-match parity is reached.

### 3. Vector authoring (feeds Stream 1)

- **`verify/` negative-non-tamper tail** — **S** each.
  §19 steps 4.* (event-level), 5.d (`prev_checkpoint_hash`), 5.e
  (consistency-proof mismatch), 6 (posture transition), 8 (anchor).
  Revoked / `valid_to` enforcement needs an explicit §19 pin — coordinate
  with Core authoring before writing the vector.

- **`export/` suite expansion** — **M**.
  ZIP-determinism edge cases, manifest variants, key-material handling,
  larger inclusion-proof sets. Per Core §18.

- **Residual `tamper/` cases** — **S** each.
  `prev_hash_break` (mutated bytes + re-sign), `missing_head` (needs a
  checkpoint), `wrong_scope` + `registry_snapshot_swap` (bundle with
  verify / export manifests).

### 4. Stream E — Respondent Ledger ↔ Trellis binding

- **(a) `eventHash` / `priorEventHash` MUST promotion** — **M**, Phase 1.
  Promote Formspec Respondent Ledger §6.2 `eventHash` / `priorEventHash`
  from SHOULD → MUST when wrapped by a Trellis envelope. Requires
  Formspec-side coordination.

- **(b, c) Case ledger + agency log semantic definitions** — **M**, Phase 4.
  Core §22 case ledger (top-level object composing sealed response-ledger
  heads with WOS governance events) and §24 agency log (operator-maintained
  log of case-ledger heads). Envelope hooks are reserved per ADR 0003;
  Phase-1 lint enforces `MUST NOT populate`. Substance (what goes in the
  hooks) defers to Phase 4 scoping.

### 5. G-2 model-check flush

- **Flush [`fixtures/vectors/_pending-model-checks.toml`](fixtures/vectors/_pending-model-checks.toml)** — **M**.
  Closes G-2. Depends on G-4 Rust model-check evidence, so sequences
  naturally behind Stream 1.

---

## Ratification close-out

- **Close out** — **XS** (mechanical).
  When all 7 gates flip to `[x]`: update
  [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md)
  with final evidence SHAs, strike "(Draft)" from Core + Companion titles,
  cut a version tag.

---

## Phase-1 end-state

The stranger test passes: a second implementation, written from
[`specs/trellis-core.md`](specs/trellis-core.md) +
[`specs/trellis-operational-companion.md`](specs/trellis-operational-companion.md) +
[`specs/trellis-agreement.md`](specs/trellis-agreement.md) alone,
byte-matches every vector in `fixtures/vectors/`. Closes when all 7
ratification gates flip to `[x]`. Phase 2–4 are explicitly out of scope.

---

## State lives in

This TODO points at work. State lives elsewhere — fetch it when you need it.

| What | Where | How to read it |
|---|---|---|
| Gate status, evidence SHAs | [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md) | open the file |
| Principles + format ADRs | [`thoughts/specs/2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md`](thoughts/specs/2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md) | open the file |
| Closed work (waves, sprints, streams) | [`COMPLETED.md`](COMPLETED.md) | open the file |
| Strategy, product arc, invariants | [`thoughts/product-vision.md`](thoughts/product-vision.md) | open the file |
| Implementation plans | [`thoughts/specs/`](thoughts/specs/) | `ls thoughts/specs/` |
| Fixture corpus (ground truth) | `fixtures/vectors/` | `ls fixtures/vectors/*/` |
| Lint + test green | — | `python3 scripts/check-specs.py && python3 -m pytest scripts/` |
| Recent commits, who changed what | — | `git log --oneline` |

When a TODO grows into a spec-sized effort, move the substance to
[`thoughts/specs/`](thoughts/specs/) and replace the entry here with a
pointer. When an item lands, move it to [`COMPLETED.md`](COMPLETED.md).
This file stays forward-looking.
