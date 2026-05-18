# ADR 0015 — AnchorAdapter Formal Deferral

**Date:** 2026-05-18
**Status:** Accepted
**Supersedes:** —
**Superseded by:** —
**Related:**
- `thoughts/specs/2026-04-24-anchor-substrate-spike.md` — background spike; enumerates three first-class candidate adapters (OTS, Rekor, Trillian), the wire-level `ExternalAnchor` field mapping, and per-candidate trust/cost/residency analysis. This ADR does not duplicate that material; it ratifies the deferral so future agents stop reopening.
- Core §11.5 — `CheckpointPayload.anchor_ref` (`bstr / null`); opaque adapter reference.
- Core §16.3 — Optional external anchors; informative AnchorAdapter note updated to point here.
- Core §28 Appendix A — `ExternalAnchor` CDDL type (`kind`, `anchor_ref`, `required`, `description`).
- ADR 0002 — list-form `external_anchors` at export-manifest layer.
- ADR 0004 — Rust byte authority.

---

## Decision

`AnchorAdapter` is formally deferred. No center trait lands until one concrete consumer fixes enough of the five variation axes (below) to avoid speculative interface design. Core §16.3's informative paragraph is updated to cite this ADR rather than characterize the trait as "end-state design"; no normative prose changes.

---

## Context

The spike doc (`thoughts/specs/2026-04-24-anchor-substrate-spike.md`) established in April 2026 that Trellis should declare an `AnchorAdapter` trait at the center-crate boundary rather than hardcoding a single anchor substrate. That posture is correct. What remained unsettled — and unresolved at the time of this ADR — is the trait's actual surface: which variation axes it resolves, which it leaves to per-adapter extension, and what the minimal conformance contract looks like.

The Phase-1 reference server passes `external_anchors: Vec::new()`. No runtime consumer exists. The Phase-1 verification algorithm (§19 step 8) treats absent or unresolvable external anchor material as a non-failure under `report.anchor_unresolved`. The signed tree head and Merkle consistency proof are sufficient for Phase-1 verified status; anchoring is additive.

The substrate externalization preflight plan (2026-05-18) confirmed no blocking: the retag train can proceed without a trait implementation, and implementing the trait now without a concrete consumer would force a choice between a maximally-narrow trait the first consumer cannot use (rewrite on first use) or a maximally-wide trait that pre-commits to speculative shapes (over-engineering). Neither is acceptable.

---

## Variation Axes the Trait MUST Decide

These are the design questions that a concrete consumer will answer by choosing one value on each axis. Designing the trait before a consumer exists requires guessing; guessing wrong means a rewrite. The axes:

1. **Submit vs. verify vs. both.** Does the adapter submit anchor requests to the external substrate, verify retrieved proofs against it, or perform both? A submit-only adapter is useful at write time; a verify-only adapter is useful in a stranger verifier. Combining them may be overloaded responsibility, or may be necessary for test seams. A concrete consumer with a concrete substrate makes this obvious; no consumer leaves it ambiguous.

2. **Online vs. offline verification.** The spike's Rust sketch required offline verify (all witness material embedded in the receipt; no network round-trip at verify time). This is correct for the export-ZIP verification contract (§16). But a deployment that only needs server-side anchoring at write time, not portable offline verification, has a different constraint profile. The trait surface differs depending on which guarantee is load-bearing.

3. **Public log vs. private log.** Public Rekor and OpenTimestamps submit data that is log-observable. A regulated adopter running a private Trillian instance has strict data-residency requirements; the adapter must never contact a public substrate. Trust-model differences downstream the receipt-format differences that the verifier must be able to distinguish. A trait that papers over this distinction cannot carry the proof-discovery shape correctly.

4. **Proof material embedded in export vs. fetched by reference.** The spike's normative stance was "embed everything offline-verifiable inside `anchor_ref` bytes." That works for OpenTimestamps and Rekor receipt sizes. It may not work if the proof material is large (e.g., a full Trillian tile set). Fetched-by-reference changes the verifier contract fundamentally: it introduces a network dependency and a caching/pinning obligation at verification time. This is a trait-surface decision, not an adapter-internal one.

