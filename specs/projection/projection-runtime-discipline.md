---
title: Trellis Companion — Projection and Runtime Discipline
version: 0.1.0-draft.3
date: 2026-04-15
status: draft
---

# Trellis Companion — Projection and Runtime Discipline v0.1

**Version:** 0.1.0-draft.3
**Date:** 2026-04-15
**Editors:** Formspec Working Group
**Companion to:** Trellis Core v0.1

---

## Status of This Document

This document is a **draft specification**. It is a companion to the Trellis Core specification and does not modify Formspec or WOS processing semantics. Implementors are encouraged to experiment with this specification and provide feedback, but MUST NOT treat it as stable for production use until a 1.0.0 release is published.

## Conventions and Terminology

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in BCP 14 [RFC 2119] [RFC 8174] when, and only when, they appear in ALL CAPITALS, as shown here.

JSON syntax and data types are as defined in [RFC 8259].

## Abstract

The Projection and Runtime Discipline companion defines strict rules for derived systems so that canonical truth never drifts into a hidden second truth. It specifies projection categories, watermarking, rebuild contracts, snapshot discipline, purge-cascade requirements, authorization-evaluator behavior, and the runtime boundary between canonical ledgers and derived processors. This companion adds projection semantics to the Trellis Core model (Core S4–S5). It does not define WOS runtime semantics.

## Table of Contents

1. Introduction
2. Conformance
3. Terminology
4. The Canonical Truth Boundary
5. Derived Artifact Requirements
6. Projection Categories
7. Projection Watermark Contract
8. Rebuild Contract and Verification
9. Projection Integrity Policy
10. Authorization Evaluator Behavior
11. Workflow State and Canonical Fact Mapping
12. Provenance Family Semantics
13. Storage, Snapshots, and Availability
14. Purge-Cascade Requirement
15. Runtime Boundary
16. Upstream Cross-References
17. Security and Privacy Considerations
18. Cross-References

---

## 1. Introduction

### 1.1 Scope

This companion governs the discipline that derived systems — projections, evaluator state, caches, timelines, dashboards, queues, indexes, snapshots, and workflow runtime — MUST observe so that none of them silently become a second source of canonical truth.

The companion adds operational rules to Trellis Core (S4–S5) for admission, watermarking, rebuild, snapshot, purge-cascade, and runtime boundary concerns. It does not redefine canonical append, order, hash, or attestation semantics, which remain authoritative in Trellis Core (S5–S7).

### 1.2 Relationship to Trellis Core

Trellis Core (S5.2, Invariant 2) establishes that **derived artifacts MUST NOT be treated as canonical truth**. This companion refines that invariant into operational requirements on the classes of derived artifact that deployments actually build.

All normative requirements in this companion MUST be read as subordinate to Trellis Core. Where this companion's text appears to conflict with Trellis Core, Trellis Core prevails.

### 1.3 Design Goal

Prevent multi-source-of-truth drift at the runtime boundary by requiring every derived artifact to be rebuildable, provenance-anchored, and non-authoritative relative to canonical records.

---

## 2. Conformance

This companion defines the following conformance roles:

1. **Projection Producer** — produces derived projections from canonical records. MUST comply with the watermark contract (S7), rebuild contract (S8), projection integrity policy (S9), and stale-status requirements (S7).
2. **Projection Verifier** — validates projection correctness against canonical inputs. MUST support rebuild comparison (S8) and watermark checks (S7).
3. **Authorization Evaluator** — a derived evaluator that makes, or contributes inputs to, rights-impacting decisions. MUST comply with S10 in full.

A conforming implementation MUST satisfy all requirements applicable to each claimed role.

Roles defined in Trellis Core (Core S2.1) remain applicable. An implementation that materializes projections is acting as a Derived Processor (Core S2.2) in addition to whichever role is claimed here.

---

## 3. Terminology

This section defines terms as used in this companion. Where a term is also used in Trellis Core, the Core definition governs; these entries are expository restatements scoped to this companion. The generic canonical-versus-derived distinction inherits from [WOS Kernel §12 Separation Principles]; the entries below are scoped to ledger-specific projection, watermark, and purge-cascade semantics.

### 3.1 Canonical Truth

The set of admitted canonical records, canonical append attestations, and canonical checkpoint material, as defined by Trellis Core (Core S5.1). Canonical truth excludes all derived runtime state. The canonical-versus-derived boundary itself is the Trellis instantiation of [WOS Kernel §12 Separation Principles].

