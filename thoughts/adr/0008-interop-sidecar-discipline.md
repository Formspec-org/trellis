# ADR 0008 — Interop Sidecar Discipline

**Date:** 2026-04-24
**Status:** Accepted (pending implementation)
**Supersedes:** —
**Superseded by:** —
**Related:** ADR 0002 (list-form anchors — SCITT receipts are one Phase-2+ anchor expression); ADR 0003 (§22/§24 reservation discipline — same reserve-but-lock-off pattern); ADR 0006 (key-class taxonomy — `did-key-view` labels the signing arm); ADR 0007 (certificate of completion — C2PA is the registered interop format for its presentation artifact); Core §11 (Checkpoint Format — SCITT-receipt derivation substrate); Core §14 / §16 (independence of verification — core crates MUST NOT pull ecosystem libs); Companion §14 (Derived-Artifact Discipline — parent discipline for canonical-first); [`thoughts/specs/2026-04-24-anchor-substrate-spike.md`](../specs/2026-04-24-anchor-substrate-spike.md) (Phase-2+ anchor-substrate DI stance — SCITT-receipt sidecar is the export-bundle-visible counterpart).

## Decision

Trellis defines **Interop Sidecars** — deterministic derivations of canonical records (events, checkpoints, registry entries, attached artifacts) into external-ecosystem envelope formats, carried alongside the canonical bytes in the export bundle without replacing them. Four sidecar kinds are registered as Phase-2+ interop seams, reserved in the envelope today with Phase-1 population locked off per ADR 0003 discipline.

| `kind` identifier | derives from | targets |
|---|---|---|
| `scitt-receipt` | Core §11 Checkpoint + one target event hash | IETF SCITT COSE-Receipts signed statement |
| `vc-jose-cose-event` | Core §6 Event | W3C VC 2.0 via VC-JOSE-COSE |
| `c2pa-manifest` | ADR 0007 Presentation Artifact | C2PA 2.x manifest attached to the presentation PDF |
| `did-key-view` | Core §8 signing-class key registry | `did:key` labeling view (no signing, no network) |

All four share a single discipline: **canonical-first, deterministic, additive, registered, crate-isolated.** The verifier MUST accept core bytes without any sidecar; sidecars are additive evidence, never a replacement path. Core and verify crates MUST NOT depend on any ecosystem library.

The rejected alternative — rehome the spec on top of an established ecosystem stack (full W3C VC + Data Integrity, or SCITT, or C2PA) — is declined because every mature stack misses a different fraction of Trellis's required surface (byte-exact stranger-test, per-case append-only chain, crypto-shredding via domain-separated encrypt-then-hash, offline-verifiable export, key-class taxonomy). Adopting one as the center forces us to rebuild the other 60% inside its vocabulary, paying a large tax for ecosystem compatibility that concrete first adopters (e.g., SBA PoC) do not ask for. Keeping Trellis at the center and emitting ecosystem envelopes as *derived* sidecars preserves both properties: our center stays coherent, and any individual adopter that needs ecosystem compat unlocks it per-kind without a wire-format break.

The second rejected alternative — wait for an adopter to ask, then retrofit interop at the envelope / export manifest layer — is declined because retrofitting the export-manifest shape after `v1.0.0` is tagged would break byte-exact stranger-test equivalence. Reserving the slot now (empty in Phase 1) is additive and free; populating it later is scoped to the adapter crate, not the core wire.

## Context

### Why now, when no adopter asks

Phase 1 is the right time because (a) wire-level reservation is cheap under ADR 0003 philosophy — reserve slots, lock off population, pay the cost later only if a kind ever activates; (b) all four target ecosystems are plausible audiences (SCITT for public-sector transparency logs, VC for SSI-native agencies, C2PA for court-facing PDF provenance, DID for portable key identity) where retrofitting after tag would be a wire break; (c) tagging `v1.0.0` as a snapshot rather than a freeze (per `TODO.md` tagging baseline) authorizes additive wire changes precisely when they prevent architectural debt. This is such a change.

### Why these four and not more

Each maps to a real ecosystem whose adoption would otherwise force a spec fork:

- **SCITT** — the IETF Supply Chain Integrity, Transparency, and Trust WG is standardizing exactly the claim-→-transparency-log-→-receipt pattern Trellis checkpoints already implement. Semantic alignment costs nothing now and keeps the door open to an IETF-blessed verifier ecosystem.
- **VC 2.0 / VC-JOSE-COSE** — W3C Recommendation. Some SSI-native agencies (health, education, identity) will standardize on VC wrappers. Offering a VC envelope derivation lets Trellis drop into those deployments without rehoming.
- **C2PA** — the content-provenance coalition's manifest format is increasingly the default for signed PDFs (journalism, legal, government document provenance). ADR 0007's certificate of completion is the exact use case C2PA was designed for.
- **DID (`did:key` only for Phase 1 registry)** — a labeling view that costs effectively nothing (the DID IS the public key) and lets SSI-fluent audiences use DID vocabulary without requiring us to change key-registry semantics. Network-dependent DID methods (`did:web`, `did:ion`, `did:plc`) are explicitly excluded — they break Core §16 offline-verification independence.

Other candidates considered and deferred:

- **SD-JWT VC / BBS+ selective disclosure** — belongs in §22 commitment-slot work when Phase 2 selective disclosure opens; not a base-layer interop envelope.
- **`did:web`** — offline-verification break as noted; a future `did-web-view` kind could be added with a strict "treat DID as opaque string offline, resolve only online" rule. Not blocking.
- **Full W3C Data Integrity with RDF canonicalization (URDNA2015 / RDFC-1.0)** — implementation-dependent canonicalization is a stranger-test landmine. If a VC-DI adopter ever arrives, layer it as a separate kind with a narrower conformance statement than the VC-JOSE-COSE kind.

### Why register now rather than via `EventPayload.extensions`

Interop sidecars are export-bundle-level artifacts, not per-event payload extensions. They describe how *the export as a whole* is additionally expressed in ecosystem formats. The export manifest (Core §18) is the correct reservation point; `EventPayload.extensions` is for event-level extension data and would be a category mistake.

## Discipline

Every interop sidecar MUST satisfy:

**ISC-01 (MUST).** Canonical-first. The export bundle MUST verify from core bytes alone; a conformant verifier MUST be able to ignore every interop sidecar and reach `integrity_verified = true` on an otherwise-valid export. Interop sidecars are additive evidence, never the sole verification path. This is the operational restatement of Companion §14.2 (No Second Canonical Truth) extended to ecosystem derivations.

**ISC-02 (MUST).** Deterministic derivation. For a pinned `(kind, derivation_version)`, `derive(kind, canonical_bytes) → sidecar_bytes` MUST be byte-exact. Two conforming adapters MUST produce byte-identical sidecars from the same canonical source. Per-kind stranger-test coverage is required at adoption time.

**ISC-03 (MUST).** Additive only. Removing any individual interop sidecar from an export MUST NOT break core verification. Sidecars live in a dedicated `interop-sidecars/` tree inside the export ZIP; each file's content digest is committed by the export manifest so tampering is detectable.

**ISC-04 (MUST).** Registered, not free-form. Interop sidecar `kind` identifiers live in the closed registry in this ADR (§"Registry — Initial entries"). New kinds are added by ADR revision or fresh ADR, not by operator choice. A Phase-1 verifier MUST reject unregistered kinds with failure `interop_sidecar_kind_unknown`.

**ISC-05 (MUST NOT).** Core crate dependency. `trellis-core`, `trellis-verify`, and `trellis-types` MUST NOT import any ecosystem library (VC, SCITT, DID, C2PA, JSON-LD processor, RDF canonicalizer). Interop adapters live in separate crates (`trellis-interop-scitt`, `trellis-interop-vc`, `trellis-interop-c2pa`, `trellis-interop-did`). Core §16 independence of verification requires this — an offline core-bytes verify on a laptop cannot pull in a multi-megabyte ecosystem dep tree. A `cargo-deny` (or equivalent) configuration MUST enumerate ecosystem libs as forbidden transitives for the core+verify workspace crates at the point implementation begins.

