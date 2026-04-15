---
title: Trellis Companion — Assurance, Identity, and Traceability
version: 0.1.0-draft.2
date: 2026-04-14
status: draft
---

# Trellis Companion — Assurance, Identity, and Traceability v0.1

**Version:** 0.1.0-draft.2
**Date:** 2026-04-14
**Editors:** Formspec Working Group
**Companion to:** Trellis Core v0.1

---

## Status of This Document

This document is a **draft specification**. It is a companion to the Trellis Core specification and does not modify Formspec or WOS processing semantics. Implementors are encouraged to experiment with this specification and provide feedback, but MUST NOT treat it as stable for production use until a 1.0.0 release is published.

This companion previously scoped only to a continuous-integration traceability matrix. This revision expands it to its full companion mandate: identity, attestation, assurance methodology, continuity semantics, and the operational traceability appendix.

## Conventions and Terminology

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in BCP 14 [RFC 2119] [RFC 8174] when, and only when, they appear in ALL CAPITALS, as shown here.

JSON syntax and data types are as defined in [RFC 8259].

Each normative section in this document carries a **Requirement class** marker indicating whether the requirement is a **Constitutional semantic** (inherited from Trellis Core and restated here for traceability) or a **Companion requirement** (introduced by this companion, subordinate to the core).

## Abstract

This companion governs the **assurance** dimension of Trellis deployments: the identity and attestation facts that enter canonical truth, the assurance level that qualifies those facts, the disclosure posture that governs how subject information is revealed, the subject-continuity references that link activity over time, and the methodology by which implementations demonstrate that core invariants are actually upheld.

It adds identity, attestation, assurance-upgrade, and continuity semantics to the Trellis verification layer defined in Core (S5, S8, and §4.7–4.10 of the source constitution draft). It does not define Formspec or WOS semantics; identity and attestation facts, when bound into canonical truth, participate in canonical order and verification under the same admissibility rules as any other canonical fact.

## Table of Contents

1. Introduction
2. Terminology
3. Identity, Attestation, and Assurance Methodology
   - 3.1 Identity, Attestation, and Continuity Detail
   - 3.2 User-Originated Signing
   - 3.3 Assurance, Disclosure, and Continuity
   - 3.4 Disclosure Posture and Assurance (Invariant 6 Restated)
   - 3.5 Attestation Provenance and Provider Neutrality
   - 3.6 Legal Sufficiency Statement
4. Assurance-Upgrade Facts
5. Invariant Scope and Assurance Methodology
6. Minimum CI Expectations
7. Evidence Retention Policy
8. Conformance
9. Security and Privacy Considerations
10. Cross-References
11. Appendix A — Operational Traceability Matrix

---

## 1. Introduction

### 1.1 Scope

**Requirement class:** Companion requirement

This companion specifies:

- normative definitions for assurance level, disclosure posture, subject continuity, and attestation, together with the preferred controlled vocabulary inherited from Trellis Core §4.10;
- the identity and attestation facts that MAY enter canonical truth, including user-originated signatures and canonical append attestations emitted over those signatures;
- the behavior of **assurance-upgrade facts** — governance facts (companion §7.3) that record that an existing subject-continuity reference has been bound to a higher assurance level without re-authoring prior canonical records;
- the assurance methodology (models, tests, fuzzing, drills) by which implementations demonstrate that each named core invariant actually holds;
- an operational traceability matrix mapping each core invariant to concrete assurance methods and evidence artifacts (Appendix A).

### 1.2 Relationship to Trellis Core and Other Companions

**Requirement class:** Companion requirement

This companion is subordinate to Trellis Core. Nothing in this document alters canonical truth, canonical append semantics, trust honesty requirements, or export-verification guarantees established by the core.

This companion interacts with:

- **Trellis Core §4.7–4.10 and §7.1, §10.2** — from which it inherits the Trust Profile, Disclosure Posture, Subject Continuity, Controlled Vocabulary, and Invariant 6 (Disclosure Posture Is Not Assurance Level). Where this companion restates material from the core, it does so for traceability only; the core governs.
- **Trust Profiles companion** — identity, attestation, and assurance facts are always evaluated under an active Trust Profile (Trust Profiles §3.0.1). Disclosure posture for identity content is constrained by the active profile's metadata budget.
- **Key Lifecycle Operating Model** — signing keys that produce user-originated signatures, and canonical append-attestation keys, are governed by the Key Lifecycle companion.
- **Export Verification Package / Disclosure Manifest** — identity, attestation, and assurance-upgrade facts included in an export MUST preserve the provenance distinctions of Trellis Core §12.4.

---

## 2. Terminology

**Requirement class:** Companion requirement (definitions), Constitutional semantic (vocabulary discipline)

This companion uses the preferred terms established by the Trellis Core controlled vocabulary (Core §4.10) — *author-originated fact*, *canonical fact*, *canonical record*, *canonical append attestation*, *derived artifact*, *disclosure or export artifact*, and *append-head reference* — and extends it with the following terms. Normative sections MUST prefer these terms over casual alternatives.

### 2.1 Assurance Level

A declared measure of the rigor with which the binding between a subject-continuity reference and a real-world principal, authoring key, or attribute has been established and is maintained. Assurance level is an **ordered** declaration: higher levels correspond to stronger process, stronger signature semantics, or stronger corroboration. Assurance level is a property of the **binding** between a subject and a continuity reference (or of a specific fact emitted against such a binding); it is not a property of the subject's identity disclosure.

An implementation MUST declare the assurance levels it supports, the admissibility rules for each level, and the observable criteria by which one level is distinguished from another. This specification does not enumerate concrete levels; profiles or bindings MAY do so.

### 2.2 Disclosure Posture

Per Trellis Core §4.8, a declared posture describing how much identity or subject information is intended to be revealed in a given context — for example, *anonymous*, *pseudonymous*, *identified*, or *public*. Disclosure posture is a property of a **context or artifact**, not of the subject. Disclosure posture MAY vary across canonical records, canonical append attestations, disclosure or export artifacts, and derived artifacts for the same subject.

### 2.3 Subject Continuity

Per Trellis Core §4.9, a stable continuity reference for a subject, record holder, or respondent that links related activity, records, or attestations across time without, by itself, requiring full legal identity disclosure. Subject continuity is a **linkage primitive**; it is not identity. An implementation MAY support subject continuity at any declared disclosure posture, including pseudonymous or anonymous postures.

### 2.4 Attestation

A statement — issued by a principal, a canonical append service, or a delegated authority — that a particular claim is true at a particular time under a particular trust profile. This companion recognizes two primary attestation families:

- **Authored attestation** — a statement signed or otherwise authenticated by an originating principal (including a user). User-originated signatures (§3.2) are authored attestations.
- **Canonical append attestation** — per Trellis Core §4.4 / §3.3, a service-issued attestation that a canonical record was accepted into canonical order.

An attestation is itself subject to the Trust Profile, Key Lifecycle, and provenance rules of the substrate. Attestations MUST remain distinguishable from the underlying facts they attest to (Trellis Core Invariant 1 — Author-Originated Fact Is Not Append Attestation).

### 2.5 Controlled Vocabulary Preference

**Requirement class:** Constitutional semantic

To reduce synonym drift, normative text in this companion uses the preferred terms established in Trellis Core §4.10. Casual alternatives (for example, "signed receipt," "inclusion proof token," "ID proof," or "verified user") SHOULD NOT be introduced without binding to a preferred term.

---

## 3. Identity, Attestation, and Assurance Methodology

This section is normative. It consolidates, under a single companion, the identity- and attestation-relevant requirements inherited from Trellis Core §10.2 and the source constitution draft §4.7–4.10 and §7.1, together with the companion detail from the source companion draft §3.5 and §3.8.3.

### 3.1 Identity, Attestation, and Continuity Detail

**Requirement class:** Companion requirement

Authentication mechanisms are not themselves canonical facts unless explicitly bound into canonical truth by an active profile or binding. If identity or attestation facts are represented canonically or in profile-specific bindings, the implementation:

- MUST distinguish the authored identity or attestation fact from the canonical append attestation that later admits it (Trellis Core Invariant 1);
- MUST distinguish the identity or attestation fact from any disclosure or export artifact that later presents it (Trellis Core §8.1);
- MUST preserve the ability for a verifier to determine, for any presented identity or attestation artifact, which subject-continuity reference it binds to and under what assurance level;
- MUST NOT conflate the disclosure posture of an identity artifact with its assurance level (see §3.4).

Identity and attestation facts, when canonically admitted, participate in canonical order (Trellis Core §6) and are subject to the same admission and idempotency rules as any other canonical fact.

### 3.2 User-Originated Signing

**Requirement class:** Companion requirement

Implementations SHOULD support user-originated signatures over author-originated facts admitted into canonical truth. Implementations MAY support offline user-originated signing, in which the signature is produced before submission to a canonical append service.

If user-originated signing is supported, the following requirements apply.

1. The resulting evidence package MUST distinguish:
   - the **user-originated signature** (or equivalent authored authentication) over the author-originated fact, and
   - the **canonical append attestation** subsequently emitted by the canonical append service over the canonical record that represents that fact.

2. A verifier presented with such an evidence package MUST be able to validate each of these attestations independently. In particular, the validity of the user-originated signature MUST NOT depend on derived runtime state, and the validity of the canonical append attestation MUST NOT be presumed from the validity of the user-originated signature.

3. Where the user-originated signature is produced offline, the implementation MUST preserve authored authentication semantics across delayed submission (source draft companion §2.1.1) and MUST NOT imply that canonical append time is identical to authored time unless such equivalence is established by the active profile or binding.

4. The signing key used to produce the user-originated signature is subject to the Key Lifecycle Operating Model companion; in particular, revocation and rotation of that key MUST NOT retroactively invalidate prior canonical records whose admission already occurred.

### 3.3 Assurance, Disclosure, and Continuity

**Requirement class:** Companion requirement (restatement of Trellis Core §10.2 / source constitution §10.2)

A conforming implementation:

1. MUST distinguish **assurance level** from **disclosure posture**;
2. MUST NOT treat higher assurance as requiring greater identity disclosure by default;
3. MAY support **subject continuity** without requiring full legal identity disclosure; and
4. MUST preserve those distinctions across trust profiles, canonical records, canonical append attestations, disclosure and export artifacts, and derived artifacts.

Profiles and bindings MAY constrain the admissible combinations of assurance level and disclosure posture for a given fact family, but they MUST NOT collapse the two dimensions into a single scalar.

### 3.4 Disclosure Posture and Assurance (Invariant 6 Restated)

**Requirement class:** Constitutional semantic (inherited from Trellis Core §7.1, Named Invariant 6)

**Invariant 6 — Disclosure Posture Is Not Assurance Level.** Disclosure posture and assurance posture MUST remain distinct and MUST NOT be conflated. This invariant is constitutional; this companion restates it, assigns it operational consequences, and maps it into the traceability matrix (Appendix A).

Behavioral consequences, normative in this companion:

- **No disclosure-coupled assurance.** Implementations MUST NOT require a disclosure posture to be escalated (for example, from *pseudonymous* to *identified*) merely because a fact is being emitted at a higher assurance level, and MUST NOT require an assurance level to be escalated merely because the disclosure posture of an artifact is wider.
- **Independent encoding.** Profiles and bindings MUST encode assurance level and disclosure posture as independently declarable properties of the relevant fact, record, or artifact. A single combined token (for example, a string such as `identified-high`) is permitted as a presentational convenience only if the binding preserves independent recovery of both components.
- **Independent transition.** A change in assurance level (for example, an assurance upgrade under §4) MUST NOT implicitly change the disclosure posture of historical canonical records or of the artifacts derived from them. A change in disclosure posture (for example, a selective-disclosure export at a narrower posture) MUST NOT implicitly change the assurance level of the underlying facts.
- **Verifier obligation.** A conforming verifier presented with a canonical record, an attestation, or a disclosure or export artifact MUST be able to report the assurance level and the disclosure posture as independent outputs. A verifier MUST NOT collapse them into a single scalar in its normative output.

