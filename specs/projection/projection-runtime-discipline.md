---
title: Trellis Companion — Projection and Runtime Discipline
version: 0.1.0-draft.1
date: 2026-04-13
status: draft
---

# Trellis Companion — Projection and Runtime Discipline v0.1

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

The Projection and Runtime Discipline companion defines strict rules for derived systems so that canonical truth never drifts into a hidden second truth. It specifies projection categories, watermarking, rebuild contracts, snapshot discipline, purge-cascade requirements, and the runtime boundary between canonical ledgers and derived processors. This companion adds projection semantics to the Trellis Core model (S4–S5). It does not define WOS runtime semantics.

## Purpose

Define strict rules for derived systems so canonical truth never drifts into hidden second truth.

## Conformance

This companion defines the following conformance roles:

1. **Projection Producer** — produces derived projections from canonical records. MUST comply with watermark, rebuild, and stale-status requirements.
2. **Projection Verifier** — validates projection correctness against canonical inputs. MUST support rebuild comparison and watermark checks.

## Projection categories

1. **Staff-facing projections.** Views presented to caseworkers, reviewers, administrators, or other operational staff. These MUST carry a watermark indicating canonical append/checkpoint state (S5). All watermark rules apply.
2. **Respondent-facing projections.** Views presented to the record subject or their delegate. These MUST carry a watermark when derived from canonical state. Respondent-facing projections MUST NOT expose staff-only metadata, internal audit trails, or governance-enforcement details not declared in the applicable trust profile's metadata budget.
3. **System projections.** Internal caches, indexes, read models, and materialized views used by the platform itself. These MUST be rebuildable from canonical records (S6) but are exempt from watermark display requirements; they remain subject to purge-cascade rules (S4).

## Normative Focus

1. Projection provenance watermarking
   - staff-facing derived views MUST carry a watermark indicating canonical append/checkpoint state.

2. Rebuild contract
   - derived systems MUST be discardable/rebuildable from canonical facts and append attestations.

3. Snapshot discipline
   - snapshots are operational artifacts; they MUST reference canonical checkpoint state and remain non-canonical.

4. Purge-cascade requirement
   - crypto-shredding is incomplete unless plaintext-derived projections/caches are purged according to policy.

5. Runtime boundary
   - workflow/orchestration engines are derived processors, not canonical ledgers.

## Projection watermark contract (draft)

Every staff-facing projection MUST expose:

1. canonical checkpoint identifier,
2. canonical append height/sequence at build time,
3. projection build timestamp,
4. projection schema/version identifier.

If the projection is stale relative to a newer canonical checkpoint, the view MUST indicate stale status.

## Rebuild verification (draft)

- Rebuilding a projection from canonical records for the same checkpoint MUST yield semantically equivalent output for declared projection fields.
- Systems SHOULD retain deterministic rebuild fixtures for critical projection types.
- Projection conformance tests MUST validate watermark presence and stale-status behavior.

## Projection integrity policy (draft)

Each conforming deployment MUST define how projection correctness is checked over time. The policy MUST include at least one of:

1. **Sampled rebuild comparison**: periodically or on demand, rebuild declared projection fields from canonical inputs for a sample of records or sequence ranges and compare against materialized projection state; or
2. **Checkpoint-bound equivalence**: at declared epoch boundaries, record a content commitment (e.g. hash) for projection state in checkpoint or export material, and verify rebuild matches that commitment before treating the snapshot as authoritative for recovery.

Access-grant or authorization-expanding projections SHOULD be checked at higher frequency than general read models.

## Deferral to WOS

The following topics are owned by WOS and are excluded from this companion:

- Execution semantics (Kernel S3, Runtime S4)
- Runtime envelope and governance-time behavior (Kernel S8, Runtime S8)
- Orchestration policy specifics (Kernel S3, Runtime S5)

> Editor's note: WOS section citations above are placeholders pending WOS spec section-number stabilization.

## Migrated requirements from `unified_ledger_core.md` (Sections 16.1 and 16.4)

1. Derived artifacts MUST be discardable and rebuildable from canonical records + append/checkpoint material.
2. Snapshots MUST remain operational artifacts and MUST carry canonical checkpoint provenance.
3. Rebuild and snapshot procedures MUST NOT mutate canonical truth.

## Security and Privacy Considerations

- Respondent-facing projections MUST NOT leak staff-only metadata beyond what the trust profile's metadata budget declares (Trust Profiles S4).
- Stale-status indications on projections MUST NOT reveal the content of canonical updates that have not yet been projected.
- Purge-cascade operations MUST NOT leave residual plaintext in system projections, caches, or backups (this companion S4, Key Lifecycle S7).
- Rebuild verification fixtures MUST be protected against tampering; compromised fixtures could mask projection drift from canonical truth.
