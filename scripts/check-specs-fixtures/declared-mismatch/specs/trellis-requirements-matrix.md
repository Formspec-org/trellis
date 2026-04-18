---
title: Trellis Requirements Matrix (Synthetic Test Fixture — declared-mismatch)
version: 1.0.0-draft.1
date: 2026-04-18
status: draft
---

# Trellis Requirements Matrix (Synthetic Test Fixture — declared-mismatch)

## Purpose

Matrix stub for the declared-mismatch lint test. The matrix assigns
invariant #1 to TR-CORE-001, but the vector manifest declares
`invariants = [99]`.  The lint must report the declared/derived mismatch.

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
| TR-CORE-001 | core | 1 | Synthetic requirement 1. | Rationale 1. | test-vector | — | Synthetic row. |
