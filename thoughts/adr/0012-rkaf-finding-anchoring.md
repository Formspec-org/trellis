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
| **(3) Verification function** | The Phase-1 Trellis verifier (Core §19) over the host event, plus the structural decode + invariant checks enumerated in §"Verifier obligations" below. Returns `commit` iff `integrity_verified = true` AND `finding_iri` matches the Rulespec subgraph the consumer is checking against. The anchor IRI (§4.6 requirement 1) resolves to `(ledger_scope, canonical_event_hash[, checkpoint_tree_size])` via the URI template pinned in §"Anchor IRI resolution" below; the verifier resolves a Finding by `canonical_event_hash`, so the URI template is read-only on the Trellis side. |
| **(4) Spec published outside Rulespec** | This ADR. PKAF spec does not name Trellis. |

The Rulespec consumer expresses the binding in its graph with:

```turtle
?finding rkaf:anchoredBy ?anchorIRI .
?anchorIRI rkaf:anchorType <urn:rkaf:anchor:trellis/1> .
# Binding-defined: anchorIRI dereferences to the Trellis event hash + checkpoint coordinates.
```

The shape of `?anchorIRI` is **pinned here** (closes Open question §1 partial — full HTTPS-resolver discussion remains open) so that two consumers reading the same Rulespec graph derive the same Trellis coordinates without coordination. See §"Anchor IRI resolution" below for the URI template. The verifier resolves a Finding by `canonical_event_hash`, not by URI; the URI template is the Rulespec-graph-readable form that consumers parse to recover the resolution coordinates.

### Anchor IRI resolution

The Rulespec `?anchorIRI` for a Trellis-bound Finding follows the URI template:

```
urn:rkaf:anchor:trellis/1:<ledger_scope>:<canonical_event_hash>[:<checkpoint_tree_size>]
```

with the following normative production rules:

- `<ledger_scope>` is the lowercase hex encoding of the host event's `EventHeader.ledger_scope` per Core §4 (case-ledger scope identifier). Bytes are encoded as a single lowercase hex string with no separators.
- `<canonical_event_hash>` is the lowercase hex encoding of the host event's `canonical_event_hash` per Core §10 (the byte-exact hash that anchors the Finding event in the chain). 64 hex characters for SHA-256.
- `<checkpoint_tree_size>` is OPTIONAL. When present, it is the decimal `tree_size` (Core §11) of a Checkpoint that includes the host event in its Merkle tree. Producers MAY omit it; consumers MAY tolerate either form. When two `?anchorIRI` values differ only in the presence/value of `<checkpoint_tree_size>` they refer to the same Finding — `canonical_event_hash` is the disambiguator.
- The colon separator is the URN `:` character per RFC 8141; no percent-encoding is applied to the hex/decimal segments (they are already URI-safe).

Phase-1 verifiers do not parse `?anchorIRI` — they receive a `canonical_event_hash` from the export bundle directly. The URI template is the **producer-side and Rulespec-graph-side** canonical form; the Trellis verifier reads the host event hash from the bundle and the consumer reconciles by comparing hash bytes after parsing the URI.