**Conformance role assignment.** The Invariant 6 Custodian role defined in §8.4 is responsible for demonstrating that the above behavioral consequences hold across the implementation's canonical, derived, and export paths.

### 3.5 Attestation Provenance and Provider Neutrality

**Requirement class:** Companion requirement

If identity or attestation facts are represented canonically or in profile-specific bindings, the implementation SHOULD represent those facts **provider-neutrally** where feasible — meaning that the canonical semantics of the attestation are defined in terms of the preferred vocabulary of Trellis Core §4.10 and this companion, not in terms of a specific issuer, adapter, federation technology, or vendor trust framework.

Provider-specific issuers, adapters, or bindings MAY be used operationally, but they SHOULD NOT define the long-lived semantic meaning of the attestation within the constitutional model. In practice:

- An attestation's issuer, key identifier, or adapter-specific metadata MAY appear in the canonical record as verifiable provenance, but the attestation's **meaning** (what claim it establishes, under what assurance level, bound to which subject-continuity reference) MUST be expressible without reference to the specific provider.
- Migrations between provider-specific adapters (for example, a change of federated identity provider) MUST NOT silently reinterpret the meaning of historical attestations (Trellis Core §16.6, Versioning and Algorithm Agility).

### 3.6 Legal Sufficiency Statement

**Requirement class:** Companion requirement

Implementations **MUST NOT** imply that cryptographic controls alone guarantee admissibility, legal sufficiency, or evidentiary weight in all jurisdictions.

Implementations MAY claim stronger evidentiary posture only to the extent supported by:

- organizational **process** (intake, review, chain-of-custody practices);
- **signature semantics** (authored authentication, declared assurance level, provider-neutral attestation meaning);
- **canonical append attestations** (service-issued inclusion evidence under the active append scope);
- declared **records practice** (retention, legal hold, archival, sealing — Trellis Core §16.5);
- **applicable law** in the relevant jurisdiction.

Claims about admissibility, non-repudiation, or evidentiary weight MUST be bounded by these considerations. This companion does not grant legal effect to canonical records; it only specifies the evidentiary primitives on which such effect MAY be built.

---

## 4. Assurance-Upgrade Facts

**Requirement class:** Companion requirement

The source companion draft §7.3 admits **verification-upgrade facts** as a recognized governance-and-processing fact family that MAY be canonically admissible. This companion normatively specifies how such facts are recorded, scoped, and presented.

### 4.1 Definition

An **assurance-upgrade fact** (called a *verification-upgrade fact* in the source companion draft §7.3) is a canonical fact that records that an existing subject-continuity reference has been re-bound, or additionally bound, at a higher assurance level than previously established, without rewriting any prior canonical record.

### 4.2 Admission Requirements

If an implementation records assurance upgrades canonically, each assurance-upgrade fact MUST:

1. reference an existing **subject-continuity reference** that identifies the subject whose assurance level is being upgraded;
2. declare both the **prior assurance level** (if known) and the **new assurance level**, using the assurance-level vocabulary declared by the active profile or binding;
3. declare the **basis** for the upgrade — that is, the authored attestation(s), attested corroboration, or process evidence that justifies the new level, as a reference admissible under the active profile or binding;
4. be admitted under the canonical append rules of Trellis Core §6 and issued a canonical append attestation under §9 of the source constitution draft (equivalently, Trellis Core §6 Admission and §7 Hash Construction plus the binding-defined attestation model);
5. preserve the **disclosure posture** of the underlying subject continuity unless a separate, explicit disclosure-posture change is being recorded (§3.4).

An assurance-upgrade fact MUST NOT rewrite, re-order, or invalidate prior canonical records. Prior canonical records retain the assurance level that was in effect at their admission; the upgrade is forward-effective only, unless the active profile or binding declares otherwise and does so within the constitutional rules of Trellis Core §16.6 (Versioning and Algorithm Agility).

### 4.3 Binding to Subject Continuity

