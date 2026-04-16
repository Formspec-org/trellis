---
title: Trellis Companion — Disclosure Manifest
version: 0.1.0-draft.2
date: 2026-04-14
status: draft
---

# Trellis Companion — Disclosure Manifest v0.1

**Version:** 0.1.0-draft.2
**Date:** 2026-04-14
**Editors:** Formspec Working Group
**Companion to:** Trellis Core v0.1

---

## Status of This Document

This document is a **draft specification**. It is a companion to the Trellis Core specification and does not modify Formspec or WOS processing semantics. Implementors are encouraged to experiment with this specification and provide feedback, but MUST NOT treat it as stable for production use until a 1.0.0 release is published.

## Abstract

The Disclosure Manifest companion defines audience-scoped disclosure semantics as first-class release artifacts, separate from canonical records and their append attestations. It specifies a minimum manifest field set, the enumerated disclosure postures, claim-class declarations, selective disclosure discipline, coverage honesty, and the boundary between the disclosure manifest and the Export Verification Package. This companion adds disclosure semantics to the Trellis export layer defined in Trellis Core (S5, S8) and refined in Export Verification Package (S3). It does not define Formspec or WOS semantics.

## Table of Contents

1. Introduction
2. Conventions and Terminology
3. Conformance
4. Terminology and Definitions
5. Manifest Structure
6. Audience Scope Declaration
7. Disclosure Posture
8. Claim Class Declaration
9. Selective Disclosure Discipline
10. Coverage Honesty
11. Posture and Assurance Non-Conflation
12. Relationship to the Export Verification Package
13. Security and Privacy Considerations
14. Interoperability Direction
15. Cross-References

---

## 1. Introduction

### 1.1 Scope

This companion defines audience-scoped disclosure artifacts — called *disclosure manifests* — that govern **what** canonical or derived material is disclosed and **to whom**. It defines the minimum structural, semantic, and honesty requirements a conforming disclosure manifest MUST satisfy.

This companion does not define:

- Formspec Definition, Response, FEL, or validation semantics (Trellis Core S1.2).
- WOS lifecycle, case state, or governance enforcement semantics (Trellis Core S9).
- Canonical ledger, append order, or canonical hash construction semantics (Trellis Core S5–S7).
- Offline verifiability of exported material (Export Verification Package S3).

### 1.2 Relationship to Trellis Core

A disclosure manifest is a species of the *disclosure or export artifact* object class defined in Trellis Core (S4.1). Disclosure manifests MUST preserve the canonical/derived distinctions of Trellis Core S4.2, S5, and S8, and MUST NOT be treated as canonical truth or as canonical rewrites.

### 1.3 Additive Invariant

A Formspec or WOS processor that ignores all disclosure manifests remains fully conformant to its respective specification and produces identical data, validation, and lifecycle results. Disclosure manifests add disclosure-layer semantics; they do not modify processing semantics in either system.

---

## 2. Conventions and Terminology

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in BCP 14 [RFC 2119] [RFC 8174] when, and only when, they appear in ALL CAPITALS, as shown here.

JSON syntax and data types are as defined in [RFC 8259]. URI syntax is as defined in [RFC 3986].

Requirement-class markers used in this companion:

- **Constitutional semantic** — preserved from Trellis Core; cited, not redefined here.
- **Profile constraint** — attaches to the Disclosure and Export Profile (Trellis Core S2.2).
- **Companion requirement** — reusable subordinate requirement introduced by this companion.

---

## 3. Conformance

### 3.1 Conformance Roles

This companion defines the following conformance roles:

1. **Disclosure Producer** — creates disclosure manifests and audience-scoped disclosure artifacts. MUST satisfy all requirements in Sections 5–11.
2. **Disclosure Verifier** — validates that a received disclosure manifest is well-formed, that declared claim classes are supported by the material referenced or included, and that audience scope, posture, and coverage declarations are internally consistent. MUST satisfy verification expectations implied by Sections 5, 7, 8, and 10.
3. **Disclosure Consumer (Audience)** — receives a disclosure manifest and relies on its declared scope and posture. This role is non-normative in this companion; audiences are not required to implement verification, but SHOULD apply Section 6.3 to confirm they are in declared scope before acting on disclosed material.

