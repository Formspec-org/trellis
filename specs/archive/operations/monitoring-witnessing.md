---
title: Trellis Companion — Monitoring and Witnessing
version: 0.1.0-draft.2
date: 2026-04-14
status: draft
---

# Trellis Companion — Monitoring and Witnessing v0.1

**Version:** 0.1.0-draft.2
**Date:** 2026-04-14
**Editors:** Formspec Working Group
**Companion to:** Trellis Core v0.1

---

## Status of This Document

This document is a **draft specification**. It is a companion to the Trellis Core specification ([trellis-core.md](../core/trellis-core.md)) and does not modify Formspec or WOS processing semantics. Implementors are encouraged to experiment with this specification and provide feedback, but MUST NOT treat it as stable for production use until a 1.0.0 release is published.

This companion is subordinate to Trellis Core. Nothing in this document redefines canonical truth, canonical order, canonical append attestation semantics, or the export-verification guarantees established by the core.

## Table of Contents

1. Introduction
2. Conventions and Terminology
3. Witness Subordination to Canonical Correctness
4. Monitor and Witness Sub-Roles
5. Checkpoint Publication Interface
6. Append-Head Consistency Checking Protocol
7. Witness Attestation Semantics
8. External Anchoring Semantics
9. Equivocation: Definition and Evidence Format
10. Detection Versus Enforcement
11. Testability Hooks
12. Conformance
13. Security Considerations
14. Privacy Considerations
15. Cross-References

---

## 1. Introduction

### 1.1 Scope

The Monitoring and Witnessing companion defines publication and verification seams for independent monitors, witnesses, and external anchoring services that observe the Trellis canonical substrate defined in Trellis Core (§5–§9). It specifies:

- the checkpoint publication interface exposed by a Canonical Append Service,
- the append-head consistency checking protocol consumed by monitors,
- the semantics and minimum content of witness attestations,
- the semantics of anchoring to an external transparency log, blockchain, or timestamping authority,
- the canonical definition of service equivocation and the publication format for equivocation evidence,
- the separation of detection from enforcement.

This companion does not define witness network topology, consensus protocols, quorum election mechanisms, wire formats, or deployment architecture. Those belong in bindings, profiles, or implementation specifications.

### 1.2 Non-Redefinition

Monitoring and witnessing are observational seams layered on top of the canonical append semantics defined in Trellis Core §7 (Fact Admission, Canonicalization, and Order). A witness, monitor, or anchor MUST NOT be treated as authoritative for canonical truth, canonical order, or canonical append attestation. The Canonical Append Service defined in Trellis Core remains the single issuer of canonical append attestations within a declared append scope.

### 1.3 Design Goal

Enable independent observers to detect append-service misbehavior — in particular, equivocation, canonical-order rewriting, and append-head inconsistency — without enlarging the authoritative surface. Monitors and witnesses add assurance posture; they do not add canonical authority.

---

## 2. Conventions and Terminology

### 2.1 BCP 14

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in BCP 14 [RFC 2119] [RFC 8174] when, and only when, they appear in ALL CAPITALS, as shown here.

JSON syntax and data types are as defined in [RFC 8259]. URI syntax is as defined in [RFC 3986].

### 2.2 Terminology

For the purposes of this document, the following terms are defined. Where a term is also used in Trellis Core, the core definition governs and this companion narrows it to its monitoring and witnessing use.

- **Append-head.** The append-head reference (Trellis Core §4.10) representing the current tip of canonical order within a declared append scope at a given logical time. An append-head commits to every prior canonical record within that scope.
- **Checkpoint.** A published snapshot of an append-head at a declared append position (log index) within a declared append scope, suitable for independent observation. A checkpoint is not itself a canonical append attestation; it is a derived publication over canonical material.
- **Consistency proof.** Proof material demonstrating that one append-head extends another within the same declared append scope — that is, that every canonical record committed by the earlier head is also committed by the later head in the same order. A consistency proof is specified by the active binding (for example, a transparency-log consistency proof over a Merkle append tree).
- **Witness.** An independent observer that attests to having observed published checkpoint material and, where the profile requires, to having verified append-growth consistency between checkpoints. A witness does not issue canonical append attestations.
- **Monitor.** An independent observer that consumes checkpoint and append-growth publications to detect misbehavior of the Canonical Append Service. A monitor MAY additionally act as a witness if it publishes attestations; the two roles are distinguished by publication behavior, not observation.
- **Anchor.** A record binding an append-head (or a digest of one) into an external log — a public transparency log, a blockchain, or a timestamping authority — such that the append-head's existence at a declared external time is independently evidenced by a system outside the Canonical Append Service's administrative scope.
- **Equivocation.** Publication by a Canonical Append Service, within a single declared append scope and at a single declared append position, of two or more incompatible append-head references (or of append-heads that do not share a common prefix consistent with append-only order). Equivocation presents forked canonical histories to different observers. Equivocation is defined further in S9.
- **Relying party.** Any party that draws assurance from a witness attestation, anchor record, or monitor publication when evaluating canonical-order, inclusion, or append-growth claims.

