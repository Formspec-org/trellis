# Trellis Companion — Monitoring and Witnessing (Draft)

## Status

Minimal seam-oriented draft started from normalization plan.

## Purpose

Define publication and verification seams for independent monitors/witnesses without constraining baseline implementation shape.

## Normative Focus (minimal for now)

1. Checkpoint publication interface.
2. Append-growth verification interface.
3. Anti-equivocation-compatible publication requirements.
4. Verifier interoperability targets for future monitors/witnesses.

## Testability hooks (draft)

- Publication interfaces MUST expose deterministic fixtures for append-growth and checkpoint-consistency checks.
- Anti-equivocation publication requirements MUST be testable via replayable monitor scenarios.

## Current Scope Constraint

This companion intentionally defines seams only; concrete witness network topologies are deferred.
