# Trellis

**Trellis is the primary place to develop the shared ledger** between **[Formspec](https://github.com/Formspec-org/formspec)** and **[wos-spec](https://github.com/Formspec-org/wos-spec)** (case lifecycle, provenance, and governance on the WOS side). This directory holds cross-cutting design: research, ADRs, normative drafts, proposals, and reviews for the unified respondent ledger, privacy-preserving client/server chains, and related crypto and sync semantics. Paths below are relative to `trellis/`.

*Authoritative specs may still live in each repo; Trellis is where joint evolution is coordinated before changes land in Formspec, wos-spec, or both.*

**Heading-level outlines** (every H1/H2 with short blurbs) live in [`REFERENCE.md`](REFERENCE.md).

## Status

Trellis is **actively being refined**: drafts get rewritten, alternatives get compared, and whole threads may be **discarded or replaced**. **No document here is an accepted or final decision** for Formspec, wos-spec, or any shipped system. This is **greenfield** work—no production legacy to preserve, no requirement to keep earlier sketches for backwards compatibility.

**Lens:** **Compute is cheap, time is cheaper, development is free** next to the long-term cost of **technical debt** locked in by the wrong seams (data model, crypto boundaries, event taxonomy, sync contracts). Treat expensive mistakes as architectural, not editorial: prefer a clean rethink over carrying a weak compromise forward.

When materials below use words like *decision* or *ADR*, read them as **structured arguments**—candidates to adopt, amend, or reject—not as ratified policy.

---

## How the pieces relate

The work clusters into three threads that **converge on one shared ledger** Formspec runtimes and WOS case workflows both need.

### 1. Formspec add-on: respondent-facing history (without replacing `Response`)

The **user-side audit proposal** argued for an optional companion document: material, path-native events alongside the frozen `Response`. The **respondent ledger spec (v0.1)** drafts the normative shape of that add-on (`RespondentLedger`, events, `ChangeSetEntry`, checkpoints, conformance). **ADR-0054** sits above both: it **argues for** chaining client capture → server authority → platform audit and export, with tiered crypto (encryption first; zk/MPC/HE where justified) and a provider-neutral identity model. It assumes the respondent ledger stays additive to core Formspec and ties forward to platform audit ADRs (e.g. 0003)—all subject to the status note above.

### 2. WOS + Formspec: one canonical event store for the case lifecycle

**ADR-0059** (draft) **reframes** the problem for the **shared** system: stop treating respondent ledger, WOS provenance, and “the database” as separate sources of truth. It **proposes** a **single append-only ledger** as the portable, ciphertext-hashed case record from intake through WOS governance, with **Temporal** limited to orchestration and projections—not as a second ledger of record. **Part 4** (unified event taxonomy) is the intended handshake surface with **wos-spec** (governance and provenance kinds) and Formspec (intake / coprocessor-shaped events), if that direction holds.

The **concrete proposal** (2026-04-10) is the engineering expansion of ADR-0059: identity and keys, envelope v2, sync and merge, coprocessor transition, **WOS provenance integration**, projections, disclosure, export, and crate layout—explicitly “build the real thing,” not a stub roadmap.

### 3. Evidence, pushback, and hardening

The **technology survey** maps ADR-0059’s capability areas to concrete OSS and managed components (immutable storage, COSE, BBS+, anchoring, KMS, DIDs) and proposes a phased stack. The **expert panel review** stress-tests ADR-0059 plus the survey: unanimous themes, a critical-issues list, and a **Phase 1 vs later** split. The **crypto expert concrete solutions** doc answers that review with protocol-level fixes (ordering, rotation, commitments, header privacy, GDPR shredding, nonce discipline) and a consolidated **Header V2**.

**Ledger risk reduction** is a counterweight to the concrete proposal: it argues for **standard, composable** pieces (transparency-log patterns, COSE, SD-JWT-first disclosure, authz engines, formal methods) wherever bespoke design does not buy clear leverage—so the relationship to the proposal stack is *tension + refinement*, not a separate product line.

### 4. Cross-cutting vocabulary: tiered privacy / TPIF

The **tiered privacy white paper** is broader than Formspec or WOS: it defines a five-tier identity and authenticity framework (PoP VC, consortium chain, advanced crypto). **ADR-0054** references this framing for **tiered privacy and assurance**; it informs language and deployment profiles rather than replacing Formspec or WOS normative text.

---

## Reading order (suggested)

| If you want to… | Start here |
|-----------------|------------|
| Understand the Formspec-only add-on | `user-side-audit-ledger-add-on-proposal.md` → `respondent-ledger-spec.md` |
| See how client/server/platform chain together | `0054-privacy-preserving-client-server-ledger-chain.md` |
| See the draft unified Formspec + WOS ledger direction (ADR-0059) | `0059-unified-ledger-as-canonical-event-store.md` |
| Drill into implementation shape | `2026-04-10-unified-ledger-concrete-proposal.md` |
| Pick components and phases | `2026-04-10-unified-ledger-technology-survey.md` |
| See external scrutiny and phased delivery | `2026-04-10-expert-panel-unified-ledger-review.md` |
| See crypto/protocol responses | `2026-04-11-crypto-expert-concrete-solutions.md` |
| Pressure-test bespoke vs standards | `ledger-risk-reduction.md` |
| See the convergence normalization plan for Trellis spec-family boundaries | `DRAFTS/trellis_spec_family_normalization_plan.md` |
| Start the split-out drafts from core + companion into spec-family docs | `specs/core/trellis-core.md` and companion drafts under `specs/*/` |
| See the draft dependency order and ownership map for the spec family | `specs/README.md` |
| Ratification gates and evidence (process, not normative specs) | `ratification/README.md` |
| Tiered identity / privacy background | `tiered-privacy-white-paper-3-24-2025.md` |

---

## One-line summaries

| Document | What it is |
|----------|------------|
| `thoughts/formspec/proposals/user-side-audit-ledger-add-on-proposal.md` | Product/architecture pitch for an optional respondent ledger beside `Response`. |
| `thoughts/formspec/specs/respondent-ledger-spec.md` | Normative v0.1 add-on: objects, events, materiality, checkpoints, conformance. |
| `thoughts/formspec/adrs/0054-privacy-preserving-client-server-ledger-chain.md` | Draft ADR: layered trust chain and tiered crypto/identity from client to export. |
| `thoughts/formspec/adrs/0059-unified-ledger-as-canonical-event-store.md` | Draft ADR: one canonical ledger for Formspec + WOS lifecycle; Temporal as execution only. |
| `thoughts/specs/2026-04-10-unified-ledger-concrete-proposal.md` | Full-stack engineering proposal extending ADR-0059 (sync, crypto, WOS hooks). |
| `thoughts/research/2026-04-10-unified-ledger-technology-survey.md` | OSS/managed survey aligned to ADR-0059’s building blocks and phases. |
| `trellis/thoughts/reviews/2026-04-10-expert-panel-unified-ledger-review.md` | Multi-expert review of ADR-0059 + survey; synthesis and roadmap. |
| `trellis/thoughts/reviews/2026-04-11-crypto-expert-concrete-solutions.md` | Detailed crypto/protocol answers; Header V2. |
| `thoughts/research/ledger-risk-reduction.md` | Standards-first risk memo relative to the concrete proposal. |
| `DRAFTS/trellis_spec_family_normalization_plan.md` | Convergence pass plan that normalizes Trellis into a core + companion family with explicit Trellis/Formspec/WOS boundaries. |
| `specs/core/trellis-core.md` | New split-out constitutional core draft started from `unified_ledger_core.md` as the first step of normalization. |
| `specs/core/shared-ledger-binding.md` | New companion draft for Formspec/WOS/trust/release family binding and canonization rules. |
| `specs/trust/trust-profiles.md` | New companion draft for trust posture declarations and metadata budgets. |
| `specs/trust/key-lifecycle-operating-model.md` | New companion draft for key classes, lifecycle states, rotation, grace periods, recovery, and crypto-shredding completeness. |
| `specs/projection/projection-runtime-discipline.md` | New companion draft for provenance watermarking, rebuild contract, snapshot discipline, and purge-cascade obligations. |
| `specs/export/export-verification-package.md` | New companion draft for offline-verifiable export package requirements and payload readability declarations. |
| `specs/export/disclosure-manifest.md` | New companion draft for audience-scoped disclosure claim semantics and provenance-preserving selective disclosure. |
| `specs/operations/monitoring-witnessing.md` | New minimal seam-oriented companion draft for checkpoint publication and anti-equivocation-compatible monitoring. |
| `specs/assurance/assurance-traceability.md` | New companion draft mapping core invariants to TLA+/Alloy/tests/fuzzing/drill evidence artifacts. |
| `ratification/ratification-checklist.md` | Draft readiness gates and stopping criteria for moving the spec family from draft to normative ratification. |
| `ratification/ratification-evidence.md` | Draft evidence registry linking checklist gates to concrete spec artifacts and identifying remaining auto-evidence gaps. |
| `ratification/README.md` | Index for ratification checklist and evidence (sits beside `specs/`, not inside it). |
| `specs/README.md` | Draft spec-family index with dependency order, Trellis/Formspec/WOS boundaries, and next extraction passes. |
| `thoughts/research/tiered-privacy-white-paper-3-24-2025.md` | TPIF white paper: tiered identity and strong crypto at internet scale. |

---

*Update [`REFERENCE.md`](REFERENCE.md) when you add files or restructure headings; keep this README’s relationship narrative in sync when the architecture story changes. None of that implies acceptance—only that the map matches the current draft set.*