An HTTPS-resolver variant for follow-on RFC work (e.g., `https://<trellis-deployment>/anchor/v1/<ledger_scope>/<canonical_event_hash>`) remains open (Open question §1 retained); the `urn:rkaf:anchor:trellis/1:…` URN form is the wire-canonical shape that bindings MUST emit.

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
                                              ; Byte-determinism: finding_iri is emitted byte-exact as the
                                              ; producer received it; producers SHOULD apply RFC 3987 §5.3
                                              ; normalization before emission. The verifier does NOT
                                              ; re-normalize; idempotency / collision detection (step 4)
                                              ; is byte-exact at the consumer.
                                              ; Idempotency key within ledger_scope: two events sharing
                                              ; the same finding_iri carry byte-identical canonical
                                              ; payloads (silent idempotent), differ only in freshness
                                              ; fields (rkaf_finding_iri_collision, collision_class =
                                              ; "freshness_update", advisory — Plan 7d re-emission), or
                                              ; differ in other fields (rkaf_finding_iri_collision,
                                              ; collision_class = "structural", global-integrity
                                              ; failure). See "Verifier obligations" step 4.
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
                                              ; PKAF-absent ↔ CDDL null; see "Cross-spec field mapping"
                                              ; for the round-trip rule.
  rationale:          tstr / null,            ; Free text per PKAF. PKAF-absent ↔ CDDL null.
  last_verified_at:   uint / null,            ; Plan 7d freshness; Unix seconds UTC. PKAF-absent ↔
                                              ; CDDL null. When non-null, see verifier-step-3 invariant.
  verified_by:        tstr / null,            ; Plan 7d freshness; verifier IRI. PKAF-absent ↔ CDDL null.
  pkaf_version:       tstr,                   ; Strict SemVer 2.0.0 string identifying the PKAF release
                                              ; whose closed taxonomies bind this Finding (e.g. "0.1.0",
                                              ; "0.2.0"). Range or wildcard keys (e.g. "0.1.x", "^0.2",
                                              ; "0.1") are NOT admitted — verifier flags
                                              ; rkaf_finding_pkaf_version_unknown if pkaf_version does
                                              ; not parse as strict SemVer 2.0.0 OR is not present in
                                              ; the verifier's supported set.
  extensions:         { * tstr => any } / null,
                                              ; Sub-map ordering follows Core §5 dCBOR deterministic
                                              ; key ordering recursively; nested maps inherit the same
                                              ; rule. See "Byte determinism corners" below.
}
```

Naming convention: snake_case for Trellis wire (per ADR 0001 byte-style), with `finding_iri` instead of `@id` because dCBOR map keys are text strings without JSON-LD `@`-prefix semantics. The `@id` ↔ `finding_iri` lift is lossless and pinned in §"Cross-spec field mapping" below.

### Byte-determinism corners

Two faithful implementations MUST produce byte-identical `RkafFindingPayload` bytes from the same logical Finding. To foreclose divergence the spec pins:

1. **`finding_iri` normalization.** RFC 3987 §5.3 admits multiple equivalent serializations of the same IRI; the verifier MUST NOT silently normalize. Producers SHOULD apply §5.3 normalization before emission; consumers compare bytes exactly. Two events whose `finding_iri` bytes differ (even if RFC 3987 §5.3 would equate them as the same logical IRI) are treated as DIFFERENT Findings by the Trellis verifier — no collision detection fires. The collision rule (step 4) is keyed on byte-exact `finding_iri` equality. Consumer-domain code MAY layer §5.3 normalization on top as part of Rulespec-graph reconciliation; that is OUT of scope for the Trellis verifier.

2. **Field and nested-map ordering.** Top-level `RkafFindingPayload` keys and any nested map inside `extensions` follow Core §5 dCBOR deterministic key ordering recursively. The `extensions` `{ * tstr => any }` shape does not relax this rule — implementations MUST sort keys per dCBOR at every map level inside the value.

3. **`pkaf_version` form.** Strict SemVer 2.0.0 strings only. Range expressions, wildcards, partial versions (`"0.1"`), pre-release tags admitted as long as the string parses under SemVer 2.0.0; an unparseable string OR a string not present in the verifier's supported set flags `rkaf_finding_pkaf_version_unknown`. This forecloses the "0.1.x vs 0.1.0 vs ^0.1" ambiguity that two implementers could resolve differently.

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

**Null vs absent round-trip.** PKAF CUE marks `rkaf:severity`, `rkaf:rationale`, `rkaf:lastVerifiedAt`, `rkaf:verifiedBy` as OPTIONAL (`?`-suffixed). The Trellis wire admits these fields as REQUIRED-but-nullable (`tstr / null` / `uint / null`). The canonical round-trip rule is:

- PKAF field **absent** ↔ Trellis CDDL field set to **`null`** (the field is still present in the dCBOR map with a null value).
- PKAF field **present-and-null** is **illegal** under PKAF §5.3 (CUE `?: string` admits absent or string, never explicit null in the JSON-LD form).
- PKAF field **present with a value** ↔ Trellis CDDL field set to that value (with type-coerced encoding per §"Cross-spec field mapping").

A producer encountering PKAF-present-and-null MUST treat the input as malformed PKAF (escalate to the PKAF layer); it MUST NOT silently lower it to CDDL null. This forecloses the "two implementers disagree on what absent-vs-null means" failure mode.

### Authority ladder note

Per CLAUDE.md, Rust is the byte authority. **This ADR pins byte semantics in CDDL** because no Rust type lands in this change. When a concrete adopter triggers Rust implementation (Open questions §3), the Rust type `RkafFindingPayload` in `trellis-types` (or a new `trellis-rkaf` crate; see §"Crate placement" below) becomes the byte oracle. CDDL is structural authority until then; ADR 0004 governs disagreements.

## Event-type registration (Core §6.7)

Add to the Core §6.7 Extension Registry:

| Container | Identifier | Phase | Purpose |
|---|---|---|---|
| `EventPayload.extensions` | `trellis.rkaf-finding.v1` | 1 | `rkaf:Finding` (PKAF ADR-0093) anchoring binding per ADR 0012. Payload shape `RkafFindingPayload` (ADR 0012 §"Wire shape"). Verifier obligations in §19 step 6f under no new domain tag (the host event's `trellis-event-v1` is sufficient — Findings are not separately signed; ADR 0012 §"Decision" R3). PKAF §4.6 anchor type URI `urn:rkaf:anchor:trellis/1`. Reject-if-unknown-at-version. |

## Domain-separation tag (Core §9.8)

**None added.** Rationale recorded under §"Decision" R3.

## Verifier obligations (Core §19 step 6f — new substep)

A conforming Phase-1 verifier processing an export bundle containing `trellis.rkaf-finding.v1` events MUST, in order:

1. **Decode** the payload against the CDDL above. Structural mismatch (missing required field, wrong CBOR type, unrecognized version-1 top-level field) is a structure failure (Core §19 step 1) with `tamper_kind = rkaf_finding_structure_invalid`.
2. **Validate closed-taxonomy membership.** Resolve `pkaf_version` to a Trellis-side registry snapshot of `#FindingKind` and `#FindingSeverity` (initial snapshot: PKAF 0.1.x values per `PKAF/constraints/core/finding.cue` at ADR-0093 ratification — see §"Closed-taxonomy registry" below). Unknown `pkaf_version` flags `rkaf_finding_pkaf_version_unknown`. `finding_kind` not in the snapshot's `#FindingKind` set flags `rkaf_finding_kind_unregistered`. `severity` non-null and not in the snapshot's `#FindingSeverity` set flags `rkaf_finding_severity_unregistered`.
3. **Validate intra-payload invariants:**
   - `detected_at == envelope.authored_at` exactly (no skew slack). Mismatch flags `rkaf_finding_timestamp_mismatch`.
   - `finding_iri`, `detected_by`, `subject`, and (when non-null) `verified_by` are syntactically valid IRIs per RFC 3987. Malformed flag `rkaf_finding_iri_malformed` localized to the offending field.
   - When `last_verified_at` is non-null, it MUST be `>= detected_at` (PKAF Plan 7d freshness semantics: `lastVerifiedAt` answers "when did someone last confirm this Finding still holds?" — re-verification of an existing detection, by definition LATER than or equal to detection. A freshness timestamp earlier than the detection it claims to re-verify is malformed). Mismatch flags `rkaf_finding_freshness_precedes_detection`.
