# Unified Ledger Core Specification

> **Normalization status (2026-04-13):** This file is now a legacy source draft.
> Active split-out draft work has started in `specs/core/trellis-core.md`.
> Companion-specific material in this file should be migrated into the new companion drafts under `specs/*/`.

### Migration map from this legacy core draft

The following sections in this file are companion-owned in the normalized family and should be migrated out of core:

- Trust Profile Semantics / Trust Honesty / Trust Profile Transitions → `specs/trust/trust-profiles.md`
- Standard Profiles (Offline, Reader-Held, Delegated Compute, Disclosure/Export, User-Held Reuse, Respondent History) → profile-oriented companion docs under `specs/trust/` and `specs/export/`
- Domain Bindings and Sidecars → `specs/core/shared-ledger-binding.md`
- Derived Artifact operational constraints, snapshots, lifecycle shredding details → `specs/projection/projection-runtime-discipline.md` and `specs/trust/key-lifecycle-operating-model.md`
- Export detail beyond core guarantees → `specs/export/export-verification-package.md` and `specs/export/disclosure-manifest.md`
- Monitoring/witnessing seams and assurance methodology detail → `specs/operations/monitoring-witnessing.md` and `specs/assurance/assurance-traceability.md`

Migration progress: initial normative extraction has been applied into the target companion drafts listed above; remaining pass is wording harmonization and removal of duplicated legacy prose.

## Abstract

This specification defines the constitutional core of a browser-originated, append-only, cryptographically verifiable ledger for high-trust workflow systems.

It standardizes:

- core object classes,
- canonical truth boundaries,
- fact admission, canonicalization, ordering, and attestation semantics,
- trust honesty requirements,
- export and verification guarantees,
- the contracts between core semantics and replaceable implementations.

It does not standardize product UX, deployment stacks, byte-level formats, proof encodings, or operational architecture.

Profiles, bindings, sidecars, and implementation specifications may define domain vocabularies, concrete serializations, product-facing interpretations, and deployment-specific mechanisms without changing the constitutional core.

Its purpose is to define what MUST remain true even if the surrounding stack changes.

## Status of This Document

This document is a draft technical specification.

This document is the **semantic constitution** of the system. It defines invariants, trust semantics, canonical truth, and verification boundaries. It does not define exact implementation mechanisms, deployment stacks, product integrations, or byte-level proof bindings.

The intended hierarchy is:

- the **core specification** defines what MUST remain true,
- **profiles** define permitted domain-specific interpretations that narrow but do not reinterpret the core,
- **bindings** define how core semantics map onto concrete technologies,
- **sidecars** collect family-specific, deployment-specific, or implementation-adjacent material without altering the core,
- **implementation specifications** define exact bytes, proofs, APIs, and operational procedures.

When a capability is described in this specification, the text MUST make clear whether that capability is:

- a **constitutional semantic**,
- a **profile constraint**, or
- a **binding or reference choice**.

Statements that mix those layers without clear boundaries SHOULD be split or revised.

The core specification standardizes semantics, invariants, and verification boundaries. Exact bytes, proof encodings, APIs, and operational procedures belong in bindings and implementation specifications.

## Table of Contents

1. Introduction
2. Conformance
3. Core-to-Implementation Contracts
4. Terminology
5. Core Ontology
6. Canonical Truth
7. Invariants
8. Fact Admission and Canonicalization
9. Canonical Order and Attestation
10. Trust Profile Semantics
11. Trust Honesty and Transitions
12. Export and Verification Guarantees
13. Profile Discipline
14. Standard Profiles
15. Domain Bindings and Sidecars
16. Supplementary Requirements and Operational Constraints
17. Security and Privacy Considerations
18. Non-Normative Guidance

---

# 1. Introduction

## 1.1 Scope

This specification defines the constitutional core of a unified ledger.

It defines:

- what kinds of objects exist in the core model,
- what becomes canonical truth,
- how facts become admitted, represented, ordered, and attested,
- what trust claims an implementation MUST declare honestly,
- what export and verification guarantees a conforming implementation MUST preserve.

It does not define:

- storage backends,
- workflow engines,
- IAM stacks,
- policy engines,
- exact serializations,
- exact proof formats,
- domain-specific vocabularies,
- respondent-history vocabularies,
- workflow-family vocabularies.