**ISC-06 (MUST).** Derivation version pin. Every sidecar entry carries a `derivation_version: uint .size 1` in the export manifest listing. A version change is wire-breaking for that kind. Verifiers MUST reject a sidecar whose `derivation_version` is not in the verifier's supported set with failure `interop_sidecar_derivation_version_unknown`.

**ISC-07 (SHOULD).** Semantic alignment over byte conformance for drafts. Where the target ecosystem format is still a moving draft (e.g., SCITT Architecture at the time of this ADR), the adapter SHOULD align semantics — field mapping, signing scope, receipt shape — without requiring strict byte conformance to a draft. Strict byte-level conformance is deferred per kind until the target format stabilizes, at which point the adapter's `derivation_version` bumps.

**ISC-08 (MUST).** Payload-disclosure honesty. A sidecar derivation MUST NOT exfiltrate material that the canonical record deliberately protected. For events whose payload is encrypted under Core §9.4, interop sidecars MUST NOT include the decrypted payload; they MAY include header-plaintext-declared fields (Core §12.2) and hashes of encrypted content. A deployment SHOULD NOT ship a sidecar kind for records where the kind's field mapping would expose protected material; that is an operator-level Posture-Declaration obligation (Companion §11.3 binding).

### Export bundle layout

```
<export.zip>/
  manifest.cbor                       ; Core §18 export manifest; adds `interop_sidecars`
  events/                             ; canonical event bytes
  checkpoints/                        ; canonical checkpoint bytes
  proofs/                             ; inclusion / consistency proofs
  key-registry/                       ; Core §8.5 registry snapshot
  interop-sidecars/                   ; OPTIONAL tree; see this ADR
    scitt-receipt/
      ckpt-<tree_size>.scitt-receipt.cbor
    vc-jose-cose-event/
      evt-<canonical_event_hash hex>.vc.cbor
    c2pa-manifest/
      cert-<certificate_id>.c2pa
    did-key-view/
      key-registry.did-key.json       ; one file, kid → did:key map
```

Absent `interop-sidecars/` tree is conformant. Present tree with zero entries is also conformant (the adapter chose not to emit any kind for this export). Mere presence of unknown files under `interop-sidecars/` not listed in the manifest's `interop_sidecars` catalog flips `integrity_verified = false` with failure `interop_sidecar_unlisted_file`.

### Export manifest reservation (Core §18)

Reserved now, Phase-1 population locked off:

```cddl
; Added to ExportManifest (Core §18) under ADR 0003 reservation discipline.
InteropSidecarEntry = {
  kind:                 "scitt-receipt" /
                        "vc-jose-cose-event" /
                        "c2pa-manifest" /
                        "did-key-view" /
                        tstr,                        ; tstr permits registry extension
  derivation_version:   uint .size 1,                ; pinned per kind; see §"Registry"
  path:                 tstr,                        ; relative to export root; MUST start with "interop-sidecars/"
  content_digest:       bstr .size 32,               ; SHA-256 under domain tag trellis-content-v1
  source_ref:           tstr,                        ; anchor back to the canonical record the sidecar derives from
  extensions:           { * tstr => any } / null,    ; future per-kind metadata
}

ExportManifestInteropSidecars = [* InteropSidecarEntry]
```

Phase-1 producers MUST emit `interop_sidecars` as `null` or `[]`. Phase-1 verifiers receiving a non-empty list MUST fail with `interop_sidecar_phase_1_locked` (ADR 0003 alignment).

## Registry — Initial entries

### `scitt-receipt` — Checkpoint + target event → SCITT COSE-Receipts

- **Target:** IETF SCITT Architecture (draft-ietf-scitt-architecture) + COSE Merkle Tree Proofs (draft-ietf-cose-merkle-tree-proofs).
- **Derives from:** One `Checkpoint` (Core §11.2) + one target `canonical_event_hash` for inclusion.
- **Derivation version:** `1` — semantic-alignment mode. Field mapping is stable; exact byte conformance to SCITT draft is not yet required. Bumps to `2` when SCITT reaches WG Last Call and a byte-conformance profile is adopted.
- **Field mapping (normative for `derivation_version = 1`):**

  | SCITT signed-statement field | Trellis source |
  |---|---|
  | issuer identity | Adapter-registered SCITT issuer (distinct from the checkpoint COSE signer; re-signs per SCITT convention) |
  | subject / artifact identifier | `canonical_event_hash` hex of the target event |
  | `iat` / timestamp | `CheckpointPayload.timestamp` |
  | log identifier | `CheckpointPayload.scope` passed through a `scope-to-log-id` registry (Phase-2 adapter config) |
  | tree size | `CheckpointPayload.tree_size` |
  | tree head hash | `CheckpointPayload.tree_head_hash` |
  | inclusion proof | Core §11.4 audit path, RFC-6962 ordering preserved |
  | receipt signature | SCITT-convention COSE_Sign1 over the signed statement, issuer key material managed by the operator's SCITT service |

