---
title: Trellis Companion — Shared Ledger Binding
version: 0.1.0-draft.2
date: 2026-04-14
status: draft
---

# Trellis Companion — Shared Ledger Binding v0.1

**Version:** 0.1.0-draft.2
**Date:** 2026-04-14
**Editors:** Formspec Working Group
**Companion to:** Trellis Core v0.1

---

## Status of This Document

This document is a **draft companion specification** to Trellis Core v0.1 (hereafter "Core"). It is subordinate to Core and is governed by the profile and binding subordination rule of Core §2.3 and the ontology discipline of Core §5.2: this binding MUST NOT redefine canonical truth, canonical order, canonical append attestation semantics, the canonical hash construction, or verification boundaries established by Core. Where this binding places additional constraints on conforming implementations, it does so only through mechanisms that remain consistent with Core.

This document defines the shared ledger binding: the wire-level, admission-level, and verifier-facing obligations for admitting Formspec-family, WOS-family, trust-family, and release-family facts into a single Trellis canonical substrate.

Implementors are encouraged to experiment with this specification and provide feedback, but MUST NOT treat it as stable for production use until a 1.0.0 release is published.

## Conventions and Terminology

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in BCP 14 [RFC 2119] [RFC 8174] when, and only when, they appear in ALL CAPITALS, as shown here.

JSON syntax and data types are as defined in [RFC 8259]. JSON Canonicalization Scheme (JCS) is defined in [RFC 8785]. SHA-256 is defined in [FIPS 180-4]. URI syntax is as defined in [RFC 3986].

### Requirement Classes

Normative sections in this document declare a **Requirement class** marker. The classes used are those of the companion framework:

- **Binding or reference choice** — concrete serializations, proof encodings, field shapes, and technology mappings that this binding commits to in order to make Core operational.
- **Companion requirement** — subordinate, reusable semantic constraints that refine Core without redefining it.
- **Non-normative guidance** — advisory text. Appears only in the Annex.

## Abstract

The Shared Ledger Binding defines how Formspec-family, WOS-family, trust-family, and release-family facts are admitted into a single Trellis canonical substrate. It specifies:

- the canonical record envelope and its deterministic serialization,
- the append-attestation proof model and the verifier-facing inclusion and consistency proofs,
- idempotency identity and verifier-visible retry semantics,
- the four-layer separation among author-originated fact, protected payload, access/key-wrapping material, and canonical append attestation,
- family admission paths, minimum fields, schema/version compatibility, and rejection semantics,
- cross-family reference rules,
- construction-ID-based algorithm agility,
- registries that underwrite all of the above,
- conformance obligations for all five Core roles.

This companion does not define Formspec or WOS semantics — it governs admission, order, attestation, and verification shape for bound records only.

---

## Table of Contents

- S1. Introduction
- S2. Conformance
- S3. Terminology
- S4. Substrate Binding and Delegation
- S5. Family Binding Matrix
- S6. Canonical Record Envelope and Serialization
- S7. Append-Attestation Proof Model
- S8. Idempotency Identity and Retry Semantics
- S9. Protected Payload and Access Material
- S10. Canonization Rules and Rejection Codes
- S11. Schema and Version Compatibility
- S12. Cross-Family Reference Rules
- S13. Family Admission Paths
- S14. Canonical Receipt Immutability
- S15. Versioning and Algorithm Agility
- S16. Registries
- S17. Security and Privacy Considerations
- S18. Cross-References to Companion Specifications
- Annex A (Non-Normative). Implementation Guidance

---

## S1. Introduction

### S1.1 Scope

This binding instantiates Core §7 (Fact Admission, Canonicalization, and Order) and Core §8 (Canonical Hash Construction) into a concrete wire format, an append-attestation proof model, idempotency identity, and a family-indexed admission matrix. It also fixes the minimum-field contract for the four families (Formspec, WOS, trust, release) admitted into the Trellis canonical substrate.

This binding does **not** redefine or restrict:

- Formspec Definition, Response, FEL, validation, or processing-model semantics (Formspec Core S1–S7);
- WOS kernel lifecycle, case state, governance, or runtime semantics (WOS Kernel S3–S8; WOS Runtime S4–S12);
- Core canonical truth, canonical order, canonical hash construction, or verification semantics (Core §6–§9).

### S1.2 Subordination Statement

**Requirement class: Companion requirement.**

Per Core §2.3 (Profile and Binding Subordination) and Core §5.2 (Ontology Discipline), this binding is subordinate to Core. Nothing in this binding may be interpreted to:

- alter the definition of canonical truth (Core §6.1),
- create a second canonical order for any governed scope (Core §6.2 invariant 3),
- redefine the canonical event hash construction (Core §6.2 invariant 4, Core §8),
- weaken independent verification requirements (Core §9), or
- redefine cross-repository authority (Core §10).

Where this binding appears to conflict with Core, Core prevails.

### S1.3 Design Goal

Provide one verifier-facing admission and proof model that works uniformly across Formspec, WOS, trust, and release facts, so that verifiers do not need to reconcile family-specific append semantics for the same canonical scope (Companion §3.3.2).

---

## S2. Conformance

### S2.1 Conformance Roles

This binding imposes obligations on all five Core conformance roles (Core §2.1):

1. **Fact Producer** (Core §2.5.1) — binding producer obligations in S2.2.1.
2. **Canonical Append Service** (Core §2.5.2) — binding append-service obligations in S2.2.2.
3. **Verifier** (Core §2.5.3) — binding verifier obligations in S2.2.3.
4. **Derived Processor** (Core §2.5.4) — binding derived-processor obligations in S2.2.4.
5. **Export Generator** (Core §2.5.5) — binding export-generator obligations in S2.2.5.

