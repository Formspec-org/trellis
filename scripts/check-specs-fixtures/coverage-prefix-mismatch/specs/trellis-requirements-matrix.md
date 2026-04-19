---
title: Trellis Requirements Matrix (Synthetic Test Fixture — coverage-prefix-mismatch)
version: 1.0.0-draft.1
date: 2026-04-18
status: draft
---

# Trellis Requirements Matrix (Synthetic Test Fixture — coverage-prefix-mismatch)

## Purpose

Two matrix rows — one TR-CORE-* and one TR-OP-* — with Verification set to `—`
so the coverage-demand rules (R4/R5/R7) do not fire. The companion manifest
mis-files each row into the OTHER bucket so the prefix-discipline rule (R8)
is what forces lint to fail.

---

## Section 1 — Core-Scope Requirements

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-020 | core | — | Synthetic core requirement. | Rationale. | — | — | Synthetic. |

## Section 2 — Operational-Scope Requirements

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-OP-042 | operational | — | Synthetic op requirement. | Rationale. | — | — | Synthetic. |
