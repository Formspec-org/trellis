# Trellis — TODO

Forward-looking tactical work only. Priority = `Importance × Debt`; size tags are
scheduling hints, never priority inputs. Work runs concurrently where
prerequisites allow, under the accepted principles/ADR posture and the
ratified `v1.0.0` Core + Operational Companion surface. State, history,
and gate tracking live elsewhere (see bottom).

Size: **XS** (≤1h) · **S** (≤1 session) · **M** (≤3 sessions) · **L** (multi-session).

---

## Gate — validated principles + ADRs

[`thoughts/adr/0001-0004-phase-1-mvp-principles-and-format-adrs.md`](thoughts/adr/0001-0004-phase-1-mvp-principles-and-format-adrs.md)
holds 7 accepted principles and 4 decided ADRs: **DAG envelope with
length-1 runtime** (ADR 0001), **list-form anchors with single-anchor
deployment default** (ADR 0002), **§22/§24 reservations held in the
envelope but `MUST NOT populate`** (ADR 0003), **Rust is byte authority
with Python retained as cross-check** (ADR 0004). (The historical
filename retains the `phase-1-mvp` slug; the prose vocabulary is
deprecated.)

Gate status: **accepted and ratified into `v1.0.0`**. Work below executes
against this posture.

No separate executable-dispatch doc is maintained. Open work is enumerated in
this file; closed work lives in [`COMPLETED.md`](COMPLETED.md) and
[`ratification/ratification-checklist.md`](ratification/ratification-checklist.md).

**On the `v1.0.0` tag — snapshot, not a freeze.** G-5 passed and `v1.0.0`
is tagged, but nothing is released and no production records exist.
Economic model: coding, time, and compute are cheap; architectural tech
debt we'd have to unwind later is the only expensive cost. If an
architectural change to the wire shape, verification contract, or
export layout prevents future debt, make it and retag. The revision
window is not closed — only real adopters can close it, and there are
none yet.

---

## Open

One sequence, from smallest unblocked closer to longest-prerequisite. Each
item lists its prerequisite inline where it has one. Closed work is out of
this file — see [`COMPLETED.md`](COMPLETED.md) and
[`ratification/ratification-checklist.md`](ratification/ratification-checklist.md).

**Cross-repo pointer — parent PLANNING.md backlog.** Stack-wide cross-spec
coordination lives in [`/PLANNING.md`](../PLANNING.md) as `PLN-XXXX` rows.
Trellis-implicated rows are referenced inline below where the mapping is
clean (items 1-4, 8-13, 18, 21, 24-29, 36-40). Trellis-internal rows
(5-7, 14-17, 19-20, 22-23) carry no parent counterpart by design — they
are envelope/verifier discipline that nothing downstream gates. The
MVP-foundation cluster (PLN-0331..0349) consumes Trellis crates
downstream; `trellis-cose` / `trellis-verify` public APIs are
"keep stable for composition" with no new rows. Cross-submodule Cargo
path-dep posture (parent **PLN-0368**) is a stack-level decision;
Trellis-side action is "comply with the chosen pattern when it lands."

**`trellis-store-postgres` ↔ downstream `EventStore` (continuation):** the
Postgres canonical store (`trellis/crates/trellis-store-postgres/`, Wave 16/18
hardening + `append_event_in_tx`) is the **Trellis-side** half of the composed
reference-server write path. **WOS-side** work is the
`wos-server-eventstore-postgres` adapter (parent **PLN-0332**) gated by
**PLN-0368** — see [`wos-spec/crates/wos-server/TODO.md`](../wos-spec/crates/wos-server/TODO.md)
+ **WS-095** (embedded / `trellis-store-memory` for single-process).
Trellis crate owners: keep `append_event_in_tx` and migration discipline stable
for that composition; no new Trellis TODO row unless a gap blocks the adapter.

**Signature-stack rows in this file** (cluster pointer for "everything
about signatures"): item **#4** (cert-of-completion ADR 0007 — the
integrity artifact for ESIGN/UETA), **#8** + **#9** (WOS-T4 + ADR 0073
shared-fixture residue), **#10** (identity attestation, supersedes
PLN-0310 → PLN-0381), **#21** (c2pa-manifest adapter — layers the
certificate onto the presentation PDF), **#36** (user-content
Attestation primitive — Trellis ADR 0010, parent PLN-0379), ~~**#37**
(AEAD nonce determinism — closed Wave 19, parent PLN-0383)~~, **#38**
(`custody-hook-encoding.md` v1.0 — the four-field append wire surface
that carries `producer_signature`, parent PLN-0385), **#39** (external
recipient lifecycle — recipients of signed events; access events,
parent PLN-0382), **#40** (tenant-scope export — bundles include
signed events, parent PLN-0392).
The integrity artifact is Trellis's; the semantics are WOS's; both
compose under PLN-0379 + PLN-0380. Parent stack closure cluster
spans PLN-0379..0398: open-contract closure (0379-0385), engineering
scaffolds (0386-0389), drift-prevention guards (0390-0391), profile-
specific extensions and procurement triggers (0392-0398) + PLN-0355
(ESIGN gate, Trigger) + PLN-0370 (DocuSign reframe).

