---
title: Trellis Companion — Export Verification Package
version: 0.1.0-draft.2
date: 2026-04-14
status: draft
---

# Trellis Companion — Export Verification Package v0.1

**Version:** 0.1.0-draft.2
**Date:** 2026-04-14
**Editors:** Formspec Working Group
**Companion to:** Trellis Core v0.1

---

## Status of This Document

This document is a **draft specification**. It is a companion to the Trellis Core specification and does not modify Formspec or WOS processing semantics. Implementors are encouraged to experiment with this specification and provide feedback, but MUST NOT treat it as stable for production use until a 1.0.0 release is published.

## Conventions and Terminology

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in BCP 14 [RFC 2119] [RFC 8174] when, and only when, they appear in ALL CAPITALS, as shown here.

JSON syntax and data types are as defined in [RFC 8259]. URI syntax is as defined in [RFC 3986].

Terminology in this document follows the Trellis Core controlled vocabulary (Trellis Core §4). The preferred terms — *author-originated fact*, *canonical fact*, *canonical record*, *canonical append attestation*, *derived artifact*, *disclosure or export artifact*, *append-head reference* — are used without restatement.

## Abstract

The Export Verification Package companion defines normative requirements for offline-verifiable export packages assembled from Trellis canonical truth. It specifies required package contents, the verifier obligation set, export verification independence from runtime systems, claim classes that a package MAY support, provenance distinctions that a package MUST preserve, and algorithm agility requirements that keep historical exports verifiable across schema and algorithm evolution.

This companion adds export-packaging semantics to the Trellis verification layer defined in Trellis Core (§9) and to the Disclosure and Export Profile in the companion family. It does not define Formspec or WOS semantics and does not modify canonical truth, canonical order, canonical append attestation, or trust-honesty semantics established by Trellis Core.

## Table of Contents

1. Introduction
2. Conformance
3. Export Requirement
4. Export Package Contents
5. Verifier Obligations
6. Export Verification Independence
7. Provenance Distinction Requirement
8. Claim Classes
9. Selective Disclosure Discipline
10. Algorithm Agility and Historical Verifiability
11. Profile-Scoped Export Honesty
12. Cross-Implementation Verification
13. Relationship to the Disclosure Manifest
14. Security and Privacy Considerations
15. Cross-References

---

## 1. Introduction

### 1.1 Scope

This companion defines what an Export Verification Package MUST contain, what a verifier MUST be able to establish from it, and what the package MUST NOT depend on. It is the normative home for export packaging semantics in the Trellis specification family.

Trellis Core §9 defines verification requirements on canonical truth in the abstract. This companion defines the **package shape** — the set of materials a producer assembles so that an offline verifier can discharge those obligations without access to the producing service.

### 1.2 Out of Scope

This companion does not define:

- byte-level canonicalization, proof encodings, or transport envelopes (binding concerns),
- disclosure-manifest structure or audience-scoped claim selection (see `trellis/specs/export/disclosure-manifest.md`),
- trust-profile declaration semantics (see `trellis/specs/trust/trust-profiles.md`),
- monitoring, witnessing, or anchoring services (see `trellis/specs/operations/monitoring-witnessing.md`),
- Formspec or WOS processing semantics.

### 1.3 Design Goal

An Export Verification Package MUST be a self-contained verifiable object. The producer disappears; the canonical append service goes offline; the operational database is discarded. The package, together with any immutable external references it names, MUST remain sufficient for a conforming Verifier to validate every claim class the package asserts.

---

## 2. Conformance

### 2.1 Conformance Roles

This companion defines requirements for two Trellis conformance roles (Trellis Core §2.1):

1. **Export Generator** — assembles Export Verification Packages from canonical truth.
2. **Verifier** — validates Export Verification Packages offline.

A system conforms to this companion only if it satisfies every applicable MUST requirement for each role it claims, in addition to the corresponding Trellis Core requirements for that role.

### 2.2 Requirement Classes

Each normative requirement in this companion is tagged with one of:

- **Constitutional semantic** — derived from or preserving Trellis Core invariants. A conflict with core governs in favor of core (Trellis Core §2.3).
- **Profile constraint** — scoped to the Disclosure and Export Profile or a narrower declared profile.

