---
title: Trellis Core Specification
version: 0.1.0-draft.1
date: 2026-04-13
status: draft
---

# Trellis Core Specification v0.1

**Version:** 0.1.0-draft.1
**Date:** 2026-04-13
**Editors:** Formspec Working Group
**Companion to:** Formspec v1.0, WOS v1.0

---

## Status of This Document

This document is a **draft specification**. It is the foundation layer of the Trellis specification family — a companion framework to Formspec v1.0 and WOS v1.0 that does not modify their processing models. Implementors are encouraged to experiment with this specification and provide feedback, but MUST NOT treat it as stable for production use until a 1.0.0 release is published.

This document defines constitutional semantics only and is paired with companion specifications for bindings, trust profiles, key lifecycle, projection/runtime discipline, export/disclosure, monitoring/witnessing, and assurance traceability.

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

### 1.2 Relationship to Formspec and WOS

Trellis is a **companion framework** to Formspec v1.0 and WOS v1.0. Trellis adds canonical ledger, trust, and disclosure semantics on top of Formspec and WOS substrates. It does not modify their processing models.

**Additive invariant.** Trellis MUST NOT alter Formspec data capture, validation, or Core processing model semantics (Definition evaluation, Response validation, FEL calculation, relevance, or the four-phase processing cycle). A Formspec processor that ignores all Trellis sidecars, bindings, and artifacts remains fully conformant to Formspec and produces identical data and validation results.

**Delegation requirement.** When Trellis behavior depends on Formspec Definition or Response semantics — including field values, relevance, validation, or calculation — processing MUST be delegated to a Formspec-conformant processor (Core S1.4). Trellis defines admission, order, attestation, and verification shape for bound records; it does not specify bind/FEL/validation rules.

**Formspec conformance tier.** Trellis-bound Formspec processors MUST implement at least Formspec Core conformance (Core S2). Whether Theme or Component tiers are required depends on the Trellis conformance class: Structural and Append Service roles require Core only; roles that present or render Formspec-backed tasks to end users additionally require Component conformance.

**Screener scope.** Trellis does not redefine Screener routing, classification, or determination semantics (Screener S1–S7). When a Trellis-bound deployment uses Formspec Screener evaluation, it MUST delegate to a Formspec-conformant Screener processor. Trellis may bind Screener determination records as canonical facts but MUST NOT alter the Screener evaluation algorithm.

### 1.3 Design Goal

Prevent multi-source-of-truth drift by enforcing one canonical append-attested substrate while allowing replaceable derived systems.

---

## 2. Conformance

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in BCP 14 [RFC 2119] [RFC 8174] when, and only when, they appear in ALL CAPITALS, as shown here.

JSON syntax and data types are as defined in [RFC 8259]. URI syntax is as defined in [RFC 3986].

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

A Canonical Append Service MUST NOT issue a canonical append attestation for a record until all binding-declared admission prerequisites are satisfied, including resolution of causal or logical dependencies required for that record class (Shared Ledger Binding S5).

Schema/version compatibility policy is defined in the Shared Ledger Binding companion (S6).

### 6.2 Order

Canonical append order MUST be monotonically append-only within governed scope.

Implementations MAY partition by scope into multiple ledgers, but MUST NOT allow competing canonical orders for the same governed scope.

Canonical order positions MUST be determined solely by rules in this specification and the applicable binding. Canonical order MUST NOT depend on wall-clock receipt time, queue depth, worker identity, or other operational accidents.

### 6.2.1 Determinism note

Bindings SHOULD specify deterministic tie-breaking where concurrent admissible records could otherwise admit more than one total order consistent with declared causal constraints.

### 6.3 Idempotency and Rejection

Replayed equivalent admissions MUST be idempotent.

Rejections MUST be explicit and auditable.

---

## 7. Canonical Hash Construction

Canonical append semantics MUST use exactly one authoritative canonical event hash construction over the sealed canonical record package.

Deterministic canonical serialization is REQUIRED for canonical hashing.

Subordinate hashes MAY exist for specialized purposes (e.g., payload identity, attachments, disclosure artifacts) but MUST NOT redefine canonical append semantics.

The `hash_construction_mismatch` rejection code (Shared Ledger Binding S7) refers to a registered construction ID table. Until a dedicated registry companion is published, the single mandatory construction is JSON Canonicalization Scheme (JCS, RFC 8785) with SHA-256. Future constructions MUST be registered before verifiers are required to accept them.

---

## 8. Verification Requirements

A conforming verifier MUST be able to validate:

- canonical record integrity,
- append attestation validity,
- inclusion and consistency claims,
- export-package canonical provenance claims,

without requiring derived runtime state.

At Trellis Core conformance, verifiers MUST support at minimum the following claim classes: canonical-record integrity, append-attestation validity, and inclusion consistency. Additional claim classes (payload integrity, authorization history, disclosure policy) are defined by companion specifications (Disclosure Manifest S4, Export Verification Package S3).

---

## 9. Cross-Repository Authority Boundaries

- **Formspec** is authoritative for Definition structure and validation (Core S4), Response semantics (Core S5), FEL evaluation (Core S3), version pinning (Core S6.4, VP-01), and the four-phase processing model (Core S7). Trellis MUST NOT restate or reinterpret these semantics; it cites them by section number.
- **WOS** is authoritative for kernel lifecycle topology (Kernel S3), case state model (Kernel S4), provenance Facts tier (Kernel S6), governance enforcement (Kernel S8), and runtime behavioral contract (Runtime S4–S12). Trellis MUST NOT restate WOS evaluation or governance semantics; it cites them by section number.
- **Trellis Core** is authoritative for canonical ledger semantics (S4–S7), append/attestation semantics (S6), verification boundaries (S8), and cross-repository authority (this section).

This specification MUST NOT be interpreted to redefine Formspec or WOS semantic authority. When Trellis normative text depends on Formspec or WOS behavior, it MUST cite the relevant specification section rather than restating the behavior.

---

## 10. Security and Privacy Considerations

Implementations SHOULD minimize metadata leakage consistent with declared trust profiles.

Details of trust profile declarations, key lifecycle controls, and disclosure semantics are specified in companion documents.

### 10.1 Baseline scope (advanced capabilities)

Baseline Trellis Core conformance MUST NOT be interpreted to require advanced selective disclosure, threshold custody, group-sharing protocols, advanced homomorphic or privacy-preserving computation, or cross-agency analytic protocols unless a declared profile, binding, or implementation specification explicitly requires them. Such capabilities MAY be introduced only through those upper layers without redefining core canonical truth, order, or hash semantics established in this document.

This baseline-scope constraint corresponds to ULCR-100 and ULCOMP-R-213–214 in the requirements matrices.
