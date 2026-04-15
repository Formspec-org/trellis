---
title: Trellis Core Requirements Matrix
version: 0.3.0-draft.1
date: 2026-04-14
status: draft
companion-to: trellis-core.md
---

# Trellis Core Requirements Matrix

**Authoritative specification:** [`trellis-core.md`](trellis-core.md)
**Historical source (non-normative):** [`../../DRAFTS/unified_ledger_core.md`](../../DRAFTS/unified_ledger_core.md)
**Purpose:** enumerate every normative obligation in the Trellis Core family — obligations retained in `trellis-core.md` and obligations delegated to named companion specifications — and record the section anchor, requirement class, keyword, and owning specification for each.

---

## 1. Abstract

This document is the current **requirements matrix** for Trellis Core. It enumerates every normative obligation that governs a conforming Trellis implementation, whether that obligation is stated in [`trellis-core.md`](trellis-core.md) directly or delegated to a named companion specification. Each obligation carries a stable `ULCR-###` identifier, a BCP 14 keyword set, a requirement class, a section anchor in the authoritative specification, and the owning companion specification where normative ownership does not rest in core.

The matrix is the single index that downstream assurance mappings, conformance checklists, and implementation plans reference when they need to point at an individual Trellis obligation. A legacy-to-current cross-reference is retained in §4.3 for continuity with historical citations of `unified_ledger_core.md`, but the matrix itself is authored against the current specification family — it is no longer a migration artifact.

## 2. Status

This document is a **draft requirements matrix** aligned to [`trellis-core.md`](trellis-core.md) v0.1.0-draft.1 and its companions. It has no independent normative authority. Where a row's obligation text differs in wording from the authoritative specification, the authoritative specification governs.

This matrix is expected to track the authoritative specification family. When an obligation is withdrawn, its row is retained with a superseded-by note rather than renumbered.

## 3. How to Read This Matrix

### 3.1 Feature Table (ULCF)

A feature family (ULCF) groups obligations by capability area. Each ULCR belongs to exactly one ULCF.

### 3.2 Requirement Table (ULCR) Columns

| Column | Definition |
|---|---|
| **ULCR ID** | Stable requirement identifier. Never reused, never renumbered. |
| **ULCF ID** | The feature family this requirement belongs to. |
| **Feature Name** | Short human-readable feature-family label (duplicated from ULCF for readability). |
| **Requirement Summary** | Normalized paraphrase of the obligation. The normative text of record lives in the authoritative specification cited in the `§ Authoritative` column. |
| **Keyword** | BCP 14 keyword(s) carried by the obligation (`MUST`, `MUST NOT`, `SHALL`, `SHOULD`, `SHOULD NOT`, `MAY`, `REQUIRED`). |
| **Class** | `Constitutional` (invariant across all conforming implementations), `Profile` (applies only under a declared profile), `Binding` (applies only under a declared binding or reference choice). |
| **Status** | `Core` (normatively owned by `trellis-core.md`), `Delegated` (normative ownership has moved to a named companion; core retains only a pointer or contract statement), or `Companion` (obligation is authored directly in a companion and referenced from core). |
| **§ Authoritative** | Section anchor in the owning specification. When Status is `Core`, this is a `trellis-core.md` section; when Status is `Delegated` or `Companion`, this is the companion section. |
| **§ Legacy** | Section anchor in `unified_ledger_core.md` for continuity with historical citations. `—` when the obligation has no legacy antecedent. |
| **Owner** | The specification that now normatively owns the obligation. |

### 3.3 Two-Anchor Policy

Every row carries the authoritative anchor in the `§ Authoritative` column. The legacy anchor is recorded for traceability only; it has no normative force. Obligations introduced after the legacy draft appear with `—` in `§ Legacy`.

## 4. Conventions

### 4.1 Identifier Conventions

- `ULCF-###` — feature family identifier.
- `ULCR-###` — requirement identifier. Numbered sequentially through **ULCR-115** in this revision.
- IDs are stable across revisions. Withdrawn rows are retained with a superseded-by note rather than renumbered.

### 4.2 Section Anchor Conventions — `trellis-core.md`

Section anchors in the `§ Authoritative` column use the current structure of [`trellis-core.md`](trellis-core.md) v0.1.0-draft.1:

| § | Title |
|---|---|
| §1 | Introduction |
| §1.2 | Relationship to Formspec and WOS |
| §2 | Conformance |
| §3 | Terminology |
| §4 | Core Model |
| §5 | Canonical Truth and Invariants |
| §6 | Canonical Admission and Order |
| §7 | Canonical Hash Construction |
| §8 | Verification Requirements |
| §9 | Cross-Repository Authority Boundaries |
| §10 | Security and Privacy Considerations |

### 4.3 Legacy-to-Current Section Mapping

For continuity with historical citations, the following table maps section anchors in the legacy `unified_ledger_core.md` draft onto their current home. Rows with **Delegated** owners indicate that the obligation left `trellis-core.md` entirely and is normatively owned by a companion specification.

| Legacy § | Legacy Topic | Current Home |
|---|---|---|
| 2.3 | Profile and binding subordination | `trellis-core.md` §4.2 (Ontology Discipline) and §9 (Cross-Repository Authority) |
| 2.4 | Core Profile | `trellis-core.md` §2.1 (Conformance Roles) |
| 2.5.1–2.5.5 | Conformance role requirements | `trellis-core.md` §2.1, §2.2 |
| 3 | Core-to-implementation contracts | `trellis-core.md` §2.2 (Role Requirements), §4.2 (Ontology Discipline) |
| 4.10 | Controlled vocabulary | `trellis-core.md` §3 (Terminology) |
| 5.2 | Ontology discipline | `trellis-core.md` §4 (Core Model) |
| 6.1 | Canonical truth boundary | `trellis-core.md` §5.1 |
| 7.1 Invariants 1–2 (Authorship, Record) | Legacy ontology invariants | Subsumed by `trellis-core.md` §4.1 (Object Classes) + §5.2 (Named Core Invariants) |
| 7.1 Invariant 3 (Derived non-canonical) | Legacy ontology invariant | `trellis-core.md` §5.2 Invariant 2 (No Second Canonical Truth) + `projection-runtime-discipline.md` §Projection integrity policy |
| 7.1 Invariants 4–5 (Provider/Reader, Delegated Compute) | Legacy trust-posture invariants | `trust-profiles.md` §Trust honesty rule, §Operational trust disclosure requirements |
| 7.1 Invariant 6 (Disclosure vs Assurance) | Legacy disclosure/assurance invariant | `trust-profiles.md` §Verification posture declaration; `assurance-traceability.md` |
| 8.1–8.3 | Admission, object distinction, state machine | `trellis-core.md` §4.1, §6.1 |
| 9.1 | Canonical order | `trellis-core.md` §6.2 |
| 9.2 | Canonical append attestation | `trellis-core.md` §6.1 (admission), §8 (verification); concrete receipt format in `shared-ledger-binding.md` §Canonization rules, §Canonical receipt immutability |
| 9.3 | Serialization / proof-binding boundary | `shared-ledger-binding.md` §Canonization rules |
| 10.1 | Trust Profile minimum object semantics | `trust-profiles.md` §Profile declaration schema |
| 10.2 | Disclosure posture and assurance | `trust-profiles.md` §Verification posture declaration; `assurance-traceability.md` |
| 11.1 | Trust honesty | `trust-profiles.md` §Trust honesty rule, §Operational trust disclosure requirements |
| 11.2 | Trust Profile transitions | `trust-profiles.md` §Trust profile transitions |
| 12.1–12.2 | Export requirement, export contents | `export-verification-package.md` |
| 12.3 | Verification requirement | `trellis-core.md` §8 + `export-verification-package.md` |
| 12.4–12.5 | Provenance distinction, verification independence | `export-verification-package.md`; `disclosure-manifest.md` |
| 13.1–13.3 | Generic profile discipline, trust inheritance, profile-scoped export | `trellis-core.md` §4.2 (Ontology Discipline) + `trust-profiles.md` + `export-verification-package.md` |
| 14.1–14.6 | Standard profiles | `trust-profiles.md`, `export-verification-package.md` |
| 15.1–15.4 | Bindings, vocabulary placement, family bindings, sidecars | `shared-ledger-binding.md` |
| 16.1 | Derived artifact requirements | `projection-runtime-discipline.md` |
| 16.2 | Metadata minimization | `trellis-core.md` §10; operational detail in `trust-profiles.md` §Metadata Budget Requirement |
| 16.3 | Idempotency and rejection | `trellis-core.md` §6.3 |
| 16.4 | Storage and snapshot discipline | `projection-runtime-discipline.md`; durable-append boundary in `shared-ledger-binding.md` |
| 16.5 | Lifecycle and cryptographic inaccessibility | `key-lifecycle-operating-model.md` |
| 16.6 | Versioning and algorithm agility | `shared-ledger-binding.md` §Schema/version compatibility policy; registry in `trellis-core.md` §7 |
| 17.3 | Trust and privacy disclosure obligations | `trust-profiles.md` §Operational trust disclosure requirements |
| 18 | Non-normative guidance | Non-normative; no ULCR rows. |

