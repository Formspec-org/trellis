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

### Open forks — need owner call before related sequence items execute cleanly

The 2026-04-24 semi-formal code review surfaced five architectural forks
that block (or weaken) downstream sequence items until the owner picks a
branch. Default leans are mine; owner call overrides.

- **Fork A — ADR 0005 step-5 erasure cascade scope.** Current step-5 chain
  cross-check inspects only `COSE_Sign1 kid` + direct `key_bag.entries`
  wraps. `scope` / `tenant-root` key destructions cannot be enforced on
  descendant keys without a lineage walk. Options:
  (a) co-land ADR 0005 + ADR 0006 with lineage walk;
  (b) Phase-1 scope ADR 0005 step-5 to `signing` + direct-wrap kids only,
  document cross-class as Phase-2 work;
  (c) accept gap and widen adversary-model section.
  **Default: (b).** Affects item #8.
- **Fork B — Key-class closure escape.** ADR 0006 `kind: … / tstr` admits
  unregistered classes with no registration contract, contradicting the
  closed-taxonomy discipline. Options:
  (a) drop the escape; five classes are final, witness / federation-delegate
  arrive via future ADR bump;
  (b) keep the escape; land a Core §8.n registry parallel to §6.7.
  **Default: (a).** Affects item #5.
- **Fork C — Phase-1 non-signing lint posture.** ADR 0006 currently says
  "warn on non-signing entries"; ADR 0003 discipline on reserved fields is
  MUST-NOT-populate → fail. Options:
  (a) fail (match ADR 0003);
  (b) silent-accept (reservation valid, unused);
  (c) warn (current middle ground).
  **Default: (a).** Affects item #5.
- **Fork D — ADR 0005 `"wrap"` vs ADR 0006 `"subject"`.** ADR 0005 `key_class`
  enum collapsing `"wrap"` → `"subject"` is only correct when every wrap
  key is subject-bound — future CM-D threshold custody introduces group /
  session wrap keys that aren't subject-bound. Options:
  (a) drop `"wrap"` entirely from ADR 0005's enum; `"subject"` is the only
  wrap-carrying class in Phase 1; CM-D adds `"session-wrap"` as a new class
  when it lands;
  (b) keep `"wrap"` as a functional-role field distinct from identity-class.
  **Default: (a).** Affects items #5 + #8.
- **Fork E — Shared-bundle cross-reference extraction grammar.** Parent
  shared-bundle design's `[expected_report.cross_references.row]` tables
  declare `from` / `to` paths as descriptive prose, not executable
  extraction rules. A misimplemented runner could `crossref_resolved = true`
  by aliasing both endpoints into the same `_common/` file. Options:
  (a) JSON Pointer (RFC 6901) per-submodule;
  (b) Rust trait with per-submodule extractors;
  (c) inline discipline invariant "from / to bytes MUST come from different
  source files."
  **Default: (a).** Blocks parent's bundle-001 scaffold landing; affects
  items #13 + #14 only once bundle 001 is green.

### Sequence

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
   (Companion §20 + OC-141..143) → Core §6.7/§19 → Rust decode + Phase-1–scoped
   step-5 chain checks → vectors `append/023..027` + `tamper/017..019` +
   export `009` / catalog → CLI → §27 tests. Expand tamper set per ADR
   *Fixture plan* follow-on row.

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