### 2.3 Profile Subordination

**Requirement class:** Profile constraint

A profile or binding that defines audience-specific, family-specific, or deployment-specific export rules MUST remain subordinate to this companion and to Trellis Core §9. It MUST NOT weaken any MUST requirement in this document, and it MUST NOT reinterpret the claim classes in §8 in a way that permits a package to imply support for a claim class it cannot verify.

---

## 3. Export Requirement

**Requirement class:** Constitutional semantic

A conforming implementation MUST support independently verifiable exports for at least one declared scope of canonical truth (Trellis Core §9).

The declared export scope MUST identify the canonical append scope from which the package is drawn. Inclusion, consistency, position, and export claims apply only within that declared scope (Trellis Core §7.4).

---

## 4. Export Package Contents

**Requirement class:** Constitutional semantic

An Export Verification Package MUST include sufficient material for an offline Verifier to validate every claim class the package asserts over the declared export scope. At minimum, the package MUST include:

1. **Canonical records.** The canonical records, or their declared canonical representations, that fall within the declared export scope.
2. **Canonical append attestations.** The canonical append attestations, or equivalent append-attested proof material, covering those canonical records within the declared scope.
3. **Verification keys.** The verification keys, or immutable content-addressed references to them, required to validate authored signatures and canonical append attestations that the package presents.
4. **Append proofs.** Inclusion proofs for each exported canonical record, and consistency proofs between the append-head references required to establish the package's claimed position within canonical order, where consistency proofs are required by the active profile.
5. **Schema and semantic digests.** Schema or semantic digests for every canonical record representation and authored artifact included in the package, together with either embedded copies of those schemas and semantic definitions or immutable content-addressed references to them.
6. **Protected payload material.** Protected payload references or included payload bodies for the exported scope where applicable, consistent with the active Trust Profile and the disclosure manifest (if present).
7. **Canonical facts.** Canonical facts relevant to the exported scope where required for claim verification (for example, grant, revocation, or lifecycle facts that affect authorization history or compliance posture claims asserted by the package).
8. **Trust Profile reference.** The active Trust Profile declaration, or an immutable reference to it, where the package asserts any claim class whose verification depends on custody posture (see `trellis/specs/trust/trust-profiles.md`).

Any reference required for offline verification — including key references, schema references, semantic references, profile references, and anchoring references — MUST be **immutable**, **content-addressed**, or **included in the package itself**. Mutable references (service URLs, registry lookups resolved at verification time, out-of-band operator knowledge) MUST NOT be relied on for offline verification.

A package MAY include optional external anchoring references (for example, transparency-log anchors or timestamping receipts). Such references MUST be clearly marked as optional external proof material and MUST NOT be required for baseline package verification unless a declared profile explicitly requires them (see `trellis/specs/operations/monitoring-witnessing.md`).

---

## 5. Verifier Obligations

**Requirement class:** Constitutional semantic

A conforming Verifier, given an Export Verification Package and its declared export scope, MUST be able to:

1. **Verify authored authentication.** Verify authored signatures, or equivalent authored authentication, over every author-originated fact the package presents, where such authentication is required by the active profile or binding.
2. **Verify canonical inclusion.** Verify canonical inclusion of each exported canonical record within the declared append scope, using the included append proofs and verification keys.
3. **Verify append-head consistency.** Verify consistency between append-head references that the package declares or relies upon, where consistency proofs are required by the active profile.
4. **Verify schema and semantic integrity.** Verify schema or semantic digests against the embedded copies or immutable references carried in the package, establishing that canonical records and authored artifacts are interpreted under the schema and semantic definitions in effect when they were produced. Schema-digest construction and impact classification follow upstream rules; see [Formspec Changelog §4] (`/specs/registry/changelog-spec.md`) and [Formspec Core §6.4 Pinning Rule VP-01] (`/specs/core/spec.md`) for the canonical version-pinning and change-classification semantics that govern historical reinterpretation.
5. **Verify included disclosure or export artifacts.** Verify any disclosure or export artifacts embedded in the package — including any Disclosure Manifest (see §13 and `trellis/specs/export/disclosure-manifest.md`) — against the canonical records and attestations they reference, without treating those derived artifacts as canonical truth.

