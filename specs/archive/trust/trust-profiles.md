---
title: Trellis Companion — Trust Profiles
version: 0.1.0-draft.3
date: 2026-04-14
status: draft
---

# Trellis Companion — Trust Profiles v0.1

**Version:** 0.1.0-draft.3
**Date:** 2026-04-14
**Editors:** Formspec Working Group
**Companion to:** Trellis Core v0.1

---

## Status of This Document

This document is a **draft specification**. It is a companion to the Trellis Core specification [Trellis Core] and does not modify Formspec or WOS processing semantics. Implementors are encouraged to experiment with this specification and provide feedback, but MUST NOT treat it as stable for production use until a 1.0.0 release is published.

This companion is **subordinate to Trellis Core**. Where this companion states a requirement that elaborates a constitutional semantic defined in [Trellis Core], the constitutional statement in [Trellis Core] governs. Where this companion defines operational detail that [Trellis Core] does not fix, this companion is the normative source.

## Abstract

The Trust Profiles companion defines custody, readability, and verification posture declarations for Trellis deployments. It specifies the minimum semantic object shape of a Trust Profile, the honesty requirements that govern what a deployment may claim, the transition rules that apply when custody or readability posture changes, the baseline profiles that implementations MAY conform to, and a non-normative catalogue of illustrative example profiles. This companion adds trust and disclosure semantics to the Trellis canonical substrate defined in [Trellis Core]. It does not define Formspec or WOS semantics — it governs who can observe what, under what declared posture, with what leakage characteristics.

## Conventions and Terminology

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in BCP 14 [RFC 2119] [RFC 8174] when, and only when, they appear in ALL CAPITALS, as shown here.

JSON syntax and data types are as defined in [RFC 8259]. URI syntax is as defined in [RFC 3986].

Each normative section is tagged with a **Requirement class** marker drawn from the following vocabulary:

- **Constitutional semantic** — a requirement that restates or directly elaborates a constitutional semantic of [Trellis Core]. Bindings and profiles MUST NOT weaken these.
- **Profile constraint** — a requirement that applies when a deployment claims conformance to a named Standard Profile defined in this companion.
- **Companion requirement** — an operational or cross-cutting requirement that applies to any implementation claiming conformance to this companion.
- **Non-normative guidance** — illustrative material, examples, or comparison aids. Not conformance material.

## Table of Contents

1. Introduction
2. Conformance
3. Trust Profile Object Semantics
4. Trust Honesty and Transitions
5. Baseline Profile Postures
6. Metadata Budget
7. Verification Posture Classes
8. Profile Declaration Schema
9. Access Semantics and Profile Honesty Detail
10. Standard Profiles
11. Relationship to Export Claim Classes
12. Audit and Conformance Hooks
13. Example Profiles (Non-normative)
14. Security and Privacy Considerations
15. Normative References

---

## 1. Introduction

### 1.1 Purpose

**Requirement class: Non-normative guidance**

Define explicit custody and readability postures, the honesty rules that bind them, and the transition discipline that applies when posture changes. A Trust Profile is the declarative object through which a deployment states, in machine- and auditor-readable form, who can observe what, under what declared posture, and with what leakage characteristics.

### 1.2 Relationship to Trellis Core

**Requirement class: Non-normative guidance**

[Trellis Core] treats the Trust Profile as a first-class constitutional semantic object (Trellis Core §4.7 and §13.4, delegating concrete object semantics to this companion). [WOS Kernel §10.5] `custodyHook` delegates Trust Profile object definition to this binding spec. This companion elaborates the object shape, the honesty requirements, the transition requirements, and the Standard Profiles that a deployment MAY conform to. Bindings MAY choose the concrete wire shape, but they MUST preserve the semantic fields and meanings defined in §3.

### 1.3 Relationship to Other Companions

**Requirement class: Non-normative guidance**

- [Key-Lifecycle-Operating-Model] governs the lifecycle of the cryptographic material by which Trust Profile postures are realized.
- [Disclosure-Manifest] and [Export-Verification-Package] govern which claim classes a declared Trust Profile makes verifiable in exports.
- The Shared Ledger Binding defines the canonical fact shapes used to record Trust Profile transitions.

---

## 2. Conformance

### 2.1 Conformance Roles

