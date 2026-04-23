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

- **Cross-stack proof for `SignatureAffirmation` through Trellis** — **S**,
  Phase 1 (downgraded from **M** now that the byte proof path exists).
  **Landed 2026-04-22 (Trellis center):** `append/019-wos-signature-affirmation`
  pins WOS `SignatureAffirmation` through ADR-0061 `custodyHook` with the
  `(caseId, recordId)` idempotency tuple; `export/006-signature-affirmations-inline`
  carries the signed event plus chain-derived `062-signature-affirmations.cbor`
  bound by `trellis.export.signature-affirmations.v1` in the export manifest;
  `verify/014-export-006-signature-row-mismatch` and
  `tamper/014-signature-catalog-digest-mismatch` exercise verifier failure modes;
  `trellis/specs/trellis-core.md` registers the extension and verifier
  obligations; generators live under `fixtures/vectors/_generator/`
  (`gen_append_019.py`, `gen_signature_export_006.py`). Evidence:
  `cargo test -p trellis-verify -p trellis-conformance`, `python3 scripts/check-specs.py`.
  **Still open:** Trellis-owned *human-facing* certificate-of-completion
  composition spec and any requirement to re-seed vectors from the parent
  Formspec signed-response fixture URL for a single cross-repo bundle;
  shared fixture consumption across Formspec / WOS / Trellis remains a
  parent-repo coordination item ([`../TODO.md`](../TODO.md) stack tracker).
  **Gate:** none for the Trellis machine-verifiable slice; parent alignment is
  coordination-only.

### 2. Case-initiation handoff export evidence

- **ADR 0073 handoff evidence in export/verify** — **S**, Phase 1.
  [`../thoughts/adr/0073-stack-case-initiation-and-intake-handoff.md`](../thoughts/adr/0073-stack-case-initiation-and-intake-handoff.md).
  **Landed 2026-04-23 (Trellis center):** `trellis.export.intake-handoffs.v1`
  is registered in Core §6.7; `063-intake-handoffs.cbor` is now a first-class
  optional export member; `trellis-verify` localizes digest mismatch, malformed
  catalog, unresolved WOS events, WOS payload mismatch, and Formspec
  `responseHash` mismatch; committed vectors now cover:
  `append/020-wos-intake-accepted-workflow-attach`,
  `append/021-wos-intake-accepted-public-create`,
  `append/022-wos-case-created-public-intake`,
  `export/007-intake-handoffs-public-create`,
  `export/008-intake-handoffs-workflow-attach`,
  `verify/015-export-007-intake-response-hash-mismatch`, and
  `tamper/015-intake-handoff-catalog-digest-mismatch`. Trellis now proves both
  accepted ADR 0073 paths in machine-verifiable export artifacts rather than
  prose only. **Still open:** if the parent repo standardizes one shared
  cross-stack fixture bundle, Trellis should consume those declarative inputs
  instead of seeding a parallel intake corpus. That is coordination work, not a
  Trellis-center gap.
  **Gate:** none — ADR 0073 is accepted and the Formspec/WOS handoff schema/reference parser landed 2026-04-23.

### 3. Identity attestation bundle shape

- **Identity attestation bundle shape** — **S**, Phase 1.
  Declare how a provider-neutral identity-proofing attestation lands in the
  Trellis record as a canonical event kind and travels in the export bundle.
  This is cheap center work once WOS lifts
  `SignatureAffirmation.identityBinding` into a reusable shape.
  **Gate:** WOS identity-attestation shape settled.

### 4. Respondent Ledger ↔ Trellis binding

- **`eventHash` / `priorEventHash` MUST promotion** — **M**, Phase 1.
  Promote Formspec Respondent Ledger §6.2 `eventHash` / `priorEventHash` from
  SHOULD → MUST when wrapped by a Trellis envelope. Land the Trellis-side spec
  amendment and conformance/lint checks once the Formspec-side promotion is
  accepted.
  **Gate:** Formspec-side coordination.

### 5. ADR 0066 — amendment / supersession / rescission / correction

- **ADR 0066 execution** — **L**, phased.
  [`../thoughts/adr/0066-stack-amendment-and-supersession.md`](../thoughts/adr/0066-stack-amendment-and-supersession.md).
  Phase 1: reserve `supersedes_chain_id` in the envelope header under
  ADR 0003 MUST-NOT-populate discipline; land `append/011-correction`,
  `append/012-amendment`, `append/013-rescission`; extend the verifier with
  D-3 correction-preservation and rescission-terminality checks. Phase 4:
  activate supersession runtime and land `supersession-graph.json`.
  **Gate:** ADR 0066 accepted.

### 6. ADR 0067 — statutory clocks

- **ADR 0067 execution** — **M**, Phase 1.
  [`../thoughts/adr/0067-stack-statutory-clocks.md`](../thoughts/adr/0067-stack-statutory-clocks.md).
  Add `open-clocks.json` to the export-bundle spec; extend the verifier with a
  D-3 advisory diagnostic for expired-unresolved clocks; land
  `append/014-clock-started`, `append/015-clock-satisfied`,
  `append/016-clock-elapsed`, `append/017-clock-paused-resumed`.
  **Gate:** ADR 0067 accepted.

### 7. Deferred by phase, not forgotten

- **Case ledger + agency log semantic definitions** — **M**, Phase 4.
  Core §22 case ledger composes sealed response-ledger heads with WOS
  governance events; Core §24 agency log is the operator-maintained log of
  case-ledger heads. Envelope hooks stay reserved under ADR 0003 and
  `MUST NOT populate` in Phase 1. Substance waits for Phase-4 scoping.

### 8. Sustaining maintenance

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
