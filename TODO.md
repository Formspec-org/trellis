# Trellis — TODO

Forward-looking work. Priority = `Importance × Debt`; size tags are scheduling
hints, never priority inputs. Concurrent where prerequisites allow, under the
ratified `v1.0.0` Core + Companion surface.

Size: **XS** (≤1h) · **S** (≤1 session) · **M** (≤3 sessions) · **L** (multi-session).

---

## Gate — validated principles + ADRs

[`thoughts/adr/0001-0004-phase-1-mvp-principles-and-format-adrs.md`](thoughts/adr/0001-0004-phase-1-mvp-principles-and-format-adrs.md)
holds 7 principles and 4 decided ADRs: **DAG envelope with length-1 runtime**
(0001), **list-form anchors with single-anchor default** (0002), **§22/§24
reservations held but `MUST NOT populate`** (0003), **Rust as byte authority,
Python as cross-check** (0004). Filename keeps the deprecated `phase-1-mvp` slug.

Status: **accepted and ratified into `v1.0.0`**. Closed work lives in
[`COMPLETED.md`](COMPLETED.md) and [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md).

**`v1.0.0` is a coherent-snapshot tag, not a freeze.** Nothing is released; no
production records exist. Architectural change that prevents future debt → make
it and retag. The revision window stays open until real adopters close it.

---

## Open

Ordered by `Importance × Debt`. Each item names its prerequisite inline.

**Cross-repo pointer — parent PLANNING.md.** Stack-wide rows live as `PLN-XXXX`
in [`/PLANNING.md`](../PLANNING.md). Items 10-15 + 22-30 cite parent rows;
items 1-9 + 16-21 + 31 are Trellis-internal envelope/verifier discipline
with no parent counterpart. **#1** (verify decomposition / layout) is **closed**;
**#2–#4** surfaced from the 2026-05-05 codebase audit (dedup, allocation hygiene,
constant hygiene); **#5** is the now-closed ADR 0008 `did-key-view` unlock. The
MVP-foundation cluster (PLN-0331..0349) consumes
`trellis-cose` / `trellis-verify` downstream — keep the public APIs stable for
composition. Cross-submodule Cargo path-dep posture is parent **PLN-0368**;
Trellis complies with the chosen pattern when it lands.

**`trellis-store-postgres` ↔ downstream `EventStore`:** the Postgres canonical
store (Wave 16/18 hardening + `append_event_in_tx`) is the Trellis-side half of
the composed reference-server write path. WOS-side work is the
`wos-server-eventstore-postgres` adapter (parent **PLN-0332**, gated by
PLN-0368) at [`workspec-server/crates/wos-server/TODO.md`](../workspec-server/crates/wos-server/TODO.md)
**WS-095** (embedded / `trellis-store-memory` for single-process). Keep
`append_event_in_tx` and migration discipline stable; no new TODO row unless a
gap blocks the adapter.

**Signature-stack cluster** (everything about signatures): items **#10** + **#11**
(WOS-T4 + ADR 0073 shared-fixture residue), **#12** (identity attestation,
supersedes PLN-0310 → PLN-0381), **#28** (external recipient lifecycle,
parent PLN-0382), **#29** (tenant-scope export bundles spanning multiple
ledger scopes, parent PLN-0392).

ADR 0010 user-content Attestation primitive (closed Wave 23,
`b1b23ce..74cd52d`), `c2pa-manifest@v1` adapter (closed Wave 25,
`3eda94d..<commit-5-sha>`), and `custody-hook-encoding.md` v1.0 + Wave 27
PLN-0385 wire-shape drift correction (closed Wave 27 — see
[`COMPLETED.md`](COMPLETED.md); the four-field surface is `caseId` /
`recordId` / `eventType` / `record` per ADR-0061 §2.3, NOT the phantom
`tag` / `payload` / `prior_event_hash` / `producer_signature` cited in
the original PLN-0385 prose). Trellis
owns the integrity artifact bytes; WOS owns the semantics. Both compose
under PLN-0379 + PLN-0380. Parent stack closure cluster spans
PLN-0379..0398 plus PLN-0355 (ESIGN gate, Trigger) and PLN-0370 (DocuSign
reframe).