**Requirement class: Companion requirement**

This companion defines the following conformance roles:

1. **Trust Profile Publisher** — publishes machine-readable Trust Profile declarations and metadata budgets.
2. **Trust Profile Verifier** — validates that operational behavior matches declared posture.
3. **Auditor** — an independent party claiming Trellis Auditor conformance. Auditors MUST be able to compare declared posture versus observed control-plane behavior. Auditor obligations are scoped to metadata and control-plane observations; auditors MUST NOT be required to access protected payloads.

A conforming implementation MUST satisfy all requirements applicable to each claimed role.

### 2.2 Profile Conformance

**Requirement class: Companion requirement**

An implementation MAY additionally claim conformance to one or more Standard Profiles defined in §10. When it does, it MUST satisfy all **Profile constraint** requirements for each claimed profile. Standard Profiles MAY be claimed in combination; an implementation claiming multiple profiles MUST ensure that the combined posture remains internally consistent under §4 (Trust Honesty).

---

## 3. Trust Profile Object Semantics

### 3.1 Minimum Object Semantics

**Requirement class: Constitutional semantic**

[Trellis Core] §4.7 names the Trust Profile as a first-class semantic object and delegates its concrete object semantics to this companion, even when a binding chooses the concrete serialization. A Trust Profile object MUST semantically include at least:

1. a profile identifier,
2. a scope or deployment mode identifier,
3. the ordinary-operation readability posture,
4. the reader-held access posture, if any,
5. the delegated compute posture, if any,
6. current-content decryption authorities,
7. historical-content decryption authorities,
8. recovery authorities and recovery conditions,
9. canonical append attestation control authorities,
10. exceptional-access authorities,
11. a statement or reference describing metadata visibility.

Bindings MAY define the concrete wire shape of a Trust Profile object, but they MUST preserve these minimum semantic fields and their meanings.

### 3.2 Disclosure Posture and Assurance

**Requirement class: Cross-reference**

Disclosure posture and assurance level semantics are defined in [WOS Assurance §2] (assurance) and [WOS Assurance §4] (independence invariant). This spec does not restate them; the Trust Profile object carries postures as declared values without reinterpretation.

---

## 4. Trust Honesty and Transitions

### 4.1 Trust Honesty Requirements

**Requirement class: Constitutional semantic**

Trust-honesty semantics are owned by this companion under the delegation from [Trellis Core] §13.4 (Trust and Privacy Disclosure Obligations). For every deployment mode that handles protected content, an implementation:

- MUST publish a Trust Profile,
- MUST state whether ordinary service operation is provider-readable, reader-held, or reader-held with delegated compute,
- MUST state whether the service runtime can access plaintext during ordinary processing,
- MUST state whether recovery can occur without the user,
- MUST state whether delegated compute exposes plaintext to ordinary service components,
- MUST NOT collapse delegated compute access into provider-readable access unless explicitly declared,
- MUST NOT describe a trust posture more strongly than the implementation behavior supports.

### 4.2 Trust Profile Transition Requirements

**Requirement class: Constitutional semantic**

Trust Profile transition semantics are owned by this companion under the delegation from [Trellis Core] §13.4. If an implementation changes custody mode, provider readability posture, recovery semantics, or delegated compute semantics for protected content, it:

- MUST treat that change as a Trust Profile transition,
- MUST make the transition auditable,
- MUST define whether the transition applies prospectively, retrospectively, or both,
- MUST NOT expand from reader-held access or delegated compute access into provider-readable access without such an explicit transition.

The generic named-lifecycle-operation pattern that governs versioned, declared transitions of governed objects is defined in [WOS Governance §2.9] (Schema Upgrade). Trust Profile transitions are the ledger-custody application of that pattern.

### 4.3 Transition Recording

**Requirement class: Companion requirement**

Trust Profile transitions MUST be append-attributable: each transition MUST be recorded as a canonical fact that identifies the actor, the prior profile, the new profile, the effective time, and the policy authority ([Trellis Core] §6.2 invariant 1, §7.1). The minimal canonical fact shape for a transition event follows the Shared Ledger Binding family matrix for trust and access facts. The append-attributability requirement is the ledger-specific declaration of the named-lifecycle-operation pattern in [WOS Governance §2.9].

### 4.4 Mutual Exclusion