### 4.4 Companion Specifications Referenced

| Short name | File |
|---|---|
| `trust-profiles.md` | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) |
| `shared-ledger-binding.md` | [`./shared-ledger-binding.md`](shared-ledger-binding.md) |
| `export-verification-package.md` | [`../export/export-verification-package.md`](../export/export-verification-package.md) |
| `disclosure-manifest.md` | [`../export/disclosure-manifest.md`](../export/disclosure-manifest.md) |
| `projection-runtime-discipline.md` | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) |
| `key-lifecycle-operating-model.md` | [`../trust/key-lifecycle-operating-model.md`](../trust/key-lifecycle-operating-model.md) |
| `monitoring-witnessing.md` | [`../operations/monitoring-witnessing.md`](../operations/monitoring-witnessing.md) |
| `assurance-traceability.md` | [`../assurance/assurance-traceability.md`](../assurance/assurance-traceability.md) |
| `companion-matrix` | [`./unified-ledger-companion-requirements-matrix.md`](unified-ledger-companion-requirements-matrix.md) |

### 4.5 Keyword Conventions

Keywords follow BCP 14. Compound keyword entries (e.g., `MUST / MUST NOT`) indicate that a single obligation combines positive and negative clauses that travel together in the source prose.

---

## 5. Feature-Family (ULCF) Table

| ULCF ID | Name | § Authoritative (primary) | Owner |
|---|---|---|---|
| ULCF-001 | Ontology discipline and companion subordination | `trellis-core.md` §4.2, §9 | `trellis-core.md` |
| ULCF-002 | Conformance roles | `trellis-core.md` §2.1, §2.2 | `trellis-core.md` |
| ULCF-003 | Core-to-implementation contracts | `trellis-core.md` §2.2, §4.2 | `trellis-core.md` |
| ULCF-004 | Terminology | `trellis-core.md` §3 | `trellis-core.md` |
| ULCF-005 | Core ontology (object classes) | `trellis-core.md` §4 | `trellis-core.md` |
| ULCF-006 | Canonical truth scope | `trellis-core.md` §5.1 | `trellis-core.md` |
| ULCF-007 | Named core invariants (structural) | `trellis-core.md` §5.2 | `trellis-core.md` |
| ULCF-008 | Fact admission and object distinction | `trellis-core.md` §4.1, §6.1 | `trellis-core.md` |
| ULCF-009 | Admission prerequisites and durable-append boundary | `trellis-core.md` §6.1; `shared-ledger-binding.md` §Canonization rules | `trellis-core.md` |
| ULCF-010 | Canonical order | `trellis-core.md` §6.2 | `trellis-core.md` |
| ULCF-011 | Canonical append attestation | `trellis-core.md` §6.1, §8 | `trellis-core.md` |
| ULCF-012 | Serialization / proof binding boundary | `shared-ledger-binding.md` §Canonization rules | `shared-ledger-binding.md` |
| ULCF-013 | Trust Profile minimum semantics | `trust-profiles.md` §Profile declaration schema | `trust-profiles.md` |
| ULCF-014 | Disclosure posture vs assurance | `trust-profiles.md` §Verification posture declaration; `assurance-traceability.md` | `trust-profiles.md` |
| ULCF-015 | Trust honesty | `trust-profiles.md` §Trust honesty rule | `trust-profiles.md` |
| ULCF-016 | Trust Profile transitions | `trust-profiles.md` §Trust profile transitions | `trust-profiles.md` |
| ULCF-017 | Export packages | `export-verification-package.md` | `export-verification-package.md` |
| ULCF-018 | Offline verification capabilities | `trellis-core.md` §8; `export-verification-package.md` | `export-verification-package.md` |
| ULCF-019 | Export provenance & verification independence | `export-verification-package.md` | `export-verification-package.md` |
| ULCF-020 | Companion subordination discipline | `trellis-core.md` §4.2, §9 | `trellis-core.md` |
| ULCF-021 | Profile trust inheritance & export honesty | `trust-profiles.md`; `export-verification-package.md` | `trust-profiles.md` |
| ULCF-022 | Offline Authoring Profile | `trust-profiles.md` | `trust-profiles.md` |
| ULCF-023 | Reader-Held Decryption Profile | `trust-profiles.md` | `trust-profiles.md` |
| ULCF-024 | Delegated Compute Profile | `trust-profiles.md` | `trust-profiles.md` |
| ULCF-025 | Disclosure and Export Profile | `export-verification-package.md`; `disclosure-manifest.md` | `export-verification-package.md` |
| ULCF-026 | User-Held Record Reuse Profile | `trust-profiles.md` | `trust-profiles.md` |
| ULCF-027 | Respondent History Profile | `trust-profiles.md` | `trust-profiles.md` |
| ULCF-028 | Bindings, vocabulary placement, sidecars | `shared-ledger-binding.md` | `shared-ledger-binding.md` |
| ULCF-029 | Derived artifacts & evaluators | `projection-runtime-discipline.md` | `projection-runtime-discipline.md` |
| ULCF-030 | Metadata minimization | `trellis-core.md` §10 | `trellis-core.md` |
| ULCF-031 | Idempotency & rejection | `trellis-core.md` §6.3 | `trellis-core.md` |
| ULCF-032 | Durable storage & snapshots | `projection-runtime-discipline.md`; `shared-ledger-binding.md` | `projection-runtime-discipline.md` |
| ULCF-033 | Lifecycle & cryptographic inaccessibility | `key-lifecycle-operating-model.md` | `key-lifecycle-operating-model.md` |
| ULCF-034 | Versioning & algorithm agility | `shared-ledger-binding.md` §Schema/version compatibility policy | `shared-ledger-binding.md` |
| ULCF-035 | Trust & privacy disclosure obligations | `trust-profiles.md` | `trust-profiles.md` |
| ULCF-036 | Integrator-critical ledger guarantees | `trellis-core.md` §§5–8 | `trellis-core.md` |
| ULCF-037 | Canonical receipt immutability | `shared-ledger-binding.md` §Canonical receipt immutability | `shared-ledger-binding.md` |
| ULCF-038 | Formspec / WOS integration boundaries | `trellis-core.md` §1.2, §9 | `trellis-core.md` |
| ULCF-039 | Canonical hash construction and registry | `trellis-core.md` §7 | `trellis-core.md` |
| ULCF-040 | Legacy trust-posture invariants (migrated) | `trust-profiles.md`; `assurance-traceability.md` | `trust-profiles.md` |

---

## 6. Requirement (ULCR) Table