### 3.2 Derived Artifact

Any runtime or materialized artifact computed from canonical records, including but not limited to queues, dashboards, indexes, caches, read models, materialized views, timelines, search projections, snapshots, and evaluator state. Cited in Trellis Core (Core S3.4). See [WOS Kernel §12 Separation Principles] for the generic separation framing.

### 3.3 Projection

A class of derived artifact that presents canonical state (or a function of it) to a consumer — staff, respondent, or system. Every projection is a derived artifact; the term is used where presentation semantics matter.

### 3.4 Snapshot

A derived artifact capturing a consistent materialized view of canonical state as of a declared canonical checkpoint, typically retained for performance, recovery, or export support. A snapshot is not canonical truth; it references canonical truth.

### 3.5 Evaluator

A derived component that computes decisions (authorization, policy, workflow progression, access gating) from canonical facts plus declared configuration. An evaluator that participates in rights-impacting decisions is an **Authorization Evaluator** and is subject to S10.

### 3.6 Watermark

Metadata affixed to a projection that identifies the canonical state from which the projection was built, sufficient to determine freshness relative to current canonical append height.

### 3.7 Purge-Cascade

The set of operations that propagate cryptographic erasure, key destruction, or policy-driven purge through every derived artifact that holds plaintext or plaintext-derived material, such that the purged content is rendered inaccessible in all derived stores as well as in canonical storage.

### 3.8 Rights-Impacting Decision

A decision that grants, denies, expands, contracts, delegates, or revokes access, authority, or capability relative to canonical records, subjects, or resources. Authorization decisions are rights-impacting; routine read-model refreshes are not.

### 3.9 Canonical Checkpoint

A Trellis Core concept (Core S5.1) referring to the append-attested state at a declared canonical order position. Watermarks in this companion reference canonical checkpoints.

---

## 4. The Canonical Truth Boundary

**Requirement class: Constitutional semantic**

The generic separation principle — that audit and governance state are not authoritative over execution and case state — is owned by [WOS Kernel §12 Separation Principles]. This section names and anchors that principle as it applies to a canonical ledger: in this implementation, **canonical records are authoritative**, and every derived artifact derives from them rather than the other way around.

### 4.1 PRD-01 — No Derived Artifact Is Canonical Truth

**PRD-01 (MUST).** No projection, evaluator state, cache, snapshot, timeline, dashboard, queue, index, read model, materialized view, workflow runtime state, or any other derived artifact is authoritative for canonical facts. Canonical truth is defined exclusively by Trellis Core (Core S5.1).

PRD-01 is the ledger-operational restatement of Trellis Core Invariant 3 (Core S5.2, "Derived Artifact Is Not Canonical Truth") and of [WOS Kernel §12 Separation Principles] for a canonical-ledger deployment. Every subsequent section of this companion that defines derived artifact behavior MUST be read as subordinate to PRD-01.

### 4.2 Enforcement Posture

A conforming implementation MUST:

- treat every artifact described in Sections 6 through 15 of this companion as a derived artifact, and therefore as non-canonical, regardless of its durability, retention, signing, or operational centrality,
- reject any configuration, policy, or disclosure that would elevate a derived artifact to canonical status,
- when derived artifact state conflicts with canonical records, resolve to canonical records and treat the derived artifact as stale, inconsistent, or in need of rebuild.

---

## 5. Derived Artifact Requirements

**Requirement class: Constitutional semantic**

This section restates, for clarity of this companion, the derived artifact requirements established in Trellis Core and in the unified ledger core text (§16.1).

### 5.1 PRD-02 — Derived Artifact Obligations

**PRD-02 (MUST).** A derived artifact:

1. MUST NOT be authoritative for canonical facts (see PRD-01),
2. MUST be rebuildable from canonical truth plus declared configuration history,
3. MUST record sufficient provenance to identify the canonical state from which it was derived,
4. MUST treat lag, rebuild, or loss as an operational condition rather than a change to canonical truth.

PRD-02 applies uniformly to staff-facing projections, respondent-facing projections, system projections, snapshots, and evaluator state.

### 5.2 Declared Configuration History

"Declared configuration history" includes every input that influences the derived artifact's shape other than canonical records themselves — for example, projection schema versions, filter predicates, evaluator policy versions, translation tables, and aggregation rules. An implementation MUST retain enough of this history that a rebuild at a prior canonical checkpoint yields the same derived output that was produced at that checkpoint, for fields declared rebuild-deterministic (S8.2).

