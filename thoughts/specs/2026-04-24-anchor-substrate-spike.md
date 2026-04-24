# Anchor substrate spike — DI-first adapter selection

**Date:** 2026-04-24
**Status:** Decided at the abstraction level; concrete choice deferred per-deployment.
**Lifecycle:** Spike — `AnchorAdapter` + Core §28 field mapping below are **normative-adjacent** until a Core/Companion ADR promotes them; keep here (not archive) while TODO #19 / external-anchor priority remains open.
**Owner:** Trellis center (abstraction); adapter authors (concrete).
**Relates to:** Core §11.5 (`CheckpointPayload.anchor_ref`); ADR 0002 (list-form anchors, single-anchor deployment default); Core §18.3 (`external_anchors` export-manifest entry); Companion §26 (witness / federation, Phase-4); `.claude/vision-model.md` § Active uncertainties (ε — anchor substrate).

## Decision

**Trellis does not pick a single anchor substrate.** Instead, the center declares an `AnchorAdapter` trait and enumerates three first-class candidate adapters (OpenTimestamps, Sigstore Rekor, Trillian-style tile-based logs). Each adopter selects per deployment based on trust, cost, jurisdiction, and audit requirements. Multi-adapter deployments are supported natively via the list-form `external_anchors` export-manifest entry.

This keeps the user profile's DI discipline intact — center declares shape X, adapters implement Y; none of the concrete substrates contaminates the spec. It also keeps ε (anchor substrate) from hardening into a load-bearing choice the vision model explicitly flags as unresolved.

## Context

### Why this isn't a single-winner decision

The three candidates optimize for different trust models:

- **OpenTimestamps** anchors to the Bitcoin blockchain via aggregated Merkle commitments. Proof-of-existence at a specific wall-clock time. No centralized trust (Bitcoin's PoW is the anchor). Free to submit, slow to confirm (~1 hour for Bitcoin block inclusion), verification is purely cryptographic against Bitcoin headers.
- **Sigstore Rekor** is a transparency log operated by the Sigstore community (Linux Foundation). RFC 9162 inclusion proofs. Free public service. Trust assumption: Rekor is honest and consistent (gossip protocols + witnessing reinforce). Immediate submission and proof.
- **Trillian** is the open-source transparency-log toolkit underlying Certificate Transparency and Rekor. Deploy as self-hosted (adopter runs their own log) or use an existing operator. Tile-based design for scalable verification. Trust assumption: the log operator (or a federated set) is consistent; gossip/witnessing reinforces.

A single-winner choice forces trust posture on adopters who don't share it. A regulated-industry adopter with strict data-locality requirements cannot submit to Bitcoin or public Rekor but can operate a private Trillian-backed log. A public-SaaS deployment shipping to hobbyists wants free public anchoring (OpenTimestamps or public Rekor) and has no budget for operating a log. The three optimize for different deployment contexts; center pinning one breaks the others.

### The DI contract

Core §11.5 already defines `CheckpointPayload.anchor_ref` as `bstr / null` — an opaque reference that the adapter interprets. ADR 0002 promoted this to list-form (`external_anchors: [AnchorRef]`) at the export-manifest layer so deployments can publish multiple anchor proofs. That list-form is the DI pattern's native expression: center declares "there is a list of anchor references"; each adapter provides the bytes.

This spike formalizes the adapter trait so implementors have a stable contract to target.

## Adapter trait

Rust sketch (sibling crate candidates; exact organization decided at implementation time):

```rust
/// An anchor adapter submits checkpoint digests to an external substrate
/// and verifies submitted anchor references against that substrate.
pub trait AnchorAdapter {
    /// Identifier of the adapter family. Pinned per-adapter, e.g.
    /// "opentimestamps-v1", "rekor-v1", "trillian-v1".
    /// The export-manifest carries this identifier alongside the anchor
    /// bytes so a verifier knows which adapter to dispatch to.
    fn adapter_id(&self) -> &'static str;

    /// Submit a checkpoint digest (typically the Merkle tree-head hash
    /// under `trellis-checkpoint-v1` domain separation) to the external
    /// substrate. Returns an `AnchorReceipt` whose bytes MUST be
    /// self-contained for offline verification — no network round-trip
    /// permitted at verification time.
    fn submit(
        &self,
        checkpoint_digest: &[u8; 32],
        submission_timestamp: u64,
    ) -> Result<AnchorReceipt, AnchorError>;

    /// Verify an `AnchorReceipt` offline: given the checkpoint digest,
    /// the submission timestamp, and the receipt bytes, return whether
    /// the substrate attests to the digest's existence at the claimed
    /// time (or before).
    ///
    /// Verification MUST NOT contact the external substrate. All
    /// substrate-specific witness material (Bitcoin headers, Rekor
    /// log consistency proofs, Trillian inclusion proofs) MUST be
    /// embedded in the receipt bytes or resolvable from a pinned
    /// local copy.
    fn verify(
        &self,
        checkpoint_digest: &[u8; 32],
        submission_timestamp: u64,
        receipt: &AnchorReceipt,
    ) -> Result<AnchorVerification, AnchorError>;
}

pub struct AnchorReceipt {
    pub adapter_id: String,     // matches AnchorAdapter::adapter_id()
    pub bytes:       Vec<u8>,   // adapter-opaque serialization
}

pub struct AnchorVerification {
    pub verified:               bool,
    pub anchored_at_or_before:  u64,  // Unix seconds; the latest time the substrate proves existence
    pub substrate_metadata:     serde_json::Value,  // adapter-specific: Bitcoin block hash, Rekor log id, etc.
}
```

### Adapter identity at the wire level (Core §28 `ExternalAnchor` — no silent field drop)

Core §28 / export-manifest `external_anchors` already use the CDDL type **`ExternalAnchor`**:

```cddl
ExternalAnchor = {
  kind:         tstr,
  anchor_ref:   bstr,
  required:     bool,
  description:  tstr,
}
```

This spike's earlier sketch renamed `kind → adapter_id` and `anchor_ref → receipt`, which **breaks** existing fixtures and prose that rely on `required` / `description`. **Normative mapping:**

| Spike / trait concept | Core `ExternalAnchor` field |
|---|---|
| `adapter_id()` / adapter family string | `kind` |
| opaque receipt / proof bytes | `anchor_ref` |
| Posture Declaration policy | `required` |
| operator / auditor prose | `description` |

**`submission_timestamp` and `anchor_witness`:** not representable in Core §28 today without a **spec-level** extension (optional `extensions` on `ExternalAnchor`, a parallel metadata array in the manifest, or embedding those values inside adapter-opaque `anchor_ref` bytes). Adapters SHOULD embed everything the offline verifier needs inside `anchor_ref` until a Core ADR extends the type; do not strip `required` / `description` to shoehorn new top-level keys.

A verifier encountering an `external_anchors` entry looks up an `AnchorAdapter` implementation for the **`kind`** string in its adapter registry, dispatches `verify(checkpoint_digest, submission_timestamp, anchor_ref)` (where `submission_timestamp` is parsed from inside `anchor_ref` when not yet a first-class field), and records the result in the verification report.

A verifier without an adapter for a given `kind` records the anchor as `adapter_unknown` and does NOT fail verification (the export is still byte-verifiable; just unanchored-from-this-verifier's-perspective). Adopters who require a specific anchor type publish that requirement in their Posture Declaration.

## The three first-class candidate adapters

### OpenTimestamps (adapter_id: `opentimestamps-v1`)

- **Substrate:** Bitcoin blockchain via aggregated Merkle commitment (OpenTimestamps calendar server aggregates many submissions into one Bitcoin transaction).
- **Submit:** Submit the checkpoint digest to an OTS calendar server; receive an `ots` file after ~1 hour (Bitcoin block confirmation) containing the full Merkle path to a Bitcoin block header.
- **Verify offline:** The receipt bytes contain the full Merkle path + the Bitcoin block header. Verifier checks (a) the Merkle path commits to the checkpoint digest, (b) the block header hashes to a known Bitcoin chain tip at or before submission + some confirmation depth.
- **Trust model:** Bitcoin PoW. No centralized party.
- **Cost:** Free to submit.
- **Constraints:** Requires the verifier to have access to Bitcoin block headers (either via a pinned local snapshot or a trusted header source). Verification is not fully airgapped unless the headers are shipped alongside.
- **Best for:** Public-SaaS, hobbyist, open-source deployments where the adopter trusts Bitcoin but cannot operate infrastructure.

### Sigstore Rekor (adapter_id: `rekor-v1`)

- **Substrate:** Public transparency log operated by the Sigstore community.
- **Submit:** POST to the Rekor API; receive a signed log entry + inclusion proof.
- **Verify offline:** Receipt contains the signed log entry + RFC 9162 inclusion proof + a pinned Rekor public key (or certificate chain). Verifier checks the log entry signature, the inclusion proof, and consistency against a witnessed checkpoint if available.
- **Trust model:** Rekor operator honesty + witness network. Gossip and cross-attestation reinforce.
- **Cost:** Free to submit.
- **Constraints:** Requires Rekor's public key in the verifier's trust store. Fate-sharing with Sigstore infrastructure.
- **Best for:** Deployments adjacent to software-supply-chain workflows where Sigstore is already in the trust stack.

### Trillian / tile-based transparency log (adapter_id: `trillian-v1`)

- **Substrate:** Self-hosted or operator-run Trillian log (adopter runs their own, or uses a vendor-operated one).
- **Submit:** POST to the log; receive a Merkle inclusion proof.
- **Verify offline:** Receipt contains the inclusion proof + the log's public key + ideally a witnessed checkpoint. Verifier checks signatures and inclusion.
- **Trust model:** Log operator + optional witness mesh.
- **Cost:** Deployment-dependent — self-host is free-but-ops-cost; vendor-operated is commercial.
- **Constraints:** Requires the log's public key in the verifier's trust store. Adopter may run their own for data-locality.
- **Best for:** Regulated-industry deployments with strict data-residency requirements; private/consortium use.

### Additional candidate (not first-class in Phase 1)

- **GNU Taler's tipping-time stamping** — variant of transparent logs, not widely adopted in US enterprise contexts. Viable as a `taler-v1` adapter if an adopter surfaces.
- **RFC 3161 / TSA (Time-Stamping Authority)** — classic PKIX time-stamping. Different trust posture (centralized TSA). Can be added as `rfc3161-v1` if regulated adopter needs PKIX compatibility.
- **Custom adopter anchor** (e.g., a consortium's internal log) — registered as a deployment-specific adapter_id per Posture Declaration.

## Selection guidance (not normative)

A deployment picks an adapter by walking these questions:

1. **Trust model.** Do you accept centralized operators (Rekor / vendor-Trillian), need fully-decentralized anchoring (OpenTimestamps / Bitcoin), or need adopter-controlled self-host (Trillian-private)?
2. **Data residency.** Does the receipt embedded in exports reveal anything sensitive about the export's existence or timing? OpenTimestamps submission is public-Bitcoin-observable; Rekor submission is public-log-observable; Trillian-private keeps it within adopter boundary.
3. **Verifier distribution.** Do third-party verifiers have Bitcoin headers / Rekor public key / your Trillian log's public key in their trust stores? Default answers: yes for Bitcoin, yes for Rekor, no for private Trillian.
4. **Operational cost.** Free-and-slow (OTS, ~1 hour confirmation); free-and-fast (Rekor); self-host-cost (Trillian-private); vendor-cost (Trillian-vendor).
5. **Regulatory posture.** Some jurisdictions prefer PKIX-compliant TSA; add `rfc3161-v1` in that case.

Multi-adapter deployments are encouraged where posture is uncertain or defense-in-depth matters. ADR 0002's list-form `external_anchors` accepts any mix; a verifier with multiple adapters loaded dispatches each.

## Out of scope for this spike

- **Picking one.** Explicitly deferred. `.claude/vision-model.md` ε stays open; adopters pick per deployment.
- **Witness-network requirements.** Adapter-internal.
- **Post-quantum anchor-integrity proofs.** Phase-4+ concern; each adapter handles its own migration.
- **Federation-proof coupling** (Core §26 Phase-4 witness work) — witness attestations layer on top of whatever anchor substrate is present; not within-anchor-adapter scope.

## Verification approach for implementation

Once first-party implementation lands (Phase 2 or earlier if needed):

1. `trellis-anchor-ots` crate implementing `AnchorAdapter` for OpenTimestamps, with a committed fixture `verify/016-anchor-ots-roundtrip` that round-trips a submit + offline verify against a pinned Bitcoin header snapshot.
2. `trellis-anchor-rekor` crate — same pattern with a pinned Rekor public key + example log entry.
3. `trellis-anchor-trillian` crate — same pattern, parameterized over the log's public key.
4. `trellis-verify` registers adapters via `AnchorAdapterRegistry`; configuration via `AnchorConfig` in verifier construction.

Phase-1 does NOT require any adapter to exist. Phase-1 exports MAY have empty `external_anchors`. The export-only posture in the vision model depends on signed exports + G-5 stranger test, not on external anchoring.

## Risk

- **Adapter registry sprawl.** Many adopters declare custom `adapter_id` values without standardizing. Mitigation: the `tstr` escape is an explicit extension point; adopters publish their `adapter_id` and verifier-integration docs. Not a Trellis-center problem to solve — it's a normal DI concern.
- **Receipt-format drift within an adapter.** OpenTimestamps `ots` format, Rekor log entry shape, Trillian proof shape could all evolve. Mitigation: version the adapter_id (`opentimestamps-v1` → `v2` if breaking change). Verifier registry dispatches on version.
- **Deprecation of a substrate.** If Sigstore community shuts Rekor down, all `rekor-v1` receipts become unverifiable from scratch (Rekor public key gone, inclusion proofs un-crosschecked). Mitigation: (a) adopters requiring long-term verifiability choose substrates with published archival plans; (b) multi-adapter anchoring defends against single-substrate failure; (c) OpenTimestamps' Bitcoin anchoring has the strongest long-term survivability.

## Follow-ons triggered

- **Anchor adapter registration doc.** Companion §26 (witness + federation Phase-4 work) gains a paragraph noting that external anchors are adapter-provided per this spike; witnesses layer on top.
- **Phase-2+ adopter picks.** SBA PoC probably picks `trillian-v1` self-hosted (data-residency). Public-SaaS wedge probably picks `opentimestamps-v1` + `rekor-v1` double-anchored.
- **Sequence item #19 (`trellis.external_anchor.v1` priority)** gets filled in as a priority-policy decision per-deployment: "when multiple adapters attest, which one's `anchored_at_or_before` drives the posture-transition priority check?" Answered in the adopter's Posture Declaration.

## Decision log

- 2026-04-24 — Declared the DI trait (`AnchorAdapter`) and enumerated three first-class candidates (OTS, Rekor, Trillian). Concrete choice deferred per-deployment. ε in vision model remains formally open; this spike narrows it from "pick a substrate" to "pick per adopter, center ships the trait."

---

*End of spike memo. No implementation action required until first Phase-2 anchor-using adopter surfaces.*