---

## 3. Witness Subordination to Canonical Correctness

**Requirement class: Binding or sidecar choice.**

External witnessing is a subordinate assurance posture, not a replacement for canonical append semantics. Implementations MAY support external witnessing or anchoring. Where they do, the following obligations apply.

External witnessing:

- MUST remain subordinate to the canonical append semantics of the core,
- MUST NOT be required for correctness unless a declared profile or binding explicitly states otherwise,
- MAY strengthen detection of equivocation or strengthen independent audit posture.

This section is the most important normative rule in this companion. It governs every other section. Three consequences follow:

1. **Witness absence does not invalidate canonical records.** A canonical append attestation issued by a conforming Canonical Append Service remains canonical even when no witness has observed the corresponding checkpoint.
2. **Witness disagreement does not rewrite canonical order.** If a witness reports inconsistency, canonical order as established by Trellis Core §7.4 is unchanged. Witness reports trigger detection workflows; they do not mutate canonical truth.
3. **Profiles MAY elevate.** A profile or binding MAY declare that canonical correctness within its scope requires witness participation (for example, a profile requiring quorum-witnessed checkpoints before export issuance). Such elevation MUST be explicit in the profile or binding and MUST define the minimum witness participation required.

---

## 4. Monitor and Witness Sub-Roles

**Requirement class: Companion requirement.**

This companion distinguishes four operational sub-roles. A single deployed component MAY implement more than one sub-role; a conforming implementation MUST declare which sub-roles each monitor or witness instance implements. Sub-roles have distinct observation rights and distinct publication obligations.

### 4.1 Passive Consistency Monitor

A passive consistency monitor consumes the checkpoint publication interface (S5) and verifies append-head consistency (S6) between successive observed checkpoints. It:

- MUST verify that each successive observed checkpoint's append-head extends the prior observed checkpoint's append-head within the declared append scope,
- MUST retain sufficient observed checkpoint material to reproduce any detected inconsistency,
- MAY emit private operational alerts on detecting inconsistency,
- MUST NOT issue witness attestations unless it also implements the witness sub-role,
- MUST NOT be treated by relying parties as evidence of third-party corroboration by virtue of its passive role alone.

### 4.2 Active Equivocation Detector

An active equivocation detector compares checkpoint material observed from the Canonical Append Service across multiple independent observation points (for example, different vantage points, different query channels, or checkpoints fetched by other detectors). It:

- MUST compare append-head references observed at the same declared append position within the same declared append scope,
- MUST treat incompatible append-head references at the same position as equivocation evidence (S9),
- MUST publish equivocation evidence in the format defined in S9.3,
- SHOULD obtain observations from vantage points it does not control wholly, to increase the difficulty of collusive suppression,
- MAY coordinate with witnesses and anchoring services to strengthen evidence provenance.

### 4.3 External Anchoring Witness

An external anchoring witness publishes append-head references (or digests thereof) into an external log — a public transparency log, a blockchain, or a timestamping authority — producing anchor records (S8). It:

- MUST produce anchor records conforming to S8.2,
- MUST preserve the distinction between a canonical append attestation (issued by the Canonical Append Service) and an anchor record (issued by the external log),
- MUST NOT present anchor records as canonical append attestations,
- SHOULD publish anchor records at a cadence declared by the active profile or binding.

### 4.4 Audit Witness

An audit witness issues witness attestations intended for later verifier or auditor consumption within a declared governance scope. It:

- MUST issue witness attestations conforming to S7,
- MUST declare the scope, cadence, and observation method of its attestations,
- MUST NOT attest to properties it did not observe (for example, MUST NOT attest to inclusion of a record it has not seen committed in an observed checkpoint),
- SHOULD declare its independence posture — operator identity, administrative separation from the Canonical Append Service, and any shared infrastructure — so relying parties can evaluate assurance weight.

---

## 5. Checkpoint Publication Interface

**Requirement class: Companion requirement.**

A Canonical Append Service that supports monitoring and witnessing MUST expose a checkpoint publication interface. The interface is protocol-agnostic; conformance is defined by the resource model, not the wire format. Bindings MAY define REST, gRPC, or other concrete encodings.

### 5.1 Required Resources

A checkpoint publication interface MUST expose at minimum:

| Resource | Description |
|---|---|
| Append scope identifier | Stable identifier for the declared append scope (Trellis Core §7.4). |
| Checkpoint identifier | Stable identifier for the checkpoint within the declared append scope. |
| Append position | Monotonic log index (Trellis Core §7.5) at checkpoint time. |
| Append-head reference | Canonical append-head reference at the checkpoint's append position (Trellis Core §4.10, §7.5). |
| Checkpoint time | Service-declared time at which the checkpoint was produced. |
| Consistency proof material | Proof material (as defined by the active binding) sufficient to verify that this checkpoint's append-head extends any earlier published checkpoint within the same append scope. |
| Inclusion proof material | Optional per-record inclusion proof material for records committed by this checkpoint, where the binding supports it. |
| Pagination and range query | Support for listing checkpoints by append position range or by time range. |

### 5.2 Publication Obligations

A conforming Canonical Append Service:

- MUST publish checkpoints at a cadence declared by the active profile or binding,
- MUST NOT publish checkpoints whose append-head reference does not correspond to a canonical append-head actually established under Trellis Core §7,
- MUST NOT revise a previously published checkpoint; corrections MUST be represented as later checkpoints and MUST NOT rewrite append position assignments,
- MUST preserve sufficient proof material to allow consistency proofs between any two published checkpoints within the same declared append scope.

---

## 6. Append-Head Consistency Checking Protocol

**Requirement class: Companion requirement.**

### 6.1 Monitor Obligation

A passive consistency monitor, and any other monitor sub-role that verifies append-growth, MUST check that each successive observed checkpoint's append-head extends the prior observed checkpoint's append-head within the declared append scope. "Extends" means there exists, under the active binding's proof model, a consistency proof demonstrating append-only growth from the prior append-head to the later append-head without reordering or removal of previously committed records.

A monitor MUST retrieve or compute the binding-defined consistency proof material for each successive checkpoint pair it evaluates.

### 6.2 Consistency Failure

A consistency failure occurs when, within a single declared append scope, any of the following conditions hold:

1. A later checkpoint's append-head cannot be proven under the active binding to extend an earlier observed checkpoint's append-head.
2. The service publishes, at the same append position within the same append scope, two append-head references that are not identical.
3. A published checkpoint's append position is less than or equal to a previously published checkpoint's append position while presenting an incompatible append-head.

Conditions (2) and (3) are specific cases of equivocation and MUST additionally be handled under S9.

### 6.3 Monitor Obligations on Detecting Consistency Failure

On detecting a consistency failure, a monitor:

- MUST preserve the evidence (the two or more checkpoints, their proof material, and the observation metadata) in a form sufficient for independent replay,
- MUST emit an alert to operators or relying parties within its declared governance scope,
- SHOULD publish structured evidence of the failure in the format of S9.3 when the failure constitutes equivocation,
- MAY, and under a declared enforcement profile SHOULD, suspend further reliance on append-head references from the affected Canonical Append Service pending investigation,
- MUST NOT rewrite or annotate canonical records as a consequence of the failure; canonical records remain as appended under Trellis Core §7.

Whether a monitor MUST halt reliance is an enforcement-posture decision and is governed by S10.

---

## 7. Witness Attestation Semantics

**Requirement class: Binding or sidecar choice.**

### 7.1 What a Witness Attests To

A witness attestation MAY attest to one or more of the following, and MUST declare which:

- **Observation of an append-head.** The witness observed the declared append-head reference at the declared append position within the declared append scope at the declared observation time.
- **Append-growth consistency.** The witness verified, under the active binding's consistency proof model, that a later observed append-head extends an earlier observed append-head within the same declared append scope.
- **Inclusion proof validity.** For a specific canonical record within the declared append scope, the witness verified an inclusion proof against an observed append-head.
- **Temporal anchoring.** The witness observed an anchor record (S8) binding an append-head into an external log at a declared external time.

### 7.2 What a Witness Does Not Attest To

A witness attestation:

- MUST NOT be presented as a canonical append attestation,
- MUST NOT be treated as evidence of fact admission, canonical record correctness, substantive fact truth, workflow policy conformance, or authorization decisions,
- MUST NOT attest to properties the witness did not observe,
- MUST NOT imply that the Canonical Append Service behaved correctly outside the declared scope, cadence, and observation method of the attestation.

### 7.3 Witness-Versus-Canonical Distinction

The Canonical Append Service issues canonical append attestations (Trellis Core §7.5): service-issued proofs that a canonical record was admitted into canonical order under the active append model. A witness issues witness attestations: independent observer statements about published checkpoint material and, optionally, about consistency, inclusion, or anchoring. The two are distinct object classes. A relying party MUST NOT substitute one for the other, and an export or disclosure artifact MUST preserve the distinction.

### 7.4 Minimum Witness Attestation Content

A witness attestation MUST include at minimum:

1. Witness identifier and binding-declared public key or equivalent authenticator.
2. Declared witness sub-role (S4) under which the attestation is issued.
3. Append scope identifier.
4. Observed append position and append-head reference, and the attested property class from S7.1.
5. Observation time (witness-declared) and, where applicable, the checkpoint time declared by the Canonical Append Service.
6. Reference to, or embedding of, the observed checkpoint material and any consistency, inclusion, or anchor proof material relied upon.
7. The witness's own signature or equivalent authentication over the attestation content.

A witness attestation that omits any minimum content field is not a conforming witness attestation and MUST NOT be relied upon as such.

### 7.5 Relying Party Verification

A relying party consuming a witness attestation MUST verify:

- the witness's authenticator and signature,
- that the declared append-head reference matches the append-head reference carried by any canonical append attestation the relying party is cross-checking,
- any consistency, inclusion, or anchor proof material embedded or referenced by the attestation, under the active binding,
- that the attested property class (S7.1) actually corresponds to the assurance the relying party is drawing.

A relying party MUST NOT draw assurance beyond the attested property class. In particular, an attestation attesting only to observation of an append-head does not attest to append-growth consistency, and vice versa.

### 7.6 Assurance-Level Claims in Witness Attestations

Where a witness attestation carries, or is consumed as input to, an assurance-level claim (for example, a profile that declares quorum-witnessed checkpoints as a precondition for a given assurance posture), the claim's taxonomy and semantics are governed by the Witnessed Observation System assurance taxonomy ([WOS Assurance §2]). The assurance taxonomy's ordered levels (L1--L4) inform, but do not directly prescribe, witness-count thresholds; this companion defines the structural count independent of the level semantics. This companion does not redefine assurance levels; it defines only the observational material (checkpoint, consistency, inclusion, anchor) from which an assurance-level claim may be composed. A relying party consuming an assurance-level claim derived from witness attestations MUST evaluate the claim under [WOS Assurance §2], not under this companion alone.

---

## 8. External Anchoring Semantics

**Requirement class: Binding or sidecar choice.**

### 8.1 Meaning of Anchoring

Anchoring binds an append-head reference (or a digest of one) into an external log that is outside the Canonical Append Service's administrative scope — for example, a public transparency log, a blockchain, or a timestamping authority. The external log provides independent evidence that the append-head existed, in the anchored form, at or before the external log's declared time.

Anchoring strengthens detection of equivocation and of retrospective rewriting of canonical order: once anchored, an append-head cannot be replaced with an incompatible append-head at the same declared append position without the substitution becoming evident against the anchor.

Anchoring is not itself a canonical append attestation, and the external log is not itself a Canonical Append Service.

### 8.2 Anchor Record Minimum Content

An anchor record MUST contain at minimum:

1. Append scope identifier.
2. Anchored append position and append-head reference (or declared digest).
3. External log identifier and the record locator within that external log (for example, external index, transaction identifier, or timestamp certificate serial).
4. External log's declared time or inclusion indicator.
5. Anchor-producer identifier (the external anchoring witness under S4.3).
6. Any external-log proof material required by the external log's own inclusion model.

### 8.3 Relying Party Verification of Anchor Proofs

A relying party consuming an anchor proof MUST:

- verify the external log's own inclusion or timestamp proof under the external log's specification,
- verify that the anchored append-head reference matches the append-head reference carried by any canonical append attestation or witness attestation the relying party is cross-checking,
- verify that the declared append scope matches the append scope under consideration,
- treat the external log's declared time as a bound on the "no later than" existence of the append-head, not as a canonical order time,
- evaluate the independence posture of the external log relative to the Canonical Append Service when weighing assurance.

A relying party MUST NOT treat an anchor record as evidence of canonical admission, canonical record substantive correctness, or authorization beyond the scope of S8.1.

---

## 9. Equivocation: Definition and Evidence Format

**Requirement class: Companion requirement.**

### 9.1 Definition

Equivocation is the publication, by a Canonical Append Service within a single declared append scope, of two or more canonical histories that cannot be reconciled under append-only order. Concretely, a Canonical Append Service equivocates when it publishes any of the following:

1. Two or more distinct append-head references at the same declared append position within the same append scope, to the same or different observers.
2. A later append-head whose implied canonical order contradicts an earlier published append-head within the same append scope — for example, by omitting, reordering, or replacing canonical records previously committed.
3. Divergent checkpoint streams presented to different observers of the same append scope, where no consistency proof can reconcile them.

Publication behavior constituting equivocation is not limited to explicit contradiction in a single response. Presenting different append-head references to different monitors for the same append position within the same declared append scope is equivocation, regardless of whether any single monitor sees both heads directly.

### 9.2 Relationship to Core Canonical Order

Under Trellis Core §7.4, canonical order has a declared scope, and the canonical append-attestation stream is the single ordered source of truth for canonical record inclusion and sequence. Equivocation is a violation of the single-ordered-source requirement at the publication layer. Equivocation does not retroactively "split" canonical truth; rather, it is evidence that the Canonical Append Service has published statements that cannot all be canonical. At most one of the published append-heads can be consistent with actual durable append state; any others are service misbehavior.

### 9.3 Equivocation Evidence Format

Evidence of equivocation MUST be structured so that a relying party outside the active monitoring deployment can verify it independently. An equivocation evidence record MUST contain at minimum:

1. Append scope identifier.
2. Declared append position at which equivocation is asserted.
3. Two or more observed append-head references that are incompatible at that position, each accompanied by:
   - the observed checkpoint material carrying the append-head,
   - the Canonical Append Service's declared authentication of that checkpoint, where the binding requires it,
   - observation metadata (observer identifier, observation time, query channel or vantage point).
4. A statement of the incompatibility class — equal-position divergence, rewriting of prior canonical order, or unreconcilable checkpoint streams — referencing the subclause of S9.1.
5. Any corroborating witness attestations (S7) or anchor records (S8) bearing on the conflicting append-heads.
6. The publishing detector's identifier and signature over the evidence record.

An equivocation evidence record:

- MUST preserve the original service-declared checkpoint authentication for each included head so that a relying party can verify the service actually issued each head,
- MUST NOT omit or alter the append-heads it alleges to be incompatible,
- MUST be publishable to parties outside the monitor's administrative scope.

---

## 10. Detection Versus Enforcement

**Requirement class: Companion requirement.**

### 10.1 Distinction

Detection and enforcement are distinct postures. Detection establishes that an anomaly has been observed; enforcement prescribes a consequence. This companion defines detection obligations. Enforcement obligations are a binding, profile, or deployment choice.

Detection MUST NOT imply enforcement. A monitor detecting a consistency failure or equivocation does not thereby acquire authority to rewrite canonical records, to invalidate previously issued canonical append attestations, or to bind the behavior of other monitors, witnesses, or relying parties.

### 10.2 Monitor Obligations on Detection

On detecting a consistency failure (S6.2) or equivocation (S9.1), a monitor:

- MUST preserve evidence sufficient for independent replay,
- MUST alert within its declared governance scope,
- MUST, for equivocation, publish equivocation evidence in the format of S9.3 to parties entitled to receive it under the active profile or binding.