### 6.1 Ontology Discipline and Companion Subordination (ULCR-001 – ULCR-005)

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-001 | ULCF-001 | Companion subordination | Companion specifications MAY refine object semantics but MUST NOT redefine canonical truth or canonical order semantics established in core. | MAY / MUST NOT | Constitutional | Core | `trellis-core.md` §4.2 | 2.3 | `trellis-core.md` |
| ULCR-002 | ULCF-001 | Companion subordination | Companion specifications MUST narrow or specialize core semantics rather than reinterpret them. | MUST | Constitutional | Core | `trellis-core.md` §4.2, §9 | 2.3 | `trellis-core.md` |
| ULCR-003 | ULCF-001 | Companion subordination | Companion specifications MUST NOT define a second canonical order for the same governed scope. | MUST NOT | Constitutional | Core | `trellis-core.md` §5.2 Invariant 3, §6.2 | 2.3 | `trellis-core.md` |
| ULCR-004 | ULCF-001 | Companion subordination | Companion specifications MUST NOT redefine canonical truth established by core. | MUST NOT | Constitutional | Core | `trellis-core.md` §5.1, §5.2 Invariant 2 | 2.3 | `trellis-core.md` |
| ULCR-005 | ULCF-001 | Companion subordination | On conflict between a companion specification and core, core governs. | MUST | Constitutional | Core | `trellis-core.md` §4.2 | 2.3 | `trellis-core.md` |

### 6.2 Conformance Roles (ULCR-006 – ULCR-029)

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-006 | ULCF-002 | Core conformance participation | A conforming implementation MUST claim one or more of the five conformance roles and MUST satisfy all requirements applicable to each claimed role. | MUST | Constitutional | Core | `trellis-core.md` §2.1 | 2.4 | `trellis-core.md` |
| ULCR-007 | ULCF-002 | Append-only participation | An implementation participating as a Canonical Append Service MUST preserve append-only semantics for canonical records. | MUST | Constitutional | Core | `trellis-core.md` §2.2, §5.2 Invariant 1 | 2.4 | `trellis-core.md` |
| ULCR-008 | ULCF-002 | Canonical / derived separation | An implementation MUST distinguish canonical records from derived artifacts across all claimed roles. | MUST | Constitutional | Core | `trellis-core.md` §4.1, §5.2 Invariant 2 | 2.4 | `trellis-core.md` |
| ULCR-009 | ULCF-002 | Exportability | An implementation claiming the Export Generator role MUST produce independently verifiable exports for at least one declared export scope. | MUST | Constitutional | Companion | `export-verification-package.md` | 2.4 | `export-verification-package.md` |
| ULCR-010 | ULCF-002 | Fact Producer attributability | A Fact Producer MUST emit attributable facts admissible under the active profile or binding. | MUST | Constitutional | Core | `trellis-core.md` §2.2 | 2.5.1 | `trellis-core.md` |
| ULCR-011 | ULCF-002 | Fact Producer authentication | A Fact Producer MUST sign or otherwise authenticate facts where the active profile or binding requires it. | MUST | Constitutional | Core | `trellis-core.md` §2.2 | 2.5.1 | `trellis-core.md` |
| ULCR-012 | ULCF-002 | Fact Producer causality | A Fact Producer MUST preserve causal references when applicable. | MUST | Constitutional | Core | `trellis-core.md` §2.2 | 2.5.1 | `trellis-core.md` |
| ULCR-013 | ULCF-002 | Fact Producer immutability | A Fact Producer MUST NOT rewrite previously emitted facts. | MUST NOT | Constitutional | Core | `trellis-core.md` §2.2 | 2.5.1 | `trellis-core.md` |
| ULCR-014 | ULCF-002 | Append Service admission | A Canonical Append Service MUST validate admissibility of candidate records under the active profile or binding. | MUST | Constitutional | Core | `trellis-core.md` §2.2, §6.1 | 2.5.2 | `trellis-core.md` |
| ULCR-015 | ULCF-002 | Append Service record formation | A Canonical Append Service MUST form canonical records for admitted facts. | MUST | Constitutional | Core | `trellis-core.md` §2.2, §4.1 | 2.5.2 | `trellis-core.md` |
| ULCR-016 | ULCF-002 | Append Service ordering | A Canonical Append Service MUST append canonical records to canonical order within the governed scope. | MUST | Constitutional | Core | `trellis-core.md` §2.2, §6.2 | 2.5.2 | `trellis-core.md` |
| ULCR-017 | ULCF-002 | Append Service attestation | A Canonical Append Service MUST issue canonical append attestations for admitted canonical records. | MUST | Constitutional | Core | `trellis-core.md` §2.2 | 2.5.2 | `trellis-core.md` |
| ULCR-018 | ULCF-002 | Append Service immutability | A Canonical Append Service MUST NOT rewrite prior canonical records. | MUST NOT | Constitutional | Core | `trellis-core.md` §2.2, §5.2 Invariant 1 | 2.5.2 | `trellis-core.md` |
| ULCR-019 | ULCF-002 | Append Service canonical scope | A Canonical Append Service MUST NOT treat workflow state, projections, or caches as canonical truth. | MUST NOT | Constitutional | Core | `trellis-core.md` §5.1, §5.2 Invariant 2 | 2.5.2 | `trellis-core.md` |
| ULCR-020 | ULCF-002 | Verifier authentication | A Verifier MUST verify authored authentication where required. | MUST | Constitutional | Core | `trellis-core.md` §2.2, §8 | 2.5.3 | `trellis-core.md` |
| ULCR-021 | ULCF-002 | Verifier append validity | A Verifier MUST verify canonical append attestation validity and inclusion consistency. | MUST | Constitutional | Core | `trellis-core.md` §8 | 2.5.3 | `trellis-core.md` |
| ULCR-022 | ULCF-002 | Verifier object distinction | A Verifier MUST distinguish author-originated facts, canonical records, canonical append attestations, and disclosure or export artifacts. | MUST | Constitutional | Core | `trellis-core.md` §4.1 | 2.5.3 | `trellis-core.md` |
| ULCR-023 | ULCF-002 | Verifier independence | A Verifier MUST NOT require access to derived runtime state to verify canonical integrity. | MUST NOT | Constitutional | Core | `trellis-core.md` §2.2, §5.2 Invariant 5, §8 | 2.5.3 | `trellis-core.md` |
| ULCR-024 | ULCF-002 | Derived Processor authoritative input | A Derived Processor MUST treat canonical records as its only authoritative input. | MUST | Constitutional | Core | `trellis-core.md` §2.2 | 2.5.4 | `trellis-core.md` |
| ULCR-025 | ULCF-002 | Derived Processor provenance | A Derived Processor MUST record sufficient provenance to support rebuild from canonical state. | MUST | Constitutional | Delegated | `projection-runtime-discipline.md` §Rebuild verification | 2.5.4 | `projection-runtime-discipline.md` |
| ULCR-026 | ULCF-002 | Derived Processor rebuildability | A Derived Processor MUST be discardable and rebuildable from canonical state without altering canonical truth. | MUST | Constitutional | Delegated | `projection-runtime-discipline.md` §Projection integrity policy | 2.5.4 | `projection-runtime-discipline.md` |
| ULCR-027 | ULCF-002 | Export Generator packaging | An Export Generator MUST package canonical records, canonical append attestations, and verification material per declared export scope. | MUST | Constitutional | Companion | `export-verification-package.md` | 2.5.5 | `export-verification-package.md` |
| ULCR-028 | ULCF-002 | Export Generator provenance | An Export Generator MUST preserve provenance distinctions when producing release artifacts. | MUST | Constitutional | Companion | `export-verification-package.md` | 2.5.5 | `export-verification-package.md` |
| ULCR-029 | ULCF-002 | Export Generator offline sufficiency | An Export Generator MUST include enough material for an offline verifier to validate the declared export scope. | MUST | Constitutional | Companion | `export-verification-package.md` | 2.5.5 | `export-verification-package.md` |