An assurance-upgrade fact MUST bind to the same subject-continuity reference as the records whose forward assurance level it affects. Implementations:

- MUST NOT use an assurance upgrade as an implicit mechanism to link two previously unlinked subject-continuity references (that would be a continuity-merge, a separate and more sensitive operation; see §9);
- MUST ensure that a verifier evaluating a later canonical record can determine the effective assurance level at the time of that record's admission, either by consulting the canonical order directly or by consulting a derived index that is rebuildable from canonical truth (Trellis Core §5.1, §16.1).

### 4.4 Appearance in Evidence Packages

An export verification package (see Export Verification Package companion) that covers records affected by an assurance upgrade MUST:

- include the assurance-upgrade fact(s) that establish the effective assurance level for each exported canonical record, or an attested reference sufficient for an offline verifier to resolve them;
- preserve the distinction between the assurance-upgrade fact itself (a canonical fact), the canonical append attestation that admitted it, and any disclosure or export artifact that presents it in human-readable form;
- present assurance level and disclosure posture independently in machine-readable form (§3.4, independent encoding).

A disclosure manifest (see Disclosure Manifest companion) that references such records MUST declare whether the assurance-upgrade history is within the disclosed scope and MUST NOT imply that records admitted before the upgrade carry the upgraded assurance level.

### 4.5 Relationship to Invariant 6

An assurance-upgrade fact is, by construction, a change to the assurance dimension; it MUST NOT implicitly change the disclosure posture of the affected subject-continuity reference (§3.4). A co-occurring change to disclosure posture MUST be represented by a separate canonical fact under the active profile or binding.

---

## 5. Invariant Scope and Assurance Methodology

### 5.1 Purpose

**Requirement class:** Companion requirement

Map each core invariant to concrete assurance methods so that assurance remains architectural, not appendix-only. Every named invariant listed in §5.2 MUST be covered by at least one primary assurance method (§5.3) and produce retained evidence artifacts (§7).

### 5.2 Invariant Scope

**Requirement class:** Companion requirement

"Every normative invariant" in this companion refers to the following defined set. Additional invariants added by future companions MUST be registered in this list before assurance methods are required for them.

1. **Trellis Core Named Invariants** — the six named invariants (Append-only Canonical History; No Second Canonical Truth; One Canonical Order per Governed Scope; One Canonical Event Hash Construction; Verification Independence; Append Idempotency) as enumerated in Trellis Core §5.2, together with the additional constitutional invariants enumerated at §7.1 of the source constitution draft, specifically **Invariant 6 — Disclosure Posture Is Not Assurance Level** (restated in §3.4 of this companion).
2. **Shared Ledger Binding canonization invariants** — canonization rules 1–4 of the Shared Ledger Binding companion §5.
3. **Key Lifecycle state-transition invariants** — allowed transitions per the Key Lifecycle Operating Model companion §3.
4. **Trust Profile honesty invariant** — Trust Profiles companion §5.
5. **Assurance companion invariants** — the distinctness obligation of §3.4 (Invariant 6), the independent-encoding obligation of §3.4, and the non-rewrite obligation of §4.2.

### 5.3 Assurance Methodology

**Requirement class:** Companion requirement

For each invariant in §5.2, the implementation MUST declare at least one **primary method** drawn from the following catalog, and SHOULD declare at least one **secondary method** where the catalog supports one:

- formal or semi-formal models (TLA+, Alloy, or equivalent);
- property-based tests;
- shared test vectors, executed across each implementation substrate (for example, native and WASM);
- adversarial replay tests;
- rebuild-from-canonical drills;
- purge and destruction drills;
- parser and verifier fuzzing;
- cross-implementation offline verifier vectors;
- profile conformance tests and metadata-budget disclosure audits.

The concrete mapping is maintained in Appendix A.

---

## 6. Minimum CI Expectations

**Requirement class:** Companion requirement

