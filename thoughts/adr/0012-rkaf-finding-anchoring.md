# ADR 0012 — `rkaf:Finding` Anchoring Binding

**Date:** 2026-05-14
**Status:** Proposal — open. Wire shape and verifier obligations defined here; Rust types, fixtures, and adapter wiring deferred until a concrete adopter (Studio compiler, BVR runtime, or `wos-server` validation event emitter) triggers implementation per the "no stubs" rule (CLAUDE.md). PKAF ADR-0093 lands the Finding primitive on the Rulespec side; this ADR is Trellis's side of the §4.6 binding.
**Supersedes:** —
**Superseded by:** —
**Related:**
- Parent-repo PKAF ADR-0093 (`../../thoughts/adr/0093-rkaf-finding-iri-addressability.md`) — promoted `rkaf:Finding` to a first-class IRI-addressable primitive in Rulespec; explicitly names this Trellis binding as open work.
- PKAF framework spec `2026-05-12-pkaf-as-public-schema-interop-framework.md` §4.6 — abstract anchoring contract (dependency-inverted: bindings know about Rulespec; Rulespec does not name a binding).
- ADR 0010 (user-content-attestation primitive) — closest pattern: a typed `EventPayload.extensions` entry that binds external content to the chain via the host event's COSE_Sign1.
- ADR 0008 (interop sidecar discipline) — parent discipline for cross-spec interop additions; this ADR follows the same "registered, additive, core-isolated" posture but ships as an `EventPayload.extensions` kind (per-event) rather than an export-manifest sidecar (per-bundle), because a Finding is event-shaped (one detection = one event), not bundle-shaped.
- Core §6.7 Extension Registry — adds one row for `trellis.rkaf-finding.v1`.
- Core §11.5 / §16.3 — external-anchor reservation; this ADR does NOT use Phase-2+ external anchoring. A `rkaf:Finding` event anchors via the standard Phase-1 mechanism (COSE_Sign1 over `EventPayload` + Merkle inclusion under a `Checkpoint`).
- CLAUDE.md — authority ladder (Rust > CDDL > prose > matrix > Python > archives) and "no stubs" discipline.

## Decision

Trellis defines a canonical event extension `trellis.rkaf-finding.v1` that anchors one `rkaf:Finding` (PKAF ADR-0093) to the case ledger by recording its structural fields plus its IRI inside a Trellis event. The host event's existing COSE_Sign1 (Core §6.6) and the chain's existing Merkle inclusion (Core §11.3) ARE the anchor — no Finding-specific signature, no Finding-specific domain-separation tag, no `anchor_ref` field. The Finding's IRI becomes verifiable-offline-from-the-export-bundle by the same machinery that anchors every other Trellis event.

The §4.6 abstract anchoring contract is satisfied as follows:

| §4.6 requirement | Trellis binding |
|---|---|
| **(1) Anchor type URI** | `urn:rkaf:anchor:trellis/1` |
| **(2) What the anchor commits to** | `dCBOR(EventPayload)` of the host Trellis event whose `EventPayload.extensions["trellis.rkaf-finding.v1"]` decodes to `RkafFindingPayload`, where `RkafFindingPayload.finding_iri` equals the `rkaf:Finding`'s `@id`. The canonical-serialization function (§4.6 req 2) is `λ Finding . dCBOR(RkafFindingPayload(Finding))`. |
| **(3) Verification function** | The Phase-1 Trellis verifier (Core §19) over the host event, plus the structural decode + invariant checks enumerated in §"Verifier obligations" below. Returns `commit` iff `integrity_verified = true` AND `finding_iri` matches the Rulespec subgraph the consumer is checking against. |
| **(4) Spec published outside Rulespec** | This ADR. PKAF spec does not name Trellis. |

The Rulespec consumer expresses the binding in its graph with:

```turtle
?finding rkaf:anchoredBy ?anchorIRI .
?anchorIRI rkaf:anchorType <urn:rkaf:anchor:trellis/1> .
# Binding-defined: anchorIRI dereferences to the Trellis event hash + checkpoint coordinates.
```