### 6.3 Core-to-Implementation Contracts (ULCR-030 – ULCR-036)

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-030 | ULCF-003 | Canonical Append Contract | Implementations MAY vary append, proof, or storage mechanisms but MUST preserve admission, canonical order, canonical record formation, and canonical append attestation semantics. | MAY / MUST | Constitutional | Core | `trellis-core.md` §2.2, §6 | 3 | `trellis-core.md` |
| ULCR-031 | ULCF-003 | Derived Artifact Contract | Derived artifacts MUST remain rebuildable from canonical truth and MUST NOT become authoritative for canonical facts. | MUST / MUST NOT | Constitutional | Core | `trellis-core.md` §4.1, §5.2 Invariant 2 | 3 | `trellis-core.md` |
| ULCR-032 | ULCF-003 | Workflow Contract | Workflow state MUST remain operational rather than canonical unless later represented as canonical records under the active profile or binding. | MUST | Constitutional | Core | `trellis-core.md` §5.1, §5.2 Invariant 5 | 3 | `trellis-core.md` |
| ULCR-033 | ULCF-003 | Authorization Contract | Grant and revocation semantics MUST remain canonical; evaluator state MUST remain derived. | MUST | Constitutional | Core | `trellis-core.md` §4.1, §5.1 | 3 | `trellis-core.md` |
| ULCR-034 | ULCF-003 | Trust Contract | Implementations MAY vary custody, key management, or delegated compute mechanisms but the active Trust Profile MUST continue to describe who can read, recover, delegate, attest, or administer access. | MAY / MUST | Constitutional | Delegated | `trust-profiles.md` §Profile declaration schema | 3 | `trust-profiles.md` |
| ULCR-035 | ULCF-003 | Export Contract | Implementations MAY vary export packaging and disclosure mechanisms but exports MUST preserve required provenance distinctions and verification claims. | MAY / MUST | Constitutional | Companion | `export-verification-package.md` | 3 | `export-verification-package.md` |
| ULCR-036 | ULCF-003 | Contracts stability | Bindings and implementations MUST preserve all core-to-implementation contracts when underlying mechanisms change. | MUST | Constitutional | Core | `trellis-core.md` §2.2, §4.2 | 3 | `trellis-core.md` |

### 6.4 Terminology, Ontology, and Canonical Truth Scope (ULCR-037 – ULCR-040)

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-037 | ULCF-004 | Controlled vocabulary | Normative sections MUST use the terminology defined in core when discussing canonical truth, records, attestations, and derived artifacts. | MUST | Constitutional | Core | `trellis-core.md` §3 | 4.10 | `trellis-core.md` |
| ULCR-038 | ULCF-005 | Core ontology | Normative sections MUST preserve distinctions among the primary object classes (author-originated facts, canonical records, canonical append attestations, derived artifacts, export/disclosure artifacts). | MUST | Constitutional | Core | `trellis-core.md` §4.1 | 5.2 | `trellis-core.md` |
| ULCR-039 | ULCF-005 | Core ontology | Normative sections MUST NOT collapse derived or disclosure/export artifacts into canonical truth. | MUST NOT | Constitutional | Core | `trellis-core.md` §4.1, §5.2 Invariant 2 | 5.2 | `trellis-core.md` |
| ULCR-040 | ULCF-006 | Canonical truth scope | Implementations MUST NOT treat derived artifacts, workflow runtime state, authorization evaluator state, indexes, caches, or unrecorded delegated-compute outputs as authoritative for canonical facts. | MUST NOT | Constitutional | Core | `trellis-core.md` §5.1 | 6.1 | `trellis-core.md` |

### 6.5 Named Core Invariants — `trellis-core.md` §5.2 (ULCR-041 – ULCR-046)

These six rows track the six named structural invariants in `trellis-core.md` §5.2. The legacy `unified_ledger_core.md` §7.1 defined a different set of six invariants covering authorship, record, derived-artifact, and trust-posture distinctions; the ontology portions of those legacy invariants are now expressed in `trellis-core.md` §4 and §5, and the trust-posture portions have migrated to `trust-profiles.md` and `assurance-traceability.md` — those migrations are recorded under ULCF-040 as ULCR-110 – ULCR-114 below.

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-041 | ULCF-007 | Invariant 1 — Append-only Canonical History | Canonical records MUST NOT be rewritten in-place. | MUST NOT | Constitutional | Core | `trellis-core.md` §5.2 Invariant 1 | 7.1 (partial) | `trellis-core.md` |
| ULCR-042 | ULCF-007 | Invariant 2 — No Second Canonical Truth | Derived artifacts MUST NOT be treated as canonical truth. | MUST NOT | Constitutional | Core | `trellis-core.md` §5.2 Invariant 2 | 7.1 Invariant 3 | `trellis-core.md` |
| ULCR-043 | ULCF-007 | Invariant 3 — One Canonical Order per Governed Scope | Exactly one canonical append-attested order MAY exist per governed scope. | MAY (unique-existence) / MUST NOT (no second) | Constitutional | Core | `trellis-core.md` §5.2 Invariant 3, §6.2 | — | `trellis-core.md` |
| ULCR-044 | ULCF-007 | Invariant 4 — One Canonical Event Hash Construction | Canonical append semantics MUST bind to exactly one canonical hash construction. | MUST | Constitutional | Core | `trellis-core.md` §5.2 Invariant 4, §7 | — | `trellis-core.md` |
| ULCR-045 | ULCF-007 | Invariant 5 — Verification Independence | Canonical verification MUST NOT depend on workflow runtime internals. | MUST NOT | Constitutional | Core | `trellis-core.md` §5.2 Invariant 5, §8 | — | `trellis-core.md` |
| ULCR-046 | ULCF-007 | Invariant 6 — Append Idempotency | Equivalent admitted canonical inputs MUST NOT create duplicate canonical order positions. | MUST NOT | Constitutional | Core | `trellis-core.md` §5.2 Invariant 6, §6.3 | — | `trellis-core.md` |

