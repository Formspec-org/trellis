---
title: Trellis Companion — Workflow, Governance, and Provenance Sidecar
version: 0.1.0-draft.0
date: 2026-04-14
status: stub
---

# Trellis Companion — Workflow, Governance, and Provenance Sidecar v0.1

**Version:** 0.1.0-draft.0
**Date:** 2026-04-14
**Editors:** Formspec Working Group
**Companion to:** Trellis Core v0.1

---

## Status of This Document

This document is a **placeholder stub**. It exists so that requirements traced from `DRAFTS/unified_ledger_companion.md` §7 (Workflow, Governance, and Provenance Sidecar) have a stable owning-spec reference in the companion requirements matrix. Its content is not yet authored; the obligations it will carry are listed under "Inbound Requirements" below.

This stub MUST NOT be cited as normative until a 0.1.0-draft.1 (or later) revision replaces this notice with extracted normative prose and a full Conformance section. Until then, the legacy text in `DRAFTS/unified_ledger_companion.md` §7.1–§7.8 remains the authoritative source for workflow-, governance-, and provenance-sidecar semantics.

## Conventions and Terminology

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "NOT RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in BCP 14 [RFC 2119] [RFC 8174] when, and only when, they appear in ALL CAPITALS, as shown here.

## Abstract

The Workflow, Governance, and Provenance Sidecar companion will define how operational workflow state, governance/processing facts, review and adjudication outputs, approval/escalation/recovery semantics, provenance families, conflict and resolution families, and workflow export views are bound to Trellis canonical records without permitting workflow runtime state to become canonical truth by default.

This companion does not define WOS runtime semantics. Workflow execution, governance evaluation, and runtime envelope behavior remain authoritative in WOS (Kernel S3–S8, Runtime S4–S12) and MUST be delegated to a WOS-conformant runtime.

## Purpose

Preserve rich workflow, governance, and provenance semantics over canonical truth without allowing workflow runtime state to override the canonical-record/derived-artifact distinction established in Trellis Core (S5.1) and Projection and Runtime Discipline (S2).

## Scope and Subordination

This companion, when ratified, MUST remain subordinate to:

1. **Trellis Core** — canonical truth, canonical order, append-only invariants, no second canonical truth (Core S5.2 invariants 1–3).
2. **Shared Ledger Binding** — WOS-family admission rules and family-matrix minimum fields.
3. **Trust Profiles** — custody postures governing what governance/review/provenance facts are visible to which observer classes.
4. **Projection and Runtime Discipline** — workflow runtime engines are derived processors, not canonical ledgers; staff-facing workflow projections MUST carry watermarks.
5. **Export Verification Package** and **Disclosure Manifest** — workflow export views MUST preserve provenance distinctions and MUST NOT imply broader coverage than the declared export scope.

## Inbound Requirements

The following requirements from the companion requirements matrix [`../core/unified-ledger-companion-requirements-matrix.md`](../core/unified-ledger-companion-requirements-matrix.md) are owned by this companion and will be extracted into normative sections in a subsequent draft:

| Inbound ID | Topic | Source in legacy draft |
|---|---|---|
| ULCOMP-R-189 | Workflow state to canonical fact mapping | `DRAFTS/unified_ledger_companion.md` §7.2 |
| ULCOMP-R-190 | Governance and processing facts | `DRAFTS/unified_ledger_companion.md` §7.3 |
| ULCOMP-R-191 | Review and adjudication semantics | `DRAFTS/unified_ledger_companion.md` §7.4 |
| ULCOMP-R-192 | Approval, escalation, and recovery semantics | `DRAFTS/unified_ledger_companion.md` §7.5 |
| ULCOMP-R-193 | Operational sequencing distinct from canonical order | `DRAFTS/unified_ledger_companion.md` §7.5 |
| ULCOMP-R-194 | Provenance family semantics | `DRAFTS/unified_ledger_companion.md` §7.6 |
| ULCOMP-R-195 | Conflict and resolution families | `DRAFTS/unified_ledger_companion.md` §7.7 |
| ULCOMP-R-196 | Workflow export views | `DRAFTS/unified_ledger_companion.md` §7.8 |

## Conformance

To be defined when normative prose is extracted. The intended conformance roles are:

1. **Workflow Sidecar Producer** — produces workflow-family sidecar artifacts that map operational workflow state to canonical facts where the binding requires it, while keeping non-canonical operational state outside canonical truth.
2. **Governance Fact Producer** — emits governance/review/adjudication facts conforming to the Shared Ledger Binding WOS-family matrix and to this companion's distinctions between canonical-admissible and operational facts.
3. **Workflow Export Generator** — assembles workflow export views as derived/disclosure-oriented artifacts subordinate to the Export Verification Package and Disclosure Manifest companions.

## Cross-References

- **Trellis Core:** `../core/trellis-core.md`
- **Shared Ledger Binding:** `../core/shared-ledger-binding.md`
- **Trust Profiles:** `../trust/trust-profiles.md`
- **Projection and Runtime Discipline:** `../projection/projection-runtime-discipline.md`
- **Export Verification Package:** `../export/export-verification-package.md`
- **Disclosure Manifest:** `../export/disclosure-manifest.md`
- **Companion Requirements Matrix:** `../core/unified-ledger-companion-requirements-matrix.md`
- **Legacy source:** `../../DRAFTS/unified_ledger_companion.md` §7