## 1.2 Problem Statement

A conventional workflow platform often creates multiple sources of truth:

- authored facts,
- workflow or provenance facts,
- operational workflow state,
- mutable application database state,
- export views assembled after the fact.

This specification defines a model in which canonical truth is explicit and narrow, and workflow engines, authorization evaluators, caches, projections, and presentation layers remain replaceable.

---

# 2. Conformance

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and **OPTIONAL** in this document are to be interpreted as described in BCP 14.

## 2.1 Conformance Classes

The following conformance classes are defined:

- **Fact Producer**: creates author-originated facts or other attributable facts admitted under the active profile or binding.
- **Canonical Append Service**: admits facts, forms canonical records, orders them, and issues canonical append attestations.
- **Verifier**: verifies authored facts, canonical append attestations, and export scopes.
- **Derived Processor**: builds derived artifacts from canonical truth.
- **Export Generator**: assembles disclosure or export artifacts and verification packages.

A system conforms to this specification only if it satisfies all requirements for each conformance class it claims.

## 2.2 Conformance Profiles

The following generic conformance profiles are defined:

- **Core Profile**
- **Offline Authoring Profile**
- **Reader-Held Decryption Profile**
- **Delegated Compute Profile**
- **Disclosure and Export Profile**
- **User-Held Record Reuse Profile**
- **Respondent History Profile**

An implementation MAY claim conformance to one or more profiles.

An implementation claiming conformance to any non-Core Profile MUST also conform to the Core Profile.

## 2.3 Profile and Binding Subordination

**Requirement class: Profile constraint**

Profiles MAY define domain-specific vocabularies, interpretations, or bindings.

A profile or binding:

- MUST remain subordinate to the core canonical truth, canonical append, trust-profile, export-verification, and core-to-implementation contract requirements of this specification,
- MUST narrow or specialize the core rather than reinterpret it,
- MUST NOT define a second canonical order,
- MUST NOT redefine canonical truth established by the Core Profile.

If a profile or binding conflicts with the core canonical truth, append, trust-profile, export-verification, or contract requirements of this specification, the core specification governs.

## 2.4 Core Profile

**Requirement class: Constitutional semantic**

An implementation conforming to the Core Profile:

- MUST produce or accept author-originated facts, canonical facts, canonical records, and canonical append attestations as applicable to its claimed role,
- MUST preserve append-only semantics for canonical records,
- MUST distinguish canonical truth from derived artifacts,
- MUST support independently verifiable export for at least one declared export scope.

## 2.5 Conformance Class Requirements

### 2.5.1 Fact Producer

**Requirement class: Constitutional semantic**

A Fact Producer conforms if it:

- produces author-originated facts or other attributable facts admitted under the active profile or binding according to this specification,
- signs or otherwise authenticates such facts where the active profile or binding requires it,
- preserves causal references when applicable,
- does not mutate previously produced facts.

### 2.5.2 Canonical Append Service

**Requirement class: Constitutional semantic**

A Canonical Append Service conforms if it:

- validates admissible facts under the active profile or binding,
- forms canonical records for admitted facts,
- appends canonical records to canonical order,
- issues canonical append attestations,
- does not rewrite prior canonical records,
- does not treat workflow state, projections, or caches as canonical truth.

### 2.5.3 Verifier

**Requirement class: Constitutional semantic**

A Verifier conforms if it:

- verifies authored authentication where required,
- verifies canonical append attestations and inclusion proofs,
- distinguishes author-originated facts, canonical records, canonical append attestations, and disclosure or export artifacts,
- does not require access to derived artifacts to verify canonical integrity.

### 2.5.4 Derived Processor

**Requirement class: Constitutional semantic**

A Derived Processor conforms if it:

- consumes canonical truth as its only authoritative input,
- records sufficient provenance to support rebuild,
- can be discarded and rebuilt without changing canonical truth.

### 2.5.5 Export Generator

**Requirement class: Constitutional semantic**

An Export Generator conforms if it:

- packages canonical records, canonical append attestations, and verification material as required by the declared export scope,
- preserves provenance distinctions,
- includes enough material for an offline verifier to validate the export scope.

---

# 3. Core-to-Implementation Contracts

