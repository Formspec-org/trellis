# Trellis — TODO

Forward-looking tactical work only. Priority = `Imp × Debt`; size tags are
scheduling hints, never priority inputs. Streams run concurrently under the
accepted Phase-1 principles/ADR posture and the ratified `v1.0.0` Core +
Operational Companion surface. State, history, and gate tracking live
elsewhere (see bottom).

Size: **XS** (≤1h) · **S** (≤1 session) · **M** (≤3 sessions) · **L** (multi-session).

---

## Gate — validated principles + ADRs

[`thoughts/specs/2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md`](thoughts/specs/2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md)
holds 7 accepted principles and 4 decided ADRs: **DAG envelope with
length-1 Phase-1 runtime** (ADR 0001), **list-form anchors with
single-anchor deployment default** (ADR 0002), **§22/§24 reservations held
in the envelope but MUST NOT populate in Phase 1** (ADR 0003), **Rust is
byte authority with Python retained as cross-check** (ADR 0004).

Gate status: **accepted and ratified into `v1.0.0`**. Streams below execute
against this posture.

No separate executable-dispatch doc is maintained. Open work is enumerated in
this file; closed work lives in [`COMPLETED.md`](COMPLETED.md) and
[`ratification/ratification-checklist.md`](ratification/ratification-checklist.md).

**On the `v1.0.0` tag — snapshot, not a freeze.** G-5 passed and `v1.0.0`
is tagged, but nothing is released and no production records exist.
Economic model: coding, time, and compute are cheap; architectural tech
debt we'd have to unwind later is the only expensive cost. If an
architectural change to the Phase-1 wire shape, verification contract,
or export layout prevents future debt, make it and retag. The revision
window is not closed — only real adopters can close it, and there are
none yet.

---

## Streams (ordered by Imp × Debt)

Closed ratification streams are out of this file. G-3/G-4/G-5 breadth,
ADR-0061 wire closure, ADR-0072 Phase-1 execution, and O-3/O-4/O-5 evidence
live in [`COMPLETED.md`](COMPLETED.md) and
[`ratification/ratification-checklist.md`](ratification/ratification-checklist.md).
This file tracks only open center work.

### 1. WOS-T4 cross-stack proof + certificate-of-completion export

- **Cross-stack proof for `SignatureAffirmation` through Trellis** — **M**,
  Phase 1.
  WOS-side Signature Profile semantics, runtime emission, lint, and
  conformance are now landed; the remaining stack work is to prove the
  end-to-end path from Formspec-authored signed-response evidence to
  WOS-emitted `SignatureAffirmation` records to Trellis append / verify /
  export artifacts. Trellis-side deliverables:
  - shared Formspec-authored canonical signed-response fixture consumed across
    Formspec / WOS / Trellis;
  - signature-specific Trellis append / export / verify coverage through the
    accepted ADR-0061 `custodyHook` wire;
  - export-bundle contract for the offline-verifiable certificate of
    completion that carries `SignatureAffirmation`, signed document hashes,
    signer attestations, and consent references.
  **Gate:** WOS-T4 next slice and the Formspec signed-response fixture.

### 2. Identity attestation bundle shape

- **Identity attestation bundle shape** — **S**, Phase 1.
  Declare how a provider-neutral identity-proofing attestation lands in the
  Trellis record as a canonical event kind and travels in the export bundle.
  This is cheap center work once WOS lifts
  `SignatureAffirmation.identityBinding` into a reusable shape.
  **Gate:** WOS identity-attestation shape settled.

### 3. Respondent Ledger ↔ Trellis binding

- **`eventHash` / `priorEventHash` MUST promotion** — **M**, Phase 1.
  Promote Formspec Respondent Ledger §6.2 `eventHash` / `priorEventHash` from
  SHOULD → MUST when wrapped by a Trellis envelope. Land the Trellis-side spec
  amendment and conformance/lint checks once the Formspec-side promotion is
  accepted.
  **Gate:** Formspec-side coordination.

### 4. ADR 0066 — amendment / supersession / rescission / correction

- **ADR 0066 execution** — **L**, phased.
  [`../thoughts/adr/0066-stack-amendment-and-supersession.md`](../thoughts/adr/0066-stack-amendment-and-supersession.md).
  Phase 1: reserve `supersedes_chain_id` in the envelope header under
  ADR 0003 MUST-NOT-populate discipline; land `append/011-correction`,
  `append/012-amendment`, `append/013-rescission`; extend the verifier with
  D-3 correction-preservation and rescission-terminality checks. Phase 4:
  activate supersession runtime and land `supersession-graph.json`.
  **Gate:** ADR 0066 accepted.

### 5. ADR 0067 — statutory clocks

- **ADR 0067 execution** — **M**, Phase 1.
  [`../thoughts/adr/0067-stack-statutory-clocks.md`](../thoughts/adr/0067-stack-statutory-clocks.md).
  Add `open-clocks.json` to the export-bundle spec; extend the verifier with a
  D-3 advisory diagnostic for expired-unresolved clocks; land
  `append/014-clock-started`, `append/015-clock-satisfied`,
  `append/016-clock-elapsed`, `append/017-clock-paused-resumed`.
  **Gate:** ADR 0067 accepted.

### 6. Deferred by phase, not forgotten

- **Case ledger + agency log semantic definitions** — **M**, Phase 4.
  Core §22 case ledger composes sealed response-ledger heads with WOS
  governance events; Core §24 agency log is the operator-maintained log of
  case-ledger heads. Envelope hooks stay reserved under ADR 0003 and
  `MUST NOT populate` in Phase 1. Substance waits for Phase-4 scoping.

### 7. Sustaining maintenance

- **Keep Rust within hours of new vectors** — **XS** per vector.
  Any new vector added by an open contract needs matching Rust conformance
  coverage immediately.

- **Commit declarative inputs under `_inputs/<op>/` when a new vector batch
  makes the traceability payoff real** — **S–M**.
  Useful for ADR-0004 traceability; not a priority stream by itself.

---

## Ratification close-out

Closed. G-5 evidence is recorded in
[`ratification/ratification-checklist.md`](ratification/ratification-checklist.md),
Core + Companion are at `1.0.0`, and the release tag is cut at close-out.

---

## Tagged baseline

`v1.0.0` describes a coherent Phase-1 snapshot: a second implementation,
written from [`specs/trellis-core.md`](specs/trellis-core.md) +
[`specs/trellis-operational-companion.md`](specs/trellis-operational-companion.md) +
[`specs/trellis-agreement.md`](specs/trellis-agreement.md) alone,
byte-matches every vector in `fixtures/vectors/`, and all ratification gates
in [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md)
are closed. Phase 2–4 are out of scope for this snapshot; follow-on work
below may revisit Phase-1 surface when doing so prevents architectural
debt. Nothing here is released.

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
| Python cross-check (G-5 harness) | `trellis-py/` | `pip install -e trellis-py && python -m trellis_py.conformance` |
| Lint + test green | — | `python3 scripts/check-specs.py && python3 -m pytest scripts/ && cargo test --workspace` |
| Recent commits, who changed what | — | `git log --oneline` |

When a TODO grows into a spec-sized effort, move the substance to
[`thoughts/specs/`](thoughts/specs/) and replace the entry here with a
pointer. When an item lands, move it to [`COMPLETED.md`](COMPLETED.md).
This file stays forward-looking.
