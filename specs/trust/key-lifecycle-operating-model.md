---
title: Trellis Companion — Key Lifecycle Operating Model
version: 0.1.0-draft.2
date: 2026-04-14
status: draft
---

# Trellis Companion — Key Lifecycle Operating Model v0.1

**Version:** 0.1.0-draft.2
**Date:** 2026-04-14
**Editors:** Formspec Working Group
**Companion to:** Trellis Core v0.1, Trellis Companion — Trust Profiles v0.1

---

## Status of This Document

This document is a **draft specification**. It is a companion to the Trellis Core specification (hereafter "Trellis Core") and is subordinate to two upstream documents:

1. **Trellis Core v0.1** — Sections 5 (Canonical Truth and Invariants), 6 (Canonical Admission and Order), and the Lifecycle and Cryptographic Inaccessibility requirements at Trellis Core S16.5 and Versioning and Algorithm Agility requirements at Trellis Core S16.6 govern this companion.
2. **Trellis Companion — Trust Profiles v0.1** — declarations made under Trust Profiles S5 (Baseline Profiles), S6 (Mandatory Profile Declarations), and S11 (Profile declaration schema) constrain which key-lifecycle behaviors a deployment is authorized to perform. In particular, this companion implements the operational requirements implied by the Trust Profile fields `recovery_mode`, `destruction_semantics`, and `disclosure_authority`.

This document does not modify Formspec or WOS processing semantics. It does not redefine constitutional semantics owned by Trellis Core. It does not redefine custody, readability, or disclosure posture owned by Trust Profiles. Where the requirements of Trellis Core and this companion conflict, Trellis Core prevails. Where the requirements of Trust Profiles and this companion conflict, Trust Profiles prevails.

Implementors are encouraged to experiment with this specification and provide feedback, but MUST NOT treat it as stable for production use until a 1.0.0 release is published.

## Conventions and Terminology

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in BCP 14 [RFC 2119] [RFC 8174] when, and only when, they appear in ALL CAPITALS, as shown here.

JSON syntax and data types are as defined in [RFC 8259].

## Abstract

The Key Lifecycle Operating Model defines cryptographic key lifecycle as first-class platform behavior — not implementation detail. It specifies key classes, lifecycle states and transitions, rotation and versioning rules, grace periods, key-destruction and crypto-shredding semantics, evidence-artifact requirements for destruction and erasure, the cryptographic proof-of-quorum format used when threshold custody is declared, the cryptographic recovery-evidence format, export-issuance facts, and historical verification across algorithm evolution. Generic lifecycle operations (retention, legal hold, archival, sealing, schema upgrade) and generic recovery-authority declaration are defined upstream — see [WOS Governance §2.9, §4.9, §7.15, §12], [WOS Assurance §6], and [Trellis Companion — Trust Profiles §3]. This companion adds key-management semantics to the Trellis trust layer defined in Trust Profiles (S5–S11). It does not define Formspec or WOS semantics.

## Table of Contents

1. Conformance
2. Key Classes and Scope Boundaries
3. Lifecycle States and Allowed Transitions
4. Rotation, Versioning, and Grace Periods
5. Lifecycle Operations
   - 5.1 Retention
   - 5.2 Legal Hold
   - 5.3 Archival
   - 5.4 Key Destruction
   - 5.5 Sealing
   - 5.6 Export Issuance
   - 5.7 Schema Upgrade
6. Erasure and Key Destruction Disclosure
7. Sealing and Later Lifecycle Facts
8. Legal Sufficiency Statement
9. Threshold and Quorum Custody
10. Recovery Authorities
11. Algorithm Agility and Historical Verification
12. Required Completeness Rule
13. Security and Privacy Considerations

---

## 1. Conformance

**Requirement class:** Companion requirement

This companion defines the following conformance roles:

1. **Key Lifecycle Manager** — manages key provisioning, rotation, suspension, retirement, and destruction. MUST comply with the lifecycle state transitions defined in §3, the grace-period rules in §4, the operation-specific requirements in §5, the destruction evidence requirements in §6, the threshold-custody operational requirements in §9, and the recovery-authority requirements in §10.

2. **Key Lifecycle Auditor** — verifies key lifecycle compliance against declared Trust Profile posture. MUST be able to verify destruction evidence artifacts (§6), recovery audit trails (§10), purge-cascade completion artifacts (§12), and threshold participation records (§9). Auditor scope is bounded by Trust Profiles S6 — auditors observe control-plane facts and MUST NOT be required to access protected payloads.

A conforming implementation MUST satisfy all requirements applicable to each claimed role.

---