A conforming implementation MUST satisfy all requirements applicable to each claimed role.

### 3.2 Profile Binding

A Disclosure Producer or Disclosure Verifier that claims the **Disclosure and Export Profile** (Trellis Core S2.2) MUST also satisfy the profile-level requirements of Unified Ledger Companion S2.4 as restated and refined in this document.

---

## 4. Terminology and Definitions

### 4.1 Disclosure or Export Artifact

**Requirement class: Constitutional semantic** (Trellis Core S4.6)

An audience-specific package, presentation, or view assembled for portability, review, or selective disclosure. A disclosure manifest is a *disclosure or export artifact*. It is distinct from the canonical record it may reference and from any canonical append attestation that binds that record into canonical order.

### 4.2 Disclosure Manifest

A structured declaration, produced by a Disclosure Producer, that specifies:

1. **what** canonical or derived material is being disclosed,
2. **to whom** it is being disclosed (audience scope),
3. **under what posture** (disclosure posture, claim classes, redaction declarations),
4. **with what provenance** back to canonical records,

and that governs interpretation of any accompanying disclosed payload.

### 4.3 Disclosure Posture

The `disclosurePosture` field carries one of the values enumerated in [Formspec Respondent Ledger §6.6 `privacyTier`]. This spec does not redeclare the enumeration.

### 4.4 Subject Continuity

Subject continuity semantics are defined in [Formspec Respondent Ledger §6.6A — Identity and implementation decoupling (subject continuity semantics)]. This manifest carries a continuity reference as an opaque string per that definition.

### 4.5 Coverage Honesty

**Requirement class: Profile constraint** (Unified Ledger Companion §2.6.2, §3.0.2)

A property of a disclosure manifest under which the manifest MUST NOT imply broader workflow, governance, custody, compliance, or disclosure coverage than its declared audience scope, claim classes, and included material actually provide.

### 4.6 Selective Disclosure

**Requirement class: Companion requirement** (Unified Ledger Companion §2.4.2)

The production of an audience-specific subset, projection, or presentation of canonical or derived material, without rewriting canonical records and while preserving provenance distinctions (Trellis Core S12.4, Export Verification Package S3.7).

### 4.7 Posture and Assurance Non-Conflation

The independence of disclosure posture and assurance level is defined in [WOS Assurance §4 Invariant 6]. This manifest's fields for `disclosurePosture` and `assuranceLevel` are independently declared and MUST NOT be coupled.

### 4.8 Controlled Vocabulary

Normative sections of this document use the controlled terms cited above. Where an upstream controlled term exists (WOS Assurance, Formspec Respondent Ledger, Trellis Core), that term MUST be used in preference to synonyms.

---

## 5. Manifest Structure

### 5.1 Minimum Required Fields

**Requirement class: Companion requirement**

A conforming disclosure manifest MUST include, at minimum, the following fields. The concrete wire shape (JSON property names, encoding) is a binding concern; the semantic fields below are normative.

1. **Format and version identifier** — the manifest format identifier and version (for example `trellis-disclosure-manifest/0.1`), sufficient for a verifier to select the applicable interpretation rules.
2. **Manifest identity and production metadata** — a stable manifest identifier, production timestamp, and producing principal or producing-service reference sufficient to attribute the manifest.
3. **Declared audience scope** — see Section 6.
4. **Disclosure posture** — exactly one value from the `privacyTier` enumeration in [Formspec Respondent Ledger §6.6], plus any binding-registered extension values that do not weaken the enumerated semantics.
5. **Declared claim classes** — the set of claim classes the manifest asserts are verifiable within its disclosed material (Section 8).
6. **Scope boundary statement** — a prose or structured declaration of what is in scope and what is explicitly out of scope, sufficient to satisfy the coverage honesty obligation (Section 10).
7. **Included canonical records** — references to each canonical record that is disclosed, at the level of detail the declared posture permits.
8. **Omitted canonical records** — a declaration of records or classes of records that exist within the declared scope but are intentionally withheld, sufficient for a verifier to recognize that the disclosure is a subset (Section 9).
9. **Redaction and readability declarations** — for each disclosed record or payload, a declaration of whether content is readable, encrypted, redacted, or intentionally omitted, and whether any redaction is irreversible at the disclosure level (Section 13.3).
10. **Provenance distinctions preserved** — explicit binding between disclosed claims and the canonical records, canonical append attestations, and earlier author-originated facts they derive from (Trellis Core S12.4).
11. **Posture–assurance non-conflation declaration** — a declaration that MUST NOT bind disclosure posture to assurance level and MUST NOT be phrased in a way that implies such binding (Section 4.7, Section 11).
12. **Relationship to any accompanying Export Verification Package** — see Section 12.

