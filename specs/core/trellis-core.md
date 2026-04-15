---
title: Trellis Core Specification
version: 0.1.0-draft.1
date: 2026-04-14
status: draft
---

# Trellis Core Specification v0.1

**Version:** 0.1.0-draft.1
**Date:** 2026-04-14
**Editors:** Formspec Working Group
**Companion to:** Formspec v1.0, WOS v1.0

---

## Status of This Document

This document is a **draft specification**. It is the foundation layer of the Trellis specification family — a companion framework to Formspec v1.0 and WOS v1.0 that does not modify their processing models. Implementors are encouraged to experiment with this specification and provide feedback, but MUST NOT treat it as stable for production use until a 1.0.0 release is published.

This document is the **semantic constitution** of the Trellis family. It defines invariants, trust semantics, canonical truth, and verification boundaries. It does not define exact implementation mechanisms, deployment stacks, product integrations, or byte-level proof bindings. It is paired with companion specifications for bindings, trust profiles, key lifecycle, projection/runtime discipline, export/disclosure, monitoring/witnessing, and assurance traceability (see §11).

The intended hierarchy is:

- the **core specification** defines what MUST remain true,
- **profiles** define permitted domain-specific interpretations that narrow but do not reinterpret the core,
- **bindings** define how core semantics map onto concrete technologies,
- **sidecars** collect family-specific, deployment-specific, or implementation-adjacent material without altering the core,
- **implementation specifications** define exact bytes, proofs, APIs, and operational procedures.

Every normative subsection of this specification is labeled with a **Requirement class** marker:

- **Constitutional semantic** — invariants that MUST remain stable across all profiles, bindings, and implementations;
- **Profile constraint** — requirements that scope or narrow core semantics for a declared profile;
- **Binding or reference choice** — requirements that fix concrete serializations, encodings, proof formats, or API procedures.

Statements that mix those layers without clear boundaries SHOULD be split or revised.

## Abstract

Trellis Core defines the minimum normative semantics for a **cryptographic append-only ledger** used by Formspec- and WOS-family systems. It is a constitutional specification for canonical record admission, ordering, hashing, attestation, and independently verifiable export — not a workflow, orchestration, or authorization substrate.

This specification normatively defines:

- core ledger object classes,
- canonical truth boundaries,
- fact admission, canonicalization, ordering, and append-attestation semantics,
- conformance roles and profiles for ledger participants,
- the contracts between canonical ledger semantics and replaceable implementations,
- independent verification and export guarantees,
- non-redefinition boundaries relative to Formspec and WOS.

Workflow, authorization, custody, and assurance-level semantics are defined by the host substrate (see [WOS Kernel §10 Named Extension Seams] and [WOS Assurance §2–§6]); this specification cross-references those layers rather than restating them.

This specification does **not** define domain vocabularies, product UX, deployment architecture, concrete serialization bytes, or proof-wire formats. Those belong in profiles, bindings, sidecars, and implementation specifications.

Its purpose is to define what MUST remain true about the ledger even if the surrounding stack changes.

## Table of Contents

1. Introduction
2. Conformance
3. Core-to-Implementation Contracts
4. Terminology
5. Core Model
6. Canonical Truth and Invariants
7. Fact Admission, Canonicalization, and Order
8. Canonical Hash Construction
9. Verification and Export Requirements
10. Cross-Repository Authority Boundaries
11. Companion Specifications
12. Supplementary Constitutional Requirements
13. Security and Privacy Considerations
14. Non-Normative Guidance

---

## 1. Introduction

### 1.1 Scope

Trellis Core governs constitutional semantics that MUST remain stable across implementations and companion specifications.

It defines:

- what kinds of objects exist in the core model,
- what becomes canonical truth,
- how facts become admitted, represented, ordered, and attested,
- what trust claims an implementation MUST declare honestly,
- what export and verification guarantees a conforming implementation MUST preserve,
- the core-to-implementation contracts that separate stable semantics from replaceable mechanisms.

It does not define storage backends, workflow engines, IAM stacks, policy engines, exact serializations, exact proof formats, or domain-specific vocabularies. Those belong in profiles, bindings, sidecars, and implementation specifications.

### 1.2 Relationship to Formspec and WOS