## 2. Key Classes and Scope Boundaries

**Requirement class:** Companion requirement

A conforming implementation MUST classify each managed key into one of the following classes. The class governs which lifecycle operations apply, which state transitions are permitted, and which evidence-artifact requirements bind.

1. **Tenant root or policy keys** — keys that anchor a tenant's cryptographic policy.
2. **Scope or ledger keys** — keys whose authority is bounded to a declared canonical scope (Trellis Core S6).
3. **Subject or record encryption keys** — keys that protect a single record or a small set of related records.
4. **Signing or attestation keys** — keys used to sign canonical facts, append attestations, or export artifacts.
5. **Recovery-only keys** — keys whose sole purpose is participation in declared recovery pathways (§10). Recovery-only keys MUST NOT be used for ordinary signing or decryption.

Each managed key MUST be assigned exactly one class. Classes MUST NOT be silently reassigned; class changes MUST be represented as canonical lifecycle facts.

---

## 3. Lifecycle States and Allowed Transitions

**Requirement class:** Companion requirement

A conforming implementation MUST represent each managed key's current state as one of the following. Transitions MUST be represented as canonical lifecycle facts (Trellis Core S16.5).

| State | Meaning | Allowed transitions |
|---|---|---|
| `provisioning` | Key material is being established and is not yet eligible for canonical use | `active`, `destroyed` |
| `active` | Current signing or decryption use is permitted within scope | `rotating`, `suspended`, `retired`, `destroyed` |
| `rotating` | Dual-validity grace handling is in progress | `active`, `retired`, `destroyed` |
| `retired` | No new encrypt or sign operations; verify and decrypt of historical material remain permitted as policy declares | `archived`, `suspended`, `destroyed` |
| `archived` | No interactive use; key material is preserved solely for historical verification or controlled re-decryption under declared recovery pathways | `destroyed` |
| `suspended` | Temporarily disabled by policy or incident response | `active`, `retired`, `destroyed` |
| `destroyed` | Cryptographic use permanently disallowed; key material is irrecoverable | _(terminal)_ |

### 3.1 Distinction between `retired` and `archived`

`retired` SHALL mean a key is removed from current operational use but is still accessible to the runtime for verification or controlled decryption. `archived` SHALL mean a key has been moved to cold custody and is not available to ordinary runtime paths; access requires an explicit retrieval operation that itself MUST be a canonical fact.

### 3.2 Hold-placed keys as a state modifier

A legal hold (§5.2) MUST be represented as a **state modifier**, not as a separate state. A key under hold retains its underlying lifecycle state (`active`, `retired`, `archived`, or `suspended`) but MUST NOT transition to `destroyed` while any hold is in effect. Implementations MUST refuse destruction transitions for held keys and MUST emit an auditable rejection fact when such a transition is attempted.

### 3.3 Transitions allowed for sealed scopes

Transitions of keys bound to a sealed scope (§5.5, §7) are governed by the implementation's declared sealing policy. An implementation MUST declare, per sealed scope class, which of `rotating`, `retired`, `archived`, `suspended`, and `destroyed` transitions are permitted after sealing. By default, a sealed scope SHALL permit transitions to `archived` and SHALL forbid transitions to `active` or `rotating`.

### 3.4 Transition to `suspended` from `retired`

A `retired` key MAY transition to `suspended` when an incident response or policy action requires that historical verification or decryption be temporarily disabled. The transition MUST be a canonical fact and MUST declare the suspending authority and effective time. The reverse transition (`suspended` → `retired`) is permitted under the same authority requirements.

### 3.5 Operational choice — binding-overridable

Bindings or sidecars MAY declare a more restrictive subset of transitions than the table in this section. Bindings MUST NOT add new states or expand the set of allowed transitions beyond what this section permits without publishing a registry update that defines the additional state, its meaning, and its transition constraints.

---

## 4. Rotation, Versioning, and Grace Periods

**Requirement class:** Companion requirement

Each managed key MUST carry a stable key identity and a monotonically increasing key-version. Lifecycle facts MUST reference both.

Rotations affecting intermittently connected clients MUST define a grace window. During grace windows, verification of historical signatures and controlled decryptability MUST remain predictable and MUST be declared in the Trust Profile (Trust Profiles S6). After grace expiry, stale-key writes MUST be rejected with auditable error facts.

Grace windows MUST be bounded; an implementation MUST NOT declare an unbounded grace window for canonical-use rotation.

---

## 5. Lifecycle Operations

**Requirement class:** Companion requirement