**Cross-repo pointer — WOS Runtime §15 (Formspec coprocessor):** no Trellis-
center tasks for the core handoff (validation, mapping, draft/submit/dismiss).
Processor and HTTP parity work lives in parent [`wos-spec/TODO.md`](../wos-spec/TODO.md)
**#66** and [`wos-spec/crates/wos-server/TODO.md`](../wos-spec/crates/wos-server/TODO.md)
**WS-011**, **WS-074–WS-075** (plus **WS-072** for ADR 0066 server surfaces once
ratified). Trellis items **12** (ADR 0066) and **17** (case ledger) may later
consume amended responses once those stacks land.

1. ~~**Key-class taxonomy — execute per ADR 0006**~~ — **CLOSED Wave 17, 2026-04-27.**
   Spec + Rust + Python + lint + matrix landed via commits `3327cbe` /
   `1b2886c` / `acfef57` / `cf0e4fd`. Core §8 renamed "Signing-Key
   Registry" → "Key Registry"; §8.7 unified `KeyEntry` taxonomy with
   five classes (`signing` / `tenant-root` / `scope` / `subject` /
   `recovery`) + extension `tstr` escape; legacy `SigningKeyEntry`
   retained byte-stable so the v1.0.0 corpus does not regenerate;
   verifiers dispatch on top-level `kind` presence. ADR 0005
   `"wrap"` → `"subject"` reconciliation landed in §8.7.6. **Vector
   corpus** (`append/031..035` + `tamper/023..025`) landed in the same
   Wave 17 train; tamper vectors plumb `key_entry_attributes_shape_mismatch`
   (TR-CORE-048) through a typed `VerifyError.kind` tag on both runtimes;
   TR-CORE-049 (unknown-`kind`) demoted to `Verification = prose` per
   ADR 0006 *Fixture plan* deferral. See [`COMPLETED.md`](COMPLETED.md)
   Wave 17 lead entry.

2. ~~**HPKE duplicate-ephemeral detection lint**~~ — **CLOSED Wave 17, 2026-04-27.**
   `scripts/check-specs.py` rule R17 walks every event payload in the
   corpus and rejects any `(ledger_scope, ephemeral_pubkey)` recurrence
   across distinct vector dirs plus any duplicate `ephemeral_pubkey`
   inside a single `key_bag.entries`. Anchored at TR-CORE-033 and
   Core §9.4. See [`COMPLETED.md`](COMPLETED.md) Wave 17 entry. Renumbering
   of items #3..#29 deferred to the wave's final landing pass.

3. ~~**Crypto-erasure evidence — Stages 2-5**~~ — **CLOSED Wave 21, 2026-04-28.**
    Stage 1 (spec deltas: Companion §20.6, Core §6.7/§19, matrix rows,
    `tamper_kind` enum pre-declaration) closed Wave 18 via `9b3d3e4`.
    Stages 2-3 + 4-A landed Wave 19 via `586de5e` / `53fc25c` / `dd408b6`.
    Stages 4-B / 4-C / 5 plus the working-tree verifier + Python-parity
    follow-on landed Wave 21 in a 9-commit train: slot collision
    resolution (`refactor(fixtures): renumber export/009 intake-handoffs
    → export/013` + R16 deprecated tombstone at slot 009), Rust verifier
    erasure-export-catalog cross-check + step-8 chain-walk extension,
    Python parity, tamper vectors `append/017..019`, export bundle
    `export/009-erasure-evidence-inline` + 432-line generator, CLI
    `erase-key` initial stub, Companion §27.1 verifier-surface prose,
    and matrix promotion (TR-OP-105 / TR-OP-107 prose → `test-vector`;
    TR-OP-106 / TR-OP-108 / TR-OP-109 / TR-OP-113 stay `prose` /
    `declaration-doc-check` per ADR 0005 *Fixture plan* follow-on).
    See [`COMPLETED.md`](COMPLETED.md) Wave 21 entry.
    *Bundle pointer:* **this closes parent `PLN-0312` entirely** —
    the foundational crypto execution bundle has no remaining row.

4. ~~**Certificate-of-completion composition — execute per ADR 0007**~~ — **CLOSED Wave 22, 2026-04-28.**
   Spec deltas (Core §6.7 / §9.8 / §19 step 6c, Companion §27.1, matrix
   TR-CORE-146..151 + TR-OP-131/132) closed Waves 18-21 via `f968663` /
   `517ec5e` / `d0043de` / `97c2082` / `c1613b2` / `00b6303` / `1cc5320`.
   Wave 22 closed the corpus + downstream surfaces in an 8-commit train:
   positive vectors `append/028..030` (PDF-minimal / dual-signer-with-
   template / HTML-template-bound), ledger-only tampers `021/023/025/026`
   (signer-count mismatch / attestation truncation / HTML-without-template-
   hash CDDL reject / certificate_id collision), verifier multi-event-chain
   indexing fix (`c9f46cc` — `cert_events` Vec→BTreeMap on global
   `event_index`), export bundle `export/010-certificate-of-completion-
   inline` + 432-line generator + `065-certificates-of-completion.cbor`
   catalog + tampers `020/022/024` (content-hash / signing-event-unresolved
   / response-ref-mismatch), Python parity in `trellis-py.conformance` +
   `verify.py` + 23 new pytest cases (G-5 84/0/0 → 95/0/0), `trellis-cli
   seal-completion --help` flag-contract stub, reference HTML template at
   `reference/certificate-of-completion/template-v1/` (template.html +
   template.css + README + template_hash.txt), and matrix promotion of
   TR-CORE-146..151 from `prose` → `test-vector` (TR-OP-131 retains its
   posture per fixture coverage in `tamper/025`; TR-OP-132 retains
   `declaration-doc-check` per matrix design). See [`COMPLETED.md`](COMPLETED.md)
   Wave 22 entry.
   *Signature stack closure:* this lands the integrity artifact for
   ESIGN / UETA compliance. Cross-stack composition with parent **PLN-0067**
   (WOS-T4 signature-complete bundle `001`), **PLN-0355** (ESIGN/UETA
   gate, Trigger), **PLN-0370** (marketing reframe), **PLN-0379**
   (Trellis user-content Attestation primitive), and **PLN-0398**
   (DocuSign 100% admin surface, Trigger) is now Trellis-side ready;
   WOS-side semantic work and parent gates close on their own clocks.
   The c2pa-manifest adapter at item #21 layers the certificate onto the
   presentation PDF in a downstream ADR slot.