- **Why re-sign rather than re-use the checkpoint signature:** SCITT receipts bind a different signed-statement structure than Trellis's `Sig_structure` for `CheckpointPayload`. Re-signing is cheap; trying to make one signature verify under both conventions is brittle and forces awkward per-signer-scope coupling.
- **Status:** Phase 1 — **locked off** (ISC-04 + ADR 0003). Phase 2+ — adapter crate `trellis-interop-scitt` implements. Trigger to unlock: SCITT Architecture draft reaches WG Last Call OR a concrete adopter needs SCITT-compatible receipts. Whichever fires first.

### `vc-jose-cose-event` — Event → W3C VC 2.0

- **Target:** W3C Verifiable Credentials v2.0 (Recommendation) via VC-JOSE-COSE (v1.0 Working Draft).
- **Derives from:** A canonical event (Core §6).
- **Derivation version:** `1`.
- **Output shape (illustrative, non-normative for this ADR; normative in the adapter spec at implementation):**

  ```json
  {
    "@context": [
      "https://www.w3.org/ns/credentials/v2",
      "<trellis-hosted, content-hashed context URI>"
    ],
    "type": ["VerifiableCredential", "TrellisEventCredential"],
    "issuer": "<issuer resolution per adapter config>",
    "validFrom": "<ISO-8601 derived from EventHeader.authored_at>",
    "credentialSubject": {
      "id": "trellis:event:<canonical_event_hash hex>",
      "canonical_event_hash": "<hex>",
      "suite_id": "<uint>",
      "kid": "<hex>",
      "event_type": "<EventHeader.event_type>",
      "classification": "<EventHeader.classification>",
      "retention_tier": "<uint>",
      "content_hash": "<hex, Core §9.3>"
    },
    "proof": { "type": "JOSE", "jws": "<COSE_Sign1 bytes, base64url>" }
  }
  ```

- **Payload-disclosure rule (ISC-08 binding):** `credentialSubject` carries **only** header-plaintext-declared fields per Core §12.2 plus content hashes. The adapter MUST NOT include decrypted payload material. A deployment MUST NOT ship this sidecar kind if the per-event mapping would expose protected material.
- **Status:** Phase 1 — locked off. Phase 2+ — adapter crate `trellis-interop-vc`. Trigger to unlock: an SSI-native adopter requires VC-native event envelopes.

### `c2pa-manifest` — Certificate of Completion → C2PA 2.x

- **Target:** C2PA (Coalition for Content Provenance and Authenticity) v2.x manifest, attached to the presentation artifact (PDF or HTML, per ADR 0007 `PresentationArtifact.media_type`).
- **Derives from:** ADR 0007 `CertificateOfCompletionPayload` + its bound `PresentationArtifact`.
- **Derivation version:** `1`.
- **Binding (normative for `derivation_version = 1`):** The C2PA manifest carries a Trellis-owned assertion (label `trellis.certificate-of-completion.v1`; formal C2PA registry submission is an implementation follow-on) pinning:

  | Assertion field | Value |
  |---|---|
  | `trellis.certificate_id` | ADR 0007 `CertificateOfCompletionPayload.certificate_id` |
  | `trellis.canonical_event_hash` | Canonical hash of the certificate-of-completion event |
  | `trellis.presentation_artifact.content_hash` | ADR 0007 `PresentationArtifact.content_hash` |
  | `trellis.kid` | Signer `kid` from the certificate event's COSE_Sign1 protected header |
  | `trellis.cose_sign1_ref` | SHA-256 (under `trellis-content-v1`) of the canonical COSE_Sign1 bytes of the certificate event |

  The manifest itself is signed under a C2PA-conventional signing method. Two verification paths are then independent and additive:

  1. **C2PA tooling path.** Open the PDF → read the C2PA manifest → verify per C2PA conventions → read the Trellis assertion and record the four pinned fields as claims about the Trellis binding.
  2. **Trellis core path.** Decode the canonical certificate event → verify per ADR 0007 verifier obligations → confirm the assertion values match the pinned Trellis fields.

  When both succeed, the presentation artifact has dual attestation. If only one succeeds, the verifier reports the partial coverage in `VerificationReport`; core attestation is load-bearing, C2PA attestation is additive.