This section defines normative requirements for the seven canonical lifecycle operations enumerated in Trellis Core S16.5. An implementation MAY support a subset, including none. If an implementation supports an operation as part of its canonical or compliance-relevant behavior, the operation MUST be represented as a lifecycle fact (Trellis Core S16.5). If the operation affects compliance posture, retention posture, or recoverability claims, the lifecycle fact MUST be a canonical fact.

### 5.1 Retention

Retention semantics are deployment-defined; no upstream specification currently defines data retention schedules. Implementations MUST declare their retention policy. This ledger represents retention as canonical lifecycle facts and honors retention end conditions when evaluating destruction eligibility.

### 5.2 Legal Hold

Legal Hold semantics are defined in [WOS Governance §7.15 Legal Hold]. This ledger honors legal holds by preventing cryptographic erasure of covered keys for the duration of the hold, in accordance with §3.2.

### 5.3 Archival

Archival semantics are deployment-defined; no upstream specification currently defines cold-custody archival semantics. Implementations MUST declare their archival policy. This ledger represents archival as canonical lifecycle facts; transitions of key state to `archived` are governed by §3.

### 5.4 Key Destruction

**Requirement class:** Companion requirement

A key-destruction operation permanently disables cryptographic use of a managed key. Destruction operations MUST emit canonical lifecycle facts that reference the destroyed key identity and key-version, the destruction authority, the effective time, and a destruction evidence artifact (§6). Destruction MUST NOT proceed while any hold (§5.2) is in effect over a scope to which the key is bound. When protected content is cryptographically destroyed, affected derived plaintext state MUST be invalidated, purged, or otherwise made unusable according to the implementation's declared cascade policy (§12).

### 5.5 Sealing

Sealing semantics are defined in [WOS Governance §12 Typed Hold Policies]. This ledger represents sealing as canonical lifecycle facts; state transitions for keys bound to sealed scopes are governed by §3.3.

### 5.6 Export Issuance

**Requirement class:** Companion requirement

An export-issuance operation produces an independently verifiable disclosure artifact derived from canonical material. An export-issuance fact MUST identify the issued export, the issuing authority, the effective time, the canonical scope covered, and the algorithm-version material required to verify the export (§11). An export verification package MUST include sufficient immutable interpretation material to verify the export under the algorithms in effect at issuance, even if those algorithm families are later retired.

### 5.7 Schema Upgrade

Schema upgrade as a lifecycle operation is defined in [WOS Governance §2.9 Schema Upgrade]. This ledger records schema upgrades as canonical facts carrying the declared migration mechanism that preserves historical verifiability (§11).

---

## 6. Erasure and Key Destruction Disclosure

**Requirement class:** Companion requirement

If an implementation uses cryptographic erasure or key destruction, it MUST document, per declared destruction class:

1. **Which content becomes irrecoverable** — the canonical scope and payload classes that lose decryptability when the destruction completes.
2. **Who retains access, if anyone** — the residual decryptor classes (Trust Profiles S11 `decryptor_classes`) that remain after destruction, including the case where no party retains access.
3. **What evidence of destruction is preserved** — the form, location, and verification path of the destruction evidence artifact.
4. **What metadata remains** — the canonical facts, envelope fields, and side-channel signals that persist after destruction completes.

### 6.1 Destruction Evidence Artifact

**Requirement class:** Companion requirement

A destruction evidence artifact is the verifiable record that a destruction operation completed under declared authority. A conforming implementation MUST produce, for each completed destruction operation, an artifact containing at minimum:

1. **Key material reference** — the destroyed key's stable identity, key-version, and a cryptographic digest of the key material as it existed immediately prior to destruction. The digest MUST be computed under the algorithm-version active for the key (§11). The artifact MUST NOT contain the key material itself.
2. **Timestamp attestation** — an attested effective time for the destruction event, bound to the canonical append attestation that admitted the destruction fact (Trellis Core S6).
3. **Witness signature** — a signature over the artifact by an authority distinct from the actor that initiated destruction. The witness authority MUST be declared in the Trust Profile (`disclosure_authority`).
4. **Countersigner record** — when the deployment's destruction class requires multi-party authorization, the artifact MUST list each countersigner identity and MUST include a countersignature for each. The required countersigner count for sufficiency MUST be declared by the deployment and MUST equal or exceed any threshold required by §9 when threshold custody applies.

A destruction operation that completes without producing a sufficient destruction evidence artifact MUST NOT be claimed as canonical destruction. Implementations MUST either refuse the operation or record a partial-destruction fact that explicitly disclaims canonical destruction status.

### 6.2 Cascade Invalidation

**Requirement class:** Companion requirement

