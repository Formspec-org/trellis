---
title: Trellis Companion — Assurance, Ledger Traceability, and CI
version: 0.1.0-draft.3
date: 2026-04-14
status: draft
---

# Trellis Companion — Assurance, Ledger Traceability, and CI v0.1

**Version:** 0.1.0-draft.3
**Date:** 2026-04-14
**Editors:** Formspec Working Group
**Companion to:** Trellis Core v0.1

---

## Status of This Document

This document is a **draft specification**. It is a companion to the Trellis Core specification and does not modify Formspec or WOS processing semantics. Implementors are encouraged to experiment with this specification and provide feedback, but MUST NOT treat it as stable for production use until a 1.0.0 release is published.

This companion is scoped to **ledger-layer assurance**: invariant-to-method mapping, continuous-integration expectations, and evidence retention for Trellis normative invariants. Identity, attestation, assurance-level methodology, and subject-continuity semantics are defined upstream (see §3) and are not restated here.

## Conventions and Terminology

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in BCP 14 [RFC 2119] [RFC 8174] when, and only when, they appear in ALL CAPITALS, as shown here.

JSON syntax and data types are as defined in [RFC 8259].

Each normative section in this document carries a **Requirement class** marker indicating whether the requirement is a **Constitutional semantic** (inherited from Trellis Core and restated here for traceability) or a **Companion requirement** (introduced by this companion, subordinate to the core).

## Abstract

This companion governs **ledger-layer assurance** for Trellis deployments: the methodology by which implementations demonstrate that the Trellis normative invariants (core, projection, monitoring) actually hold, the continuous-integration expectations that operationalize that methodology, and the operational traceability matrix (Appendix A) that maps each invariant to concrete verification methods.

It does not define identity, attestation, assurance-level, or subject-continuity semantics. Those are defined by [WOS Assurance] and, where Formspec-layer respondent identity is concerned, by the [Formspec Respondent Ledger] spec. Trellis deployments that bind identity or attestation facts into canonical truth MUST comply with those upstream specs; this companion inherits their obligations by cross-reference rather than restating them.

## Table of Contents

1. Introduction
2. Terminology
3. Identity, Attestation, and Assurance Methodology (cross-reference)
4. Assurance-Upgrade Facts (cross-reference)
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

- the **invariant scope** covered by ledger-layer assurance (§5);
- the **assurance methodology** catalog (models, property tests, fixtures, drills, fuzzing) available for each invariant (§5.3);
- the **minimum CI expectations** that operationalize the methodology (§6);
- the **evidence retention policy** that preserves the artifacts produced (§7);
- the **operational traceability matrix** mapping each Trellis normative invariant to its verification method (Appendix A).

Identity, attestation, assurance-level, and subject-continuity semantics are **out of scope** and are governed upstream (§3).

### 1.2 Relationship to Trellis Core and Other Companions

**Requirement class:** Companion requirement

This companion is subordinate to Trellis Core. Nothing in this document alters canonical truth, canonical append semantics, trust honesty requirements, or export-verification guarantees established by the core.

This companion interacts with:

- **Trellis Core** — the source of the normative invariants enumerated in §5.2 and mapped in Appendix A.
- **Trellis Projection companion** — the source of projection-layer invariants (PRD-01 and §10 evaluator invariants, §14 purge-cascade).
- **Trellis Monitoring companion** — the source of witness / equivocation invariants.
- **WOS Assurance** and **Formspec Respondent Ledger** — upstream sources of identity, attestation, assurance-level, and continuity semantics inherited by cross-reference (§3).

---

## 2. Terminology

**Requirement class:** Companion requirement (definitions), Constitutional semantic (vocabulary discipline)

This companion uses the preferred terms established by the Trellis Core controlled vocabulary — *author-originated fact*, *canonical fact*, *canonical record*, *canonical append attestation*, *derived artifact*, *disclosure or export artifact*, and *append-head reference*. Normative sections MUST prefer these terms over casual alternatives.