**Requirement class: Constitutional semantic**

The core specification defines explicit contracts between canonical semantics and replaceable implementations.

At minimum, the following contracts apply:

- **Canonical Append Contract**: an implementation MAY vary append mechanisms, proof mechanisms, or storage mechanisms, but it MUST preserve fact admission, canonical order, canonical record formation, and canonical append attestation semantics.
- **Derived Artifact Contract**: an implementation MAY vary projection, indexing, evaluator, or caching mechanisms, but derived artifacts MUST remain rebuildable from canonical truth and MUST NOT become authoritative for canonical facts.
- **Workflow Contract**: an implementation MAY vary workflow or orchestration mechanisms, but workflow state MUST remain operational rather than canonical unless later represented as canonical facts under the active profile or binding.
- **Authorization Contract**: an implementation MAY vary authorization evaluators or policy engines, but grant and revocation semantics MUST remain canonical and evaluator state MUST remain derived.
- **Trust Contract**: an implementation MAY vary custody, key-management, or delegated compute mechanisms, but the active Trust Profile MUST continue to describe who can read, recover, delegate, attest, or administer access.
- **Export Contract**: an implementation MAY vary export packaging and disclosure mechanisms, but exports MUST preserve the provenance distinctions and verification claims required by this specification.

Bindings and implementations MUST preserve these contracts even when the underlying mechanisms change.

---

# 4. Terminology

## 4.1 Author-Originated Fact

A statement, assertion, or action attributable to an originating actor before, or independent of, canonical append attestation.

## 4.2 Canonical Fact

A fact that belongs to canonical truth under this specification.

A canonical fact may begin as an author-originated fact that becomes canonical, a grant or revocation fact, a lifecycle fact, or another admitted fact under the active profile or binding.

## 4.3 Canonical Record

A canonical representation of a canonical fact within canonical order.

A canonical record is distinct from both the underlying canonical fact and the canonical append attestation that later attests to its inclusion.

## 4.4 Canonical Append Attestation

A service-issued attestation that a canonical record was accepted into canonical order and bound into the canonical append structure.

## 4.5 Derived Artifact

A non-canonical projection, evaluator state, cache, index, timeline, or other rebuildable interpretation computed from canonical truth.

## 4.6 Disclosure or Export Artifact

An audience-specific package, presentation, or view assembled for portability, review, or selective disclosure.

## 4.7 Trust Profile

A semantic object that declares which actors may decrypt, recover, attest, block, or administer data in a deployment mode.

## 4.8 Disclosure Posture

A declared posture describing how much identity or subject information is intended to be revealed in a given context, such as anonymous, pseudonymous, identified, or public.

## 4.9 Subject Continuity

A stable continuity reference for a subject, record holder, or respondent that links related activity, records, or attestations across time without, by itself, requiring full legal identity disclosure.

## 4.10 Controlled Vocabulary

To reduce synonym drift in normative sections, this specification uses these preferred terms:

- **author-originated fact** for originating assertions or actions,
- **canonical fact** for a fact that belongs to canonical truth,
- **canonical record** for the canonical representation of a canonical fact in canonical order,
- **canonical append attestation** for service-issued inclusion or ordering attestations,
- **derived artifact** for any non-canonical rebuilt or computed output,
- **disclosure or export artifact** for audience-specific packages or presentations,
- **append-head reference** as the preferred generic term for an attested append-head reference, of which a signed tree head is one possible binding-specific form.

Normative sections MUST avoid casual alternatives when a preferred term already exists.

---

# 5. Core Ontology

## 5.1 Primary Object Classes

This specification is organized around five primary object classes:

- **Author-Originated Facts**
- **Canonical Records**
- **Canonical Append Attestations**
- **Derived Artifacts**
- **Disclosure and Export Artifacts**

Canonical facts are a semantic category of facts that belong to canonical truth. Canonical records are their canonical representations in canonical order.

## 5.2 Ontology Discipline

**Requirement class: Constitutional semantic**

Normative sections MUST preserve the distinctions among the primary object classes and MUST NOT collapse derived artifacts or disclosure and export artifacts into canonical truth.

---

# 6. Canonical Truth

## 6.1 Scope of Canonical Truth

**Requirement class: Constitutional semantic**