A conforming implementation MUST satisfy all binding obligations for each Core role it claims.

### S2.2 Role Obligations

#### S2.2.1 Fact Producer

**Requirement class: Companion requirement.**

A conforming Fact Producer MUST:

- emit each author-originated fact in a form that can be mapped to a canonical record envelope (S6) satisfying minimum-field obligations for its declared family (S5);
- include a `construction_id` (S15) on the payload digest that it binds into the fact;
- preserve the four-layer separation of S9 when the fact carries protected payload or access material;
- refrain from rewriting previously emitted facts (Core §2.5.1); corrections MUST be new facts linked by cross-family reference (S12).

#### S2.2.2 Canonical Append Service

**Requirement class: Companion requirement.**

A conforming Canonical Append Service MUST:

- enforce append-only canonical order per governed scope (Core §6.2 invariants 1 and 3, Core §7.4);
- produce canonical record envelopes (S6) deterministically serialized under the registered `construction_id` (S15);
- compute the canonical event hash under the registered construction (Core §8);
- issue append attestations carrying the inclusion and consistency proof material defined in S7;
- enforce idempotency per S8 so that retries and replays resolve deterministically to the same canonical record reference or declared no-op outcome;
- reject non-admissible submissions with a rejection code drawn from the registry in S10.2;
- **not** decrypt protected payloads, evaluate workflow policy, or inspect protected content unless the active Trust Profile explicitly permits it (Companion §3.3).

#### S2.2.3 Verifier

**Requirement class: Companion requirement.**

A conforming Verifier MUST, without recourse to derived runtime state:

- validate canonical record envelope integrity against the bound `construction_id` (S6, S15);
- validate append-attestation integrity (S7.2) and inclusion in the append head at a declared head position (S7.3);
- validate consistency between two append heads over the same governed scope (S7.4);
- detect and report construction-ID mismatch (S15.3, rejection code `hash_construction_mismatch`);
- resolve cross-family references structurally (S12) without requiring family runtime semantics;
- treat a missing registry entry for a referenced `construction_id`, `fact_kind`, `custody_mode`, or `rejection_code` as a verification failure, not a silent pass (S16).

#### S2.2.4 Derived Processor

**Requirement class: Companion requirement.**

A conforming Derived Processor MUST:

- treat canonical records as authoritative input (Core §2.5.4);
- rebuild deterministically from the canonical append order, without relying on operational receipt time or in-memory runtime state;
- preserve provenance distinctions (S9) when re-presenting canonical facts in derived views;
- **not** reinterpret a canonical record under a different `construction_id` than the one it was admitted under (S15.4).

#### S2.2.5 Export Generator

**Requirement class: Companion requirement.**

A conforming Export Generator MUST, per Core §2.5.5 and in coordination with the Export Verification Package companion (see S18):

- include, for each exported canonical record, the envelope and append-attestation material sufficient for a Verifier (S2.2.3) to revalidate inclusion and consistency without contacting the originating append service;
- embed or immutably reference the `construction_id` and registry entries (S16) needed to interpret every exported record;
- preserve the four-layer separation of S9 in the export artifact — protected payload and access material MUST remain separable from the author-originated fact and append attestation.

---

## S3. Terminology

Terms defined in Core §4 apply. This binding additionally defines:

### S3.1 Canonical Record Envelope

The JSON object defined in S6.1 that carries the author-originated fact, family identification, schema pinning, construction identifier, idempotency identity, and family-required minimum fields. The envelope is the pre-serialization input to the canonical event hash (Core §8).

### S3.2 Append Head

A verifier-addressable commitment to the state of a canonical append-attested order at a specific position. An append head is identified by a governed-scope identifier and a monotonically increasing tree-size integer (S7.3).

### S3.3 Inclusion Proof

Integrity-verifiable evidence that a specific canonical record is part of the canonical order at a specific position under a specific append head (S7.3).

### S3.4 Consistency Proof

Integrity-verifiable evidence that one append head is a monotonic extension of an earlier append head over the same governed scope (S7.4).

### S3.5 Construction ID

A registered identifier (S16.2) naming the exact serialization, digest, and append-attestation construction used for a canonical record. Baseline Trellis binds `construction_id = "trellis-jcs-sha256-v1"` to JCS + SHA-256 per Core §8.

### S3.6 Idempotency Identity

The pair (`governed_scope`, `idempotency_key`) that determines whether a submission is a new admission, an idempotent retry of a prior admission, or a declared no-op (S8).

### S3.7 Custody Mode

A registered identifier (S16.3) naming the access posture under which a protected payload is custodied. Custody modes recognized by this binding are enumerated in S9.3.

---

## S4. Substrate Binding and Delegation

### S4.1 One Substrate per Governed Scope

**Requirement class: Companion requirement.**

This binding MUST admit Formspec-family, WOS-family, trust-family, and release-family facts into **one** governed canonical substrate per governed scope, using the shared append, hash, and verification rules of Core §6–§9 as instantiated by this binding.

### S4.2 Non-Reinterpretation

**Requirement class: Companion requirement.**

This binding MUST NOT reinterpret Formspec or WOS semantic authority (Core §10). Family payloads remain authoritative in their source repositories. Trellis governs admission, order, attestation, and verification shape for bound records only.

### S4.3 Delegation Requirement

**Requirement class: Companion requirement.**

When binding behavior depends on Formspec Definition or Response semantics — including field values, relevance, validation, calculation, or version pinning — processing MUST be delegated to a Formspec-conformant processor per [Formspec Core §6 Version Pinning (VP-01, VP-02)] and [Formspec Core §1.3 Scope]. This binding defines admission, order, attestation, and verification shape for bound records; it does not specify bind, FEL, or validation rules.

