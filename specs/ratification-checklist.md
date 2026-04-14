# Trellis Spec Family Ratification Checklist (Draft)

## Purpose

Define a concrete stopping condition for moving draft specs toward normative ratification.

## Global gates

- [x] Core/companion boundaries are stable and non-overlapping. *(evidence: `G-1` in `ratification-evidence.md`)*
- [x] Trellis/Formspec/WOS ownership boundaries are explicit and unambiguous. *(evidence: `G-2`)*
- [x] Every MUST-level requirement has at least one traceability entry in `assurance-traceability.md`. *(evidence: `G-3`)*
- [ ] Shared native/WASM vectors exist for canonical serialization + hash construction. *(pending auto evidence: `G-4`)*
- [ ] Offline verifier behavior is reproducible across at least two independent implementations. *(pending auto evidence: `G-5`)*

## Per-document readiness gates

### `trellis-core.md`

- [x] Canonical append semantics use final normative language. *(evidence: `C-1`)*
- [x] One-order-per-scope rule is formally testable. *(evidence: `C-1`)*
- [x] Canonical hash construction is uniquely specified. *(evidence: `C-1`)*
- [x] Idempotency rules are explicit and test-backed. *(evidence: `C-1`)*

### `shared-ledger-binding.md`

- [x] Family IDs and minimum fields are complete for Formspec/WOS/trust/release families. *(evidence: `B-1`)*
- [x] Schema/version evolution rules include compatibility policy. *(evidence: `B-1`)*
- [x] Canonization rejection reasons are machine-testable. *(evidence: `B-1`)*

### `trust-profiles.md`

- [x] Reader-held/provider-readable/tenant-operated profiles have complete declaration schemas. *(evidence: `T-1`)*
- [x] Metadata budget declaration is mandatory and machine-readable. *(evidence: `T-1`)*
- [x] Trust honesty rule is audited with conformance checks. *(evidence: `T-1`)*

### `key-lifecycle-operating-model.md`

- [x] Lifecycle transitions are complete and policy-safe. *(evidence: `K-1`)*
- [x] Grace-period behavior is fully specified for offline clients. *(evidence: `K-1`)*
- [x] Recovery and destruction semantics include evidence requirements. *(evidence: `K-1`)*

### `projection-runtime-discipline.md`

- [x] Watermark contract is fully specified and implemented in staff-facing projections. *(evidence: `P-1`)*
- [x] Rebuild equivalence criteria are deterministic and tested. *(evidence: `P-1`)*
- [x] Purge-cascade verification is part of operational runbooks. *(evidence: `P-1`)*

### `export-verification-package.md`

- [x] Verification manifest fields are finalized. *(evidence: `E-1`)*
- [x] Readability declarations and trust-profile carriage are consistent. *(evidence: `E-1`)*
- [x] Offline verification passes cross-implementation vectors. *(evidence: `E-1`; auto evidence pending in `G-5`)*

### `disclosure-manifest.md`

- [x] Claim-class taxonomy is finalized. *(evidence: `D-1`)*
- [x] Selective disclosure semantics preserve canonical provenance. *(evidence: `D-1`)*
- [x] Disclosure artifacts cannot be mistaken for canonical rewrites. *(evidence: `D-1`)*

### `monitoring-witnessing.md`

- [x] Checkpoint publication seam is stable. *(evidence: `M-1`)*
- [x] Append-growth verification seam is stable. *(evidence: `M-1`)*
- [x] Anti-equivocation publication requirements are testable. *(evidence: `M-1`)*

### `assurance-traceability.md`

- [x] Every core invariant maps to executable checks. *(evidence: `A-1`)*
- [x] Evidence artifact retention policy is defined. *(evidence: `A-1`)*
- [x] Recovery/destruction drills have recurring cadence. *(evidence: `A-1`)*

## Natural stopping point for this extraction phase

This phase is complete when all documents above have draft-level section coverage and ratification gates are populated (even if unchecked).