Canonical truth includes, at minimum:

- canonical facts admitted under the active profile or binding, including author-originated facts that have become canonical, grant and revocation facts, conflict-resolution facts, and lifecycle or compliance facts where the implementation defines them,
- canonical append attestations.

These categories identify what the core recognizes. They do not require every conforming implementation to emit every category.

Canonical truth excludes, at minimum:

- derived artifacts,
- workflow runtime state,
- authorization evaluator state,
- search indexes,
- caches,
- delegated compute outputs unless recorded as canonical facts or canonically referenced artifacts.

A conforming implementation MUST NOT treat excluded artifacts as authoritative for canonical facts.

---

# 7. Invariants

## 7.1 Named Invariants

**Requirement class: Constitutional semantic**

The following invariants are fundamental to this specification:

- **Invariant 1 — Author-Originated Fact Is Not Append Attestation**: an author-originated fact and a canonical append attestation are distinct object classes and MUST remain distinguishable.
- **Invariant 2 — Canonical Fact Is Not Canonical Record**: a canonical fact and the canonical record that represents it MUST remain distinguishable.
- **Invariant 3 — Derived Artifact Is Not Canonical Truth**: projections, evaluator state, caches, timelines, and other derived artifacts MUST remain non-canonical.
- **Invariant 4 — Provider-Readable Access Is Not Reader-Held Access**: provider-readable and reader-held access MUST remain distinct trust postures.
- **Invariant 5 — Delegated Compute Is Not General Provider Readability**: delegated compute access MUST NOT be treated as blanket plaintext access for the service operator.
- **Invariant 6 — Disclosure Posture Is Not Assurance Level**: disclosure posture and assurance posture MUST remain distinct and MUST NOT be conflated.

Later sections SHOULD preserve these invariants and reference them explicitly where useful.

---

# 8. Fact Admission and Canonicalization

## 8.1 Semantic Object Distinction

**Requirement class: Constitutional semantic**

This section narrows the broader ontology to the semantic objects that participate directly in canonical representation, canonical append, and later disclosure or export.

A conforming implementation MUST keep distinguishable:

- an author-originated fact,
- a canonical fact,
- a canonical record,
- a canonical append attestation,
- a disclosure or export artifact.

A canonical record MUST remain distinguishable from the underlying canonical fact it represents.

A disclosure or export artifact MUST NOT be treated as identical to the underlying canonical record it may reference or represent.

## 8.2 Core Admissibility Categories

**Requirement class: Constitutional semantic**

The core specification permits at least the following categories of canonically admissible facts:

- author-originated facts that satisfy the active profile or binding,
- grant and revocation facts,
- lifecycle and compliance facts,
- governance or processing facts attributable under the active profile or binding,
- conflict-resolution facts.

Profiles MAY narrow these categories by:

- permitting only a subset of them,
- imposing additional admission predicates,
- constraining which actors or systems may originate or submit them.

Profiles MUST NOT reinterpret these categories in a way that changes canonical truth or creates an alternate canonical order.

## 8.3 Fact Admission and Canonicalization State Machine

**Requirement class: Constitutional semantic**

The core specification defines the following lifecycle for fact admission and canonicalization:

| State | Meaning | Entered by | What is true in this state |
|---|---|---|---|
| Originated | An attributable fact exists but has not yet been submitted for canonical admission. | Fact Producer | The fact may be signed or otherwise authenticated, but it is not yet canonical. |
| Submitted for Admission | The fact has been presented to a Canonical Append Service under an active profile or binding. | Fact Producer or submitting system | The fact is pending admission. Submission alone does not make it canonical. |
| Admissible | The Canonical Append Service has determined that the fact satisfies the active admissibility rules. | Canonical Append Service | The fact is eligible to become canonical, but canonical record formation and ordering may still be pending. |
| Accepted for Durable Append | The fact has passed admission checks and is accepted for durable append. | Canonical Append Service | The fact has passed admission checks, but it is not yet canonical. |
| Canonical Record Formed | A canonical record representing the fact has crossed the declared durable-append boundary and been bound into canonical order. | Canonical Append Service | The fact is now canonical and has an ordered canonical representation. |
| Canonical Append Attested | A canonical append attestation has been issued for the canonical record. | Canonical Append Service | The implementation can prove that the canonical record was accepted into canonical order under the active append model. |
| Exported | A disclosure or export artifact has been assembled for a declared scope. | Export Generator | Export exists, but export does not alter canonical truth. |

