---
title: Trellis Companion — Shared Ledger Binding
version: 0.1.0-draft.1
date: 2026-04-13
status: draft
---

# Trellis Companion — Shared Ledger Binding v0.1

**Version:** 0.1.0-draft.1
**Date:** 2026-04-13
**Editors:** Formspec Working Group
**Companion to:** Trellis Core v0.1

---

## Status of This Document

This document is a **draft specification**. It is a companion to the Trellis Core specification and does not modify Formspec or WOS processing semantics. Implementors are encouraged to experiment with this specification and provide feedback, but MUST NOT treat it as stable for production use until a 1.0.0 release is published.

## Conventions and Terminology

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in BCP 14 [RFC 2119] [RFC 8174] when, and only when, they appear in ALL CAPITALS, as shown here.

JSON syntax and data types are as defined in [RFC 8259]. URI syntax is as defined in [RFC 3986].

## Abstract

The Shared Ledger Binding defines how Formspec-family, WOS-family, trust-family, and release-family facts are admitted into a single Trellis canonical substrate. It specifies family identification, minimum required fields, schema/version compatibility, canonization rejection, and cross-family reference rules. This companion does not define Formspec or WOS semantics — it governs admission, order, attestation, and verification shape for bound records only.

## Purpose

Bind Formspec-family, WOS-family, trust/access-family, and release-family facts to one Trellis canonical substrate without redefining source-repo semantics.

## Substrate binding

This binding MUST admit Formspec-family and WOS-family facts (and related trust- and release-family facts declared in this binding) into **one** governed canonical substrate per governed scope, using the shared append, hash, and verification rules established in Trellis Core (S4–S7).

This binding MUST NOT reinterpret Formspec or WOS semantic authority: family payloads remain authoritative in their source repositories; Trellis only governs canonical admission, order, attestation, and verification shape for bound records.

### Delegation requirement

When binding behavior depends on Formspec Definition or Response semantics — including field values, relevance, validation, calculation, or version pinning — processing MUST be delegated to a Formspec-conformant processor (Core S1.4, Core S6.4). This binding defines admission, order, attestation, and verification shape for bound records; it does not specify bind, FEL, or validation rules.

Trellis-bound Formspec processors MUST implement at least Formspec Core conformance (Core S2). Whether Theme or Component tiers are required depends on the Trellis conformance class and the bound family's declared requirements.

## Normative Focus

1. Family IDs and minimum required fields by family.
2. Schema/version references and upgrade rules.
3. Canonization eligibility and rejection reasons.
4. Cross-family reference rules (e.g., authored fact to governance fact links).
5. Binding conformance matrix for producers, appenders, verifiers.

## Family binding matrix (draft scaffold)

| Family | Authority | Required minimum fields | Notes |
|---|---|---|---|
| Formspec-authored facts | Formspec (Core S4–S7) | `family_id`, `schema_ref`, `authored_at`, `author_ref`, `payload_ref` | Trellis binds, does not redefine authored semantics (Core S1.4). |
| WOS governance/workflow facts | WOS (Kernel S3–S8) | `family_id`, `schema_ref`, `governance_scope`, `actor_ref`, `payload_ref` | Runtime meaning remains in WOS (Runtime S4–S12). |
| Trust/access facts | Trellis companions (Trust Profiles S3) | `family_id`, `profile_ref`, `policy_ref`, `effective_at`, `subject_ref` | Must align with trust-profile declarations. |
| Release/export/disclosure facts | Trellis companions (Export Verification S3) | `family_id`, `package_ref`, `audience_ref`, `readability_ref`, `version_ref` | Used for verifier/export semantics. |

Property-level type and cardinality for each family's minimum fields will be defined in a `$defs` section when the JSON Schema for this binding is authored (Phase 4). Until then, the field names above are normative and their presence is required; their types are string unless otherwise specified by the owning family's schema.

## Canonization rules (draft)

1. Binding MUST reject records missing required minimum fields for the declared family.
2. Binding MUST carry stable schema/version references for verification portability.
3. Binding MUST preserve canonical hash construction from Trellis core without family-specific overrides.
4. Binding MUST NOT permit two families to create competing canonical order claims for the same governed scope.

## Canonical receipt immutability (draft)

Binding-defined fields on the canonical append attestation (or equivalent canonical receipt) that record ingest-time verification posture, payload-readiness class, or other admission-time commitments are part of canonical truth for that append position.

Such fields MUST NOT be rewritten, upgraded, or downgraded in place after the append attestation is issued. Changes to verification or readability posture MUST be represented as new canonical facts or attestations according to binding rules, not by mutating prior receipt fields.

