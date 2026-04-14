---
title: Trellis Companion — Trust Profiles
version: 0.1.0-draft.1
date: 2026-04-13
status: draft
---

# Trellis Companion — Trust Profiles v0.1

**Version:** 0.1.0-draft.1
**Date:** 2026-04-13
**Editors:** Formspec Working Group
**Companion to:** Trellis Core v0.1

---

## Status of This Document

This document is a **draft specification**. It is a companion to the Trellis Core specification and does not modify Formspec or WOS processing semantics. Implementors are encouraged to experiment with this specification and provide feedback, but MUST NOT treat it as stable for production use until a 1.0.0 release is published.

## Conventions and Terminology

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in BCP 14 [RFC 2119] [RFC 8174] when, and only when, they appear in ALL CAPITALS, as shown here.

JSON syntax and data types are as defined in [RFC 8259].

## Abstract

The Trust Profiles companion defines custody, readability, and verification posture declarations for Trellis deployments. It specifies baseline profiles, mandatory profile declarations, metadata budgets, verification posture classes, and trust-honesty rules. This companion adds trust and disclosure semantics to the Trellis canonical substrate defined in Trellis Core (S4–S8). It does not define Formspec or WOS semantics — it governs who can observe what, under what declared posture, with what leakage characteristics.

## Purpose

Define explicit custody/readability postures and trust-honesty declarations.

## Conformance

This companion defines the following conformance roles:

1. **Trust Profile Publisher** — publishes machine-readable profile declarations and metadata budgets.
2. **Trust Profile Verifier** — validates that operational behavior matches declared posture.
3. **Auditor** — independent party claiming Trellis Auditor conformance. Auditors MUST be able to compare declared posture versus observed control-plane behavior. Auditor obligations are scoped to metadata and control-plane observations; auditors MUST NOT be required to access protected payloads.

A conforming implementation MUST satisfy all requirements applicable to each claimed role.

## Baseline Profiles

1. **Reader-held by default.** Payload decryption keys are held by the record subject or their designated reader. Providers MUST NOT hold decryption capability unless explicitly declared. This profile MUST be the default when no other profile is declared.
2. **Provider-readable (explicit, not implied).** Providers MAY decrypt protected payload classes, but only when explicitly declared in the profile and disclosed in the metadata budget. This MUST NOT be the default.
3. **Tenant-operated key plane.** Key management operates within the tenant's administrative boundary. Cross-tenant key access MUST be declared explicitly.

Mutual exclusion: a deployment MUST NOT simultaneously claim reader-held and provider-readable posture for the same payload class without an explicit transition record (see S6).

## Mandatory Profile Declarations

- who can decrypt what,
- visible metadata classes,
- stable-linkage behavior,
- delegated compute behavior,
- recovery paths,
- destruction/disclosure authority.

## Metadata Budget Requirement

Each declared trust profile MUST include a **metadata budget** scoped by canonical fact family. For each family in scope, the profile MUST document at least:

1. **Visible fields** — which canonical or envelope fields are visible to which observer classes under ordinary operation;
2. **Observer classes** — who may observe append metadata, timing, correlation identifiers, or side channels;
3. **Timing and access-pattern leakage** — what timing, frequency, or access-pattern signals the deployment exposes;
4. **Linkage stability** — which identifiers remain stable across sessions, exports, or disclosures;
5. **Delegated-compute effects** — what metadata or plaintext exposure delegated compute introduces relative to nominal posture.

The metadata budget MAY be presented as a table or structured object; it MUST be sufficient for an auditor to compare declared leakage against observed behavior.

## Verification posture declaration (draft)

Where a deployment uses **tiered verification** for canonical records (for example, structural or ciphertext admission before full payload verification completes), the deployment MUST declare **verification posture classes** and which downstream workflow or release classes each posture MAY feed.

### Verification posture class registry

| Posture class | Meaning | Allowed transitions | MUST NOT |
|---|---|---|---|
| `structural_admitted` | Record has passed structural/schema validation only | → `payload_verified` | Escalate to high-stakes outcome class |
| `payload_verified` | Payload integrity and schema conformation confirmed | → (terminal for this record) | Silent escalation from `structural_admitted` |
| `cryptographic_verified` | Full cryptographic verification including signatures/proofs | → (terminal) | Bypass `payload_verified` for high-stakes outcomes |