5. **Key-rotation grace-window semantics** — **XS**.
    *Land proactively or when the first production rotation plans.* Core
    §8.4 enumerates `Active / Rotating / Retired / Revoked` but does not
    pin the overlap window where both pre- and post-rotation keys verify.
    Companion §20 prose + one boundary-crossing vector + `trellis-verify`
    dual-key acceptance during `Rotating`.

6. **Cadence subtypes beyond height-based** — **M**.
    *Land when a non-height adopter surfaces, or proactively under the
    same impulse as the fixture corpus breadth work.* `projection/003`
    and `projection/004` cover only height-based cadence; time-driven /
    event-driven / hybrid untested.

7. **O-4 ledger-replay lint rules 7–13** — **M**.
    *Land when the first external adopter publishes a declaration they
    want machine-verified against actual ledger emission history.* Seven
    declaration-vs-runtime checks: `max_agents_per_case` ceiling,
    `max_invocations_per_day` ceiling, WOS autonomy-cap superset,
    delegation-chain monotonicity, actor-discriminator on emitted events,
    `agent_identity` attribution match, emitted types ⊆ `audit.event_types`.
    Static Rules 1–6 + 11 already cover the declaration-internal surface;
    these are the runtime-cross-check rules.

8. **WOS-T4 residue — shared cross-repo fixture bundle re-seeding** — **S**.
    *Lands when the parent repo standardizes a single shared cross-stack
    fixture bundle.* Trellis consumes those declarative inputs rather than
    seeding a parallel corpus. Coordination, not a Trellis-center gap.
    Parent backlog: **PLN-0067** (shared bundle), **PLN-0068** (required
    negative — response-hash mismatch), **PLN-0069** (CI/conformance gate).

9. **ADR 0073 handoff residue — shared fixture alignment** — **S**.
    *Same prerequisite as #8 (shared cross-stack fixture bundle).*
    Workflow-initiated attach and public-intake create vectors are
    live; the residue is consuming from one shared bundle rather than
    parallel corpora. Parent backlog: **PLN-0067**.

10. **Identity attestation bundle shape** — **S**.
    *Lands after parent ratifies the stack ADR for IdentityAttestation
    per **PLN-0381**.* Synthesis-merge 2026-04-27 promoted identity
    attestation from Trigger to P0 center commitment; **PLN-0310 closed
    by supersession** (PLN-0381 carries the work). Trellis-side action:
    declare how a provider-neutral identity-proofing attestation lands
    as a canonical event kind and travels in the export bundle.
    Composes with the new `wos.identity.*` event taxonomy (parent
    **PLN-0384** — gates the namespace) and with PLN-0380 (signer-
    authority claim shape, distinct from authentication-method). Cross-
    stack fixtures (Formspec → WOS provenance → Trellis envelope) prove
    the identity claim composes.

11. **Respondent Ledger ↔ Trellis `eventHash` MUST promotion** — **M**.
    *Lands after Formspec-side promotes §6.2 `eventHash` / `priorEventHash`
    from SHOULD → MUST.* Trellis-side spec amendment + conformance/lint
    checks follow the Formspec promotion. Parent backlog: **PLN-0311**
    (Respondent Ledger offline-authoring profile + chain semantics).

