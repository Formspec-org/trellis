# Unified Ledger Companion Specification

> **Normalization status (2026-04-13):** This file is now a legacy omnibus source draft.
> Active split-out draft work has started in `specs/`:
> - `core/shared-ledger-binding.md`
> - `trust/trust-profiles.md`
> - `trust/key-lifecycle-operating-model.md`
> Further companion extraction should continue here before this omnibus draft is retired.

## Abstract

This document is a companion to the **Unified Ledger Core Specification**.

It collects profile semantics, sidecars, appendices, examples, and companion requirements excluded from the constitutional core so the core can remain narrow, stable, and implementation-agnostic.

This document:

- MAY define profile-specific constraints,
- MAY define binding- or sidecar-oriented interpretation layers,
- MAY define reusable companion requirements that refine but do not reinterpret the core,
- MUST remain subordinate to the constitutional semantics of the Unified Ledger Core Specification.

It does not redefine canonical truth, canonical order, canonical append attestation semantics, trust honesty requirements, or export-verification guarantees established by the core.

## Status of This Document

This document is a draft companion specification.

Its purpose is to preserve important semantic and operational detail without expanding the constitutional core again.

The intended hierarchy is:

- the **core specification** defines what MUST remain true,
- this **companion specification** organizes subordinate profiles, sidecars, appendices, and reusable companion requirements,
- **bindings** define concrete serializations, proof encodings, APIs, and technology mappings,
- **implementation specifications** define exact operational procedures, deployment details, and reference stacks.

Additional requirements in this companion MUST be interpreted consistently with the core specification.

---

# Table of Contents

1. Companion Scope and Discipline
2. Standard Profiles
3. Cross-Cutting Companion Requirements
4. Sidecars
5. Trust Profile Example Sidecar
6. Forms and Respondent-History Sidecar
7. Workflow, Governance, and Provenance Sidecar
8. Companion Appendices
9. Non-Normative Guidance

---

# 1. Companion Scope and Discipline

## 1.1 Relationship to the Core

This document is subordinate to the Unified Ledger Core Specification.

Nothing in this document:

- creates a second canonical order,
- alters the definition of canonical truth,
- collapses derived artifacts into canonical truth,
- weakens trust honesty requirements,
- weakens export-verification guarantees.

## 1.2 Requirement Classes in this Companion

This companion uses the following requirement classes:

- **Profile constraint** for requirements that attach only to a declared profile,
- **Binding or sidecar choice** for technology-family or domain-family material,
- **Companion requirement** for subordinate but reusable semantic constraints that do not belong in the constitutional core,
- **Non-normative guidance** for advisory text.

## 1.3 Companion Reduction Rule

A capability belongs in this companion rather than the core when it:

- sharpens a profile rather than defining the constitutional model,
- clarifies a deployment posture rather than defining canonical truth,
- describes domain-family interpretation layers,
- describes operational admissibility, custody, disclosure, or workflow detail that remains subordinate to the core,
- captures appendix-grade material useful to implementers, verifiers, profile authors, or reference-spec authors.

## 1.4 Companion-Owned Detail

This companion holds detail too important to lose and too specific for the constitutional core.

That includes, at minimum:

- richer profile semantics,
- access-grant, delegation, and evaluator detail,
- export claim-class detail,
- lifecycle and legal posture detail,
- concrete trust-profile examples,
- forms and respondent-history family semantics,
- workflow, governance, and provenance family semantics.

---

# 2. Standard Profiles

# 2.1 Offline Authoring Profile

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

## 2.1.1 Offline Submission Semantics

**Requirement class: Profile constraint**

Offline-originated facts MAY be submitted after delay.

If accepted, the implementation:

- MUST preserve authored authentication semantics,
- MUST distinguish later admission and later append attestation from earlier authorship,
- MUST NOT imply that canonical append time is identical to authorship time unless that equivalence is established.

## 2.1.2 Pending Local State

**Requirement class: Profile constraint**

If local pending state exists before admission, that state:

- MUST remain non-canonical,
- MUST NOT define alternate canonical order,
- SHOULD remain separable from draft-collaboration state,
- MUST be transformable into submitted facts without silently rewriting prior authored facts.

---

# 2.2 Reader-Held Decryption Profile

**Requirement class: Profile constraint**

An implementation conforming to the Reader-Held Decryption Profile:

- MUST declare that ordinary service operation does not require general plaintext access for declared protected content,
- MUST identify which principals may decrypt within scope,
- MUST identify whether the provider can assist recovery,
- MUST remain consistent with the active Trust Profile,
- MUST distinguish reader-held access from provider-readable access,
- MUST distinguish reader-held access from delegated compute access.

## 2.2.1 Reader-Held Access Semantics

**Requirement class: Companion requirement**

Reader-held access means an explicitly authorized human or tenant-side principal can decrypt content within its scope.

Reader-held access:

- MUST NOT be described as provider-readable ordinary operation,
- MAY coexist with recovery assistance if the Trust Profile declares it honestly,
- MAY coexist with delegated compute if delegation remains explicit, scoped, and auditable.