4. **Detect IRI collision.** After decoding all `rkaf-finding.v1` events in `ledger_scope`, two events sharing `finding_iri` are classified as follows:
   - **Byte-identical canonical payloads** → idempotent and silent (no flag; first-seen wins is non-normative).
   - **Differing bytes confined to the freshness fields** (`last_verified_at` and/or `verified_by`; all other fields byte-identical) → flag `rkaf_finding_iri_collision` with `collision_class = "freshness_update"`. This is the PKAF Plan 7d freshness re-emission case; the Trellis layer surfaces the signal but consumer-domain (Studio / WOS) resolves admissibility. NOT a global-integrity failure.
   - **Differing bytes outside the freshness fields** → flag `rkaf_finding_iri_collision` with `collision_class = "structural"`. A `rkaf:Finding` IRI identifies one detection; structurally-disagreeing re-emissions under the same IRI are producer error or tampering. IS a global-integrity failure.
5. **Cross-check detector identity (advisory).** `RkafFindingPayload.detected_by` is the Rulespec detector IRI; `EventHeader.author` is the Trellis principal URI of the signer. These two are linked at the deployment layer (the Posture Declaration declares which Trellis principals are authorized to sign Findings on behalf of which Rulespec detectors). The Trellis verifier MUST surface the pair in the report but MUST NOT enforce a binding rule at this layer — detector↔principal mapping is consumer-domain (Studio / `wos-server` / BVR runtime), not Trellis-domain. The optional `report.rkaf_findings[*].detector_principal_consistent` field is populated only when a consumer-domain resolver is provided to the verifier (parallel to the `trellis-verify-wos` consumer-resolver pattern from ADR 0008 path-(b) and the identity-attestation resolver in ADR 0010 §"Verifier obligations" step 4). The contract for this resolver is pinned in §"Resolver contract" below; when no resolver is provided, the field is `null` and global integrity is not affected by this check.
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
     collision_class:               tstr / null,    ; "freshness_update" or "structural" when iri_collision = true; null otherwise.
     detector_principal_consistent: bool / null,    ; null when no consumer resolver is provided
     failures:                      [* tstr],
   }
   ```

**Global integrity (Finding slice).** `integrity_verified = false` if any entry has any of `structure_valid = false`, `taxonomy_valid = false`, `intra_payload_invariants_ok = false`, OR (`iri_collision = true` AND `collision_class = "structural"`). A `freshness_update` collision is NOT a global-integrity failure — it is the Plan 7d freshness re-emission case, surfaced to the consumer-domain layer (Studio / WOS) which resolves admissibility per ADR 0008 ISC-01 / ADR 0008 path-(b) discipline. `detector_principal_consistent = false` is also NOT a global-integrity failure (same discipline).

### Resolver contract

The detector↔principal cross-check (step 5) is **optional** at the Trellis layer but its API surface is pinned here so two implementations facing the same posture cannot legally disagree on whether to populate the field. The Trellis verifier takes an optional consumer-domain resolver (parallel to `trellis-verify-wos::WosFormspecResolver` per ADR 0008 path-(b) consumer pattern, and the cross-spec resolver shape ADR 0010 step 4 uses for identity-attestation resolution at `trellis-core.md` §19 step 6d):

- **(a) Resolver provided.** The verifier MUST invoke the resolver for every `RkafFindingPayload` decoded in step 1, MUST surface a non-null `detector_principal_consistent: bool` in the corresponding `RkafFindingVerificationEntry`, and MUST record the resolver's failure code (when `false`) under `failures: [* tstr]`. A resolver that itself errors out for reasons unrelated to the detector↔principal mapping (e.g., transport failure) is reported as `detector_principal_consistent = null` with a localized `resolver_error` failure — distinct from "resolver ran and said inconsistent."
- **(b) Resolver NOT provided.** The verifier MUST set `detector_principal_consistent = null` and MUST NOT fail `integrity_verified` on the absent cross-check. The advisory nature of the field is honest reporting of "no consumer-domain validator was wired in," not a silent pass.
- **(c) Posture Declaration field shape.** OPEN (Open question §2 — the Posture Declaration shape `authorized_rkaf_detectors: [{ trellis_principal, detector_iri }]` is illustrative, not pre-committed). The resolver API surface (this subsection) is CLOSED here — Posture Declaration shape can evolve without breaking the verifier interface.

This mirrors the discipline ADR 0010 §"Verifier obligations" step 4 applies to identity-attestation resolution: the resolver interface is consumer-supplied, the verifier reports honestly when absent, and absence is not silent-pass.

## Closed-taxonomy registry

The PKAF `#FindingKind` and `#FindingSeverity` enumerations are versioned by PKAF and snapshotted Trellis-side per `pkaf_version`. Snapshot rules:

- Trellis-side snapshots are **derived artifacts** of PKAF releases. The Trellis canonical store is `crates/trellis-types/src/rkaf/finding_taxonomy.rs` (Rust) when the binding lands; until then, the snapshot for PKAF 0.1.x is the literal value-set enumerated in `PKAF/constraints/core/finding.cue` at PKAF ADR-0093 ratification (2026-05-14):
  - `#FindingKind` 0.1.x: `rkaf:warning`, `rkaf:error`, `rkaf:staleDependency`, `rkaf:registryUnavailable`, `rkaf:registryVersionOutOfRange`, `rkaf:conceptConflict`, `rkaf:authorityBroken`, `rkaf:unsupportedAnchor`, `rkaf:other`.
  - `#FindingSeverity` 0.1.x: `rkaf:informational`, `rkaf:operationalConflict`, `rkaf:publicationBlocking`, `rkaf:authorityCritical`.
- A PKAF version bump that adds taxonomy values requires a Trellis-side snapshot landing in the SAME change train that admits the new `pkaf_version` to the verifier's supported set. This is the "vectors and Rust move together" discipline (CLAUDE.md) applied to cross-spec closed-taxonomy snapshots.
- Trellis does NOT independently validate PKAF taxonomy semantics; it only validates set-membership. Semantic meaning (what `rkaf:operationalConflict` *means*) is PKAF's.