12. **ADR 0066 execution — amendment / supersession / rescission / correction**
    — **L**.
    *Lands after parent accepts ADR 0066.* Canonical ADR:
    [`../thoughts/adr/0066-stack-amendment-and-supersession.md`](../thoughts/adr/0066-stack-amendment-and-supersession.md).
    WOS checklist: [`../wos-spec/TODO.md#adr-0066-exec-checklist`](../wos-spec/TODO.md#adr-0066-exec-checklist).
    Parent backlog: **PLN-0055**, **PLN-0056**, **PLN-0050**
    (`ResponseCorrection` linkage shape), **PLN-0051**
    (supersession-start linkage).
    + [ ] Envelope header carries `supersedes_chain_id` (Core + CDDL); for
      genesis events on a non-superseding chain it is null.
    + [ ] Single-chain vectors: `append/011-correction`,
      `append/012-amendment`, `append/013-rescission` under
      `fixtures/vectors/append/`.
    + [ ] Verifier **D-3:** correction-preservation (original + corrected
      field values in report output); rescission-terminality (any
      determination after `DeterminationRescinded` on the same chain →
      integrity violation); chain-linkage (superseding header cites
      predecessor checkpoint hash with byte equality).
    + [ ] Core §17 / §19 prose + export-manifest hooks for verifier inputs
      (coordinate with Formspec `ResponseCorrection` + WOS payload shapes).
    + [ ] Cross-chain: normative `supersession-graph.json` at bundle root;
      verifier BFS over `head_chain_id` / `predecessors`; cycles =
      integrity failure (ADR default — note open Q2 alternative:
      linear-only).
    + [ ] Optional predecessor chain members in export bundle (ADR D-4).

13. **ADR 0067 execution — statutory clocks** — **M**.
    *Lands after parent accepts ADR 0067.* Coordinate payload hashes with WOS
    `clockStarted` / `clockResolved` (parent [`wos-spec/TODO.md`](../wos-spec/TODO.md#adr-0067-exec-checklist)).
    Parent backlog: **PLN-0159** (`open-clocks.json` export), **PLN-0160**
    (verifier diagnostics with chosen severity per **PLN-0170**), **PLN-0161**
    (pause/resume verifier composition), **PLN-0162** (vectors), **PLN-0164**
    (cross-stack clock-composition fixture).
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

14. **`trellis.external_anchor.v1` priority interaction** — **S**.
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

15. **ADR 0005 follow-ons (erasure evidence)** — **M–L**.
    Four open questions from
    [`thoughts/adr/0005-crypto-erasure-evidence.md`](thoughts/adr/0005-crypto-erasure-evidence.md)
    §"Open questions / follow-ups":
    (1) LAK rotation + erasure interaction — re-wrap cascade mode or
    coupled recipe; lands with the first live LAK rotation touching
    erasure-cascade-bearing subjects.
    (2) `hsm_receipt_kind` format registry; lands with the second
    deployment adopter on a different HSM vendor.
    (3) Legal-hold-coupled erasure lint (OC-78 vs §20.6 conflict
    detection).
    (4) Multi-operator quorum attestation shape — co-lands with the
    first federated deployment.

16. **Disclosure-profile scope granularity (per-case)** — **M**.
    *Lands when case-ledger composition opens.* Companion A.5.2
    reserves an `extensions` slot for per-case refinement; current
    semantics are deployment-scope only.

17. **Case ledger + agency log semantic definitions** — **M**.
    *Lands when case-ledger / agency-log scoping opens.* Core §22
    case ledger composes sealed response-ledger heads with WOS
    governance events; Core §24 agency log is the operator-maintained
    log of case-ledger heads. Envelope hooks stay reserved under
    ADR 0003 and `MUST NOT populate` until this lands.

18. ~~**Interop sidecar reservation — execute per ADR 0008**~~ — **CLOSED Wave 20, 2026-04-27.**
    Core §18.3a gains `interop_sidecars` field with `InteropSidecarEntry`
    CDDL; lock-off prose; TR-CORE-145 matrix row.
    `trellis-verify` rejects non-empty `interop_sidecars` with
    `interop_sidecar_phase_1_locked` fatal failure. Fixtures:
    `export/011-interop-sidecars-absent` (canonical positive, absent),
    `export/012-interop-sidecars-empty-list` (canonical positive, empty
    array), `tamper/027-interop-sidecar-populated-phase-1` (verifier
    rejects populated entry). Empty crates
    `trellis-interop-{scitt,vc,c2pa,did}` scaffolded in workspace;
    `deny.toml` cargo-deny config reserves ecosystem-lib ban list.
    `scripts/check-specs.py` TAMPER_KIND_ENUM extended.

19. **`scitt-receipt` adapter — execute per ADR 0008** — **M**.
    *Lands when SCITT Architecture draft reaches WG Last Call OR a
    concrete adopter requires SCITT-compatible checkpoint receipts,
    whichever fires first.* Implements `trellis-interop-scitt` against
    the ADR 0008 §"Registry" field-mapping table for `derivation_version
    = 1` (semantic-alignment mode). Re-signs the SCITT signed statement
    with a distinct SCITT-issuer key managed by the operator's SCITT
    service (not the checkpoint COSE signer). Unlocks the `scitt-receipt`
    kind in the verifier's registry; adds round-trip byte-exact
    vectors per kind. Follow-up: `derivation_version = 2` when SCITT
    adopts a byte-conformance profile.

20. **`vc-jose-cose-event` adapter — execute per ADR 0008** — **M**.
    *Lands when an SSI-native adopter (deployment that standardizes on
    W3C VC 2.0 event envelopes) shows up.* Implements
    `trellis-interop-vc` per ADR 0008 §"Registry" illustrative VC shape.
    Requires resolving the three open questions in ADR 0008: VC
    `@context` hosting + content hash, issuer-resolution policy, and
    the Posture-Declaration binding for ISC-08 payload-disclosure
    honesty per event kind. Unlocks the `vc-jose-cose-event` kind.

21. **`c2pa-manifest` adapter — execute with ADR 0007** — **M**.
    *Co-lands with ADR 0007 reference-template work.* Implements
    `trellis-interop-c2pa` per ADR 0008 §"Registry"; layers C2PA
    manifest emission on the reference HTML-to-PDF pipeline so the
    presentation artifact ships with a
    `trellis.certificate-of-completion.v1` assertion pinning
    `certificate_id`, `canonical_event_hash`,
    `presentation_artifact.content_hash`, signer `kid`, and canonical
    COSE_Sign1 digest. Requires C2PA assertion-label registration (may
    need C2PA coalition membership step per ADR 0008 open question 3).
    Unlocks the `c2pa-manifest` kind.

22. **`did-key-view` adapter — execute with item #1 (ADR 0006)** — **XS**.
    *Co-lands with ADR 0006 `KeyEntry` migration (item #1 above).*
    Implements `trellis-interop-did` per ADR 0008 §"Registry" — a
    one-way labeling view mapping each signing-class `kid` to its
    `did:key` rendering under the Ed25519 multicodec. No signing, no
    network, no verification-behavior change (the `did:key` IS the
    public key). Unlocks the `did-key-view` kind. Non-signing key
    classes are explicitly out of scope; a future `did-tenant-root-view`
    or similar requires a separate landing ADR.

23. **Core §17 `idempotency_key` — Rust + fixtures + stores + verify (G-4
    scaffold catch-up)** — **L**.
    Normative: Core §6.1 / §17 + CDDL already pin `idempotency_key` as
    `bstr .size (1..64)` and `(ledger_scope, idempotency_key)` identity;
    the Rust append scaffold (`ParsedAuthoredEvent`, `StoredEvent`,
    `LedgerStore::append_event`) does not yet thread or enforce it.
    **WOS `custodyHook` binding:** parent
    [`wos-spec/thoughts/adr/0061-custody-hook-trellis-wire-format.md`](../wos-spec/thoughts/adr/0061-custody-hook-trellis-wire-format.md)
    §2.4 — semantic tuple `(caseId, recordId)` (TypeIDs); Trellis wire key
    `SHA-256(len_prefix("trellis-wos-idempotency-v1") || dCBOR({"caseId",
    "recordId"}))` per §17 / Core §23.5. Land Trellis-center work here;
    WOS runtime wiring tracks in parent [`wos-spec/TODO.md`](../wos-spec/TODO.md).
    + [ ] **Fixtures first:** extend append corpus authored CBOR with
      `idempotency_key`; add vectors for retry no-op and
      `IdempotencyKeyPayloadMismatch` (same key, divergent payload).
    + [ ] **`trellis-cddl` / `trellis-types`:** parse, validate length,
      include in canonical / preimage construction per spec (today only
      `ledger_scope` + `sequence` on the authored path).
    + [ ] **`trellis-core` + stores:** extend `LedgerStore` / `StoredEvent`
      (or equivalent seam) so stores enforce §17.3; unique
      `(ledger_scope, idempotency_key)` in `trellis-store-postgres`;
      in-memory map in `trellis-store-memory`.
    + [ ] **`trellis-verify`:** reject chains with duplicate idempotency
      identity and divergent canonical material; align with §17.5.
    + [ ] **`trellis-conformance` + `trellis-cli`:** drive updated vectors.
    + [ ] **`trellis-py`:** G-5 parity with Rust on key + dedup semantics.
    + [ ] **Hygiene:** `trellis-verify` declares `trellis-cddl` as a
      dev-dependency but does not use it — add a focused CDDL cross-check
      test or remove the dep until wired.
    + [ ] **Close the loop in docs:** parent
      [`.claude-plugin/skills/trellis-core/SKILL.md`](../.claude-plugin/skills/trellis-core/SKILL.md)
      “Findings since last sync” + matrix rows when behavior is real.

24. **ADR 0068 execution — tenant in envelope and verifier** — **M**.
    *Lands after parent accepts ADR 0068 (gated on parent **PLN-0004**,
    **PLN-0011**, **PLN-0013**, **PLN-0015** — tenant grammar, ID scope,
    actor identity, identity-vs-pin split).* Envelope already
    reserves capacity under ADR 0003; activation is the runtime + verifier
    + vector work. Parent backlog: **PLN-0002**, **PLN-0009**, **PLN-0023**
    (tenant portion), **PLN-0030**.
    + [ ] Required `tenant` field in envelope header and bundle-level
      export metadata; CDDL + Rust + dCBOR ordering pinned per ADR 0004.
    + [ ] Verifier refusal when expected tenant scope ≠ chain/bundle
      tenant; failure taxonomy distinct from hash/signature integrity.
    + [ ] Vectors: `tamper/0NN-tenant-mismatch`, `tamper/0NN-tenant-missing`,
      plus the cross-tenant export-bundle rejection case.

25. **ADR 0071 execution — `CaseOpenPin` and migration transitions**
    — **M–L**.
    *Lands after parent accepts ADR 0071 (gated on parent **PLN-0019**
    authoritative wire home + **PLN-0095** wire encoding decision).*
    Coordinates with WOS `MigrationPinChanged` shape (parent **PLN-0021**)
    and ops guardrails (parent **PLN-0027**). Parent backlog: **PLN-0023**
    (envelope/verifier obligations — pin portion), **PLN-0026**
    (cross-version replay determinism vectors).
    + [ ] `caseOpenPin` slot in envelope header (Core + CDDL); populated
      on case-open events.
    + [ ] Verifier: pin immutability unless `MigrationPinChanged` anchors
      a transition; envelope-version-lineage compatibility across prior
      versions.
    + [ ] Vectors: pin-set, pin-mutation-rejected, valid-pin-transition
      under `MigrationPinChanged`.

26. **ADR 0070 execution — `CommitAttemptFailure` ProvenanceKind** — **M**.
    *Lands after parent accepts ADR 0070 (gated on parent **PLN-0035**
    failure-contract closure).* Trellis local append is the stack commit
    point per ADR 0070 D-1; this item adds the Facts-tier evidence shape
    for retryable / budget-exhausted / terminal commit failures plus
    optional bundle-level export summary. Parent backlog: **PLN-0044**,
    **PLN-0045**, **PLN-0089** (cross-repo failure-scenario bundle).
    + [ ] New `recordKind` literal (per ADR 0070 final naming) under
      Core §6.7 / §19; Facts tier; reservation under ADR 0080's
      open-discriminator pattern.
    + [ ] Verifier reporting taxonomy distinguishing the three typed
      outcomes; advisory-not-integrity-failure.
    + [ ] Optional `failures.json` at bundle root summarizing
      commit-attempt failures in scope.
    + [ ] Vectors: `append/0NN-commit-failure-retried`,
      `append/0NN-commit-failure-stalled`,
      `tamper/0NN-failures-json-mismatch`.

27. **ADR 0069 execution — chain timestamp-order verification** — **S–M**.
    *Lands after parent accepts ADR 0069 (gated on parent **PLN-0073**
    + **PLN-0114** + **PLN-0115** + **PLN-0117** — UTC wire, precision
    profile, leap-second policy, FEL timezone rollout).* Parent backlog:
    **PLN-0077**, **PLN-0083**, **PLN-0131** (distinct failure taxonomy),
    **PLN-0082** (cross-repo timestamp fixture bundle), **PLN-0084**
    (leap-second vectors).
    + [ ] Verifier check: chain timestamps non-decreasing across linked
      events; D-3 precision profile honored.
    + [ ] Failure taxonomy: temporal-order violations classified
      separately from hash/signature integrity failures, so reports
      distinguish "the chain says 'after' but the clocks say 'before'"
      from "the bytes were tampered."
    + [ ] Vectors: `tamper/0NN-timestamp-backwards` including the edge
      case where the hash chain is valid but temporal order fails.

28. **ADR 0081 execution — content-addressed artifact identity custody integration**
    — **S**.
    *Lands after parent ratifies ADR 0081 (parent **PLN-0358**) and WOS
    lands the three-segment `*Ref` syntax (parent **PLN-0359**).* WOS
    emits a definition-hash event on `caseCreated` and `determination`;
    Trellis anchors it via the existing `custodyHook` seam. No new
    Trellis primitive — the existing evidence-anchoring pattern handles
    it. Parent backlog: **PLN-0360**.
    + [ ] Register a definition-hash event-type tag under Core §6.7.
    + [ ] `custodyHook` ingest path for the WOS-emitted record;
      dCBOR-canonicalized; round-trip byte-exact.
    + [ ] Vector exercising definition-hash anchor + verifier
      cross-check; cross-stack three-way agreement (WOS spec + Trellis
      verifier + reference adapter).

29. ~~**Wave 15 BLOCKER — Companion §A.5.2 reason-code corpus reconciliation**~~ — **CLOSED Wave 18, 2026-04-27.**
    Path (a) executed: §A.5.2 renumbered to mirror §A.5.1. Code 4 =
    `governance-policy-change` (matches the four committed disclosure-
    profile fixture artifacts and A.5.1 code 4); codes 2 / 3 hold the
    disclosure-only meanings (`audience-scope-change`,
    `disclosure-policy-realignment`) in A.5.1's custody-specific slots so
    meaning-equivalent transitions across families share numeric value;
    `255 = Other` cross-family invariant intact. Trailing paragraph adds
    a "locks at first runtime use" pin note per Core §6 (Event Format)
    §6.9 governance discipline. Fixtures byte-stable. Spec renumber
    landed via sibling-train commit `9b3d3e4`; sibling-scout #34's R19
    corpus-vs-table parity lint
    (`scripts/check-specs.py::check_reason_code_corpus_parity`) +
    `TestReasonCodeCorpusParity::test_real_corpus_parity_via_table_authority`
    transitioned RED→GREEN as the renumber landed. See
    [`COMPLETED.md`](COMPLETED.md) Wave 18 §A.5.2 entry.

30. ~~**Wave 15 follow-up — R15 temporal-in-force enforcement**~~ — **CLOSED Wave 18, 2026-04-27.**
    Picked option (a): extended R15 to assert the predecessor's half-open
    in-force window `[effective_from, scope.time_bound)` covers the
    successor's `effective_from`. Cycle-detection DFS converted to
    iterative explicit-stack (closes Wave 15 review F6 nit). Six new
    tests including a real-declaration `shutil.copytree` case (closes
    Wave 15 review F3 follow-up). OC-70e + TR-OP-048 prose restored to
    state both clauses; §A.6 rule 15 pinned as the single-source-of-truth
    contract. Commits `2d559e5` (RED) / `3ea9849` (GREEN + iterative
    DFS) / `3575a10` (prose) plus sibling commit `abaef36` for the
    matrix line. See [`COMPLETED.md`](COMPLETED.md) Wave 18 entry.

31. ~~**HPKE crate hardening (Wave 16 review follow-ups)**~~ — **CLOSED Wave 18, 2026-04-27.**
    All four sub-bullets landed as one change train (`0ac4261` /
    `1c87dc3` / `0576ccd` / `4d18b40`):
    (1) `chacha20poly1305 = 0.10.1`, `hkdf = 0.12.4`, `x25519-dalek =
    2.0.1`, `sha2 = 0.10.9`, `rand_core = 0.9.5` `=`-pinned alongside
    the existing `hpke = 0.13.0` pin; (2) `# DO NOT BUMP without
    re-verifying:` comment block adjacent to the pins names the
    `#[doc(hidden)]`-but-`pub` `hpke::kdf::*` symbols
    `wrap_dek_with_pinned_ephemeral` leans on; spike doc promoted to
    [`thoughts/adr/0009-hpke-crate-selection.md`](thoughts/adr/0009-hpke-crate-selection.md);
    (3) `test-vectors` Cargo feature (default off) gates
    `wrap_dek_with_pinned_ephemeral`, the related fixture-only KDF /
    X25519 / AEAD imports, and `tests/append_004_byte_match.rs` so the
    production crate-graph cannot link the carve-out path; (4)
    `scripts/check-verifier-isolation.sh` + `make check-verifier-isolation`
    asserts `cargo tree -p trellis-verify` is HPKE-clean. Wired into
    `make test`; CI fails loud if `trellis-verify`'s graph regains
    any of `hpke`, `x25519-dalek`, `chacha20poly1305`, or `hkdf`.
    See [`COMPLETED.md`](COMPLETED.md) Wave 18 entry.

32. ~~**`trellis-store-postgres` review follow-ups (Wave 16)**~~ — **CLOSED Wave 18, 2026-04-27.**
    Three substantive follow-ups landed via `db4ad29` / `c33c91c` / `6684b23`.
    `MemoryTransaction::commit` now returns `Result<(), Infallible>` so
    cross-store generic test bodies share `tx.commit()?` against both
    adapters; pinned by `commit_supports_question_mark_chaining`.
    `require_loopback_dsn` gained four edge-case tests (comma-separated
    host list rejected conservatively; empty `host=` accepts libpq
    local-socket fallback; relative-path "socket" hosts rejected;
    IPv6 `[::1]` accepts in both kv and URI forms) — and the IPv6-URI
    test surfaced a real bug in `extract_dsn_host` where bracketed IPv6
    literals were sliced internally by the `rsplit_once(':')` port-split
    (host `[::1]` produced host=`[:`, port=`1]`); fixed in the same
    commit. Migration runner gained a refuse-on-future-version guard
    inside the advisory-lock-bracketed apply; an integration test forges
    a v999 row and asserts `PostgresStore::connect` refuses with
    `MigrationFailed`. See [`COMPLETED.md`](COMPLETED.md) Wave 18 entry.

33. ~~**ADR 0006 vector corpus completion**~~ — **CLOSED Wave 17, 2026-04-27.**
    Subsumed by item #1's same-wave landing. `append/031..035` cover the
    five reservation classes; `tamper/023..025` cover the three negative
    cases. TR-CORE-039 / TR-CORE-047 / TR-CORE-048 covered;
    TR-CORE-049 demoted to `Verification = prose` per ADR 0006
    *Fixture plan* (unknown-`kind` corner deferred to follow-on row).

34. ~~**Reason-code parity lint**~~ — **CLOSED Wave 18, 2026-04-27.**
    `scripts/check-specs.py` rule R19 (`check_reason_code_corpus_parity`)
    walks every `derivation.md` under `fixtures/vectors/` and every
    `gen_*.py` generator, parses Companion §A.5.1 / §A.5.2 / ADR 0005
    reason-code tables as source of truth, and rejects any
    `(family, code, annotated-name)` triple that disagrees with the
    table. Mirrors Wave 15's R13 `tamper_kind` parity discipline.
    Anchored at TR-CORE-069 (Core §6.9 ReasonCode Registry). 12 unit
    tests cover positive, drift, unregistered-code, family-ambiguous,
    cross-family integer-collision, body-prose form, generator-comment
    form, code-255-Other-floor, and live-corpus parity. Co-lands with
    sibling item #29's §A.5.2 table reconciliation.

35. **Stack-level security disclosure policy** — **S**, stack-coordination.
    *Coordinates parent **PLN-0308**.* Trellis is in the security
    perimeter (envelope, verifier, export attack surface); without a
    published intake channel and scope, security reports route through
    private conversation rather than a durable governance path.
    Trellis-side action once stack governance picks the policy home:
    contribute Trellis-specific scope notes (which crates and surfaces
    are in-scope, which are out-of-scope archive material).

36. **User-content Attestation primitive — author + execute Trellis ADR 0010**
    — **M–L**, **signature-stack**.
    *Coordinates parent **PLN-0379** (parent renumbered ADR reference
    to 0010 after the Wave 18 #31 collision was surfaced — HPKE
    crate-selection spike took 0009 via commit `0576ccd`).* Adds the
    user-content Attestation envelope per ADR 0001-0004 maximalist
    discipline: CDDL §28 entry; new §9.8 domain-separation tag; binding
    proof to host event (chain position); reference to IdentityAttestation;
    `signing_intent` as URI (Trellis owns bytes, WOS owns meaning per
    parent **PLN-0380**). Distinct from existing Companion App A.5
    Attestation (custody / disclosure / erasure). Mirror ADR 0007
    precedent: ~300 lines + 11 vectors + verifier-obligation update.
    Composes with item #4 (cert-of-completion) for full DocuSign-100%
    parity per VISION §X.
    + [ ] Author `thoughts/adr/0010-user-content-attestation-primitive.md`.
    + [ ] CDDL §28 + §9.8 domain-separation tag.
    + [ ] Verifier-obligation update (chain-position binding,
      IdentityAttestation reference resolution).
    + [ ] 11 fixture vectors per ADR 0007 precedent.
    + [ ] G-5 stranger gate extension passes.

37. ~~**AEAD nonce determinism — Core §9.4 + §17 amendment**~~ — **CLOSED Wave 19, 2026-04-27.**
    Core §9.4 prose pinned deterministic AEAD nonce derivation rule:
    `nonce = HKDF-SHA256(salt = dCBOR(idempotency_key), ikm =
    SHA-256(plaintext_payload), info = "trellis-payload-nonce-v1",
    length = 12)`. Core §17.3 no-op retry clause updated to reference
    "post-dCBOR canonicalization and post-§9.4 deterministic AEAD
    nonce". Vector `append/041-aead-retry-determinism` (real
    ChaCha20-Poly1305 + HPKE suite-1 wrap) proves retry determinism
    with seven byte-identity assertions (nonce, ciphertext,
    content_hash, authored bytes, author_event_hash, event_payload
    bytes, canonical_event_hash). Rust `trellis-types`
    `derive_payload_nonce` helper + 2 unit tests; `trellis-conformance`
    `vector_dirs` filter tightened to skip dirs without `manifest.toml`
    (handles incomplete tamper/017-018 dirs safely). TR-CORE-144
    matrix row added. Full suite green: `cargo test --workspace` 0
    failures, `python3 scripts/check-specs.py` clean, `trellis-py`
    G-5 34 passed.

38. **`custody-hook-encoding.md` v1.0 + cross-stack ingestion fixture**
    — **S**.
    *Coordinates parent **PLN-0385**.* WOS-side companion authoring
    promotes `wos-spec/specs/kernel/custody-hook-encoding.md` to v1.0
    status (currently informally referenced); Trellis-side action is
    the cross-stack ingestion fixture proving one authored WOS record
    → dCBOR canonicalization → Trellis envelope ingest. Four-field
    append wire surface: `tag`, `payload`, `prior_event_hash`,
    **`producer_signature`** (signature-stack relevance — this is the
    field that carries WOS-side signatures into the Trellis envelope).
    Without companion at v1.0, "Trellis owns bytes / WOS owns meaning"
    decomposes silently. Cited from Kernel §10.5 + Trellis Core §22
    (RL composition).
    + [ ] WOS-side: companion at v1.0 (parent-tracked).
    + [ ] Trellis-side: cross-stack fixture in shared bundle (one
      authored record → dCBOR → envelope, byte-exact).
    + [ ] Verifier round-trip: Trellis envelope → dCBOR decode →
      authored record → byte-equal.

39. **External recipient lifecycle — Trellis-side ingestion** — **M**.
    *Lands after parent ratifies stack ADR per **PLN-0382**.* Privacy
    Profile registers external systems as per-class recipients;
    ledgered `wos.governance.access-granted` / `wos.governance.access-
    revoked` events flow through `custodyHook`; recipient-rotation
    rule is per-event scope (past events keep existing key_bag
    immutably; future events scoped to current recipients). Trellis-
    side: ingest the new event types; clarify Companion §6.4 + §9.4 +
    §25.6 + §8.6 `LedgerServiceWrapEntry` re-wrap semantics; matrix
    explicit. Closes "External recipient lifecycle" center commitment
    in VISION §V. Cross-stack fixture proves rotation across two
    events (PLN-0382 done-criterion). Composes with `wos.governance.*`
    namespace ratification at parent **PLN-0384**.

40. **Tenant-scope Trellis export shape** — **M**, Trigger.
    *Coordinates parent **PLN-0392**.* Core §18 ZIP layout is per-
    `ledger_scope`; tenant-scope spans many. Owner lean: option (a) —
    new `070-tenant-package-manifest.cbor` cataloging constituent
    per-scope ZIPs with cross-binding digests. Alternative (b): new
    top-level package format nesting per-scope exports. Activate
    trigger: first tenant-scope export use case surfaces. Abandon
    trigger: none — center-adjacent profile-specific extension; will
    eventually land. Depends on item #39 (export must cover recipient-
    rotation events). Signature-stack relevance: tenant-scope export
    bundles include signed events spanning multiple ledger scopes —
    procurement + audit may demand a single bundle.
    + [ ] Choice ratified (lean: option (a)).
    + [ ] CDDL written.
    + [ ] Fixture vector for one tenant spanning two `ledger_scope`s.
    + [ ] Verifier accepts; secret-exclusion list (per ADR-0013
      absorption) enforced.

---

## Ratification close-out

Closed. G-5 evidence is recorded in
[`ratification/ratification-checklist.md`](ratification/ratification-checklist.md),
Core + Companion are at `1.0.0`, and the release tag is cut at close-out.

---

## Tagged baseline

`v1.0.0` describes a coherent snapshot of what's built: a second
implementation, written from [`specs/trellis-core.md`](specs/trellis-core.md) +
[`specs/trellis-operational-companion.md`](specs/trellis-operational-companion.md) +
[`specs/trellis-agreement.md`](specs/trellis-agreement.md) alone,
byte-matches every vector in `fixtures/vectors/`, and all ratification gates
in [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md)
are closed. Future work below may revisit any surface in this snapshot
when doing so prevents architectural debt. Nothing here is released.

---

## State lives in

This TODO points at work. State lives elsewhere — fetch it when you need it.

| What | Where | How to read it |
|---|---|---|
| Gate status, evidence SHAs | [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md) | open the file |
| Principles + format ADRs | [`thoughts/adr/0001-0004-phase-1-mvp-principles-and-format-adrs.md`](thoughts/adr/0001-0004-phase-1-mvp-principles-and-format-adrs.md) | open the file |
| Trellis-local ADRs | [`thoughts/adr/`](thoughts/adr/) | `ls thoughts/adr/` — 0001-0004 (foundational principles + format ADRs; filename retains historical `phase-1-mvp` slug), 0005 (crypto-erasure evidence), 0006 (key-class taxonomy), 0007 (certificate-of-completion composition) |
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