---

# 2.3 Delegated Compute Profile

**Requirement class: Profile constraint**

An implementation conforming to the Delegated Compute Profile:

- MUST distinguish delegated compute access from provider-readable access,
- MUST make delegated compute explicit, attributable, and auditable,
- MUST define delegation scope,
- MUST define delegation authority,
- SHOULD define purpose bounds or time bounds,
- MUST NOT imply that delegated compute grants general service readability,
- MUST define what audit facts or audit events are emitted for delegation and use.

## 2.3.1 Delegated Compute Access Semantics

**Requirement class: Companion requirement**

Delegated compute access is a specific grant under which a compute agent or model is allowed to process declared content for a declared purpose.

A delegated compute grant:

- MUST be explicit,
- MUST be attributable to a principal, policy authority, or comparable authority,
- MUST be scoped to declared content or classes of content,
- SHOULD be time-bounded or purpose-bounded,
- MUST be auditable,
- MUST NOT be interpreted as conferring standing plaintext access to the ordinary service runtime.

## 2.3.2 Compute Output Reliance

**Requirement class: Companion requirement**

If workflow, policy, adjudication, access decisions, or materially consequential system actions rely on delegated compute output, the implementation:

- MUST record that output as a canonical fact or maintain a canonical reference to a stable output artifact,
- MUST preserve an auditable link to the authorizing principal,
- MUST preserve an auditable link to the compute agent identity,
- MUST preserve an auditable link to the scope of delegated access relevant to that output,
- MUST define whether the relied-upon output is advisory, recommendatory, or decision-contributory.

---

# 2.4 Disclosure and Export Profile

**Requirement class: Profile constraint**

An implementation conforming to the Disclosure and Export Profile:

- MUST support at least one verifiable disclosure or export form,
- MUST preserve the distinction between author-originated facts, canonical records, canonical append attestations, and later disclosure or export artifacts,
- MUST define which claims remain verifiable when payload readability is absent,
- MUST define profile-specific audience scope where relevant,
- MUST remain subordinate to the export guarantees of the core specification.

## 2.4.1 Export Claim Classes

**Requirement class: Companion requirement**

A Disclosure and Export Profile SHOULD state which of the following claim classes are verifiable within that profile:

- authorship claims,
- append or inclusion claims,
- payload-integrity claims,
- authorization-history claims,
- disclosure claims,
- lifecycle or compliance claims where included by scope.

An implementation MUST NOT imply that an export supports a claim class unless the export contains sufficient material to verify that class.

## 2.4.2 Selective Disclosure Discipline

**Requirement class: Companion requirement**

Selective disclosure SHOULD occur through disclosure or export artifacts rather than by overloading canonical records.

A disclosure-oriented artifact:

- MAY present an audience-specific subset or presentation,
- MUST preserve provenance distinctions,
- MUST NOT be treated as a rewrite of canonical truth.

---

# 2.5 User-Held Record Reuse Profile

**Requirement class: Profile constraint**

An implementation conforming to the User-Held Record Reuse Profile:

- MUST support submission or reference of previously user-held records, supporting material, or attestations,
- MUST bind exactly what was reused or disclosed when such material is introduced into canonical workflows,
- MUST distinguish reusable prior records from canonical workflow state,
- MUST distinguish workflow submission from prior-record possession,
- MUST avoid treating the entire user-held record layer as canonical workflow state by default.

## 2.5.1 Selective Submission Preference

**Requirement class: Companion requirement**

If an implementation supports user-held reusable prior records, selective submission SHOULD be favored over bulk transfer of unrelated user-held content.

## 2.5.2 Reuse Provenance

**Requirement class: Companion requirement**

When reused material is introduced into canonical truth, the implementation:

- MUST bind what was introduced,
- SHOULD bind the reuse context where relevant,
- MUST preserve provenance distinctions among pre-existing user-held material, canonical submission, and later disclosure artifacts.

---

# 2.6 Respondent History Profile

**Requirement class: Profile constraint**

An implementation conforming to the Respondent History Profile:

- MUST scope itself to respondent-originated or respondent-visible material history,
- MAY support respondent-history moments such as draft, save, submit, amendment, attachment, validation, prepopulation, or materially relevant attestation boundaries,
- MUST treat respondent-history views as projections or profile-specific interpretations over canonical truth rather than as a separate source of truth,
- MUST NOT define a second canonical append model,
- MUST NOT imply complete workflow, governance, custody, or compliance coverage unless the declared profile scope actually includes those materials.

## 2.6.1 Materiality Discipline

**Requirement class: Profile constraint**

A Respondent History Profile:

- MUST prioritize material respondent-side state changes over raw UI telemetry,
- MUST NOT require keystroke, focus, blur, rendering, or equivalent ephemeral interface event capture,
- SHOULD expose validation, submission, amendment, and materially relevant identity or attestation boundaries where they matter to human review,
- MAY define profile-specific change-set semantics aligned to stable form-path and item-key semantics where those concepts exist.