New posture classes MAY be added only through a companion or registry update that defines the class name, meaning, allowed transitions, and MUST NOT constraints. Posture class names MUST use `snake_case` and MUST be unique within a governed scope.

Implementations MUST NOT attach high-stakes outcomes—including adverse action, selective disclosure issuance, commitment-driven analytics, or profile-defined equivalents—to records that have not reached `payload_verified` posture or higher. Escalating effective verification posture MUST NOT occur silently; it MUST be represented by explicit canonical facts or binding-defined attestations (Shared Ledger Binding S5) when posture affects eligibility for those outcomes.

## Trust honesty rule (draft)

A deployment MUST NOT claim a stronger trust posture than it operationally provides.

At minimum, each profile declaration MUST make explicit:

1. whether provider operators can decrypt protected payload classes,
2. whether delegated compute introduces readable exposure beyond nominal posture,
3. whether recovery paths can reintroduce provider readability,
4. what destruction guarantees exist and what residual metadata remains.

## Profile declaration schema (draft)

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `profile_id` | string (URI) | Yes | Stable unique identifier for this trust profile |
| `decryptor_classes` | array of string | Yes | Observer classes permitted to decrypt protected payloads |
| `metadata_budget_ref` | string (URI) | Yes | Reference to the metadata budget declaration for this profile |
| `delegated_compute_mode` | enum: `none`, `audit_logged`, `full` | Yes | Whether and how delegated compute is permitted |
| `recovery_mode` | enum: `none`, `emergency_only`, `declared_pathways` | Yes | Whether key recovery pathways exist |
| `destruction_semantics` | enum: `crypto_shredding`, `key_destruction`, `none` | Yes | What destruction guarantees the profile provides |
| `disclosure_authority` | array of string | Yes | Actor classes authorized to issue disclosure artifacts |

## Trust profile transitions

Trust profile transitions MUST be append-attributable: each transition MUST be recorded as a canonical fact with the actor, prior profile, new profile, effective time, and policy authority (Trellis Core S6.1, S5.2 invariant 1). The minimal canonical fact shape for a transition event follows the Shared Ledger Binding family matrix for trust/access facts.

## Relationship to export claim classes

Trust profile declarations determine which export manifest claim classes a deployment is authorized to assert. The following table maps profile fields to export claim-class eligibility:

| Profile field | Export claim class affected | Qualification |
|---|---|---|
| `decryptor_classes` | payload-integrity (Disclosure Manifest S4) | Verifiable only if decryptor_classes include the verifier |
| `destruction_semantics` | authorization-history (Disclosure Manifest S4) | Destruction claims require `crypto_shredding` or `key_destruction` semantics |
| `recovery_mode` | disclosure-policy (Disclosure Manifest S4) | Recovery-mode declarations affect disclosure trustworthiness |
| `delegated_compute_mode` | payload-integrity (Disclosure Manifest S4) | Delegated compute exposure must be disclosed in the manifest |

## Conformance audit hooks (draft)

- Deployments MUST publish machine-readable profile declarations.
- Independent auditors claiming Trellis Auditor conformance MUST be able to compare declared posture versus observed control-plane behavior.
- Auditors MAY observe control-plane metadata and timing; auditors MUST NOT be required to access protected payloads.
- Any mismatch between declaration and operation MUST be reported as trust-honesty nonconformance.

## Operational trust disclosure requirements (draft)

- Trust claims MUST be consistent with declared decryptability, delegated compute behavior, recovery behavior, and destruction semantics.
- Metadata-leakage characteristics MUST be declared at profile level and MUST NOT be omitted from trust posture statements.

## Migrated requirements from `unified_ledger_core.md` (Sections 10, 11, 14)

1. Trust profile objects MUST declare disclosure posture and assurance posture explicitly.
2. Trust profile transitions MUST be append-attributable and auditable.
3. Reader-held and delegated-compute semantics MUST remain distinguishable in declarations and audits.
4. Profile-specific export claims MUST NOT overstate payload readability.

## Security and Privacy Considerations

- Trust profile declarations are themselves canonical facts and MUST NOT be altered after append (Trellis Core S5.2 invariant 1).
- Escalating effective verification posture MUST NOT occur silently; posture transitions MUST be represented by explicit canonical facts (this companion S5, Shared Ledger Binding S5).
- Auditor access is bounded: auditors MAY observe metadata and timing but MUST NOT be required to access protected payloads (this companion S6).
- Metadata-budget declarations MUST account for all observable side channels, not just direct data access.
