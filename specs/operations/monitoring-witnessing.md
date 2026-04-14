---
title: Trellis Companion — Monitoring and Witnessing
version: 0.1.0-draft.1
date: 2026-04-13
status: draft
---

# Trellis Companion — Monitoring and Witnessing v0.1

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

The Monitoring and Witnessing companion defines publication and verification seams for independent monitors and witnesses. It specifies the checkpoint publication interface, append-growth verification, and anti-equivocation publication requirements. This companion adds operational observability to the Trellis canonical substrate defined in Core (S4–S8). It does not define witness network topology, consensus protocols, or deployment architecture.

## Purpose

Define publication and verification seams for independent monitors/witnesses without constraining baseline implementation shape.

## Normative Focus

1. Checkpoint publication interface.
2. Append-growth verification interface.
3. Anti-equivocation-compatible publication requirements.
4. Verifier interoperability targets for future monitors/witnesses.

## Checkpoint publication interface (draft)

A checkpoint publication interface MUST expose at minimum:

| Resource | Description |
|---|---|
| Checkpoint ID | Stable identifier for the checkpoint within the governed scope |
| Head hash | Canonical hash of the append head at checkpoint time (Trellis Core S7) |
| Append height | Monotonic sequence number at checkpoint time |
| Merkle proof | Optional inclusion proof for the checkpoint (if the deployment uses tree-based commitments) |
| Pagination | Support for listing checkpoints by range |

The interface is protocol-agnostic (REST, gRPC, or other). Conformance is defined by the resource model, not the wire format.

## Testability hooks (draft)

- Publication interfaces MUST expose deterministic fixtures for append-growth and checkpoint-consistency checks.
- Anti-equivocation publication requirements MUST be testable via replayable monitor scenarios.
- Deterministic test fixtures SHOULD be published at a well-known repository path (e.g., `trellis/test-vectors/monitoring/`) with a documented format.

## Anti-equivocation and core append model

External witnessing builds on the core append model defined in Trellis Core S6 and S7. Monitors observe the same append attestations and checkpoint material that verifiers use (Core S8). Anti-equivocation ensures that the append service cannot present different canonical histories to different observers. This corresponds to ULCOMP-R-124–127 in the companion requirements matrix.

## Security considerations (draft)

- Publication rate: deployments SHOULD define rate limits for checkpoint queries to prevent DoS against the append service.
- Monitor authentication: deployments SHOULD authenticate monitors before granting access to append-growth or anti-equivocation interfaces.
- Witness privacy: monitor identities and query patterns SHOULD NOT be visible to other monitors or to unauthenticated observers.

## Conformance

This companion defines the following conformance roles:

1. **Append Service (monitoring)** — publishes checkpoint and append-growth interfaces. MUST expose the resources defined in the checkpoint publication interface and MUST support deterministic test fixtures.
2. **Monitor / Witness** — consumes checkpoint and append-growth interfaces. MUST verify append-growth consistency and MAY detect equivocation by comparing observed checkpoint material across independent observations.

## Current Scope Constraint

This companion intentionally defines seams only; concrete witness network topologies are deferred.