For this specification, a fact becomes canonical only when its canonical record has crossed the implementation's declared durable-append boundary.

A canonical append attestation proves that the corresponding canonical record was admitted into canonical order under the active append model. By itself, it does not prove the substantive correctness, legality, or wisdom of the underlying fact beyond the scope of admission and attestation.

---

# 9. Canonical Order and Attestation

## 9.1 Canonical Order Requirements

**Requirement class: Constitutional semantic**

Canonical records that have crossed the declared durable-append boundary MUST be bound into a canonical ordered append structure.

Canonical order MUST have a declared scope. That scope MAY be global or limited to a declared ledger, case, tenant, or other append domain. Inclusion, consistency, position, and export claims apply only within that scope.

The canonical append-attestation stream, or its equivalent append-attested order, MUST be the single ordered source of truth for canonical record inclusion and sequence.

No workflow runtime, projection, authorization evaluator, or collaboration layer MAY define an alternate canonical order.

## 9.2 Canonical Append Attestation Requirements

**Requirement class: Constitutional semantic**

A Canonical Append Service MUST return a canonical append attestation for canonical records that have crossed the declared durable-append boundary.

A Canonical Append Service MUST NOT issue a canonical append attestation before the durable-append boundary has been crossed.

A canonical append attestation MUST include or reference:

- canonical append position or log index,
- inclusion-oriented proof material,
- append-head reference,
- sufficient verifier metadata to validate canonical inclusion.

## 9.3 Binding Boundary for Serialization and Proofs

**Requirement class: Binding or reference choice**

By itself, the core specification does not standardize byte-level canonicalization, proof encodings, or exact append APIs.

Bindings MAY define deterministic encodings, canonical byte sequences, exact proof formats, or API procedures for author-originated facts, canonical records, canonical append attestations, or disclosure and export artifacts.

If such a binding is declared, conforming implementations for that binding MUST follow it.

---

# 10. Trust Profile Semantics

## 10.1 Trust Profile Minimum Object Semantics

**Requirement class: Constitutional semantic**

The core specification treats Trust Profile as a first-class semantic object, even when a binding chooses the concrete serialization.

A Trust Profile object MUST semantically include at least:

- a profile identifier,
- a scope or deployment mode identifier,
- the ordinary-operation readability posture,
- the reader-held access posture, if any,
- the delegated compute posture, if any,
- current-content decryption authorities,
- historical-content decryption authorities,
- recovery authorities and recovery conditions,
- canonical append attestation control authorities,
- exceptional-access authorities,
- a statement or reference describing metadata visibility.

Bindings MAY define the concrete wire shape of a Trust Profile object, but they MUST preserve these minimum semantic fields and their meanings.

## 10.2 Disclosure Posture and Assurance

**Requirement class: Constitutional semantic**

A conforming implementation:

- MUST distinguish assurance level from disclosure posture,
- MUST NOT treat higher assurance as requiring greater identity disclosure by default,
- MAY support subject continuity without requiring full legal identity disclosure,
- MUST preserve those distinctions across trust profiles, exports, and disclosures.

---

# 11. Trust Honesty and Transitions

## 11.1 Trust Honesty Requirements

**Requirement class: Constitutional semantic**

For every deployment mode that handles protected content, an implementation:

- MUST publish a Trust Profile,
- MUST state whether ordinary service operation is provider-readable, reader-held, or reader-held with delegated compute,
- MUST state whether the service runtime can access plaintext during ordinary processing,
- MUST state whether recovery can occur without the user,
- MUST state whether delegated compute exposes plaintext to ordinary service components,
- MUST NOT collapse delegated compute access into provider-readable access unless explicitly declared,
- MUST NOT describe a trust posture more strongly than the implementation behavior supports.

## 11.2 Trust Profile Transition Requirements

**Requirement class: Constitutional semantic**

If an implementation changes custody mode, provider readability posture, recovery semantics, or delegated compute semantics for protected content, it:

- MUST treat that change as a Trust Profile transition,
- MUST make the transition auditable,
- MUST define whether the transition applies prospectively, retrospectively, or both,
- MUST NOT expand from reader-held access or delegated compute access into provider-readable access without such an explicit transition.

---

# 12. Export and Verification Guarantees

## 12.1 Export Requirement

**Requirement class: Constitutional semantic**

Conforming implementations MUST support independently verifiable exports for at least one declared scope of canonical truth.

## 12.2 Export Contents

**Requirement class: Constitutional semantic**

An export package MUST include sufficient material for an offline verifier to validate the export scope, including:

- canonical records or their declared canonical representations,
- canonical append attestations or equivalent proof material for the declared scope,
- verification keys or immutable key references,
- append proofs,
- schema or semantic digests plus either embedded copies or immutable references,
- protected payload references or included payloads for exported scope where applicable,
- canonical facts relevant to the exported scope where required for claim verification.

Any reference required for offline verification MUST be immutable, content-addressed, or included in the export package.

## 12.3 Verification Requirement

**Requirement class: Constitutional semantic**

A conforming verifier MUST be able to:

1. verify authored signatures or equivalent authored authentication where required,
2. verify canonical inclusion within the declared append scope,
3. verify append-head consistency when required by the profile,
4. verify schema or semantic digests and any embedded copies or immutable references required for offline verification,
5. verify any included disclosure or export artifacts.

## 12.4 Provenance Distinction Requirement

**Requirement class: Constitutional semantic**

Exports MUST preserve the distinction between:

- author-originated facts,
- canonical records,
- canonical append attestations,
- later assembled disclosure or export artifacts.

## 12.5 Export Verification Independence

**Requirement class: Constitutional semantic**

Export verification MUST NOT depend on:

- derived artifacts,
- workflow runtime state,
- mutable service databases,
- access to ordinary service APIs beyond what the export explicitly references as optional external proof material.

If an export omits payload readability, the export MUST still disclose which integrity, provenance, and append claims remain verifiable.

---

# 13. Profile Discipline

## 13.1 Generic Profile Discipline

**Requirement class: Profile constraint**

Profiles MAY define domain-specific interpretation layers over the core specification.

Such layers:

- MUST remain profiles rather than constitutional core semantics,
- MUST NOT alter core truth, admission, order, attestation, trust honesty, or export-verification semantics,
- MUST NOT define an alternate canonical source of truth.

## 13.2 Profile Trust Inheritance

**Requirement class: Profile constraint**

All profiles and bindings inherit the active Trust Profile.

A profile or binding:

- MUST distinguish provider-readable access, reader-held access, and delegated compute access when protected content is involved,
- MUST NOT imply stronger confidentiality than the active Trust Profile supports,
- MUST NOT use profile-local wording to weaken or bypass the Trust Profile requirements of this specification.

## 13.3 Profile-Scoped Export Honesty

**Requirement class: Profile constraint**

A profile-scoped export MAY present a profile-specific timeline, delta history, or audience-specific interpretation.

Such an export:

- MUST preserve the distinction between author-originated facts, canonical records, canonical append attestations, and later disclosure or export artifacts,
- MUST NOT imply broader coverage than its declared export scope actually includes.

---

# 14. Standard Profiles

## 14.1 Offline Authoring Profile

**Requirement class: Profile constraint**

An Offline Authoring Profile MAY require:

- delayed submission semantics,
- preservation of authored time or authored context where available,
- local pending state prior to admission,
- authored authentication before canonical admission.

Such a profile MUST preserve the core state machine and provenance distinctions of Section 8.

## 14.2 Reader-Held Decryption Profile

**Requirement class: Profile constraint**

A Reader-Held Decryption Profile MAY require that ordinary service operation does not require general plaintext access for declared protected content.

Such a profile MUST:

- identify which principals may decrypt within scope,
- remain consistent with the active Trust Profile,
- preserve the distinction between reader-held access and provider-readable access.

## 14.3 Delegated Compute Profile

**Requirement class: Profile constraint**

A Delegated Compute Profile MAY define requirements for explicit, attributable, auditable, and scoped delegated compute access.

Such a profile MUST NOT imply that delegated compute confers general provider readability.

If workflow or adjudication materially relies on delegated compute output, the profile MUST require either:

- canonical representation of that output as a canonical fact, or
- a canonical reference to a stable output artifact.

## 14.4 Disclosure and Export Profile

**Requirement class: Profile constraint**

A Disclosure and Export Profile MAY define:

- audience-specific export scopes,
- disclosure postures,
- profile-specific claim classes,
- profile-specific presentation rules.

Such a profile MUST remain subordinate to Section 12.

## 14.5 User-Held Record Reuse Profile

**Requirement class: Profile constraint**

A User-Held Record Reuse Profile MAY define how previously user-held records, attestations, or supporting material are referenced or submitted into canonical workflows.

Such a profile MUST:

- distinguish reusable prior records from canonical facts,
- bind exactly what was reused or submitted when such content enters canonical truth,
- avoid treating the entire user-held record layer as canonical workflow state by default.

## 14.6 Respondent History Profile

**Requirement class: Profile constraint**

A Respondent History Profile MAY define respondent-originated or respondent-visible material history, including draft, save, submit, amendment, attachment, validation, prepopulation, or materially relevant attestation boundaries.

Such a profile MUST:

- treat respondent-history timelines as derived artifacts over canonical truth,
- avoid defining a second canonical append model,
- avoid implying full workflow, governance, custody, or compliance coverage unless that profile scope actually includes those materials.

---

# 15. Domain Bindings and Sidecars

## 15.1 Generic Binding Discipline

**Requirement class: Binding or reference choice**

Bindings MAY define concrete serializations, APIs, proof encodings, or technology mappings for the core semantics.

Bindings MUST preserve the constitutional semantics and contracts defined by this specification.

## 15.2 Domain Vocabulary Placement

**Requirement class: Profile constraint**

Domain vocabularies, respondent-history vocabularies, forms vocabularies, workflow-family vocabularies, and similar interpretation layers SHOULD be defined in profiles rather than in the constitutional core.

## 15.3 Family Bindings

**Requirement class: Binding or reference choice**

A binding MAY map the constitutional core onto a specific forms family, workflow family, or respondent-history family.

Such a binding MAY define:

- stable path semantics,
- item-key semantics,
- validation-boundary semantics,
- amendment or migration semantics,
- profile-specific change-set structures.

These remain binding- or profile-level choices unless a higher-level profile adopts them.

## 15.4 Sidecar Candidates

**Requirement class: Binding or reference choice**

A sidecar MAY collect family-specific, deployment-specific, or implementation-adjacent material that remains subordinate to the constitutional core.

Useful sidecar candidates include:

- respondent-history sidecars,
- user-held record reuse sidecars,
- forms-family binding sidecars,
- workflow-family binding sidecars,
- deployment-profile sidecars,
- concrete trust-profile examples.

A sidecar MUST NOT alter the constitutional semantics of this specification.

---

# 16. Supplementary Requirements and Operational Constraints

## 16.1 Derived Artifact Requirements

**Requirement class: Constitutional semantic**

A derived artifact:

- MUST NOT be authoritative for canonical facts,
- MUST be rebuildable from canonical truth plus declared configuration history,
- MUST record enough provenance to identify the canonical state from which it was derived,
- MUST treat lag, rebuild, or loss as an operational condition rather than a change to canonical truth.

If a derived evaluator is used for access, policy, workflow, or other rights-impacting decisions, the implementation:

- MUST be able to trace evaluator inputs back to canonical facts,
- MUST define evaluator rebuild behavior,
- MUST define behavior when evaluator state is stale, missing, or inconsistent with canonical facts.

## 16.2 Metadata Minimization

**Requirement class: Constitutional semantic**

Metadata minimization is a hard design constraint.

Visible metadata SHOULD be limited to what is required for:

- canonical verification,
- schema or semantic lookup,
- required audit-visible declarations,
- conflict gating,
- append processing.

Implementations SHOULD NOT keep visible metadata merely to accelerate derived artifacts.

Implementations MUST NOT retain visible append-related metadata merely for operational convenience when the same function can be satisfied by derived or scoped mechanisms.

## 16.3 Idempotency and Rejection Semantics

**Requirement class: Constitutional semantic**

Canonical append operations MUST define idempotency semantics for retried or replayed submissions.

