# Trellis — TODO

Forward-looking tactical work only. Priority = `Imp × Debt`; size tags are
scheduling hints, never priority inputs. Work runs concurrently where
prerequisites allow, under the accepted Phase-1 principles/ADR posture
and the ratified `v1.0.0` Core + Operational Companion surface. State,
history, and gate tracking live elsewhere (see bottom).

Size: **XS** (≤1h) · **S** (≤1 session) · **M** (≤3 sessions) · **L** (multi-session).

---

## Gate — validated principles + ADRs

[`thoughts/adr/0001-0004-phase-1-mvp-principles-and-format-adrs.md`](thoughts/adr/0001-0004-phase-1-mvp-principles-and-format-adrs.md)
holds 7 accepted principles and 4 decided ADRs: **DAG envelope with
length-1 Phase-1 runtime** (ADR 0001), **list-form anchors with
single-anchor deployment default** (ADR 0002), **§22/§24 reservations held
in the envelope but MUST NOT populate in Phase 1** (ADR 0003), **Rust is
byte authority with Python retained as cross-check** (ADR 0004).

Gate status: **accepted and ratified into `v1.0.0`**. Work below executes
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

## Open

One sequence, from smallest unblocked closer to longest-prerequisite. Each
item lists its prerequisite inline where it has one. Closed work is out of
this file — see [`COMPLETED.md`](COMPLETED.md) and
[`ratification/ratification-checklist.md`](ratification/ratification-checklist.md).

1. **`tamper_kind` normative enum in Core §17.5** — **XS**.
   Values are de-facto consistent across the tamper corpus but not
   normatively enumerated. Closes a de-facto contract with spec prose.

2. **`ReasonCode` registry governance** — **XS**.
   Cross-cutting: both O-5 posture transitions (Companion A.5.1 / A.5.2)
   and ADR 0005 erasure evidence carry `reason_code` with ad-hoc enum
   values pinned inline (1–5 + 255 for both). Register `ReasonCode` per-
   family under Core §6.7 as append-only, codify `255 = Other`, decide
   namespacing across families.

3. **O-4 static lint rules 14 + 15** — **S**.
   Rule 14 validates signing-key structure without running crypto
   verification; Rule 15 (`supersedes` chain acyclicity) is unimplemented.
   Pure `scripts/check-specs.py` additions. Closes the O-4 ratification
   claim in full.

4. **Fixture-renumbering pre-merge CI guard** — **S**.
   `check_vector_lifecycle_fields()` covers deprecation/status only; no
   renumber / branch-diff logic exists. Corpus has 63 vectors with
   derivation cross-references and Rust conformance-test IDs; silent
   renumber corrupts both. Corpus-integrity protection.

5. **Key-class taxonomy ADR** — **M**.
   Before Phase-2 custody-model work (CM-D threshold, CM-F client-origin
   sovereign) opens. Core §8 defines only `SigningKeyEntry`; archived
   family separated Tenant-root / Scope / Subject / Signing / Recovery-
   only with distinct lifecycles. ADR decides tagged-union on
   `SigningKeyEntry` vs sibling CDDL types; envelope reservation lands
   now, runtime activates in Phase 2. Prevents a wire break later.
   Gap source: [`specs/archive/cross-reference-map-coverage-analysis.md`](specs/archive/cross-reference-map-coverage-analysis.md)
   §8.

6. **HPKE wrap/unwrap in Rust** — **M**.
   Core §9.4 amendment landed; `append/004-hpke-wrapped-inline` and the
   Python stranger both exercise real HPKE; `trellis-core` has no Rust
   wrap/unwrap path, so Rust only round-trips committed bytes. Strengthens
   the G-5 reproducibility-across-two-independent-implementations claim
   from "vectors match" to "both implementations do the crypto work."
   Land the Rust path + one integration test matching `append/004` byte-
   for-byte.

