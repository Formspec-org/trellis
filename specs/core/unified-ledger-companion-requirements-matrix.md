---
title: Trellis — Companion Requirements Matrix (Ownership Traceability)
version: 0.2.0
date: 2026-04-14
status: living document
---

# Trellis — Companion Requirements Matrix

This document is a **current-state ownership traceability tool**. It maps every requirement extracted from the legacy companion draft [`../../DRAFTS/unified_ledger_companion.md`](../../DRAFTS/unified_ledger_companion.md) to its **owning Trellis companion spec** (or marks it as legacy-only when no current spec owns it). It is no longer a migration tracker — it is the canonical pointer from a stable requirement ID (`ULCOMP-R-NNN`) to the spec section that today carries (or will carry) the normative obligation.

## Table of Contents

1. [Identifier Conventions](#1-identifier-conventions)
2. [Requirement Classes](#2-requirement-classes)
3. [Owning-Spec Distribution Summary](#3-owning-spec-distribution-summary)
4. [Legend](#4-legend)
5. [Feature Index (ULCOMP-F)](#5-feature-index-ulcomp-f)
6. [Requirements Matrix (ULCOMP-R)](#6-requirements-matrix-ulcomp-r)
7. [User-Value Themes](#7-user-value-themes)
8. [Coverage Notes](#8-coverage-notes)
9. [Cross-References](#9-cross-references)

---

## 1. Identifier Conventions

| Prefix | Meaning |
|--------|---------|
| `ULCOMP-F-###` | Feature / capability area extracted from the legacy companion draft. |
| `ULCOMP-R-###` | Normative requirement (`ULCOMP-R-001` through `ULCOMP-R-223`). |

These IDs are stable. They MUST be preserved when prose moves from this matrix or from `DRAFTS/unified_ledger_companion.md` into a current companion spec; the receiving spec section SHOULD cite the inbound `ULCOMP-R-NNN` so reverse traceability stays intact.

`ULCOMP-*` IDs are distinct from the core matrix IDs (`ULCF-*` / `ULCR-*`) tracked in [`unified-ledger-requirements-matrix.md`](unified-ledger-requirements-matrix.md). Do not collapse the two ID spaces.

**Synthesis rows.** `ULCOMP-R-215`–`220` formalize projection discipline drawn from [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md). `ULCOMP-R-221` formalizes the metadata-budget obligation in [`../trust/trust-profiles.md`](../trust/trust-profiles.md). `ULCOMP-R-222` and `ULCOMP-R-223` formalize verification-posture and projection-integrity policy in those same companions.

---

## 2. Requirement Classes

Each row carries one class, drawn from the legacy draft and preserved here for normative continuity:

| Class | Meaning |
|-------|---------|
| **PC** | **Profile constraint** — applies only within a declared profile (offline authoring, reader-held decryption, delegated compute, disclosure/export, user-held reuse, respondent history). |
| **BSC** | **Binding or sidecar choice** — a binding- or sidecar-author obligation: the choice is optional, but if exercised the constraints apply. |
| **CR** | **Companion requirement** — applies across the companion family regardless of profile or sidecar choice. |

Non-normative appendices appear only where a testable **SHOULD** remains.

---

## 3. Owning-Spec Distribution Summary

| Owning companion spec | Row count |
|---|---:|
| [`shared-ledger-binding.md`](shared-ledger-binding.md) | 25 |
| [`trellis-core.md`](trellis-core.md) | 22 |
| [`../trust/trust-profiles.md`](../trust/trust-profiles.md) | 48 |
| [`../trust/key-lifecycle-operating-model.md`](../trust/key-lifecycle-operating-model.md) | 2 |
| [`../export/export-verification-package.md`](../export/export-verification-package.md) | 22 |
| [`../export/disclosure-manifest.md`](../export/disclosure-manifest.md) | 4 |
| [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) | 18 |
| [`../operations/monitoring-witnessing.md`](../operations/monitoring-witnessing.md) | 5 |
| [`../assurance/assurance-traceability.md`](../assurance/assurance-traceability.md) | 1 |
| [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) (stub) | 20 |
| [`../workflow/workflow-governance-provenance.md`](../workflow/workflow-governance-provenance.md) (stub) | 8 |
| _(legacy only — no current owner)_ | 48 |
| **Total** | **223** |

The "legacy only" rows are concentrated in three profile areas — **offline authoring** (`§2.1`, 18 rows), **user-held record reuse** (`§2.5`, 9 rows), and **identity, attestation, signing, and continuity** (`§3.5`, 7 rows) — plus a handful of cross-cutting topics (delegated-compute output reliance §2.3.2 (5 rows), conflict-handling discretionary rules §3.4 (3 rows), the three non-normative App. F migration `SHOULD`s, sealing/precedence §3.8.2 (2 rows), and sharing-mode discipline §3.1.2 (1 row)) that have not yet been promoted into a dedicated companion. They remain authoritative in [`../../DRAFTS/unified_ledger_companion.md`](../../DRAFTS/unified_ledger_companion.md) until split out.

---

## 4. Legend

| Column | Meaning |
|--------|---------|
| **ULCOMP-R** | Normative requirement ID. |
| **ULCOMP-F** | Feature / capability area. |
| **Feature name** | Short label for the capability. |
| **Requirement summary** | Compressed normative statement; not a substitute for the cited section. |
| **Keyword** | RFC 2119 keyword(s) the row carries. |
| **Class** | One of `PC`, `BSC`, `CR` (see §2). |
| **Legacy §** | Section in `DRAFTS/unified_ledger_companion.md` (or other cited spec) that the row was extracted from. |
| **Owning Spec** | Current companion spec that owns the obligation, with the section reference. `(legacy only — no current owner)` when no current spec carries it. |

---

## 5. Feature Index (ULCOMP-F)

| ID | Name | Legacy § (primary) |
|----|------|--------------------|
| ULCOMP-F-001 | Companion scope, subordination, interpretation | Abstract, Status, 1 |
| ULCOMP-F-002 | Offline Authoring Profile | 2.1 |
| ULCOMP-F-003 | Reader-Held Decryption Profile | 2.2 |
| ULCOMP-F-004 | Delegated Compute Profile | 2.3 |
| ULCOMP-F-005 | Disclosure and Export Profile | 2.4 |
| ULCOMP-F-006 | User-Held Record Reuse Profile | 2.5 |
| ULCOMP-F-007 | Respondent History Profile | 2.6 |
| ULCOMP-F-008 | Trust inheritance and scoped export honesty | 3.0 |
| ULCOMP-F-009 | Access grants, revocations, delegation, evaluators | 3.1 |
| ULCOMP-F-010 | Provider / reader / delegated access and honesty | 3.2 |
| ULCOMP-F-011 | Canonical Append Service, idempotency, proof model, witnessing | 3.3 |
| ULCOMP-F-012 | Conflict handling | 3.4 |
| ULCOMP-F-013 | Identity, attestation, signing, assurance / disclosure | 3.5 |
| ULCOMP-F-014 | Protected payloads and access material | 3.6 |
| ULCOMP-F-015 | Storage, snapshots, durable boundary | 3.7 |
| ULCOMP-F-016 | Lifecycle, erasure, sealing, legal sufficiency | 3.8 |
| ULCOMP-F-017 | Privacy and metadata minimization | 3.9 |
| ULCOMP-F-018 | Sidecar discipline and recognized families | 4 |
| ULCOMP-F-019 | Trust Profile example sidecar | 5 |
| ULCOMP-F-020 | Forms and respondent-history sidecar | 6 |
| ULCOMP-F-021 | Workflow, governance, provenance sidecar | 7 |
| ULCOMP-F-022 | Appendix A — suggested registries | App. A |
| ULCOMP-F-023 | Appendix B — rejection semantics | App. B |
| ULCOMP-F-024 | Appendix C — versioning and algorithm agility | App. C |
| ULCOMP-F-025 | Appendix D — security testing guidance | App. D |
| ULCOMP-F-026 | Appendix F — migration guidance | App. F |
| ULCOMP-F-027 | Appendix G — companion conformance boundary | App. G |
| ULCOMP-F-028 | Projection and staff-view discipline (normalized) | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) |
| ULCOMP-F-029 | Trust profiles and metadata budget (normalized) | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) |

---

## 6. Requirements Matrix (ULCOMP-R)

| ULCOMP-R | ULCOMP-F | Feature name | Requirement summary | Keyword | Class | Legacy § | Owning Spec |
|---|---|---|---|---|---|---|---|
| ULCOMP-R-001 | ULCOMP-F-001 | Companion scope | Companion MAY define profile-specific constraints. | MAY | CR | Abstract | [`trellis-core.md`](trellis-core.md) §11 |
| ULCOMP-R-002 | ULCOMP-F-001 | Companion scope | Companion MAY define binding- or sidecar-oriented interpretation layers. | MAY | CR | Abstract | [`trellis-core.md`](trellis-core.md) §11 |
| ULCOMP-R-003 | ULCOMP-F-001 | Companion scope | Companion MAY define reusable companion requirements that refine but do not reinterpret the core. | MAY | CR | Abstract | [`trellis-core.md`](trellis-core.md) §11 |
| ULCOMP-R-004 | ULCOMP-F-001 | Companion scope | Companion MUST remain subordinate to the constitutional semantics of the Trellis Core specification. | MUST | CR | Abstract | [`trellis-core.md`](trellis-core.md) §2.3 |
| ULCOMP-R-005 | ULCOMP-F-001 | Interpretation | Additional requirements in this companion MUST be interpreted consistently with the core specification. | MUST | CR | Status | [`trellis-core.md`](trellis-core.md) §2.3 |
| ULCOMP-R-006 | ULCOMP-F-001 | Relationship to core | Nothing in this document creates a second canonical order. | MUST NOT (document property) | CR | 1.1 | [`trellis-core.md`](trellis-core.md) §6.2 |
| ULCOMP-R-007 | ULCOMP-F-001 | Relationship to core | Nothing in this document alters the definition of canonical truth. | MUST NOT (document property) | CR | 1.1 | [`trellis-core.md`](trellis-core.md) §6.1 |
| ULCOMP-R-008 | ULCOMP-F-001 | Relationship to core | Nothing in this document collapses derived artifacts into canonical truth. | MUST NOT (document property) | CR | 1.1 | [`trellis-core.md`](trellis-core.md) §6.2 |
| ULCOMP-R-009 | ULCOMP-F-001 | Relationship to core | Nothing in this document weakens trust honesty requirements. | MUST NOT (document property) | CR | 1.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-010 | ULCOMP-F-001 | Relationship to core | Nothing in this document weakens export-verification guarantees. | MUST NOT (document property) | CR | 1.1 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §6 |
| ULCOMP-R-011 | ULCOMP-F-002 | Offline Authoring | MUST permit author-originated facts to exist prior to canonical append. | MUST | PC | 2.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-012 | ULCOMP-F-002 | Offline Authoring | MUST preserve authored authentication semantics across delayed submission. | MUST | PC | 2.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-013 | ULCOMP-F-002 | Offline Authoring | MUST preserve authored time or authored context where available. | MUST | PC | 2.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-014 | ULCOMP-F-002 | Offline Authoring | MUST distinguish authored time from canonical append time unless equivalence is established explicitly. | MUST | PC | 2.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-015 | ULCOMP-F-002 | Offline Authoring | MUST define how local pending facts remain non-canonical until admitted. | MUST | PC | 2.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-016 | ULCOMP-F-002 | Offline Authoring | MUST define duplicate-submission and replay behavior for delayed submissions. | MUST | PC | 2.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-017 | ULCOMP-F-002 | Offline Authoring | MUST preserve provenance distinctions among authored fact, canonical record, and canonical append attestation. | MUST | PC | 2.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-018 | ULCOMP-F-002 | Offline Authoring | SHOULD minimize local pending state to what is necessary for user-authoring continuity. | SHOULD | PC | 2.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-019 | ULCOMP-F-002 | Offline Authoring | SHOULD avoid treating broad local collaboration state as canonical truth. | SHOULD | PC | 2.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-020 | ULCOMP-F-002 | Offline Authoring | SHOULD define how rejected offline submissions are surfaced without implying canonical admission. | SHOULD | PC | 2.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-021 | ULCOMP-F-002 | Offline submission | Offline-originated facts MAY be submitted after delay. | MAY | PC | 2.1.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-022 | ULCOMP-F-002 | Offline submission | If accepted, MUST preserve authored authentication semantics. | MUST | PC | 2.1.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-023 | ULCOMP-F-002 | Offline submission | MUST distinguish later admission and later append attestation from earlier authorship. | MUST | PC | 2.1.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-024 | ULCOMP-F-002 | Offline submission | MUST NOT imply canonical append time is identical to authorship time unless equivalence is established. | MUST NOT | PC | 2.1.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-025 | ULCOMP-F-002 | Pending local state | If local pending state exists before admission, it MUST remain non-canonical. | MUST | PC | 2.1.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-026 | ULCOMP-F-002 | Pending local state | MUST NOT define alternate canonical order. | MUST NOT | PC | 2.1.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-027 | ULCOMP-F-002 | Pending local state | SHOULD remain separable from draft-collaboration state. | SHOULD | PC | 2.1.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-028 | ULCOMP-F-002 | Pending local state | MUST be transformable into submitted facts without silently rewriting prior authored facts. | MUST | PC | 2.1.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-029 | ULCOMP-F-003 | Reader-Held | MUST declare ordinary service operation does not require general plaintext access for declared protected content. | MUST | PC | 2.2 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Baseline Profiles |
| ULCOMP-R-030 | ULCOMP-F-003 | Reader-Held | MUST identify which principals may decrypt within scope. | MUST | PC | 2.2 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Mandatory Profile Declarations |
| ULCOMP-R-031 | ULCOMP-F-003 | Reader-Held | MUST identify whether the provider can assist recovery. | MUST | PC | 2.2 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Mandatory Profile Declarations |
| ULCOMP-R-032 | ULCOMP-F-003 | Reader-Held | MUST remain consistent with the active Trust Profile. | MUST | PC | 2.2 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-033 | ULCOMP-F-003 | Reader-Held | MUST distinguish reader-held access from provider-readable access. | MUST | PC | 2.2 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Baseline Profiles |
| ULCOMP-R-034 | ULCOMP-F-003 | Reader-Held | MUST distinguish reader-held access from delegated compute access. | MUST | PC | 2.2 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Baseline Profiles |
| ULCOMP-R-035 | ULCOMP-F-003 | Reader-held semantics | Reader-held access MUST NOT be described as provider-readable ordinary operation. | MUST NOT | CR | 2.2.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-036 | ULCOMP-F-003 | Reader-held semantics | MAY coexist with recovery assistance if Trust Profile declares it honestly. | MAY | CR | 2.2.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-037 | ULCOMP-F-003 | Reader-held semantics | MAY coexist with delegated compute if delegation remains explicit, scoped, and auditable. | MAY | CR | 2.2.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Profile declaration schema |
| ULCOMP-R-038 | ULCOMP-F-004 | Delegated Compute | MUST distinguish delegated compute access from provider-readable access. | MUST | PC | 2.3 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-039 | ULCOMP-F-004 | Delegated Compute | MUST make delegated compute explicit, attributable, and auditable. | MUST | PC | 2.3 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Mandatory Profile Declarations |
| ULCOMP-R-040 | ULCOMP-F-004 | Delegated Compute | MUST define delegation scope. | MUST | PC | 2.3 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Profile declaration schema |
| ULCOMP-R-041 | ULCOMP-F-004 | Delegated Compute | MUST define delegation authority. | MUST | PC | 2.3 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Profile declaration schema |
| ULCOMP-R-042 | ULCOMP-F-004 | Delegated Compute | SHOULD define purpose bounds or time bounds. | SHOULD | PC | 2.3 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Mandatory Profile Declarations |
| ULCOMP-R-043 | ULCOMP-F-004 | Delegated Compute | MUST NOT imply delegated compute grants general service readability. | MUST NOT | PC | 2.3 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-044 | ULCOMP-F-004 | Delegated Compute | MUST define what audit facts or audit events are emitted for delegation and use. | MUST | PC | 2.3 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Family binding matrix |
| ULCOMP-R-045 | ULCOMP-F-004 | Delegated compute grant | Delegated compute grant MUST be explicit. | MUST | CR | 2.3.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Mandatory Profile Declarations |
| ULCOMP-R-046 | ULCOMP-F-004 | Delegated compute grant | MUST be attributable to a principal, policy authority, or comparable authority. | MUST | CR | 2.3.1 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Family binding matrix |
| ULCOMP-R-047 | ULCOMP-F-004 | Delegated compute grant | MUST be scoped to declared content or classes of content. | MUST | CR | 2.3.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Profile declaration schema |
| ULCOMP-R-048 | ULCOMP-F-004 | Delegated compute grant | SHOULD be time-bounded or purpose-bounded. | SHOULD | CR | 2.3.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Mandatory Profile Declarations |
| ULCOMP-R-049 | ULCOMP-F-004 | Delegated compute grant | MUST be auditable. | MUST | CR | 2.3.1 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Family binding matrix |
| ULCOMP-R-050 | ULCOMP-F-004 | Delegated compute grant | MUST NOT be interpreted as conferring standing plaintext access to the ordinary service runtime. | MUST NOT | CR | 2.3.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-051 | ULCOMP-F-004 | Compute output reliance | If system relies materially on delegated compute output (workflow, policy, adjudication, access, consequential actions), MUST record output as canonical fact or canonical reference to stable artifact. | MUST | CR | 2.3.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-052 | ULCOMP-F-004 | Compute output reliance | MUST preserve auditable link to authorizing principal. | MUST | CR | 2.3.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-053 | ULCOMP-F-004 | Compute output reliance | MUST preserve auditable link to compute agent identity. | MUST | CR | 2.3.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-054 | ULCOMP-F-004 | Compute output reliance | MUST preserve auditable link to scope of delegated access relevant to that output. | MUST | CR | 2.3.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-055 | ULCOMP-F-004 | Compute output reliance | MUST define whether relied-upon output is advisory, recommendatory, or decision-contributory. | MUST | CR | 2.3.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-056 | ULCOMP-F-005 | Disclosure & Export | MUST support at least one verifiable disclosure or export form. | MUST | PC | 2.4 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §3 |
| ULCOMP-R-057 | ULCOMP-F-005 | Disclosure & Export | MUST preserve distinction among author facts, canonical records, attestations, later disclosure/export artifacts. | MUST | PC | 2.4 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §7 |
| ULCOMP-R-058 | ULCOMP-F-005 | Disclosure & Export | MUST define which claims remain verifiable when payload readability is absent. | MUST | PC | 2.4 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §6.1 |
| ULCOMP-R-059 | ULCOMP-F-005 | Disclosure & Export | MUST define profile-specific audience scope where relevant. | MUST | PC | 2.4 | [`../export/disclosure-manifest.md`](../export/disclosure-manifest.md) §Normative Focus |
| ULCOMP-R-060 | ULCOMP-F-005 | Disclosure & Export | MUST remain subordinate to export guarantees of the core specification. | MUST | PC | 2.4 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §2.3 |
| ULCOMP-R-061 | ULCOMP-F-005 | Export claim classes | Disclosure and Export Profile SHOULD state which listed claim classes are verifiable within that profile. | SHOULD | CR | 2.4.1 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §8 |
| ULCOMP-R-062 | ULCOMP-F-005 | Export claim classes | Implementation MUST NOT imply export supports a claim class unless export contains sufficient material to verify that class. | MUST NOT | CR | 2.4.1 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §8 |
| ULCOMP-R-063 | ULCOMP-F-005 | Selective disclosure | Selective disclosure SHOULD occur through disclosure or export artifacts rather than overloading canonical records. | SHOULD | CR | 2.4.2 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §9 |
| ULCOMP-R-064 | ULCOMP-F-005 | Disclosure-oriented artifact | MAY present audience-specific subset or presentation. | MAY | CR | 2.4.2 | [`../export/disclosure-manifest.md`](../export/disclosure-manifest.md) §Claim-class taxonomy |
| ULCOMP-R-065 | ULCOMP-F-005 | Disclosure-oriented artifact | MUST preserve provenance distinctions. | MUST | CR | 2.4.2 | [`../export/disclosure-manifest.md`](../export/disclosure-manifest.md) §Claim-class taxonomy |
| ULCOMP-R-066 | ULCOMP-F-005 | Disclosure-oriented artifact | MUST NOT be treated as a rewrite of canonical truth. | MUST NOT | CR | 2.4.2 | [`../export/disclosure-manifest.md`](../export/disclosure-manifest.md) §Claim-class taxonomy |
| ULCOMP-R-067 | ULCOMP-F-006 | User-Held Reuse | MUST support submission or reference of previously user-held records, supporting material, or attestations. | MUST | PC | 2.5 | _(legacy only — no current owner)_ |
| ULCOMP-R-068 | ULCOMP-F-006 | User-Held Reuse | MUST bind exactly what was reused or disclosed when material enters canonical workflows. | MUST | PC | 2.5 | _(legacy only — no current owner)_ |
| ULCOMP-R-069 | ULCOMP-F-006 | User-Held Reuse | MUST distinguish reusable prior records from canonical workflow state. | MUST | PC | 2.5 | _(legacy only — no current owner)_ |
| ULCOMP-R-070 | ULCOMP-F-006 | User-Held Reuse | MUST distinguish workflow submission from prior-record possession. | MUST | PC | 2.5 | _(legacy only — no current owner)_ |
| ULCOMP-R-071 | ULCOMP-F-006 | User-Held Reuse | MUST avoid treating entire user-held record layer as canonical workflow state by default. | MUST | PC | 2.5 | _(legacy only — no current owner)_ |
| ULCOMP-R-072 | ULCOMP-F-006 | Selective submission | If user-held reusable prior records supported, selective submission SHOULD be favored over bulk transfer of unrelated content. | SHOULD | CR | 2.5.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-073 | ULCOMP-F-006 | Reuse provenance | When reused material enters canonical truth, MUST bind what was introduced. | MUST | CR | 2.5.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-074 | ULCOMP-F-006 | Reuse provenance | SHOULD bind reuse context where relevant. | SHOULD | CR | 2.5.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-075 | ULCOMP-F-006 | Reuse provenance | MUST preserve provenance distinctions among pre-existing user-held material, canonical submission, later disclosure artifacts. | MUST | CR | 2.5.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-076 | ULCOMP-F-007 | Respondent History | MUST scope to respondent-originated or respondent-visible material history. | MUST | PC | 2.6 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Inbound Requirements |
| ULCOMP-R-077 | ULCOMP-F-007 | Respondent History | MAY support listed respondent-history moments (draft, save, submit, etc.). | MAY | PC | 2.6 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Inbound Requirements |
| ULCOMP-R-078 | ULCOMP-F-007 | Respondent History | MUST treat respondent-history views as projections/profile interpretations over canonical truth, not separate source of truth. | MUST | PC | 2.6 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Scope and Subordination |
| ULCOMP-R-079 | ULCOMP-F-007 | Respondent History | MUST NOT define a second canonical append model. | MUST NOT | PC | 2.6 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Scope and Subordination |
| ULCOMP-R-080 | ULCOMP-F-007 | Respondent History | MUST NOT imply complete workflow/governance/custody/compliance coverage unless declared scope includes those materials. | MUST NOT | PC | 2.6 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Scope and Subordination |
| ULCOMP-R-081 | ULCOMP-F-007 | Materiality | MUST prioritize material respondent-side state changes over raw UI telemetry. | MUST | PC | 2.6.1 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Inbound Requirements |
| ULCOMP-R-082 | ULCOMP-F-007 | Materiality | MUST NOT require keystroke, focus, blur, rendering, or equivalent ephemeral interface event capture. | MUST NOT | PC | 2.6.1 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Inbound Requirements |
| ULCOMP-R-083 | ULCOMP-F-007 | Materiality | SHOULD expose validation, submission, amendment, materially relevant identity/attestation boundaries where they matter to human review. | SHOULD | PC | 2.6.1 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Inbound Requirements |
| ULCOMP-R-084 | ULCOMP-F-007 | Materiality | MAY define profile-specific change-set semantics aligned to stable form-path and item-key semantics where those exist. | MAY | PC | 2.6.1 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Inbound Requirements |
| ULCOMP-R-085 | ULCOMP-F-007 | Coverage honesty | Respondent-history export or view MAY present profile-specific timeline or delta history. | MAY | PC | 2.6.2 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Inbound Requirements |
| ULCOMP-R-086 | ULCOMP-F-007 | Coverage honesty | MUST preserve provenance distinctions. | MUST | PC | 2.6.2 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Scope and Subordination |
| ULCOMP-R-087 | ULCOMP-F-007 | Coverage honesty | MUST NOT imply broader workflow/governance/custody/compliance coverage than declared profile scope provides. | MUST NOT | PC | 2.6.2 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Scope and Subordination |
| ULCOMP-R-088 | ULCOMP-F-008 | Trust inheritance | Profiles/bindings/sidecars MUST remain consistent with active Trust Profile. | MUST | CR | 3.0.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-089 | ULCOMP-F-008 | Trust inheritance | MUST distinguish provider-readable, reader-held, delegated compute when protected content involved. | MUST | CR | 3.0.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Baseline Profiles |
| ULCOMP-R-090 | ULCOMP-F-008 | Trust inheritance | MUST NOT imply stronger confidentiality, weaker provider visibility, or weaker recovery than active Trust Profile supports. | MUST NOT | CR | 3.0.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-091 | ULCOMP-F-008 | Trust inheritance | MUST NOT use profile/binding/sidecar-local wording to weaken or bypass Trust Profile requirements. | MUST NOT | CR | 3.0.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-092 | ULCOMP-F-008 | Scoped export honesty | Profile/sidecar export or view MUST preserve author/canonical record/attestation/disclosure distinctions. | MUST | CR | 3.0.2 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §7 |
| ULCOMP-R-093 | ULCOMP-F-008 | Scoped export honesty | MUST preserve provenance distinctions when presenting profile-specific timeline, delta, or interpretation. | MUST | CR | 3.0.2 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §11 |
| ULCOMP-R-094 | ULCOMP-F-008 | Scoped export honesty | MUST NOT imply broader workflow/governance/custody/compliance/disclosure coverage than declared scope includes. | MUST NOT | CR | 3.0.2 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §11 |
| ULCOMP-R-095 | ULCOMP-F-009 | Grants & revocations | Access grants/revocations affecting canonical authorization semantics MUST be recorded as append-only canonical facts. | MUST | CR | 3.1 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Family binding matrix |
| ULCOMP-R-096 | ULCOMP-F-009 | Derived evaluators | Authorization evaluators MAY be derived artifacts. | MAY | CR | 3.1 | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Projection categories |
| ULCOMP-R-097 | ULCOMP-F-009 | Derived evaluators | If derived, MUST be rebuildable from canonical grant and revocation facts. | MUST | CR | 3.1 | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Rebuild verification |
| ULCOMP-R-098 | ULCOMP-F-009 | Derived evaluators | If derived, MUST NOT be authoritative for grant existence, grant history, or revocation history. | MUST NOT | CR | 3.1 | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Normative Focus |
| ULCOMP-R-099 | ULCOMP-F-009 | Derived evaluators | If derived, MUST preserve canonical grant/revocation semantics when evaluator absent, stale, or rebuilding. | MUST | CR | 3.1 | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Projection watermark contract |
| ULCOMP-R-100 | ULCOMP-F-009 | Delegation facts | If delegation affects authorization, legal authority, or access posture, delegation grants/revocations MUST be canonical facts. | MUST | CR | 3.1.1 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Family binding matrix |
| ULCOMP-R-101 | ULCOMP-F-009 | Sharing-mode discipline | If both narrow sharing and long-lived collaborative membership supported, SHOULD avoid forcing both into one mechanism if that increases KM/audit complexity. | SHOULD | CR | 3.1.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-102 | ULCOMP-F-009 | Evaluator rebuild | If derived evaluator used for rights-impacting decisions, MUST trace inputs to canonical facts. | MUST | CR | 3.1.3 | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Projection integrity policy |
| ULCOMP-R-103 | ULCOMP-F-009 | Evaluator rebuild | MUST define evaluator rebuild behavior. | MUST | CR | 3.1.3 | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Rebuild verification |
| ULCOMP-R-104 | ULCOMP-F-009 | Evaluator rebuild | MUST define behavior when evaluator stale, missing, inconsistent with canonical facts, or unavailable during rebuild. | MUST | CR | 3.1.3 | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Projection watermark contract |
| ULCOMP-R-105 | ULCOMP-F-009 | Evaluator rebuild | MUST preserve rule that evaluator state does not override canonical grant/revocation semantics. | MUST | CR | 3.1.3 | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Normative Focus |
| ULCOMP-R-106 | ULCOMP-F-010 | Access categories | Implementations handling protected content MUST distinguish provider-readable, reader-held, and delegated compute access. | MUST | CR | 3.2 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Baseline Profiles |
| ULCOMP-R-107 | ULCOMP-F-010 | Access categories | Conforming implementation MUST describe these categories consistently with actual behavior. | MUST | CR | 3.2 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-108 | ULCOMP-F-010 | Profile honesty detail | MUST disclose whether provider-readable access exists in ordinary operation. | MUST | CR | 3.2.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-109 | ULCOMP-F-010 | Profile honesty detail | MUST disclose whether delegated compute is provider-operated, tenant-operated, client-side, or otherwise isolated. | MUST | CR | 3.2.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Mandatory Profile Declarations |
| ULCOMP-R-110 | ULCOMP-F-010 | Profile honesty detail | MUST disclose what metadata remains visible to service or other observers. | MUST | CR | 3.2.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Metadata Budget Requirement |
| ULCOMP-R-111 | ULCOMP-F-010 | Profile honesty detail | MUST NOT describe trust posture more strongly than those facts support. | MUST NOT | CR | 3.2.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-112 | ULCOMP-F-011 | CAS obligations | CAS MUST validate append admissibility, preserve append-only, issue attestations, retain/reference sufficient proof for verification. | MUST | CR | 3.3 | [`trellis-core.md`](trellis-core.md) §2.5.2 |
| ULCOMP-R-113 | ULCOMP-F-011 | CAS non-obligations | By canonical role alone, CAS MUST NOT be required to decrypt payloads, evaluate workflow policy, resolve workflow runtime, compute projections/indexes, or inspect protected content unless Trust Profile permits/requires. | MUST NOT | CR | 3.3 | [`trellis-core.md`](trellis-core.md) §2.5.2 |
| ULCOMP-R-114 | ULCOMP-F-011 | Append idempotency | Canonical append MUST define idempotency for retried/replayed/duplicate submissions. | MUST | CR | 3.3.1 | [`trellis-core.md`](trellis-core.md) §7.6 |
| ULCOMP-R-115 | ULCOMP-F-011 | Append idempotency | MUST define stable idempotency key or equivalent causal submission identity. | MUST | CR | 3.3.1 | [`trellis-core.md`](trellis-core.md) §7.6 |
| ULCOMP-R-116 | ULCOMP-F-011 | Append idempotency | MUST define whether submission rejected, no-op, resolved to existing record ref, or other declared rule consistent with append semantics. | MUST | CR | 3.3.1 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Canonization rejection codes |
| ULCOMP-R-117 | ULCOMP-F-011 | Append idempotency | Duplicate/retried handling MUST NOT create ambiguity about newly appended vs previously appended vs not admitted. | MUST NOT | CR | 3.3.1 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Canonization rejection codes |
| ULCOMP-R-118 | ULCOMP-F-011 | Append idempotency | For given idempotency identity in append scope, every successful retry MUST resolve to same record ref or same declared no-op. | MUST | CR | 3.3.1 | [`trellis-core.md`](trellis-core.md) §7.6 |
| ULCOMP-R-119 | ULCOMP-F-011 | Append idempotency | If idempotent acceptance supported, MUST define verifier-visible consequences. | MUST | CR | 3.3.1 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Canonization rejection codes |
| ULCOMP-R-120 | ULCOMP-F-011 | Proof model | MUST present one verifier-facing canonical append proof model per declared append scope at a time. | MUST | CR | 3.3.2 | [`trellis-core.md`](trellis-core.md) §7.5 |
| ULCOMP-R-121 | ULCOMP-F-011 | Proof model | MUST NOT require verifiers to reconcile multiple overlapping append-attestation semantics for same canonical scope. | MUST NOT | CR | 3.3.2 | [`trellis-core.md`](trellis-core.md) §7.5 |
| ULCOMP-R-122 | ULCOMP-F-011 | Proof model | If proof model changes, MUST define explicit migration boundary so verifiers never reconcile overlapping semantics for same scope. | MUST | CR | 3.3.2 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Schema/version compatibility policy |
| ULCOMP-R-123 | ULCOMP-F-011 | Proof model | SHOULD use transparency-log-style append with order, inclusion proofs, consistency proofs between append heads. | SHOULD | CR | 3.3.2 | [`../operations/monitoring-witnessing.md`](../operations/monitoring-witnessing.md) §Checkpoint publication interface |
| ULCOMP-R-124 | ULCOMP-F-011 | External witnessing | MAY support external witnessing or anchoring. | MAY | BSC | 3.3.3 | [`../operations/monitoring-witnessing.md`](../operations/monitoring-witnessing.md) §Anti-equivocation and core append model |
| ULCOMP-R-125 | ULCOMP-F-011 | External witnessing | External witnessing MUST remain subordinate to core canonical append semantics. | MUST | BSC | 3.3.3 | [`../operations/monitoring-witnessing.md`](../operations/monitoring-witnessing.md) §Anti-equivocation and core append model |
| ULCOMP-R-126 | ULCOMP-F-011 | External witnessing | MUST NOT be required for correctness unless profile/binding explicitly states otherwise. | MUST NOT | BSC | 3.3.3 | [`../operations/monitoring-witnessing.md`](../operations/monitoring-witnessing.md) §Current Scope Constraint |
| ULCOMP-R-127 | ULCOMP-F-011 | External witnessing | MAY strengthen equivocation detection or independent audit posture. | MAY | BSC | 3.3.3 | [`../operations/monitoring-witnessing.md`](../operations/monitoring-witnessing.md) §Anti-equivocation and core append model |
| ULCOMP-R-128 | ULCOMP-F-012 | Conflict handling | MAY define conflict-sensitive fact categories. | MAY | CR | 3.4 | _(legacy only — no current owner)_ |
| ULCOMP-R-129 | ULCOMP-F-012 | Conflict handling | Conflict handling MUST be evaluated within declared append scope of affected canonical facts. | MUST | CR | 3.4 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Canonization rules |
| ULCOMP-R-130 | ULCOMP-F-012 | Conflict handling | If resolution needed: append in unaffected scopes MUST continue. | MUST | CR | 3.4 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Canonization rules |
| ULCOMP-R-131 | ULCOMP-F-012 | Conflict handling | Affected derived systems/policies/workflows MAY gate on explicit resolution facts. | MAY | CR | 3.4 | _(legacy only — no current owner)_ |
| ULCOMP-R-132 | ULCOMP-F-012 | Conflict handling | Derived artifacts MUST NOT silently rewrite canonical facts to resolve conflicts. | MUST NOT | CR | 3.4 | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Normative Focus |
| ULCOMP-R-133 | ULCOMP-F-012 | Conflict handling | Conflict resolution SHOULD be via later canonical facts, explicit rejection, or profile-defined admission rules. | SHOULD | CR | 3.4 | _(legacy only — no current owner)_ |
| ULCOMP-R-134 | ULCOMP-F-012 | Conflict handling | MUST NOT stall unrelated append scopes solely because conflict unresolved in another scope. | MUST NOT | CR | 3.4 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Canonization rules |
| ULCOMP-R-135 | ULCOMP-F-013 | Identity & attestation | If identity/attestation facts represented canonically or in bindings, SHOULD represent provider-neutrally where feasible. | SHOULD | CR | 3.5 | _(legacy only — no current owner)_ |
| ULCOMP-R-136 | ULCOMP-F-013 | Identity & attestation | Provider-specific issuers/adapters MAY be used operationally; SHOULD NOT define long-lived semantic meaning of attestation in constitutional model. | MAY / SHOULD NOT | CR | 3.5 | _(legacy only — no current owner)_ |
| ULCOMP-R-137 | ULCOMP-F-013 | User signing | SHOULD support user-originated signatures. | SHOULD | CR | 3.5.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-138 | ULCOMP-F-013 | User signing | MAY support offline user-originated signing. | MAY | CR | 3.5.1 | _(legacy only — no current owner)_ |
| ULCOMP-R-139 | ULCOMP-F-013 | User signing | If supported, evidence package MUST distinguish user signature/auth from later canonical append attestation. | MUST | CR | 3.5.1 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §7 |
| ULCOMP-R-140 | ULCOMP-F-013 | Assurance & disclosure | MUST distinguish assurance from disclosure posture. | MUST | CR | 3.5.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-141 | ULCOMP-F-013 | Assurance & disclosure | MUST NOT treat higher assurance as requiring greater identity disclosure by default. | MUST NOT | CR | 3.5.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-142 | ULCOMP-F-013 | Assurance & disclosure | MAY support subject continuity without full legal identity disclosure. | MAY | CR | 3.5.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-143 | ULCOMP-F-013 | Assurance & disclosure | MUST preserve distinctions across trust profiles, exports, disclosures, sidecars. | MUST | CR | 3.5.2 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §7 |
| ULCOMP-R-144 | ULCOMP-F-014 | Protected payloads | Sensitive content SHOULD reside in protected payloads when protection required by Trust Profile or binding. | SHOULD | CR | 3.6 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Baseline Profiles |
| ULCOMP-R-145 | ULCOMP-F-014 | Protected payloads | MUST define which data visible for canonical verification vs payload-protected. | MUST | CR | 3.6 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Metadata Budget Requirement |
| ULCOMP-R-146 | ULCOMP-F-014 | Protected payloads | Canonical records with protected payloads MUST include or reference sufficient access material for authorized recipients per custody/binding. | MUST | CR | 3.6 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §4 |
| ULCOMP-R-147 | ULCOMP-F-014 | Protected payloads | Conforming representation MUST preserve semantic distinction among author fact, payload content, access/key material, append attestation material. | MUST | CR | 3.6 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §7 |
| ULCOMP-R-148 | ULCOMP-F-015 | Storage | Canonical records MUST be stored durably and immutably from perspective of ordinary append participants. | MUST | CR | 3.7 | [`trellis-core.md`](trellis-core.md) §6.2 |
| ULCOMP-R-149 | ULCOMP-F-015 | Storage | Protected payloads MAY be stored in one or more blob stores. | MAY | CR | 3.7 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Family binding matrix |
| ULCOMP-R-150 | ULCOMP-F-015 | Storage | Canonical acceptance MUST define which durable write conditions are required. | MUST | CR | 3.7 | [`trellis-core.md`](trellis-core.md) §7.5 |
| ULCOMP-R-151 | ULCOMP-F-015 | Storage | MUST declare durable-append boundary governing attestation, retry, export issuance. | MUST | CR | 3.7 | [`trellis-core.md`](trellis-core.md) §7.5 |
| ULCOMP-R-152 | ULCOMP-F-015 | Storage | Proof/referenced state needed to recover/verify within export scope MUST be durably recoverable no later than that boundary. | MUST | CR | 3.7 | [`../export/export-verification-package.md`](../export/export-verification-package.md) §4 |
| ULCOMP-R-153 | ULCOMP-F-015 | Storage | Replica completion state MUST remain operational, not canonical truth. | MUST | CR | 3.7 | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Normative Focus |
| ULCOMP-R-154 | ULCOMP-F-015 | Snapshots | Snapshots MAY be used for performance; MUST be derived artifacts; MUST NOT become canonical truth. | MAY / MUST / MUST NOT | CR | 3.7 | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Normative Focus |
| ULCOMP-R-155 | ULCOMP-F-016 | Lifecycle | MAY define lifecycle facts for listed operations (retention, hold, archival, etc.). | MAY | CR | 3.8 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Family binding matrix |
| ULCOMP-R-156 | ULCOMP-F-016 | Lifecycle | MAY support subset or none of lifecycle operations. | MAY | CR | 3.8 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Family binding matrix |
| ULCOMP-R-157 | ULCOMP-F-016 | Lifecycle | If operation is part of canonical/compliance-relevant behavior, MUST represent as lifecycle fact. | MUST | CR | 3.8 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Family binding matrix |
| ULCOMP-R-158 | ULCOMP-F-016 | Lifecycle | If fact affects compliance/retention/recoverability claims, MUST be canonical fact. | MUST | CR | 3.8 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Family binding matrix |
| ULCOMP-R-159 | ULCOMP-F-016 | Erasure | If cryptographic erasure or key destruction used, MUST document irrecoverable content, who retains access, destruction evidence, remaining metadata. | MUST | CR | 3.8.1 | [`../trust/key-lifecycle-operating-model.md`](../trust/key-lifecycle-operating-model.md) §Recovery and destruction evidence requirements |
| ULCOMP-R-160 | ULCOMP-F-016 | Erasure | If protected content destroyed/inaccessible per lifecycle rules, affected derived plaintext MUST be invalidated/purged/unusable per declared policy. | MUST | CR | 3.8.1 | [`../trust/key-lifecycle-operating-model.md`](../trust/key-lifecycle-operating-model.md) §Required Completeness Rule |
| ULCOMP-R-161 | ULCOMP-F-016 | Sealing | MUST define whether sealed cases/records/scopes permit later lifecycle or governance facts. | MUST | CR | 3.8.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-162 | ULCOMP-F-016 | Sealing | MUST define whether retention or hold rules take precedence when both apply. | MUST | CR | 3.8.2 | _(legacy only — no current owner)_ |
| ULCOMP-R-163 | ULCOMP-F-016 | Legal sufficiency | MUST NOT imply crypto alone guarantees admissibility or legal sufficiency in all jurisdictions. | MUST NOT | CR | 3.8.3 | [`trellis-core.md`](trellis-core.md) §13.4 |
| ULCOMP-R-164 | ULCOMP-F-016 | Legal sufficiency | MAY claim stronger evidentiary posture only to extent supported by process, signatures, attestations, records practice, law. | MAY | CR | 3.8.3 | [`trellis-core.md`](trellis-core.md) §13.4 |
| ULCOMP-R-165 | ULCOMP-F-017 | Privacy | MUST document what is protected from whom. | MUST | CR | 3.9 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Mandatory Profile Declarations |
| ULCOMP-R-166 | ULCOMP-F-017 | Privacy | Payload confidentiality MUST NOT be described as equivalent to metadata privacy. | MUST NOT | CR | 3.9 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Metadata Budget Requirement |
| ULCOMP-R-167 | ULCOMP-F-017 | Privacy | If provider-readable in ordinary operation, MUST say so plainly. | MUST | CR | 3.9 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-168 | ULCOMP-F-017 | Privacy | If delegated compute without general provider readability, MUST distinguish from provider-readable custody. | MUST | CR | 3.9 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-169 | ULCOMP-F-017 | Metadata minimization | Visible metadata SHOULD be limited to purposes listed (verification, schema lookup, audit, conflict gating, append). | SHOULD | CR | 3.9.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Metadata Budget Requirement |
| ULCOMP-R-170 | ULCOMP-F-017 | Metadata minimization | SHOULD NOT keep visible metadata merely to accelerate derived artifacts. | SHOULD NOT | CR | 3.9.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Metadata Budget Requirement |
| ULCOMP-R-171 | ULCOMP-F-017 | Metadata minimization | MUST NOT retain visible append-related metadata merely for operational convenience when derived/scoped mechanisms suffice. | MUST NOT | CR | 3.9.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Metadata Budget Requirement |
| ULCOMP-R-172 | ULCOMP-F-017 | Metadata minimization | SHOULD reduce offline coordination scope and visible coordination metadata where it does not weaken canonical verifiability. | SHOULD | CR | 3.9.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Metadata Budget Requirement |
| ULCOMP-R-173 | ULCOMP-F-018 | Sidecar discipline | Sidecar MAY collect subordinate family/deployment material. | MAY | BSC | 4.1 | [`trellis-core.md`](trellis-core.md) §11 |
| ULCOMP-R-174 | ULCOMP-F-018 | Sidecar discipline | Sidecar MUST NOT redefine canonical truth or order, collapse provenance, weaken trust honesty, weaken export verification. | MUST NOT | BSC | 4.1 | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Substrate binding |
| ULCOMP-R-175 | ULCOMP-F-019 | Example sidecar purpose | Illustrative examples MUST NOT override actual Trust Profile declarations. | MUST NOT | BSC | 5.1 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-176 | ULCOMP-F-019 | Example Profile A | Profile using provider-readable posture MUST say so plainly and MUST NOT imply provider blindness. | MUST / MUST NOT | BSC | 5.2 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Baseline Profiles |
| ULCOMP-R-177 | ULCOMP-F-019 | Example Profile B | Trust Profile MUST describe who can assist recovery and under what conditions. | MUST | BSC | 5.3 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Mandatory Profile Declarations |
| ULCOMP-R-178 | ULCOMP-F-019 | Example Profile C | Trust Profile MUST state whether plaintext visible to any provider-operated components during delegation. | MUST | BSC | 5.4 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Trust honesty rule |
| ULCOMP-R-179 | ULCOMP-F-019 | Example Profile D | Recovery conditions, quorum thresholds, exceptional access MUST be declared; threshold participation MUST NOT be overstated. | MUST / MUST NOT | BSC | 5.5 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Mandatory Profile Declarations |
| ULCOMP-R-180 | ULCOMP-F-019 | Example Profile E | Trust Profile MUST identify scope of organizational authority and exceptional-access controls; MUST distinguish provider-readable from organization-controlled where they differ. | MUST | BSC | 5.6 | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Mandatory Profile Declarations |
| ULCOMP-R-181 | ULCOMP-F-020 | Stable paths | If stable path semantics defined: SHOULD remain stable across non-material presentation changes; SHOULD use semantic positions; MUST distinguish structural path from display-order accidents; SHOULD define deprecation/migration; SHOULD support history without UI telemetry. | SHOULD / MUST | BSC | 6.2 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Inbound Requirements |
| ULCOMP-R-182 | ULCOMP-F-020 | Item keys | If item-key semantics defined: SHOULD stable logical item across cycles; SHOULD distinguish changed vs new item when reliable; SHOULD define key lifecycle; MUST NOT rely on ephemeral rendering state as sole identity. | SHOULD / MUST NOT | BSC | 6.3 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Inbound Requirements |
| ULCOMP-R-183 | ULCOMP-F-020 | Validation snapshots | If defined, SHOULD capture material outcomes, bind paths/keys, distinguish blocking/advisory/informational, record boundary/scope, permit later verification at material boundary. | SHOULD | BSC | 6.4 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Inbound Requirements |
| ULCOMP-R-184 | ULCOMP-F-020 | Amendment cycles | If defined, SHOULD distinguish initiation/progress/submit/complete; define baseline; preserve linkage; distinguish amendment kinds; define abandoned visibility. | SHOULD | BSC | 6.5 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Inbound Requirements |
| ULCOMP-R-185 | ULCOMP-F-020 | Migration outcomes | If defined, SHOULD distinguish listed outcome types; migration sidecar SHOULD define canonical vs derived vs export-visible outcomes. | SHOULD | BSC | 6.6 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Inbound Requirements |
| ULCOMP-R-186 | ULCOMP-F-020 | Change sets | If defined, SHOULD use stable paths/keys, prefer material deltas, define attachments/amendments, distinguish derived vs authored, preserve provenance. | SHOULD | BSC | 6.7 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Inbound Requirements |
| ULCOMP-R-187 | ULCOMP-F-020 | History moments | If moments defined, SHOULD be meaningful, reproducible from canonical/profile material, not dependent on raw UI telemetry. | SHOULD | BSC | 6.8 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Inbound Requirements |
| ULCOMP-R-188 | ULCOMP-F-020 | Respondent export views | Respondent-facing export views MUST remain derived or disclosure-oriented; MUST NOT become canonical truth. | MUST / MUST NOT | BSC | 6.9 | [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) §Scope and Subordination |
| ULCOMP-R-189 | ULCOMP-F-021 | Workflow mapping | If mappings defined, MUST distinguish operational non-canonical state, workflow events as canonical, governance/review as canonical, derived dashboards/queues/status. | MUST | BSC | 7.2 | [`../workflow/workflow-governance-provenance.md`](../workflow/workflow-governance-provenance.md) §Inbound Requirements |
| ULCOMP-R-190 | ULCOMP-F-021 | Governance facts | If fact families defined, SHOULD define which canonically admissible vs operational. | SHOULD | BSC | 7.3 | [`../workflow/workflow-governance-provenance.md`](../workflow/workflow-governance-provenance.md) §Inbound Requirements |
| ULCOMP-R-191 | ULCOMP-F-021 | Review semantics | If defined, SHOULD distinguish listed review stages; SHOULD define which outputs canonical vs derived workflow. | SHOULD | BSC | 7.4 | [`../workflow/workflow-governance-provenance.md`](../workflow/workflow-governance-provenance.md) §Inbound Requirements |
| ULCOMP-R-192 | ULCOMP-F-021 | Approval & recovery | If defined, SHOULD distinguish timer ops, human actions, retries, recovery, compensating, exceptional handling. | SHOULD | BSC | 7.5 | [`../workflow/workflow-governance-provenance.md`](../workflow/workflow-governance-provenance.md) §Inbound Requirements |
| ULCOMP-R-193 | ULCOMP-F-021 | Approval & recovery | Operational sequencing alone MUST NOT be mistaken for canonical order. | MUST NOT | BSC | 7.5 | [`../workflow/workflow-governance-provenance.md`](../workflow/workflow-governance-provenance.md) §Scope and Subordination |
| ULCOMP-R-194 | ULCOMP-F-021 | Provenance family | If defined, SHOULD trace derived to canonical, distinguish workflow from truth, preserve provenance in export, support rebuild. | SHOULD | BSC | 7.6 | [`../workflow/workflow-governance-provenance.md`](../workflow/workflow-governance-provenance.md) §Inbound Requirements |
| ULCOMP-R-195 | ULCOMP-F-021 | Conflict families | If defined, SHOULD distinguish unresolved/rejected/resolution/superseding/gating; MUST remain subordinate to no silent rewrite of canonical facts by derived systems. | SHOULD / MUST | BSC | 7.7 | [`../workflow/workflow-governance-provenance.md`](../workflow/workflow-governance-provenance.md) §Inbound Requirements |
| ULCOMP-R-196 | ULCOMP-F-021 | Workflow export views | MUST preserve provenance distinctions; MUST NOT imply broader coverage than declared export scope. | MUST / MUST NOT | BSC | 7.8 | [`../workflow/workflow-governance-provenance.md`](../workflow/workflow-governance-provenance.md) §Scope and Subordination |
| ULCOMP-R-197 | ULCOMP-F-022 | Registries | Implementations SHOULD define versioned registries for at least listed identifier/kind categories. | SHOULD | CR | App. A | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Schema/version compatibility policy |
| ULCOMP-R-198 | ULCOMP-F-023 | Rejection | MUST define rejection behavior for at least listed failure categories. | MUST | CR | App. B | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Canonization rejection codes |
| ULCOMP-R-199 | ULCOMP-F-023 | Rejection | Rejected submissions MUST NOT be treated as canonically appended. | MUST NOT | CR | App. B | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Canonization rejection codes |
| ULCOMP-R-200 | ULCOMP-F-023 | Rejection | If duplicates accepted as idempotent no-op or ref to existing record, MUST define that behavior explicitly. | MUST | CR | App. B | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Canonization rejection codes |
| ULCOMP-R-201 | ULCOMP-F-024 | Versioning | MUST version canonical algorithms and any schema or semantic digests, embedded copies, or immutable references needed for historical verification. | MUST | CR | App. C | [`../export/export-verification-package.md`](../export/export-verification-package.md) §10 |
| ULCOMP-R-202 | ULCOMP-F-024 | Versioning | MUST version author-originated fact semantics where profile- or binding-specific semantics exist. | MUST | CR | App. C | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Schema/version compatibility policy |
| ULCOMP-R-203 | ULCOMP-F-024 | Versioning | MUST version canonical record semantics, append semantics, export verification semantics, and trust profile semantics. | MUST | CR | App. C | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Schema/version compatibility policy |
| ULCOMP-R-204 | ULCOMP-F-024 | Versioning | MUST preserve enough information to verify historical records under the algorithms and rules in effect when they were produced. | MUST | CR | App. C | [`../export/export-verification-package.md`](../export/export-verification-package.md) §10 |
| ULCOMP-R-205 | ULCOMP-F-024 | Versioning | MUST NOT silently reinterpret historical records under newer rules without an explicit migration mechanism. | MUST NOT | CR | App. C | [`../export/export-verification-package.md`](../export/export-verification-package.md) §10 |
| ULCOMP-R-206 | ULCOMP-F-024 | Versioning | MUST ensure algorithm or schema evolution does not silently invalidate prior export verification. | MUST | CR | App. C | [`../export/export-verification-package.md`](../export/export-verification-package.md) §10 |
| ULCOMP-R-207 | ULCOMP-F-024 | Versioning | MUST NOT rely on out-of-band operator knowledge to interpret historical records. | MUST NOT | CR | App. C | [`../export/export-verification-package.md`](../export/export-verification-package.md) §10 |
| ULCOMP-R-208 | ULCOMP-F-024 | Versioning | MUST preserve enough immutable interpretation material to verify historical records without live registry lookups, mutable references, or out-of-band operator knowledge. | MUST | CR | App. C | [`../export/export-verification-package.md`](../export/export-verification-package.md) §10 |
| ULCOMP-R-209 | ULCOMP-F-025 | Security testing | Implementations SHOULD test canonical invariants via model checking, replay, property-based tests, protocol fuzzing. | SHOULD | CR | App. D | [`../assurance/assurance-traceability.md`](../assurance/assurance-traceability.md) §Minimum CI expectations |
| ULCOMP-R-210 | ULCOMP-F-026 | Migration guidance | Implementers SHOULD reduce offline coordination scope where possible. | SHOULD | CR | App. F | _(legacy only — non-normative migration guidance)_ |
| ULCOMP-R-211 | ULCOMP-F-026 | Migration guidance | Offline capabilities SHOULD be reserved for authoring, signing, bounded local transitions not requiring broad multi-party reconciliation. | SHOULD | CR | App. F | _(legacy only — non-normative migration guidance)_ |
| ULCOMP-R-212 | ULCOMP-F-026 | Migration guidance | Implementers SHOULD separate draft collaboration semantics from canonical semantics. | SHOULD | CR | App. F | _(legacy only — non-normative migration guidance)_ |
| ULCOMP-R-213 | ULCOMP-F-027 | Conformance boundary | Listed advanced capabilities not required for baseline core/companion conformance unless profile/binding/impl spec requires them. | (declarative) | CR | App. G | [`trellis-core.md`](trellis-core.md) §2.3 |
| ULCOMP-R-214 | ULCOMP-F-027 | Conformance boundary | Profiles/bindings/sidecars/impl specs MAY define such capabilities separately. | MAY | CR | App. G | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Conformance |
| ULCOMP-R-215 | ULCOMP-F-028 | Projection watermark | Staff-facing derived views MUST carry a watermark indicating canonical append/checkpoint state. | MUST | CR | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Normative Focus | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Normative Focus |
| ULCOMP-R-216 | ULCOMP-F-028 | Projection watermark fields | Every staff-facing projection MUST expose: canonical checkpoint identifier; canonical append height/sequence at build time; projection build timestamp; projection schema/version identifier. | MUST | CR | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Projection watermark contract | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Projection watermark contract |
| ULCOMP-R-217 | ULCOMP-F-028 | Stale projection indication | If a projection is stale relative to a newer canonical checkpoint, the view MUST indicate stale status. | MUST | CR | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Projection watermark contract | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Projection watermark contract |
| ULCOMP-R-218 | ULCOMP-F-028 | Purge cascade | Crypto-shredding is incomplete unless plaintext-derived projections/caches are purged according to policy. | MUST | CR | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Normative Focus | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Normative Focus |
| ULCOMP-R-219 | ULCOMP-F-028 | Rebuild equivalence | Rebuilding a projection from canonical records for the same checkpoint MUST yield semantically equivalent output for declared projection fields. | MUST | CR | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Rebuild verification | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Rebuild verification |
| ULCOMP-R-220 | ULCOMP-F-028 | Projection conformance tests | Projection conformance tests MUST validate watermark presence and stale-status behavior. | MUST | CR | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Rebuild verification | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Rebuild verification |
| ULCOMP-R-221 | ULCOMP-F-029 | Metadata budget | Each declared trust profile MUST include a metadata budget by canonical fact family (e.g. visible fields, observer classes, timing/access-pattern leakage, linkage stability, delegated-compute effects). | MUST | CR | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Metadata Budget Requirement | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Metadata Budget Requirement |
| ULCOMP-R-222 | ULCOMP-F-029 | Verification posture | Tiered verification deployments MUST declare verification posture classes and which downstream workflow or release classes each posture MAY feed; MUST NOT attach high-stakes outcomes to records below declared minimum posture; posture escalation MUST NOT be silent. | MUST / MUST NOT | CR | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Verification posture declaration | [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Verification posture declaration |
| ULCOMP-R-223 | ULCOMP-F-028 | Projection integrity policy | Each conforming deployment MUST define ongoing projection correctness checks including at least sampled rebuild comparison or checkpoint-bound equivalence; access-grant or authorization-expanding projections SHOULD be checked more frequently than general read models. | MUST / SHOULD | CR | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Projection integrity policy | [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Projection integrity policy |

---

## 7. User-Value Themes

These themes group `ULCOMP-R` rows by the operational-safety and trust-clarity properties they advance. They are read alongside [`unified-ledger-requirements-matrix.md`](unified-ledger-requirements-matrix.md) (core matrix) for full coverage.

| User-value theme | Primary ULCOMP-R IDs |
|------------------|----------------------|
| Append idempotency (no ambiguous retries) | ULCOMP-R-114–119 |
| Grants/revocations as canonical facts; evaluators subordinate | ULCOMP-R-095–099, ULCOMP-R-100, ULCOMP-R-102–105 |
| Provider vs reader-held vs delegated compute honesty | ULCOMP-R-106–111 |
| Metadata budget per canonical fact family | **ULCOMP-R-221** |
| Tiered verification posture vs high-stakes workflows | **ULCOMP-R-222** |
| Projection integrity (sampling or checkpoint equivalence) | **ULCOMP-R-223** |
| Rejection and duplicate handling | ULCOMP-R-198–200 |
| Erasure + derived plaintext invalidation | ULCOMP-R-159–160, **ULCOMP-R-218** |
| Staff projections: watermark + stale + mandatory fields | **ULCOMP-R-215–217** |
| Rebuild equivalence + projection conformance tests | **ULCOMP-R-219–220** |

**Invariant → verification methods:** see [`../assurance/assurance-traceability.md`](../assurance/assurance-traceability.md).

**Substrate binding** and **baseline advanced-crypto scope** are tracked in the core matrix (**ULCR-099–100**).

---

## 8. Coverage Notes

1. **Legacy §4.2** lists recognized sidecar families descriptively; no separate MUST beyond sidecar discipline (ULCOMP-R-173–174). Owning spec for sidecar discipline is [`shared-ledger-binding.md`](shared-ledger-binding.md) §Substrate binding (no second canonical truth) and [`trellis-core.md`](trellis-core.md) §11 (Companion Specifications).

2. **Legacy §5.7** (example comparison guidance) and **Appendix D** threat list are non-normative; only the testable **SHOULD** in Appendix D (ULCOMP-R-209) and the testable **SHOULD** in Appendix F (ULCOMP-R-210–212) are carried. **Appendix E** (Privacy Considerations Detail) is **deliberately omitted from this matrix as a normative source**: its five enumerated considerations (visible fact categories, timing patterns, access-pattern observability, disclosure linkability, user-held record reuse correlation risks) are subsumed by the metadata-budget obligation (`ULCOMP-R-221`, owned by [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Metadata Budget Requirement), which already requires every declared profile to document timing/access-pattern leakage, linkage stability, observer classes, and delegated-compute effects per fact family. Re-extracting Appendix E would create duplicate, weaker `SHOULD` rows for obligations that the metadata budget carries as `MUST`.

3. **Legacy §3.5** opening sentence (“Authentication mechanisms are not themselves canonical facts unless…”) is definitional; requirements begin at ULCOMP-R-135. Most §3.5 rows remain `(legacy only — no current owner)` pending a dedicated identity/attestation companion.

4. **Legacy §6.1** and **§7.1** state purpose; binding obligations appear in conditional “If … defined” rows (ULCOMP-R-181–188 and ULCOMP-R-189–196 respectively). These are owned by the stub specs [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md) and [`../workflow/workflow-governance-provenance.md`](../workflow/workflow-governance-provenance.md). The stubs exist to give rows a stable owning-spec reference; their `Status of This Document` clearly notes the legacy draft remains authoritative until normative prose is extracted.

5. Companion **PC** rows (legacy §2.x) overlap thematically with core [`unified-ledger-requirements-matrix.md`](unified-ledger-requirements-matrix.md) **ULCF-022–027**; both ID spaces are kept distinct so that core and companion obligations can be cited independently.

6. **ULCOMP-R-181–187** each summarize multiple conditional **SHOULD**/**MUST** bullets under legacy §6.2–6.8 (“If … defined”); split further if you need one ID per bullet for tooling.

7. **ULCOMP-R-215–220** track [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md); align IDs if that draft gains its own requirement table.

8. **ULCOMP-R-221** tracks [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Metadata Budget Requirement.

9. **ULCOMP-R-222** tracks [`../trust/trust-profiles.md`](../trust/trust-profiles.md) §Verification posture declaration; **ULCOMP-R-223** tracks [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md) §Projection integrity policy.

10. **Legacy-only rows remain authoritative in `DRAFTS/unified_ledger_companion.md`.** When a row marked `(legacy only — no current owner)` is later promoted into a current companion spec, update its **Owning Spec** column in this matrix (do not change the `ULCOMP-R-NNN` ID) and add a backreference in the receiving spec section of the form `(formerly ULCOMP-R-NNN; see DRAFTS/unified_ledger_companion.md §X.Y)`.

---

## 9. Cross-References

- **Trellis Core:** [`trellis-core.md`](trellis-core.md)
- **Shared Ledger Binding:** [`shared-ledger-binding.md`](shared-ledger-binding.md)
- **Trust Profiles:** [`../trust/trust-profiles.md`](../trust/trust-profiles.md)
- **Key Lifecycle Operating Model:** [`../trust/key-lifecycle-operating-model.md`](../trust/key-lifecycle-operating-model.md)
- **Export Verification Package:** [`../export/export-verification-package.md`](../export/export-verification-package.md)
- **Disclosure Manifest:** [`../export/disclosure-manifest.md`](../export/disclosure-manifest.md)
- **Projection and Runtime Discipline:** [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md)
- **Monitoring and Witnessing:** [`../operations/monitoring-witnessing.md`](../operations/monitoring-witnessing.md)
- **Assurance Traceability:** [`../assurance/assurance-traceability.md`](../assurance/assurance-traceability.md)
- **Forms and Respondent-History Sidecar (stub):** [`../forms/forms-respondent-history.md`](../forms/forms-respondent-history.md)
- **Workflow, Governance, and Provenance Sidecar (stub):** [`../workflow/workflow-governance-provenance.md`](../workflow/workflow-governance-provenance.md)
- **Core matrix (distinct ID space):** [`unified-ledger-requirements-matrix.md`](unified-ledger-requirements-matrix.md)
- **Legacy companion draft:** [`../../DRAFTS/unified_ledger_companion.md`](../../DRAFTS/unified_ledger_companion.md)