Canonical append operations MUST define a stable idempotency key or equivalent causal submission identity.

A conforming implementation MUST define whether a retried submission is:

- rejected,
- treated as a no-op, or
- resolved to an existing canonical record reference.

Rejected submissions MUST NOT be treated as canonically appended.

For a given idempotency identity within a declared append scope, every successful retry MUST resolve to the same canonical record reference or the same declared no-op outcome.

## 16.4 Storage and Snapshot Discipline

**Requirement class: Constitutional semantic**

Canonical records MUST be stored durably and immutably from the perspective of ordinary append participants.

A conforming implementation MUST declare the durable-append boundary that makes a canonical record accepted for attestation, retry handling, and export.

Protected payloads MAY be stored in one or more blob stores.

Snapshots MAY be used for performance, but snapshots MUST be treated as derived artifacts and MUST NOT become canonical truth.

Replica completion state MUST remain operational state, not canonical truth.

## 16.5 Lifecycle and Cryptographic Inaccessibility

**Requirement class: Constitutional semantic**

Implementations MAY define lifecycle facts for:

- retention,
- legal hold,
- archival,
- key destruction,
- sealing,
- export issuance,
- schema upgrade.

An implementation MAY support only a subset of these lifecycle operations or none of them.

If an implementation supports one of these operations as part of its canonical or compliance-relevant behavior, it MUST represent that operation as a lifecycle fact.

If such a fact affects compliance posture, retention posture, or recoverability claims, it MUST be a canonical fact.

If an implementation uses cryptographic erasure or key destruction, it MUST document:

- which content becomes irrecoverable,
- who retains access, if anyone,
- what evidence of destruction is preserved,
- what metadata remains.

If protected content is cryptographically destroyed or otherwise made inaccessible, affected derived plaintext state MUST be invalidated, purged, or otherwise made unusable according to the implementation's declared policy.

## 16.6 Versioning and Algorithm Agility

**Requirement class: Constitutional semantic**

A conforming implementation:

- MUST version canonical algorithms and schema or semantic references,
- MUST version author-originated fact semantics where profile- or binding-specific semantics exist, canonical record semantics, append semantics, export verification semantics, and trust profile semantics,
- MUST preserve enough information to verify historical records under the algorithms and rules in effect when they were produced,
- MUST NOT silently reinterpret historical records under newer rules without an explicit migration mechanism,
- MUST ensure that algorithm or schema evolution does not silently invalidate prior export verification,
- MUST NOT rely on out-of-band operator knowledge to interpret historical records.

---

# 17. Security and Privacy Considerations

## 17.1 Security Considerations

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

## 17.2 Privacy Considerations

This specification supports strong payload confidentiality, but it does not eliminate metadata disclosure by default.

Implementers should consider:

- visible fact categories,
- timing patterns,
- access-pattern observability,
- disclosure linkability,
- user-held record reuse correlation risks.

## 17.3 Trust and Privacy Disclosure Obligations

**Requirement class: Constitutional semantic**

A conforming implementation handling protected content:

- MUST disclose what metadata remains visible,
- MUST disclose which parties can observe that metadata,
- MUST disclose whether ordinary service operation is provider-readable,
- MUST disclose whether delegated compute exposes plaintext to ordinary service components,
- MUST NOT describe ciphertext storage as equivalent to provider blindness when decryption paths exist.

---

# 18. Non-Normative Guidance

This section is non-normative.

## 18.1 Practical Implementation Guidance

A practical implementation may use:

- a transactional canonical store,
- a transparency-log-style append mechanism,
- protected payload storage,
- workflow orchestration as a derived operational layer,
- derived authorization evaluators,
- independently verifiable export packaging.

## 18.2 Practical Reduction Rule

If a capability does not define:

- a core object class,
- canonical truth,
- fact admission or attestation semantics,
- trust honesty requirements,
- or export-verification guarantees,

it probably belongs in a profile, binding, sidecar, or implementation specification rather than in the constitutional core.

## 18.3 Final Recommendation

Implementers should build a system with a small custom constitution and a large standard library of replaceable infrastructure.

A useful test is this: treat every major seam as an explicit contract between core semantics and replaceable implementations, not as an informal architectural preference.

The point of this specification is to define what MUST remain true even if the surrounding stack changes.