7. **HPKE duplicate-ephemeral detection lint** — **S**.
   *After #6.* §9.4 requires X25519 ephemeral uniqueness across every
   wrap in a ledger scope; no lint currently detects accidental reuse
   (weak-RNG / developer-error class). Deferred by design in the HPKE-
   freshness ADR until Rust-side infrastructure exists to hang the lint on.

8. **Crypto-erasure evidence — execute per ADR 0005** — **M–L**.
   [ADR 0005](thoughts/adr/0005-crypto-erasure-evidence.md) nine-step arc:
   Companion §20 rewrite + Core §6.7 + Core §19 extension → Rust decode +
   chain cross-check → first positive vector (`append/023`) → Python
   stranger mirror → remaining positives (`append/024..027`) → tamper
   vectors (`tamper/017..019`) → export catalog (`export/009` +
   `064-erasure-evidence.cbor`) → `trellis-cli erase-key` → Companion §27
   conformance extension. Steps 1–3 are the minimum for the claim to
   hold; later steps are breadth + ergonomics.

9. **Certificate-of-completion composition** — **M**.
   Human-readable signed artifact (PDF-equivalent) that an applicant
   hands to counsel, a bank, or an appeals court. Closes the DocuSign-
   replacement pitch; without it the stack is engineering-facing only.

10. **Key-rotation grace-window semantics** — **XS**.
    *Land proactively or when the first production rotation plans.* Core
    §8.4 enumerates `Active / Rotating / Retired / Revoked` but does not
    pin the overlap window where both pre- and post-rotation keys verify.
    Companion §20 prose + one boundary-crossing vector + `trellis-verify`
    dual-key acceptance during `Rotating`.

11. **Cadence subtypes beyond height-based** — **M**.
    *Land when a non-height adopter surfaces, or proactively under the
    same impulse as the fixture corpus breadth work.* `projection/003`
    and `projection/004` cover only height-based cadence; time-driven /
    event-driven / hybrid untested.

12. **O-4 ledger-replay lint rules 7–13** — **M**.
    *Land when the first external adopter publishes a declaration they
    want machine-verified against actual ledger emission history.* Seven
    declaration-vs-runtime checks: `max_agents_per_case` ceiling,
    `max_invocations_per_day` ceiling, WOS autonomy-cap superset,
    delegation-chain monotonicity, actor-discriminator on emitted events,
    `agent_identity` attribution match, emitted types ⊆ `audit.event_types`.
    Static Rules 1–6 + 11 already cover the declaration-internal surface;
    these are the runtime-cross-check rules.

13. **WOS-T4 residue — shared cross-repo fixture bundle re-seeding** — **S**.
    *Lands when the parent repo standardizes a single shared cross-stack
    fixture bundle.* Trellis consumes those declarative inputs rather than
    seeding a parallel corpus. Coordination, not a Trellis-center gap.

14. **ADR 0073 handoff residue — shared fixture alignment** — **S**.
    *Same prerequisite as #13.* Workflow-initiated attach and public-
    intake create vectors are live; the residue is consuming from one
    shared bundle rather than parallel corpora.

15. **Identity attestation bundle shape** — **S**.
    *Lands once WOS lifts `SignatureAffirmation.identityBinding` into a
    reusable shape.* Declare how a provider-neutral identity-proofing
    attestation lands as a canonical event kind and travels in the
    export bundle.

16. **Respondent Ledger ↔ Trellis `eventHash` MUST promotion** — **M**.
    *Lands after Formspec-side promotes §6.2 `eventHash` / `priorEventHash`
    from SHOULD → MUST.* Trellis-side spec amendment + conformance/lint
    checks follow the Formspec promotion.

17. **ADR 0066 execution — amendment / supersession / rescission / correction**
    — **L**, phased across Phase 1 + Phase 4.
    *Lands after parent accepts ADR 0066.* Phase 1: reserve
    `supersedes_chain_id` in the envelope header under ADR 0003
    MUST-NOT-populate discipline; land `append/011-correction`,
    `append/012-amendment`, `append/013-rescission`; extend the verifier
    with D-3 correction-preservation and rescission-terminality checks.
    Phase 4: activate supersession runtime and land
    `supersession-graph.json`.