**Requirement class: Companion requirement**

A deployment MUST NOT simultaneously claim reader-held and provider-readable posture for the same payload class without an explicit transition record recorded under §4.3.

---

## 5. Baseline Profile Postures

### 5.1 Baseline Postures

**Requirement class: Companion requirement**

An implementation MAY characterize its ordinary operation against one of the following baseline postures:

1. **Reader-held by default.** Payload decryption keys are held by the record subject or their designated reader. Providers MUST NOT hold decryption capability unless explicitly declared. This posture MUST be the default when no other posture is declared.
2. **Provider-readable (explicit, not implied).** Providers MAY decrypt protected payload classes, but only when explicitly declared in the Trust Profile and disclosed in the metadata budget. This MUST NOT be the default.
3. **Tenant-operated key plane.** Key management operates within the tenant's administrative boundary. Cross-tenant key access MUST be declared explicitly.

### 5.2 Mandatory Declarations

**Requirement class: Companion requirement**

Each Trust Profile declaration MUST make explicit:

- who can decrypt what,
- visible metadata classes,
- stable-linkage behavior,
- delegated compute behavior,
- recovery paths,
- destruction and disclosure authority.

---

## 6. Metadata Budget

### 6.1 Metadata Budget Requirement

**Requirement class: Companion requirement**

Each declared Trust Profile MUST include a **metadata budget** scoped by canonical fact family. For each family in scope, the metadata budget MUST document at least:

1. **Visible fields** — which canonical or envelope fields are visible to which observer classes under ordinary operation;
2. **Observer classes** — who MAY observe append metadata, timing, correlation identifiers, or side channels;
3. **Timing and access-pattern leakage** — what timing, frequency, or access-pattern signals the deployment exposes;
4. **Linkage stability** — which identifiers remain stable across sessions, exports, or disclosures;
5. **Delegated-compute effects** — what metadata or plaintext exposure delegated compute introduces relative to nominal posture.

### 6.2 Metadata Budget Form

**Requirement class: Companion requirement**

The metadata budget MAY be presented as a table or a structured object; it MUST be sufficient for an auditor to compare declared leakage against observed behavior.

---

## 7. Verification Posture Classes

### 7.1 Declaration Requirement

**Requirement class: Companion requirement**

Where a deployment uses **tiered verification** for canonical records (for example, structural or ciphertext admission before full payload verification completes), the deployment MUST declare **verification posture classes** and which downstream workflow or release classes each posture MAY feed.

### 7.2 Posture Class Registry

**Requirement class: Companion requirement**

| Posture class | Meaning | Allowed transitions | MUST NOT |
|---|---|---|---|
| `structural_admitted` | Record has passed structural/schema validation only | → `payload_verified` | Feed a high-stakes outcome class |
| `payload_verified` | Payload integrity and schema conformation confirmed | → (terminal for this record) | Be silently escalated from `structural_admitted` |
| `cryptographic_verified` | Full cryptographic verification including signatures and proofs | → (terminal) | Bypass `payload_verified` for high-stakes outcomes |

New posture classes MAY be added only through a companion or registry update that defines the class name, meaning, allowed transitions, and MUST NOT constraints. Posture class names MUST use `snake_case` and MUST be unique within a governed scope.

### 7.3 High-Stakes Outcome Gating

**Requirement class: Companion requirement**

Implementations MUST NOT attach high-stakes outcomes — including adverse action, selective disclosure issuance, commitment-driven analytics, or profile-defined equivalents — to records that have not reached `payload_verified` posture or higher. Escalation of effective verification posture MUST NOT occur silently; it MUST be represented by explicit canonical facts or binding-defined attestations (Shared Ledger Binding S5) when posture affects eligibility for those outcomes.

---

## 8. Profile Declaration Schema

### 8.1 Declaration Shape

**Requirement class: Companion requirement**