A Verifier MUST discharge each of these obligations using only material carried in the package, material referenced immutably by the package, and optional external proof material the package explicitly names. A Verifier MUST NOT require access to the producing service to complete any of these obligations.

A package MUST either carry material sufficient for each of the above obligations or MUST explicitly declare which obligations are out of scope for that package. A package MUST NOT assert a claim class whose corresponding verifier obligation it does not support (§8).

---

## 6. Export Verification Independence

**Requirement class:** Constitutional semantic

Export verification MUST be independent of the producing service and of any runtime system. A conforming Export Verification Package MUST NOT require, for any verifier obligation in §5, access to:

1. **Derived artifacts** — projections, evaluator state, caches, timelines, indexes, or other rebuildable interpretations of canonical truth.
2. **Workflow runtime state** — operational workflow state, task queues, orchestration state, or session state.
3. **Mutable service databases** — any mutable operational database maintained by the producing service or by a downstream service.
4. **Ordinary service APIs** — live API access to the producing service, the canonical append service, or any downstream service, beyond what the export package explicitly references as **optional external proof material**.

The Verifier MUST be able to complete its obligations on an air-gapped system given only the package and the optional external proof material the package names.

### 6.1 Payload-Absent Claim Honesty

**Requirement class:** Constitutional semantic

If a package omits payload readability — for example because protected payloads are reader-held, because payloads are referenced rather than included, or because a disclosure manifest redacts content — the package MUST still explicitly disclose which integrity, provenance, and append claims remain verifiable over the exported scope.

A package MUST NOT assert or imply verifiability of any claim class for which the required material is absent, redacted, or inaccessible to the Verifier.

---

## 7. Provenance Distinction Requirement

**Requirement class:** Constitutional semantic

An Export Verification Package MUST preserve the distinctions among the primary object classes defined in Trellis Core §5.1. Specifically, the package MUST keep distinguishable:

1. author-originated facts,
2. canonical records,
3. canonical append attestations,
4. disclosure or export artifacts assembled from the above.

A package MUST NOT collapse a canonical record into the author-originated fact it represents, MUST NOT collapse a canonical append attestation into the canonical record it attests to, and MUST NOT represent a disclosure or export artifact as canonical truth. These distinctions MUST survive any packaging, selective disclosure, or presentation applied by the producer.

---

## 8. Claim Classes

**Requirement class:** Profile constraint

A Disclosure and Export Profile SHOULD state which of the following claim classes are verifiable within that profile, and an Export Verification Package SHOULD declare, for each claim class it asserts, which package contents support it.

| Claim class | Question the verifier answers | Package material required |
| --- | --- | --- |
| **Authorship** | Was this fact originated by the claimed actor under the active authentication model? | Authored signatures or equivalent authored authentication; verification keys or immutable key references. |
| **Append / inclusion** | Was this canonical record admitted into canonical order within the declared append scope? | Canonical record; canonical append attestation; inclusion proof; append-head reference. |
| **Payload integrity** | Has the protected payload, or its reference, remained bit-for-bit consistent with what was canonically admitted? | Payload body or immutable payload reference; schema or semantic digest; attestation linking payload digest to canonical record. |
| **Authorization history** | What grants, revocations, and delegations were canonical at a given append-head? | See [WOS Governance §provenanceLayer attachment (§§1.4, 11)] (`wos-spec/specs/governance/workflow-governance.md`) for grant/revocation/delegation provenance; package material for these facts is canonical-fact append proofs (§4 item 7). |
| **Disclosure** | What was disclosed to this audience, and with what redaction posture? | See `trellis/specs/export/disclosure-manifest.md`; provenance links MUST satisfy §7. |
| **Lifecycle / compliance** | What retention, legal-hold, sealing, or export-issuance state was canonical at a given boundary? | See [Formspec Respondent Ledger §6.6.1 Assurance levels] (`/specs/audit/respondent-ledger-spec.md`) and [WOS Assurance §6 Legal-Sufficiency Disclosure Obligations] (`wos-spec/specs/assurance/assurance.md`) for the upstream lifecycle and assurance vocabulary; package material is the corresponding canonical lifecycle facts (§4 item 7) and their append proofs. |