## Schema/version compatibility policy (draft)

1. Backward-compatible additive fields MAY be accepted within a declared major version. Breaking versus additive classification follows Changelog S4 (Impact Classification).
2. Breaking semantic changes MUST require a new major schema reference (Changelog S4.2).
3. Verifiers MUST reject unknown major versions unless an explicit compatibility adapter is declared (Changelog S4.3).

## Canonization rejection codes

| Code | Meaning | Typical remediation | Safe to retry? |
|------|---------|---------------------|----------------|
| `missing_required_field` | Record omits a field required for the declared family | Add the missing field and resubmit | Yes |
| `invalid_schema_ref` | Schema reference does not resolve or does not match payload | Correct the schema reference or update the payload | Yes |
| `unsupported_major_version` | Major schema version is not recognized by this append service | Upgrade to a supported version or declare a compatibility adapter | No (requires adapter) |
| `hash_construction_mismatch` | Canonical hash construction does not match the registered construction ID (Trellis Core S7) | Use the registered hash construction for this governed scope | No (construction is fixed per scope) |
| `scope_order_conflict` | Record would create competing canonical order for the same governed scope (Trellis Core S5.2 invariant 3) | Verify scope assignment; resubmit to correct scope | Yes (if scope was wrong) |

## Cross-family reference rules (draft)

1. Allowed edges: authored facts (Formspec) MAY reference governance facts (WOS) via `payload_ref`; governance facts MAY reference authored facts via the same mechanism.
2. Forbidden cycles: a canonical record MUST NOT transitively reference itself through cross-family links.
3. `payload_ref` shapes MUST conform to the declaring family's schema; cross-family references MUST use the target family's canonical record identifier format.
4. Failure to resolve a cross-family reference during admission MUST produce a `missing_required_field` or `invalid_schema_ref` rejection code; it MUST NOT silently omit the reference.

## Formspec fact admission path

1. Ingest a Formspec Response or Definition reference submitted as a Formspec-authored fact.
2. Validate the reference against the pinned Definition version (Core S6.4, VP-01). If the reference cites a Definition version not recognized by the Formspec-conformant processor, reject with `invalid_schema_ref`.
3. Delegate Definition and Response validation to a Formspec-conformant processor (Core S1.4). If validation fails, reject with `invalid_schema_ref`.
4. Map the validated reference to a canonical record per this binding's family matrix fields.
5. Apply core admission rules (Trellis Core S6.1) and, if admissible, append to canonical order.

## WOS fact admission path

1. Ingest a WOS governance/workflow fact submitted as a WOS-family fact.
2. Validate the fact against the declared WOS schema version (Kernel S3, Runtime S3). If the version is not recognized, reject with `unsupported_major_version`.
3. Verify the fact's governance scope and actor reference conform to WOS authority semantics (Kernel S4, Governance S2). Trellis does not evaluate WOS governance rules — it checks that required fields are present and structurally valid.
4. Map the validated fact to a canonical record per this binding's family matrix fields.
5. Apply core admission rules (Trellis Core S6.1) and, if admissible, append to canonical order.

## Deferral Rules

- Formspec remains authoritative for authored/spec/runtime semantics (Core S4–S7).
- WOS remains authoritative for workflow/governance meaning and runtime envelope (Kernel S3–S8, Runtime S4–S12).
- Trellis binding MUST NOT reinterpret Formspec or WOS meaning; it cites by section number.

## Core handoff note

Core admission semantics are defined in Trellis Core S6.

This companion owns schema/version compatibility policy (S6) and machine-testable rejection codes (S7) used during canonical admission.

## Conformance

This companion defines the following conformance roles:

1. **Binding Producer** — produces binding-compliant canonical records for admission. MUST comply with family matrix minimum fields, canonization rules, and cross-family reference rules.
2. **Binding Verifier** — validates binding compliance of canonical records. MUST check family matrix fields, rejection code semantics, and cross-family reference integrity.

## Security and Privacy Considerations

- Cross-family references MUST NOT create cycles that could be exploited to inflate canonical order or create scope-order conflicts (Trellis Core S5.2 invariant 3).
- Rejection codes MUST NOT leak payload content or internal state to unauthenticated callers.
- Schema/version references MUST be validated before admission to prevent injection of unrecognized schema references that could compromise downstream verification.

## Migrated requirements from `unified_ledger_core.md` (Section 15)

1. Domain vocabularies MUST be bound through family IDs and schema references, not by redefining core ontology.
2. Family bindings MUST remain subordinate to core canonical truth/order/hash semantics.
3. Sidecar and binding content MUST NOT create alternate canonical truth or competing canonical order for the same governed scope.