### 6.6 Fact Admission, Object Distinction, and Canonical Order (ULCR-047 – ULCR-060)

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-047 | ULCF-008 | Object distinction | Implementations MUST keep distinguishable: author-originated fact, canonical record, canonical append attestation, derived artifact, disclosure or export artifact. | MUST | Constitutional | Core | `trellis-core.md` §4.1 | 8.1 | `trellis-core.md` |
| ULCR-048 | ULCF-008 | Object distinction | A canonical record MUST remain distinguishable from the underlying authored content it represents. | MUST | Constitutional | Core | `trellis-core.md` §4.1, §3 | 8.1 | `trellis-core.md` |
| ULCR-049 | ULCF-008 | Object distinction | A disclosure or export artifact MUST NOT be treated as identical to the underlying canonical record it may reference. | MUST NOT | Constitutional | Core | `trellis-core.md` §4.1 | 8.1 | `trellis-core.md` |
| ULCR-050 | ULCF-008 | Admissibility narrowing | Companions MAY narrow admissibility (subset, predicates, actors) but MUST NOT reinterpret categories in a way that changes canonical truth or creates an alternate canonical order. | MAY / MUST NOT | Profile | Core | `trellis-core.md` §4.2, §6.1 | 8.2 | `trellis-core.md` |
| ULCR-051 | ULCF-009 | Durable-append boundary | A fact becomes canonical only when its canonical record has crossed the binding-declared durable-append boundary. | MUST (semantic) | Constitutional | Delegated | `shared-ledger-binding.md` §Canonization rules | 8.3 | `shared-ledger-binding.md` |
| ULCR-052 | ULCF-009 | Scope of attestation | A canonical append attestation proves inclusion and order under the active append model; by itself it does not prove the substantive correctness of the underlying content beyond the scope of admission and attestation. | (scope statement) | Constitutional | Core | `trellis-core.md` §8 | 8.3 | `trellis-core.md` |
| ULCR-053 | ULCF-010 | Canonical order | Canonical append order MUST be monotonically append-only within governed scope. | MUST | Constitutional | Core | `trellis-core.md` §6.2 | 9.1 | `trellis-core.md` |
| ULCR-054 | ULCF-010 | Canonical order scope | Canonical order MUST have a declared scope; inclusion, consistency, position, and export claims apply only within that scope. | MUST | Constitutional | Core | `trellis-core.md` §6.2 | 9.1 | `trellis-core.md` |
| ULCR-055 | ULCF-010 | Canonical order single source | The canonical append-attestation stream (or its equivalent) MUST be the single ordered source of truth for canonical record inclusion and sequence within a governed scope. | MUST | Constitutional | Core | `trellis-core.md` §5.2 Invariant 3, §6.2 | 9.1 | `trellis-core.md` |
| ULCR-056 | ULCF-010 | No alternate canonical order | No workflow runtime, projection, authorization evaluator, or collaboration layer MAY define an alternate canonical order for the same governed scope. | MUST NOT | Constitutional | Core | `trellis-core.md` §5.2 Invariant 3, §6.2 | 9.1 | `trellis-core.md` |
| ULCR-057 | ULCF-011 | Append attestation issuance | A Canonical Append Service MUST return a canonical append attestation for canonical records that have crossed the durable-append boundary. | MUST | Constitutional | Core | `trellis-core.md` §2.2, §6.1 | 9.2 | `trellis-core.md` |
| ULCR-058 | ULCF-011 | Append attestation timing | A Canonical Append Service MUST NOT issue a canonical append attestation before the durable-append boundary has been crossed. | MUST NOT | Constitutional | Core | `trellis-core.md` §6.1 | 9.2 | `trellis-core.md` |
| ULCR-059 | ULCF-011 | Append attestation contents | A canonical append attestation MUST include or reference the canonical append position, inclusion-oriented proof material, an append-head reference, and sufficient verifier metadata to validate canonical inclusion. | MUST | Constitutional | Delegated | `shared-ledger-binding.md` §Canonization rules | 9.2 | `shared-ledger-binding.md` |
| ULCR-060 | ULCF-012 | Serialization binding | Where a binding declares deterministic encodings, canonical byte sequences, exact proof formats, or API procedures, conforming implementations for that binding MUST follow it. | MUST | Binding | Delegated | `shared-ledger-binding.md` §Canonization rules | 9.3 | `shared-ledger-binding.md` |

### 6.7 Trust Profile Semantics, Honesty, and Transitions (ULCR-061 – ULCR-065)

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-061 | ULCF-013 | Trust Profile object | A Trust Profile MUST semantically include the minimum fields (profile identifier, scope, ordinary-operation readability posture, reader-held and delegated-compute postures, current and historical decryption authorities, recovery authorities and conditions, append-attestation control authorities, exceptional-access authorities, metadata visibility). | MUST | Constitutional | Delegated | `trust-profiles.md` §Profile declaration schema | 10.1 | `trust-profiles.md` |
| ULCR-062 | ULCF-013 | Trust Profile wire shape | Bindings MAY define the concrete wire shape of a Trust Profile object but MUST preserve the minimum semantic fields and their meanings. | MAY / MUST | Binding | Delegated | `trust-profiles.md` §Profile declaration schema | 10.1 | `trust-profiles.md` |
| ULCR-063 | ULCF-014 | Disclosure vs assurance | Implementations MUST distinguish assurance from disclosure posture, MUST NOT treat higher assurance as requiring greater identity disclosure by default, MAY support subject continuity without full legal identity, and MUST preserve these distinctions across trust profiles, exports, and disclosures. | MUST / MUST NOT / MAY | Constitutional | Delegated | `trust-profiles.md` §Verification posture declaration; `assurance-traceability.md` | 10.2 | `trust-profiles.md` |
| ULCR-064 | ULCF-015 | Trust honesty | For each deployment mode handling protected content, implementations MUST publish a Trust Profile and MUST state: ordinary-operation readability posture; whether runtime accesses plaintext; whether recovery can occur without the user; whether delegated compute exposes plaintext to ordinary service components. Implementations MUST NOT collapse delegated compute into provider-readable access unless explicitly declared, and MUST NOT overstate trust posture. | MUST / MUST NOT | Constitutional | Delegated | `trust-profiles.md` §Trust honesty rule | 11.1 | `trust-profiles.md` |
| ULCR-065 | ULCF-016 | Trust transitions | On custody, readability, recovery, or delegated-compute change affecting protected content, implementations MUST treat the change as a Trust Profile transition, MUST make it auditable, MUST define whether it applies prospectively, retrospectively, or both, and MUST NOT expand reader-held or delegated-compute access into provider-readable access without such an explicit transition. | MUST / MUST NOT | Constitutional | Delegated | `trust-profiles.md` §Trust profile transitions | 11.2 | `trust-profiles.md` |

### 6.8 Export and Verification Guarantees (ULCR-066 – ULCR-072)

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-066 | ULCF-017 | Export | Conforming implementations claiming the Export Generator role MUST support independently verifiable exports for at least one declared scope of canonical truth. | MUST | Constitutional | Companion | `export-verification-package.md` | 12.1 | `export-verification-package.md` |
| ULCR-067 | ULCF-017 | Export package contents | An export MUST include sufficient material for an offline verifier to validate the declared scope: canonical records or their declared representations, canonical append attestations or equivalent proof material, verification keys or immutable key references, append proofs, schema or semantic digests plus embedded copies or immutable references, protected payload references or included payloads, and canonical facts required for claim verification. | MUST | Constitutional | Companion | `export-verification-package.md` | 12.2 | `export-verification-package.md` |
| ULCR-068 | ULCF-017 | Export reference immutability | Any reference required for offline verification MUST be immutable, content-addressed, or included in the export package. | MUST | Constitutional | Companion | `export-verification-package.md` | 12.2 | `export-verification-package.md` |
| ULCR-069 | ULCF-018 | Verification capabilities | A conforming verifier MUST be able to (1) verify authored signatures or equivalent authored authentication, (2) verify canonical inclusion within the declared append scope, (3) verify append-head consistency when required by the profile, (4) verify schema or semantic digests and any embedded copies or immutable references required for offline verification, and (5) verify any included disclosure or export artifacts. | MUST | Constitutional | Core | `trellis-core.md` §8; detail in `export-verification-package.md` | 12.3 | `trellis-core.md` |
| ULCR-070 | ULCF-019 | Export provenance | Exports MUST preserve the distinction among author-originated facts, canonical records, canonical append attestations, and later-assembled disclosure or export artifacts. | MUST | Constitutional | Companion | `export-verification-package.md` | 12.4 | `export-verification-package.md` |
| ULCR-071 | ULCF-019 | Verification independence | Export verification MUST NOT depend on derived artifacts, workflow runtime state, mutable service databases, or ordinary service APIs beyond what the export explicitly references as optional external proof material. | MUST NOT | Constitutional | Core | `trellis-core.md` §5.2 Invariant 5, §8 | 12.5 | `trellis-core.md` |
| ULCR-072 | ULCF-019 | Disclosure posture surfacing | Where an export omits payload readability, the export MUST still disclose which integrity, provenance, and append claims remain verifiable. | MUST | Constitutional | Companion | `disclosure-manifest.md` | 12.5 | `disclosure-manifest.md` |

