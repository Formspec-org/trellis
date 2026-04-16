---
title: Trellis Companion — Forms and Respondent-History Sidecar
version: 0.1.0-draft.0
date: 2026-04-14
status: stub
---

# Trellis Companion — Forms and Respondent-History Sidecar v0.1

**Version:** 0.1.0-draft.0
**Date:** 2026-04-14
**Editors:** Formspec Working Group
**Companion to:** Trellis Core v0.1

---

## Status of This Document

This document is a **placeholder stub**. It exists so that requirements traced from `DRAFTS/unified_ledger_companion.md` §6 (Forms and Respondent-History Sidecar) have a stable owning-spec reference in the companion requirements matrix. Its content is not yet authored; the obligations it will carry are listed under "Inbound Requirements" below.

This stub MUST NOT be cited as normative until a 0.1.0-draft.1 (or later) revision replaces this notice with extracted normative prose and a full Conformance section. Until then, the legacy text in `DRAFTS/unified_ledger_companion.md` §6.1–§6.9 remains the authoritative source for forms- and respondent-history-sidecar semantics.

## Conventions and Terminology

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in BCP 14 [RFC 2119] [RFC 8174] when, and only when, they appear in ALL CAPITALS, as shown here.

## Abstract

The Forms and Respondent-History Sidecar companion will define concrete forms-family and respondent-history sidecar semantics — stable path semantics, item-key semantics, validation snapshots, amendment cycles, migration outcomes, change-set structure, respondent-visible history moments, and respondent-facing export views — that remain subordinate to Trellis Core canonical-truth invariants and to the Trust Profiles, Projection, Export, and Disclosure companions.

This companion does not define Formspec processing semantics. Authored semantics for form fields, validation, calculation, and version pinning remain authoritative in Formspec Core ([Formspec Core §1.4 Conformance], [Formspec Core §6.4 Response Pinning]) and MUST be delegated to a Formspec-conformant processor.

## Purpose

Preserve stable respondent-visible meaning across drafts, submissions, amendments, validation cycles, and schema migrations without turning forms-family or respondent-history semantics into constitutional requirements.

## Scope and Subordination

This companion, when ratified, MUST remain subordinate to:

1. **Trellis Core** — canonical truth, canonical order, canonical hash construction, append-only invariants (Trellis Core §6.2).
2. **Shared Ledger Binding** — Formspec-family admission rules and family-matrix minimum fields.
3. **Trust Profiles** — custody and metadata-budget declarations governing respondent-visible material.
4. **Projection and Runtime Discipline** — respondent-facing projections MUST carry watermarks and remain rebuildable from canonical records.
5. **Export Verification Package** and **Disclosure Manifest** — respondent-facing export views MUST remain derived/disclosure-oriented and MUST NOT become canonical truth.

## Inbound Requirements

Requirements formerly tracked here (ULCOMP-R-181..188) have been retired from the Trellis companion matrix and moved upstream. See [`../cross-reference-map.md`](../cross-reference-map.md) for their current homes.

## Conformance

To be defined when normative prose is extracted. The intended conformance roles are:

1. **Forms Sidecar Producer** — produces forms-family sidecar artifacts that bind to canonical records via the Shared Ledger Binding family matrix.
2. **Respondent-History Producer** — produces respondent-history sidecar artifacts (change sets, history moments, validation snapshots) consistent with this companion's stable-path and item-key semantics.
3. **Respondent-Facing Export Generator** — assembles respondent-facing export views as derived/disclosure-oriented artifacts subordinate to the Export Verification Package and Disclosure Manifest companions.

## Cross-References

- **Trellis Core:** `../core/trellis-core.md`
- **Shared Ledger Binding:** `../core/shared-ledger-binding.md`
- **Trust Profiles:** `../trust/trust-profiles.md`
- **Projection and Runtime Discipline:** `../projection/projection-runtime-discipline.md`
- **Export Verification Package:** `../export/export-verification-package.md`
- **Disclosure Manifest:** `../export/disclosure-manifest.md`
- **Companion Requirements Matrix:** `../core/unified-ledger-companion-requirements-matrix.md`
- **Legacy source:** `../../DRAFTS/unified_ledger_companion.md` §6