- **Status:** Phase 2 — **active** (Wave 25, 2026-04-28). Adapter crate `trellis-interop-c2pa` implements `emit_c2pa_manifest_for_certificate` + `extract_trellis_assertion` + `TrellisAssertion::verify_against_canonical_chain`. Phase-1 verifier dispatches manifest-listed `c2pa-manifest@v1` entries to **path-(b) digest-binds only**: recompute `content_digest` against the on-disk sidecar file under `trellis-content-v1` and validate `path` / `kind` / `derivation_version` against the closed registry. **The Phase-1 core verifier does NOT resolve `source_ref` to the canonical event** (presence-only validation; cross-binding to the certificate event is a property of the C2PA-tooling path, not core). The C2PA-tooling-path consumer pattern (read manifest, extract assertion, verify five-field cross-check against `CanonicalChainContext`) is documented in `trellis-interop-c2pa/README.md`; that path layers additive evidence on top of the canonical-bytes verification per ISC-01. Other three kinds (`scitt-receipt`, `vc-jose-cose-event`, `did-key-view`) remain locked.

### `did-key-view` — Signing-key registry → `did:key` labeling view

- **Target:** W3C `did:key` method (v0.7).
- **Derives from:** Core §8 signing-class key registry entries (current `SigningKeyEntry`; post-ADR-0006 `KeyEntrySigning`).
- **Derivation version:** `1`.
- **Output shape:** A single JSON file mapping each registered signing-class `kid` to its `did:key` rendering under the Ed25519 multicodec encoding:

  ```json
  {
    "version": 1,
    "derivation_version": 1,
    "suite_id": 1,
    "entries": [
      { "kid": "<hex>", "did:key": "did:key:z6Mk..." }
    ]
  }
  ```

- **Semantics:** This is a **labeling view**, not a signing artifact. No signing occurs; no network resolution is required (the `did:key` IS the public key bytes under a multicodec wrapper). Verifiers that prefer DID vocabulary resolve `did:key` locally; verifiers that prefer Trellis vocabulary use `kid`. Both reach the same public key bytes. Offline verification is preserved trivially.
- **Scope:** Only the `kind = "signing"` arm per ADR 0006. Non-signing key classes (tenant-root, scope, subject, recovery) are out of scope for this kind; a future kind (e.g., `did-tenant-root-view`) may be added if an adopter asks.
- **Status:** Phase 1 — locked off. Phase 2+ — adapter crate `trellis-interop-did`. **Trigger to unlock: co-landing with ADR 0006 `KeyEntry` wire-shape migration.**

## Phase-1 verifier obligation

A conforming Phase-1 verifier processing an export bundle MUST:

1. If `interop-sidecars/` is absent and the manifest's `interop_sidecars` is `null` or `[]`, proceed with normal verification. Core bytes alone MUST yield `integrity_verified = true` on a valid export.
2. If `interop_sidecars` is non-empty OR files exist under `interop-sidecars/` that are not listed in the manifest, run dispatch in this order — each check fails closed on the first violation, the rest of the list still walks so multiple failures localize per-entry:
   - Listed `kind` values: check against the closed registry in this ADR. Unregistered kind → `interop_sidecar_kind_unknown`.
   - Listed `derivation_version` not in the verifier's supported set for that kind → `interop_sidecar_derivation_version_unknown`. (Wave 25 supports `c2pa-manifest@v1`.)
   - Listed `path`: MUST start with the byte prefix `interop-sidecars/`; otherwise → `interop_sidecar_path_invalid`.
   - Listed entries: verify each `content_digest` against the file bytes (SHA-256 under domain tag `trellis-content-v1`). Mismatch → `interop_sidecar_content_mismatch`.
   - Files under `interop-sidecars/` not listed in the manifest → `interop_sidecar_unlisted_file`.
   - Phase-1 lock-off: registered kinds **except `c2pa-manifest@v1`** are still locked off under ADR 0003 alignment; a present listed entry under any of the three locked kinds → `interop_sidecar_phase_1_locked`. The `c2pa-manifest@v1` kind passes through to `VerificationReport.interop_sidecars` with `phase_1_locked = false`.