**Cross-repo pointer — WOS Runtime §15 (Formspec coprocessor):** no Trellis-
center tasks for the core handoff. Processor and HTTP parity work lives in
parent [`work-spec/TODO.md`](../work-spec/TODO.md) **#66** and
[`workspec-server/crates/wos-server/TODO.md`](../workspec-server/crates/wos-server/TODO.md)
**WS-011**, **WS-074–WS-075** (plus **WS-072** for ADR 0066 server surfaces
once ratified). Items **#14** (ADR 0066) and **#19** (case ledger) may later
consume amended responses.

**Cross-repo pointer — ADR 0082 (Stack Public REST API Contract, accepted
2026-05-05):** no Trellis-center tasks. The WOS public REST API authored
this cycle composes Trellis at three seams without changing Trellis bytes:
(1) `bundle.schema.json` exposes `GET /api/v1/bundles/{urn}/download`
streaming Trellis Core §18 export-package bytes verbatim
(`Content-Type: application/cbor`); `Bundle.certificateOfCompletionDigest`
references Trellis ADR 0007's `presentation_artifact.content_hash`;
(2) `audit.schema.json` `AuditAttestationView` composes with item **#11**
PLN-0381 identity-attestation bundle shape — projection-only, the canonical
identity-attestation home stays Trellis-side; (3) parent **PLN-0408**
adds a `bundle-completed` literal to `NotificationType` so consumers
discover Trellis bundle completion via the notification feed instead of
polling. New parent **PLN-0407** + **PLN-0408** rows are WOS-API-internal;
new parent **PLN-0401** (utoipa) / **PLN-0402** (legacy-route deletion) /
**PLN-0405** (portal regen) / **PLN-0406** (Rust `ProvenanceKind` enum
extension) carry the implementation residue. Item **#23** (ADR 0070
`CommitAttemptFailure`) gains a typed downstream consumer in
`EventSubmissionResponse.correlationGroupResult` once 0070 ratifies.
Item **#28** (tenant-scope export, parent **PLN-0392**) gains a clearer
downstream consumer in the WOS `bundle.schema.json` shape when activated.

1. **`trellis-verify` decomposition + crate hygiene** — **L**. **Closed**
    (2026-05-05). Multi-module layout under `crates/trellis-verify/src/` (`types`,
    `parse`, `merkle`, `erasure`, `certificate`, `user_attestation`,
    `interop_sidecar`, `export`, `util`, `kinds`, `lib` orchestration); explicit
    module `use` graph (no `pub(crate) use …::*` flattening); flattened `tests`
    module; `cargo clippy -p trellis-verify --no-deps -- -D warnings` clean.
    Downstream-facing API unchanged (`verify_single_event`, `verify_tampered_ledger`,
    `verify_export_zip`, report / failure kinds). *Roll up narrative in*
    [`COMPLETED.md`](COMPLETED.md) *on the next doc batch if not already present.*

2. **Verify-engine dedup — CBOR helpers, utility functions, error-type enums**
    — **M**. **Closed** (Wave 32, 2026-05-06).
    *Three dedup concerns; can land independently.*
    + [x] CBOR map-lookup helpers (`map_lookup_bytes`, `map_lookup_u64`,
      `map_lookup_fixed_bytes`, etc.) duplicated across `trellis-cddl:242-327`,
      `trellis-verify:6303-6502` (15 variants), `trellis-interop-c2pa:362-399`,
      and `trellis-conformance:558-603`. Structurally identical; only error type
      differs. Extract to shared `trellis-types` module with generic error param
      or closure-based approach. ~400 lines deduplicated.
    + [x] Utility functions duplicated: `checkpoint_digest` (verify:6063 vs
      conformance:593 — byte-identical), `sha256_bytes` (verify returns `Vec<u8>`,
      conformance returns `[u8; 32]` — same logic), `decode_value` (verify:6298
      vs conformance:537).
    + [x] `VerifyError.kind` is `Option<&'static str>`,
      `VerificationFailure.kind` is `String` — 40+ distinct failure-kind strings
      that callers must string-compare. Convert both to `VerifyErrorKind` /
      `VerificationFailureKind` enums (cf. `PostgresStoreErrorKind`,
      `CddlErrorKind`).
    + [x] `verify_event_set_with_classes` takes 8 parameters behind
      `#[allow(clippy::too_many_arguments)]` — introduce an options struct.
    + [x] Post-dedup review: `decode_value` routes CBOR helper errors through
      `From<CborHelperError>`; `kinds.rs` unit tests lock wire strings for every
      `VerificationFailureKind` plus `VerifyErrorKind` bridge parity.
    Closed evidence: shared `trellis-types` CBOR helpers and digest helpers are
    consumed by CDDL / verify / conformance / interop crates; structured
    `VerifyErrorKind` / `VerificationFailureKind` values are exported with
    wire-string parity tests; `VerifyEventSetOptions` replaced the broad
    argument list.