Trellis is a **companion framework** to Formspec v1.0 and WOS v1.0. Trellis adds canonical ledger, trust, and disclosure semantics on top of Formspec and WOS substrates. It does not modify their processing models.

**Additive invariant.** Trellis MUST NOT alter Formspec data capture, validation, or Core processing model semantics (Definition evaluation, Response validation, FEL calculation, relevance, or the four-phase processing cycle). A Formspec processor that ignores all Trellis sidecars, bindings, and artifacts remains fully conformant to Formspec and produces identical data and validation results.

**Delegation requirement.** When Trellis behavior depends on Formspec Definition or Response semantics — including field values, relevance, validation, or calculation — processing MUST be delegated to a Formspec-conformant processor (Core S1.4). Trellis defines admission, order, attestation, and verification shape for bound records; it does not specify bind/FEL/validation rules.

**Formspec conformance tier.** Trellis-bound Formspec processors MUST implement at least Formspec Core conformance (Core S2). Whether Theme or Component tiers are required depends on the Trellis conformance class: Structural and Append Service roles require Core only; roles that present or render Formspec-backed tasks to end users additionally require Component conformance.

**Screener scope.** Trellis does not redefine Screener routing, classification, or determination semantics (Screener S1–S7). When a Trellis-bound deployment uses Formspec Screener evaluation, it MUST delegate to a Formspec-conformant Screener processor. Trellis may bind Screener determination records as canonical facts but MUST NOT alter the Screener evaluation algorithm.

### 1.3 Design Goal

Prevent multi-source-of-truth drift by enforcing one canonical append-attested substrate while allowing replaceable derived systems.

A conventional workflow platform often creates multiple sources of truth: authored facts, workflow or provenance facts, operational workflow state, mutable application database state, and export views assembled after the fact. This specification defines a model in which canonical truth is explicit and narrow, and workflow engines, authorization evaluators, caches, projections, and presentation layers remain replaceable.

---

## 2. Conformance

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in BCP 14 [RFC 2119] [RFC 8174] when, and only when, they appear in ALL CAPITALS, as shown here.

JSON syntax and data types are as defined in [RFC 8259]. URI syntax is as defined in [RFC 3986].

### 2.1 Conformance Roles

An implementation MAY claim one or more of the following roles:

1. **Fact Producer** — creates author-originated facts or other attributable facts admitted under the active profile or binding.
2. **Canonical Append Service** — admits facts, forms canonical records, orders them, and issues canonical append attestations.
3. **Verifier** — verifies authored facts, canonical append attestations, and export scopes.
4. **Derived Processor** — builds derived artifacts from canonical truth.
5. **Export Generator** — assembles disclosure or export artifacts and verification packages.

A conforming implementation MUST satisfy all requirements applicable to each claimed role.

### 2.2 Conformance Profiles

The following generic conformance profiles are defined:

- **Core Profile** (defined in §2.4);
- Profile-level conformance classes defined in the Trust companions (`trellis/specs/trust/trust-profiles.md`): Offline Authoring, Reader-Held Decryption, Delegated Compute, User-Held Record Reuse, and Respondent History;
- **Disclosure and Export Profile** defined in `trellis/specs/export/disclosure-manifest.md` and `trellis/specs/export/export-verification-package.md`.

An implementation MAY claim conformance to one or more profiles.

An implementation claiming conformance to any non-Core Profile MUST also conform to the Core Profile.

### 2.3 Profile and Binding Subordination

**Requirement class:** Profile constraint

Profiles MAY define domain-specific vocabularies, interpretations, or bindings.

A profile or binding:

- MUST remain subordinate to the core canonical truth, canonical append, trust-profile, export-verification, and core-to-implementation contract requirements of this specification,
- MUST narrow or specialize the core rather than reinterpret it,
- MUST NOT define a second canonical order,
- MUST NOT redefine canonical truth established by the Core Profile.

If a profile or binding conflicts with the core canonical truth, append, trust-profile, export-verification, or contract requirements of this specification, the core specification governs.

### 2.4 Core Profile

**Requirement class:** Constitutional semantic

An implementation conforming to the Core Profile:

1. MUST produce or accept author-originated facts, canonical facts, canonical records, and canonical append attestations as applicable to its claimed role;
2. MUST preserve append-only semantics for canonical records;
3. MUST distinguish canonical truth from derived artifacts;
4. MUST support independently verifiable export for at least one declared export scope.