3. Phase-1 core verifier path is **path-(b): digest-binds only.** It does NOT resolve `source_ref` to the canonical event, does NOT decode the C2PA manifest bytes, and does NOT depend on `c2pa-rs` (ISC-05; `c2pa-rs` is bound to `trellis-interop-c2pa` per `deny.toml`). The C2PA-tooling-path consumer is a separate verification path documented in `trellis-interop-c2pa/README.md`; consumers run that path additively, never as a replacement for canonical bytes (ISC-01).

`VerificationReport.interop_sidecars` is a new optional field, parallel to `posture_transitions` / `erasure_evidence` / `certificates_of_completion`, carrying per-entry outcomes:

```cddl
InteropSidecarVerificationEntry = {
  kind:                 tstr,
  path:                 tstr,
  derivation_version:   uint .size 1,
  content_digest_ok:    bool,
  kind_registered:      bool,
  phase_1_locked:       bool,
  failures:             [* tstr],
}
```

## Crate-hygiene contract

A Phase-1 `cargo test -p trellis-verify` build MUST NOT pull in any interop adapter crate, any JSON-LD processor, any RDF canonicalizer, any VC library, any SCITT library, any DID library, or any C2PA library. A `cargo-deny` (or equivalent) guard configuration MUST enumerate forbidden transitive deps for `trellis-core` / `trellis-verify` / `trellis-types` at the point Phase-1 verifier reservation work (step 2 of *Implementation sequencing*) executes.

The adapter crates, when implemented, MUST depend on `trellis-types` only (for canonical struct shapes); they MUST NOT depend on `trellis-core` or `trellis-verify` runtime code. This keeps the dependency graph one-way: Core → Types; Interop → Types; never Core → Interop.

## Phase alignment

- **Phase 1 envelope compatible.** Adds an `interop_sidecars` field to the export manifest (Core §18) under ADR 0003 reservation discipline. No event-envelope change. Phase-1 producers MUST emit `null` or `[]`; Phase-1 verifiers accept both. Invariant #10 preserved — Phase-1 envelope is still a strict subset of the Phase-3 event format.
- **Phase 2+ evolution.** Each kind unlocks per its trigger condition. Unlocking is spec-additive: the kind's Status flips from `Phase 1 locked` to `Phase 2 supported`; the adapter crate ships; the kind's fixture corpus (derivation-version correctness, round-trip byte-exactness, field-mapping invariants) lands with the adapter.
- **Phase 3 case-ledger composition.** Interop sidecars compose at the case-ledger head level identically to the Phase-1 envelope — each case-ledger head MAY carry its own `interop-sidecars/` tree, derived from that head's canonical bytes under the same discipline. Invariant #12 preserved.
- **Phase 4 federation.** Sidecars are derivable per-node. A federation member MAY ship its own interop sidecars without coordination as long as each derivation is from the same canonical record. Witness nodes are not obliged to emit sidecars; they verify core bytes.

## Fixture plan

Phase-1 corpus additions (reservation + lock-off proofs):

