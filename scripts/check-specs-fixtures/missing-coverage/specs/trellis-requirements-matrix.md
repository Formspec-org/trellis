---
title: Trellis Requirements Matrix (Synthetic Test Fixture)
version: 1.0.0-draft.1
date: 2026-04-18
status: draft
---

# Trellis Requirements Matrix (Synthetic Test Fixture)

## Purpose

Minimal matrix stub for lint test harness. Contains exactly one testable row with no corresponding vector.

---

## Column Schema (traceability)

| Column | Definition |
|---|---|
| **ID** | Stable identifier. |
| **Scope** | `core` or `operational`. |
| **Invariant** | Phase 1 invariant number or `—`. |
| **Requirement** | Normative statement. |
| **Rationale** | Why the requirement exists. |
| **Verification** | How the requirement is tested. |
| **Legacy** | Prior matrix IDs. |
| **Notes** | Optional notes. |

---

## Section 1 — Core-Scope Requirements (`TR-CORE-NNN`)

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-001 | core | 1 | Implementations MUST preserve the canonical append contract. | The contract is what interop rests on. | test-vector | — | Synthetic test row — no vector provided. |