The following are RECOMMENDED but not required unless a declared profile or binding makes them MUST:

- publishing consistency failure evidence beyond the monitor's governance scope,
- halting reliance on the affected Canonical Append Service,
- refusing to issue further export artifacts depending on the affected append scope.

### 10.3 Enforcement as a Profile or Binding Choice

A profile or binding MAY elevate any of the RECOMMENDED behaviors in S10.2 to MUST. A profile or binding MAY additionally require:

- minimum independent-witness counts before canonical append attestations are usable for export (see S13.4),
- quorum-attested checkpoints before consistency failures trigger service-level halt,
- anchor-record cadence minimums,
- escalation paths to external authorities.

Enforcement rules declared by a profile or binding MUST remain consistent with Trellis Core. They MUST NOT redefine canonical truth or canonical order, and they MUST NOT authorize monitors or witnesses to mutate canonical records.

---

## 11. Testability Hooks

**Requirement class: Companion requirement.**

- Publication interfaces MUST expose deterministic fixtures for append-growth and checkpoint-consistency checks.
- Equivocation detection MUST be testable via replayable monitor scenarios that present synthetic divergent checkpoint streams.
- Deterministic test fixtures SHOULD be published at a well-known repository path (for example, `trellis/test-vectors/monitoring/`) with a documented format sufficient to reproduce consistency proofs, inclusion proofs, and equivocation evidence records end-to-end.

---

## 12. Conformance

**Requirement class: Companion requirement.**

This companion defines the following conformance roles. An implementation MAY claim more than one role and MUST satisfy the obligations of each claimed role.

1. **Append Service (monitoring).** Publishes the checkpoint publication interface defined in S5 and preserves sufficient proof material for consistency proofs between any two published checkpoints within the same declared append scope. MUST support deterministic test fixtures under S11.
2. **Passive Consistency Monitor.** Implements S4.1 and S6.
3. **Active Equivocation Detector.** Implements S4.2, S6, and S9.
4. **External Anchoring Witness.** Implements S4.3 and S8.
5. **Audit Witness.** Implements S4.4 and S7.

A Canonical Append Service that claims conformance to this companion MUST declare which monitoring and witnessing roles it supports on the client side (what it exposes) and MUST NOT misrepresent independent observer roles as provided by the service itself.

---

## 13. Security Considerations

**Requirement class: Companion requirement** (S13.4), otherwise informational unless elevated by a profile or binding.

### 13.1 Witness Compromise

A compromised witness may issue false consistency attestations or false observation attestations, or may fail to report observed anomalies. Relying parties drawing assurance from a single witness inherit that witness's risk. Profiles or bindings requiring strong anti-equivocation guarantees SHOULD require multiple independent witnesses (see S13.4). A compromised anchor producer presents analogous risk with respect to anchor records.

### 13.2 Witness Collusion and Quorum Suppression

Witness collusion — coordinated silence or coordinated false attestation among witnesses — can suppress equivocation evidence. Collusion below the quorum threshold declared by an active profile may go undetected by relying parties that only consult quorum-level evidence. Witness collusion is undetectable below the quorum threshold by design: a relying party querying only quorum-summary evidence cannot distinguish genuine agreement from coordinated suppression. Mitigations include independence-posture disclosure (S4.4), diversity of administrative scope, external anchoring (S8), and cross-channel active equivocation detection (S4.2).

### 13.3 Minimum Witness Attestation Content

A witness attestation that omits any of the fields required by S7.4 is not a conforming witness attestation. Relying parties MUST reject such attestations as insufficient. Implementations MUST NOT silently accept degraded witness attestations (for example, attestations without observation time or without attested property class) as equivalent to conforming ones.

### 13.4 Minimum Independent-Witness Counts

**Requirement class: Profile or binding constraint.**

Profiles or bindings requiring anti-equivocation guarantees MUST define a minimum independent-witness count sufficient for their threat model. "Independent" means administratively separate from the Canonical Append Service and from other witnesses in the set. A profile requiring anti-equivocation guarantees MUST NOT rely on a single witness and MUST NOT treat multiple witnesses sharing a common administrative scope as independent for the purposes of the minimum count. Where minimum independent-witness counts are set to meet a declared assurance posture, the posture itself is governed by [WOS Assurance §2]; this companion defines only the structural count, not the assurance-level thresholds the count is tuned to satisfy.