### 5.2 Canonical Rewrite Prohibition

**Requirement class: Companion requirement**

A disclosure manifest MUST NOT be represented as, serialized as, or treated as a canonical record, a canonical rewrite, or a canonical append attestation. A disclosure manifest references canonical records; it does not replace them.

### 5.3 Manifest Integrity

**Requirement class: Companion requirement**

A disclosure manifest SHOULD be integrity-bound to its referenced material — for example, by content-addressed references, signed envelopes, or inclusion in an Export Verification Package (Section 12). Integrity binding mechanisms are a binding concern and are not normatively constrained by this companion beyond the requirement that any integrity binding MUST NOT be claimed stronger than it is.

---

## 6. Audience Scope Declaration

### 6.1 Required Audience-Scope Fields

**Requirement class: Companion requirement**

A disclosure manifest's declared audience scope MUST include, at minimum:

1. **Audience identifier or audience class** — the intended recipient(s), named as individuals, roles, organizations, tenant scopes, or publicly declared audience classes.
2. **Audience qualification criteria** — the criteria an audience MUST satisfy to be within scope (for example, role, custody relationship, regulatory authority, or public availability), expressed precisely enough for an audience to self-check.
3. **Scope temporal bounds** — the time window during which the declared scope is valid, or an explicit declaration that no temporal bound applies.
4. **Scope purpose** — the declared disclosure purpose or permitted uses, where the profile or binding requires it.

### 6.2 Scope Binding to the Manifest

**Requirement class: Companion requirement**

The audience scope declaration MUST be bound to the manifest such that altering the audience scope alters the manifest's identity or integrity binding. Audience scope MUST NOT be an external, mutable reference that could be changed after manifest production without detection.

### 6.3 Audience Self-Verification

**Requirement class: Companion requirement**

A disclosure manifest MUST express audience scope in terms that allow an audience to determine, from the manifest alone or from the manifest plus declared external references, whether the audience is within declared scope. A Disclosure Verifier MUST be able to:

1. read the audience scope declaration,
2. evaluate the declared qualification criteria, and
3. report whether a named audience is, is not, or cannot be determined to be within scope,

without requiring access to derived runtime state, canonical append service internals, or non-public operator knowledge beyond what the manifest declares.

---

## 7. Disclosure Posture

### 7.1 Enumerated Posture Values

**Requirement class: Companion requirement**

A disclosure manifest MUST declare exactly one disclosure posture value from the `privacyTier` enumeration in [Formspec Respondent Ledger §6.6] (`anonymous`, `pseudonymous`, `identified`, `public`).

Profiles or bindings MAY extend this enumeration with additional values that specialize rather than redefine the upstream base values. An extension posture MUST declare which base value it specializes.

The independence of `disclosurePosture` and `assuranceLevel` is governed by [WOS Assurance §4 Invariant 6] and [Formspec Respondent Ledger §6.7]; this manifest MUST preserve that independence and MUST NOT couple the two fields.

### 7.2 Posture Honesty

**Requirement class: Companion requirement**

A disclosure manifest:

- MUST NOT declare a posture weaker than the material actually discloses.
- MUST NOT declare a posture stronger than the material actually disclosed supports (for example, declaring `anonymous` while disclosing a subject continuity reference).
- MUST NOT describe the same disclosure as different postures to different audiences unless each distinct posture is bound to a distinct manifest.

### 7.3 Subject Continuity Compatibility