Trellis-bound Formspec processors MUST implement at least Formspec Core conformance per [Formspec Core §1.4 Conformance]. Whether Theme or Component tiers are required depends on the Trellis conformance class and the bound family's declared requirements (Core §1.2).

When binding behavior depends on WOS kernel or runtime semantics, processing MUST be delegated to a WOS-conformant processor per [WOS Kernel §2 Conformance Classes] and [WOS Kernel §2.1].

---

## S5. Family Binding Matrix

**Requirement class: Binding or reference choice.**

The following minimum-field contract is normative. Every canonical record envelope (S6) admitted under this binding MUST carry, at minimum, the fields listed for its declared family. Profiles MAY require additional fields; they MUST NOT remove any field listed here. Field types are JSON strings unless stated otherwise.

| `family_id` | Authority | Required minimum fields | Notes |
|---|---|---|---|
| `formspec.authored` | Formspec Core S4–S7 | `family_id`, `schema_ref`, `authored_at` (RFC 3339), `author_ref`, `payload_ref` | Trellis binds; does not redefine authored semantics (Core §10). |
| `wos.governance` | WOS Kernel S3–S8; WOS Runtime S4–S12 | `family_id`, `schema_ref`, `governance_scope`, `actor_ref`, `payload_ref` | Runtime meaning remains in WOS. |
| `trellis.trust` | Trust Profiles S3 | `family_id`, `profile_ref`, `policy_ref`, `effective_at` (RFC 3339), `subject_ref` | MUST align with trust-profile declarations. |
| `trellis.release` | Export Verification Package S3 | `family_id`, `package_ref`, `audience_ref`, `readability_ref`, `version_ref` | Used for verifier/export semantics. |

**Profile-overridable:** Profiles MAY extend this matrix with additional `family_id` values (e.g., `trellis.lifecycle` for retention/hold/seal facts). Such extensions MUST register their `family_id` per S16.1 and MUST declare their minimum-field contract in the profile.

---

## S6. Canonical Record Envelope and Serialization

### S6.1 Envelope Shape

**Requirement class: Binding or reference choice.**

A canonical record envelope is a JSON object ([RFC 8259]) with the following fields. Fields marked REQUIRED MUST be present; fields marked OPTIONAL MAY be present; no other top-level fields are defined by this binding, and profiles MAY extend under the `extensions` field only.

```json
{
  "construction_id": "trellis-jcs-sha256-v1",
  "governed_scope": "<scope-identifier>",
  "family_id": "formspec.authored",
  "schema_ref": "<immutable schema reference>",
  "idempotency_key": "<binding-defined identity; see S8>",
  "authored_at": "2026-04-14T12:00:00Z",
  "author_ref": "<authoring principal reference>",
  "payload_ref": "<content-addressed payload reference>",
  "payload_digest": {
    "construction_id": "trellis-jcs-sha256-v1",
    "digest": "<lowercase hex SHA-256 over JCS-serialized payload>"
  },
  "access_material_ref": "<optional; see S9>",
  "custody_mode": "<optional; see S9.3>",
  "references": [ { "family_id": "...", "record_ref": "..." } ],
  "extensions": { }
}
```

REQUIRED for all families: `construction_id`, `governed_scope`, `family_id`, `schema_ref`, `idempotency_key`, `payload_ref`, `payload_digest`.

REQUIRED additionally by family: the family-specific minimum fields of S5.

OPTIONAL: `access_material_ref`, `custody_mode`, `references`, `extensions`.

### S6.2 Deterministic Serialization

**Requirement class: Binding or reference choice.**

The canonical serialization of the envelope for hashing (Core §8) and for transmission between binding-conformant systems MUST be [RFC 8785] JSON Canonicalization Scheme (JCS) applied to the envelope as written in S6.1.

### S6.3 Canonical Event Hash

**Requirement class: Binding or reference choice.**

Under baseline `construction_id = "trellis-jcs-sha256-v1"`, the canonical event hash over an envelope is:

```
canonical_event_hash = SHA-256( JCS(envelope) )
```

represented as a lowercase hex string. This is the instantiation of Core §8 for this binding. Future constructions MUST be registered per S16.2 before verifiers are required to accept them (S15.3).

### S6.4 Envelope Integrity Under Profiles

**Requirement class: Companion requirement.**

Profiles MAY add envelope fields under `extensions`. A profile MUST NOT introduce envelope fields that:

- mutate meaning across time (the envelope is immutable once hashed, per Core §6.2 invariant 1);
- bypass the `construction_id` (every envelope carries exactly one);
- create alternate paths for the idempotency identity of S8.

---

## S7. Append-Attestation Proof Model

### S7.1 One Verifier-Facing Proof Model per Governed Scope

**Requirement class: Companion requirement.**

Per Companion §3.3.2, a conforming implementation MUST present exactly one verifier-facing canonical append proof model per declared governed scope at a time. The default proof model defined by this binding is the **transparency-log-style append model** (S7.2). Profiles MAY register alternate proof models per S16 and S15.3, but MUST NOT require verifiers to reconcile multiple overlapping proof models for the same governed scope.

### S7.2 Default: Transparency-Log-Style Append Model

**Requirement class: Binding or reference choice.**

The default proof model is a Merkle tree over canonical event hashes (S6.3) in append order. The append head (S3.2) commits to the state of the tree at a given size.

Append head shape:

```json
{
  "construction_id": "trellis-jcs-sha256-v1",
  "governed_scope": "<scope-identifier>",
  "tree_size": 12345,
  "root_hash": "<lowercase hex; Merkle root under construction_id>",
  "issued_at": "2026-04-14T12:00:01Z",
  "signature": { "key_ref": "...", "value": "..." }
}
```

The Merkle tree construction under `trellis-jcs-sha256-v1` is RFC 6962-compatible: leaf hash = `SHA-256(0x00 || canonical_event_hash)`, interior hash = `SHA-256(0x01 || left || right)`, with odd-node carry-up.

**Profile-overridable:** `signature` shape, key-binding, and witness-cosignatures are profile choices. Monitoring and witnessing semantics are defined in the Monitoring and Witnessing companion (see S18).

### S7.3 Inclusion Proof

**Requirement class: Binding or reference choice.**

Under the default proof model, an inclusion proof for a canonical record at position `leaf_index` under an append head with `tree_size = N` is:

```json
{
  "governed_scope": "<scope-identifier>",
  "construction_id": "trellis-jcs-sha256-v1",
  "leaf_index": 42,
  "tree_size": 12345,
  "leaf_hash": "<canonical_event_hash>",
  "audit_path": [ "<hex>", "<hex>", "..." ]
}
```

A Verifier (S2.2.3) MUST recompute the Merkle root from `leaf_hash`, `leaf_index`, and `audit_path`, and MUST confirm that it equals the `root_hash` of the append head at `tree_size = N`.

### S7.4 Consistency Proof

**Requirement class: Binding or reference choice.**

Under the default proof model, a consistency proof between two append heads `H_m` (with `tree_size = m`) and `H_n` (with `tree_size = n`, `m < n`) over the same governed scope is:

```json
{
  "governed_scope": "<scope-identifier>",
  "construction_id": "trellis-jcs-sha256-v1",
  "from_tree_size": 1000,
  "to_tree_size": 12345,
  "proof_path": [ "<hex>", "..." ]
}
```

A Verifier MUST, using `proof_path`, recompute the root at `from_tree_size` and the root at `to_tree_size` and MUST confirm both match their respective append heads.

An append service MUST NOT publish an append head at `tree_size = n` whose consistency proof against any previously published head at `tree_size = m < n` would fail; doing so is an append-only violation under Core §6.2 invariant 1.

### S7.5 Proof Portability

**Requirement class: Companion requirement.**

Every inclusion and consistency proof MUST carry `governed_scope` and `construction_id`. A Verifier receiving a proof with a `construction_id` not present in the registry (S16.2) MUST reject the proof (see `hash_construction_mismatch`, S10.2).

---

## S8. Idempotency Identity and Retry Semantics

### S8.1 Idempotency Key

**Requirement class: Binding or reference choice.**

Every envelope MUST carry an `idempotency_key` field. The idempotency identity for a submission is the pair (`governed_scope`, `idempotency_key`).

A conforming Fact Producer SHOULD generate `idempotency_key` as a stable function of the author-originated fact's causal submission identity (Companion §3.3.1), such that equivalent authored submissions produce equal keys. The exact function is profile-defined.

### S8.2 Retry Resolution

**Requirement class: Companion requirement.**

Per Companion §3.3.1 and Core §6.2 invariant 6, for a given idempotency identity within a declared governed scope, a Canonical Append Service (S2.2.2) MUST resolve every successful retry to exactly one of:

1. the same canonical record reference that was admitted on the first successful submission (idempotent admission), or
2. the same declared no-op outcome (idempotent no-op).

The service MUST NOT, on retry, create a new canonical order position with a different canonical event hash under the same idempotency identity.

### S8.3 Verifier-Visible Semantics

**Requirement class: Companion requirement.**

If a submission is resolved as an idempotent no-op, the service MUST return a reference resolving to the prior canonical record (if one was admitted) or to an explicit no-op outcome declaration (if none was admitted). A Verifier MUST be able to distinguish the three outcomes — newly admitted, resolved to prior admission, explicit no-op — from the response without appeal to private service state.

### S8.4 Idempotency-Key Mismatch

**Requirement class: Binding or reference choice.**

If two envelopes share an idempotency identity but would hash to different canonical event hashes under the same `construction_id`, the service MUST reject the second submission with rejection code `idempotency_conflict` (S10.2). This prevents silent reinterpretation of a prior admission.

---

## S9. Protected Payload and Access Material

### S9.1 Four-Layer Separation

**Requirement class: Companion requirement.**

Per Companion §3.6, a canonical record MUST preserve the semantic distinction among four layers:

1. **Author-originated fact** — the content the authoring principal committed to, identified by `author_ref` and (for digest integrity) `payload_digest`.
2. **Protected payload** — sensitive content whose plaintext is protected per the active Trust Profile, referenced by `payload_ref`; the envelope carries only the digest and a (possibly opaque) pointer.
3. **Access or key-wrapping material** — material an authorized principal uses to obtain plaintext, referenced by `access_material_ref`; never inlined into the envelope in plaintext form.
4. **Canonical append attestation** — the append head, inclusion proof, and consistency proof material of S7.

### S9.2 Separation Obligations

**Requirement class: Binding or reference choice.**

- The envelope (S6.1) MUST NOT inline protected-payload plaintext.
- The envelope MAY carry `access_material_ref` (an identifier or URI) but MUST NOT inline key material that would grant provider-readable access when the active Trust Profile declares reader-held or delegated-compute custody (see S18 → Trust Profiles).
- The append attestation (S7) MUST NOT depend on access-material state; attestation remains verifiable whether or not any given Verifier can decrypt the payload (Core §9).

### S9.3 Custody Mode Field

**Requirement class: Binding or reference choice.**