A machine-readable Trust Profile declaration MUST include at least the following properties. Bindings MAY extend this shape but MUST NOT remove or rename these properties.

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `profile_id` | string (URI) | Yes | Stable unique identifier for this Trust Profile |
| `decryptor_classes` | array of string | Yes | Observer classes permitted to decrypt protected payloads |
| `metadata_budget_ref` | string (URI) | Yes | Reference to the metadata budget declaration for this profile (§6) |
| `delegated_compute_mode` | enum: `none`, `audit_logged`, `full` | Yes | Whether and how delegated compute is permitted |
| `recovery_mode` | enum: `none`, `emergency_only`, `declared_pathways` | Yes | Whether key recovery pathways exist |
| `destruction_semantics` | enum: `crypto_shredding`, `key_destruction`, `none` | Yes | What destruction guarantees the profile provides |
| `disclosure_authority` | array of string | Yes | Actor classes authorized to issue disclosure artifacts |

### 8.2 Machine-Readable Publication

**Requirement class: Companion requirement**

Deployments MUST publish machine-readable Trust Profile declarations. The declaration MUST be retrievable by the `profile_id` URI and MUST remain immutable for its declared effective window; subsequent changes MUST be recorded under §4.3.

---

## 9. Access Semantics and Profile Honesty Detail

### 9.1 Trust Profile Inheritance Across Profiles, Bindings, and Sidecars

**Requirement class: Companion requirement**

All profiles, bindings, and sidecars inherit the active Trust Profile.

A profile, binding, or sidecar:

- MUST remain consistent with the active Trust Profile,
- MUST distinguish provider-readable access, reader-held access, and delegated compute access when protected content is involved,
- MUST NOT imply stronger confidentiality, weaker provider visibility, or weaker recovery capability than the active Trust Profile supports,
- MUST NOT use profile-local, binding-local, or sidecar-local wording to weaken or bypass Trust Profile requirements.

### 9.2 Profile-Scoped Export and View Honesty

**Requirement class: Companion requirement**

Any profile-scoped export, sidecar-scoped export, audience-specific view, or family-specific presentation:

- MUST preserve the distinction between author-originated facts, canonical records, canonical append attestations, and later disclosure or export artifacts,
- MUST preserve provenance distinctions even when presenting a profile-specific timeline, delta history, or interpretation layer,
- MUST NOT imply broader workflow, governance, custody, compliance, or disclosure coverage than its declared scope actually includes.

### 9.3 Access Categories

**Requirement class: Companion requirement**

Implementations handling protected content MUST distinguish the following forms of access:

- **Provider-readable access.** The service operator or ordinary service-side components can decrypt content during ordinary operation.
- **Reader-held access.** An explicitly authorized human or tenant-side principal can decrypt content within its scope.
- **Delegated compute access.** A specific compute agent or model is granted scoped access to process content for a specific purpose.

A conforming implementation MUST describe these categories consistently with its actual behavior. (Note: WOS Assurance does not currently define a generic access-category taxonomy at the substrate layer; this spec is the normative home for these custody-mode definitions in the Trellis distributed-trust binding.)

### 9.4 Profile Honesty Detail

**Requirement class: Companion requirement**

A conforming implementation:

- MUST disclose whether provider-readable access exists in ordinary operation,
- MUST disclose whether delegated compute is provider-operated, tenant-operated, client-side, or otherwise isolated,
- MUST disclose what metadata remains visible to the service or other observers,
- MUST NOT describe a trust posture more strongly than those facts support.

---

## 10. Standard Profiles

### 10.1 Offline Authoring Profile

**Requirement class: Profile constraint**

An implementation conforming to the Offline Authoring Profile:

- MUST permit author-originated facts to exist prior to canonical append,
- MUST preserve authored authentication semantics across delayed submission,
- MUST preserve authored time or authored context where available,
- MUST distinguish authored time from canonical append time unless the implementation can establish equivalence explicitly,
- MUST define how local pending facts remain non-canonical until admitted,
- MUST define duplicate-submission and replay behavior for delayed submissions,
- MUST preserve provenance distinctions among authored fact, canonical record, and canonical append attestation.

A conforming Offline Authoring Profile SHOULD:

- minimize local pending state to what is necessary for user-authoring continuity,
- avoid treating broad local collaboration state as canonical truth,
- define how rejected offline submissions are surfaced without implying canonical admission.

Such a profile MUST preserve the core state machine and provenance distinctions of [Trellis Core] §7.3 (Fact Admission State Machine).

#### 10.1.1 Offline Submission Semantics

**Requirement class: Profile constraint**

Offline-originated facts MAY be submitted after delay. If accepted, the implementation:

- MUST preserve authored authentication semantics,
- MUST distinguish later admission and later append attestation from earlier authorship,
- MUST NOT imply that canonical append time is identical to authorship time unless that equivalence is established.

#### 10.1.2 Pending Local State

**Requirement class: Profile constraint**

If local pending state exists before admission, that state:

- MUST remain non-canonical,
- MUST NOT define alternate canonical order,
- SHOULD remain separable from draft-collaboration state,
- MUST be transformable into submitted facts without silently rewriting prior authored facts.

### 10.2 Reader-Held Decryption Profile

**Requirement class: Profile constraint**

An implementation conforming to the Reader-Held Decryption Profile:

- MUST declare that ordinary service operation does not require general plaintext access for declared protected content,
- MUST identify which principals may decrypt within scope,
- MUST identify whether the provider can assist recovery,
- MUST remain consistent with the active Trust Profile,
- MUST distinguish reader-held access from provider-readable access,
- MUST distinguish reader-held access from delegated compute access.

#### 10.2.1 Reader-Held Access Semantics

**Requirement class: Companion requirement**

Reader-held access means an explicitly authorized human or tenant-side principal can decrypt content within its scope.

Reader-held access:

- MUST NOT be described as provider-readable ordinary operation,
- MAY coexist with recovery assistance if the Trust Profile declares it honestly,
- MAY coexist with delegated compute if delegation remains explicit, scoped, and auditable.

### 10.3 Delegated Compute Profile

**Requirement class: Profile constraint**

An implementation conforming to the Delegated Compute Profile:

- MUST distinguish delegated compute access from provider-readable access,
- MUST make delegated compute explicit, attributable, and auditable,
- MUST define delegation scope,
- MUST define delegation authority,
- SHOULD define purpose bounds or time bounds,
- MUST NOT imply that delegated compute grants general service readability,
- MUST define what audit facts or audit events are emitted for delegation and use.

If workflow or adjudication materially relies on delegated compute output, the profile MUST require either:

- canonical representation of that output as a canonical fact, or
- a canonical reference to a stable output artifact.

#### 10.3.1 Delegated Compute Access Semantics

**Requirement class: Companion requirement**

Delegated compute access is a specific grant under which a compute agent or model is allowed to process declared content for a declared purpose.

A delegated compute grant:

- MUST be explicit,
- MUST be attributable to a principal, policy authority, or comparable authority,
- MUST be scoped to declared content or classes of content,
- SHOULD be time-bounded or purpose-bounded,
- MUST be auditable,
- MUST NOT be interpreted as conferring standing plaintext access to the ordinary service runtime.

#### 10.3.2 Compute Output Reliance

**Requirement class: Companion requirement**

If workflow, policy, adjudication, access decisions, or materially consequential system actions rely on delegated compute output, the implementation:

- MUST record that output as a canonical fact or maintain a canonical reference to a stable output artifact,
- MUST preserve an auditable link to the authorizing principal,
- MUST preserve an auditable link to the compute agent identity,
- MUST preserve an auditable link to the scope of delegated access relevant to that output,
- MUST define whether the relied-upon output is advisory, recommendatory, or decision-contributory.

### 10.4 Disclosure and Export Profile

**Requirement class: Profile constraint**

An implementation conforming to the Disclosure and Export Profile:

- MUST support at least one verifiable disclosure or export form,
- MUST preserve the distinction between author-originated facts, canonical records, canonical append attestations, and later disclosure or export artifacts,
- MUST define which claims remain verifiable when payload readability is absent,
- MUST define profile-specific audience scope where relevant,
- MUST remain subordinate to the export guarantees of [Trellis Core] §9 and to [Export-Verification-Package].

#### 10.4.1 Export Claim Classes

**Requirement class: Companion requirement**

A Disclosure and Export Profile SHOULD state which of the following claim classes are verifiable within that profile:

- authorship claims,
- append or inclusion claims,
- payload-integrity claims,
- authorization-history claims,
- disclosure claims,
- lifecycle or compliance claims where included by scope.

An implementation MUST NOT imply that an export supports a claim class unless the export contains sufficient material to verify that class.

#### 10.4.2 Selective Disclosure Discipline

**Requirement class: Companion requirement**

Selective disclosure SHOULD occur through disclosure or export artifacts rather than by overloading canonical records.