Ledger-specific terms introduced by this companion:

### 2.1 Assurance Artifact

A machine-readable output of an assurance method (model check result, property-test report, fixture execution log, drill report, fuzz crash report, or equivalent) that is retained as evidence that a named invariant was exercised at a given build and time.

### 2.2 Primary Method / Secondary Method

For each invariant in §5.2, the **primary method** is the verification technique an implementation declares as its principal source of assurance for that invariant; a **secondary method** is any additional technique declared as corroborating evidence.

### 2.3 Upstream Identity and Assurance Terms

Assurance level, disclosure posture, subject-continuity reference, attestation, and assurance-upgrade fact are defined upstream. See [WOS Assurance §2] (assurance level, disclosure posture), [WOS Assurance §3] (attestation), [WOS Assurance §5] (subject continuity), and [Formspec Respondent Ledger §6.6, §6.6A] (Formspec-layer respondent identity and authored signatures). This companion does not redefine these terms.

---

## 3. Identity, Attestation, and Assurance Methodology

**Requirement class:** Companion requirement (cross-reference)

Identity, attestation, and assurance methodology are defined in [WOS Assurance §§2–6]. Formspec-Response-layer identity and authored signatures are defined in [Formspec Respondent Ledger §§6.6–6.8]. This spec does not restate those obligations; Trellis deployments MUST comply with them as applicable.

---

## 4. Assurance-Upgrade Facts

**Requirement class:** Companion requirement (cross-reference)

Assurance-upgrade facts (the canonical admission, subject-continuity binding, evidence-packaging, and non-rewrite obligations for forward-effective assurance-level changes) are defined in [WOS Assurance §2.3 Assurance-Upgrade Facts]. Trellis deployments that canonically admit such facts MUST comply with that specification.

---

## 5. Invariant Scope and Assurance Methodology

### 5.1 Purpose

**Requirement class:** Companion requirement

Map each Trellis normative invariant to concrete assurance methods so that assurance remains architectural, not appendix-only. Every invariant listed in §5.2 MUST be covered by at least one primary assurance method (§5.3) and produce retained evidence artifacts (§7).

### 5.2 Trellis Invariant Scope

**Requirement class:** Companion requirement

"Every Trellis normative invariant" in this companion refers to the following set:

1. **Trellis Core invariants** — the six canonical-truth invariants enumerated at `trellis-core.md §6.2` (Append-only Canonical History; No Second Canonical Truth; One Canonical Order per Governed Scope; One Canonical Event Hash Construction; Verification Independence; Append Idempotency). Mapped in Appendix A as `TRELLIS-INV-1` through `TRELLIS-INV-6`.
2. **Trellis Projection invariants** — the projection-layer invariants at Projection §5 (Derived Artifact Is Not Authoritative) and Projection §10 / §14 (evaluator-input traceability, rebuild behavior, fail-closed on stale state, canonical-semantics precedence, purge-cascade completeness). Mapped as `TRELLIS-PRD-01`, `TRELLIS-PRD-05`–`TRELLIS-PRD-08`, and `TRELLIS-PRD-13`.
3. **Trellis Monitoring invariants** — the witness-subordination invariant at Monitoring §3 and the equivocation evidence-format invariant at Monitoring §9. Mapped as `TRELLIS-MONITOR-1` and `TRELLIS-MONITOR-2`.

Additional invariants added by future Trellis companions MUST be registered in this list before assurance methods are required for them. Invariants owned by upstream specs (WOS Assurance, Formspec Respondent Ledger) are covered by those specs' own conformance matrices and are not duplicated here.

### 5.3 Assurance Methodology

**Requirement class:** Companion requirement

For each invariant in §5.2, the implementation MUST declare at least one **primary method** drawn from the following catalog, and SHOULD declare at least one **secondary method** where the catalog supports one:

- formal or semi-formal models (TLA+, Alloy, or equivalent);
- property-based tests;
- shared test vectors (fixtures), executed across each implementation substrate (for example, native and WASM);
- adversarial replay tests;
- rebuild-from-canonical drills;
- purge and destruction drills;
- parser and verifier fuzzing;
- cross-implementation offline verifier vectors.

The concrete mapping is maintained in Appendix A.

---

## 6. Minimum CI Expectations

**Requirement class:** Companion requirement

1. Every Trellis normative invariant in §5.2 MUST map to at least one automated check.
2. Hash and serialization vectors MUST execute in every substrate the implementation ships (at minimum, native and WASM where both exist).
3. Recovery, destruction, and rebuild drills MUST run on a recurring schedule and produce retained evidence artifacts (§7).
4. Fuzzing outcomes MUST feed parser and verifier hardening backlogs.

### 6.1 Role-Based Applicability

**Requirement class:** Companion requirement

| Requirement | Verifier implementations | Canonical Append Service | Studio / tooling |
|---|---|---|---|
| Invariant-to-check mapping | MUST | MUST | SHOULD |
| Native + WASM test vectors | MUST | SHOULD | MAY |
| Recovery / destruction / rebuild drills | SHOULD | MUST | MAY |
| Fuzzing backlogs | MUST | SHOULD | MAY |

---

## 7. Evidence Retention Policy

**Requirement class:** Companion requirement

- Assurance artifacts SHOULD be retained for at least one full major-version support window. The definition of "major-version support window" is an engineering decision, not a legal guarantee; jurisdictions MAY impose longer retention requirements.
- Artifacts MUST include build and version identifiers and execution timestamps.
- Failed assurance runs MUST be retained with remediation linkage. Failed runs MUST NOT be suppressed or deleted after remediation.
- Evidence artifacts covering export or disclosure paths MUST preserve the provenance distinctions required by Trellis Core when included in an export.

---

## 8. Conformance

**Requirement class:** Companion requirement

This companion defines the following conformance roles. An implementation MAY claim one or more roles and MUST satisfy all requirements applicable to each claimed role.

### 8.1 Assurance Producer

Implements automated checks, drills, and fuzzing for the Trellis normative invariants registered in §5.2. MUST map every registered invariant to at least one primary assurance method and produce evidence artifacts consistent with §7.

### 8.2 Assurance Auditor

Reviews evidence artifacts, remediation linkage, and retention compliance. MUST verify that evidence artifacts include build and version identifiers and execution timestamps, and MUST verify that failed runs are retained with remediation linkage.

### 8.3 Upstream Identity and Assurance Roles

Identity/assurance-related conformance obligations are inherited from [WOS Assurance §§2–6]. Trellis deployments that admit identity, attestation, or assurance-upgrade facts canonically MUST satisfy the applicable WOS Assurance obligations; this companion does not restate them.

---

## 9. Security and Privacy Considerations

**Requirement class:** Companion requirement (ledger-specific disclosure obligations); non-normative guidance (threat enumeration)

Generic privacy-disclosure obligations for identity, attestation, and subject-continuity material in evidence artifacts are defined in [WOS Assurance §6] and MUST be satisfied by Trellis deployments that include such material in assurance evidence. The ledger-specific obligations below are additional.

### 9.1 Assurance-Artifact Sensitivity

Assurance artifacts MAY contain sensitive implementation details. Evidence retention policy (§7) MUST account for access control. Failed assurance runs MUST NOT be suppressed or deleted; they MUST be retained with remediation linkage even when the underlying issue has been resolved. Fuzzing corpora and crash reports SHOULD be treated as sensitive artifacts.

### 9.2 Cross-Jurisdiction Retention

Retained assurance artifacts MAY be subject to differing data-protection, retention, and disclosure regimes depending on where they are stored. Implementations:

- MUST document the jurisdictional scope assumed by their retention policy (§7);
- SHOULD avoid co-locating high-sensitivity assurance artifacts with low-sensitivity artifacts whose broader access is expected.

### 9.3 Threat Enumeration (non-normative)