### 2.5 Role Requirements

The following subsections specify per-role obligations. A conforming implementation that claims a role MUST satisfy every requirement listed for that role.

#### 2.5.1 Fact Producer

**Requirement class:** Constitutional semantic

A Fact Producer conforms if it:

- produces author-originated facts or other attributable facts admitted under the active profile or binding according to this specification,
- signs or otherwise authenticates such facts where the active profile or binding requires it,
- preserves causal references when applicable,
- does not mutate previously produced facts.

#### 2.5.2 Canonical Append Service

**Requirement class:** Constitutional semantic

A Canonical Append Service conforms if it:

- validates admissible facts under the active profile or binding,
- forms canonical records for admitted facts,
- appends canonical records to canonical order,
- issues canonical append attestations,
- does not rewrite prior canonical records,
- does not treat workflow state, projections, or caches as canonical truth.

A Canonical Append Service MUST NOT issue a canonical append attestation for a record until all binding-declared admission prerequisites are satisfied, including resolution of causal or logical dependencies required for that record class (Shared Ledger Binding S5).

#### 2.5.3 Verifier

**Requirement class:** Constitutional semantic

A Verifier conforms if it:

- verifies authored authentication where required,
- verifies canonical append attestations and inclusion proofs,
- distinguishes author-originated facts, canonical records, canonical append attestations, and disclosure or export artifacts,
- does not require access to derived artifacts to verify canonical integrity.

At Core Profile conformance, verifiers MUST support at minimum the following claim classes: canonical-record integrity, append-attestation validity, and inclusion consistency. Additional claim classes (payload integrity, authorization history, disclosure policy) are defined by companion specifications (`trellis/specs/export/disclosure-manifest.md` S4 and `trellis/specs/export/export-verification-package.md` S3).

#### 2.5.4 Derived Processor

**Requirement class:** Constitutional semantic

A Derived Processor conforms if it:

- consumes canonical truth as its only authoritative input,
- records sufficient provenance to support rebuild,
- can be discarded and rebuilt without changing canonical truth.

#### 2.5.5 Export Generator

**Requirement class:** Constitutional semantic

An Export Generator conforms if it:

- packages canonical records, canonical append attestations, and verification material as required by the declared export scope,
- preserves provenance distinctions,
- includes enough material for an offline verifier to validate the export scope.

---

## 3. Core-to-Implementation Contracts

**Requirement class:** Constitutional semantic

The core specification defines explicit contracts between canonical semantics and replaceable implementations. Bindings and implementations MUST preserve these contracts even when the underlying mechanisms change.

At minimum, the following contracts apply:

1. **Canonical Append Contract.** An implementation MAY vary append mechanisms, proof mechanisms, or storage mechanisms, but it MUST preserve fact admission, canonical order, canonical record formation, and canonical append attestation semantics.
2. **Derived Artifact Contract.** An implementation MAY vary projection, indexing, evaluator, or caching mechanisms, but derived artifacts MUST remain rebuildable from canonical truth and MUST NOT become authoritative for canonical facts.
3. **Export Contract.** An implementation MAY vary export packaging and disclosure mechanisms, but exports MUST preserve the provenance distinctions and verification claims required by this specification.

See [WOS Kernel §10 Named Extension Seams] for the workflow, authorization, and custody contracts WOS defines.

---

## 4. Terminology

### 4.1 Author-Originated Fact

A statement, assertion, or action attributable to an originating actor before, or independent of, canonical append attestation.

### 4.2 Canonical Fact

A fact that belongs to canonical truth under this specification. A canonical fact may begin as an author-originated fact that becomes canonical, a grant or revocation fact, a lifecycle fact, or another admitted fact under the active profile or binding.

### 4.3 Canonical Record

A canonical representation of a canonical fact within canonical order. A canonical record is distinct from both the underlying canonical fact and the canonical append attestation that later attests to its inclusion.

### 4.4 Canonical Append Attestation

A service-issued attestation that a canonical record was accepted into canonical order and bound into the canonical append structure. Integrity-verifiable append evidence for a canonical record at a specific canonical order position.

### 4.5 Derived Artifact