18. **ADR 0067 execution — statutory clocks** — **M**.
    *Lands after parent accepts ADR 0067.* Add `open-clocks.json` to the
    export-bundle spec; extend the verifier with a D-3 advisory
    diagnostic for expired-unresolved clocks; land `append/014-clock-started`,
    `append/015-clock-satisfied`, `append/016-clock-elapsed`,
    `append/017-clock-paused-resumed`.

19. **`trellis.external_anchor.v1` priority interaction** — **S**, Phase 2.
    *Lands when external anchoring opens.* O-5 posture-transition events
    may want higher anchor priority in deployments with external-anchor
    chains.

20. **ADR 0005 follow-ons (erasure evidence)** — **M–L**, phased.
    Four open questions from
    [`thoughts/adr/0005-crypto-erasure-evidence.md`](thoughts/adr/0005-crypto-erasure-evidence.md)
    §"Open questions / follow-ups":
    (1) LAK rotation + erasure interaction — re-wrap cascade mode or
    coupled recipe; lands with the first live LAK rotation touching
    erasure-cascade-bearing subjects.
    (2) `hsm_receipt_kind` format registry; lands with the second
    deployment adopter on a different HSM vendor.
    (3) Legal-hold-coupled erasure lint (OC-78 vs §20.6 conflict
    detection); Phase 2.
    (4) Multi-operator quorum attestation shape; Phase 4 federation.

21. **Disclosure-profile scope granularity (per-case)** — **M**, Phase 3.
    *Lands when Phase-3 case-ledger composition opens.* Companion A.5.2
    reserves an `extensions` slot for per-case refinement; current
    semantics are deployment-scope only.

22. **Case ledger + agency log semantic definitions** — **M**, Phase 4.
    *Lands with Phase-4 scoping.* Core §22 case ledger composes sealed
    response-ledger heads with WOS governance events; Core §24 agency
    log is the operator-maintained log of case-ledger heads. Envelope
    hooks stay reserved under ADR 0003 and `MUST NOT populate` in Phase
    1 until this lands.

## Sustaining

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
| Principles + format ADRs | [`thoughts/adr/0001-0004-phase-1-mvp-principles-and-format-adrs.md`](thoughts/adr/0001-0004-phase-1-mvp-principles-and-format-adrs.md) | open the file |
| Trellis-local ADRs | [`thoughts/adr/`](thoughts/adr/) | `ls thoughts/adr/` — 0001-0004 (Phase-1 principles + format ADRs), 0005 (crypto-erasure evidence) |
| Closed work (waves, sprints, streams) | [`COMPLETED.md`](COMPLETED.md) | open the file |
| Strategy, product arc, invariants | [`thoughts/product-vision.md`](thoughts/product-vision.md) | open the file |
| In-flight design docs | [`thoughts/specs/`](thoughts/specs/) | `ls thoughts/specs/` — currently empty after 2026-04-23 audit; new design work lands here before promotion to `thoughts/adr/` or archive |
| Fixture corpus (ground truth) | `fixtures/vectors/` | `ls fixtures/vectors/*/` |
| Rust reference implementation | `crates/` | `cargo test --workspace` |
| Python cross-check (G-5 harness) | `trellis-py/` | `pip install -e trellis-py && python -m trellis_py.conformance` |
| Lint + test green | — | `python3 scripts/check-specs.py && python3 -m pytest scripts/ && cargo test --workspace` |
| Recent commits, who changed what | — | `git log --oneline` |

When a TODO grows into a spec-sized effort, move the substance to
[`thoughts/specs/`](thoughts/specs/) and replace the entry here with a
pointer. When an item lands, move it to [`COMPLETED.md`](COMPLETED.md).
This file stays forward-looking.
