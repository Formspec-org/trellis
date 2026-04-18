# Trellis

**Trellis is the cryptographic integrity substrate** for the **[Formspec](https://github.com/Formspec-org/formspec)** (intake) + **[WOS](https://github.com/Formspec-org/wos-spec)** (governance) stack. It specifies the envelope, chain, checkpoint, and export-bundle format by which a Formspec response and its downstream WOS governance events become a single append-only, signed, offline-verifiable record.

Trellis does not replace Formspec or WOS. It concretely answers two already-written deferrals: the Respondent Ledger §13 `LedgerCheckpoint` seam and the WOS `custodyHook` (§10.5). What survives when the system, the vendor, and the years go away is the Trellis export.

Paths below are relative to `trellis/`.

---

## Status

Trellis is **actively being refined**. The repository contains normative drafts (under [`specs/`](specs/)), the product vision and roadmap (under [`thoughts/`](thoughts/)), and valuable prior research that informed both (under [`thoughts/research/`](thoughts/research/) and [`thoughts/reviews/`](thoughts/reviews/)).

**No document here is an accepted final decision** for Formspec, WOS, or any shipped system. Trellis is greenfield: no production legacy to preserve, no backwards-compatibility obligation.

**Operating lens:** compute is cheap, time is cheaper, development is near-free next to the long-term cost of architectural debt. Expensive mistakes are architectural (data model, crypto boundaries, event taxonomy, sync contracts), not editorial. Prefer clean rethink over carrying a weak compromise forward. The [product vision](thoughts/product-vision.md) and the [Phase 1 envelope invariants](thoughts/product-vision.md#phase-1-envelope-invariants-non-negotiable) encode this: Phase 1 must name byte-exact decisions now because each is cheap to include and wire-breaking to retrofit.

---

## Licensing

Trellis is part of the Formspec monorepo. All specification documents, drafts, and research materials here are licensed under **Apache-2.0**. See the root [`LICENSE`](../LICENSE) and [`LICENSING.md`](../LICENSING.md).

---

## The four top-level documents

Start here. Read in order.

| Document | Role |
|---|---|
| [`thoughts/product-vision.md`](thoughts/product-vision.md) | Authoritative product roadmap. Four-phase arc, 15 non-negotiable Phase 1 envelope invariants, ledger/log vocabulary, delivery shape, tracks A–E. |
| [`specs/trellis-agreement.md`](specs/trellis-agreement.md) | Non-normative decision gate. Scope, primitives, seams, delivery shape, success criterion. A sign-off here authorizes the rest of Track A. |
| [`specs/trellis-core.md`](specs/trellis-core.md) | **Normative.** Phase 1 byte protocol: envelope, canonical encoding, signature profile, chain, checkpoint, export, verification. |
| [`specs/trellis-operational-companion.md`](specs/trellis-operational-companion.md) | **Normative.** Phase 2+ operator obligations: custody models, projection discipline, metadata budgets, delegated-compute honesty, sidecars. |

Supporting:

- [`specs/trellis-requirements-matrix.md`](specs/trellis-requirements-matrix.md) — Traceability matrix (79 TR-CORE + 49 TR-OP rows, legacy ULCR/ULCOMP-R provenance, gap log). Prose in Core and the Companion wins on conflict.
- [`specs/cross-reference-map.md`](specs/cross-reference-map.md) — Upstream-rehoming map for concepts owned by Formspec Respondent Ledger or WOS.
- [`specs/README.md`](specs/README.md) — Reading order, authority claims, archive pointers.

Completed handoff log — Groups A/B/C resolved in commit `3a143a1`. See [`thoughts/specs/2026-04-17-trellis-normalization-handoff.md`](thoughts/specs/2026-04-17-trellis-normalization-handoff.md) for the original task breakdown and evidence.

---

## Research and reviews (reference material, not normative)

These informed the current specs and remain cited where their insights survive. None are normative.

- [`thoughts/research/2026-04-10-unified-ledger-technology-survey.md`](thoughts/research/2026-04-10-unified-ledger-technology-survey.md) — OSS and managed component survey (COSE, Merkle, OpenTimestamps, Trillian, KMS, DIDs) with phase assignments.
- [`thoughts/research/ledger-risk-reduction.md`](thoughts/research/ledger-risk-reduction.md) — Standards-first counterweight: where to prefer composable pieces (transparency-log patterns, COSE, SD-JWT) over bespoke crypto.
- [`thoughts/research/tiered-privacy-white-paper-3-24-2025.md`](thoughts/research/tiered-privacy-white-paper-3-24-2025.md) — TPIF framework (proof of personhood, tiered identity/authenticity). Informs Phase 4 Sovereign framing.
- [`thoughts/research/unified_implementation_proposal.md`](thoughts/research/unified_implementation_proposal.md) — Substrate-selection matrix (Temporal / OpenFGA / OpenFisca / CloudEvents / PROV / MCP / Postgres) for the surrounding stack. **Not** a Trellis spec; informs Track B/D engineering.
- [`thoughts/reviews/2026-04-10-expert-panel-unified-ledger-review.md`](thoughts/reviews/2026-04-10-expert-panel-unified-ledger-review.md) — Multi-expert review; Phase 1 vs later split; critical issues list.
- [`thoughts/reviews/2026-04-11-crypto-expert-concrete-solutions.md`](thoughts/reviews/2026-04-11-crypto-expert-concrete-solutions.md) — Protocol-level fixes (ordering, rotation, commitments, header privacy, GDPR shredding).

---

## Upstream-owned (referenced, not authored here)

- [`thoughts/formspec/specs/respondent-ledger-spec.md`](thoughts/formspec/specs/respondent-ledger-spec.md) — Formspec Respondent Ledger add-on (v0.1). Owns `Profile A/B/C` (privacy × identity × integrity-anchoring posture). Trellis binds to its §6.2 `eventHash`/`priorEventHash` and §13 `LedgerCheckpoint` seams; a Track E §21 spec extension adds normative case-ledger and agency-log objects.
- Formspec Core, WOS Kernel, WOS Assurance, WOS Governance — referenced by Trellis Core composition sections (§22, §23) and by the cross-reference map for upstream-rehomed requirements.

---

## Historical material (archived, not normative)

- [`specs/archive/`](specs/archive/) — The previous 8-spec family (`core/`, `trust/`, `projection/`, `export/`, `operations/`, `forms/`, `workflow/`, `assurance/`). Superseded by the two-spec model; retained for provenance. **Do not cite as normative.**
- [`thoughts/archive/drafts/`](thoughts/archive/drafts/) — Legacy DRAFTS mined into the current specs: `unified_ledger_core`, `unified_ledger_companion`, both legacy requirements matrices, and the eight-spec normalization plan.
- [`thoughts/archive/specs/2026-04-10-unified-ledger-concrete-proposal.md`](thoughts/archive/specs/2026-04-10-unified-ledger-concrete-proposal.md) — 160K omnibus proposal; §§3, 3b, 8, 11, 16 mined into `trellis-core.md`.
- [`thoughts/formspec/adrs/0054-privacy-preserving-client-server-ledger-chain.md`](thoughts/formspec/adrs/0054-privacy-preserving-client-server-ledger-chain.md) — Historical ADR; informed Phase 4 Sovereign variant.
- [`thoughts/formspec/adrs/0059-unified-ledger-as-canonical-event-store.md`](thoughts/formspec/adrs/0059-unified-ledger-as-canonical-event-store.md) — Historical ADR; "unified canonical event store now" decision superseded by the phased arc (Phase 1 export bundles → Phase 3 unified case ledger).
- [`thoughts/formspec/proposals/user-side-audit-ledger-add-on-proposal.md`](thoughts/formspec/proposals/user-side-audit-ledger-add-on-proposal.md) — Originating proposal for the Respondent Ledger add-on; user-side framing adopted into Phase 4 Sovereign.

---

## Process

- [`ratification/`](ratification/) — Readiness gates and evidence for moving the two normative specs toward ratification.
- [`scripts/check-specs.py`](scripts/check-specs.py) — Lint enforcing forbidden patterns (signature zero-fill prose, JCS references, stale version strings, unarchived per-family paths, Profile-namespace hygiene).

---

## Reading order by goal

| If you want to… | Start here |
|---|---|
| Understand the roadmap and why the architecture looks the way it does | [`thoughts/product-vision.md`](thoughts/product-vision.md) |
| Gate the project for sign-off | [`specs/trellis-agreement.md`](specs/trellis-agreement.md) |
| Implement append/verify/export against fixtures | [`specs/trellis-core.md`](specs/trellis-core.md) |
| Understand operator obligations (custody, projections, sidecars) | [`specs/trellis-operational-companion.md`](specs/trellis-operational-companion.md) |
| Trace a legacy ULCR/ULCOMP-R row to its current home | [`specs/trellis-requirements-matrix.md`](specs/trellis-requirements-matrix.md) + [`specs/cross-reference-map.md`](specs/cross-reference-map.md) |
| See outstanding architectural work | [`thoughts/specs/2026-04-17-trellis-normalization-handoff.md`](thoughts/specs/2026-04-17-trellis-normalization-handoff.md) |
| Understand how the crypto choices were reached | Research + reviews folders |

Heading-level inventories for every active document live in [`REFERENCE.md`](REFERENCE.md).