When `custody_mode` is present in an envelope, its value MUST be a registered identifier drawn from the Custody Modes registry (S16.3). The wire-level requirement is that `custody_mode` is a JSON string matching a registered identifier; verifiers MUST reject envelopes carrying an unregistered `custody_mode` (S16, S10.2 `custody_mode_inconsistent`).

Custody mode semantics — the operator-readable, reader-held, threshold, and recovery postures — are defined and operationalized for Trellis distributed deployments in `trellis/specs/trust/trust-profiles.md` §2 Object Shape. This binding does not enumerate or redefine those semantics; it fixes only the wire field, the registry binding (S16.3), and the admission-time consistency check against the active Trust Profile (S13.3).

**Profile-overridable:** the exact set of recognized custody modes is profile-defined via the registry (S16.3); the Trust Profiles companion (see S18) is authoritative for custody-mode semantics and Trust Profile alignment.

---

## S10. Canonization Rules and Rejection Codes

### S10.1 Canonization Rules

**Requirement class: Binding or reference choice.**

A Canonical Append Service MUST:

1. reject records missing required minimum fields for the declared family (S5);
2. carry stable schema/version references (`schema_ref`) for verification portability (S11);
3. preserve the canonical hash construction bound by `construction_id` without family-specific overrides (Core §8, S15);
4. reject records whose computed canonical event hash does not match the registered construction (`hash_construction_mismatch`);
5. refuse to admit a record that would create competing canonical order for the same governed scope (Core §6.2 invariant 3).

### S10.2 Rejection Codes

**Requirement class: Binding or reference choice.**

The following rejection codes are normative and are registered per S16.5. Implementations MUST emit exactly one code per rejected submission. Additional codes MAY be registered by profiles.

| Code | Meaning | Typical remediation | Safe to retry? |
|---|---|---|---|
| `missing_required_field` | Record omits a field required for the declared family (S5) or the envelope (S6.1). | Add the missing field and resubmit. | Yes |
| `invalid_schema_ref` | `schema_ref` does not resolve, does not match payload, or references an unknown schema. | Correct the schema reference or update the payload. | Yes |
| `unsupported_major_version` | Major schema version not recognized by this append service (S11). | Upgrade to a supported version or declare a compatibility adapter. | No (requires adapter) |
| `hash_construction_mismatch` | Canonical hash construction does not match the registered `construction_id` for this governed scope (Core §8, S15). | Use the registered construction. | No (construction is fixed per scope) |
| `scope_order_conflict` | Record would create competing canonical order for the same governed scope (Core §6.2 invariant 3). | Verify scope assignment; resubmit to correct scope. | Yes (if scope was wrong) |
| `idempotency_conflict` | Envelope shares idempotency identity with a prior admission but hashes differently under the same `construction_id` (S8.4). | Use a distinct idempotency key, or resolve the authored fact so that retries are byte-equivalent under JCS. | No |
| `admission_prerequisite_unmet` | Binding-declared admission prerequisite (e.g., causal dependency, delegation fact) is not yet admitted (Core §7.1). | Admit the prerequisite fact first, then retry. | Yes |
| `custody_mode_inconsistent` | Declared `custody_mode` or `access_material_ref` is inconsistent with the active Trust Profile (S9, S18). | Align the envelope with the active Trust Profile. | Yes (after alignment) |

Rejections MUST be explicit and auditable (Core §7.6).

---

## S11. Schema and Version Compatibility

**Requirement class: Binding or reference choice.**

Breaking-vs-additive classification applied to `schema_ref` follows [Formspec Changelog §4 Impact Classification]. This binding does not redefine that classification; it applies it to the `schema_ref` field on the canonical record envelope (S6.1).

1. Backward-compatible additive fields MAY be accepted within a declared major version, per [Formspec Changelog §4 Impact Classification].
2. Breaking semantic changes MUST require a new major `schema_ref`, per [Formspec Changelog §4 Impact Classification].
3. Verifiers MUST reject unknown major versions unless an explicit compatibility adapter is declared and registered (rejection code `unsupported_major_version`).

**Profile-overridable:** Profiles MAY tighten these rules (e.g., forbid additive-field acceptance for specific families) but MUST NOT loosen them.

---

## S12. Cross-Family Reference Rules

**Requirement class: Binding or reference choice.**

1. **Allowed edges.** Authored facts (Formspec) MAY reference governance facts (WOS) via the envelope `references` array; governance facts MAY reference authored facts via the same mechanism. Trust and release facts MAY reference any admitted fact. Each entry in `references` MUST carry the target's `family_id` and a canonical `record_ref`.
2. **Forbidden cycles.** A canonical record MUST NOT transitively reference itself through cross-family links. Producers SHOULD detect cycles at emission; services MUST detect and reject cycles detectable at admission time.
3. **Reference shape.** `references[i].record_ref` MUST use the target family's canonical record identifier format (derived from the envelope's canonical event hash, S6.3, unless a profile registers an alternate form).
4. **Resolution failure.** Failure to resolve a cross-family reference during admission MUST produce `invalid_schema_ref` or `admission_prerequisite_unmet`. It MUST NOT silently omit the reference.

---

## S13. Family Admission Paths

### S13.1 Formspec-Authored Fact Admission

**Requirement class: Binding or reference choice.**

1. Ingest a Formspec Response or Definition reference submitted as a `formspec.authored` fact.
2. Validate the reference against the pinned Definition version per [Formspec Core §6 Version Pinning VP-01] (Response is always validated against its pinned Definition version, never against a newer version). If the reference cites a Definition version not recognized by the Formspec-conformant processor, reject with `invalid_schema_ref`.
3. Delegate Definition and Response validation to a Formspec-conformant processor per [Formspec Core §1.3 Scope] and [Formspec Core §6 VP-01]. If validation fails, reject with `invalid_schema_ref`.
4. Map the validated reference to a canonical record envelope per S5 and S6.
5. Apply canonization rules (S10.1) and, if admissible, append to canonical order and issue an append attestation (S7).