## 2.6.2 Coverage Honesty

**Requirement class: Profile constraint**

A respondent-history export or view:

- MAY present a profile-specific timeline or delta history,
- MUST preserve provenance distinctions,
- MUST NOT imply broader workflow, governance, custody, or compliance coverage than the declared profile scope actually provides.

---

# 3. Cross-Cutting Companion Requirements

## 3.0 Profile, Binding, and Sidecar Inheritance Rules

### 3.0.1 Trust Profile Inheritance Across Profiles, Bindings, and Sidecars

**Requirement class: Companion requirement**

All profiles, bindings, and sidecars inherit the active Trust Profile.

A profile, binding, or sidecar:

- MUST remain consistent with the active Trust Profile,
- MUST distinguish provider-readable access, reader-held access, and delegated compute access when protected content is involved,
- MUST NOT imply stronger confidentiality, weaker provider visibility, or weaker recovery capability than the active Trust Profile supports,
- MUST NOT use profile-local, binding-local, or sidecar-local wording to weaken or bypass Trust Profile requirements.

### 3.0.2 Profile-Scoped Export and View Honesty

**Requirement class: Companion requirement**

Any profile-scoped export, sidecar-scoped export, audience-specific view, or family-specific presentation:

- MUST preserve the distinction between author-originated facts, canonical records, canonical append attestations, and later disclosure or export artifacts,
- MUST preserve provenance distinctions even when presenting a profile-specific timeline, delta history, or interpretation layer,
- MUST NOT imply broader workflow, governance, custody, compliance, or disclosure coverage than its declared scope actually includes.

---

# 3.1 Access Grants and Revocations

**Requirement class: Companion requirement**

Access grants and revocations that affect canonical authorization semantics MUST be recorded as append-only canonical facts.

Authorization evaluators MAY be derived artifacts.

If so, they:

- MUST be rebuildable from canonical grant and revocation facts,
- MUST NOT be authoritative for grant existence, grant history, or revocation history,
- MUST preserve canonical grant and revocation semantics even when evaluator state is absent, stale, or rebuilding.

## 3.1.1 Delegation Facts

**Requirement class: Companion requirement**

If delegation affects authorization, legal authority, or access posture, delegation grants and revocations MUST be recorded as canonical facts.

## 3.1.2 Sharing-Mode Discipline

**Requirement class: Companion requirement**

If a system supports both narrow per-record or per-scope sharing and long-lived collaborative membership, the implementation SHOULD avoid forcing both use cases into a single mechanism if doing so increases key-management or audit complexity.

## 3.1.3 Derived Evaluator Rebuild and Stale-State Behavior

**Requirement class: Companion requirement**

If a derived evaluator is used for access, policy, workflow, or other rights-impacting decisions, the implementation:

- MUST be able to trace evaluator inputs back to canonical facts,
- MUST define evaluator rebuild behavior,
- MUST define behavior when evaluator state is stale, missing, inconsistent with canonical facts, or unavailable during rebuild,
- MUST preserve the rule that evaluator state does not override canonical grant and revocation semantics.

---

# 3.2 Provider Access, Reader Access, and Delegated Compute

**Requirement class: Companion requirement**

Implementations handling protected content MUST distinguish the following forms of access:

- **Provider-readable access**: the service operator or ordinary service-side components can decrypt content during ordinary operation.
- **Reader-held access**: an explicitly authorized human or tenant-side principal can decrypt content within its scope.
- **Delegated compute access**: a specific compute agent or model is granted scoped access to process content for a specific purpose.

A conforming implementation MUST describe these categories consistently with its actual behavior.

## 3.2.1 Profile Honesty Detail

**Requirement class: Companion requirement**

A conforming implementation:

- MUST disclose whether provider-readable access exists in ordinary operation,
- MUST disclose whether delegated compute is provider-operated, tenant-operated, client-side, or otherwise isolated,
- MUST disclose what metadata remains visible to the service or other observers,
- MUST NOT describe a trust posture more strongly than those facts support.

---

# 3.3 Canonical Append Service Semantics

**Requirement class: Companion requirement**

The Canonical Append Service MUST:

- validate append admissibility,
- preserve append-only semantics,
- issue canonical append attestations,
- retain or reference sufficient proof material for later verification.

The Canonical Append Service MUST NOT, by virtue of its canonical role alone, be required to:

- decrypt protected payloads,
- evaluate workflow policy,
- resolve workflow runtime state,
- compute projections or search indexes,
- inspect protected content unless the active Trust Profile explicitly permits or requires such access.

## 3.3.1 Canonical Append Idempotency Semantics

**Requirement class: Companion requirement**

Canonical append operations MUST define idempotency semantics for retried, replayed, or duplicate submissions.

Canonical append operations MUST define a stable idempotency key or equivalent causal submission identity.

A conforming implementation MUST define whether such a submission is:

- rejected,
- treated as a no-op,
- resolved to an existing canonical record reference,
- or otherwise handled by an explicitly declared idempotency rule consistent with canonical append semantics.

Duplicate or retried submission handling MUST NOT create ambiguity about whether a canonical record was newly appended, previously appended, or not admitted.

For a given idempotency identity within a declared append scope, every successful retry MUST resolve to the same canonical record reference or the same declared no-op outcome.

If idempotent acceptance is supported, the implementation MUST define the verifier-visible consequences of that behavior.

## 3.3.2 Verifier-Facing Proof Model Discipline

**Requirement class: Companion requirement**

A conforming implementation MUST present one verifier-facing canonical append proof model per declared append scope at a time.

An implementation MUST NOT require verifiers to reconcile multiple overlapping append-attestation semantics for the same canonical scope.

If an implementation changes that proof model, it MUST define an explicit migration boundary so verifiers never need to reconcile overlapping append-attestation semantics for the same canonical scope.

Implementations SHOULD use a transparency-log-style append model with:

- append order,
- inclusion proofs,
- consistency proofs between append heads.

## 3.3.3 External Witnessing

**Requirement class: Binding or sidecar choice**

Implementations MAY support external witnessing or anchoring.

External witnessing:

- MUST remain subordinate to the canonical append semantics of the core,
- MUST NOT be required for correctness unless a declared profile or binding explicitly states otherwise,
- MAY strengthen detection of equivocation or strengthen independent audit posture.

---

# 3.4 Conflict Handling

**Requirement class: Companion requirement**

Implementations MAY define conflict-sensitive fact categories.

Conflict handling MUST be evaluated within the declared append scope of the affected canonical facts.

If conflict-sensitive facts require resolution:

- canonical append in unaffected append scopes MUST continue,
- affected derived systems, policies, or workflows MAY gate on explicit resolution facts,
- derived artifacts MUST NOT silently rewrite canonical facts in order to resolve conflicts,
- conflict resolution SHOULD be represented through later canonical facts, explicit rejection, or profile-defined admission rules.

An implementation MUST NOT stall unrelated append scopes solely because conflict handling remains unresolved in another declared append scope.

---

# 3.5 Identity, Attestation, and Continuity Detail

**Requirement class: Companion requirement**

Authentication mechanisms are not themselves canonical facts unless explicitly bound into canonical truth by an active profile or binding.

If identity or attestation facts are represented canonically or in profile-specific bindings, the implementation SHOULD represent those facts provider-neutrally where feasible.

Provider-specific issuers, adapters, or bindings MAY be used operationally, but they SHOULD NOT define the long-lived semantic meaning of the attestation within the constitutional model.

## 3.5.1 User-Originated Signing

**Requirement class: Companion requirement**

Implementations SHOULD support user-originated signatures.

Implementations MAY support offline user-originated signing.

If user-originated signing is supported, the resulting evidence package MUST distinguish:

- the user-originated signature or equivalent authored authentication,
- the later canonical append attestation.

## 3.5.2 Assurance, Disclosure, and Continuity

**Requirement class: Companion requirement**

A conforming implementation:

- MUST distinguish assurance level from disclosure posture,
- MUST NOT treat higher assurance as requiring greater identity disclosure by default,
- MAY support subject continuity without requiring full legal identity disclosure,
- MUST preserve those distinctions across trust profiles, exports, disclosures, and sidecars.

---

# 3.6 Protected Payloads and Access Material

**Requirement class: Companion requirement**

Sensitive content SHOULD reside in protected payloads when payload protection is required by the active Trust Profile or binding.

Implementations MUST define which data remains visible for canonical verification and which data is payload-protected.

Canonical records containing protected payloads MUST include or reference sufficient access material for authorized recipients according to the selected custody mode and applicable binding.

A conforming representation MUST preserve a semantic distinction between:

- the author-originated fact,
- protected payload content where such content exists,
- access or key-wrapping material where such material exists,
- canonical append attestation material.

---

# 3.7 Storage, Snapshots, and Availability Detail

**Requirement class: Companion requirement**

Canonical records MUST be stored durably and immutably from the perspective of ordinary append participants.

Protected payloads MAY be stored in one or more blob stores.

Canonical acceptance MUST define which durable write conditions are required.

A conforming implementation MUST declare the durable-append boundary that governs attestation, retry handling, and export issuance for a canonical record.

Any proof material or referenced state required to recover or verify a canonical record within the declared export scope MUST be durably recoverable no later than that boundary.

Replica completion state MUST remain operational state, not canonical truth.

Snapshots MAY be used for performance, but snapshots MUST be treated as derived artifacts and MUST NOT become canonical truth.

---

# 3.8 Lifecycle, Erasure, and Legal Sufficiency

**Requirement class: Companion requirement**

Implementations MAY define lifecycle facts for:

- retention,
- legal hold,
- archival,
- key destruction,
- sealing,
- schema upgrade,
- export issuance.

An implementation MAY support only a subset of these lifecycle operations or none of them.