### 6.9 Companion Subordination Discipline (ULCR-073 – ULCR-075)

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-073 | ULCF-020 | Companion discipline | Companion specifications MUST remain subordinate to core; MUST NOT alter core canonical truth, admission, order, attestation, trust-honesty, or verification semantics; MUST NOT define an alternate canonical source of truth. | MUST / MUST NOT | Constitutional | Core | `trellis-core.md` §4.2, §9 | 13.1 | `trellis-core.md` |
| ULCR-074 | ULCF-021 | Profile trust inheritance | Profiles and bindings inherit the active Trust Profile; MUST distinguish provider-readable, reader-held, and delegated-compute access when protected content is involved; MUST NOT imply stronger confidentiality than the Trust Profile supports; MUST NOT weaken Trust Profile requirements through profile-local wording. | MUST / MUST NOT | Profile | Delegated | `trust-profiles.md` §Trust honesty rule | 13.2 | `trust-profiles.md` |
| ULCR-075 | ULCF-021 | Profile-scoped export | A profile-scoped export MAY present a profile-specific view; MUST preserve the object-class distinctions required for exports; MUST NOT imply broader coverage than the declared export scope. | MAY / MUST / MUST NOT | Profile | Companion | `export-verification-package.md` | 13.3 | `export-verification-package.md` |

### 6.10 Standard Profiles (Delegated) (ULCR-076 – ULCR-081)

All six legacy standard profiles are normatively owned by companion specifications.

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-076 | ULCF-022 | Offline Authoring Profile | MAY require delayed submission, preservation of authored time or context, local pending state, and authored authentication before admission; MUST preserve canonical admission semantics and provenance distinctions. | MAY / MUST | Profile | Delegated | `trust-profiles.md` §Baseline Profiles | 14.1 | `trust-profiles.md` |
| ULCR-077 | ULCF-023 | Reader-Held Decryption Profile | MAY require that ordinary service operation does not require general plaintext access for declared protected content; MUST identify decrypting principals; MUST remain consistent with the active Trust Profile; MUST preserve the reader-held vs provider-readable distinction. | MAY / MUST | Profile | Delegated | `trust-profiles.md` §Baseline Profiles | 14.2 | `trust-profiles.md` |
| ULCR-078 | ULCF-024 | Delegated Compute Profile | MAY define scoped delegated-compute requirements; MUST NOT imply general provider readability; where workflow materially relies on delegated output, MUST require either a canonical record representing the output or a canonical reference to a stable output artifact. | MAY / MUST / MUST NOT | Profile | Delegated | `trust-profiles.md` §Baseline Profiles | 14.3 | `trust-profiles.md` |
| ULCR-079 | ULCF-025 | Disclosure and Export Profile | MAY define audience-specific scopes, disclosure postures, claim classes, and presentation rules; MUST remain subordinate to core verification semantics. | MAY / MUST | Profile | Companion | `export-verification-package.md`; `disclosure-manifest.md` | 14.4 | `export-verification-package.md` |
| ULCR-080 | ULCF-026 | User-Held Record Reuse Profile | MAY define how user-held records, attestations, or supporting material are referenced or submitted; MUST distinguish reusable prior records from canonical records; MUST bind what was reused when content enters canonical truth; MUST NOT treat the entire user-held layer as canonical workflow state by default. | MAY / MUST / MUST NOT | Profile | Delegated | `trust-profiles.md` §Baseline Profiles | 14.5 | `trust-profiles.md` |
| ULCR-081 | ULCF-027 | Respondent History Profile | MAY define respondent-originated or respondent-visible history; MUST treat timelines as derived artifacts over canonical truth; MUST NOT define a second canonical append model; MUST NOT imply full workflow, governance, custody, or compliance coverage unless in scope. | MAY / MUST / MUST NOT | Profile | Delegated | `trust-profiles.md` §Baseline Profiles | 14.6 | `trust-profiles.md` |

### 6.11 Bindings, Vocabulary Placement, and Sidecars (ULCR-082 – ULCR-085)

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-082 | ULCF-028 | Bindings | Bindings MAY define concrete serializations, APIs, proof encodings, or technology mappings; MUST preserve constitutional semantics and core-to-implementation contracts. | MAY / MUST | Binding | Delegated | `shared-ledger-binding.md` §Normative Focus | 15.1 | `shared-ledger-binding.md` |
| ULCR-083 | ULCF-028 | Vocabulary placement | Domain vocabularies, respondent-history vocabularies, forms vocabularies, workflow-family vocabularies, and similar interpretation layers SHOULD be defined in companion specifications rather than in core. | SHOULD | Binding | Delegated | `shared-ledger-binding.md` §Normative Focus | 15.2 | `shared-ledger-binding.md` |
| ULCR-084 | ULCF-028 | Family bindings | A binding MAY map core onto a specific forms, workflow, or respondent-history family and MAY define stable path semantics, item-key semantics, validation-boundary semantics, amendment or migration semantics, and family-specific change-set structures, remaining binding- or profile-level unless adopted higher. | MAY | Binding | Delegated | `shared-ledger-binding.md` §Family binding matrix | 15.3 | `shared-ledger-binding.md` |
| ULCR-085 | ULCF-028 | Sidecars | A sidecar MAY collect family-specific, deployment-specific, or implementation-adjacent material subordinate to core; MUST NOT alter constitutional semantics. | MAY / MUST NOT | Binding | Delegated | `shared-ledger-binding.md` §Normative Focus | 15.4 | `shared-ledger-binding.md` |

### 6.12 Supplementary Constitutional Requirements (ULCR-086 – ULCR-094)

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-086 | ULCF-029 | Derived artifacts | A derived artifact MUST NOT be authoritative for canonical facts; MUST be rebuildable from canonical truth plus declared configuration history; MUST record enough provenance to identify the canonical state from which it was derived; MUST treat lag, rebuild, or loss as operational rather than a change to canonical truth. | MUST / MUST NOT | Constitutional | Delegated | `projection-runtime-discipline.md` §Projection integrity policy | 16.1 | `projection-runtime-discipline.md` |
| ULCR-087 | ULCF-029 | Rights-impacting evaluators | Where a derived evaluator is used for access, policy, workflow, or other rights-impacting decisions, implementations MUST be able to trace evaluator inputs back to canonical records, MUST define evaluator rebuild behavior, and MUST define behavior when evaluator state is stale, missing, or inconsistent with canonical records. | MUST | Constitutional | Delegated | `projection-runtime-discipline.md` §Rebuild verification | 16.1 | `projection-runtime-discipline.md` |
| ULCR-088 | ULCF-030 | Metadata minimization | Visible metadata SHOULD be limited to canonical verification, schema or semantic lookup, required audit-visible declarations, conflict gating, and append processing; implementations SHOULD NOT keep visible metadata merely to accelerate derived artifacts; MUST NOT retain visible append-related metadata merely for operational convenience where derived or scoped mechanisms suffice. | SHOULD / SHOULD NOT / MUST NOT | Constitutional | Core | `trellis-core.md` §10; operational detail in `trust-profiles.md` §Metadata Budget Requirement | 16.2 | `trellis-core.md` |
| ULCR-089 | ULCF-031 | Idempotency & rejection | Canonical append operations MUST define idempotency semantics for retried or replayed submissions; MUST define a stable idempotency key or equivalent causal submission identity; MUST define whether a retried submission is rejected, treated as a no-op, or resolved to an existing canonical record reference; rejected submissions MUST NOT be treated as canonically appended; for a given idempotency identity within a declared scope, every successful retry MUST resolve to the same canonical record reference or the same declared no-op outcome. | MUST / MUST NOT | Constitutional | Core | `trellis-core.md` §5.2 Invariant 6, §6.3 | 16.3 | `trellis-core.md` |
| ULCR-090 | ULCF-032 | Storage & snapshots | Canonical records MUST be stored durably and immutably from the perspective of ordinary append participants; implementations MUST declare the durable-append boundary; snapshots MAY be used for performance but MUST be treated as derived artifacts; replica completion state MUST remain operational state rather than canonical truth. | MUST / MAY | Constitutional | Delegated | `projection-runtime-discipline.md` §Projection categories; durable-append boundary in `shared-ledger-binding.md` §Canonization rules | 16.4 | `projection-runtime-discipline.md` |
| ULCR-091 | ULCF-033 | Lifecycle facts | Where an implementation supports a listed lifecycle operation (retention, legal hold, archival, key destruction, sealing, export issuance, schema upgrade) as part of canonical or compliance-relevant behavior, it MUST represent that operation as a lifecycle fact; where the fact affects compliance, retention, or recoverability claims, it MUST be a canonical fact. | MUST | Constitutional | Delegated | `key-lifecycle-operating-model.md` | 16.5 | `key-lifecycle-operating-model.md` |
| ULCR-092 | ULCF-033 | Cryptographic inaccessibility | Where cryptographic erasure or key destruction is used, implementations MUST document which content becomes irrecoverable, who retains access, what evidence of destruction is preserved, and what metadata remains; affected derived plaintext state MUST be invalidated, purged, or otherwise made unusable per declared policy. | MUST | Constitutional | Delegated | `key-lifecycle-operating-model.md` | 16.5 | `key-lifecycle-operating-model.md` |
| ULCR-093 | ULCF-034 | Versioning & agility | Implementations MUST version canonical algorithms and schema or semantic references; MUST version author-originated-fact semantics (where profile- or binding-specific), canonical record semantics, append semantics, export-verification semantics, and trust-profile semantics; MUST preserve enough information to verify historical records under the algorithms and rules in effect when produced; MUST NOT silently reinterpret historical records without an explicit migration mechanism; MUST NOT silently invalidate prior export verification via evolution; MUST NOT rely on out-of-band operator knowledge to interpret historical records. | MUST / MUST NOT | Constitutional | Delegated | `shared-ledger-binding.md` §Schema/version compatibility policy | 16.6 | `shared-ledger-binding.md` |
| ULCR-094 | ULCF-035 | Trust/privacy disclosure | Implementations handling protected content MUST disclose what metadata remains visible and which parties can observe it, whether ordinary service operation is provider-readable, and whether delegated compute exposes plaintext to ordinary service components; MUST NOT describe ciphertext storage as equivalent to provider blindness when decryption paths exist. | MUST / MUST NOT | Constitutional | Delegated | `trust-profiles.md` §Operational trust disclosure requirements | 17.3 | `trust-profiles.md` |