**Requirement class: Companion requirement**

A `pseudonymous` manifest MAY carry a subject continuity reference per [Formspec Respondent Ledger §6.6A]. An `anonymous` manifest MUST NOT carry any subject continuity reference or any other identifier that is demonstrably linkable to a subject within declared audience scope. An `identified` or `public` manifest MAY carry identifying material consistent with its declared posture.

---

## 8. Claim Class Declaration

### 8.1 Claim Class Taxonomy

**Requirement class: Companion requirement**

A conforming disclosure manifest MUST declare which claim classes are verifiable within its disclosed material. The manifest's ledger-anchored claim classes are:

1. **Authorship claims** — that a disclosed fact was authored by a specific principal.
2. **Append or inclusion claims** — that a disclosed canonical record was admitted into canonical order (Trellis Core S6).
3. **Payload-integrity claims** — that a disclosed payload matches the canonical record it is bound to.

Authorization-history, disclosure-policy, lifecycle, and compliance claim classes are governed by their respective upstream specifications and MUST be cited from those sources rather than redeclared here. Profiles or bindings MAY define additional claim classes.

### 8.2 Verifiability Honesty

**Requirement class: Companion requirement** (Unified Ledger Companion §2.4.1)

A disclosure manifest MUST NOT imply support for a claim class unless the disclosure contains sufficient material to verify that class — whether included in the manifest, included in an accompanying Export Verification Package (Section 12), or referenced by an immutable, content-addressed, or otherwise durably resolvable reference.

If any declared claim class cannot be verified with the material available to the declared audience, the manifest MUST declare that limitation explicitly.

---

## 9. Selective Disclosure Discipline

**Requirement class: Companion requirement** (Unified Ledger Companion §2.4.2)

### 9.1 Audience-Specific Subsets

A disclosure manifest MAY present an audience-specific subset, projection, or presentation of canonical or derived material.

### 9.2 Provenance Preservation

A disclosure manifest MUST preserve the distinction among:

- author-originated facts,
- canonical records,
- canonical append attestations,
- derived artifacts,
- later disclosure or export artifacts,

for every disclosed item (Trellis Core S4.1, S12.4).

### 9.3 No Canonical Rewrite

A disclosure manifest MUST NOT be treated as a rewrite of canonical truth, MUST NOT overload canonical records with audience-specific content, and MUST NOT cause the Disclosure Producer or any downstream system to mutate a canonical record on the basis of a disclosure decision.

### 9.4 Preference for Artifact-Level Disclosure

Selective disclosure SHOULD occur through disclosure or export artifacts (this companion, Export Verification Package, profile-specific artifacts) rather than by overloading canonical records with audience-specific variants.

---

## 10. Coverage Honesty

**Requirement class: Profile constraint** (Unified Ledger Companion §2.6.2, §3.0.2)

### 10.1 Coverage Honesty Obligation

A disclosure manifest MUST NOT imply broader workflow, governance, custody, compliance, or disclosure coverage than its declared audience scope, claim classes, and included material actually provide.

### 10.2 Explicit Scope Boundary

A disclosure manifest MUST include a scope boundary statement (Section 5.1 field 6) that declares, at minimum:

- what is in scope,
- what is explicitly out of scope,
- what claim classes are verifiable within scope,
- what claim classes are NOT verifiable within the manifest as produced (where relevant).

### 10.3 Presentation Coverage

A manifest's audience-facing presentation, titles, and summary language MUST NOT imply coverage the manifest does not actually provide, including implied coverage through profile labels, branding, or non-normative prose.

### 10.4 Cross-Profile Coverage

Where a disclosure manifest is assembled within a profile that itself has a narrow declared scope (for example, the Respondent History Profile, Unified Ledger Companion §2.6), the manifest inherits that profile's coverage boundary and MUST NOT imply broader coverage by virtue of being a disclosure artifact.

---

## 11. Posture and Assurance Non-Conflation

### 11.1 Non-Conflation Obligation

This manifest MUST preserve the independence declared in [WOS Assurance §4 Invariant 6]. Implementations producing this manifest MUST NOT encode `disclosurePosture` and `assuranceLevel` as a joint value.