3. **Verify-engine allocation hygiene — `.clone()` reduction + re-decode
    elimination** — **M**. **Closed** (Wave 32, 2026-05-06).
    *Performance-correctness refactor.*
    + [x] 86 `.clone()` calls in the verify hot path; notable patterns clone
      entire `ErasureEvidenceDetails`, `CertificateDetails`,
      `UserContentAttestationDetails` structs just to push into collectors.
      Audit each; replace with references or `Cow` where possible.
      *(Hot-path collectors now `take()` extension payloads from the decoded
      event instead of cloning; certificate finalize avoids cloning inline
      affirmation payload bytes.)*
    + [x] `finalize_certificates_of_completion` and
      `finalize_user_content_attestations` re-decode all events
      the main event loop already decoded. Pass decoded `EventDetails` to
      finalize functions instead.
      *(`build_event_details_lookup` seeds from per-index main-loop decodes;
      remaining indices decode once for legacy parity.)*
    + [x] `VerificationReport` integrity fold computed in 3 separate places
      with risk of silent divergence.
      Consolidate to single computation site.
      *(`VerificationReport::integrity_verified_from_parts` — export ZIP path,
      `from_integrity_state`, and interop fold aligned.)*
    + [x] Conformance tests: ~70 bare `.unwrap()` calls with no failure
      localization (which vector directory, which field). Add fixture-name
      context to assertion failures. (Same pain called out in the 2026-05-05
      verify-engine dedup code review; lands with this hygiene pass.)
      *(Per-fixture thread-local + contextual panics in shared helpers.)*
    Closed evidence: decoded extension payloads are moved into collectors where
    possible, certificate / user-content finalizers consume `EventDetails`
    lookup pools instead of re-decoding the main pass, `VerificationReport`
    has one shared integrity fold, and conformance panics include fixture
    labels.

4. **Cross-crate constant dedup + CLI parameterization + COSE named constants**
    — **S**. **Closed** (2026-05-05).
    *Three small hygiene items.*
    + [x] `IDEMPOTENCY_KEY_MAX_LEN = 64` independently declared in
      `trellis-types:138`, `trellis-store-postgres:64`, `trellis-store-memory:139`.
      Stores import from `trellis_types`.
      *(Store crates now re-export the canonical `trellis_types` constant and
      enforce bounds through `idempotency_key_length_in_bound`.)*
    + [x] `trellis-cli` duplicated command implementations: `append_001`/
      `append_002`, `verify_001`/`verify_002`, `export_001`/`export_002` are
      structurally identical except fixture directory name. Single parameterized
      function per operation.
    + [x] `trellis-cose` builds CBOR bytes with hardcoded `0xa3`, `0x84`, `0xd2`
      (map-3, array-4, tag-18). Name as constants or use `encode_major_len`
      from `trellis_types`.

5. **`did-key-view` adapter — execute per ADR 0008** — **Closed** (Wave 29,
    2026-05-05).
    *Co-lands with ADR 0006 (closed Wave 17 — prerequisite met).* Implements
    `trellis-interop-did` as a one-way labeling view mapping each signing-class
    `kid` to its `did:key` rendering under the Ed25519 multicodec. No signing,
    no network, no verification-behavior change (the `did:key` IS the public
    key). Unlocks the `did-key-view` kind. Non-signing key classes are out of
    scope; a `did-tenant-root-view` or similar gets a separate ADR.
    + [x] `trellis-interop-did` parses legacy `SigningKeyEntry` and ADR 0006
      `KeyEntrySigning` registry rows, skips non-signing `KeyEntry` rows,
      rejects duplicate signing `kid` values / non-Ed25519 suites, and emits
      byte-stable compact JSON sorted by `kid`.
    + [x] `trellis-verify` admits manifest-listed `did-key-view@v1` through the
      existing path-(b) digest-binding path; it does not parse DID JSON, perform
      network resolution, or resolve `source_ref`.