A package MUST NOT imply support for a claim class that it cannot verify. An implementation MUST NOT describe a package as verifying a claim class unless the package contents are sufficient, under §5, to discharge the Verifier obligation corresponding to that class.

---

## 9. Selective Disclosure Discipline

**Requirement class:** Profile constraint

Selective disclosure SHOULD be achieved through disclosure or export artifacts (see §13) rather than by overloading canonical records. A disclosure-oriented artifact:

- MAY present an audience-specific subset or presentation of canonical truth,
- MUST preserve the provenance distinctions of §7,
- MUST NOT be treated as a rewrite of canonical truth,
- MUST NOT cause a package to imply broader claim-class support than the disclosed subset can actually verify.

Selective disclosure MUST NOT leak the existence of undisclosed claims through side channels such as hash-inclusion proofs that reveal record counts or positional information beyond what the declared audience is intended to learn.

---

## 10. Algorithm Agility and Historical Verifiability

**Requirement class:** Constitutional semantic

An Export Verification Package exists to be verified at a time later than the time it was produced. Algorithm and schema evolution MUST NOT silently invalidate prior export verification, and it MUST NOT silently reinterpret historical records under newer rules (Trellis Core §12.2; companion Appendix C).

A conforming Export Generator:

1. MUST version canonical algorithms and any schema or semantic digests, embedded copies, or immutable references needed for historical verification of the exported scope,
2. MUST version canonical record semantics, append semantics, export-verification semantics, and trust-profile semantics referenced by the package,
3. MUST preserve in the package enough immutable interpretation material to verify the exported records under the algorithms and rules in effect when they were produced,
4. MUST NOT rely on live registry lookups, mutable references, or out-of-band operator knowledge for historical verification,
5. MUST NOT silently reinterpret historical records under newer rules without an explicit migration mechanism, and MUST NOT emit a package that causes a Verifier to do so,
6. MUST ensure that algorithm or schema evolution at the producing service does not invalidate prior export verification of previously issued packages.

A conforming Verifier MUST validate an Export Verification Package under the algorithms, schemas, and semantic references the package itself carries or immutably names, not under the Verifier's current defaults.

See also: `trellis/specs/trust/key-lifecycle-operating-model.md` for key and algorithm lifecycle obligations that underwrite historical verifiability of authored signatures and append attestations.

---

## 11. Profile-Scoped Export Honesty

**Requirement class:** Profile constraint

A profile-scoped export MAY present a profile-specific timeline, delta history, or audience-specific interpretation, but MUST preserve the provenance distinctions of §7 and MUST NOT imply stronger confidentiality, weaker provider visibility, or weaker recovery capability than the active Trust Profile supports (see `trellis/specs/trust/trust-profiles.md`).

The generic no-overclaim discipline that bounds what a profile-scoped artifact may assert about workflow, governance, compliance, or disclosure coverage is defined upstream; see [WOS Assurance §6 Legal-Sufficiency Disclosure Obligations] (`wos-spec/specs/assurance/assurance.md`). This companion does not restate that discipline.

---

## 12. Cross-Implementation Verification

**Requirement class:** Profile constraint

At least two independent Verifier implementations SHOULD be able to validate the same Export Verification Package fixture and produce equivalent claim outcomes for every claim class the package asserts.

Divergent outcomes between independent Verifiers on the same fixture indicate either a package-level ambiguity or a Verifier-level defect, and SHOULD be treated as a conformance issue in the responsible implementation.

Cross-implementation verification SHOULD be performed in isolated environments to prevent side-channel leakage between Verifier implementations, consistent with the security considerations in §14.

---

## 13. Relationship to the Disclosure Manifest

This companion and `trellis/specs/export/disclosure-manifest.md` define complementary but distinct artifacts.

- The **Disclosure Manifest** defines **what is disclosed and to whom** — audience scope, claim-class declarations against that audience, selective-disclosure discipline, payload readability posture, and redaction declarations. It is a release-level artifact.
- The **Export Verification Package** defines **what is offline-verifiable** — the canonical records, attestations, proofs, keys, and interpretation material required to discharge the Verifier obligations of §5 against the declared export scope.