### 11.2 Presentation Discipline

Audience-facing prose, field labels, and profile descriptors in a disclosure manifest MUST NOT be phrased in a way that implies posture–assurance equivalence (for example, "high-assurance identified disclosure" SHOULD NOT be used as if the two qualifiers are mutually required).

---

## 12. Relationship to the Export Verification Package

### 12.1 Manifest/Payload Boundary

**Requirement class: Companion requirement**

This companion and the Export Verification Package companion govern distinct concerns and MUST NOT be conflated:

- **Disclosure Manifest** governs **what is disclosed** and **to whom**: audience scope, disclosure posture, declared claim classes, selective disclosure discipline, coverage honesty, and provenance preservation at the disclosure layer.
- **Export Verification Package** governs **offline verifiability** of the canonical and disclosed material: required package members, payload readability declarations, trust-profile carriage, verification mode, cross-implementation verification, and provenance distinction at the package layer (Export Verification Package S3–S7).

### 12.2 Composition

**Requirement class: Companion requirement**

A disclosure manifest MAY be included as a member of an Export Verification Package. When included:

1. The manifest's declared claim classes (Section 8) MUST align with the package's declared claim classes (Export Verification Package S3.4).
2. The manifest's disclosure posture and audience scope MUST be preserved by the package wrapper; the package MUST NOT restate, weaken, or broaden them.
3. The package's payload readability and redaction declarations (Export Verification Package S3.2) MUST be consistent with the manifest's redaction and readability declarations (Section 5.1 field 9).

### 12.3 Standalone Manifests

**Requirement class: Companion requirement**

A disclosure manifest MAY be produced standalone, without an accompanying Export Verification Package. When produced standalone:

1. The manifest MUST NOT claim offline-verifiable canonical integrity for referenced canonical records beyond what the manifest's own integrity binding supports.
2. The manifest MUST NOT declare claim classes (Section 8) that require offline canonical verification material unless that material is included by immutable, content-addressed, or otherwise durably resolvable reference that the declared audience can resolve.
3. The manifest MUST declare, in its scope boundary statement (Section 10.2), that offline canonical integrity verification is not provided by the manifest alone.

Standalone manifests remain subject to all other requirements of this companion, including selective disclosure discipline (Section 9), coverage honesty (Section 10), and posture/assurance non-conflation (Section 11).

### 12.4 Provenance Distinction Across the Boundary

**Requirement class: Constitutional semantic** (Trellis Core S12.4)

Whether bundled or standalone, a disclosure manifest MUST preserve the Trellis Core S12.4 distinction among author-originated facts, canonical records, canonical append attestations, and later disclosure or export artifacts. This distinction MUST NOT be collapsed at either the manifest layer or the package layer.

---

## 13. Security and Privacy Considerations

Generic privacy-disclosure considerations are governed by [WOS Assurance §6]. The subsections below are scoped to ledger-manifest-specific concerns and do not restate upstream obligations.

### 13.1 Linkability and Correlation

Selective disclosure under a pseudonymous posture does not eliminate linkability. Implementers SHOULD consider:

- cross-manifest correlation when multiple manifests carry the same or derivable subject continuity references (per [Formspec Respondent Ledger §6.6A]),
- time-pattern correlation across manifests produced for overlapping audiences,
- audience-collusion correlation when distinct audiences compare manifests received independently,
- re-identification risk from payload content, scope declarations, or claim-class metadata even when identifiers are withheld.

A Disclosure Producer SHOULD minimize linkable metadata consistent with the selective disclosure discipline of Section 9 and the coverage honesty obligation of Section 10.

### 13.2 Metadata Leakage

Manifest-level metadata (audience identifiers, temporal bounds, scope purpose, claim-class declarations) is itself disclosed to the audience and potentially to observers of the manifest's distribution path. Implementers SHOULD limit manifest metadata to what the declared posture and claim classes require, in line with the metadata-minimization discipline of Trellis Core (S10; Unified Ledger Companion §3.9.1).

### 13.3 Redaction Irreversibility