6. **Key-rotation grace-window semantics** — **Closed** (Wave 30,
    2026-05-06).
    *Landed proactively before production rotation.* Core §8.4 now pins
    `Rotating` as a bounded new-signature authority:
    `valid_from <= signature_time <= valid_to`, with `valid_to = null`
    meaning the overlap is still open in the embedded snapshot. Companion
    §20.8 OC-147 owns the operator registry-publishing obligation.
    `trellis-verify` parses `valid_from`, admits user-content attestations
    under `Rotating` only inside the overlap, and rejects out-of-overlap
    `Rotating` via `user_content_attestation_key_not_active`. Boundary vector:
    `tamper/043-uca-rotating-after-valid-to`; positive in-overlap coverage:
    `trellis-verify` unit tests.

7. **Cadence subtypes beyond height-based** — **M**.
    *Land with a non-height adopter, or proactively under fixture-corpus
    breadth work.* `projection/003` and `projection/004` cover height-based
    only; time-driven / event-driven / hybrid untested.

8. **O-4 ledger-replay lint rules 7–13** — **M**.
    *Land when the first external adopter publishes a declaration to verify
    against actual ledger emission history.* Seven declaration-vs-runtime
    checks: `max_agents_per_case` ceiling, `max_invocations_per_day` ceiling,
    WOS autonomy-cap superset, delegation-chain monotonicity,
    actor-discriminator on emitted events, `agent_identity` attribution match,
    emitted types ⊆ `audit.event_types`. Static rules 1–6 + 11 cover the
    declaration-internal surface; these add the runtime cross-check.

9. **WOS-T4 residue — shared cross-repo fixture bundle re-seeding** — **S**.
    *Land when parent standardizes a single shared cross-stack fixture bundle.*
    Trellis consumes those declarative inputs rather than seeding a parallel
    corpus. Coordination, not a Trellis-center gap. Parent backlog:
    **PLN-0067** (shared bundle), **PLN-0068** (response-hash mismatch
    negative), **PLN-0069** (CI/conformance gate).

10. **ADR 0073 handoff residue — shared fixture alignment** — **S**.
    *Same prerequisite as #12.* Workflow-initiated attach and public-intake
    create vectors are live; the residue is consuming from one shared bundle
    rather than parallel corpora. Parent backlog: **PLN-0067**.

11. **Identity attestation bundle shape** — **S**.
    *Land after parent ratifies the IdentityAttestation stack ADR per
    **PLN-0381**.* Synthesis-merge 2026-04-27 promoted identity attestation
    from Trigger to P0 center commitment; **PLN-0310 closed by supersession**.
    Trellis-side action: declare how a provider-neutral identity-proofing
    attestation lands as a canonical event kind and travels in the export
    bundle. Composes with the new `wos.identity.*` event taxonomy (parent
    **PLN-0384**, namespace gate) and PLN-0380 (signer-authority claim shape,
    distinct from authentication-method). Cross-stack fixtures (Formspec → WOS
    provenance → Trellis envelope) prove composition.

12. **Respondent Ledger ↔ Trellis `eventHash` MUST promotion** — **M**.
    *Land after Formspec promotes §6.2 `eventHash` / `priorEventHash` from
    SHOULD → MUST.* Trellis spec amendment + conformance/lint follow.
    Parent backlog: **PLN-0311** (Respondent Ledger offline-authoring profile +
    chain semantics).

