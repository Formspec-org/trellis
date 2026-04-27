# Trellis — TODO

Forward-looking tactical work only. Priority = `Importance × Debt`; size tags are
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

**Cross-repo pointer — WOS Runtime §15 (Formspec coprocessor):** no Trellis-
center tasks for the core handoff (validation, mapping, draft/submit/dismiss).
Processor and HTTP parity work lives in parent [`wos-spec/TODO.md`](../../wos-spec/TODO.md)
**#66** and [`wos-spec/crates/wos-server/TODO.md`](../../wos-spec/crates/wos-server/TODO.md)
**WS-011**, **WS-074–WS-075** (plus **WS-072** for ADR 0066 server surfaces once
ratified). Trellis items **17** (ADR 0066) and **22** (case ledger) may later
consume amended responses once those stacks land.

1. **`tamper_kind` normative enum in Core §17.5** — **XS**.
   Prerequisite: add or extend a tamper vector (or conformance assertion)
   that fails today if enum values drift — then pin prose in Core §17.5.
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

5. **Key-class taxonomy — execute per ADR 0006** — **M**.
   [ADR 0006](thoughts/adr/0006-key-class-taxonomy.md): Core §8 `KeyEntry`
   + five classes; flat signing arm per ADR *Wire preservation*; Rust/Python
   dispatch; lint warn on non-`signing`; vectors `append/031..035` +
   `tamper/023..025`; reconcile ADR 0005 `key_class`. Gap source:
   [`specs/archive/cross-reference-map-coverage-analysis.md`](specs/archive/cross-reference-map-coverage-analysis.md) §8.

6. **HPKE wrap/unwrap in Rust** — **M**.
   Core §9.4 amendment landed; `append/004-hpke-wrapped-inline` and the
   Python stranger both exercise real HPKE; `trellis-core` has no Rust
   wrap/unwrap path, so Rust only round-trips committed bytes. Strengthens
   the G-5 reproducibility-across-two-independent-implementations claim
   from "vectors match" to "both implementations do the crypto work."
   Crate selection + interface sketch + verification approach pinned in
   [`thoughts/specs/2026-04-24-hpke-crate-spike.md`](thoughts/specs/2026-04-24-hpke-crate-spike.md)
   (decision: `hpke` crate, version-pinned). Land the Rust path + one
   integration test matching `append/004` byte-for-byte.

7. **HPKE duplicate-ephemeral detection lint** — **S**.
   *After #6.* §9.4 requires X25519 ephemeral uniqueness across every
   wrap in a ledger scope; no lint currently detects accidental reuse
   (weak-RNG / developer-error class). Deferred by design in the HPKE-
   freshness ADR until Rust-side infrastructure exists to hang the lint on.

8. **Crypto-erasure evidence — execute per ADR 0005** — **M–L**.
   [ADR 0005](thoughts/adr/0005-crypto-erasure-evidence.md): spec deltas
   (Companion §20 + OC-141..143) → Core §6.7/§19 → Rust decode + the
   full 10-step verifier checklist (ADR 0005 steps 1–10), with Phase-1
   chain-walk (step 8) scoped to `signing` + `subject` kids per ADR step-8
   Phase-1 bound; other classes co-land with item #5 → vectors
   `append/023..027` + `tamper/017..019` + export `009` / catalog → CLI →
   §27 tests. Expand tamper set per ADR *Fixture plan* follow-on row.