| Vector | Purpose |
|---|---|
| `export/011-interop-sidecars-absent` | Canonical positive: `interop-sidecars/` tree absent; manifest omits `interop_sidecars`. |
| `export/012-interop-sidecars-empty-list` | Canonical positive: `interop-sidecars/` tree present but empty; manifest has `interop_sidecars: []`. |
| `export/014-interop-sidecar-c2pa-manifest` | **Wave 25 positive (c2pa-manifest@v1).** Manifest lists one `c2pa-manifest@v1` entry; sidecar file present at the listed path; `content_digest` matches; verifier emits a populated `interop_sidecars` slice on `VerificationReport` with `content_digest_ok = kind_registered = true`, `phase_1_locked = false`. |
| `tamper/027-interop-sidecar-populated-phase-1` | Manifest lists a `scitt-receipt` entry in Phase-1 export; verifier fails with `interop_sidecar_phase_1_locked` (still locked under Wave 25). |
| `tamper/037-interop-sidecar-content-mismatch` | **Wave 25.** Sidecar file bytes altered relative to manifest `content_digest`; fails with `interop_sidecar_content_mismatch`. |
| `tamper/038-interop-sidecar-kind-unknown` | **Wave 25.** Manifest lists unregistered kind `"made-up-kind"`; fails with `interop_sidecar_kind_unknown`. |
| `tamper/039-interop-sidecar-unlisted-file` | **Wave 25.** `interop-sidecars/c2pa-manifest/stray.cbor` present, not listed in manifest; fails with `interop_sidecar_unlisted_file`. |
| `tamper/040-interop-sidecar-derivation-version-unknown` | **Wave 25.** Listed `derivation_version: 99` for a registered kind; fails with `interop_sidecar_derivation_version_unknown`. |

> **Slot reassignment note (Wave 25, 2026-04-28).** The Phase-1-reserved
> slots originally numbered 028..031 in this fixture plan were absorbed by
> the Wave 23 user-content-attestation corpus (tamper/028..034). The four
> dispatched-verifier interop fixtures relocate to slots 037..040 with
> kind-agnostic names (no `c2pa` substring in the slug) so future kinds can
> reuse the same negative scaffolding without slot churn.

Per-kind Phase-2+ vectors are deferred to per-adapter implementation; each adapter unlock lands its own `append/` / `export/` / `tamper/` fixtures covering derivation-version round-trip, byte-exact field mapping, and per-kind failure modes.

## Open questions / follow-ons

1. **SCITT issuer-key provenance.** The `scitt-receipt` adapter re-signs with a distinct SCITT-issuer key (not the checkpoint signer). Whether that key MUST be registered in the Trellis signing-key registry (ADR 0006 `KeyEntry`) or in an adapter-local registry is deferred; resolved in the adapter's own landing ADR.
2. **VC `@context` hosting.** The Trellis-hosted VC context URI must resolve to a content-hashed JSON-LD document at a stable address. Hosting location, content-hash commitment, and long-term URL stability policy are deferred to the VC-adapter implementation.
3. **C2PA assertion label registration.** ~~Deferred to C2PA-adapter implementation.~~ **Resolved Wave 25 (2026-04-28):** the adapter ships under the **vendor-prefix label `org.formspec.trellis.certificate-of-completion.v1`** per C2PA assertion-naming conventions for vendor-defined assertions. Vendor-prefix is the load-bearing rationale: it sidesteps the C2PA-coalition membership-and-registration cycle (multi-month, governance-gated) without forfeiting interop with the C2PA-tooling path; consumers parse the assertion the same way regardless of label namespace. A formal short-form registration with the C2PA assertion registry remains a follow-on deferred to a coalition-membership ADR; that ADR will rename the label and bump the kind's `derivation_version` per ISC-06.
4. **`did:web` for agency keys.** Excluded from the initial registry because resolution requires network access, which breaks Core §16 offline-verification independence. A future `did-web-view` kind could be added with an explicit "verifier treats DID as opaque string offline; MAY dereference only when online; verification outcome MUST NOT depend on resolution success" rule. Not blocking.
5. **Ecosystem-library avoidance for Phase-1 verifier reservation work.** Step 2 of *Implementation sequencing* adds manifest-level reservation to `trellis-core` and `trellis-verify`. Even that work MUST NOT pull in ecosystem libs — the registry is a closed set of string literals, digest checks are plain SHA-256, and the verifier rejects all populated entries in Phase 1. No ecosystem dep is required until a kind actually unlocks. *Wave 25 status: `c2pa-manifest@v1` activates per path-(b); `trellis-verify` remains free of `c2pa-rs` (the dep lives only in `trellis-interop-c2pa`); ISC-05 holds.* **`source_ref` resolution semantics deferred to a future ADR.** The Phase-1 verifier under Wave 25 validates `source_ref` for **presence only** — a non-empty `tstr`, no schema enforcement, no canonical-event-hash binding, no cross-resolution into the chain. Full `source_ref` resolution semantics (URI scheme, canonical-event-hash binding shape, cross-resolution into the certificate event) are a separable design question that benefits from being decided once across all four kinds (each derives from a different canonical anchor: checkpoint+event-hash for SCITT, event for VC, certificate-event for C2PA, key-registry for DID); deferred to a later ADR landing alongside the second kind that activates.