### 13.5 Relying Party Verification Obligations

A relying party consuming a witness attestation MUST perform the verification listed in S7.5. A relying party consuming an anchor record MUST perform the verification listed in S8.3. A relying party MUST NOT treat witness or anchor material as canonical append attestation (S7.3).

### 13.6 Other Considerations

Implementers should additionally consider:

- publication-rate abuse: checkpoint interfaces SHOULD be rate-limited to prevent denial-of-service against the Canonical Append Service,
- observer authentication: deployments SHOULD authenticate monitors and witnesses before granting access to proof material where exposure of such material is itself sensitive,
- service equivocation detection latency: the window between publication and detection is a function of monitor cadence and witness diversity,
- evidence forgery: a false equivocation evidence record attempting to implicate a compliant service must fail S9.3's requirement that each included head carry the service's own authentication,
- key-compromise of the Canonical Append Service authenticator, which can simulate service equivocation or suppress genuine equivocation evidence, and is out of scope for observation-layer controls alone.

---

## 14. Privacy Considerations

**Requirement class: Companion requirement.**

### 14.1 What Witnesses Observe

Witnesses and monitors observe:

- append-head references (which commit to, but do not reveal, protected payload content),
- checkpoint cadence and append positions,
- query timing and query volume,
- anchor record timing and external log positions.

Even in a trust profile where protected payloads are reader-held (see `trellis/specs/trust/trust-profiles.md` §9.3 Access Categories), append-head and cadence observation can reveal information about append volume, activity patterns, and governance rhythm.

### 14.2 What Witnesses MUST NOT Observe

Witnesses and monitors MUST NOT be permitted to observe:

- protected payload plaintext,
- reader-held access material,
- author-originated fact content beyond what the active Trust Profile declares to be verifier-visible,
- authorization decisions, workflow runtime state, or other derived material except as necessary to validate the append-growth and equivocation-detection obligations of the claimed monitoring or witnessing role.

### 14.3 Minimization

The checkpoint publication interface SHOULD expose only the material required for append-growth and equivocation detection. Implementations SHOULD NOT include payload-content hints, personally identifying metadata, or workflow state in checkpoint publications. Where per-record inclusion proofs are exposed (S5.1), the binding SHOULD permit per-record proof material to be fetched only by authorized verifiers rather than distributed to every monitor by default.

---

## 15. Cross-References

This companion is intended to be read alongside:

- **Trellis Core** ([`../core/trellis-core.md`](../core/trellis-core.md)) — constitutional definitions of canonical truth, canonical order, canonical records, and canonical append attestations. Anchoring and witness semantics in this document are subordinate to Trellis Core §7.
- **Shared Ledger Binding** ([`../core/shared-ledger-binding.md`](../core/shared-ledger-binding.md)) — concrete proof model (inclusion and consistency proofs, append-head binding) invoked by S6, S7, and S8 of this companion.
- **Export Verification Package** ([`../export/export-verification-package.md`](../export/export-verification-package.md)) — consistency proof and append-head binding material carried in export packages for independent verification.
- **Assurance Traceability** ([`../assurance/assurance-traceability.md`](../assurance/assurance-traceability.md)) — how monitor, witness, and anchor material contributes to assurance claims traceable across canonical records, exports, and disclosure artifacts.

Upstream normative references:

- **[WOS Kernel]** — upstream substrate defining observer, custodian, and witness roles at the kernel layer. Trellis is a ledger-substrate specialization of the WOS substrate; monitor, witness, and anchor sub-roles in this companion are ledger-level specializations layered over the kernel's substrate-generic observer and custodian semantics. See `work-spec/specs/kernel/spec.md`.
- **[WOS Assurance §2]** — upstream assurance-level taxonomy. Assurance-level claims carried by or composed from witness attestations (S7.6) and independent-witness counts tuned to assurance thresholds (S13.4) are governed by this upstream taxonomy. See `work-spec/specs/assurance/assurance.md` §2.

---

## Current Scope Constraint

This companion intentionally defines seams only. Concrete witness network topologies, quorum election mechanisms, anchor-chain selection, consensus protocols, and deployment architectures are out of scope and belong in bindings, profiles, or implementation specifications.