## Crate placement

When Rust implementation triggers (Open questions §3), the binding code lives in **a new sibling crate `trellis-rkaf`** in `trellis/crates/`, NOT in `trellis-core` / `trellis-types` / `trellis-verify`, and NOT under the `trellis-interop-*` cluster. Rationale:

- Core §16 verification independence (CLAUDE.md "verification independence contract is load-bearing") forbids non-essential dependencies in `trellis-verify`.
- Per ADR 0008 ISC-05 and the consumer-crate-boundary discipline in CLAUDE.md, consumer-specific (here: Rulespec-specific) semantics live outside Trellis center crates.
- **Why not `trellis-interop-rkaf`?** The `trellis-interop-*` cluster (`trellis-interop-c2pa`, `trellis-interop-did`, `trellis-interop-scitt`, `trellis-interop-vc`) is the home for **export-bundle sidecars** per ADR 0008: deterministic derivations of canonical records into external-ecosystem envelope formats, carried alongside canonical bytes in `interop-sidecars/` and dispatched path-(b) by the verifier (digest-binds only; no `source_ref` resolution). ADR 0012 is structurally different — it is a **first-class event payload extension** under `EventPayload.extensions["trellis.rkaf-finding.v1"]`, anchored by the host event's COSE_Sign1 + Merkle inclusion (not by sidecar digest). This is the same event-extension-vs-sidecar distinction enumerated in §"Decision" R1. The `interop-*` naming would conflate two architecturally distinct patterns (event-extension and export-sidecar) and force the wrong dispatch shape onto consumers.
- The dependency graph is one-way: `trellis-rkaf → trellis-types` (for `EventPayload` access). `trellis-verify` MAY depend on `trellis-rkaf` for the dispatch path (parallel to how `trellis-verify-wos` is the WOS dispatch path), but `trellis-core` MUST NOT.
- A future `trellis-interop-rkaf` adapter could emit a SCITT receipt or VC envelope over Findings per ADR 0008 — that **would** belong under the `interop-*` cluster because it would be an export-bundle sidecar derivation. That adapter (when needed) lives separately and depends on `trellis-rkaf` for the canonical struct. The two crates compose without overlap: `trellis-rkaf` owns the in-chain event extension; `trellis-interop-rkaf` (hypothetical) would own the sidecar derivation.

This placement is normative for implementation; the crate need not exist until an adopter triggers it.

## Phase alignment

- **Phase 1 envelope compatible.** Adds one row to Core §6.7 extension registry; no event-envelope structural change. Phase-1 producers MAY emit `trellis.rkaf-finding.v1` events; Phase-1 verifiers MUST process them per §"Verifier obligations" above (reject-if-unknown-at-version). Invariant #10 preserved.
- **Phase 2+ external anchoring** (Core §11.5, OpenTimestamps / SCITT receipt). A Finding event MAY ALSO carry a Phase-2+ `trellis.external_anchor.v1` entry alongside `trellis.rkaf-finding.v1`. The two extensions compose without coupling: the Phase-1 anchor (COSE_Sign1 + Merkle inclusion) is the canonical anchor; an external anchor is additive evidence per ADR 0008 ISC-01.
- **Phase 3 case-ledger composition.** Findings compose at the case-ledger head level identically to other events. A case ledger MAY carry Findings about the case, about responses within the case, or about Rulespec artifacts cited by the case — all under the same wire shape.
- **Phase 4 federation.** Witness nodes verify Finding events via the standard Phase-1 verifier path; no witness-specific obligation. A federation member MAY emit Findings independently; downstream consumers reconcile via the Rulespec `rkaf:Finding` IRI namespace.