The shape of `?anchorIRI` (resolution URI scheme, how it composes the Trellis event hash + checkpoint coordinates) is **deferred to a follow-on RFC** (see Open questions §1). Phase-1 adopters MAY use any URI that resolves to a `(ledger_scope, canonical_event_hash, checkpoint_ref)` triple; the Trellis side does not need to know how `?anchorIRI` is minted because the verifier resolves the Finding by hash, not by URI.

### Rejected alternatives

**(R1) Use ADR 0008 sidecar discipline instead of an event-extension.** Rejected because a Finding is event-shaped (one detection = one fact at one moment by one detector about one subject), not export-bundle-shaped (a sidecar re-expresses the whole export in another ecosystem's vocabulary). Findings need per-event addressability so downstream primitives (PKAF `Attestation(targetFinding=…)`, Studio `waived` lifecycle) can reference them; sidecars are not addressable per-event. ADR 0008 remains the right pattern for "the export as a whole, additionally expressed as a SCITT receipt"; ADR 0012 is the right pattern for "one detection, recorded in-chain".

**(R2) Reuse `trellis.user-content-attestation.v1` (ADR 0010).** Rejected because the semantics differ:
- A user-content-attestation has a *user attestor* with an identity-attestation reference and a `signing_intent`. A Finding has a *detector* (a BVR / lint rule / validator service) which is generally NOT a person and has no identity-attestation event.
- A user-content-attestation carries a per-attestation Ed25519 signature under `trellis-user-content-attestation-v1`. A Finding does not need a per-Finding signature; the host event's COSE_Sign1 is sufficient — the detector identity binds via `EventHeader.author` of the host event, not via a separate per-Finding signature.

**(R3) Add a Phase-1 domain-separation tag `trellis-rkaf-finding-v1`.** Rejected as unnecessary. Findings are not separately signed; the host event's `trellis-event-v1` domain tag already covers the payload bytes that include the Finding extension. Adding a Finding-specific tag would imply a Finding-specific signature, which (R2) declines.

**(R4) Wait for a concrete adopter before any ADR.** Rejected because PKAF ADR-0093 explicitly names "Trellis — open" as the cross-stack gap. Filing the wire shape now lets Studio + BVR + `wos-server` schedule their adoption against a known target; deferring it would either (a) force Studio to invent a one-off binding that we then break, or (b) block PKAF Phase D on a chicken-and-egg dependency.

## Context

### What a Finding is (PKAF side)

Per PKAF ADR-0093 + `PKAF/constraints/core/finding.cue`, a `rkaf:Finding` is a typed, IRI-addressable record of one validation / audit detection. Source-of-truth shape (CUE, lightly normalized to JSON-LD):

```jsonld
{
  "@type": "rkaf:Finding",
  "@id":   "<IRI>",
  "rkaf:findingKind":      "<#FindingKind>",   /* closed taxonomy */
  "rkaf:detectedAt":       "<xsd:dateTime>",
  "rkaf:detectedBy":       "<IRI>",            /* detector */
  "rkaf:subject":          "<IRI>",            /* what the finding is about */
  "rkaf:severity":         "<#FindingSeverity>?", /* closed taxonomy, optional */
  "rkaf:rationale":        "<string>?",
  "rkaf:lastVerifiedAt":   "<xsd:dateTime>?",  /* Plan 7d freshness */
  "rkaf:verifiedBy":       "<IRI>?"            /* Plan 7d freshness */
}
```

`#FindingKind` and `#FindingSeverity` are **closed taxonomies** at PKAF L1 — partner extension requires an RFC per PKAF §13.4. Trellis MUST honor this closedness at the verifier boundary (see §"Verifier obligations" step 2).

### Why event-extension is the right shape

A Finding event answers four questions an offline verifier needs to answer:

1. *Was this Finding emitted?* — Yes iff the host event is chain-present (Core §10).
2. *Who emitted it?* — `EventHeader.author` of the host event (Trellis principal URI), cross-checked against `RkafFindingPayload.detected_by` (Rulespec detector IRI).
3. *When?* — `EventHeader.authored_at` of the host event, cross-checked against `RkafFindingPayload.detected_at`.
4. *About what, with what severity, why?* — `RkafFindingPayload.subject` + `.severity` + `.rationale`.

Anchoring is "this event was signed and merkle-included" — the standard Phase-1 mechanism. The extension does not introduce a second signature or a parallel hash chain; it adds a typed payload slot.

## Wire shape

Under `EventPayload.extensions["trellis.rkaf-finding.v1"]`:

```cddl
; --- ADR 0012: rkaf:Finding anchoring binding ---

RkafFindingPayload = {
  finding_iri:        tstr,                   ; the rkaf:Finding @id; MUST be an absolute IRI per RFC 3987.
                                              ; Idempotency key within ledger_scope: two events sharing
                                              ; the same finding_iri MUST carry byte-identical canonical
                                              ; payloads or the verifier flags rkaf_finding_iri_collision.
  finding_kind:       tstr,                   ; MUST be one of the closed #FindingKind values per
                                              ; PKAF/constraints/core/finding.cue at the producer's
                                              ; declared PKAF version (see pkaf_version below).
                                              ; Verifier checks set-membership against a Trellis-side
                                              ; registry snapshot keyed by pkaf_version.
  detected_at:        uint,                   ; Unix seconds UTC. MUST equal envelope authored_at
                                              ; exactly (no skew slack). PKAF's xsd:dateTime form is a
                                              ; lossless lift of this integer (per Core's general rule
                                              ; that timestamps are seconds-resolution on the wire).
  detected_by:        tstr,                   ; IRI of the detector (BVR, lint rule, validator,
                                              ; attester). Distinct from EventHeader.author (which is
                                              ; the Trellis principal URI of the SIGNER); these two
                                              ; are linked but not equal — see Verifier obligation step 5.
  subject:            tstr,                   ; IRI of the object the finding concerns.
  severity:           tstr / null,            ; MUST be one of the closed #FindingSeverity values when
                                              ; non-null, per the registry snapshot keyed by pkaf_version.
  rationale:          tstr / null,            ; Free text per PKAF.
  last_verified_at:   uint / null,            ; Plan 7d freshness; Unix seconds UTC.
  verified_by:        tstr / null,            ; Plan 7d freshness; verifier IRI.
  pkaf_version:       tstr,                   ; SemVer of the PKAF release whose closed taxonomies
                                              ; bind this Finding (e.g. "0.1.x" or "0.2.0"). The
                                              ; verifier resolves finding_kind / severity admissibility
                                              ; against this version's registry snapshot; unknown
                                              ; versions flag rkaf_finding_pkaf_version_unknown.
  extensions:         { * tstr => any } / null,
}
```

Naming convention: snake_case for Trellis wire (per ADR 0001 byte-style), with `finding_iri` instead of `@id` because dCBOR map keys are text strings without JSON-LD `@`-prefix semantics. The `@id` ↔ `finding_iri` lift is lossless and pinned in §"Cross-spec field mapping" below.

### Cross-spec field mapping

| PKAF / JSON-LD | Trellis CDDL | Notes |
|---|---|---|
| `@id` | `finding_iri` | Identity. |
| `@type` (`"rkaf:Finding"`) | implicit (the extension key `trellis.rkaf-finding.v1` is the type discriminator) | No wire field. |
| `rkaf:findingKind` | `finding_kind` | Closed taxonomy. |
| `rkaf:detectedAt` (xsd:dateTime) | `detected_at` (uint, seconds UTC) | Lossy in sub-second precision direction (Trellis is seconds); this matches every other Trellis timestamp (Core §6.1). Producers that need sub-second precision MUST encode it in `extensions` per Core §6.7. |
| `rkaf:detectedBy` | `detected_by` | IRI; verifier syntactic check only. |
| `rkaf:subject` | `subject` | IRI; verifier syntactic check only. |
| `rkaf:severity` | `severity` (nullable) | Closed taxonomy. |
| `rkaf:rationale` | `rationale` (nullable) | Free text. |
| `rkaf:lastVerifiedAt` | `last_verified_at` (nullable, seconds UTC) | Plan 7d. |
| `rkaf:verifiedBy` | `verified_by` (nullable) | Plan 7d. |
| (none — Trellis-side only) | `pkaf_version` | Pins the registry snapshot version. PKAF's own version stamping (e.g. JSON-LD `@context` URI) is consumer-side; Trellis carries an explicit version because the verifier needs to know which closed-taxonomy snapshot to check against. |

This mapping is **the canonical serialization function** (PKAF §4.6 requirement 2). Two consumers given the same PKAF Finding MUST produce byte-identical `RkafFindingPayload` bytes after dCBOR encoding under deterministic field ordering (Core §5 dCBOR). Byte-exact equivalence is the Trellis-side guarantee that re-anchoring the same Rulespec subgraph produces a verifiable equality test.

### Authority ladder note

Per CLAUDE.md, Rust is the byte authority. **This ADR pins byte semantics in CDDL** because no Rust type lands in this change. When a concrete adopter triggers Rust implementation (Open questions §3), the Rust type `RkafFindingPayload` in `trellis-types` (or a new `trellis-rkaf` crate; see §"Crate placement" below) becomes the byte oracle. CDDL is structural authority until then; ADR 0004 governs disagreements.

## Event-type registration (Core §6.7)

Add to the Core §6.7 Extension Registry:

| Container | Identifier | Phase | Purpose |
|---|---|---|---|
| `EventPayload.extensions` | `trellis.rkaf-finding.v1` | 1 | `rkaf:Finding` (PKAF ADR-0093) anchoring binding per ADR 0012. Payload shape `RkafFindingPayload` (ADR 0012 §"Wire shape"). Verifier obligations in §19 step 6e under no new domain tag (the host event's `trellis-event-v1` is sufficient — Findings are not separately signed; ADR 0012 §"Decision" R3). PKAF §4.6 anchor type URI `urn:rkaf:anchor:trellis/1`. Reject-if-unknown-at-version. |

## Domain-separation tag (Core §9.8)

**None added.** Rationale recorded under §"Decision" R3.

## Verifier obligations (Core §19 step 6e — new substep)

A conforming Phase-1 verifier processing an export bundle containing `trellis.rkaf-finding.v1` events MUST, in order:

1. **Decode** the payload against the CDDL above. Structural mismatch (missing required field, wrong CBOR type, unrecognized version-1 top-level field) is a structure failure (Core §19 step 1) with `tamper_kind = rkaf_finding_structure_invalid`.
2. **Validate closed-taxonomy membership.** Resolve `pkaf_version` to a Trellis-side registry snapshot of `#FindingKind` and `#FindingSeverity` (initial snapshot: PKAF 0.1.x values per `PKAF/constraints/core/finding.cue` at ADR-0093 ratification — see §"Closed-taxonomy registry" below). Unknown `pkaf_version` flags `rkaf_finding_pkaf_version_unknown`. `finding_kind` not in the snapshot's `#FindingKind` set flags `rkaf_finding_kind_unregistered`. `severity` non-null and not in the snapshot's `#FindingSeverity` set flags `rkaf_finding_severity_unregistered`.
3. **Validate intra-payload invariants:**
   - `detected_at == envelope.authored_at` exactly (no skew slack). Mismatch flags `rkaf_finding_timestamp_mismatch`.
   - `finding_iri`, `detected_by`, `subject`, and (when non-null) `verified_by` are syntactically valid IRIs per RFC 3987. Malformed flag `rkaf_finding_iri_malformed` localized to the offending field.
   - When `last_verified_at` is non-null, it MUST be `<= detected_at` (a Finding cannot be verified later than it was detected and still appear in the same event — re-verification produces a NEW Finding event, not a retroactive update; this is the Plan 7d freshness discipline binding). Mismatch flags `rkaf_finding_freshness_temporal_inversion`.
4. **Detect IRI collision.** After decoding all `rkaf-finding.v1` events in `ledger_scope`, two events sharing `finding_iri` with disagreeing canonical payloads flag `rkaf_finding_iri_collision`. Re-emission with byte-identical canonical payloads is idempotent and silent (first-seen wins is non-normative).
5. **Cross-check detector identity (advisory).** `RkafFindingPayload.detected_by` is the Rulespec detector IRI; `EventHeader.author` is the Trellis principal URI of the signer. These two are linked at the deployment layer (the Posture Declaration declares which Trellis principals are authorized to sign Findings on behalf of which Rulespec detectors). The Trellis verifier MUST surface the pair in the report but MUST NOT enforce a binding rule at this layer — detector↔principal mapping is consumer-domain (Studio / `wos-server` / BVR runtime), not Trellis-domain. The optional `report.rkaf_findings[*].detector_principal_consistent` field is populated only when a consumer-domain resolver is provided to the verifier (parallel to the `trellis-verify-wos` resolver pattern in ADR 0007); when absent, the field is `null` and global integrity is not affected by this check.
6. **Accumulate** outcomes into `VerificationReport.rkaf_findings`, parallel to `posture_transitions` / `erasure_evidence` / `certificates_of_completion` / `user_content_attestations` / `interop_sidecars`. Each entry carries:

   ```cddl
   RkafFindingVerificationEntry = {
     finding_iri:                   tstr,
     finding_kind:                  tstr,
     severity:                      tstr / null,
     subject:                       tstr,
     detected_at:                   uint,
     pkaf_version:                  tstr,
     structure_valid:               bool,
     taxonomy_valid:                bool,
     intra_payload_invariants_ok:   bool,
     iri_collision:                 bool,
     detector_principal_consistent: bool / null,    ; null when no consumer resolver is provided
     failures:                      [* tstr],
   }
   ```

**Global integrity (Finding slice).** `integrity_verified = false` if any entry has any of `structure_valid = false`, `taxonomy_valid = false`, `intra_payload_invariants_ok = false`, or `iri_collision = true`. `detector_principal_consistent = false` is NOT a global-integrity failure — it surfaces in the consumer-domain layer (Studio / WOS verifier) per ADR 0008 ISC-01 / ADR 0007 path-(b) discipline.

## Closed-taxonomy registry

The PKAF `#FindingKind` and `#FindingSeverity` enumerations are versioned by PKAF and snapshotted Trellis-side per `pkaf_version`. Snapshot rules:

- Trellis-side snapshots are **derived artifacts** of PKAF releases. The Trellis canonical store is `crates/trellis-types/src/rkaf/finding_taxonomy.rs` (Rust) when the binding lands; until then, the snapshot for PKAF 0.1.x is the literal value-set enumerated in `PKAF/constraints/core/finding.cue` at PKAF ADR-0093 ratification (2026-05-14):
  - `#FindingKind` 0.1.x: `rkaf:warning`, `rkaf:error`, `rkaf:staleDependency`, `rkaf:registryUnavailable`, `rkaf:registryVersionOutOfRange`, `rkaf:conceptConflict`, `rkaf:authorityBroken`, `rkaf:unsupportedAnchor`, `rkaf:other`.
  - `#FindingSeverity` 0.1.x: `rkaf:informational`, `rkaf:operationalConflict`, `rkaf:publicationBlocking`, `rkaf:authorityCritical`.
- A PKAF version bump that adds taxonomy values requires a Trellis-side snapshot landing in the SAME change train that admits the new `pkaf_version` to the verifier's supported set. This is the "vectors and Rust move together" discipline (CLAUDE.md) applied to cross-spec closed-taxonomy snapshots.
- Trellis does NOT independently validate PKAF taxonomy semantics; it only validates set-membership. Semantic meaning (what `rkaf:operationalConflict` *means*) is PKAF's.

## Crate placement

When Rust implementation triggers (Open questions §3), the binding code lives in **a new sibling crate `trellis-rkaf`** in `trellis/crates/`, NOT in `trellis-core` / `trellis-types` / `trellis-verify`. Rationale:

- Core §16 verification independence (CLAUDE.md "verification independence contract is load-bearing") forbids non-essential dependencies in `trellis-verify`.
- Per ADR 0008 ISC-05 and the consumer-crate-boundary discipline in CLAUDE.md, consumer-specific (here: Rulespec-specific) semantics live outside Trellis center crates.
- The dependency graph is one-way: `trellis-rkaf → trellis-types` (for `EventPayload` access). `trellis-verify` MAY depend on `trellis-rkaf` for the dispatch path (parallel to how `trellis-verify-wos` is the WOS dispatch path), but `trellis-core` MUST NOT.
- A future `trellis-interop-rkaf` adapter could emit a SCITT receipt or VC envelope over Findings per ADR 0008; that adapter (when needed) lives separately and depends on `trellis-rkaf` for the canonical struct.

This placement is normative for implementation; the crate need not exist until an adopter triggers it.

## Phase alignment

- **Phase 1 envelope compatible.** Adds one row to Core §6.7 extension registry; no event-envelope structural change. Phase-1 producers MAY emit `trellis.rkaf-finding.v1` events; Phase-1 verifiers MUST process them per §"Verifier obligations" above (reject-if-unknown-at-version). Invariant #10 preserved.
- **Phase 2+ external anchoring** (Core §11.5, OpenTimestamps / SCITT receipt). A Finding event MAY ALSO carry a Phase-2+ `trellis.external_anchor.v1` entry alongside `trellis.rkaf-finding.v1`. The two extensions compose without coupling: the Phase-1 anchor (COSE_Sign1 + Merkle inclusion) is the canonical anchor; an external anchor is additive evidence per ADR 0008 ISC-01.
- **Phase 3 case-ledger composition.** Findings compose at the case-ledger head level identically to other events. A case ledger MAY carry Findings about the case, about responses within the case, or about Rulespec artifacts cited by the case — all under the same wire shape.
- **Phase 4 federation.** Witness nodes verify Finding events via the standard Phase-1 verifier path; no witness-specific obligation. A federation member MAY emit Findings independently; downstream consumers reconcile via the Rulespec `rkaf:Finding` IRI namespace.

## Idempotency, mutability, and supersession

- **Idempotency.** Re-emission of a Finding event with the same `finding_iri` MUST carry byte-identical canonical payload bytes (idempotent by hash). Collision flags `rkaf_finding_iri_collision` per §"Verifier obligations" step 4.
- **Mutability.** Findings are append-only at the Trellis layer. "The detector re-checked at T+1 and found the same issue" produces a NEW Finding event (new `finding_iri`, new `detected_at`) and SHOULD use ADR 0066 supersession linkage (`trellis.supersedes-chain-id.v1`) if it logically supersedes a prior Finding about the same subject. Studio's "waived" lifecycle composes via PKAF `Attestation(targetFinding=…)` referencing the Finding IRI, NOT by mutating the Finding event.
- **Lifecycle state outside Trellis.** Whether a Finding is "open" / "waived" / "remediated" is consumer-domain (Studio readiness-tier projection per PKAF ADR-0093). Trellis stores the detection; the lifecycle is a downstream graph computation over the case ledger.

## Fixture plan

Deferred to the implementation trigger (Open questions §3). When the first emitter ships, the corpus additions are:

| Vector | Purpose |
|---|---|
| `append/NNN-rkaf-finding-emission` | Canonical positive: one Finding event with all required fields; verifier emits a populated `rkaf_findings` slice with all booleans `true`. |
| `append/NNN-rkaf-finding-optional-fields-omitted` | Canonical positive: `severity`, `rationale`, `last_verified_at`, `verified_by` all null; verifier accepts. |
| `tamper/NNN-rkaf-finding-kind-unregistered` | `finding_kind = "rkaf:not-a-real-kind"`; verifier fails with `rkaf_finding_kind_unregistered`. |
| `tamper/NNN-rkaf-finding-severity-unregistered` | `severity = "rkaf:catastrophic"`; verifier fails with `rkaf_finding_severity_unregistered`. |
| `tamper/NNN-rkaf-finding-timestamp-mismatch` | `detected_at != envelope.authored_at`; verifier fails with `rkaf_finding_timestamp_mismatch`. |
| `tamper/NNN-rkaf-finding-iri-malformed` | `finding_iri` is not a valid RFC 3987 IRI; verifier fails with `rkaf_finding_iri_malformed`. |
| `tamper/NNN-rkaf-finding-iri-collision` | Two events sharing `finding_iri` with disagreeing canonical payloads; verifier fails with `rkaf_finding_iri_collision`. |
| `tamper/NNN-rkaf-finding-freshness-inversion` | `last_verified_at > detected_at`; verifier fails with `rkaf_finding_freshness_temporal_inversion`. |
| `tamper/NNN-rkaf-finding-pkaf-version-unknown` | `pkaf_version = "99.99.99"`; verifier fails with `rkaf_finding_pkaf_version_unknown`. |

Slot numbers TBD per the corpus-batching convention in effect at implementation time.

## Cross-references

- **PKAF ADR-0093** — promotes `rkaf:Finding` to first-class primitive; names this Trellis binding as the open cross-stack work item. This ADR closes that item from the Trellis side.
- **PKAF §4.6** — abstract anchoring contract. Trellis is one of several plausible bindings; the framework is honest that VC / COSE_Sign1 / Sigstore / IPFS are equally valid alternative bindings, and adopters MAY use multiple bindings simultaneously.
- **Core §6.7 Extension Registry** — one row addition (table above).
- **Core §19** — adds step 6e (new substep) with the verifier checklist above. Companion update lands with implementation.
- **TR-CORE-178 (new row, to be added to `specs/trellis-requirements-matrix.md`)** — anchors the verifier obligations above and the closed-taxonomy snapshot discipline. `Verification = test-vector` (deferred until the fixture corpus above lands). This ADR's `Status: Proposal — open` is preserved on the matrix row until the Rust binding ships.
- **ADR 0010** — comparator pattern for "external-content-anchored event with structural payload"; differences enumerated under §"Decision" R2.
- **ADR 0008** — comparator pattern for "cross-spec interop addition"; differences enumerated under §"Decision" R1.
- **ADR 0066** — supersession linkage primitive that Findings about the same subject SHOULD compose with.
- **CLAUDE.md (Trellis)** — authority ladder (Rust > CDDL > prose > matrix > Python > archives), no-stubs rule, consumer-crate-boundary discipline.

## Open questions / follow-ons

1. **`?anchorIRI` resolution URI scheme.** PKAF §4.6 declares Rulespec assertions point at anchors via `rkaf:anchoredBy ?anchorIRI` where the anchor IRI's structure is binding-defined. This ADR defers the concrete URI shape (probably `urn:rkaf:anchor:trellis/1:<ledger_scope>:<canonical_event_hash hex>[:<checkpoint_tree_size>]` or a Trellis-hosted HTTPS resolver). Decided in a follow-on RFC alongside the broader `source_ref` resolution question raised in ADR 0008 Open Q5. This is benign for verification (the verifier resolves Findings by canonical_event_hash, not by anchor IRI) but matters for Rulespec graph readability.
2. **Detector-principal mapping at the Posture Declaration.** §"Verifier obligations" step 5 surfaces detector↔principal consistency as advisory. The companion-spec change (Posture Declaration field declaring `authorized_rkaf_detectors: [{ trellis_principal, detector_iri }]`) lands when the first Rulespec-aware deployment configures one — likely co-landed with the `trellis-verify-wos`-style resolver for Findings. The ADR does not pre-commit the Posture Declaration shape; it pre-commits the verifier API surface (`detector_principal_consistent` is nullable, populated by an optional resolver).
3. **Rust implementation trigger.** The first concrete adopter (Studio compiler emitting Findings, `wos-server` validation event emission, or a standalone BVR runtime) triggers `trellis-rkaf` crate creation + fixture corpus + Python stranger mirror. The CLAUDE.md "no stubs" rule forbids creating the crate empty.
4. **Studio readiness-tier projection.** PKAF ADR-0093 names this as a Studio-side concern: Findings + their PKAF Attestation waivers compose into a readiness tier per case. Trellis's responsibility ends at "Findings are anchored, addressable, and verifiable offline"; readiness projection is over the case ledger graph, not in the wire shape. Documented here to foreclose scope creep into Trellis.
5. **PKAF version-snapshot governance.** Open question: who owns the Trellis-side `pkaf_version` → taxonomy-snapshot table? Two options: (a) Trellis maintains the snapshot table and bumps it on each PKAF release (loose coupling — Trellis can lag); (b) PKAF emits a machine-readable taxonomy descriptor per release and Trellis consumes it (tight coupling — bumps are mechanical). Decision deferred; the wire shape is identical either way. Probably (b) once PKAF's release cadence stabilizes (Phase 3 framework + finalized anchoring contract per the PKAF roadmap).

---

*End of ADR 0012.*
