---
title: Trellis Companion — Disclosure Manifest
version: 0.1.0-draft.1
date: 2026-04-13
status: draft
---

# Trellis Companion — Disclosure Manifest v0.1

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

The Disclosure Manifest companion defines audience-scoped disclosure semantics as first-class release artifacts, separate from canonical append records. It specifies claim-class declarations, selective disclosure without canonical rewrite, payload readability and redaction, and interoperability profiles. This companion adds disclosure semantics to the Trellis export layer defined in Export Verification Package (S3). It does not define Formspec or WOS semantics.

## Purpose

Define audience-scoped disclosure semantics as first-class release artifacts, separate from canonical append records.

## Normative Focus

1. Disclosure scope and audience declaration.
2. Claim-class declaration (authorship, append, payload integrity, authorization history, etc.).
3. Provenance preservation from canonical facts to disclosed claims.
4. Selective disclosure semantics without canonical rewrite.
5. Payload readability and redaction declarations.

## Claim-class taxonomy (draft)

1. authorship claim
2. append/inclusion claim
3. payload-integrity claim
4. authorization-history claim
5. disclosure-policy claim

Disclosure artifacts MUST preserve references back to canonical records and MUST NOT be represented as canonical rewrites.

## Relationship to Export Verification Package

A disclosure manifest MAY be included as a member of an Export Verification Package (Export Verification Package S3). When included, the manifest's claim classes MUST align with the package's declared claim classes (Export Verification Package S3.4). The disclosure manifest governs what is disclosed and to whom; the export package governs offline verifiability of the disclosed and canonical material together.

## Conformance

This companion defines the following conformance roles:

1. **Disclosure Producer** — creates disclosure manifests and audience-scoped release artifacts. MUST preserve provenance from canonical records and MUST NOT represent disclosure artifacts as canonical rewrites.
2. **Disclosure Verifier** — validates that disclosed claims align with canonical records and declared claim classes. MUST verify provenance chain integrity without access to the full canonical history.

## Security and Privacy Considerations

- Selective disclosure MUST NOT leak the existence of undisclosed claims through side channels (e.g., hash inclusion proofs that reveal record counts).
- Disclosure manifests for different audiences MUST be independently producible without requiring access to other audiences' disclosure policies.
- Redaction MUST be irreversible at the disclosure level; disclosed artifacts MUST NOT contain reversible redaction markers that could be exploited to reconstruct redacted content.

## Interop Direction

- SD-JWT / VC profile path is preferred for early disclosure interoperability.
- Advanced privacy mechanisms (e.g., BBS selective disclosure variants) remain later-phase seams.