Redaction declarations at the disclosure level (Section 5.1 field 9) MUST be irreversible at the disclosure layer. Disclosed artifacts MUST NOT contain reversible redaction markers (for example, redacted content hidden behind presentation layers but still present in the artifact bytes, or redaction tokens that disclose redacted-length oracles to an attacker) that could be exploited to reconstruct redacted content. This anchors and strengthens the general redaction requirements of Export Verification Package S3.2.

### 13.4 Side-Channel Existence Leakage

Selective disclosure MUST NOT leak the existence of undisclosed claims through side channels. In particular:

- inclusion proofs SHOULD NOT reveal undisclosed record counts or positions beyond what is required for the declared claim classes,
- cross-audience manifests SHOULD NOT leak, via shared proof material, that an undisclosed record exists within a different audience's disclosed scope,
- omitted-record declarations (Section 5.1 field 8) SHOULD be stated at a granularity consistent with the declared posture — an **anonymous** manifest MUST NOT reveal subject-linked omissions in omitted-record metadata.

### 13.5 Audience Independence

Disclosure manifests for different audiences MUST be independently producible without requiring access to other audiences' disclosure policies, audience identifiers, or audience-specific material.

### 13.6 Posture/Assurance Non-Conflation in Deployed Systems

Implementers SHOULD review audience-facing UI, API field names, and downstream consumer documentation to ensure they do not reintroduce posture–assurance conflation (Section 11) that the manifest itself prohibits.

### 13.7 Selective Disclosure Discipline in Practice

All requirements of Section 9 (selective disclosure discipline) and Section 10 (coverage honesty) are security- and privacy-relevant and SHOULD be treated as such during threat modeling, not only as governance obligations.

---

## 14. Interoperability Direction

### 14.1 Near-Term Interoperability

SD-JWT [RFC 9535 / draft-ietf-oauth-selective-disclosure-jwt] and Verifiable Credentials profile paths are the preferred near-term interoperability targets for a binding of this companion. These paths align well with the audience-scope, posture, and claim-class model of Sections 6–8.

### 14.2 Later-Phase Seams

Advanced privacy-preserving disclosure mechanisms, including BBS+-style selective disclosure signatures and other unlinkable-credential schemes, remain later-phase seams and are not required for baseline conformance (Trellis Core S10.1).

---

## 15. Cross-References

Normative cross-references:

- **WOS Assurance §4 Invariant 6** — `../../../wos-spec/specs/assurance/assurance.md`. Constitutional independence of disclosure posture and assurance level (Sections 4.7, 7.1, 11).
- **WOS Assurance §6** — same path. Generic privacy-disclosure obligations (Section 13).
- **Formspec Respondent Ledger §6.6 `privacyTier`** — `../../../specs/audit/respondent-ledger-spec.md`. Canonical disclosure-posture enumeration (Sections 4.3, 5.1 field 4, 7.1).
- **Formspec Respondent Ledger §6.6A** — same path. Subject continuity definition (Sections 4.4, 7.3, 13.1).
- **Formspec Respondent Ledger §6.7** — same path. Disclosure tier and assurance independence (Section 7.1).
- **Trellis Core Specification** — `../core/trellis-core.md`. Parent specification. Constitutional semantics: canonical truth (S5), admission/order (S6), hash construction (S7), verification (S8), cross-repository authority (S9).
- **Export Verification Package companion** — `./export-verification-package.md`. Sibling companion. Offline verifiability, package members, verification mode.
- **Trust Profiles companion** — `../trust/trust-profiles.md`. Trust-profile declarations inherited by disclosure manifests.
- **Assurance Traceability companion** — `../assurance/assurance-traceability.md`. Assurance-level semantics; relevant to posture/assurance non-conflation (Section 11).

Non-normative cross-references:

- **Unified Ledger Core draft** — `../../DRAFTS/unified_ledger_core.md`. Source material for canonical semantics migrated into Trellis Core.
- **Unified Ledger Companion draft** — `../../DRAFTS/unified_ledger_companion.md`. Source material for Disclosure and Export Profile (§2.4), coverage honesty (§2.6.2, §3.0.2), selective disclosure discipline (§2.4.2), and claim classes (§2.4.1).