## Idempotency, mutability, and supersession

- **Idempotency.** Re-emission of a Finding event with the same `finding_iri` AND byte-identical canonical payload bytes is idempotent and silent (no `rkaf_finding_iri_collision`; first-seen wins is non-normative).
- **Same-IRI re-emission with differing bytes** (collision) covers two distinct cases:
  - **Freshness re-emission** (PKAF Plan 7d): re-verification of an existing Finding updates `last_verified_at` and `verified_by` on the SAME `finding_iri`, producing a NEW event whose bytes differ from the prior emission. The `finding_iri` collision flag (`rkaf_finding_iri_collision`) fires correctly as the structural-change signal — but the Trellis-layer interpretation is **advisory only**. Whether the freshness update is admitted is consumer-domain (Studio / WOS readiness-tier projection): for the Trellis verifier, a collision whose differing bytes are confined to `last_verified_at` and/or `verified_by` (with all other fields byte-identical to the prior emission) does NOT fail `integrity_verified`; it surfaces as `iri_collision = true` with a `collision_class = "freshness_update"` localizer in the report entry so the consumer-domain layer can resolve.
  - **Structural collision**: differing bytes outside the freshness fields (e.g., `severity` changed, `subject` changed, `finding_kind` changed) is a hard collision. The Trellis verifier flags `rkaf_finding_iri_collision` with `collision_class = "structural"` AND fails `integrity_verified` — a `rkaf:Finding` IRI is intended to identify one detection; structurally-disagreeing re-emissions under the same IRI is producer error or tampering.
- **Mutability.** Findings are append-only at the Trellis layer; freshness re-emissions are the ONE in-IRI mutation admitted (per Plan 7d). A logical supersession ("the detector re-checked at T+1 and now reports a different severity / different subject / a related new detection") MUST use a NEW `finding_iri` and SHOULD use ADR 0066 supersession linkage (`trellis.supersedes-chain-id.v1`) if it logically supersedes a prior Finding about the same subject. Studio's "waived" lifecycle composes via PKAF `Attestation(targetFinding=…)` referencing the Finding IRI, NOT by mutating the Finding event.
- **Lifecycle state outside Trellis.** Whether a Finding is "open" / "waived" / "remediated" is consumer-domain (Studio readiness-tier projection per PKAF ADR-0093). Trellis stores the detection (and admits freshness re-emissions per Plan 7d); the lifecycle is a downstream graph computation over the case ledger.

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
| `tamper/NNN-rkaf-finding-iri-collision-structural` | Two events sharing `finding_iri` with differing bytes outside the freshness fields (e.g., `severity` changed); verifier fails with `rkaf_finding_iri_collision` and `collision_class = "structural"` — IS a global-integrity failure. |
| `append/NNN-rkaf-finding-freshness-reemission` | Two events sharing `finding_iri` with differing bytes confined to `last_verified_at` and/or `verified_by`; verifier flags `rkaf_finding_iri_collision` with `collision_class = "freshness_update"` but does NOT fail `integrity_verified` (Plan 7d re-verification case). |
| `tamper/NNN-rkaf-finding-freshness-precedes-detection` | `last_verified_at < detected_at`; verifier fails with `rkaf_finding_freshness_precedes_detection` (Plan 7d freshness MUST be `>= detected_at`). |
| `tamper/NNN-rkaf-finding-pkaf-version-unknown` | `pkaf_version = "99.99.99"`; verifier fails with `rkaf_finding_pkaf_version_unknown`. |

Slot numbers TBD per the corpus-batching convention in effect at implementation time.

## Cross-references

