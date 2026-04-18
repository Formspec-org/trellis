---
title: Trellis Requirements Matrix (Synthetic Test Fixture — invariant-gap)
version: 1.0.0-draft.1
date: 2026-04-18
status: draft
---

# Trellis Requirements Matrix (Synthetic Test Fixture — invariant-gap)

## Purpose

Matrix stub for the invariant-gap lint test. Uses hash-prefixed and
multi-value invariant cell formats (`#5`, `#1, #4`) to exercise the
`parse_invariants_cell` helper.  A vector covers only TR-CORE-005
(invariant #5).  Invariants #1 and #4 (both in TR-CORE-006) are left
uncovered so the lint must report exactly those two as gaps.

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
| TR-CORE-005 | core | #5 | Hash-prefixed invariant cell. | Exercises #N format. | test-vector | — | Synthetic row. |
| TR-CORE-006 | core | #1, #4 | Multi-value invariant cell. | Exercises #N, #M format. | test-vector | — | Synthetic row. |