A disclosure-oriented artifact:

- MAY present an audience-specific subset or presentation,
- MUST preserve provenance distinctions,
- MUST NOT be treated as a rewrite of canonical truth.

### 10.5 User-Held Record Reuse (Cross-Reference)

**Requirement class: Cross-reference**

User-held record reuse is a data-contract concern, not a custody-mode binding. See [Formspec Respondent Ledger §15A.1 Profile A] for the normative definition.

### 10.6 Respondent History (Cross-Reference)

**Requirement class: Cross-reference**

Respondent-history projection is a data-contract concern, not a custody-mode binding. See [Formspec Respondent Ledger §6.6A] for the normative definition.

---

## 11. Relationship to Export Claim Classes

**Requirement class: Companion requirement**

Trust Profile declarations determine which export manifest claim classes a deployment is authorized to assert. The following table maps profile fields to export claim-class eligibility as defined in [Disclosure-Manifest] S4.

| Profile field | Export claim class affected | Qualification |
|---|---|---|
| `decryptor_classes` | payload-integrity | Verifiable only if `decryptor_classes` include the verifier |
| `destruction_semantics` | authorization-history | Destruction claims require `crypto_shredding` or `key_destruction` semantics |
| `recovery_mode` | disclosure-policy | Recovery-mode declarations affect disclosure trustworthiness |
| `delegated_compute_mode` | payload-integrity | Delegated compute exposure MUST be disclosed in the manifest |

---

## 12. Audit and Conformance Hooks

### 12.1 Auditor Access and Obligations

**Requirement class: Companion requirement**

- Deployments MUST publish machine-readable Trust Profile declarations.
- Independent auditors claiming Trellis Auditor conformance MUST be able to compare declared posture versus observed control-plane behavior.
- Auditors MAY observe control-plane metadata and timing; auditors MUST NOT be required to access protected payloads.
- Any mismatch between declaration and operation MUST be reported as trust-honesty nonconformance.

### 12.2 Operational Trust Disclosure

**Requirement class: Companion requirement**

- Trust claims MUST be consistent with declared decryptability, delegated compute behavior, recovery behavior, and destruction semantics.
- Metadata-leakage characteristics MUST be declared at profile level and MUST NOT be omitted from trust posture statements.

---

## 13. Example Profiles (Non-normative)

The examples in this section illustrate how Trust Profile declarations may be composed for common custody patterns. They are not conformance material and MUST NOT override actual Trust Profile declarations published by a deployment.

### 13.1 Purpose

**Requirement class: Non-normative guidance**

This section provides concrete examples of trust and custody postures without altering the semantic honesty requirements of [Trellis Core] or the normative sections of this companion. Implementors MAY use these examples as starting points when composing a Trust Profile declaration; auditors MAY use them as reference points when comparing a declared posture against a common pattern.

### 13.2 Example Profile A — Provider-Readable Custodial

**Requirement class: Non-normative guidance**

Characteristics:

- ordinary service operation is provider-readable,
- the provider can decrypt current content in ordinary operation,
- historical content remains provider-readable unless separately sealed or destroyed,
- recovery may occur without the user if the deployment declares that capability,
- delegated compute, if present, is not confidentiality-improving because provider-readable operation already exists.

A profile using this posture MUST say so plainly and MUST NOT imply provider blindness.

### 13.3 Example Profile B — Reader-Held with Recovery Assistance

**Requirement class: Non-normative guidance**

Characteristics:

- ordinary service operation is not generally provider-readable,
- explicitly authorized readers can decrypt within scope,
- a recovery authority may assist recovery under declared conditions,
- recovery assistance does not by itself imply ordinary provider-readable operation,
- the Trust Profile MUST describe who can assist recovery and under what conditions.

### 13.4 Example Profile C — Reader-Held with Tenant-Operated Delegated Compute

**Requirement class: Non-normative guidance**

Characteristics:

- ordinary service operation is not generally provider-readable,
- explicitly authorized readers decrypt within scope,
- delegated compute is possible only through explicit, scoped delegation,
- delegated compute is tenant-operated or otherwise isolated from the ordinary service runtime,
- the Trust Profile MUST state whether plaintext becomes visible to any provider-operated components during delegation.

### 13.5 Example Profile D — Threshold-Assisted Custody

