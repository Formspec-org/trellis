---
title: Trellis Companion — Key Lifecycle Operating Model
version: 0.1.0-draft.1
date: 2026-04-13
status: draft
---

# Trellis Companion — Key Lifecycle Operating Model v0.1

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

The Key Lifecycle Operating Model defines key lifecycle as first-class platform behavior — not implementation detail. It specifies key classes, lifecycle states and transitions, rotation and versioning rules, grace periods, recovery, destruction, crypto-shredding, and historical verification across key evolution. This companion adds key-management semantics to the Trellis trust layer defined in Trust Profiles (S3–S6). It does not define Formspec or WOS semantics.

## Purpose

Define key lifecycle as first-class platform behavior, not implementation detail.

## Normative Focus

1. Key classes and scope boundaries.
2. Lifecycle states and allowed transitions.
3. Rotation and versioning rules.
4. Grace periods for offline/intermittent clients.
5. Recovery and re-establishment procedures.
6. Destruction / crypto-shredding semantics.
7. Historical verification across key evolution.

## Key classes (draft)

1. Tenant root / policy keys
2. Scope or ledger keys
3. Subject/record encryption keys
4. Signing/attestation keys
5. Recovery-only keys (if supported)

## Lifecycle states (draft)

| State | Meaning | Allowed transitions (draft) |
|---|---|---|
| `provisioning` | Key material is being established | `active`, `destroyed` |
| `active` | Current signing/decryption use | `rotating`, `suspended`, `destroyed` |
| `rotating` | Dual-validity/grace handling in progress | `active`, `retired`, `destroyed` |
| `retired` | No new encrypt/sign operations; verify/decrypt history as permitted | `destroyed` |
| `suspended` | Temporarily disabled by policy/incident response | `active`, `destroyed` |
| `destroyed` | Cryptographic use permanently disallowed | _(terminal)_ |

## Grace-period rule (draft)

- Rotations affecting intermittently connected clients MUST define a grace window.
- During grace windows, verification of historical signatures and controlled decryptability MUST remain predictable and declared.
- After grace expiry, stale-key writes MUST be rejected with auditable errors.

## Required Completeness Rule

Crypto-shredding is not complete unless plaintext-derived projections and caches are purged according to declared cascade policy. The normative definition of purge-cascade semantics and projection rebuild requirements lives in the Projection and Runtime Discipline companion (S4). This companion owns the evidence-artifact requirements for key-destruction and purge-cascade completion; it defers to Projection S4 for what must be purged and how rebuild correctness is verified.

## Recovery and destruction evidence requirements (draft)

1. Recovery operations MUST emit auditable events with actor, scope, and policy authority.
2. Destruction operations MUST emit auditable events with destroyed key references and effective time.
3. Purge-cascade completion MUST produce verifiable evidence artifacts tied to canonical checkpoint state.

## Conformance

This companion defines the following conformance roles:

1. **Key Lifecycle Manager** — manages key provisioning, rotation, and destruction. MUST comply with lifecycle state transitions, grace-period rules, and destruction evidence requirements.
2. **Key Lifecycle Auditor** — verifies key lifecycle compliance against declared policies. MUST verify destruction evidence, recovery audit trails, and purge-cascade completion artifacts.

## Security and Privacy Considerations

- Key destruction and crypto-shredding are irreversible; recovery from destruction requires separate recovery-key infrastructure that MUST NOT reuse destroyed key material.
- Grace-period windows create intervals where both old and new key material are valid; deployments MUST declare and bound these intervals in trust-profile declarations (Trust Profiles S3).
- Purge-cascade completion evidence MUST NOT reveal plaintext content of purged projections (Projection S4).
- Key lifecycle events are canonical facts; they MUST NOT be redacted from audit trails even after key destruction (Trellis Core S5.2 invariant 1).

## Migrated requirements from `unified_ledger_core.md` (Section 16.5)

1. Cryptographic inaccessibility claims MUST include scope, authority, and effective-time semantics.
2. Key-destruction claims MUST be distinguishable from payload-redaction or disclosure filtering events.
3. Historical verification across key evolution MUST remain possible where declared by policy.
