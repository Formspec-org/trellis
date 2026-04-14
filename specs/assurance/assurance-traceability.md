---
title: Trellis Companion — Assurance Traceability
version: 0.1.0-draft.1
date: 2026-04-13
status: draft
---

# Trellis Companion — Assurance Traceability v0.1

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

The Assurance Traceability companion maps each core invariant to concrete assurance methods so that assurance is architectural, not appendix-only. It specifies a traceability matrix, minimum CI expectations, evidence retention policy, and role-based applicability. This companion adds assurance semantics to the Trellis verification layer defined in Core (S5, S8). It does not define Formspec or WOS semantics.

## Purpose

Map each core invariant to concrete assurance methods so assurance remains architectural, not appendix-only.

## Invariant scope

"Every normative invariant" in this companion refers to the following defined set:

1. Trellis Core S5.2 invariants 1–6 (Append-only, No Second Truth, One Order, One Hash Construction, Verification Independence, Append Idempotency).
2. Shared Ledger Binding canonization invariants (S5 rules 1–4).
3. Key Lifecycle state-transition invariants (Key Lifecycle Operating Model S3 allowed transitions).
4. Trust Profile honesty invariant (Trust Profiles S5).

Additional invariants added by future companions MUST be registered in this list before assurance methods are required for them.

## Traceability matrix (draft)

| Core invariant | Normative definition | Primary methods | Secondary methods | Evidence artifacts |
|---|---|---|---|---|
| Append-only canonical history | Trellis Core S5.2 invariant 1 | TLA+ model of append transitions | Property-based append tests | Model files, passing model checks, test logs |
| One canonical order per governed scope | Trellis Core S5.2 invariant 3 | TLA+ scope/order invariants | Adversarial replay tests | Scope-partition proofs, replay test reports |
| One canonical event hash construction | Trellis Core S5.2 invariant 4 | Shared test vectors (native + WASM) | Parser/verifier fuzzing | Vector fixtures, fuzz corpus/crash reports |
| No second canonical truth | Trellis Core S5.2 invariant 2 | Alloy constraints on canonical/derived separation | Rebuild-from-canonical drills | Alloy model checks, rebuild drill logs |
| Append idempotency | Trellis Core S5.2 invariant 6 | Property-based idempotency tests | Failure/retry chaos tests | Deterministic idempotency test suite outputs |
| Trust profile honesty | Trust Profiles S5 | Profile conformance tests | Metadata-budget disclosure audits | Profile declarations + audit records |
| Key lifecycle correctness | Key Lifecycle S3 | State-transition property tests | Rotation/recovery drills | Transition test reports, recovery drill artifacts |
| Crypto-shredding completeness | Key Lifecycle S7, Projection S4 | Purge-cascade integration tests | Snapshot/cache residue scans | Purge verification reports |
| Export offline verifiability | Export Verification Package S4 | Offline verifier cross-implementation vectors | Package corruption fuzz tests | Verifier outputs, corruption-detection logs |

## Minimum CI expectations (draft)

1. Every normative invariant MUST map to at least one automated check.
2. Hash/serialization vectors MUST run in native and WASM paths.
3. Recovery and destruction drills MUST run on a recurring schedule and produce retained evidence.
4. Fuzzing outcomes MUST feed parser/verifier hardening backlogs.

### Role-based applicability

| Requirement | Verifier implementations | Append Service | Studio / tooling |
|---|---|---|---|
| Invariant-to-check mapping | MUST | MUST | SHOULD |
| Native + WASM test vectors | MUST | SHOULD | MAY |
| Recovery/destruction drills | SHOULD | MUST | MAY |
| Fuzzing backlogs | MUST | SHOULD | MAY |

## Evidence retention policy (draft)

- Assurance artifacts SHOULD be retained for at least one full major-version support window. The definition of "major-version support window" is an engineering decision, not a legal guarantee; jurisdictions MAY impose longer retention requirements.
- Artifacts MUST include build/version identifiers and execution timestamps.
- Failed assurance runs MUST be retained with remediation linkage.

## Conformance

This companion defines the following conformance roles:

1. **Assurance Producer** — implements automated checks, drills, and fuzzing for registered invariants. MUST map every registered invariant to at least one primary assurance method and produce evidence artifacts.
2. **Assurance Auditor** — reviews evidence artifacts, remediation linkage, and retention compliance. MUST verify that evidence artifacts include build/version identifiers and execution timestamps.

## Security and Privacy Considerations

- Assurance artifacts MAY contain sensitive implementation details; evidence retention policy MUST account for access control.
- Failed assurance runs MUST NOT be suppressed or deleted; they MUST be retained with remediation linkage even when the underlying issue has been resolved.
- Fuzzing corpora and crash reports SHOULD be treated as sensitive artifacts; publication of fuzzing inputs that reveal parser behavior SHOULD be governed by the deployment's trust profile (Trust Profiles S3).