Implementers should consider at least: verifier divergence; replay and reordering of canonical append attestations; snapshot misuse of retained fixtures; equivocation in canonical append attestation; silent rebuild divergence between substrates; stale evaluator state not failing closed.

---

## 10. Cross-References

**Requirement class:** Companion requirement (normative citations)

This companion normatively cross-references the following documents. Where this companion's requirements depend on behavior defined elsewhere, the cited document governs.

- **Trellis Core Specification** (`trellis/specs/core/trellis-core.md`) — constitutional semantics, canonical truth, admission, hash construction, verification requirements; source of `TRELLIS-INV-1` through `TRELLIS-INV-6` (§6.2).
- **Trellis Projection companion** (`trellis/specs/projection/...`) — source of `TRELLIS-PRD-01` (§5), `TRELLIS-PRD-05`–`TRELLIS-PRD-08` (§10), and `TRELLIS-PRD-13` (§14).
- **Trellis Monitoring companion** (`trellis/specs/monitoring/...`) — source of `TRELLIS-MONITOR-1` (§S3) and `TRELLIS-MONITOR-2` (§S9).
- **[WOS Assurance]** (`work-spec/specs/assurance/assurance.md`) — identity, attestation, assurance level, disclosure posture, subject continuity, assurance-upgrade facts (§§2–6) inherited by Trellis deployments.
- **[Formspec Respondent Ledger]** (`specs/audit/respondent-ledger-spec.md`) — Formspec-layer respondent identity, authored signatures, and ledger-scoped obligations (§§6.6–6.8).

---

## Appendix A: Operational Traceability Matrix

Implementations SHOULD maintain this matrix mapping Trellis normative invariants to their verification methods. Fixture IDs are implementation-declared; the invariant IDs and source §s are normative.

| Invariant ID | Source § | Summary | Verification Method |
|---|---|---|---|
| `TRELLIS-INV-1` | trellis-core.md §6.2 | Append-only Canonical History | property test / model check |
| `TRELLIS-INV-2` | trellis-core.md §6.2 | No Second Canonical Truth | property test |
| `TRELLIS-INV-3` | trellis-core.md §6.2 | One Canonical Order per Governed Scope | property test |
| `TRELLIS-INV-4` | trellis-core.md §6.2 | One Canonical Event Hash Construction | fixture |
| `TRELLIS-INV-5` | trellis-core.md §6.2 | Verification Independence | fixture |
| `TRELLIS-INV-6` | trellis-core.md §6.2 | Append Idempotency | property test |
| `TRELLIS-PRD-01` | projection §5 | Derived Artifact Is Not Authoritative | property test |
| `TRELLIS-PRD-05` | projection §10 | Evaluator Inputs Traceable to Canonical Facts | fixture |
| `TRELLIS-PRD-06` | projection §10 | Evaluator Rebuild Behavior Defined | fixture |
| `TRELLIS-PRD-07` | projection §10 | Stale Evaluator State Fail-Closed | fixture |
| `TRELLIS-PRD-08` | projection §10 | Canonical Semantics Prevail Over Evaluator State | fixture |
| `TRELLIS-PRD-13` | projection §14 | Purge-Cascade Completeness | fixture |
| `TRELLIS-MONITOR-1` | monitoring §3 | Witness Subordination to Canonical Correctness | fixture |
| `TRELLIS-MONITOR-2` | monitoring §9 | Equivocation Evidence Format Validity | fixture |

Note: `trellis-core.md §6.2` numbers the six canonical-truth invariants 1–6 (Append-only Canonical History, No Second Canonical Truth, One Canonical Order per Governed Scope, One Canonical Event Hash Construction, Verification Independence, Append Idempotency); `§6.3` separately names object-distinction invariants `A`–`F`. The `TRELLIS-INV-1` through `TRELLIS-INV-6` IDs above use a stable convention aligned with `§6.2`; implementations MUST update the matrix when the trellis-core numbering changes.