5. **Provider-specific receipt detail vs. normalized verification outcome.** The spike's `AnchorVerification` struct exposed `substrate_metadata: serde_json::Value` — a catch-all for provider-specific data (Bitcoin block hash, Rekor log ID). A normalized outcome (`verified: bool`, `anchored_at_or_before: u64`) is provider-agnostic but lossy. Provider-specific receipts give auditors more to work with but create verifier-side coupling. The trait must pick a position or define a two-level return (normalized + opaque extension), which is itself a design choice.

---

## Trigger

The first concrete deployment that requires external anchor verification — OpenTimestamps, Sigstore Rekor, Trillian, RFC 3161 TSA, or equivalent — forms a concrete consumer with a concrete substrate. That consumer's needs fix axis values 1–5 above. The trait's initial shape follows from those fixed values, not from speculation.

**Candidate prior art to evaluate at trigger time (not a pre-commitment):**

| Substrate | Trust model | Rust ecosystem | Adopt posture |
|---|---|---|---|
| OpenTimestamps | Bitcoin PoW, decentralized, free | `opentimestamps-rs` (sparse); Python `python-opentimestamps` is more complete | Wrap `opentimestamps-rs` if mature enough; else implement against the OTS spec directly with the Python impl as oracle |
| Sigstore Rekor | Sigstore community log, RFC 9162 inclusion proofs | Community `rekor-rs`; Go `github.com/sigstore/rekor` is reference | Adopt `rekor-rs` as adapter dependency if it reaches a stable API; verify offline receipt handling |
| Trillian | Self-hosted or vendor Merkle log, gRPC API | No dedicated Rust client; gRPC from protobuf schema | Generate from proto; wrap in a thin adapter crate |
| RFC 3161 TSA | Centralized PKIX TSA, regulated-sector compatible | `cms-rs`, `rfc3161-client` (Python) | Adopt `cms-rs` for DER parsing; validate against TSA trust anchors from the operator's PKI store |

None is pre-committed. Evaluate at trigger time against the consuming deployment's trust model, data-residency requirements, verifier-distribution constraints, and operational cost.

---

## Consequences

- Trellis center stays free of anchor-substrate dependencies. No crate in `trellis/crates/` depends on OTS, Rekor, Trillian, or RFC 3161 libraries until the trait lands.
- Core §16.3's informative paragraph no longer implies the trait is imminent; it points here and to the spike doc for design context.
- No `trellis-core::AnchorAdapter` trait, no `trellis-anchor-*` crates, and no `AnchorAdapterRegistry` type ship in the current retag train.
- Phase-1 verification is unaffected: §19 step 8 already treats absent anchor material as `report.anchor_unresolved` (non-failure), and `external_anchors: Vec::new()` remains the reference-server default.
- Future agents reading this ADR understand the decision, the trigger, and the axis choices that drive the trait surface. The spike doc provides the substrate detail; this ADR provides the rationale for non-implementation.

---

## Alternatives Considered

**Write the trait now, without a consumer.** Rejected. The five variation axes are unresolved. Shipping a trait before a consumer fixes them forces either a maximally-narrow surface the first consumer cannot use (requiring a rewrite on first use) or a maximally-wide surface that pre-commits to speculative shapes. The DI principle in the parent CLAUDE.md is explicit: "define the right boundary, ship what the current need requires behind it, and avoid over-engineering for later just to keep the architecture clean."

**Defer without an ADR (silent deferral).** Rejected. Silent deferral leaves future agents without a documented rationale; every future reader of Core §16.3 or the spike doc re-litigates the decision. This ADR closes that loop.

**Name a single anchor substrate and implement one adapter.** Rejected. The spike doc's central finding was that the three first-class candidates optimize for different trust models, data-residency postures, and operational cost profiles. A pre-committed substrate forces trust posture on adopters who don't share it. The DI contract at Core §11.5 (`anchor_ref: bstr / null`) and §16.3 was designed to avoid this.

---

## Reopen Criteria

Reopen this ADR if any of:

1. The first concrete deployment requiring external anchor verification materializes. That consumer's concrete substrate and variation-axis choices set the trait's initial shape; implement against them rather than reopening this ADR's deferral reasoning.
2. Core §16.3 acquires a normative MUST that the absence of a trait blocks (it does not today; the Phase-1 text is fully satisfied by `external_anchors: Vec::new()`).
3. A second external document (Trellis Operational Companion, WOS Trellis Verification spec, or a deployment-class registration) creates a hard dependency on a named adapter type in `trellis-core`. Surface that dependency and resolve it before the trait lands.

---

*End of ADR 0015.*