A Disclosure Manifest MAY be included as a member of an Export Verification Package. When included:

1. The manifest's declared claim classes MUST be a subset of the claim classes the package can verify under §5 and §8.
2. The package MUST carry the material required to verify any provenance link the manifest asserts back to canonical records.
3. The manifest remains a disclosure or export artifact under Trellis Core §5.1; it MUST NOT be treated as canonical truth, and its inclusion MUST NOT alter the provenance distinctions of §7.

The **manifest-vs-package boundary**: the manifest governs **disclosure policy and audience scope**; the package governs **offline verifiability of canonical and disclosed material together**. Either artifact MAY exist without the other. A package without a manifest is a raw verifiable export. A manifest without a package is an audience-scoped disclosure declaration whose offline verifiability is not asserted by this companion.

---

## 14. Security and Privacy Considerations

### 14.1 Security

- Export Verification Packages MUST NOT include cryptographic secrets. Verification relies on public key material and canonical attestations (Trellis Core §9).
- Packages containing protected payloads or Trust Profile declarations MUST be protected in transit and at rest consistent with the active Trust Profile.
- Implementers SHOULD consider key compromise, verifier or parser divergence, replay and reordering, service equivocation, and snapshot misuse. External anchoring (§4, §4 optional references) MAY strengthen detection of equivocation.

### 14.2 Privacy

- Payload-absent claim honesty (§6.1) is a privacy requirement as well as an integrity requirement. A package MUST NOT reveal the existence, count, or position of redacted claims beyond what the disclosure policy declares.
- Cross-implementation verification (§12) SHOULD be performed in isolated environments to avoid side-channel leakage between Verifier implementations.
- Metadata visible in the package (for example, schema identifiers, profile identifiers, or append-head positions) SHOULD be minimized to what is required for verification, consistent with the metadata-minimization obligations of the Trellis family.
- Generic privacy-disclosure obligations (legal-sufficiency disclaimers, audience-scoped disclosure honesty independent of ledger mechanics) are defined upstream; see [WOS Assurance §6] (`wos-spec/specs/assurance/assurance.md`).

---

## 15. Cross-References

### 15.1 Trellis family

- **Trellis Core:** `trellis/specs/core/trellis-core.md` — canonical truth, invariants, verification requirements (§9), conformance roles (§2).
- **Disclosure Manifest:** `trellis/specs/export/disclosure-manifest.md` — audience-scoped disclosure; boundary defined in §13.
- **Trust Profiles:** `trellis/specs/trust/trust-profiles.md` — custody postures that determine what is exportable and under what confidentiality posture.
- **Key Lifecycle Operating Model:** `trellis/specs/trust/key-lifecycle-operating-model.md` — key and algorithm lifecycle that underwrites historical verifiability (§10).
- **Monitoring and Witnessing:** `trellis/specs/operations/monitoring-witnessing.md` — append-head consistency proofs and optional external anchoring (§4, §5.3).

### 15.2 Upstream specifications

- **WOS Kernel:** `wos-spec/specs/kernel/spec.md` — the workflow processing model whose Facts tier underlies canonical truth.
- **WOS Governance:** `wos-spec/specs/governance/workflow-governance.md` — `provenanceLayer` seam (§§1.4, 6.5) that hosts authorization-history facts (§8 claim-class table).
- **WOS Assurance:** `wos-spec/specs/assurance/assurance.md` — assurance levels, subject continuity, no-overclaim discipline (§§5–6) referenced from §11 and §14.2.
- **Formspec Core:** `/specs/core/spec.md` — definition versioning and Pinning Rule VP-01 (§6.4) referenced from §5 step 4.
- **Formspec Changelog:** `/specs/registry/changelog-spec.md` — change classification (§4) referenced from §5 step 4 for schema-digest semantics.
- **Formspec Respondent Ledger:** `/specs/audit/respondent-ledger-spec.md` — assurance-level vocabulary (§6.6.1) referenced from §8 lifecycle/compliance row.