A non-canonical projection, evaluator state, cache, index, timeline, or other rebuildable interpretation computed from canonical truth. Examples include queues, dashboards, indexes, caches, and snapshots.

### 4.6 Disclosure or Export Artifact

An audience-specific package, presentation, or view assembled for portability, review, or selective disclosure.

### 4.7 Trust Profile

See [WOS Assurance §5 Provider-Neutral Attestation] and [WOS Kernel §10.5 `custodyHook` seam] for the substrate-generic custody and trust-posture taxonomy. Distributed-custody object semantics are in `trellis/specs/trust/trust-profiles.md`.

### 4.8 Disclosure Posture

See [WOS Assurance §4 Invariant 6: Disclosure Posture Is Not Assurance Level] and [Formspec Respondent Ledger §6.6 Identity attestation object] for the normative definition and posture taxonomy.

### 4.9 Subject Continuity

See [WOS Assurance §3 Subject Continuity] and [Formspec Respondent Ledger §6.6A Identity and implementation decoupling] for the normative definition and requirements.

### 4.10 Append-Head Reference

The preferred generic term for an attested reference to the current head of canonical order. A signed tree head is one possible binding-specific form; other bindings MAY use different concrete mechanisms.

### 4.11 Controlled Vocabulary

**Requirement class:** Constitutional semantic

To reduce synonym drift in normative sections, this specification uses these preferred terms:

- **author-originated fact** for originating assertions or actions,
- **canonical fact** for a fact that belongs to canonical truth,
- **canonical record** for the canonical representation of a canonical fact in canonical order,
- **canonical append attestation** for service-issued inclusion or ordering attestations,
- **derived artifact** for any non-canonical rebuilt or computed output,
- **disclosure or export artifact** for audience-specific packages or presentations,
- **trust profile** for the semantic object governing readability, recovery, and administrative authorities,
- **disclosure posture** for declared identity-visibility stance,
- **subject continuity** for stable cross-time subject references that do not require full identity disclosure,
- **append-head reference** as the preferred generic term for an attested append-head reference.

Normative sections MUST avoid casual synonyms when a preferred term already exists. Companion specifications SHOULD adopt these terms verbatim.

---

## 5. Core Model

### 5.1 Primary Object Classes

Core object classes are:

- author-originated facts,
- canonical records,
- canonical append attestations,
- derived artifacts,
- disclosure and export artifacts.

Canonical facts are a semantic category of facts that belong to canonical truth. Canonical records are their canonical representations in canonical order.

### 5.2 Ontology Discipline

**Requirement class:** Constitutional semantic

Normative sections MUST preserve the distinctions among the primary object classes and MUST NOT collapse derived artifacts or disclosure and export artifacts into canonical truth.

Companion specifications MAY refine object semantics but MUST NOT redefine canonical truth or canonical order semantics established in this document.

---

## 6. Canonical Truth and Invariants

### 6.1 Scope of Canonical Truth

**Requirement class:** Constitutional semantic

Canonical truth includes, at minimum:

- admitted authored, workflow, trust, and release facts in canonical record form,
- canonical append attestations,
- canonical checkpoint material where an implementation defines it.

These categories identify what the core recognizes. They do not require every conforming implementation to emit every category.

Canonical truth excludes, at minimum:

- derived artifacts,
- workflow runtime state,
- authorization evaluator state,
- search indexes,
- caches,
- delegated compute outputs, unless recorded as canonical facts or canonically referenced artifacts.

A conforming implementation MUST NOT treat excluded artifacts as authoritative for canonical facts.

### 6.2 Named Core Invariants

**Requirement class:** Constitutional semantic

1. **Append-only Canonical History.** Canonical records MUST NOT be rewritten in-place.
2. **No Second Canonical Truth.** Derived artifacts MUST NOT be treated as canonical truth.
3. **One Canonical Order per Governed Scope.** Exactly one canonical append-attested order MAY exist per governed scope.
4. **One Canonical Event Hash Construction.** Canonical append semantics MUST bind to exactly one canonical hash construction.
5. **Verification Independence.** Canonical verification MUST NOT depend on workflow runtime internals.
6. **Append Idempotency.** Equivalent admitted canonical inputs MUST NOT create duplicate canonical order positions.

### 6.3 Named Semantic Invariants