---

## 6. Projection Categories

**Requirement class: Companion requirement**

A conforming Projection Producer MUST classify each projection it produces and apply the corresponding ledger rules. The categories below are scoped to ledger-display and ledger-rebuild obligations; see `trust-profiles.md` S4 for audience-scoped metadata budgets.

### 6.1 Consumer-Facing Projections

Projections delivered to a human consumer (staff or respondent). These MUST carry a watermark (S7) and MUST surface stale status (S7.3). Respondent-scoped views MUST additionally respect the trust profile's metadata budget (`trust-profiles.md` S4).

### 6.2 System Projections

Internal caches, indexes, read models, and materialized views used by the platform itself. System projections MUST be rebuildable from canonical records (Core S6, S8 of this companion) and MUST internally record the watermark fields of S7.1, but are exempt from the display requirements in S7.2. Purge-cascade rules (S14) apply.

### 6.3 Category Declaration

A Projection Producer MUST declare the category of each projection it produces. Changing a projection's category is a configuration change (S5.2) and MUST be captured in declared configuration history.

---

## 7. Projection Watermark Contract

**Requirement class: Companion requirement**

### 7.1 PRD-03 — Required Watermark Fields

**PRD-03 (MUST).** Every projection in scope of S7 MUST carry a watermark that exposes at minimum:

1. the canonical checkpoint identifier from which the projection was built,
2. the canonical append height or sequence position at build time,
3. the projection build timestamp,
4. the projection schema and version identifier.

### 7.2 Display Requirements

Consumer-facing projections (S6.1) MUST display or otherwise make available to the consumer the watermark fields required for that consumer to assess freshness. Implementations MAY elide fields that are not meaningful to the consumer (for example, schema version on respondent-scoped views) but MUST NOT elide the canonical checkpoint reference.

### 7.3 Stale Status

If a projection is stale relative to a newer canonical checkpoint available to the producer, the view MUST indicate stale status. Stale indications MUST NOT reveal the content of canonical updates that have not yet been projected (see S17).

### 7.4 System Projection Exemption

System projections (S6.2) are exempt from display requirements (S7.2) but MUST record the watermark fields of S7.1 internally such that rebuild, purge-cascade, and verifier operations can determine the canonical state that produced them.

---

## 8. Rebuild Contract and Verification

**Requirement class: Companion requirement**

### 8.1 PRD-04 — Rebuild Obligation

**PRD-04 (MUST).** Rebuilding a projection from canonical records, at the same canonical checkpoint and under the same declared configuration (S5.2), MUST yield semantically equivalent output for every projection field declared rebuild-deterministic.

### 8.2 Declared Fields

A Projection Producer MUST declare which fields of each projection are rebuild-deterministic. Fields that intentionally incorporate non-canonical inputs (for example, live operational metrics) MUST be declared non-deterministic and MUST NOT be relied upon for verification.

### 8.3 Fixtures

Implementations SHOULD retain deterministic rebuild fixtures for critical projection types. Fixtures MUST be protected against tampering; a compromised fixture could mask projection drift from canonical truth (see S17).

### 8.4 Conformance Tests

Projection conformance tests MUST validate watermark presence (S7.1) and stale-status behavior (S7.3).

---

## 9. Projection Integrity Policy

**Requirement class: Companion requirement**

Each conforming deployment MUST define how projection correctness is checked over time. The policy MUST include at least one of the following mechanisms:

1. **Sampled rebuild comparison.** Periodically or on demand, rebuild declared projection fields from canonical inputs for a sample of records or sequence ranges and compare against materialized projection state.
2. **Checkpoint-bound equivalence.** At declared epoch boundaries, record a content commitment (for example, a hash) for projection state in checkpoint or export material, and verify rebuild matches that commitment before treating the snapshot as authoritative for recovery.

Authorization-expanding projections (that is, projections whose output is consumed by an Authorization Evaluator in S10) SHOULD be checked at higher frequency than general read models.

---

## 10. Authorization Evaluator Behavior

**Requirement class: Companion requirement**

An Authorization Evaluator (S3.5) is a derived evaluator that participates in rights-impacting decisions (S3.8). This section defines the mandatory behavior of such evaluators. It adapts the unified ledger companion (§3.1, §3.1.3) and the unified ledger core (§16.1) into this companion.

### 10.1 PRD-05 — Traceability to Canonical Facts