9. **Certificate-of-completion composition — execute per ADR 0007** — **M**.
   [ADR 0007](thoughts/adr/0007-certificate-of-completion-composition.md):
   `trellis.certificate-of-completion.v1` + ADR 0072 attachment binding +
   `ChainSummary` / `covered_claims` verifier cross-checks. Vectors
   `append/028..030`, `tamper/020..025`, export `010` + catalog, CLI,
   reference HTML template.

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
    *Lands after parent accepts ADR 0066.* Canonical ADR:
    [`../thoughts/adr/0066-stack-amendment-and-supersession.md`](../thoughts/adr/0066-stack-amendment-and-supersession.md).
    WOS checklist: [`../wos-spec/TODO.md#adr-0066-exec-checklist`](../wos-spec/TODO.md#adr-0066-exec-checklist).
    **Phase 1 (correction, amendment, rescission on one chain):**
    + [ ] Reserve `supersedes_chain_id` in the envelope header (Core + CDDL)
      under ADR 0003 **MUST NOT populate** lint discipline.
    + [ ] Vectors: `append/011-correction`, `append/012-amendment`,
      `append/013-rescission` under `fixtures/vectors/append/`.
    + [ ] Verifier **D-3:** correction-preservation (original + corrected
      field values in report output when a correction-shaped event is in
      scope); rescission-terminality (any determination after
      `DeterminationRescinded` on the same chain → integrity violation).
    + [ ] Core §17 / §19 prose + any export-manifest hooks needed for Phase-1
      verifier inputs (coordinate with Formspec `ResponseCorrection` and WOS
      payload shapes as they land).
    **Phase 4 (supersession runtime + cross-chain bundle):**
    + [ ] Activate `supersedes_chain_id` population when the phase gate opens.
    + [ ] Verifier **D-3 chain-linkage:** superseding header cites predecessor
      checkpoint hash with byte equality.
    + [ ] Normative **`supersession-graph.json`** at bundle root; verifier BFS
      over `head_chain_id` / `predecessors`; **cycles = integrity failure**
      (ADR default — note open Q2 alternative: linear-only Phase 1).
    + [ ] Optional predecessor chain members in export bundle (ADR D-4).

18. **ADR 0067 execution — statutory clocks** — **M**.
    *Lands after parent accepts ADR 0067.* Coordinate payload hashes with WOS
    `clockStarted` / `clockResolved` (parent [`wos-spec/TODO.md`](../wos-spec/TODO.md#adr-0067-exec-checklist)).
    + [ ] **Export bundle:** normative **`open-clocks.json`** at bundle root
      (D-3): enumerate open clocks `{ clock_id, clock_kind, computed_deadline,
      origin_event_hash }` for every `clockStarted` lacking a matching
      `clockResolved` at export time.
    + [ ] **Verifier — D-3 advisory:** for each open clock with
      `computed_deadline < bundle.sealed_at` and no `clockResolved`, emit an
      **advisory** diagnostic (not an integrity failure).
    + [ ] **Verifier — D-4 composition:** walk the chain to compose pause
      segments (`clockResolved` paused → subsequent `clockStarted` residual)
      into cumulative duration / segment accounting for audit tooling.
    + [ ] **Vectors:** `append/014-clock-started`, `015-clock-satisfied`,
      `016-clock-elapsed`, `017-clock-paused-resumed` (+ matching export/verify
      corpus hooks as needed for byte-identity CI).

19. **`trellis.external_anchor.v1` priority interaction** — **S**, Phase 2.
    *Lands when external anchoring opens.* O-5 posture-transition events
    may want higher anchor priority in deployments with external-anchor
    chains. Anchor substrate is adapter-tier per the DI-first
    [anchor-substrate spike](thoughts/specs/2026-04-24-anchor-substrate-spike.md)
    — center ships an `AnchorAdapter` trait + enumerates OpenTimestamps,
    Sigstore Rekor, and Trillian as first-class candidates; adopters pick
    per deployment. This item is the priority-policy decision per
    deployment (which adapter's `anchored_at_or_before` drives posture
    transition priority when multiple adapters attest), declared in the
    Posture Declaration.

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

23. **Interop sidecar reservation — execute per ADR 0008** — **S**, Phase 1.
    [ADR 0008](thoughts/adr/0008-interop-sidecar-discipline.md) registers
    four ecosystem-derivation sidecar kinds (`scitt-receipt`,
    `vc-jose-cose-event`, `c2pa-manifest`, `did-key-view`) under
    canonical-first, deterministic, additive discipline. Phase-1 scope is
    **reservation only**: Core §18 export-manifest gains `interop_sidecars:
    [* InteropSidecarEntry] / null`; Phase-1 producers emit null/empty;
    Phase-1 verifiers reject any populated entry with
    `interop_sidecar_phase_1_locked`. Vectors `export/011-012` +
    `tamper/027-031` per ADR *Fixture plan*. Also scaffolds empty crates
    `trellis-interop-{scitt,vc,c2pa,did}` + `cargo-deny` config forbidding
    ecosystem libs from `trellis-core` / `trellis-verify` / `trellis-types`
    (ADR 0008 ISC-05 hygiene contract).

24. **`scitt-receipt` adapter — execute per ADR 0008** — **M**, Phase 2+.
    *Lands when SCITT Architecture draft reaches WG Last Call OR a
    concrete adopter requires SCITT-compatible checkpoint receipts,
    whichever fires first.* Implements `trellis-interop-scitt` against
    the ADR 0008 §"Registry" field-mapping table for `derivation_version
    = 1` (semantic-alignment mode). Re-signs the SCITT signed statement
    with a distinct SCITT-issuer key managed by the operator's SCITT
    service (not the checkpoint COSE signer). Unlocks the `scitt-receipt`
    kind in the Phase-1 verifier's registry; adds round-trip byte-exact
    vectors per kind. Follow-up: `derivation_version = 2` when SCITT
    adopts a byte-conformance profile.

25. **`vc-jose-cose-event` adapter — execute per ADR 0008** — **M**, Phase 2+.
    *Lands when an SSI-native adopter (deployment that standardizes on
    W3C VC 2.0 event envelopes) shows up.* Implements
    `trellis-interop-vc` per ADR 0008 §"Registry" illustrative VC shape.
    Requires resolving the three open questions in ADR 0008: VC
    `@context` hosting + content hash, issuer-resolution policy, and
    the Posture-Declaration binding for ISC-08 payload-disclosure
    honesty per event kind. Unlocks the `vc-jose-cose-event` kind.

26. **`c2pa-manifest` adapter — execute with ADR 0007** — **M**, Phase 2+.
    *Co-lands with ADR 0007 implementation sequencing step 9
    (reference template).* Implements `trellis-interop-c2pa` per
    ADR 0008 §"Registry"; layers C2PA manifest emission on the
    reference HTML-to-PDF pipeline so the presentation artifact ships
    with a `trellis.certificate-of-completion.v1` assertion pinning
    `certificate_id`, `canonical_event_hash`,
    `presentation_artifact.content_hash`, signer `kid`, and canonical
    COSE_Sign1 digest. Requires C2PA assertion-label registration (may
    need C2PA coalition membership step per ADR 0008 open question 3).
    Unlocks the `c2pa-manifest` kind.

27. **`did-key-view` adapter — execute with item #5 (ADR 0006)** — **XS**, Phase 2+.
    *Co-lands with ADR 0006 `KeyEntry` migration (item #5 above).*
    Implements `trellis-interop-did` per ADR 0008 §"Registry" — a
    one-way labeling view mapping each signing-class `kid` to its
    `did:key` rendering under the Ed25519 multicodec. No signing, no
    network, no verification-behavior change (the `did:key` IS the
    public key). Unlocks the `did-key-view` kind. Non-signing key
    classes are explicitly out of scope; a future `did-tenant-root-view`
    or similar requires a separate landing ADR.

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
| Trellis-local ADRs | [`thoughts/adr/`](thoughts/adr/) | `ls thoughts/adr/` — 0001-0004 (Phase-1 principles + format ADRs), 0005 (crypto-erasure evidence), 0006 (key-class taxonomy), 0007 (certificate-of-completion composition) |
| Closed work (waves, sprints, streams) | [`COMPLETED.md`](COMPLETED.md) | open the file |
| Strategy, product arc, invariants | [`thoughts/product-vision.md`](thoughts/product-vision.md) | open the file |
| In-flight design docs | [`thoughts/specs/`](thoughts/specs/) | `ls thoughts/specs/` — G-3 fixture system design (active lint contract); 2026-04-24 HPKE + anchor-substrate spikes; new work lands here before promotion to `thoughts/adr/` or `thoughts/archive/specs/` |
| Fixture corpus (ground truth) | `fixtures/vectors/` | `ls fixtures/vectors/*/` |
| Rust reference implementation | `crates/` | `cargo test --workspace` |
| Python cross-check (G-5 harness) | `trellis-py/` | `pip install -e trellis-py && python -m trellis_py.conformance` |
| Lint + test green | — | `python3 scripts/check-specs.py && python3 -m pytest scripts/ && cargo test --workspace` |
| Recent commits, who changed what | — | `git log --oneline` |
| Archived Trellis ADRs (landing zone) | `thoughts/archive/adr/` | create when first ADR moves out of `thoughts/adr/` — today empty |

When a TODO grows into a spec-sized effort, move the substance to
[`thoughts/specs/`](thoughts/specs/) and replace the entry here with a
pointer. When an item lands, move it to [`COMPLETED.md`](COMPLETED.md).
This file stays forward-looking.
