# Trellis — TODO

Forward-looking tactical work only. Priority = `Imp × Debt`; size tags are
scheduling hints, never priority inputs. Streams run concurrently under the
accepted Phase-1 principles/ADR posture. State, history, and gate tracking
live elsewhere (see bottom).

Size: **XS** (≤1h) · **S** (≤1 session) · **M** (≤3 sessions) · **L** (multi-session).

---

## Gate — validate principles + ADRs

[`thoughts/specs/2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md`](thoughts/specs/2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md)
holds 7 accepted principles and 4 decided ADRs: **DAG envelope with
length-1 Phase-1 runtime** (ADR 0001), **list-form anchors with
single-anchor deployment default** (ADR 0002), **§22/§24 reservations held
in the envelope but MUST NOT populate in Phase 1** (ADR 0003), **Rust is
byte authority with Python retained as cross-check** (ADR 0004).

Gate status: **accepted**. Streams below execute against this posture.

Executable breakdown for the post-gate queue lives in
[`thoughts/specs/2026-04-20-trellis-todo-executable-task-dispatch.md`](thoughts/specs/2026-04-20-trellis-todo-executable-task-dispatch.md).

**Governance rule — zero records before G-5.** No records are issued under
the Phase-1 envelope shape until the stranger test (G-5) passes. Protects
the cheap-revision window the maximalist-envelope ADRs rely on — runtime
scope stays Phase 1, but the wire shape remains free to absorb revision
until a second impl has proved the spec prose pins it.

---

## Streams (ordered by Imp × Debt)

### 1. Rust reference implementation — sustaining

G-4 is closed (see [`COMPLETED.md`](COMPLETED.md) and
[`ratification/ratification-checklist.md`](ratification/ratification-checklist.md)).
The 10-crate workspace under [`crates/`](crates/) replays the full committed
corpus byte-for-byte. This stream now tracks sustaining work only.

- **Keep Rust within hours of new vectors** — **XS** per vector.
  New vectors (from Stream 3) need a corresponding Rust conformance entry.

- **Commit declarative inputs under `_inputs/<op>/`** — **S–M**.
  `fixtures/vectors/_inputs/` has shared payload inputs only; most committed
  vectors remain self-contained in their vector directories. Per-op
  declarative inputs remain useful for full ADR-0004 traceability, but this is
  sustaining work, not a ratification blocker.

### 2. G-5 stranger implementation

Pure Imp, zero contributor cost — wall-clock runs in parallel.

- **Commission `trellis-py` or `trellis-go`** — **L** (elapsed).
  Commission brief is drafted at
  [`thoughts/specs/2026-04-21-trellis-g5-stranger-commission-brief.md`](thoughts/specs/2026-04-21-trellis-g5-stranger-commission-brief.md).
  Clean tracked-file read-set package is staged at
  [`ratification/g5-package/`](ratification/g5-package/).
  Implementor reads only the allowed inputs named there — never
  `fixtures/vectors/_generator/`, never the Rust impl, never planning docs.
  Closes when a genuinely independent implementation byte-matches the corpus.

### 3. Vector authoring (feeds Stream 1)

Phase-1 G-3 breadth is closed for the current surface:

- `verify/001–012` now cover Core §19 steps 1–8, including the residual
  step-4 revoked/`valid_to` branch, step-6 posture-transition path, and
  step-8 optional-anchor handling.
- `export/001–004` now cover the baseline two-event chain, revoked-key
  history, a three-event transition chain with larger proof sets, and a
  bundled-`PayloadExternal` / optional-anchor manifest variant.
- `tamper/001–012` now include the residual `prev_hash_break`,
  `missing_head`, `wrong_scope`, and `registry_snapshot_swap` cases.

No additional vector-authoring queue remains unless the spec surface grows.

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

### 5. WOS `custodyHook` joint ADR

Joint design between WOS and Trellis for the provenance-record shape WOS
emits and Trellis anchors. Load-bearing for WOS 1.0 closure; mirror of
WOS TODO Do-next #3.

Drafts landed:

- [`../wos-spec/thoughts/adr/0061-custody-hook-trellis-wire-format.md`](../wos-spec/thoughts/adr/0061-custody-hook-trellis-wire-format.md)
- [`thoughts/specs/2026-04-21-trellis-wos-custody-hook-wire-format.md`](thoughts/specs/2026-04-21-trellis-wos-custody-hook-wire-format.md)

No open Trellis-side ADR task remains in this stream. Next changes should be
driven by WOS-side review or cross-submodule implementation work.

### 6. O-gates — operational-companion ratification fixtures

Named 1.0 ratification gates from
[`ratification/ratification-checklist.md`](ratification/ratification-checklist.md).
O-3, O-4, and O-5 are closed. Reopen this stream only if the operational
companion grows new ratification surface.

---

## Ratification close-out

- **Close out** — **XS** (mechanical).
  Blocked on G-5. When G-5 flips to `[x]`: update
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
| Rust reference implementation | `crates/` | `cargo test --workspace` |
| Lint + test green | — | `python3 scripts/check-specs.py && python3 -m pytest scripts/ && cargo test --workspace` |
| Recent commits, who changed what | — | `git log --oneline` |

When a TODO grows into a spec-sized effort, move the substance to
[`thoughts/specs/`](thoughts/specs/) and replace the entry here with a
pointer. When an item lands, move it to [`COMPLETED.md`](COMPLETED.md).
This file stays forward-looking.