- **PKAF ADR-0093** — promotes `rkaf:Finding` to first-class primitive; names this Trellis binding as the open cross-stack work item. This ADR closes that item from the Trellis side.
- **PKAF §4.6** — abstract anchoring contract. Trellis is one of several plausible bindings; the framework is honest that VC / COSE_Sign1 / Sigstore / IPFS are equally valid alternative bindings, and adopters MAY use multiple bindings simultaneously.
- **Core §6.7 Extension Registry** — one row addition (table above).
- **Core §19** — adds step 6f (new substep) with the verifier checklist above. Step 6e is already occupied by ADR 0066 cross-chain supersession-graph verification (`trellis-core.md` §19 step 6 supersession block); the next free slot is 6f. Companion update lands with implementation.
- **TR-CORE-178 (new row, to be added to `specs/trellis-requirements-matrix.md`)** — anchors the verifier obligations above and the closed-taxonomy snapshot discipline. `Verification = test-vector` (deferred until the fixture corpus above lands). This ADR's `Status: Proposal — open` is preserved on the matrix row until the Rust binding ships.
- **ADR 0010** — comparator pattern for "external-content-anchored event with structural payload"; differences enumerated under §"Decision" R2.
- **ADR 0008** — comparator pattern for "cross-spec interop addition"; differences enumerated under §"Decision" R1.
- **ADR 0066** — supersession linkage primitive that Findings about the same subject SHOULD compose with.
- **CLAUDE.md (Trellis)** — authority ladder (Rust > CDDL > prose > matrix > Python > archives), no-stubs rule, consumer-crate-boundary discipline.

## Open questions / follow-ons

1. **`?anchorIRI` HTTPS-resolver variant.** **Partially closed:** the URN-form wire-canonical shape `urn:rkaf:anchor:trellis/1:<ledger_scope hex>:<canonical_event_hash hex>[:<checkpoint_tree_size>]` is pinned in §"Anchor IRI resolution" — this is the form bindings MUST emit. **Open:** an HTTPS-resolver variant (e.g., `https://<trellis-deployment>/anchor/v1/<ledger_scope>/<canonical_event_hash>`) for browser-readable Rulespec graphs. Decided in a follow-on RFC alongside the broader `source_ref` resolution question raised in ADR 0008 Open Q5. The wire form is benign for verification (the verifier resolves Findings by canonical_event_hash, not by URI); the HTTPS variant matters for human-readable Rulespec graph rendering.
2. **Detector-principal mapping at the Posture Declaration.** §"Verifier obligations" step 5 surfaces detector↔principal consistency as advisory. The companion-spec change (Posture Declaration field declaring `authorized_rkaf_detectors: [{ trellis_principal, detector_iri }]`) lands when the first Rulespec-aware deployment configures one — likely co-landed with the `trellis-verify-wos`-style resolver for Findings. The ADR does not pre-commit the Posture Declaration shape; it pre-commits the verifier API surface (`detector_principal_consistent` is nullable, populated by an optional resolver).
3. **Rust implementation trigger.** The first concrete adopter (Studio compiler emitting Findings, `wos-server` validation event emission, or a standalone BVR runtime) triggers `trellis-rkaf` crate creation + fixture corpus + Python stranger mirror. The CLAUDE.md "no stubs" rule forbids creating the crate empty.
4. **Studio readiness-tier projection.** PKAF ADR-0093 names this as a Studio-side concern: Findings + their PKAF Attestation waivers compose into a readiness tier per case. Trellis's responsibility ends at "Findings are anchored, addressable, and verifiable offline"; readiness projection is over the case ledger graph, not in the wire shape. Documented here to foreclose scope creep into Trellis.
5. **PKAF version-snapshot governance.** Open question: who owns the Trellis-side `pkaf_version` → taxonomy-snapshot table? Two options: (a) Trellis maintains the snapshot table and bumps it on each PKAF release (loose coupling — Trellis can lag); (b) PKAF emits a machine-readable taxonomy descriptor per release and Trellis consumes it (tight coupling — bumps are mechanical). Decision deferred; the wire shape is identical either way. Probably (b) once PKAF's release cadence stabilizes (Phase 3 framework + finalized anchoring contract per the PKAF roadmap).

---

*End of ADR 0012.*