### S13.2 WOS Governance Fact Admission

**Requirement class: Binding or reference choice.**

1. Ingest a WOS governance/workflow fact submitted as a `wos.governance` fact.
2. Validate the fact against the declared WOS schema version (WOS Kernel S3; WOS Runtime S3). If the version is not recognized, reject with `unsupported_major_version`.
3. Verify structural conformance of `governance_scope` and `actor_ref` (WOS Kernel S4; WOS Governance S2). Trellis does not evaluate WOS governance rules; it checks that required fields are present and structurally valid.
4. Map the validated fact to a canonical record envelope per S5 and S6.
5. Apply canonization rules (S10.1) and, if admissible, append to canonical order.

### S13.3 Trust Fact Admission

**Requirement class: Binding or reference choice.**

1. Ingest a `trellis.trust` fact.
2. Validate `profile_ref` against the active Trust Profile declaration (Trust Profiles S3). If the profile is not recognized, reject with `invalid_schema_ref`.
3. Validate that any declared `custody_mode` and key-lifecycle references align with the Trust Profile (S9.3; Trust Profiles S4; Key Lifecycle Operating Model S3). Misalignment yields `custody_mode_inconsistent`.
4. Map and admit per S6 and S10.

### S13.4 Release Fact Admission

**Requirement class: Binding or reference choice.**

1. Ingest a `trellis.release` fact.
2. Validate `package_ref`, `audience_ref`, and `readability_ref` against the Export Verification Package specification (Export Verification Package S3).
3. Map and admit per S6 and S10.

---

## S14. Canonical Receipt Immutability

**Requirement class: Companion requirement.**

Binding-defined fields on the canonical append attestation (S7) — including ingest-time verification posture, payload-readiness class, or other admission-time commitments — are part of canonical truth for that append position (Core §6.1).

Such fields MUST NOT be rewritten, upgraded, or downgraded in place after the append attestation is issued (Core §6.2 invariant 1). Changes to verification or readability posture MUST be represented as new canonical facts or attestations per S12 and S13, not by mutating prior receipt fields.

---

## S15. Versioning and Algorithm Agility

### S15.1 Construction ID on Every Record

**Requirement class: Binding or reference choice.**

Every canonical record envelope (S6.1) and every append attestation, inclusion proof, and consistency proof (S7) MUST carry a `construction_id` identifying the exact serialization, digest, and append-attestation construction in force when the record was admitted.

### S15.2 Self-Contained Interpretation Material

**Requirement class: Companion requirement.**

Per Companion Appendix C, a conforming implementation MUST preserve enough immutable interpretation material — construction specifications, schema references, algorithm parameters — to verify historical records without live registry lookups, mutable references, or out-of-band operator knowledge. Export artifacts MUST embed or immutably reference (by content-addressed digest) the registry entries needed to interpret every exported record.

### S15.3 No Silent Reinterpretation

**Requirement class: Companion requirement.**

An implementation MUST NOT reinterpret a historical canonical record under a `construction_id` other than the one under which it was admitted. Migration to a new `construction_id` for a governed scope MUST be represented by:

1. registering the new construction (S16.2) before any record is admitted under it;
2. declaring an explicit migration boundary (Companion §3.3.2) — a canonical fact on the prior order that marks the transition;
3. starting a new append head under the new construction, with consistency between old and new heads demonstrated by profile-declared migration procedure (not by a cross-construction Merkle consistency proof, which is ill-defined).

Verifiers encountering a record whose `construction_id` is not registered MUST reject with `hash_construction_mismatch`.

### S15.4 Construction Fixity per Scope

**Requirement class: Binding or reference choice.**

Core §6.2 invariant 4 ("One Canonical Event Hash Construction") is instantiated as: exactly one `construction_id` is in force for a given governed scope at a given point in its append order. Two records at the same append position MUST carry the same `construction_id`.

---

## S16. Registries

**Requirement class: Companion requirement.**

A conforming deployment MUST maintain versioned registries for the identifiers used by this binding. Until a dedicated registry companion is published, the following in-spec registries apply at minimum. Each registry is a content-addressed, append-only list of entries; each entry carries a registration date and a stable identifier.

### S16.1 Family IDs

Registers `family_id` values admissible under this binding. Initial entries:

- `formspec.authored`
- `wos.governance`
- `trellis.trust`
- `trellis.release`

Profiles MAY register additional family IDs (e.g., `trellis.lifecycle`).

### S16.2 Construction IDs

Registers `construction_id` values. Initial entry (the baseline mandated by Core §8):

- `trellis-jcs-sha256-v1` — JCS ([RFC 8785]) serialization, SHA-256 digest, RFC 6962-compatible Merkle tree with domain-separated leaf and interior hashes (S7.2).

Future constructions (e.g., a post-quantum digest replacement) MUST be registered before verifiers are required to accept them (S15.3).

### S16.3 Custody Modes

Registers `custody_mode` values. Initial entries: `provider-readable`, `reader-held`, `reader-held-with-recovery`, `threshold` (S9.3). The Trust Profiles companion (see S18) is authoritative for custody-mode semantics.

### S16.4 Lifecycle Fact Kinds

Registers recognized lifecycle fact kinds for use in `trellis.lifecycle` (or equivalent profile-registered) families. Recognized kinds (Companion §3.8):