1. Every normative invariant in §5.2 MUST map to at least one automated check.
2. Hash and serialization vectors MUST execute in every substrate the implementation ships (at minimum, native and WASM where both exist).
3. Recovery, destruction, and assurance-upgrade drills MUST run on a recurring schedule and produce retained evidence artifacts (§7).
4. Fuzzing outcomes MUST feed parser and verifier hardening backlogs.
5. Assurance-upgrade and user-originated signing paths MUST be exercised by dedicated CI checks that include: valid upgrade admission, rejection of rewrite attempts, verifier-independent validation of user-originated signatures (§3.2.2), and independent reporting of assurance level and disclosure posture (§3.4).

### 6.1 Role-Based Applicability

**Requirement class:** Companion requirement

| Requirement | Verifier implementations | Canonical Append Service | Studio / tooling |
|---|---|---|---|
| Invariant-to-check mapping | MUST | MUST | SHOULD |
| Native + WASM test vectors | MUST | SHOULD | MAY |
| Recovery / destruction drills | SHOULD | MUST | MAY |
| Assurance-upgrade drills | SHOULD | MUST | MAY |
| User-originated signing checks | MUST | SHOULD | SHOULD |
| Fuzzing backlogs | MUST | SHOULD | MAY |

---

## 7. Evidence Retention Policy

**Requirement class:** Companion requirement

- Assurance artifacts SHOULD be retained for at least one full major-version support window. The definition of "major-version support window" is an engineering decision, not a legal guarantee; jurisdictions MAY impose longer retention requirements.
- Artifacts MUST include build and version identifiers and execution timestamps.
- Failed assurance runs MUST be retained with remediation linkage. Failed runs MUST NOT be suppressed or deleted after remediation.
- Evidence artifacts covering identity, attestation, or assurance-upgrade paths MUST preserve provenance distinctions consistent with Trellis Core §12.4 when included in an export.

---

## 8. Conformance

**Requirement class:** Companion requirement

This companion defines the following conformance roles. An implementation MAY claim one or more roles and MUST satisfy all requirements applicable to each claimed role.

### 8.1 Assurance Producer

Implements automated checks, drills, and fuzzing for the invariants registered in §5.2. MUST map every registered invariant to at least one primary assurance method and produce evidence artifacts consistent with §7.

### 8.2 Assurance Auditor

Reviews evidence artifacts, remediation linkage, and retention compliance. MUST verify that evidence artifacts include build and version identifiers and execution timestamps, and MUST verify that failed runs are retained with remediation linkage.

### 8.3 Identity-Fact Implementer

An implementation that admits identity or attestation facts canonically. MUST satisfy §3.1 (Identity, Attestation, and Continuity Detail) and §3.5 (Attestation Provenance and Provider Neutrality). If user-originated signing is supported, it MUST additionally satisfy §3.2. Identity-Fact Implementers MUST NOT imply evidentiary weight beyond the bounds established in §3.6.

### 8.4 Invariant 6 Custodian

The role responsible for demonstrating the behavioral consequences of §3.4 across canonical, derived, and export paths. An Invariant 6 Custodian:

- MUST produce, as part of its assurance evidence, a machine-readable report per release that extracts assurance level and disclosure posture as independent outputs for each fact family covered by the implementation;
- MUST demonstrate, by test or by model, that an assurance-upgrade fact does not implicitly change disclosure posture (§3.4, §4.5);
- MUST demonstrate that a disclosure-posture-narrowing export does not imply a change in the assurance level of the underlying facts.

### 8.5 Assurance-Upgrade Recorder

An implementation that canonically admits assurance-upgrade facts per §4. MUST satisfy §4.2 (admission), §4.3 (continuity binding), §4.4 (evidence packaging), and §4.5 (independence from disclosure-posture changes).

### 8.6 Legal-Sufficiency Bounded Implementer

Any implementation that makes external claims about admissibility, evidentiary weight, or legal sufficiency. MUST satisfy §3.6 (Legal Sufficiency Statement).

---

## 9. Security and Privacy Considerations

**Requirement class:** Companion requirement (disclosure obligations); non-normative guidance (threat enumeration)

### 9.1 Assurance-Artifact Sensitivity