If an implementation supports one of these operations as part of its canonical or compliance-relevant behavior, it MUST represent that operation as a lifecycle fact.

If such a fact affects compliance posture, retention posture, or recoverability claims, it MUST be a canonical fact.

## 3.8.1 Erasure and Key Destruction

**Requirement class: Companion requirement**

If an implementation uses cryptographic erasure or key destruction, it MUST document:

- which content becomes irrecoverable,
- who retains access, if anyone,
- what evidence of destruction is preserved,
- what metadata remains.

If protected content is cryptographically destroyed or made inaccessible under the implementation's lifecycle rules, affected derived plaintext state MUST be invalidated, purged, or otherwise made unusable according to the implementation's declared policy.

## 3.8.2 Sealing and Later Lifecycle Facts

**Requirement class: Companion requirement**

A conforming implementation MUST define whether sealed cases, sealed records, or equivalent sealed scopes permit later lifecycle or governance facts.

The implementation MUST also define whether retention or hold rules take precedence when both apply.

## 3.8.3 Legal Sufficiency Statement

**Requirement class: Companion requirement**

Implementations MUST NOT imply that cryptographic controls alone guarantee admissibility or legal sufficiency in all jurisdictions.

Implementations MAY claim stronger evidentiary posture only to the extent supported by process, signature semantics, canonical append attestations, records practice, and applicable law.

---

# 3.9 Privacy Detail and Metadata Minimization

**Requirement class: Companion requirement**

Implementations MUST document what is protected from whom.

Payload confidentiality MUST NOT be described as equivalent to metadata privacy.

If provider-readable access exists in ordinary operation, the implementation MUST say so plainly.

If delegated compute access exists without general provider readability, the implementation MUST distinguish that model from provider-readable custody.

## 3.9.1 Metadata Minimization Detail

**Requirement class: Companion requirement**

Metadata minimization is a hard design constraint.

Visible metadata SHOULD be limited to what is required for:

- canonical verification,
- schema or semantic lookup,
- required audit-visible declarations,
- conflict gating,
- append processing.

Implementations SHOULD NOT keep visible metadata merely to accelerate derived artifacts.

Implementations MUST NOT retain visible append-related metadata merely for operational convenience when the same function can be satisfied by derived or scoped mechanisms.

Implementations SHOULD reduce offline coordination scope and visible coordination metadata where doing so does not weaken canonical verifiability.

---

# 4. Sidecars

## 4.1 Sidecar Discipline

**Requirement class: Binding or sidecar choice**

A sidecar MAY collect family-specific, deployment-specific, or implementation-adjacent material that remains subordinate to the constitutional core and this companion document.

A sidecar MUST NOT:

- redefine canonical truth,
- redefine canonical order,
- collapse provenance distinctions,
- weaken trust honesty obligations,
- weaken export-verification guarantees.

## 4.2 Sidecar Families Recognized by this Companion

**Requirement class: Binding or sidecar choice**

This companion recognizes these sidecar families:

- trust-profile example sidecars,
- forms and respondent-history sidecars,
- workflow, governance, and provenance sidecars,
- user-held record reuse sidecars,
- deployment-profile sidecars,
- binding-family sidecars.

---

# 5. Trust Profile Example Sidecar

## 5.1 Purpose

**Requirement class: Binding or sidecar choice**

This sidecar provides concrete examples of trust and custody postures without altering the semantic honesty requirements of the core and companion documents.

These examples are illustrative and MUST NOT override actual Trust Profile declarations.

## 5.2 Example Profile A — Provider-Readable Custodial

**Requirement class: Binding or sidecar choice**

Characteristics:

- ordinary service operation is provider-readable,
- the provider can decrypt current content in ordinary operation,
- historical content remains provider-readable unless separately sealed or destroyed,
- recovery may occur without the user if the deployment declares that capability,
- delegated compute, if present, is not confidentiality-improving because provider-readable operation already exists.

A profile using this posture MUST say so plainly and MUST NOT imply provider blindness.

## 5.3 Example Profile B — Reader-Held with Recovery Assistance

**Requirement class: Binding or sidecar choice**

Characteristics:

- ordinary service operation is not generally provider-readable,
- explicitly authorized readers can decrypt within scope,
- a recovery authority may assist recovery under declared conditions,
- recovery assistance does not by itself imply ordinary provider-readable operation,
- the Trust Profile MUST describe who can assist recovery and under what conditions.

## 5.4 Example Profile C — Reader-Held with Tenant-Operated Delegated Compute

**Requirement class: Binding or sidecar choice**

Characteristics:

- ordinary service operation is not generally provider-readable,
- explicitly authorized readers decrypt within scope,
- delegated compute is possible only through explicit, scoped delegation,
- delegated compute is tenant-operated or otherwise isolated from the ordinary service runtime,
- the Trust Profile MUST state whether plaintext becomes visible to any provider-operated components during delegation.

## 5.5 Example Profile D — Threshold-Assisted Custody

**Requirement class: Binding or sidecar choice**