- `retention`, `legal_hold`, `archival`, `key_destruction`, `sealing`, `schema_upgrade`, `export_issuance`.

An implementation MAY support a subset; any supported kind MUST be represented as a canonical fact if it affects compliance or recoverability posture.

### S16.5 Rejection Codes

Registers the rejection codes enumerated in S10.2. The code `hash_construction_mismatch` referenced in Core §8 is registered here (moved from Core for operationalization). Additional codes MAY be registered by profiles; code identifiers are immutable once registered.

### S16.6 Registry Discipline

All registries MUST be:

- append-only and content-addressed;
- embeddable or immutably referenceable in export artifacts (S2.2.5, S15.2);
- versioned (Companion Appendix A);
- separable per family where doing so reduces audit blast radius.

---

## S17. Security and Privacy Considerations

This section operationalizes Companion Appendix D (Security Considerations Detail) and Appendix E (Privacy Considerations Detail) for this binding.

### S17.1 Security Considerations

**Requirement class: Companion requirement.**

Implementations MUST consider at least the following:

1. **Key compromise.** Append-service signing keys (S7.2 signature) and Fact Producer signing keys are high-value targets. Key lifecycle MUST follow the Key Lifecycle Operating Model companion (S18). Compromise recovery MUST NOT rewrite prior canonical records (Core §6.2 invariant 1).
2. **Verifier or parser divergence.** Divergent JCS ([RFC 8785]) or digest implementations produce different canonical event hashes from the same logical envelope. Implementations MUST conformance-test against a reference serializer and MUST treat any parser-level inconsistency as `hash_construction_mismatch`.
3. **Metadata leakage.** The envelope (S6.1) is largely in-the-clear for verification. Implementations MUST minimize metadata per Companion §3.9.1: visible metadata SHOULD be limited to what is required for canonical verification, schema lookup, required audit-visible declarations, conflict gating, and append processing.
4. **Replay and reordering attacks.** Idempotency identity (S8) prevents replay from creating duplicate canonical positions. Reordering attacks against a Canonical Append Service are prevented by Core §6.2 invariant 1 and by consistency-proof verification (S7.4). Verifiers MUST revalidate consistency on every append-head change.
5. **Recovery abuse.** Recovery authorities declared under `reader-held-with-recovery` or `threshold` custody (S9.3) MUST emit auditable lifecycle facts (S16.4) when exercised. Silent recovery is prohibited.
6. **Authorization drift between canonical facts and derived evaluators.** Derived evaluators (Companion §3.1.3) MUST be traceable to canonical grant/revocation facts and MUST NOT override canonical semantics. See the Projection Runtime Discipline companion (S18).
7. **Snapshot misuse.** Snapshots (Companion §3.7) are derived artifacts; treating them as canonical truth is a Core §6.2 invariant 2 violation.
8. **Service equivocation.** An append service presenting different append heads for the same governed scope to different verifiers MUST be detectable via the Monitoring and Witnessing companion (S18) consistency-proof cross-checks.
9. **Delayed offline submission edge cases.** The Offline Authoring Profile (Companion §2.1) governs these; idempotency identity (S8) covers replay on reconnection.
10. **Over-broad delegated compute grants.** Delegated compute access (Companion §2.3, §3.2) MUST be scoped and attributable; this binding does not weaken those constraints.
11. **Accidental expansion from delegated compute into standing provider-readable access.** A delegated compute grant MUST NOT be interpreted as reclassifying the envelope's `custody_mode` (S9.3). Custody mode changes MUST be represented as new canonical facts.

Implementations SHOULD test canonical invariants using model checking, replay testing, property-based testing, and protocol fuzzing (Companion Appendix D).

### S17.2 Privacy Considerations

**Requirement class: Companion requirement.**

Payload confidentiality MUST NOT be described as equivalent to metadata privacy (Companion §3.9). Implementations MUST consider:

1. **Visible fact categories.** `family_id`, `schema_ref`, `governed_scope`, and `authored_at` are visible at the envelope layer. Profiles with strong metadata-minimization requirements MAY register coarsened forms under `extensions`, but MUST NOT remove envelope-required fields.
2. **Timing patterns.** `authored_at` (RFC 3339, S5) and append-head `issued_at` (S7.2) leak timing. Profiles MAY round timestamps to a declared granularity.
3. **Access-pattern observability.** Inclusion-proof requests disclose which records a verifier is examining. Implementations SHOULD support batched or oblivious proof retrieval where the Trust Profile requires it.
4. **Disclosure linkability.** Export artifacts (S2.2.5; Export Verification Package companion) MAY link across governed scopes via `references`. Export Generators MUST preserve the four-layer separation of S9 so that unnecessary linkage is not created by export packaging.
5. **User-held record reuse correlation.** When prior user-held records are reintroduced (User-Held Record Reuse Profile, Companion §2.5), the binding MUST bind exactly what was reused and MUST NOT admit bulk user-held state by default.

### S17.3 Subordination of Security Claims

**Requirement class: Companion requirement.**

Legal-sufficiency and disclosure-obligation semantics are defined by [WOS Assurance §6 Legal-Sufficiency Disclosure Obligations]. This binding MUST NOT be interpreted to imply that cryptographic controls alone guarantee admissibility or legal sufficiency in any jurisdiction; evidentiary strength is governed upstream per [WOS Assurance §6]. This binding addresses only ledger-level append-attestation and envelope integrity.

---

## S18. Cross-References to Companion Specifications

### S18.1 Upstream Specifications

This binding is subordinate to and cites the following upstream specifications. Trellis is an implementation of WOS hosting Formspec; these upstreams govern the semantics that Trellis admits, orders, and attests.