## Cross-references

- **Core §11 Checkpoint Format** — SCITT-receipt derivation substrate. A brief non-normative §11.7 pointer to this ADR lands with this ADR's acceptance (no wire change).
- **Core §16 Independence of Verification** — crate-hygiene contract (ISC-05) enforces this.
- **Core §18 Export Package Layout** — reserves `interop_sidecars: [* InteropSidecarEntry] / null` per ADR 0003 discipline; Phase-1 producers emit null or empty; Phase-1 verifiers reject populated entries with `interop_sidecar_phase_1_locked`.
- **Companion §14 Derived-Artifact Discipline** — parent discipline for ISC-01 (canonical-first); interop sidecars are a registered subclass of derived artifacts with additional obligations.
- **Companion §11 Posture-Declaration Honesty** — ISC-08 (payload-disclosure honesty) binds here; operators declare per-kind emission posture in the Posture Declaration.
- **ADR 0001–0004** — reservation-with-lockoff pattern applied to export-manifest level.
- **ADR 0005** — wire-shape-first-then-implementation pattern (same shape followed here).
- **ADR 0006** — `did-key-view` sidecar depends on the `KeyEntry` migration.
- **ADR 0007** — `c2pa-manifest` sidecar is the registered binding for the certificate's presentation artifact; co-lands with ADR 0007 execution.
- **STACK.md** — interop sidecars don't cross any of the five cross-layer contracts; they re-express existing canonical claims in external vocabularies.
- **[`thoughts/specs/2026-04-24-anchor-substrate-spike.md`](../specs/2026-04-24-anchor-substrate-spike.md)** — SCITT-receipt-via-sidecar is the export-bundle-visible counterpart to the `AnchorAdapter` DI stance. An anchor substrate produces Phase-2+ receipts; the `scitt-receipt` sidecar is how those receipts travel in the export bundle.

## Implementation sequencing

1. **Spec** — this ADR; one-paragraph non-normative §11.7 pointer in Core; one-line tag update in ADR 0007; TODO.md entry consolidating per-kind triggers. **(Closed by this change.)**
2. **Core §18 export-manifest reservation** — add `interop_sidecars` field with the CDDL above; add Phase-1 lint requiring null-or-empty; add Phase-1 verifier reservation-check behavior (the eight failure codes in §"Phase-1 verifier obligation"). Lands with the export/011, export/012, tamper/027..031 fixture batch.
3. **Phase-1 fixture corpus additions** — `export/011`, `export/012`, `tamper/027..031` per *Fixture plan*.
4. **Python stranger mirror** — `trellis-py` updates to decode + reject populated entries identically.
5. **Adapter-crate hygiene scaffolding** — create empty crates `trellis-interop-scitt`, `trellis-interop-vc`, `trellis-interop-c2pa`, `trellis-interop-did` with no adapter logic; add `cargo-deny` config forbidding ecosystem libs from `trellis-core` / `trellis-verify` / `trellis-types`. Locks the hygiene contract before any adapter lands.
6. **Per-kind adapter implementation** — deferred per-trigger. Each kind's adapter ships with its own landing ADR (or a revision of this one) that names the adapter crate's concrete wire, byte-exact derivation rules, and adapter-scope fixture corpus.

Steps 1 is closed by this change. Steps 2–5 are the Phase-1 implementation scope and co-execute when the wire reservation lands. Step 6 fires per-kind on its stated trigger.

---

*End of ADR 0008.*