### 6.13 Integrator-Critical Guarantees — Cross-Family and Binding (ULCR-095 – ULCR-103)

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-095 | ULCF-036 | Governed scope & order | Exactly one canonical append-attested order per governed scope; implementations MAY partition into multiple ledgers by scope but MUST NOT allow competing canonical orders for the same governed scope. | MUST / MAY / MUST NOT | Constitutional | Core | `trellis-core.md` §5.2 Invariant 3, §6.2 | — | `trellis-core.md` |
| ULCR-096 | ULCF-036 | Canonical event hash | Canonical append semantics MUST use exactly one authoritative canonical event hash construction over the sealed canonical record package; deterministic canonical serialization is REQUIRED; subordinate hashes MAY exist for specialized purposes but MUST NOT redefine canonical append semantics. | MUST / REQUIRED / MAY / MUST NOT | Constitutional | Core | `trellis-core.md` §5.2 Invariant 4, §7 | — | `trellis-core.md` |
| ULCR-097 | ULCF-036 | Verifier obligations | A conforming verifier MUST be able to validate canonical record integrity, append attestation validity, inclusion and consistency claims, and export-package canonical provenance claims without requiring derived runtime state. | MUST | Constitutional | Core | `trellis-core.md` §8 | — | `trellis-core.md` |
| ULCR-098 | ULCF-036 | Cross-repository authority | Trellis Core semantics MUST NOT be interpreted to redefine Formspec or WOS semantic authority. | MUST NOT | Constitutional | Core | `trellis-core.md` §1.2, §9 | — | `trellis-core.md` |
| ULCR-099 | ULCF-036 | Substrate binding | Trellis MUST bind Formspec-family and WOS-family facts (and related trust/release families per binding spec) into one governed canonical substrate with shared append, hash, and verification rules; the binding MUST NOT reinterpret Formspec or WOS meaning. | MUST / MUST NOT | Constitutional | Delegated | `shared-ledger-binding.md` §Substrate binding | — | `shared-ledger-binding.md` |
| ULCR-100 | ULCF-036 | Baseline scope | Baseline Trellis Core conformance MUST NOT be interpreted to require advanced selective disclosure, threshold custody, group-sharing protocols, advanced homomorphic or privacy-preserving computation, or cross-agency analytic protocols unless a declared profile, binding, or implementation specification explicitly requires them. | MUST NOT | Constitutional | Core | `trellis-core.md` §10.1 | — | `trellis-core.md` |
| ULCR-101 | ULCF-036 | Admission prerequisites | A Canonical Append Service MUST NOT issue a canonical append attestation until binding-declared admission prerequisites are satisfied, including resolution of causal or logical dependencies required for that record class. | MUST NOT | Constitutional | Core | `trellis-core.md` §6.1 | — | `trellis-core.md` |
| ULCR-102 | ULCF-036 | Order independence from operations | Canonical order MUST be determined solely by this specification and the applicable binding; MUST NOT depend on wall-clock receipt time, queue depth, worker identity, or other operational accidents. | MUST / MUST NOT | Constitutional | Core | `trellis-core.md` §6.2 | — | `trellis-core.md` |
| ULCR-103 | ULCF-037 | Receipt immutability | Binding-defined ingest-time verification or payload-readiness fields on the canonical append attestation (or equivalent receipt) MUST NOT be rewritten in place after issuance; posture changes MUST be recorded as new canonical facts or attestations per binding. | MUST NOT | Binding | Delegated | `shared-ledger-binding.md` §Canonical receipt immutability | — | `shared-ledger-binding.md` |

### 6.14 Formspec / WOS Integration Boundaries (ULCR-104 – ULCR-109)

These rows capture normative obligations in `trellis-core.md` §1.2 and §6 that govern Trellis's relationship to Formspec and WOS. They are additive invariants: Trellis adds canonical ledger, trust, and disclosure semantics without altering Formspec or WOS processing models.

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-104 | ULCF-038 | Formspec delegation | When Trellis behavior depends on Formspec Definition or Response semantics — including field values, relevance, validation, or calculation — processing MUST be delegated to a Formspec-conformant processor (Formspec Core S1.4). Trellis MUST NOT specify bind, FEL, or validation rules. | MUST / MUST NOT | Constitutional | Core | `trellis-core.md` §1.2 | — | `trellis-core.md` |
| ULCR-105 | ULCF-038 | Additive invariant | Trellis MUST NOT alter Formspec data capture, validation, or Core processing model semantics. A Formspec processor that ignores all Trellis sidecars, bindings, and artifacts MUST remain fully conformant to Formspec and MUST produce identical data and validation results. | MUST / MUST NOT | Constitutional | Core | `trellis-core.md` §1.2 | — | `trellis-core.md` |
| ULCR-106 | ULCF-038 | Formspec conformance tier | Trellis-bound Formspec processors MUST implement at least Formspec Core conformance (Formspec Core S2). Structural and Append Service roles require Core only; roles that present Formspec-backed tasks to end users additionally require Component conformance. | MUST | Constitutional | Core | `trellis-core.md` §1.2 | — | `trellis-core.md` |
| ULCR-107 | ULCF-038 | Screener delegation | When a Trellis-bound deployment uses Formspec Screener evaluation, it MUST delegate to a Formspec-conformant Screener processor and MUST NOT alter the Screener evaluation algorithm (Formspec Screener S1–S7). | MUST / MUST NOT | Constitutional | Core | `trellis-core.md` §1.2 | — | `trellis-core.md` |
| ULCR-108 | ULCF-031 | Rejection explicitness | Rejections of canonical append submissions MUST be explicit and auditable. | MUST | Constitutional | Core | `trellis-core.md` §6.3 | — | `trellis-core.md` |
| ULCR-109 | ULCF-039 | Hash construction registry | Future canonical hash constructions MUST be registered before verifiers are required to accept them. Until a dedicated registry companion is published, the single mandatory construction is JSON Canonicalization Scheme (JCS, RFC 8785) with SHA-256. | MUST | Constitutional | Core | `trellis-core.md` §7 | — | `trellis-core.md` |