13. **ADR 0066 execution — amendment / supersession / rescission / correction**
    — **L**.
    *Land after parent accepts ADR 0066* —
    [`../thoughts/adr/0066-stack-amendment-and-supersession.md`](../thoughts/adr/0066-stack-amendment-and-supersession.md).
    WOS checklist:
    [`../work-spec/TODO.md#adr-0066-exec-checklist`](../work-spec/TODO.md#adr-0066-exec-checklist).
    Parent backlog: **PLN-0055**, **PLN-0056**, **PLN-0050**
    (`ResponseCorrection` linkage), **PLN-0051** (supersession-start linkage).
    + [ ] `EventPayload.extensions` carries **`trellis.supersedes-chain-id.v1`**
      per Core §6.7 (spec table landed); align Companion/CDDL + fixtures with
      that identifier and payload `{ chain_id, checkpoint_hash }`; null /
      absent on genesis non-superseding chains.
    + [ ] Single-chain vectors: `append/011-correction`, `012-amendment`,
      `013-rescission`.
    + [ ] Verifier **D-3:** correction-preservation; rescission-terminality
      (any determination after `DeterminationRescinded` → integrity violation);
      chain-linkage with byte-equal predecessor checkpoint hash.
    + [ ] Core §17 / §19 prose + export-manifest hooks; coordinate with
      Formspec `ResponseCorrection` + WOS payload shapes.
    + [ ] Cross-chain: normative `supersession-graph.json` at bundle root;
      verifier BFS over `head_chain_id` / `predecessors`; cycles = integrity
      failure (ADR default; Q2 alternative is linear-only).
    + [ ] Optional predecessor chain members in export bundle (ADR D-4).

14. **ADR 0067 execution — statutory clocks** — **M**.
    *Land after parent accepts ADR 0067.* Coordinate payload hashes with WOS
    `clockStarted` / `clockResolved` (parent
    [`work-spec/TODO.md`](../work-spec/TODO.md#adr-0067-exec-checklist)).
    Parent backlog: **PLN-0159** (`open-clocks.json` export), **PLN-0160**
    (verifier diagnostics, severity per **PLN-0170**), **PLN-0161**
    (pause/resume composition), **PLN-0162** (vectors), **PLN-0164**
    (cross-stack composition fixture).
    + [ ] **Export bundle:** normative `open-clocks.json` at bundle root —
      `{ clock_id, clock_kind, computed_deadline, origin_event_hash }` for
      every `clockStarted` lacking a matching `clockResolved` at export time.
    + [ ] **Verifier — D-3 advisory:** open clock with
      `computed_deadline < bundle.sealed_at` and no `clockResolved` emits an
      advisory diagnostic, not an integrity failure.
    + [ ] **Verifier — D-4 composition:** walk the chain to compose pause
      segments into cumulative duration / segment accounting.
    + [ ] **Vectors:** `append/014-clock-started`, `015-clock-satisfied`,
      `016-clock-elapsed`, `017-clock-paused-resumed` (+ matching export/verify
      hooks for byte-identity CI).

15. **`trellis.external_anchor.v1` priority interaction** — **S**.
    *Land when external anchoring opens.* O-5 posture-transition events may
    want higher anchor priority in deployments with external-anchor chains.
    Anchor substrate is adapter-tier per the
    [anchor-substrate spike](thoughts/specs/2026-04-24-anchor-substrate-spike.md)
    — center ships an `AnchorAdapter` trait + enumerates OpenTimestamps,
    Sigstore Rekor, and Trillian as first-class candidates. This item picks the
    priority policy per deployment (which adapter's `anchored_at_or_before`
    drives posture-transition priority when multiple adapters attest), declared
    in the Posture Declaration.

16. **ADR 0005 follow-ons (erasure evidence)** — **M–L**.
    Four open questions from
    [`thoughts/adr/0005-crypto-erasure-evidence.md`](thoughts/adr/0005-crypto-erasure-evidence.md):
    + [ ] LAK rotation × erasure interaction — re-wrap cascade or coupled
      recipe; lands with the first live LAK rotation touching erasure-cascade
      subjects.
    + [ ] `hsm_receipt_kind` registry; lands with the second deployment on a
      different HSM vendor.
    + [ ] Legal-hold-coupled erasure lint (OC-78 vs §20.6 conflict detection).
    + [ ] Multi-operator quorum attestation shape — co-lands with the first
      federated deployment.

17. **Disclosure-profile scope granularity (per-case)** — **M**.
    *Land when case-ledger composition opens.* Companion A.5.2 reserves an
    `extensions` slot for per-case refinement; current semantics are
    deployment-scope only.

18. **Case ledger + agency log semantic definitions** — **M**.
    *Land when case-ledger / agency-log scoping opens.* Core §22 case ledger
    composes sealed response-ledger heads with WOS governance events; Core §24
    agency log is the operator-maintained log of case-ledger heads. Envelope
    hooks stay reserved under ADR 0003 with `MUST NOT populate` until this
    lands.

19. **`scitt-receipt` adapter — execute per ADR 0008** — **M**.
    *Land when SCITT Architecture draft reaches WG Last Call OR an adopter
    requires SCITT-compatible checkpoint receipts, whichever fires first.*
    Implements `trellis-interop-scitt` against ADR 0008 §"Registry" for
    `derivation_version = 1` (semantic alignment). Re-signs the SCITT signed
    statement with a distinct SCITT-issuer key managed by the operator's SCITT
    service (not the checkpoint COSE signer). Unlocks the `scitt-receipt` kind;
    adds round-trip byte-exact vectors. Follow-up: `derivation_version = 2`
    when SCITT adopts a byte-conformance profile.

20. **`vc-jose-cose-event` adapter — execute per ADR 0008** — **M**.
    *Land when an SSI-native adopter (W3C VC 2.0 event envelopes) shows up.*
    Implements `trellis-interop-vc` per ADR 0008 §"Registry". Requires
    resolving three ADR 0008 open questions: VC `@context` hosting + content
    hash, issuer-resolution policy, and Posture-Declaration binding for
    ISC-08 payload-disclosure honesty per kind. Unlocks the
    `vc-jose-cose-event` kind.

21. **ADR 0068 execution — tenant in envelope and verifier** — **M**.
     *Gates closed:* **PLN-0004** (D-1.1 grammar), **PLN-0005** (D-1.2
     payload.tenant authoritative), **PLN-0011** (D-4 tenant×ledger scoped),
     **PLN-0013** (D-3 global identity + per-tenant authority), **PLN-0015**
     (D-2 immutable tuple vs 0071 mutable pins). *Remaining gate:*
     **PLN-0012** (supersession carry-forward, deferred to 0066 cluster).
     Envelope reserves capacity under ADR 0003; activation is the runtime +
     verifier + vector work.
     Parent backlog: **PLN-0002**, **PLN-0009**, **PLN-0023** (tenant portion),
     **PLN-0030**.
    + [ ] Required `tenant` field in envelope header and bundle metadata; CDDL
      + Rust + dCBOR ordering pinned per ADR 0004.
    + [ ] Verifier refusal when expected tenant ≠ chain/bundle tenant; failure
      taxonomy distinct from hash/signature integrity.
    + [ ] Vectors: `tamper/0NN-tenant-mismatch`, `tamper/0NN-tenant-missing`,
      cross-tenant export-bundle rejection.

22. **ADR 0071 execution — `CaseOpenPin` and migration transitions** — **M–L**.
    *Land after parent accepts ADR 0071 (gated on parent **PLN-0019** wire
    home + **PLN-0095** wire encoding).* Coordinates with WOS
    `MigrationPinChanged` (parent **PLN-0021**) and ops guardrails (parent
    **PLN-0027**). Parent backlog: **PLN-0023** (envelope/verifier obligations,
    pin portion), **PLN-0026** (cross-version replay determinism vectors).
    + [ ] `caseOpenPin` slot in envelope header (Core + CDDL); populated on
      case-open events.
    + [ ] Verifier: pin immutability unless `MigrationPinChanged` anchors a
      transition; envelope-version-lineage compatibility.
    + [ ] Vectors: pin-set, pin-mutation-rejected, valid-pin-transition under
      `MigrationPinChanged`.

23. **ADR 0070 execution — `CommitAttemptFailure` ProvenanceKind** — **M**.
    *Land after parent accepts ADR 0070 (gated on parent **PLN-0035**
    failure-contract closure).* Trellis local append is the stack commit point
    per ADR 0070 D-1; this adds the Facts-tier evidence shape for retryable /
    budget-exhausted / terminal commit failures plus optional bundle-level
    summary. Parent backlog: **PLN-0044**, **PLN-0045**, **PLN-0089**
    (cross-repo failure-scenario bundle).
    + [ ] New `recordKind` literal under Core §6.7 / §19; Facts tier;
      reservation under ADR 0080's open-discriminator pattern.
    + [ ] Verifier reporting taxonomy distinguishing the three typed outcomes;
      advisory, not integrity failure.
    + [ ] Optional `failures.json` at bundle root.
    + [ ] Vectors: `append/0NN-commit-failure-retried`,
      `append/0NN-commit-failure-stalled`,
      `tamper/0NN-failures-json-mismatch`.

24. **ADR 0069 execution — chain timestamp-order verification + CBOR wire migration** — **M**.
    *Land after parent accepts ADR 0069 (gated on parent **PLN-0073** +
    **PLN-0114** + **PLN-0115** + **PLN-0117**).* Parent backlog: **PLN-0077**,
    **PLN-0083**, **PLN-0131** (failure taxonomy), **PLN-0082** (cross-repo
    timestamp fixture bundle), **PLN-0084** (leap-second vectors).
    + [x] **CBOR envelope timestamp wire migration: `uint` seconds →
      protobuf-pattern `[uint, uint .le 999999999]`** per ADR 0069 D-2.1.
      Changed from original `uint64` nanoseconds plan to protobuf-pattern
      `[seconds, nanos]` arrays (industry-standard, no overflow risk). CDDL
      §28 base type, all timestamp sites, Rust `TrellisTimestamp` struct,
      Python `TrellisTimestamp` class, 53 generators migrated via `ts()`
      helper, 439 CBOR fixtures regenerated, 27 manifest hashes updated.
      `legacy_timestamp_format` rejection for bare-uint timestamps.
      Remaining: negative vector `tamper/0NN-timestamp-legacy-uint-rejected`
      + §19.1 enum registration for `legacy_timestamp_format`.
    + [x] Verifier check: chain timestamps non-decreasing across linked events;
      D-3 precision profile honored.
    + [x] Failure taxonomy: temporal-order violations classified separately
      from hash/signature integrity, so reports distinguish "the chain says
      'after' but the clocks say 'before'" from "the bytes were tampered."
    + [x] Vectors: `tamper/041-timestamp-backwards`, including the case where
      the hash chain is valid but temporal order fails.

25. **ADR 0081 execution — content-addressed artifact identity** — **S**.
    *Land after parent ratifies ADR 0081 (parent **PLN-0358**) and WOS lands
    the three-segment `*Ref` syntax (parent **PLN-0359**).* WOS emits a
    definition-hash event on `caseCreated` and `determination`; Trellis anchors
    via the existing `custodyHook` seam. No new Trellis primitive — the
    evidence-anchoring pattern handles it. Parent backlog: **PLN-0360**.
    + [ ] Register a definition-hash event-type tag under Core §6.7.
    + [ ] `custodyHook` ingest path; dCBOR-canonicalized; round-trip byte-exact.
    + [ ] Vector exercising definition-hash anchor + verifier cross-check;
      cross-stack three-way agreement (WOS spec + Trellis verifier + reference
      adapter).

26. **Stack-level security disclosure policy** — **S**, stack-coordination.
    *Coordinates parent **PLN-0308**.* Trellis is in the security perimeter
    (envelope, verifier, export attack surface); without a published intake
    channel and scope, security reports route through private conversation.
    Trellis-side action once stack governance picks the policy home:
    contribute scope notes (which crates and surfaces are in / out of scope).

27. **External recipient lifecycle — Trellis-side ingestion** — **M**.
    *Land after parent ratifies the stack ADR per **PLN-0382**.* Privacy
    Profile registers external systems as per-class recipients; ledgered
    `wos.governance.access-granted` / `access-revoked` events flow through
    `custodyHook`; recipient-rotation rule is per-event scope (past events
    keep existing key_bag immutably; future events scope to current
    recipients). Trellis-side: ingest the new event types; clarify Companion
    §6.4 + §9.4 + §25.6 + §8.6 `LedgerServiceWrapEntry` re-wrap semantics;
    matrix explicit. Closes "External recipient lifecycle" center commitment
    in VISION §V. Cross-stack fixture proves rotation across two events
    (PLN-0382 done-criterion). Composes with `wos.governance.*` namespace
    ratification at parent **PLN-0384**.

28. **Tenant-scope Trellis export shape** — **M**, Trigger.
    *Coordinates parent **PLN-0392**. Activate trigger:* first tenant-scope
    export use case. Core §18 ZIP layout is per-`ledger_scope`; tenant-scope
    spans many. Owner lean: option (a) — new `070-tenant-package-manifest.cbor`
    cataloging constituent per-scope ZIPs with cross-binding digests.
    Alternative (b): top-level package format nesting per-scope exports.
    Depends on item #28 (export must cover recipient-rotation events).
    Signature-stack: tenant-scope export bundles span signed events across
    ledger scopes — procurement + audit may demand a single bundle.
    + [ ] Choice ratified (lean: option (a)).
    + [ ] CDDL written.
    + [ ] Fixture vector for one tenant spanning two `ledger_scope`s.
    + [ ] Verifier accepts; secret-exclusion list (per ADR-0013 absorption)
      enforced.

29. **PLN-0379..0398 cluster drift audit** — **S**.
    *Land before authoring against any sibling row in the parent stack-closure
    cluster (proactive; no external trigger).* Wave 27 closure of PLN-0385
    surfaced a phantom four-field surface (`tag` / `payload` /
    `prior_event_hash` / `producer_signature`) cited in parent prose but
    absent from every spec / crate / schema / fixture; correct surface per
    ADR-0061 §2.3 is `caseId` / `recordId` / `eventType` / `record`.
    `producer_signature` exists nowhere — envelope-level signing is COSE_Sign1
    around the envelope, at the Trellis layer above the WOS-authored record
    surface. Sibling rows (PLN-0379, 0380, 0381, 0382, 0383, 0384, 0386, 0387)
    were authored from the same wos-server-centric mental model and plausibly
    carry parallel drift. Audit each sibling: cross-reference cited wire
    surface, schema names, event-type strings, and field labels against
    Trellis CDDL §28, work-spec schemas, custody-hook companion, and ADR-0061.
    Output: per-row corrections applied in-place to PLANNING.md + the
    referenced source docs (mirroring the Wave 27 train pattern). Prevents
    repeating Wave 27 archaeology cost (cross-stack-scout dispatch → Fork-B
    drift discovery → three-commit cross-repo train) once per sibling row.
    See [`COMPLETED.md`](COMPLETED.md) Wave 27 for the discovery pattern.

30. **`interop_sidecar_content_mismatch` failure-code split** — **Closed**
    (Wave 31, 2026-05-06). `verify_interop_sidecars` now reports
    `interop_sidecar_missing` when the manifest promises a dispatched sidecar
    path the ZIP does not carry, while `interop_sidecar_content_mismatch`
    remains digest-only for present-but-mutated bytes. Core §18.3a / §19.1,
    ADR 0008, the matrix, and the fixture corpus now carry TR-CORE-168 and
    `tamper/044-interop-sidecar-missing`. See [`COMPLETED.md`](COMPLETED.md)
    Wave 31.

---

## Ratification close-out

Closed. G-5 evidence in
[`ratification/ratification-checklist.md`](ratification/ratification-checklist.md);
Core + Companion at `1.0.0`; release tag cut at close-out.

---

## Tagged baseline

`v1.0.0` is a coherent snapshot: a second implementation written from
[`specs/trellis-core.md`](specs/trellis-core.md) +
[`specs/trellis-operational-companion.md`](specs/trellis-operational-companion.md) +
[`specs/trellis-agreement.md`](specs/trellis-agreement.md) byte-matches every
vector in `fixtures/vectors/`, and every ratification gate in
[`ratification/ratification-checklist.md`](ratification/ratification-checklist.md)
is closed. Future work may revisit any surface here when doing so prevents
architectural debt. Nothing is released.

---

## State lives in

| What | Where |
|---|---|
| Phase 1 ratification record (closed) | [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md) |
| Trellis ADRs | [`thoughts/adr/`](thoughts/adr/) |
| Closed work | [`COMPLETED.md`](COMPLETED.md) |
| Stack vision (Trellis section §XI) | [`../VISION.md`](../VISION.md) |
| In-flight designs | [`thoughts/specs/`](thoughts/specs/) |
| Fixture corpus | `fixtures/vectors/` |
| Rust reference impl | `crates/` (workspace root: `../Cargo.toml`) |
| Python cross-check (G-5) | `trellis-py/` |
| Green check | `python3 scripts/check-specs.py && cargo nextest run --workspace && python -m trellis_py.conformance` |

Spec-sized work: move substance to [`thoughts/specs/`](thoughts/specs/), keep a pointer here. Landed work: move to [`COMPLETED.md`](COMPLETED.md).