If protected content is cryptographically destroyed or otherwise made inaccessible, affected derived plaintext state MUST be invalidated, purged, or otherwise made unusable according to the implementation's declared cascade policy. The normative definition of purge-cascade semantics and projection rebuild requirements lives in the Projection and Runtime Discipline companion (S4); this companion owns the evidence-artifact requirements for cascade completion.

---

## 7. Sealing and Later Lifecycle Facts

Per-sealed-scope policy declarations governing which fact classes may be appended after sealing, and which authorities may append them, are defined in [WOS Governance §12 Typed Hold Policies]. This ledger honors those declarations when admitting later lifecycle facts against sealed scopes.

---

## 8. Legal Sufficiency Statement

See [WOS Assurance §6 Legal-Sufficiency Disclosure Obligations].

---

## 9. Threshold and Quorum Custody

**Requirement class:** Binding or sidecar choice

Generic N-of-M authorization policy is defined in [WOS Governance §4.9 Quorum-Based Delegation]. This section specifies the cryptographic proof-of-quorum format Trellis uses.

Threshold custody is not required for baseline conformance to Trellis Core or to this companion (Trellis Companion Conformance Boundary, draft companion Appendix G). A deployment MAY declare threshold custody in its Trust Profile. When threshold custody is declared, this companion owns the operational requirements that follow.

### 9.1 Minimum Quorum Count

**Requirement class:** Companion requirement

A deployment that declares threshold custody MUST declare, per protected operation class (decryption, destruction, recovery, export issuance), the **minimum quorum count** of distinct custodians required to authorize the operation. The declared quorum count for destruction MUST be at least two. The declared quorum count for recovery MUST be at least the count required by the deployment's recovery posture (§10).

Quorum counts MUST be declared as integers in the Trust Profile or in a binding-defined sidecar. Quorum counts MUST NOT be silently lowered; any reduction MUST be a canonical Trust Profile transition (Trust Profiles S12).

### 9.2 Required Evidence Artifact for Threshold Destruction

**Requirement class:** Companion requirement

When threshold custody applies to a destruction operation, the destruction evidence artifact (§6.1) MUST additionally include:

1. **Quorum declaration** — the quorum count that was in effect at the time of destruction, by reference to the active Trust Profile version.
2. **Per-custodian participation record** — for each custodian whose participation contributed to reaching quorum: the custodian identity, the participation timestamp attested under the active timestamp authority, and a per-custodian signature over the operation manifest.
3. **Sufficiency proof** — a verifiable assertion that the count of valid per-custodian participation records equals or exceeds the declared quorum count.

A threshold destruction operation that does not produce a sufficient artifact MUST NOT be claimed as canonical destruction.

### 9.3 Auditable Custodian Participation Record

**Requirement class:** Companion requirement

The implementation MUST preserve an auditable record of custodian participation that allows a Key Lifecycle Auditor to verify, after the fact, that the declared quorum was reached for each protected operation. The record MUST be canonical (Trellis Core S16.5) and MUST NOT be redacted from audit trails even after the underlying key material is destroyed (Trellis Core S5.2 invariant 1).

### 9.4 Exceptional-Access Posture

**Requirement class:** Companion requirement

If the deployment supports an exceptional-access pathway that bypasses the declared quorum (for example, a court-ordered single-custodian disclosure), that pathway MUST be declared as part of the threshold-custody profile and MUST itself be governed by §10 (Recovery Authorities). Threshold participation MUST NOT be described more strongly than the actual recovery process supports (Trust Profiles example Profile D).

---

## 10. Recovery Authorities

**Requirement class:** Companion requirement

Generic recovery-authority declaration (recovery posture, recovery class conditions, authorized assistors, provider-assisted vs reader-held distinction) is held in the Trust Profile object — see [Trellis Companion — Trust Profiles §3]. This section specifies the cryptographic recovery-evidence format Trellis uses when a recovery operation completes.

### 10.1 Recovery Evidence Artifact

**Requirement class:** Companion requirement

Each completed recovery operation MUST produce an auditable canonical fact that identifies the recovery class, the authorizing condition, the participating assistors, the effective time, and the post-recovery custody posture. When threshold custody applies, the recovery quorum MUST equal or exceed the relevant quorum count declared under §9.1, and the recovery evidence artifact MUST include the per-custodian participation records described in §9.2. Recovery facts MUST NOT be redacted from audit trails (Trellis Core S5.2 invariant 1).

---

## 11. Algorithm Agility and Historical Verification

**Requirement class:** Companion requirement

A conforming implementation:

- MUST version canonical algorithms and any schema or semantic digests, embedded copies, or immutable references needed for historical verification,
- MUST version author-originated fact semantics where profile- or binding-specific semantics exist,
- MUST version canonical record semantics, append semantics, export verification semantics, and trust-profile semantics,
- MUST preserve enough information to verify historical records under the algorithms and rules in effect when they were produced,
- MUST NOT silently reinterpret historical records under newer rules without an explicit migration mechanism,
- MUST ensure that algorithm or schema evolution does not silently invalidate prior export verification,
- MUST NOT rely on out-of-band operator knowledge to interpret historical records.

A conforming implementation **MUST preserve enough immutable interpretation material to verify historical records without live registry lookups, mutable references, or out-of-band operator knowledge** (draft companion Appendix C).

### 11.1 Required Versioning Fields in Key-Lifecycle Facts

**Requirement class:** Companion requirement

Each canonical key-lifecycle fact MUST carry, at minimum:

1. **Key identity** — the stable identity of the affected key, scoped per §2.
2. **Key-version** — the monotonically increasing version of the affected key.
3. **Algorithm-version reference** — an immutable reference (digest, embedded copy, or registry-bound version identifier preserved alongside the fact) sufficient to identify the cryptographic algorithm family and parameters in effect for this key-version.
4. **Semantic-version reference** — an immutable reference to the lifecycle-fact schema version under which the fact was authored.
5. **Effective time** — the canonical effective time for the lifecycle event, distinct from append admission time (Trellis Core S6.1).

### 11.2 Verifier Discovery of Active Algorithm Version

**Requirement class:** Companion requirement

A verifier reconstructing the historical state of a managed key MUST be able to determine, for any key-version, the active algorithm version at any past instant in that key-version's lifetime, using only canonical lifecycle facts and the immutable interpretation material those facts carry. A verifier MUST NOT be required to consult a live registry, a mutable reference, or out-of-band operator knowledge.

If algorithm-version identification depends on an external registry, the implementation MUST embed sufficient registry material (digests, snapshots, or signed registry slices) into or alongside each key-lifecycle fact such that the verifier can resolve the reference offline.

### 11.3 Export Verification Across Retired Algorithm Families

**Requirement class:** Companion requirement

When an export-issuance fact (§5.6) references an algorithm family that is later retired, the export verification package MUST remain verifiable under the algorithms and parameters in effect at issuance. The implementation MUST NOT delete, rotate away, or make inaccessible any interpretation material on which historical export verification depends without first migrating verifiability under an explicit migration mechanism (Trellis Core S16.6).

If an algorithm family is retired and no migration mechanism is provided, the implementation MUST disclose, in the export-issuance fact's recoverability metadata, that subsequent verification depends on preserved historical algorithm support.

---

## 12. Required Completeness Rule

**Requirement class:** Companion requirement

Crypto-shredding is not complete unless plaintext-derived projections and caches are purged according to the declared cascade policy. The normative definition of purge-cascade semantics and projection rebuild requirements lives in the Projection and Runtime Discipline companion (S4). This companion owns the evidence-artifact requirements for key-destruction (§6.1) and purge-cascade completion; it defers to Projection S4 for what must be purged and how rebuild correctness is verified.

Purge-cascade completion MUST produce verifiable evidence artifacts tied to canonical checkpoint state. Purge-cascade evidence MUST NOT reveal plaintext content of purged projections.

---

## 13. Security and Privacy Considerations

This section is normative. Generic privacy-disclosure obligations are defined in [WOS Assurance §6 Legal-Sufficiency Disclosure Obligations]; the items below cover ledger-specific cryptographic threats.

- Key destruction and crypto-shredding are irreversible. Recovery from destruction requires separate recovery-key infrastructure that MUST NOT reuse destroyed key material.
- Grace-period windows (§4) create intervals where both old and new key material are valid. Deployments MUST declare and bound these intervals in the Trust Profile (Trust Profiles S6).
- Purge-cascade completion evidence MUST NOT reveal plaintext content of purged projections (Projection S4).
- Key-lifecycle facts are canonical facts. They MUST NOT be redacted from audit trails even after key destruction (Trellis Core S5.2 invariant 1).
- Threshold participation records (§9.3) and recovery facts (§10.1) reveal organizational topology. Deployments SHOULD account for this metadata exposure in the Trust Profile metadata budget (Trust Profiles S7).
- Cryptographic inaccessibility claims MUST include scope, authority, and effective-time semantics. Key-destruction claims MUST be distinguishable from payload-redaction or disclosure-filtering events. Historical verification across key evolution MUST remain possible where declared by policy.
