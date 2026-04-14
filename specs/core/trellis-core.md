# Trellis Core Specification

## Status of This Document

This is a draft specification in the Trellis spec family.

This document is intended to follow W3C-style specification structure and language discipline. It defines constitutional semantics only and is paired with companion specifications for bindings, trust profiles, key lifecycle, projection/runtime discipline, export/disclosure, monitoring/witnessing, and assurance traceability.

## Abstract

Trellis Core defines the minimum normative semantics for a shared canonical substrate used by Formspec- and WOS-family systems.

This specification normatively defines:

- canonical truth boundaries,
- canonical append-attested order semantics,
- canonical record/hash requirements,
- conformance roles,
- independent verification requirements,
- non-redefinition boundaries relative to Formspec and WOS.

This specification does **not** define domain vocabularies, product UX, deployment architecture, concrete serialization bytes, or proof-wire formats.

## Table of Contents

1. Introduction
2. Conformance
3. Terminology
4. Core Model
5. Canonical Truth and Invariants
6. Canonical Admission and Order
7. Canonical Hash Construction
8. Verification Requirements
9. Cross-Repository Authority Boundaries
10. Security and Privacy Considerations

---

## 1. Introduction

### 1.1 Scope

Trellis Core governs constitutional semantics that MUST remain stable across implementations and companion specifications.

Trellis Core excludes profile and operational detail that belongs in companions.

### 1.2 Design Goal

Prevent multi-source-of-truth drift by enforcing one canonical append-attested substrate while allowing replaceable derived systems.

---

## 2. Conformance

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHOULD**, **SHOULD NOT**, and **MAY** in this document are to be interpreted as described in BCP 14.

### 2.1 Conformance Roles

An implementation MAY claim one or more of the following roles:

1. **Fact Producer**
2. **Canonical Append Service**
3. **Verifier**
4. **Derived Processor**
5. **Export Generator**

A conforming implementation MUST satisfy all requirements applicable to each claimed role.

### 2.2 Role Requirements (Core)

- A **Fact Producer** MUST emit attributable facts without rewriting previously emitted facts.
- A **Canonical Append Service** MUST enforce append-only canonical order and MUST issue append attestations for admitted canonical records.
- A **Verifier** MUST validate canonical integrity without requiring derived runtime state.
- A **Derived Processor** MUST treat canonical records as authoritative input and MUST remain rebuildable from canonical state.
- An **Export Generator** MUST preserve canonical integrity semantics when producing release artifacts.

---

## 3. Terminology

### 3.1 Author-Originated Fact

A fact attributable to an authoring principal prior to canonical append.

### 3.2 Canonical Record

The normalized admitted record package that participates in canonical append-attested order.

### 3.3 Canonical Append Attestation

Integrity-verifiable append evidence for a canonical record at a specific canonical order position.

### 3.4 Derived Artifact

Any runtime or materialized artifact computed from canonical records (e.g., queues, dashboards, indexes, caches, snapshots).

---

## 4. Core Model

### 4.1 Object Classes

Core object classes are:

- author-originated facts,
- canonical records,
- canonical append attestations,
- derived artifacts,
- export/disclosure artifacts.

### 4.2 Ontology Discipline

Companion specifications MAY refine object semantics but MUST NOT redefine canonical truth or canonical order semantics established in this document.

---

## 5. Canonical Truth and Invariants

### 5.1 Canonical Truth Boundary

Canonical truth includes:

- admitted authored/workflow/trust/release facts in canonical record form,
- canonical append attestations,
- canonical checkpoint material.

Canonical truth excludes derived runtime state and all derived artifacts.

### 5.2 Named Core Invariants

1. **Append-only Canonical History**: canonical records MUST NOT be rewritten in-place.
2. **No Second Canonical Truth**: derived artifacts MUST NOT be treated as canonical truth.
3. **One Canonical Order per Governed Scope**: exactly one canonical append-attested order MAY exist per governed scope.
4. **One Canonical Event Hash Construction**: canonical append semantics MUST bind to exactly one canonical hash construction.
5. **Verification Independence**: canonical verification MUST NOT depend on workflow runtime internals.
6. **Append Idempotency**: equivalent admitted canonical inputs MUST NOT create duplicate canonical order positions.

---

## 6. Canonical Admission and Order

### 6.1 Admission

A canonical append service MUST admit only records that satisfy core admissibility constraints.

Schema/version compatibility policy is defined in the Shared Ledger Binding companion.

### 6.2 Order

Canonical append order MUST be monotonically append-only within governed scope.

Implementations MAY partition by scope into multiple ledgers, but MUST NOT allow competing canonical orders for the same governed scope.

### 6.3 Idempotency and Rejection

Replayed equivalent admissions MUST be idempotent.

Rejections MUST be explicit and auditable.

---

## 7. Canonical Hash Construction

Canonical append semantics MUST use exactly one authoritative canonical event hash construction over the sealed canonical record package.

Deterministic canonical serialization is REQUIRED for canonical hashing.

Subordinate hashes MAY exist for specialized purposes (e.g., payload identity, attachments, disclosure artifacts) but MUST NOT redefine canonical append semantics.

---

## 8. Verification Requirements

A conforming verifier MUST be able to validate:

- canonical record integrity,
- append attestation validity,
- inclusion and consistency claims,
- export-package canonical provenance claims,

without requiring derived runtime state.

---

## 9. Cross-Repository Authority Boundaries

- **Formspec** remains authoritative for authored/spec/runtime semantics.
- **WOS** remains authoritative for workflow/governance meaning and runtime envelope.
- **Trellis Core** is authoritative for canonical ledger semantics, append/attestation semantics, and verification boundaries.

This specification MUST NOT be interpreted to redefine Formspec or WOS semantic authority.

---

## 10. Security and Privacy Considerations

Implementations SHOULD minimize metadata leakage consistent with declared trust profiles.

Details of trust profile declarations, key lifecycle controls, and disclosure semantics are specified in companion documents.