- **WOS Kernel** — `wos-spec/specs/kernel/spec.md`. Authoritative for workflow lifecycle, actor model, case state, and conformance classes. This binding cites [WOS Kernel §2 Conformance Classes] for workflow-fact delegation (S4.3), WOS Kernel S3–S8 for `wos.governance` authority (S5, S13.2), and WOS Kernel §2.1 for processor conformance scope.
- **WOS Assurance** — `wos-spec/specs/assurance/assurance.md`. Authoritative for legal-sufficiency disclosure obligations. This binding cites [WOS Assurance §6] for legal-sufficiency disclosure obligations (S17.3). Custody-mode semantics are defined in `trellis/specs/trust/trust-profiles.md` (S9.3).
- **WOS Governance** — `wos-spec/specs/governance/workflow-governance.md`. Authoritative for workflow governance structural requirements. This binding cites it for `governance_scope` and `actor_ref` structural conformance (S13.2).
- **Formspec Core** — `specs/core/spec.md`. Authoritative for Definition, Response, FEL, validation, processing model, and version pinning. This binding cites [Formspec Core §1.3 Scope] and [Formspec Core §6 Version Pinning VP-01, VP-02] for response-fact delegation (S4.3, S13.1) and [Formspec Core §1.4 Conformance] for processor conformance (S4.3).
- **Formspec Changelog** — `specs/registry/changelog-spec.md`. Authoritative for breaking-vs-additive classification. This binding cites [Formspec Changelog §4 Impact Classification] for `schema_ref` version compatibility (S11).
- **Formspec Respondent Ledger** — `specs/audit/respondent-ledger-spec.md`. Authoritative for respondent-facing audit change tracking when Formspec-authored facts carry draft, reopen, or amendment history. This binding does not redefine the respondent-ledger contract; where a `formspec.authored` admission carries respondent-ledger material, that material remains authoritative in the source repository per Core §10.

### S18.2 Trellis Companion Specifications

This binding depends on and is depended upon by the following Trellis companion specifications:

- **Trust Profiles** — `trellis/specs/trust/trust-profiles.md`. Authoritative for Trust Profile declarations, custody-mode semantics, provider/reader/delegated-compute distinctions. This binding cites Trust Profiles S3 for `profile_ref` validation (S13.3) and S4 for custody-mode alignment (S9.3, S13.3).
- **Key Lifecycle Operating Model** — `trellis/specs/trust/key-lifecycle-operating-model.md`. Authoritative for signing-key, wrapping-key, and recovery-key lifecycle. This binding cites it for S17.1(1) (key compromise) and S13.3 (trust-fact lifecycle references).
- **Export Verification Package** — `trellis/specs/export/export-verification-package.md`. Authoritative for release/export artifact structure. This binding cites it for S2.2.5 (Export Generator obligations) and S13.4 (release fact admission). Export artifacts MUST embed binding-registered material per S15.2.
- **Projection Runtime Discipline** — `trellis/specs/projection/projection-runtime-discipline.md`. Authoritative for derived-processor rebuild discipline. This binding cites it for S2.2.4 (Derived Processor obligations) and S17.1(6) (authorization drift).
- **Monitoring and Witnessing** — `trellis/specs/operations/monitoring-witnessing.md`. Authoritative for append-head witnessing and consistency-proof cross-checks. This binding cites it for S7.2 (append-head signature and witness cosignatures) and S17.1(8) (service equivocation detection).
- **Disclosure Manifest** — `trellis/specs/export/disclosure-manifest.md`. Authoritative for disclosure claim classes in exports (Companion §2.4.1).

---

## Annex A (Non-Normative). Implementation Guidance

**Requirement class: Non-normative guidance.**

This annex is non-normative. Nothing in Annex A imposes requirements on conforming implementations.

### A.1 Worked Envelope Example

A minimal `formspec.authored` envelope for a submitted Response might look like:

```json
{
  "construction_id": "trellis-jcs-sha256-v1",
  "governed_scope": "example.org/case/2026-0414-0001",
  "family_id": "formspec.authored",
  "schema_ref": "formspec:definition/example/v2",
  "idempotency_key": "sha256:2f3c...a91b",
  "authored_at": "2026-04-14T12:00:00Z",
  "author_ref": "did:example:alice",
  "payload_ref": "cid:bafy...xyz",
  "payload_digest": {
    "construction_id": "trellis-jcs-sha256-v1",
    "digest": "7d865e959b2466918c9863afca942d0fb89d7c9ac0c99bafc3749504ded97730"
  },
  "custody_mode": "reader-held",
  "access_material_ref": "urn:example:wrap/2026-0414-0001/alice",
  "references": [],
  "extensions": {}
}
```

### A.2 Operational Suggestions

- Generate `idempotency_key` as `sha256` of the canonically serialized authored-fact intent (distinct from `payload_digest`, which covers the protected payload).
- Co-locate registry-entry caches with append-service verifiers so that `construction_id` resolution does not become a live-registry dependency (S15.2).
- Surface rejection codes (S10.2) verbatim at API boundaries; do not synthesize or wrap them, so that retry safety tables are preserved across hops.
- Treat witness cosignatures (S7.2, S18) as the primary defense against service equivocation; do not rely solely on service-signed append heads for high-impact governed scopes.

### A.3 Relation to Recommended Technology Shape

Companion §9.1 suggests a transparency-log-style append service, protected payloads with access material, and verifiable export packaging. This binding's default proof model (S7.2), four-layer separation (S9), and Export Generator obligations (S2.2.5) instantiate that shape without prescribing vendors or wire protocols beyond JCS and SHA-256.