Characteristics:

- decryption or recovery requires cooperation by multiple parties or custodians,
- no single ordinary operator is sufficient to recover protected content,
- recovery conditions, quorum thresholds, and exceptional access posture MUST be declared,
- threshold participation MUST NOT be described more strongly than the actual recovery process supports.

## 5.6 Example Profile E — Delegated Organizational Custody

**Requirement class: Binding or sidecar choice**

Characteristics:

- an organization, tenant authority, or delegated administrative authority controls recovery or access posture,
- authorized organizational actors may manage access grants, revocations, and delegation,
- the Trust Profile MUST identify the scope of organizational authority and any exceptional-access controls,
- this posture MUST still distinguish provider-readable access from organization-controlled access where they differ.

## 5.7 Example Comparison Guidance

**Requirement class: Non-normative guidance**

These examples are useful when documenting tradeoffs among:

- who can read current content,
- who can read historical content,
- whether recovery is possible without the user,
- whether delegated compute exposes plaintext to ordinary service components,
- who controls canonical append attestation,
- who can block or administer exceptional access.

---

# 6. Forms and Respondent-History Sidecar

## 6.1 Purpose

**Requirement class: Binding or sidecar choice**

This sidecar defines concrete forms-family and respondent-history semantics that remain subordinate to the core and companion profiles.

Its purpose is to preserve stable respondent-visible meaning across drafts, submissions, amendments, validation cycles, and schema migrations without turning those family semantics into constitutional requirements.

## 6.2 Stable Path Semantics

**Requirement class: Binding or sidecar choice**

A forms-family sidecar MAY define stable path semantics for addressing logically stable locations within a form, questionnaire, or structured respondent artifact.

If stable path semantics are defined, they:

- SHOULD remain stable across non-material presentation changes,
- SHOULD identify semantically meaningful positions rather than transient rendering positions,
- MUST distinguish structural path meaning from display-order accidents,
- SHOULD define behavior when a path is deprecated, split, merged, or migrated,
- SHOULD support respondent-history deltas and validation snapshots without relying on UI telemetry.

## 6.3 Item-Key Semantics

**Requirement class: Binding or sidecar choice**

A forms-family sidecar MAY define item-key semantics for stable identification of repeatable items, attachments, or list elements.

If item-key semantics are defined, they:

- SHOULD identify a stable logical item across save, submit, amend, and export cycles,
- SHOULD distinguish a changed item from a newly inserted item when the family can do so reliably,
- SHOULD define when keys are preserved, regenerated, merged, or invalidated,
- MUST NOT rely on ephemeral client rendering state as the sole source of identity.

## 6.4 Validation Snapshot Structure

**Requirement class: Binding or sidecar choice**

A respondent-history sidecar MAY define validation snapshot structure.

If validation snapshot structure is defined, it SHOULD:

- capture materially relevant validation outcomes rather than keystroke-level activity,
- bind the applicable stable paths and item keys where relevant,
- distinguish between blocking validation, advisory validation, and informational validation,
- record the validation boundary or scope to which the snapshot applies,
- permit later verification of what validation result existed at a material history boundary.

## 6.5 Amendment Cycle Semantics

**Requirement class: Binding or sidecar choice**

A forms-family sidecar MAY define amendment cycle semantics.

If amendment cycle semantics are defined, they SHOULD:

- distinguish amendment initiation, amendment in progress, amendment submission, and amendment completion,
- define whether amendment cycles operate against a prior canonical submission, a prior respondent-visible version, or another declared baseline,
- preserve linkage between the amended artifact and the prior baseline,
- distinguish additive amendment, corrective amendment, superseding amendment, and partial amendment where those concepts exist,
- define whether abandoned amendments remain visible in respondent-history views.

## 6.6 Migration Outcome Semantics

**Requirement class: Binding or sidecar choice**

A forms-family sidecar MAY define migration outcome semantics for schema or version changes.

If migration outcome semantics are defined, they SHOULD distinguish at least:

- unchanged carry-forward,
- transformed carry-forward,
- split or merged outcomes,
- dropped or deprecated outcomes,
- review-required outcomes,
- migration failure outcomes.

A migration sidecar SHOULD define whether migration outcomes become canonical facts, derived migration metadata, or export-visible annotations.

## 6.7 Respondent-History Change-Set Structure

**Requirement class: Binding or sidecar choice**

A respondent-history sidecar MAY define profile-specific change-set structure.

If change sets are defined, they SHOULD:

- use stable paths and item keys where relevant,
- prefer material respondent-visible deltas over UI-event streams,
- define how attachments, deletions, replacements, and amendments are represented,
- define how derived or inferred changes are distinguished from directly authored changes,
- preserve provenance distinctions between authored input, validation interpretation, and later presentation.

## 6.8 Respondent-Visible History Moments

**Requirement class: Binding or sidecar choice**

A respondent-history sidecar MAY define concrete respondent-visible history moments, including:

- draft creation,
- save,
- submit,
- attachment add or replace,
- validation boundary reached,
- prepopulation applied,
- amendment start,
- amendment submit,
- materially relevant attestation boundary.

Such moments SHOULD be defined in a way that is:

- meaningful to human review,
- reproducible from canonical or profile-defined source material,
- not dependent on raw UI telemetry.

## 6.9 Respondent-History Export Views

**Requirement class: Binding or sidecar choice**

A respondent-history sidecar MAY define respondent-facing export views.

Such views MAY include:

- timeline views,
- delta views,
- amendment views,
- validation-summary views,
- migration-annotation views.

These views MUST remain derived or disclosure-oriented and MUST NOT become canonical truth.

---

# 7. Workflow, Governance, and Provenance Sidecar

## 7.1 Purpose

**Requirement class: Binding or sidecar choice**

This sidecar defines workflow-family, governance-family, review-family, and provenance-family interpretation layers that remain subordinate to the constitutional core.

Its purpose is to preserve rich workflow semantics without allowing workflow runtime state to become canonical by default.

## 7.2 Workflow State and Canonical Fact Mapping

**Requirement class: Binding or sidecar choice**

A workflow-family sidecar MAY define how operational workflow state maps, when necessary, to canonical facts.

If such mappings are defined, they MUST distinguish:

- operational state that remains non-canonical,
- workflow events that become canonical facts,
- governance or review facts that become canonical facts,
- derived dashboards, queues, and status views.

## 7.3 Governance and Processing Facts

**Requirement class: Binding or sidecar choice**

A workflow or governance sidecar MAY define governance and processing fact families, including:

- intake receipt facts,
- assignment facts,
- review-open and review-close facts,
- adjudication or recommendation facts,
- approval or denial facts,
- escalation facts,
- verification-upgrade facts,
- export-issuance facts,
- lifecycle or compliance boundary facts.

If such fact families are defined, the sidecar SHOULD define which of them are canonically admissible and which remain operational.

## 7.4 Review and Adjudication Semantics

**Requirement class: Binding or sidecar choice**

A governance sidecar MAY define review and adjudication semantics.

If such semantics are defined, the sidecar SHOULD distinguish:

- review assignment,
- review in progress,
- review completion,
- recommendation,
- adjudicative decision,
- appeal or reconsideration,
- override or exception.

The sidecar SHOULD define which review outputs are canonical facts and which are derived workflow state.

## 7.5 Approval, Escalation, and Recovery Semantics

**Requirement class: Binding or sidecar choice**

A workflow sidecar MAY define approval, escalation, retry, and recovery semantics.

If such semantics are defined, the sidecar SHOULD distinguish:

- timer-driven operational behavior,
- human approval or review actions,
- system retries,
- recovery procedures,
- compensating actions,
- exceptional handling.

Operational sequencing alone MUST NOT be mistaken for canonical order.

## 7.6 Provenance Family Semantics

**Requirement class: Binding or sidecar choice**

A provenance sidecar MAY define family-specific provenance semantics for how canonical facts, derived artifacts, workflow state, and export views relate.

If such provenance semantics are defined, they SHOULD:

- trace derived outputs back to canonical inputs,
- distinguish workflow interpretation from canonical truth,
- preserve provenance across export packaging,
- support rebuild of derived views where feasible.

## 7.7 Conflict and Resolution Families

**Requirement class: Binding or sidecar choice**

A workflow or governance sidecar MAY define conflict-sensitive categories and resolution families.

If such semantics are defined, the sidecar SHOULD distinguish:

- admissible but unresolved conflict,
- explicit rejection,
- explicit resolution,
- superseding resolution,
- profile-specific gating conditions.

These families MUST remain subordinate to the core rule that canonical facts are not silently rewritten by derived systems.

## 7.8 Workflow Export Views

**Requirement class: Binding or sidecar choice**

A workflow-family sidecar MAY define workflow or governance export views, including:

- case history views,
- governance timeline views,
- review-chain views,
- adjudication summary views,
- compliance lifecycle views.

Such views MUST preserve provenance distinctions and MUST NOT imply broader coverage than their declared export scope actually includes.

---

# 8. Companion Appendices

# Appendix A. Suggested Registries

**Requirement class: Companion requirement**

Implementations SHOULD define versioned registries for at least:

- fact kinds,
- schema or semantic digests or immutable references,
- custody modes,
- trust profile identifiers,
- lifecycle fact kinds,
- disclosure or export artifact kinds,
- profile identifiers,
- sidecar identifiers where used.

---

# Appendix B. Error Handling and Rejection Semantics

**Requirement class: Companion requirement**

A conforming implementation MUST define rejection behavior for at least:

- invalid signatures or invalid required authored authentication,
- malformed author-originated facts or malformed canonical records under the active profile or binding,
- unsupported schema or algorithm versions,
- duplicate submissions,
- revoked or expired delegated compute authority,
- unauthorized access or disclosure attempts.

Rejected submissions MUST NOT be treated as canonically appended.