**Requirement class:** Constitutional semantic

The following object-distinction invariants complement the core invariants above. Later sections preserve them and reference them explicitly where useful.

- **Invariant A — Author-Originated Fact Is Not Append Attestation.** An author-originated fact and a canonical append attestation are distinct object classes and MUST remain distinguishable.
- **Invariant B — Canonical Fact Is Not Canonical Record.** A canonical fact and the canonical record that represents it MUST remain distinguishable.
- **Invariant C — Derived Artifact Is Not Canonical Truth.** Projections, evaluator state, caches, timelines, and other derived artifacts MUST remain non-canonical.
- **Invariant D — Provider-Readable Access Is Not Reader-Held Access.** Provider-readable and reader-held access MUST remain distinct trust postures.
- **Invariant E — Delegated Compute Is Not General Provider Readability.** Delegated compute access MUST NOT be treated as blanket plaintext access for the service operator.
- **Invariant F — Disclosure Posture Is Not Assurance Level (ledger anchor).** The normative statement is at [WOS Assurance §4]. This ledger's canonical append, export, and disclosure machinery MUST preserve the independence of disclosure posture and assurance level across all canonical records and exports.

---

## 7. Fact Admission, Canonicalization, and Order

### 7.1 Semantic Object Distinction

**Requirement class:** Constitutional semantic

A conforming implementation MUST keep distinguishable:

- an author-originated fact,
- a canonical fact,
- a canonical record,
- a canonical append attestation,
- a disclosure or export artifact.

A canonical record MUST remain distinguishable from the underlying canonical fact it represents. A disclosure or export artifact MUST NOT be treated as identical to the underlying canonical record it may reference or represent.

### 7.2 Core Admissibility Categories

**Requirement class:** Constitutional semantic

The core specification permits at least the following categories of canonically admissible facts:

- author-originated facts that satisfy the active profile or binding,
- grant and revocation facts,
- lifecycle and compliance facts,
- governance or processing facts attributable under the active profile or binding,
- conflict-resolution facts.

Profiles MAY narrow these categories by permitting only a subset of them, imposing additional admission predicates, or constraining which actors or systems may originate or submit them. Profiles MUST NOT reinterpret these categories in a way that changes canonical truth or creates an alternate canonical order.

A Canonical Append Service MUST admit only records that satisfy core admissibility constraints. Schema and version compatibility policy is defined in `trellis/specs/core/shared-ledger-binding.md` S6.

### 7.3 Fact Admission State Machine

**Requirement class:** Constitutional semantic

The core specification defines the following lifecycle for fact admission and canonicalization. A fact becomes canonical only when its canonical record has crossed the implementation's declared durable-append boundary.

| State | Meaning | Entered by | What is true in this state |
|---|---|---|---|
| Originated | An attributable fact exists but has not yet been submitted for canonical admission. | Fact Producer | The fact may be signed or otherwise authenticated, but it is not yet canonical. |
| Submitted for Admission | The fact has been presented to a Canonical Append Service under an active profile or binding. | Fact Producer or submitting system | The fact is pending admission. Submission alone does not make it canonical. |
| Admissible | The Canonical Append Service has determined that the fact satisfies the active admissibility rules. | Canonical Append Service | The fact is eligible to become canonical, but canonical record formation and ordering may still be pending. |
| Accepted for Durable Append | The fact has passed admission checks and is accepted for durable append. | Canonical Append Service | The fact has passed admission checks, but it is not yet canonical. |
| Canonical Record Formed | A canonical record representing the fact has crossed the declared durable-append boundary and been bound into canonical order. | Canonical Append Service | The fact is now canonical and has an ordered canonical representation. |
| Canonical Append Attested | A canonical append attestation has been issued for the canonical record. | Canonical Append Service | The implementation can prove that the canonical record was accepted into canonical order under the active append model. |
| Exported | A disclosure or export artifact has been assembled for a declared scope. | Export Generator | Export exists, but export does not alter canonical truth. |

A canonical append attestation proves that the corresponding canonical record was admitted into canonical order under the active append model. By itself, it does not prove the substantive correctness, legality, or wisdom of the underlying fact beyond the scope of admission and attestation.

### 7.4 Canonical Order Requirements

**Requirement class:** Constitutional semantic

