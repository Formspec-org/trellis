---
title: Trellis Requirements Matrix (Synthetic Test Fixture — spec-cross-ref-missing-cite)
version: 1.0.0-draft.1
date: 2026-04-18
status: draft
---

# Trellis Requirements Matrix (Synthetic Test Fixture — spec-cross-ref-missing-cite)

## Purpose

A single TR-CORE row with `Verification=spec-cross-ref` and no `Core §N` or
`Companion §N` citation in Requirement / Rationale / Notes. R6 must emit a
warning (not a hard error) to preserve the current repo's uncited rows as
tolerable drift while surfacing the gap.

---

## Section 1 — Core-Scope Requirements

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-001 | core | — | Synthetic core requirement. | Rationale without any section citation. | spec-cross-ref | — | Synthetic. |