### 6.15 Legacy Trust-Posture Invariants (Migrated) (ULCR-110 – ULCR-114)

The legacy `unified_ledger_core.md` §7.1 declared six invariants that mixed ontology distinctions with trust-posture distinctions. The ontology distinctions (Author vs. Attestation; Fact vs. Record; Derived is Non-Canonical) are now expressed in `trellis-core.md` §4 and §5.2 and carried by ULCR-038, ULCR-039, ULCR-042, ULCR-047, ULCR-048. The trust-posture distinctions have migrated out of core and are recorded here for traceability.

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-110 | ULCF-040 | Legacy Invariant — Provider-Readable vs Reader-Held | Provider-readable access and reader-held access MUST remain distinct trust postures. | MUST | Constitutional | Delegated | `trust-profiles.md` §Trust honesty rule, §Operational trust disclosure requirements | 7.1 Invariant 4 | `trust-profiles.md` |
| ULCR-111 | ULCF-040 | Legacy Invariant — Delegated Compute Scope | Delegated compute access MUST NOT be treated as blanket plaintext access for the service operator. | MUST NOT | Constitutional | Delegated | `trust-profiles.md` §Trust honesty rule | 7.1 Invariant 5 | `trust-profiles.md` |
| ULCR-112 | ULCF-040 | Legacy Invariant — Disclosure vs Assurance | Disclosure posture and assurance posture MUST remain distinct and MUST NOT be conflated. | MUST / MUST NOT | Constitutional | Delegated | `trust-profiles.md` §Verification posture declaration; `assurance-traceability.md` | 7.1 Invariant 6 | `assurance-traceability.md` |
| ULCR-113 | ULCF-040 | Legacy Invariant — Author-Originated Fact vs Append Attestation | An author-originated fact and a canonical append attestation MUST remain distinguishable object classes. (Subsumed in current ontology by ULCR-038 and ULCR-047.) | MUST | Constitutional | Core | `trellis-core.md` §4.1 | 7.1 Invariant 1 | `trellis-core.md` |
| ULCR-114 | ULCF-040 | Legacy Invariant — Canonical Fact vs Canonical Record | The canonical record and the underlying authored content it represents MUST remain distinguishable. (Subsumed in current ontology by ULCR-048.) | MUST | Constitutional | Core | `trellis-core.md` §4.1, §3 | 7.1 Invariant 2 | `trellis-core.md` |

### 6.16 Determinism (ULCR-115)

| ULCR ID | ULCF ID | Feature Name | Requirement Summary | Keyword | Class | Status | § Authoritative | § Legacy | Owner |
|---|---|---|---|---|---|---|---|---|---|
| ULCR-115 | ULCF-010 | Deterministic tie-breaking | Bindings SHOULD specify deterministic tie-breaking where concurrent admissible records could otherwise admit more than one total order consistent with declared causal constraints. | SHOULD | Binding | Delegated | `trellis-core.md` §6.2.1; binding detail in `shared-ledger-binding.md` §Canonization rules | — | `shared-ledger-binding.md` |

---

## 7. Coverage Notes

1. **Legacy §2.1–§2.2 (conformance-class and profile enumeration)** are structural scaffolding for the conformance roles in `trellis-core.md` §2.1; no independent MUST rows are generated from them.
2. **Legacy §8.2 (admissibility categories)** is enumerative; the only normative constraint on categories is ULCR-050.
3. **Legacy §8.3 (state-machine table)** is definitional. ULCR-051 captures the durable-append rule (now owned by `shared-ledger-binding.md`) and ULCR-052 captures the scope-of-attestation statement.
4. **Legacy §17.1–§17.2** are non-normative; only §17.3 yields a ULCR row (ULCR-094).
5. **Legacy §18 (Non-Normative Guidance)** is non-normative; no ULCR rows.
6. **Standard profiles (legacy §14)** are normatively owned by `trust-profiles.md` and `export-verification-package.md`. ULCR-076 – ULCR-081 are retained for cross-reference.
7. **Domain bindings and sidecars (legacy §15)** are normatively owned by `shared-ledger-binding.md`. Core retains only the subordination rule.
8. **Lifecycle and cryptographic inaccessibility (legacy §16.5)** are normatively owned by `key-lifecycle-operating-model.md`. Core retains no direct obligation.
9. **Invariant set reconciliation.** Current `trellis-core.md` §5.2 declares six structural invariants (append-only, no-second-truth, one-order-per-scope, one-hash-construction, verification-independence, append-idempotency). These are recorded as ULCR-041 – ULCR-046. The legacy `unified_ledger_core.md` §7.1 declared a different set of six invariants mixing ontology and trust-posture distinctions; the ontology distinctions are carried by ULCR-038, ULCR-039, ULCR-042, ULCR-047, ULCR-048, ULCR-113, ULCR-114, and the trust-posture distinctions are carried by ULCR-110, ULCR-111, ULCR-112.
10. **Verification claim classes.** `trellis-core.md` §8 requires baseline claim classes (canonical-record integrity, append-attestation validity, inclusion consistency). Additional claim classes (payload integrity, authorization history, disclosure policy) are defined by companion specifications and are tracked in the companion matrix.

---

## 8. References

### 8.1 Specifications Cited

- [`trellis-core.md`](trellis-core.md) — Trellis Core Specification (authoritative).
- [`shared-ledger-binding.md`](shared-ledger-binding.md) — Shared Ledger Binding companion.
- [`../trust/trust-profiles.md`](../trust/trust-profiles.md) — Trust Profiles companion.
- [`../trust/key-lifecycle-operating-model.md`](../trust/key-lifecycle-operating-model.md) — Key Lifecycle Operating Model.
- [`../export/export-verification-package.md`](../export/export-verification-package.md) — Export Verification Package.
- [`../export/disclosure-manifest.md`](../export/disclosure-manifest.md) — Disclosure Manifest.
- [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) — Projection / Runtime Discipline.
- [`../operations/monitoring-witnessing.md`](../operations/monitoring-witnessing.md) — Monitoring and Witnessing.
- [`../assurance/assurance-traceability.md`](../assurance/assurance-traceability.md) — Assurance Traceability.
- [`./unified-ledger-companion-requirements-matrix.md`](unified-ledger-companion-requirements-matrix.md) — Companion requirements matrix (ULCOMP-R-*).
- [`../../DRAFTS/unified_ledger_core.md`](../../DRAFTS/unified_ledger_core.md) — historical omnibus draft (non-normative).

### 8.2 Normative References

- [BCP 14] Bradner, S., "Key words for use in RFCs to Indicate Requirement Levels", BCP 14, RFC 2119, March 1997, and Leiba, B., "Ambiguity of Uppercase vs Lowercase in RFC 2119 Key Words", BCP 14, RFC 8174, May 2017.
- [RFC 8259] Bray, T., Ed., "The JavaScript Object Notation (JSON) Data Interchange Format", STD 90, RFC 8259, December 2017.
- [RFC 8785] Rundgren, A., Jordan, B., and S. Erdtman, "JSON Canonicalization Scheme (JCS)", RFC 8785, June 2020.

### 8.3 Companion Matrix Cross-Reference

User-value themes, projection watermarks, stale indication, purge cascades, rebuild and conformance expectations, metadata budget, and tiered verification posture are tracked in the companion matrix under `ULCOMP-R-215` through `ULCOMP-R-223`. See [`unified-ledger-companion-requirements-matrix.md`](unified-ledger-companion-requirements-matrix.md).