Canonical records that have crossed the declared durable-append boundary MUST be bound into a canonical ordered append structure.

Canonical append order MUST be monotonically append-only within governed scope. Canonical order MUST have a declared scope. That scope MAY be global or limited to a declared ledger, case, tenant, or other append domain. Inclusion, consistency, position, and export claims apply only within that scope.

Implementations MAY partition by scope into multiple ledgers, but MUST NOT allow competing canonical orders for the same governed scope. The canonical append-attestation stream, or its equivalent append-attested order, MUST be the single ordered source of truth for canonical record inclusion and sequence. No workflow runtime, projection, authorization evaluator, or collaboration layer MAY define an alternate canonical order.

Canonical order positions MUST be determined solely by rules in this specification and the applicable binding. Canonical order MUST NOT depend on wall-clock receipt time, queue depth, worker identity, or other operational accidents.

#### 7.4.1 Determinism note

**Requirement class:** Binding or reference choice

Bindings SHOULD specify deterministic tie-breaking where concurrent admissible records could otherwise admit more than one total order consistent with declared causal constraints.

### 7.5 Canonical Append Attestation Requirements

**Requirement class:** Constitutional semantic

A Canonical Append Service MUST return a canonical append attestation for canonical records that have crossed the declared durable-append boundary. A Canonical Append Service MUST NOT issue a canonical append attestation before the durable-append boundary has been crossed.

A canonical append attestation MUST include or reference:

- canonical append position or log index,
- inclusion-oriented proof material,
- an append-head reference,
- sufficient verifier metadata to validate canonical inclusion.

### 7.6 Idempotency and Rejection

**Requirement class:** Constitutional semantic

Replayed equivalent admissions MUST be idempotent. Rejections MUST be explicit and auditable.

Canonical append operations MUST define idempotency semantics for retried or replayed submissions, and MUST define a stable idempotency key or equivalent causal submission identity. A conforming implementation MUST define whether a retried submission is rejected, treated as a no-op, or resolved to an existing canonical record reference. Rejected submissions MUST NOT be treated as canonically appended.

For a given idempotency identity within a declared append scope, every successful retry MUST resolve to the same canonical record reference or the same declared no-op outcome.

Concrete rejection code enumerations (including `hash_construction_mismatch`) are defined in `trellis/specs/core/shared-ledger-binding.md` S7.

### 7.7 Binding Boundary for Serialization and Proofs

**Requirement class:** Binding or reference choice

By itself, the core specification does not standardize byte-level canonicalization, proof encodings, or exact append APIs.

Bindings MAY define deterministic encodings, canonical byte sequences, exact proof formats, or API procedures for author-originated facts, canonical records, canonical append attestations, or disclosure and export artifacts. If such a binding is declared, conforming implementations for that binding MUST follow it. See `trellis/specs/core/shared-ledger-binding.md`.

---

## 8. Canonical Hash Construction

**Requirement class:** Constitutional semantic

Canonical append semantics MUST use exactly one authoritative canonical event hash construction over the sealed canonical record package.

Deterministic canonical serialization is REQUIRED for canonical hashing.

Subordinate hashes MAY exist for specialized purposes (e.g., payload identity, attachments, disclosure artifacts) but MUST NOT redefine canonical append semantics.

**Requirement class:** Binding or reference choice

The set of registered canonical hash constructions, the mandatory default, and the rejection-code behavior for construction mismatch are defined in `trellis/specs/core/shared-ledger-binding.md` S7. Future constructions MUST be registered in that binding before verifiers are required to accept them.

---

## 9. Verification and Export Requirements

### 9.1 Verification Requirement

**Requirement class:** Constitutional semantic

A conforming verifier MUST be able to validate, without requiring derived runtime state:

- canonical record integrity,
- canonical append attestation validity,
- inclusion and consistency claims within the declared append scope,
- append-head consistency when required by the active profile,
- schema or semantic digests and any embedded copies or immutable references required for offline verification,
- any included disclosure or export artifacts,
- export-package canonical provenance claims.

At Core Profile conformance, verifiers MUST support at minimum canonical-record integrity, append-attestation validity, and inclusion consistency. Additional claim classes are defined by companion specifications (see §11).

### 9.2 Export Requirement

**Requirement class:** Constitutional semantic