**PRD-05 (MUST).** An Authorization Evaluator MUST be able to trace every input contributing to a rights-impacting decision back to the canonical facts from which the input was derived. Evaluator inputs that cannot be traced to canonical facts MUST NOT contribute to rights-impacting decisions.

### 10.2 PRD-06 — Rebuild Behavior

**PRD-06 (MUST).** An Authorization Evaluator MUST define its rebuild behavior. The definition MUST specify:

1. the canonical inputs required to rebuild evaluator state,
2. the declared configuration history required to rebuild evaluator state deterministically,
3. the procedure by which a rebuild is initiated, completed, and verified,
4. the expected relationship between a rebuilt evaluator and canonical records at the rebuild checkpoint (for example, strict equivalence on grant and revocation outcomes).

### 10.3 PRD-07 — Behavior Under Stale, Missing, Inconsistent, or Unavailable State

**PRD-07 (MUST).** An Authorization Evaluator MUST define its behavior when evaluator state is:

1. **stale** relative to current canonical facts,
2. **missing** (no evaluator state exists for the scope),
3. **inconsistent** with canonical facts (evaluator state disagrees with canonical records),
4. **unavailable during rebuild** (evaluator state cannot be consulted because it is being rebuilt or is otherwise temporarily inaccessible).

For each of these conditions, the implementation MUST declare — in advance, as part of its conformance statement — whether a rights-impacting decision under that condition:

- is deferred (the decision is not made until the condition is resolved),
- fails closed (the default is to deny the rights-expanding outcome),
- falls back to a declared recovery evaluator sourced from canonical facts, or
- is rejected outright.

Silent fail-open behavior — granting or preserving access because evaluator state cannot be consulted — is NON-CONFORMANT. Unspecified behavior under any of the four conditions enumerated above is NON-CONFORMANT.

### 10.4 PRD-08 — Canonical Semantics Prevail

**PRD-08 (MUST).** An Authorization Evaluator MUST preserve canonical grant and revocation semantics regardless of evaluator state. Evaluator state MUST NOT override, suppress, delay, or reinterpret a grant or revocation recorded as a canonical fact. Where evaluator state and canonical facts disagree, canonical facts prevail and the evaluator MUST be treated as stale or inconsistent per S10.3.

PRD-08 is the non-negotiable safety rule of this section. It restates, for rights-impacting decisions specifically, the boundary established by PRD-01 (S4.1). A system that applies evaluator state as a veto or override on canonical grant or revocation semantics violates both PRD-08 and Trellis Core Invariant 3.

### 10.5 Interaction With Key Lifecycle

Cryptographic-erasure events recorded as canonical lifecycle facts (see `key-lifecycle-operating-model.md`) invalidate any evaluator state that depends on cryptographically destroyed material. Such invalidation cascades into evaluator state exactly as it cascades into other derived artifacts (S14).

---

## 11. Workflow State and Canonical Fact Mapping

**Requirement class: Companion requirement**

The generic separation of lifecycle state from case state is owned by [WOS Kernel §12(1) Lifecycle vs. Case State Separation]. This section adds the ledger-specific canonical-admission rule that governs when a workflow event crosses into canonical truth in a Trellis deployment. Workflow runtime is a derived processor (Core S2.2) and is governed by PRD-01.

### 11.1 PRD-09 — Canonical-Admission Distinctions

**PRD-09 (MUST).** A workflow-family deployment that maps operational workflow state to canonical facts MUST distinguish, in its configuration and in any projections it produces:

1. **operational state that remains non-canonical** — in-flight task assignments, transient queue memberships, scheduler ticks, ephemeral session data (governed by [WOS Kernel §12(1)]),
2. **workflow events that become canonical facts** — intake receipts, review-open and review-close events, adjudicative decisions, governance and review outcomes — but only where the active binding declares them canonically admissible,
3. **derived dashboards, queues, and status views** — which remain derived artifacts under PRD-01 and are subject to PRD-02 through PRD-04.

### 11.2 Non-Elevation

No operational sequencing, queue depth, scheduler event, or workflow runtime state is canonical truth solely by virtue of its operational role. Elevation to canonical status requires the workflow event to be admitted as a canonical record by the active binding (Core S6).

---

## 12. Provenance Family Semantics

**Requirement class: Companion requirement**

This section adapts the unified ledger companion (§7.6) into this companion.

### 12.1 PRD-10 — Provenance Obligations

**PRD-10 (MUST).** When a deployment defines provenance semantics for how canonical facts, derived artifacts, workflow state, and export views relate, those semantics MUST:

1. trace derived outputs back to the canonical inputs from which they were computed,
2. distinguish workflow interpretation from canonical truth, preserving the boundary established by PRD-01 and PRD-09,
3. preserve provenance across export packaging, so that consumers of export artifacts can recover the derived-to-canonical trace (see `trellis-core.md` S8 for verifier obligations on export-package canonical provenance claims),
4. support rebuild of derived views from the preserved provenance, consistent with PRD-02 and PRD-04.

### 12.2 Interpretation Layers

Family-specific interpretation layers (workflow timelines, review chains, adjudication summaries) remain derived artifacts. They MUST NOT imply broader workflow, governance, custody, compliance, or disclosure coverage than their declared scope actually includes.

---

## 13. Storage, Snapshots, and Availability

**Requirement class: Companion requirement**

This section adapts the unified ledger companion (§3.7) and the unified ledger core (§16.4) into this companion.

### 13.1 PRD-11 — Durable-Append Boundary

**PRD-11 (MUST).** A conforming implementation MUST declare the durable-append boundary that governs attestation, retry handling, and export issuance for a canonical record. The boundary MUST be expressed such that consumers and verifiers can determine whether a given canonical record has crossed it.

### 13.2 PRD-12 — Proof Material Recoverability

**PRD-12 (MUST).** Any proof material or referenced state required to recover or verify a canonical record within the declared export scope MUST be durably recoverable no later than the durable-append boundary declared under PRD-11.

### 13.3 Replica Completion State

Replica completion state MUST remain operational state, not canonical truth. The presence, absence, or synchronization lag of any individual replica is a derived condition and MUST NOT be treated as modifying canonical facts.

### 13.4 Snapshots

Snapshots are derived artifacts under PRD-01 and are subject to PRD-02. Snapshots MAY be used for performance, recovery, or export support, but MUST NOT become canonical truth. A snapshot MUST reference the canonical checkpoint state from which it was built (S7.1) and MUST NOT be relied upon for verification unless its rebuild equivalence has been established under S9.

### 13.5 Protected Payload Storage

Protected payloads MAY be stored in one or more blob stores. Such storage is subordinate to the active Trust Profile (see `trust-profiles.md`) and does not alter canonical append semantics.

---

## 14. Purge-Cascade Requirement

**Requirement class: Companion requirement**

### 14.1 PRD-13 — Cascade Obligation

**PRD-13 (MUST).** If canonical lifecycle facts declare that protected content has been cryptographically destroyed, sealed, or otherwise made inaccessible, every derived artifact that holds plaintext or plaintext-derived material subject to that declaration MUST be invalidated, purged, or otherwise made unusable according to the implementation's declared policy.

Cryptographic erasure is incomplete until the purge-cascade completes. An implementation that retains plaintext in a derived artifact after a canonical erasure event is NON-CONFORMANT regardless of the mechanism by which canonical content was destroyed.

### 14.2 Scope of Cascade

The cascade MUST reach, at minimum:

1. consumer-facing and system projections (S6.1, S6.2),
2. evaluator state that incorporated the destroyed material,
3. snapshots, including those retained for performance or recovery,
4. caches, indexes, and materialized views,
5. rebuild fixtures (S8.3) that contain the destroyed material.

Backups are governed by the implementation's retention and recovery policy but MUST NOT be used to resurrect destroyed plaintext into live derived artifacts.

### 14.3 Interaction With Key Lifecycle

See `key-lifecycle-operating-model.md` for the canonical lifecycle-fact semantics that trigger the cascade. Cryptographic-erasure invalidation cascades into projection state through PRD-13.

---

## 15. Runtime Boundary

**Requirement class: Companion requirement**

### 15.1 PRD-14 — Workflow Runtime Is a Derived Processor

**PRD-14 (MUST).** Workflow and orchestration engines are derived processors (Core S2.2), not canonical ledgers. Their runtime state is a derived artifact under PRD-01 and is subject to PRD-02.

### 15.2 Binding-Declared Admission Only

A workflow or orchestration engine contributes to canonical truth only by submitting facts through the Canonical Append Service under the active binding's admission rules (Core S6). The engine MUST NOT write canonical records out-of-band, replay them into canonical order independently, or reinterpret admitted records.

---

## 16. Upstream Cross-References

The following topics are owned upstream and are excluded from this companion:

- Generic separation of audit, governance, execution, and case state — see [WOS Kernel §12 Separation Principles].
- Lifecycle-vs-case-state separation as it applies to workflow runtime — see [WOS Kernel §12(1)].
- Workflow execution and orchestration semantics, governance-time behavior, and orchestration policy — see [WOS Governance: Workflow Governance].

This companion's Runtime Boundary (S15) restricts how workflow runtime state relates to canonical truth in a ledger deployment. It does not prescribe workflow execution semantics, which remain owned by [WOS Governance].

---

## 17. Security and Privacy Considerations

Audience-scoped metadata budgets and projection content disclosure are owned by `trust-profiles.md` S4. The items below are the projection-runtime-specific safety considerations.

### 17.1 Stale-Status Disclosure

Stale-status indications on projections (S7.3) MUST NOT reveal the content of canonical updates that have not yet been projected. A stale indicator communicates freshness relative to canonical append height; it MUST NOT be used as a covert channel for the content of unprojected updates.

### 17.2 Purge-Cascade Completeness

Purge-cascade operations (S14) MUST NOT leave residual plaintext in system projections, caches, backups, evaluator state, or rebuild fixtures. An incomplete cascade undermines the confidentiality guarantees of cryptographic erasure recorded as a canonical lifecycle fact (see `key-lifecycle-operating-model.md` S7).

### 17.3 Rebuild Fixture Integrity

Rebuild verification fixtures (S8.3) MUST be protected against tampering. Compromised fixtures could mask projection drift from canonical truth, defeating the verification discipline of S8 and S9.

### 17.4 Authorization Evaluator Safety

An Authorization Evaluator whose stale-state behavior is undeclared, silently fail-open, or permissive by omission is a security defect regardless of implementation effort (S10.3, S10.4). Deployments SHOULD treat PRD-07 and PRD-08 as safety-critical and SHOULD include them in their monitoring-and-witnessing posture (see `monitoring-witnessing.md`).

---

## 18. Cross-References

This companion relates to the following upstream and Trellis documents. Citations use section numbers where available.

**Upstream:**

- **[WOS Kernel §12 Separation Principles]** — generic separation of audit, governance, execution, and case state. PRD-01 (S4.1) is the ledger-deployment instantiation of this principle.
- **[WOS Kernel §12(1) Lifecycle vs. Case State Separation]** — generic lifecycle-vs-case-state framing referenced by S11 (PRD-09).
- **[WOS Governance: Workflow Governance]** — workflow execution and orchestration semantics referenced by S15 and S16.

**Trellis:**

- **`trellis-core.md`** — the foundation specification. This companion refines:
  - Invariant 3 (Core S5.2, "Derived Artifact Is Not Canonical Truth") → anchored here as PRD-01 (S4.1),
  - canonical truth boundary (Core S5.1) → scoped here for derived artifacts (S4, S5),
  - Derived Processor role (Core S2.2) → extended here with projection-specific obligations (S6–S9) and Authorization Evaluator obligations (S10),
  - verifier obligations on export-package canonical provenance claims (Core S8) → referenced by S12.1.
- **`trust-profiles.md`** — governs metadata budgets for respondent-facing and staff-facing views (S6.2, S17.1) and the Trust Profile inheritance posture that derived artifacts MUST respect.
- **`key-lifecycle-operating-model.md`** — canonical lifecycle facts for cryptographic erasure and key destruction; cryptographic-erasure invalidation cascades into projection and evaluator state (S10.5, S14.3, S17.3).
- **`monitoring-witnessing.md`** — observability posture for detecting projection drift, stale evaluator state, and purge-cascade incompleteness (S17.5).

---

## Appendix A. Migrated Requirements Map

The following requirements were migrated from the unified ledger drafts into this companion.

| Source                                          | Destination in this companion          |
|-------------------------------------------------|----------------------------------------|
| Unified ledger core §7.1 Invariant 3            | S4.1 (PRD-01)                          |
| Unified ledger core §16.1                       | S5.1 (PRD-02); S10.1, S10.2, S10.3     |
| Unified ledger core §16.4                       | S13.1–S13.5 (PRD-11, PRD-12)           |
| Unified ledger companion §3.1                   | S10.1, S10.2, S10.4 (PRD-05, PRD-06, PRD-08) |
| Unified ledger companion §3.1.3                 | S10.1–S10.4 (PRD-05 through PRD-08)    |
| Unified ledger companion §3.7                   | S13.1–S13.5                            |
| Unified ledger companion §7.2                   | S11 (PRD-09)                           |
| Unified ledger companion §7.6                   | S12 (PRD-10)                           |