If duplicate submissions are accepted as idempotent no-ops or as references to an already appended canonical record, the implementation MUST define that behavior explicitly.

---

# Appendix C. Versioning and Algorithm Agility Detail

**Requirement class: Companion requirement**

A conforming implementation:

- MUST version canonical algorithms and any schema or semantic digests, embedded copies, or immutable references needed for historical verification,
- MUST version author-originated fact semantics where profile- or binding-specific semantics exist,
- MUST version canonical record semantics, append semantics, export verification semantics, and trust profile semantics,
- MUST preserve enough information to verify historical records under the algorithms and rules in effect when they were produced,
- MUST NOT silently reinterpret historical records under newer rules without an explicit migration mechanism,
- MUST ensure that algorithm or schema evolution does not silently invalidate prior export verification,
- MUST NOT rely on out-of-band operator knowledge to interpret historical records.

A conforming implementation MUST preserve enough immutable interpretation material to verify historical records without live registry lookups, mutable references, or out-of-band operator knowledge.

---

# Appendix D. Security Considerations Detail

**Requirement class: Non-normative guidance**

Implementers should consider at least the following:

- key compromise,
- verifier or parser divergence,
- metadata leakage,
- replay and reordering attacks,
- recovery abuse,
- authorization drift between canonical facts and derived evaluators,
- snapshot misuse,
- service equivocation,
- delayed offline submission edge cases,
- over-broad delegated compute grants,
- accidental expansion from delegated compute access into standing provider-readable access.

Implementations SHOULD test canonical invariants using model checking, replay testing, property-based testing, and protocol fuzzing.

---

# Appendix E. Privacy Considerations Detail

**Requirement class: Non-normative guidance**

This system may support strong payload confidentiality, but it does not eliminate metadata disclosure by default.

Implementers should consider:

- visible fact categories,
- timing patterns,
- access-pattern observability,
- disclosure linkability,
- user-held record reuse correlation risks.

---

# Appendix F. Migration Guidance

**Requirement class: Non-normative guidance**

A migration from a pre-existing system should preserve, where possible:

- browser-originated authoring,
- append-only facts,
- explicit grant and revocation facts,
- projection rebuildability,
- explicit custody modes,
- verifiable export goals.

It should replace, where possible:

- bespoke receipt-chain logic,
- overlapping proof models,
- custom authorization evaluators,
- custom workflow runtimes in the canonical path,
- disclosure defaults that over-centralize high-risk cryptographic features.

Implementers SHOULD reduce offline coordination scope where possible.

Offline capabilities SHOULD be reserved for authoring, signing, and bounded local state transitions that do not require broad multi-party reconciliation.

Implementers SHOULD separate draft collaboration semantics from canonical semantics.

---

# Appendix G. Companion Conformance Boundary

**Requirement class: Companion requirement**

The following capabilities are not required for baseline conformance to the core specification or to this companion unless a declared profile, binding, or implementation specification explicitly requires them:

- advanced selective disclosure,
- threshold custody,
- group-sharing protocols,
- advanced homomorphic or privacy-preserving computation,
- cross-agency analytic protocols.

Profiles, bindings, sidecars, or implementation specifications MAY define such capabilities separately.

---

# 9. Non-Normative Guidance

## 9.1 Recommended Technology Shape

A practical implementation may use:

- deterministic CBOR or another binding-defined canonical encoding,
- signed or authenticated authored objects,
- protected payloads with access material,
- a transparency-log-style append service,
- a transactional canonical store,
- workflow orchestration as a derived operational layer,
- relationship-based and policy-based authorization as derived evaluators,
- independently verifiable export packaging.

Implementers should prefer a model in which:

- the service stores and receipts protected content by default,
- authorized readers decrypt within scope,
- delegated compute receives explicit scoped access when needed,
- the ordinary service runtime does not gain broad plaintext access merely because a compute task required plaintext,
- durable workflow execution, retries, timers, approvals, and recovery procedures are handled outside the canonical core.

## 9.2 Suggested Capability Families

A conservative companion profile may use:

- a transparency-log-backed append service,
- a transactional canonical store,
- object storage for protected payloads,
- a durable workflow execution layer for retries, timers, approvals, and recovery procedures,
- a derived relationship-based authorization layer,
- a derived policy evaluation layer,
- phishing-resistant or federated authentication,
- verifiable disclosure or export artifacts.

A more ambitious companion profile may additionally use:

- threshold custody,
- group key management for collaborative sharing,
- advanced selective disclosure mechanisms,
- external witnessing or anchoring services.

## 9.3 Final Companion Recommendation

The companion should function as the semantic spillway for important detail that would otherwise bloat the constitutional core.

A useful practical test is:

- keep the core small enough to survive infrastructure churn,
- keep the companion rich enough to support profile authors, sidecar authors, and reference-spec builders,
- keep bindings and implementation specs concrete enough that verifiers and operators can interoperate without guessing.

The point of this companion is to preserve important detail without letting the constitution become an architecture document.