Conforming implementations MUST support independently verifiable exports for at least one declared scope of canonical truth.

### 9.3 Export Contents

**Requirement class:** Constitutional semantic

An export package MUST include sufficient material for an offline verifier to validate the export scope, including:

- canonical records or their declared canonical representations,
- canonical append attestations or equivalent proof material for the declared scope,
- verification keys or immutable key references,
- append proofs,
- schema or semantic digests plus either embedded copies or immutable references,
- protected payload references or included payloads for the exported scope where applicable,
- canonical facts relevant to the exported scope where required for claim verification.

Any reference required for offline verification MUST be immutable, content-addressed, or included in the export package.

### 9.4 Provenance Distinction Requirement

**Requirement class:** Constitutional semantic

Exports MUST preserve the distinction between:

- author-originated facts,
- canonical records,
- canonical append attestations,
- later assembled disclosure or export artifacts.

### 9.5 Export Verification Independence

**Requirement class:** Constitutional semantic

Export verification MUST NOT depend on derived artifacts, workflow runtime state, mutable service databases, or access to ordinary service APIs beyond what the export explicitly references as optional external proof material.

If an export omits payload readability, the export MUST still disclose which integrity, provenance, and append claims remain verifiable.

Concrete export-packaging and disclosure-manifest normative requirements are specified in `trellis/specs/export/export-verification-package.md` and `trellis/specs/export/disclosure-manifest.md`.

---

## 10. Cross-Repository Authority Boundaries

**Requirement class:** Constitutional semantic

- **Formspec** is authoritative for Definition structure and validation (Core S4), Response semantics (Core S5), FEL evaluation (Core S3), version pinning (Core S6.4, VP-01), and the four-phase processing model (Core S7). Trellis MUST NOT restate or reinterpret these semantics; it cites them by section number.
- **WOS** is authoritative for kernel lifecycle topology (Kernel S3), case state model (Kernel S4), provenance Facts tier (Kernel S6), governance enforcement (Kernel S8), and runtime behavioral contract (Runtime S4–S12). Trellis MUST NOT restate WOS evaluation or governance semantics; it cites them by section number.
- **Trellis Core** is authoritative for canonical ledger semantics (§§5–7), append/attestation semantics (§7.4–§7.5), canonical hash construction scope (§8), verification boundaries (§9), and cross-repository authority (this section).

This specification MUST NOT be interpreted to redefine Formspec or WOS semantic authority. When Trellis normative text depends on Formspec or WOS behavior, it MUST cite the relevant specification section rather than restating the behavior.

---

## 11. Companion Specifications

The Trellis family divides normative material across the following companion specifications. Each companion MUST remain subordinate to this core and MUST NOT redefine canonical truth, canonical order, trust honesty, or verification-independence semantics.

| Concern | Companion |
|---|---|
| Byte-level bindings, rejection codes, schema/version policy, registered hash constructions | `trellis/specs/core/shared-ledger-binding.md` |
| Trust Profile object, trust honesty, trust-profile transitions, standard trust profiles | `trellis/specs/trust/trust-profiles.md` |
| Key lifecycle, cryptographic erasure operating model | `trellis/specs/trust/key-lifecycle-operating-model.md` |
| Derived-artifact runtime discipline, projection semantics, snapshot lifecycle | `trellis/specs/projection/projection-runtime-discipline.md` |
| Export package structure and verification claim classes | `trellis/specs/export/export-verification-package.md` |
| Disclosure manifest and audience-specific disclosure semantics | `trellis/specs/export/disclosure-manifest.md` |
| Monitoring, witnessing, and append-log observability | `trellis/specs/operations/monitoring-witnessing.md` |
| Assurance traceability and methodology detail | `trellis/specs/assurance/assurance-traceability.md` |

Cross-references in the body of this specification use these paths.

---

## 12. Supplementary Constitutional Requirements

This section collects requirements that are constitutional in scope but pertain to specific operational dimensions. Additional operational material that is not constitutional — metadata minimization guidance, storage and snapshot discipline, lifecycle and cryptographic-inaccessibility detail — is delegated to the companions identified in §11 (in particular `trellis/specs/projection/projection-runtime-discipline.md` and `trellis/specs/trust/key-lifecycle-operating-model.md`).