23. **Review follow-through — 2026-04-24 semi-formal code review** — **S–M**.
    Six-agent parallel review produced a findings dump; judgment-call
    items are in the Open forks block above; the remainder are mechanical
    fold-ins to execute alongside or inside the relevant sequence items.
    Grouped by parent item:

    *Verifier hardening (runs alongside item #1 or as its own small PR):*
    - **Dual-key decoder mutual-exclusion** in `trellis-verify`
      (`crates/trellis-verify/src/lib.rs:1561-1573`) and Python stranger.
      An event carrying both `trellis.custody-model-transition.v1` AND
      `trellis.disclosure-profile-transition.v1` currently returns only
      the first-matched decode; disclosure-profile branch silently skips.
      Same class as the G-O-5 reopen, one decoder-level deeper. Add
      structure failure on both-keys-present, plus `tamper/0NN-event-carries-both-transition-keys`.
    - **`scope_change` closed enum** (Rust + Python). Currently decoded as
      `tstr`; spec pins three values (`Narrowing` / `Widening` / `Orthogonal`).
      Raise structure failure on unknown values; mirror in Python stranger.
    - **Narrowing + Widening fixture coverage.** Current tamper/016 exercises
      only Orthogonal + dual attestation. Add `append/0NN-narrowing-single-attestation`
      (positive) + `tamper/0NN-widening-single-attestation` (negative).

    *Folds into item #5 (key-class taxonomy execution) once Forks B + C + D decided:*
    - CDDL disjoint-union for `KeyAttributes` per variant (the existing
      choice group is correct; doc cross-field rules for `RecoveryKeyAttributes`
      quorum / quorum_set).
    - ADR 0005 `key_class` enum reconciliation per Fork D outcome.

    *Folds into item #6 (HPKE Rust) execution:*
    - `WrapResult` shape — return `{ ephemeral_pubkey, wrapped_dek }` where
      `wrapped_dek = ciphertext || tag` per RFC 9180 Seal contract, not
      three split fields. Matches Core §9.4 `wrapped_dek` bytestring shape
      1:1; no caller reassembly.
    - Verification expansion — generate N=100 wraps, assert ephemeral
      uniqueness + X25519 pubkey validity. Supplements the round-trip
      test; stages the item-#7 dup-lint on real implementation.
    - Pin `hpke` crate version in `Cargo.toml` and record it in the
      requirements matrix as load-bearing for G-5 byte conformance.

    *Folds into item #8 (crypto-erasure execution) once Fork A decided:*
    - Rewrite `SubjectScope` as a CDDL choice group keyed on `kind`
      (`PerSubjectScope` / `PerLedgerScope` / `PerTenantScope` /
      `DeploymentWideScope`), not parallel-nullable siblings. Step-2
      cross-field check collapses to tautology.
    - Expand tamper corpus: `tamper/0NN-erasure-subject-scope-mismatch`,
      `tamper/0NN-erasure-hsm-receipt-kind-asymmetric`,
      `tamper/0NN-erasure-attestation-bad-sig`,
      `tamper/0NN-erasure-dual-attestation-missing` (reason_code=3).
      Current plan covers 2 of ~7 localizable failures.
    - Clarify `VerificationReport` shape: `erasure_evidence` is an
      in-`event_failures` fold with codes (`post_erasure_use`,
      `post_erasure_wrap`, `erasure_attestation_invalid`), not a new
      top-level CDDL field. Name the Core §19.1 edit verbatim.
    - Replace `"opaque-vendor-receipt-v1"` catch-all for `hsm_receipt_kind`
      with a registered set: `"aws-kms-destroy-v1"`, `"pkcs11-c-destroyobject-v1"`,
      `"gcp-kms-destroy-v1"`, `"azure-kv-delete-v1"`, sentinel `"unregistered"`
      with verifier warning. Register under Core §6.7 or new Companion A.8.
    - Promote legal-hold-coupled erasure detection from Phase-2 deferral
      to Phase-1 verifier MAY warning (OC-74 already pins precedence;
      verifier has the signals).
    - Pin `evidence_id` ↔ `idempotency_key` relationship explicitly
      (e.g., `idempotency_key = dCBOR(evidence_id)`). Core §17.3 mismatch
      rejection is the failure mode for payload-mutation retry.
    - Pin `destruction_actor` / `policy_authority` / `attestations[].authority`
      cross-field relationship (who MUST appear as an attesting authority,
      with which `authority_class`).

    *Folds into item #9 (certificate-of-completion execution):*
    - Pin `signed_at` comparison target: compare against WOS-payload
      `signedAt` (semantic); envelope `authored_at` is a skew-sanity
      bound (±N seconds declared in Posture Declaration). Drop the
      prose "within the window" phrasing that's currently ambiguous.
    - Expand tamper corpus to cover all 7 verifier-obligation failure
      steps (currently 3/7 covered). Add `tamper/0NN-cert-signed-at-outside-window`,
      `tamper/0NN-cert-workflow-status-inconsistent`,
      `tamper/0NN-cert-response-ref-mismatch`,
      `tamper/0NN-cert-attestation-invalid`.
    - Promote reference HTML template from step-9 adopter-ergonomics to
      a Phase-1 deliverable gated on first COC vector (a day of work,
      closes "will adopters actually write their own" decisively).
    - Align Core §19 CDDL `VerificationReport` update with item #8's
      erasure-evidence update — one commit landing both parallel arrays
      rather than two independent Core §19 edits.
    - Rename "rejected Option D" to "rejected Option C" (Option D is
      the chosen approach, letter collision confuses readers).

    *Folds into item #19 (external_anchor priority) — anchor-substrate spike refinements:*
    - Pin `AnchorAdapter::verify()` trust-root loading: verifier's trust
      roots (Bitcoin headers, Rekor public keys, Trillian log pubkeys)
      load out-of-band from the export, symmetric with Core §16
      verification-independence. Receipt need not be fully self-contained;
      resolvable against a pinned adopter-declared trust bundle.
    - Add sixth selection axis: "verification horizon" (substrate
      survivability over the years the export must stay verifiable).
    - Flag Trillian-private-without-witness-mesh as "operator is their
      own anchor = tamper-evidence against self, not third-party attestation."
    - Consider integer `anchor_suite_id: uint` with Trellis-governed
      registry, parallel to Core §7.2 `suite_id`, instead of vendor-brand
      strings baked into `kind`. Decouples wire from vendor naming.
    - Name `AnchorAdapterRegistry` as a first-class center-owned subsystem
      in §Adapter trait.

    *Parent + WOS cross-repo follow-through (coordination, not Trellis-center):*
    - Parent `README.md:83` + `CLAUDE.md:169` were flagged as citing
      archived fixture-system-design; **CLAUDE.md:169 was repaired
      inline by the user's 2026-04-24 edits**; parent `README.md:83`
      needs grep-verification on next parent-repo touch.
    - 16-doc broader audit (`thoughts/audit-2026-04-23-design-docs-vs-specs-and-code.md`)
      still lists Finding #1 (O-5 disclosure-profile verifier gap) as
      open. That gap was closed same-session. Add a post-hoc header note
      on the audit doc so a cold reader doesn't re-open the work.
    - `scripts/check-specs.py` now ~1818 lines vs its own 800-line
      revisit threshold; user profile "don't refactor for fun" says
      leave it until it actively blocks a new rule.
    - Decide whether to move `## Sustaining` bucket items
      (Rust-within-hours, declarative inputs) into `CLAUDE.md` as
      standing norms rather than TODO entries.

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