**Requirement class: Non-normative guidance**

Characteristics:

- decryption or recovery requires cooperation by multiple parties or custodians,
- no single ordinary operator is sufficient to recover protected content,
- recovery conditions, quorum thresholds, and exceptional access posture MUST be declared,
- threshold participation MUST NOT be described more strongly than the actual recovery process supports.

### 13.6 Example Profile E — Delegated Organizational Custody

**Requirement class: Non-normative guidance**

Characteristics:

- an organization, tenant authority, or delegated administrative authority controls recovery or access posture,
- authorized organizational actors may manage access grants, revocations, and delegation,
- the Trust Profile MUST identify the scope of organizational authority and any exceptional-access controls,
- this posture MUST still distinguish provider-readable access from organization-controlled access where they differ.

### 13.7 Example Comparison Guidance

**Requirement class: Non-normative guidance**

These examples are useful when documenting tradeoffs among:

- who can read current content,
- who can read historical content,
- whether recovery is possible without the user,
- whether delegated compute exposes plaintext to ordinary service components,
- who controls canonical append attestation,
- who can block or administer exceptional access.

---

## 14. Security and Privacy Considerations

**Requirement class: Companion requirement**

- Trust Profile declarations are themselves canonical facts and MUST NOT be altered after append ([Trellis Core] §6.2 invariant 1).
- Escalation of effective verification posture MUST NOT occur silently; posture transitions MUST be represented by explicit canonical facts (§7.3, Shared Ledger Binding S5).
- Auditor access is bounded: auditors MAY observe metadata and timing but MUST NOT be required to access protected payloads (§12.1).
- Metadata-budget declarations MUST account for all observable side channels, not just direct data access (§6).
- A Trust Profile MUST NOT describe a posture more strongly than the deployment's actual behavior supports (§4.1). A deployment that discovers such overstatement MUST treat the correction as a Trust Profile transition under §4.2 and MUST NOT retroactively rewrite prior declarations.

---

## 15. Normative References

- **[Trellis Core]** — Trellis Core Specification v0.1 (`trellis/specs/core/trellis-core.md`).
- **[Key-Lifecycle-Operating-Model]** — Trellis Companion: Key Lifecycle Operating Model v0.1 (`trellis/specs/trust/key-lifecycle-operating-model.md`).
- **[Disclosure-Manifest]** — Trellis Companion: Disclosure Manifest v0.1 (`trellis/specs/export/disclosure-manifest.md`).
- **[Export-Verification-Package]** — Trellis Companion: Export Verification Package v0.1 (`trellis/specs/export/export-verification-package.md`).
- **[Shared Ledger Binding]** — Trellis Binding: Shared Ledger Binding v0.1 (`trellis/specs/core/shared-ledger-binding.md`).
- **[WOS Kernel §10.5]** — WOS Kernel Specification, §10 Seams, S10.5 `custodyHook` (`wos-spec/specs/kernel/spec.md`).
- **[WOS Assurance §2]**, **[WOS Assurance §4]** — WOS Assurance Specification, §2 Assurance Levels, §4 Invariant 6 (Disclosure Posture Is Not Assurance Level) (`wos-spec/specs/assurance/assurance.md`).
- **[WOS Governance §2.9]** — WOS Workflow Governance Specification, §2.9 Schema Upgrade (`wos-spec/specs/governance/workflow-governance.md`).
- **[Formspec Respondent Ledger §6.6A / §15A.1 Profile A]** — Formspec Respondent Ledger Specification (`specs/audit/respondent-ledger-spec.md`).
- **[RFC 2119]** — Bradner, S., "Key words for use in RFCs to Indicate Requirement Levels", BCP 14, RFC 2119, March 1997.
- **[RFC 8174]** — Leiba, B., "Ambiguity of Uppercase vs Lowercase in RFC 2119 Key Words", BCP 14, RFC 8174, May 2017.
- **[RFC 8259]** — Bray, T., Ed., "The JavaScript Object Notation (JSON) Data Interchange Format", STD 90, RFC 8259, December 2017.
- **[RFC 3986]** — Berners-Lee, T., Fielding, R., Masinter, L., "Uniform Resource Identifier (URI): Generic Syntax", STD 66, RFC 3986, January 2005.