### 12.1 Derived Artifact Requirements

**Requirement class:** Constitutional semantic

A derived artifact:

- MUST NOT be authoritative for canonical facts,
- MUST be rebuildable from canonical truth plus declared configuration history,
- MUST record enough provenance to identify the canonical state from which it was derived,
- MUST treat lag, rebuild, or loss as an operational condition rather than a change to canonical truth.

If a derived evaluator is used for access, policy, workflow, or other rights-impacting decisions, the implementation:

- MUST be able to trace evaluator inputs back to canonical facts,
- MUST define evaluator rebuild behavior,
- MUST define behavior when evaluator state is stale, missing, or inconsistent with canonical facts.

Operational discipline for derived artifacts, including projection rebuild, snapshot lifecycle, and staleness handling, is specified in `trellis/specs/projection/projection-runtime-discipline.md`.

### 12.2 Versioning and Algorithm Agility

**Requirement class:** Constitutional semantic

A conforming implementation:

- MUST version canonical algorithms and schema or semantic references,
- MUST version author-originated fact semantics where profile- or binding-specific semantics exist, canonical record semantics, append semantics, export-verification semantics, and trust-profile semantics,
- MUST preserve enough information to verify historical records under the algorithms and rules in effect when they were produced,
- MUST NOT silently reinterpret historical records under newer rules without an explicit migration mechanism,
- MUST ensure that algorithm or schema evolution does not silently invalidate prior export verification,
- MUST NOT rely on out-of-band operator knowledge to interpret historical records.

Concrete versioning registries, migration mechanisms, and schema-compatibility rules are specified in `trellis/specs/core/shared-ledger-binding.md` S6.

---

## 13. Security and Privacy Considerations

### 13.1 Security Considerations

Implementers should consider at least:

- key compromise,
- verifier or parser divergence,
- replay and reordering attacks,
- recovery abuse,
- authorization drift between canonical facts and derived evaluators,
- snapshot misuse,
- service equivocation,
- delayed offline submission edge cases,
- over-broad delegated compute grants,
- accidental expansion from delegated compute access into standing provider-readable access.

### 13.2 Privacy Considerations

This specification supports strong payload confidentiality, but it does not eliminate metadata disclosure by default. Implementers should consider visible fact categories, timing patterns, access-pattern observability, disclosure linkability, and user-held record reuse correlation risks.

### 13.3 Baseline Scope

**Requirement class:** Constitutional semantic

Baseline Trellis Core conformance MUST NOT be interpreted to require advanced selective disclosure, threshold custody, group-sharing protocols, advanced homomorphic or privacy-preserving computation, or cross-agency analytic protocols unless a declared profile, binding, or implementation specification explicitly requires them. Such capabilities MAY be introduced only through those upper layers without redefining core canonical truth, order, or hash semantics established in this document.

This baseline-scope constraint corresponds to ULCR-100 and ULCOMP-R-213–214 in the requirements matrices.

### 13.4 Trust and Privacy Disclosure Obligations

See [WOS Assurance §6 Legal-Sufficiency Disclosure Obligations] for the normative disclosure obligations that apply to this implementation. Concrete Trust Profile object semantics are specified in `trellis/specs/trust/trust-profiles.md`; key lifecycle and cryptographic-erasure operating detail is specified in `trellis/specs/trust/key-lifecycle-operating-model.md`.

---

## 14. Non-Normative Guidance

*This section is non-normative.*

### 14.1 Practical Implementation Guidance

A practical implementation may use:

- a transactional canonical store,
- a transparency-log-style append mechanism,
- protected payload storage,
- workflow orchestration as a derived operational layer,
- derived authorization evaluators,
- independently verifiable export packaging.

### 14.2 Practical Reduction Rule

If a capability does not define:

- a core object class,
- canonical truth,
- fact admission or attestation semantics,
- trust honesty requirements,
- or export-verification guarantees,

it probably belongs in a profile, binding, sidecar, or implementation specification rather than in this constitutional core.

### 14.3 Final Recommendation

Implementers should build a system with a small custom constitution and a large standard library of replaceable infrastructure.

A useful test: treat every major seam as an explicit contract between core semantics and replaceable implementations, not as an informal architectural preference.

The point of this specification is to define what MUST remain true even if the surrounding stack changes.