Assurance artifacts MAY contain sensitive implementation details. Evidence retention policy (§7) MUST account for access control. Failed assurance runs MUST NOT be suppressed or deleted; they MUST be retained with remediation linkage even when the underlying issue has been resolved. Fuzzing corpora and crash reports SHOULD be treated as sensitive artifacts; publication SHOULD be governed by the deployment's active Trust Profile (Trust Profiles §3, §5).

### 9.2 Identity and Attestation in Evidence Packages

Evidence packages that include identity or attestation facts expose an elevated privacy surface. Implementations:

- MUST ensure that identity or attestation facts included in an evidence package are covered by the disclosure posture declared for that package and by the active Trust Profile's metadata budget (Trust Profiles §metadata-budget);
- MUST NOT include identity-revealing material (including identifiers, attribute attestations, issuer-specific identifiers, or adapter metadata that is effectively identifying) in assurance artifacts whose retention window exceeds the retention terms of the active Trust Profile;
- SHOULD prefer subject-continuity references to raw identifiers when including assurance-path evidence, and SHOULD distinguish test fixtures (synthetic subjects) from production evidence at the storage layer.

### 9.3 Subject-Continuity References and Re-Identification Risk

Subject-continuity references (§2.3) are designed to be stable linkage primitives without requiring legal identity disclosure. Their presence in retained assurance or evidence artifacts nevertheless creates re-identification risk when combined with external data. Implementations:

- MUST NOT treat subject-continuity references as non-identifying merely because they are pseudonymous;
- SHOULD minimize the number of subject-continuity references that appear in assurance artifacts that are retained beyond the relevant operational window;
- SHOULD prefer derived aggregates or synthetic fixtures in CI artifacts rather than production subject-continuity references.

### 9.4 Cross-Jurisdiction Considerations

Retained assurance artifacts MAY be subject to differing data-protection, retention, and disclosure regimes depending on where they are stored and where the underlying subjects reside. Implementations:

- MUST document the jurisdictional scope assumed by their retention policy (§7);
- SHOULD avoid co-locating high-sensitivity assurance artifacts (for example, user-originated signing drills against real production keys) with low-sensitivity artifacts whose broader access is expected.

### 9.5 Privacy Surface of User-Originated Signing

User-originated signing (§3.2) introduces distinctive privacy considerations:

- Signing metadata (device attestation, client identifiers, offline-submission timing) can be identifying even when the signed payload is not disclosed. Implementations MUST account for such metadata within the metadata budget of the active Trust Profile.
- Revocation and rotation events on user-held signing keys are themselves linkage signals. The Key Lifecycle Operating Model companion governs their recording; this companion requires only that such events MUST NOT be surfaced in assurance artifacts beyond what the active Trust Profile permits.

### 9.6 Threat Enumeration (non-normative)

Implementers should consider at least: key compromise; verifier divergence; replay and reordering of user-originated signatures; recovery abuse against signing keys; authorization drift between canonical identity facts and derived evaluators; snapshot misuse of retained identity fixtures; equivocation in canonical append attestation over identity facts; over-broad delegated compute grants that observe plaintext identity material; silent provider-adapter migration that reinterprets historical attestation meaning (§3.5).

---

## 10. Cross-References

**Requirement class:** Companion requirement (normative citations)

This companion normatively cross-references the following documents. Where this companion's requirements depend on behavior defined elsewhere, the cited document governs.

- **Trellis Core Specification** (`trellis/specs/core/trellis-core.md`) — constitutional semantics, named invariants, canonical truth, admission, hash construction, verification requirements. In particular: §4.7–4.10 (Trust Profile, Disclosure Posture, Subject Continuity, Controlled Vocabulary), §7.1 Named Invariant 6 (Disclosure Posture Is Not Assurance Level), §10.2 (Disclosure Posture and Assurance), §5.2 named invariants, §12.4 provenance-distinction requirement.
- **Trust Profiles** (`trellis/specs/trust/trust-profiles.md`) — active Trust Profile declaration, metadata budget, trust honesty rule, verification posture classes.
- **Key Lifecycle Operating Model** (`trellis/specs/trust/key-lifecycle-operating-model.md`) — lifecycle governance for user-originated signing keys and canonical append-attestation keys.
- **Export Verification Package** (`trellis/specs/export/export-verification-package.md`) — requirements for including identity, attestation, and assurance-upgrade facts in exports.
- **Disclosure Manifest** (`trellis/specs/export/disclosure-manifest.md`) — requirements for disclosure-scope declarations covering identity and assurance material.
- **Shared Ledger Binding** (source companion draft / `trellis/specs/core/shared-ledger-binding.md` when published) — canonization rules referenced in §5.2.

