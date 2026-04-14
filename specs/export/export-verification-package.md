---
title: Trellis Companion — Export Verification Package
version: 0.1.0-draft.1
date: 2026-04-13
status: draft
---

# Trellis Companion — Export Verification Package v0.1

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

The Export Verification Package companion defines requirements for offline-verifiable export packages that carry canonical integrity claims. It specifies required package members, payload readability declarations, trust-profile carriage, verification mode, and cross-implementation verification. This companion adds export semantics to the Trellis verification layer defined in Trellis Core (S8). It does not define Formspec or WOS semantics.

## Purpose

Define offline-verifiable export package requirements for canonical integrity claims.

## Normative Focus

1. Required package members
   - canonical records in scope,
   - append attestations/checkpoint material,
   - verification manifest and schema/version references.

2. Payload readability declaration
   - package MUST declare what payload content is readable, encrypted, or intentionally omitted.

3. Trust-profile carriage
   - package SHOULD include the active trust-profile declaration and metadata-budget reference.

4. Verification mode
   - verifier MUST be able to validate integrity and append claims offline.
   - ingest- and export-time verification posture MUST align with declarations in the Trust Profiles companion when packages assert claim classes tied to verification strength.

5. Optional external anchoring seam
   - package MAY include anchoring references (e.g., OpenTimestamps) without making anchoring mandatory.

## Verification manifest minimum fields (draft)

1. package format/version identifier,
2. canonical checkpoint reference,
3. included claim classes,
4. hash/canonicalization algorithm identifiers,
5. trust-profile reference (if declared),
6. disclosure/readability declarations.

## Cross-implementation verification requirement (draft)

At least two independent verifier implementations SHOULD validate the same package fixture set and produce equivalent claim outcomes.

## Provenance distinction requirement (draft)

Export packages MUST preserve distinction among:

1. canonical records,
2. canonical append attestations,
3. derived release/disclosure artifacts.

## Conformance

This companion defines the following conformance roles:

1. **Package Producer** — assembles export verification packages. MUST include all required package members, declare payload readability, and preserve provenance distinctions.
2. **Package Verifier** — validates export packages offline. MUST verify integrity and append claims without requiring runtime access to the canonical append service.

## Security and Privacy Considerations

- Export packages containing cryptographic key material or trust-profile declarations MUST be protected in transit and at rest.
- Payload readability declarations MUST NOT reveal the existence of redacted content beyond what is explicitly declared.
- Cross-implementation verification SHOULD be performed in isolated environments to prevent side-channel leakage between verifier implementations.
- Packages MUST NOT include cryptographic secrets for verification; verification relies on public key material and canonical attestations (Trellis Core S8).

## Migrated requirements from `unified_ledger_core.md` (Section 12)

These requirements are now normatively defined in Trellis Core S8 (Verification Requirements) and this companion S3 (Verification mode). The migrated text below is retained for traceability only; canonical obligations live in the cited sections.

1. Export packages MUST include sufficient material to verify declared claim classes (Trellis Core S8, this companion S3).
2. Export verification MUST remain independent of runtime-only derived artifacts (Trellis Core S8, Projection S4).
3. Exports MUST preserve provenance distinction among authored facts, canonical records, append attestations, and disclosure artifacts (Trellis Core S5.1, Disclosure Manifest S3).