---

## Appendix A — Operational Traceability Matrix

This appendix is **normative for mapping** and **operational for CI**. It is retained from v0.1.0-draft.1 and extended to cover Invariant 6 and assurance-upgrade facts.

### A.1 Invariant-to-Method Mapping

| Core invariant | Normative definition | Primary methods | Secondary methods | Evidence artifacts |
|---|---|---|---|---|
| Append-only canonical history | Trellis Core §5.2 invariant 1 | TLA+ model of append transitions | Property-based append tests | Model files, passing model checks, test logs |
| One canonical order per governed scope | Trellis Core §5.2 invariant 3 | TLA+ scope / order invariants | Adversarial replay tests | Scope-partition proofs, replay test reports |
| One canonical event hash construction | Trellis Core §5.2 invariant 4 | Shared test vectors (native + WASM) | Parser / verifier fuzzing | Vector fixtures, fuzz corpus and crash reports |
| No second canonical truth | Trellis Core §5.2 invariant 2 | Alloy constraints on canonical / derived separation | Rebuild-from-canonical drills | Alloy model checks, rebuild drill logs |
| Verification independence | Trellis Core §5.2 invariant 5 | Offline verifier cross-implementation vectors | Package corruption fuzz tests | Verifier outputs, corruption-detection logs |
| Append idempotency | Trellis Core §5.2 invariant 6 | Property-based idempotency tests | Failure / retry chaos tests | Deterministic idempotency test suite outputs |
| **Disclosure posture is not assurance level (Invariant 6)** | **Trellis Core §7.1 (source constitution §7.1) / restated in this companion §3.4** | **Independent-encoding conformance tests; Invariant 6 Custodian release report (§8.4)** | **Assurance-upgrade non-coupling drills (§4.5); export-posture-narrowing drills** | **Per-release independent-output report; drill logs; combined-token decomposition fixtures** |
| Trust profile honesty | Trust Profiles §5 | Profile conformance tests | Metadata-budget disclosure audits | Profile declarations + audit records |
| Key lifecycle correctness | Key Lifecycle §3 | State-transition property tests | Rotation / recovery drills | Transition test reports, recovery drill artifacts |
| Crypto-shredding completeness | Key Lifecycle §7, Projection §4 | Purge-cascade integration tests | Snapshot / cache residue scans | Purge verification reports |
| Export offline verifiability | Export Verification Package §4 | Offline verifier cross-implementation vectors | Package corruption fuzz tests | Verifier outputs, corruption-detection logs |
| **Assurance-upgrade fact admission and non-rewrite** | **This companion §4 (source companion §7.3 verification-upgrade governance family)** | **Admission conformance tests; rewrite-rejection property tests** | **Cross-export provenance-distinction drills; continuity-merge negative tests** | **Admission test logs; rejection proofs; export-package inclusion fixtures** |
| User-originated signing distinctness | This companion §3.2 (source companion §3.5.1) | Verifier independence tests (signature vs. canonical append attestation) | Offline-signing delayed-submission drills | Evidence-package fixtures; verifier independence reports |
| Legal-sufficiency statement bound | This companion §3.6 (source companion §3.8.3) | Public-claim audit against §3.6 criteria | Process-and-practice review | Claim-audit reports |

### A.2 Registering Additional Invariants

New invariants introduced by future companions MUST be added to this matrix before assurance methods become required for them (§5.2). Each new row MUST declare at least a primary method and an evidence artifact class.
