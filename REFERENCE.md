# Trellis — document outline reference

Per-file inventories of **H1** (`#`) and **H2** (`##`) headings with one-line summaries. Use this for navigation and grep alignment; narrative and relationships are in [`README.md`](README.md).

Paths are relative to `trellis/`.

---

## `specs/core/trellis-core.md`

**H1 — Trellis Core Specification**  
W3C-style constitutional core draft defining append-attested canonical semantics, invariants, verification requirements, and cross-repo authority boundaries.

| H2 | Summary |
|----|---------|
| Status of This Document | Declares draft status and W3C-style structural intent. |
| Abstract | Summarizes constitutional scope and explicit out-of-scope exclusions. |
| Table of Contents | Lists the normative section structure for the core specification. |
| 1. Introduction | Defines scope and design goal. |
| 2. Conformance | Defines RFC2119 interpretation and role requirements. |
| 3. Terminology | Defines core semantic terms used normatively. |
| 4. Core Model | Defines object classes and ontology discipline. |
| 5. Canonical Truth and Invariants | Defines canonical boundary and named invariants. |
| 6. Canonical Admission and Order | Defines admission, order, and idempotency/rejection semantics. |
| 7. Canonical Hash Construction | Defines single authoritative hash construction requirement. |
| 8. Verification Requirements | Defines verifier obligations independent of runtime state. |
| 9. Cross-Repository Authority Boundaries | Reasserts Trellis/Formspec/WOS authority boundaries. |
| 10. Security and Privacy Considerations | Captures metadata minimization and trust-honesty baseline. |

---

## `specs/README.md`

**H1 — Trellis Specs (Draft Family)**  
Index file for the normalized spec family with dependency order, ownership boundaries, maturity markers, and next extraction passes.

| H2 | Summary |
|----|---------|
| Layout | Maps each topical folder under `specs/` (`core`, `trust`, `projection`, `export`, `operations`, `assurance`) to its role. |
| Normative dependency order (draft) | Lists the intended review sequence across `specs/`; ratification process docs are linked from `../ratification/`. |
| Ownership boundaries (draft) | States Trellis vs Formspec vs WOS authority boundaries. |
| Current maturity markers | Gives quick status labels for each spec draft. |
| Immediate next extraction passes | Lists short-term migration work from legacy omnibus drafts into focused specs. |
| Phase status | Notes ratification links and open auto-evidence gates. |

---

## `specs/core/shared-ledger-binding.md`

**H1 — Trellis Companion — Shared Ledger Binding (Draft)**  
Companion draft for family bindings and canonization discipline across Formspec/WOS/trust/release facts.

| H2 | Summary |
|----|---------|
| Status | Marks extraction from legacy companion omnibus draft. |
| Purpose | Binds fact families into one canonical substrate without reinterpreting source semantics. |
| Normative Focus | Family IDs, required fields, schema/version rules, canonization eligibility, conformance matrix. |
| Family binding matrix (draft scaffold) | Provides initial family-to-authority/required-field mapping scaffold. |
| Canonization rules (draft) | Adds baseline acceptance/rejection and no-competing-order rules. |
| Schema/version compatibility policy (draft) | Defines additive vs breaking change compatibility expectations. |
| Canonization rejection codes (draft) | Enumerates canonical rejection code scaffold for machine testing. |
| Deferral Rules | Reaffirms Formspec and WOS authority boundaries. |
| Core handoff note | Clarifies core admission semantics vs binding-owned schema/version policy. |
| Migrated requirements from `unified_ledger_core.md` (Section 15) | Captures binding-side extracted requirements from legacy core domain-binding sections. |

---

## `specs/trust/trust-profiles.md`

**H1 — Trellis Companion — Trust Profiles (Draft)**  
Companion draft for custody/readability posture declarations and trust-honesty semantics.

| H2 | Summary |
|----|---------|
| Status | Marks extraction from legacy companion omnibus draft. |
| Purpose | Defines explicit trust postures and mandatory profile declarations. |
| Baseline Profiles | Reader-held default, provider-readable, tenant-operated key plane. |
| Metadata Budget Requirement | Requires per-fact-family visibility/leakage/delegation declarations. |
| Trust honesty rule (draft) | Requires claims to match operational decryptability/recovery/delegation realities. |
| Profile declaration schema (draft) | Lists required machine-readable trust profile declaration fields. |
| Conformance audit hooks (draft) | Adds auditability expectations for trust declaration enforcement. |
| Operational trust disclosure requirements (draft) | Moves explicit decryptability/delegation/recovery/destruction disclosure obligations into trust companion scope. |
| Migrated requirements from `unified_ledger_core.md` (Sections 10, 11, 14) | Captures trust-profile, transition, and profile-export requirements extracted from legacy core. |

---

## `specs/trust/key-lifecycle-operating-model.md`

**H1 — Trellis Companion — Key Lifecycle Operating Model (Draft)**  
Companion draft for lifecycle state machine semantics, rotation, grace periods, recovery, and destruction completeness.

| H2 | Summary |
|----|---------|
| Status | Marks extraction from legacy companion omnibus draft. |
| Purpose | Treats key lifecycle as first-class behavior. |
| Normative Focus | Key classes, transitions, rotation, grace, recovery, shredding, historical verification. |
| Key classes (draft) | Defines draft key class taxonomy for policy/lifecycle handling. |
| Lifecycle states (draft) | Introduces draft state machine and allowed transitions. |
| Grace-period rule (draft) | Adds explicit offline-client rotation/grace behavior requirements. |
| Required Completeness Rule | States purge-cascade requirement for plaintext-derived projections/caches. |
| Recovery and destruction evidence requirements (draft) | Adds required evidence outputs for recovery/destruction operations. |
| Migrated requirements from `unified_ledger_core.md` (Section 16.5) | Captures lifecycle and cryptographic inaccessibility requirements from legacy core. |

---

## `specs/projection/projection-runtime-discipline.md`

**H1 — Trellis Companion — Projection and Runtime Discipline (Draft)**  
Companion draft for derived-system discipline, provenance watermarking, rebuildability, snapshot boundaries, and runtime deferral seams.

| H2 | Summary |
|----|---------|
| Status | Marks draft startup from the normalization sequence. |
| Purpose | Prevents derived systems from becoming hidden canonical truth. |
| Normative Focus | Watermarking, rebuild contract, snapshot discipline, purge cascades, runtime boundary. |
| Projection watermark contract (draft) | Defines required canonical checkpoint/build metadata for staff-facing projections. |
| Rebuild verification (draft) | Adds semantic equivalence expectations for projection rebuilds. |
| Deferral to WOS | Defers execution semantics and runtime envelope specifics to WOS. |
| Migrated requirements from `unified_ledger_core.md` (Sections 16.1 and 16.4) | Captures derived artifact and snapshot discipline requirements from legacy core. |

---

## `specs/export/export-verification-package.md`

**H1 — Trellis Companion — Export Verification Package (Draft)**  
Companion draft for offline-verifiable package composition and verification obligations.

| H2 | Summary |
|----|---------|
| Status | Marks draft startup from normalization sequence. |
| Purpose | Defines package semantics for offline integrity verification. |
| Normative Focus | Required members, readability declaration, trust-profile carriage, offline verifier behavior, anchoring seam. |
| Verification manifest minimum fields (draft) | Defines required manifest fields for portable offline verification. |
| Cross-implementation verification requirement (draft) | Adds expectation for equivalent outcomes across independent verifiers. |
| Provenance distinction requirement (draft) | Requires export packages to preserve canonical vs derived artifact distinctions. |
| Migrated requirements from `unified_ledger_core.md` (Section 12) | Captures export/verification guarantees extracted from legacy core. |

---

## `specs/export/disclosure-manifest.md`

**H1 — Trellis Companion — Disclosure Manifest (Draft)**  
Companion draft for audience-specific disclosure manifests as first-class release artifacts.

| H2 | Summary |
|----|---------|
| Status | Marks draft startup from normalization sequence. |
| Purpose | Separates disclosure semantics from canonical append records. |
| Normative Focus | Audience scope, claim classes, provenance preservation, selective disclosure discipline, readability/redaction declarations. |
| Claim-class taxonomy (draft) | Defines disclosure claim classes and canonical-reference rule. |
| Interop Direction | Prioritizes SD-JWT / VC path; defers advanced mechanisms. |

---

## `specs/operations/monitoring-witnessing.md`

**H1 — Trellis Companion — Monitoring and Witnessing (Draft)**  
Minimal companion draft defining publication/verification seams for independent monitoring and witness compatibility.

| H2 | Summary |
|----|---------|
| Status | Marks minimal seam-oriented startup. |
| Purpose | Creates anti-equivocation-ready seam without forcing a witness network design now. |
| Normative Focus (minimal for now) | Checkpoint publication, append-growth verification, anti-equivocation-compatible publication, verifier interop targets. |
| Testability hooks (draft) | Adds fixture/replay expectations for monitor verification seams. |
| Current Scope Constraint | Defers concrete witness topology choices. |

---

## `specs/assurance/assurance-traceability.md`

**H1 — Trellis Companion — Assurance Traceability (Draft)**  
Companion draft that binds core invariants to concrete assurance methods and expected evidence artifacts.

| H2 | Summary |
|----|---------|
| Status | Marks assurance traceability as active spec-family work. |
| Purpose | Keeps assurance architectural and testable rather than appendix prose. |
| Traceability matrix (draft) | Maps invariants to TLA+/Alloy/property/fuzz/drill methods and outputs. |
| Minimum CI expectations (draft) | Sets baseline automation and evidence-retention expectations. |
| Evidence retention policy (draft) | Adds retention and remediation linkage expectations for assurance artifacts. |

---

## `ratification/README.md`

**H1 — Trellis ratification (draft)**  
Pointer to ratification checklist and evidence alongside the `specs/` tree.

| H2 | Summary |
|----|---------|
| *(body)* | Explains these are process gates, not protocol specs, with links to checklist and evidence. |

---

## `ratification/ratification-checklist.md`

**H1 — Trellis Spec Family Ratification Checklist (Draft)**  
Cross-document readiness checklist defining global and per-spec gates for moving drafts toward normative ratification.

| H2 | Summary |
|----|---------|
| Purpose | Defines stopping conditions and readiness gates for ratification. |
| Global gates | Lists family-wide requirements (boundaries, traceability, vectors, verifier reproducibility). |
| Per-document readiness gates | Provides file-specific ratification checks for each core/companion draft. |
| Natural stopping point for this extraction phase | Defines completion criteria for the current drafting phase. |

---

## `ratification/ratification-evidence.md`

**H1 — Trellis Ratification Evidence Registry (Draft)**  
Evidence-linked registry mapping checklist gates to current proof artifacts and identifying remaining automated-evidence gaps.

| H2 | Summary |
|----|---------|
| Purpose | Provides evidence references for each checklist gate. |
| Evidence status key | Defines `PROSE` vs `PENDING-AUTO` evidence states. |
| Global gate evidence | Tracks evidence IDs for global gates and outstanding automation gaps. |
| Per-document gate evidence | Tracks evidence IDs for each document-specific readiness gate. |
| Follow-up required to move PENDING-AUTO to complete | Lists remaining automation tasks required for full closure. |

---

## `DRAFTS/trellis_spec_family_normalization_plan.md`

**H1 — Trellis Spec Family Normalization Plan (Convergence Pass)**  
Decision-oriented convergence plan that validates the core/companion split, answers boundary questions directly, and maps current Trellis materials into normative companions, profile sidecars, and rationale.

| H2 | Summary |
|----|---------|
| 1) Assessment of the proposed split | Confirms the split direction and directly answers key structural questions (core size, trust/key split, projection deferral, export/disclosure split, monitoring companion). |
| 2) Recommended final Trellis spec family | Defines final normative companion set, priority order, profile/sidecar artifacts, and rationale set. |
| 3) Mapping from current Trellis docs into the new family | Maps both major clusters and each current file into keep/split/demote actions and target homes. |
| 4) Explicit repo-boundary decisions (Trellis vs WOS vs Formspec) | States ownership, binding duties, and non-redefinition boundaries across repos. |
| 5) Top 5 drafting priorities | Lists the highest-leverage drafting actions for reducing semantic debt and preserving hard conclusions. |
| 5.1 Immediate repo normalization sequence (recommended) | Provides an ordered execution sequence for migrating from current draft layout to the target family structure. |
| 6) Anything structurally wrong in the current draft | Calls out overloaded companion and boundary/assurance gaps. |
| 7) Technical conclusions still not captured strongly enough | Highlights missing explicitness for idempotency, snapshots, assurance traceability, metadata budget tables, and Phase-1 profile. |
| 8) What the current canvas-style restructuring got right vs what should still change | Separates confirmed wins from remaining structural refinements. |

---

## `thoughts/specs/2026-04-10-unified-ledger-concrete-proposal.md`

**H1 — Unified Ledger: Concrete Proposal**  
Dated technical proposal to “build the real thing”: extends ADR-0059 with identity, events, crypto, storage, sync, projections, disclosure, export, and crate layout (not phased stubs).

| H2 | Summary |
|----|---------|
| Principles | Browser-origin ledger, Rust+WASM, content-addressed ciphertext, immutable grants, server as processor not owner. |
| 1. Identity & Key Management | OIDC + WebAuthn PRF, TMK/LAK/DEK, DIDs/VCs, signing registry, recovery/re-grant. |
| 2. Event Data Model | Author-event envelope v2 (HLC, commitments, causal deps); server receipts and merge semantics. |
| 3. Cryptographic Stack | Algorithms, crates, bindings for signing, encryption, hashing. |
| 3b. Key Rotation Protocol | Rotation across tenant/ledger/respondent layers. |
| 4. Storage Topology | Ciphertext blobs, ledger DB, KMS, Postgres/object-store options. |
| 5. Client Architecture | Browser/WASM responsibilities, caching, keys, sync. |
| 6. Server Architecture | Ingest, sequencing, governance events, checkpoints. |
| 7. Sync Protocol | Offline/multi-writer: DAG/HLC, merge frontier, conflicts, ordering. |
| 8. Coprocessor Transition | Execution path toward coprocessor model. |
| 8b. WOS Provenance Integration | Ledger events ↔ WOS provenance kinds. |
| 9. Materialized Views & Projections | CQRS projections, rebuild, snapshots, performance. |
| 10. Selective Disclosure & Permissioned Sharing | BBS+/SD-JWT-style disclosure and grants. |
| 11. Export Artifact | Deterministic export bundle and offline verification. |
| 12. Degraded Modes | Partial failure, stale checkpoints. |
| 13. Rust Crate Layout | Native + WASM crate boundaries. |
| 14. What This Does NOT Include | Explicit out-of-scope list. |
| 15. What We Build | Consolidated build scope. |
| 16. Event Type Registry (Draft) | Draft event kind registry. |
| 17. Error Model | Error taxonomy and handling. |
| 18. Envelope and Receipt Versioning | Versioning rules. |
| 19. Pre-Implementation Spikes | Spikes (dCBOR, PRF, HPKE, etc.) before full build. |

---

## `thoughts/research/ledger-risk-reduction.md`

**H1 — Reducing Custom Risk in a Privacy-Preserving, Event-Sourced, Cryptographically Verifiable Ledger Workflow System**  
Reframes the concrete proposal toward mature building blocks (transparency-log style, COSE, SD-JWT, authz engines, Temporal, formal methods) and flags where custom design is justified vs dangerous.

| H2 | Summary |
|----|---------|
| Executive synthesis | Ten substitutions, five risky custom areas, five keep-custom areas, three reframes. |
| Critical read of the proposal through the five lenses | Spec vs rhetoric, distributed systems, reliability, privacy/crypto, maintainability, compliance. |
| Evidence-based substitutions and composable building blocks by layer | Keep/Adopt/Compose by layer: log core, crypto, identity/authz, workflow, storage, verification. |
| Proposed safer target architecture summary | CT-style ordering, projections + watermarks, Temporal as orchestration, SD-JWT-first disclosure. |
| Keep / Adopt / Compose / Avoid matrix and ranked subsystem recommendations | Comparison matrix; ranked picks (incl. QLDB avoid). |
| Methodologies, assurance practices, reference stacks, red flags, and decision rubric | FM, fuzzing, threat modeling, boring vs ambitious stacks, procurement rubric. |

---

## `trellis/thoughts/reviews/2026-04-10-expert-panel-unified-ledger-review.md`

**H1 — Expert Panel Review: Unified Ledger Architecture**  
Four independent reviews of ADR-0059 and the technology survey: consensus themes, fixes, ideal “proof replaces trust” vision, concrete architectures, Phase 1 vs later.

| H2 | Summary |
|----|---------|
| Round 1: Critical Review | Unanimous themes; critical issues table (PRK derivation, canonicalization, idempotency, snapshots, rotation, UX). |
| Round 2: Ideal End-State Vision | Trust replaced by proof for respondent, government, auditors, cross-agency. |
| Round 3: Concrete Technical Architectures | Merkle/export; TFHE eligibility; dual-mode projections + Temporal; FedRAMP three-plane deployment. |
| Synthesis: What to Build Now vs. What to Design For | Phase 1 (Postgres, CBOR, Pedersen, KMS, Temporal, idempotent append, shredding) vs deferred. |
| Sources | Citations and links. |

---

## `thoughts/research/2026-04-10-unified-ledger-technology-survey.md`

**H1 — Unified Ledger Technology Survey**  
Dated survey mapping OSS/managed options to ADR-0059’s seven areas; recommended composed stack, phased rollout, risk table.

| H2 | Summary |
|----|---------|
| Executive Summary | Capability → mature option; VC 2.0 / BBS+ / eIDAS timing; summary tech table. |
| 1. Immutable Storage with Cryptographic Verification | immudb, Trillian/Tessera, SQL Ledger, Postgres DIY; Phase 1 vs production Merkle path. |
| 2. Event and Checkpoint Signing | COSE/`coset` vs JWS; Ed25519 + COSE checkpoints. |
| 3. Selective Disclosure via BBS+ Signatures | Standards status, Rust crates, procurement caveats. |
| 4. External Anchoring | OpenTimestamps vs Rekor; using both for different verification modes. |
| 5. Key Management and Crypto-Shredding | Vault Transit vs cloud KMS; EDPB; per-respondent destruction; tiered KMS. |
| 6. Decentralized Identity | VC 2.0, DIDs, eIDAS/mDL signals, OIDC adapters (Login.gov/ID.me). |
| 7. Merkle Tree Implementations (Rust) | `ct_merkle`, `rs_merkle`, vs immudb built-in. |
| 8. Government Compliance Landscape | FedRAMP logging, EDPB shredding, FRE 803(6) angle. |
| 9. Technology Risk Assessment | Risks/mitigations for immudb, BBS+, Rekor, Vault, VCs, OTS. |
| 10. Recommended Architecture | Layered diagram; Phase 1 MVP vs Phase 2+ triggers. |
| Sources | Bibliography by topic. |

---

## `thoughts/formspec/specs/respondent-ledger-spec.md`

**H1 — Respondent Ledger Add-On Specification v0.1**  
Normative v0.1: optional respondent-facing append-only audit ledger on Formspec core without replacing `Response`.

| H2 | Summary |
|----|---------|
| 1. Purpose | Role of add-on; questions answered; non-replacement of `Response`. |
| 2. Relationship to Formspec core | Definition / Response / Ledger layers; conformance; `extensions` pointer. |
| 3. Design goals and non-goals | Optional, append-only, material, path-native, portable, integrity-ready; no keystroke telemetry. |
| 4. Core model | One ledger per `responseId`; canonical object types incl. `LedgerCheckpoint`. |
| 5. RespondentLedger object | Header fields, semantics, JSON example. |
| 6. RespondentLedgerEvent object | Fields, `actor` / `source` / `identityAttestation`, tiered privacy. |
| 7. ChangeSetEntry object | Atomic change: `op`, `valueClass`, paths, sensitive-value rules. |
| 8. Event taxonomy | Required/optional `eventType` values; explicit exclusions. |
| 9. Materiality rules | Material changes only; autosave coalescing. |
| 10. Interaction with Formspec response semantics | Pinning, status, validation snapshots, non-relevance, calculated, prepop. |
| 11. Amendments, migration, and version evolution | Amendment events; migration preserves history; changelog refs. |
| 12. Storage and retention model | Separate storage; append-only; retention/redaction. |
| 13. Integrity checkpoints | `LedgerCheckpoint`; hash chaining / anchoring. |
| 14. Recommended JSON shape | Illustrative full JSON. |
| 15. Implementation guidance | Timeline UX, diffs, support/dispute, `extensions`. |
| 15A. Recommended deployment profiles | Profiles A/B/C: local → pseudonymous anchored → identity-bound. |
| 16. Conformance summary | MUST/SHOULD checklist. |
| 17. Open follow-on work | Fixtures, canonicalization, schemas. |

---

## `thoughts/formspec/adrs/0054-privacy-preserving-client-server-ledger-chain.md`

**H1 — ADR-0054: Privacy-Preserving Client/Server Ledger Chain for the Formspec Ecosystem**  
Client capture → server audit → verification/export with tiered crypto (zk/MPC/HE) and provider-neutral identity.

| H2 | Summary |
|----|---------|
| Title | Scope: eight numbered capability themes for the ledger chain. |
| Context | vs platform audit + respondent-ledger spec; prior ADRs. |
| Problem Statement | History, append-only audit, integrity, privacy, proofs, practicality, identity. |
| Decision Drivers | E2E trust, privacy-by-architecture, multi-party assurance, portability. |
| Assumptions | `Response` canonical; ledger additive; clients untrusted; crypto optional by tier. |
| Considered Options | A–C: server-only vs full crypto parity vs layered chain; choice of C. |
| Decision | Four-layer trust chain; encryption, zk, MPC, HE, DIDs/PoP rules. |
| Canonical Architecture | Bridge into architecture subsections. |
| 1. Respondent client implementation | Local ledger, envelope encryption, attestation, sync payloads. |
| 2. Server authoritative implementation | Validation, sequencing, conflicts, encryption, checkpoints, proofs. |
| Identity, Proof of Personhood, and DID Architecture | Bridge for identity subsections. |
| 3A. Identity layer goal | PoP, delegation, DIDs, wallets, providers. |
| 3A.1 Decoupled planes | Response vs audit vs identity; anonymous/pseudonymous paths. |
| 3B. Tiered privacy model | `assuranceLevel` / `privacyTier`; TPIF-inspired framing. |
| 3C. Canonical identity evidence model | Provider-neutral attestation field set. |
| 3D. DID compatibility | What to record; ledger not a DID registry. |
| 3E. Proof of Personhood integration | When PoP is material; event types. |
| 3F. Adapter boundary for providers such as ID.me | Adapters vs native shapes. |
| 3. Ecosystem chain model | Stages from client segment to export/anchor. |
| Cryptographic Capability Rules | Bridge for crypto subsections. |
| 4. Encryption | Mandatory encryption; custody; selective disclosure building blocks. |
| 5. zkSNARKs | Narrow approved uses; not default hot path. |
| 6. Multi-party computation (MPC) | Split-control; optional by tier. |
| 7. Homomorphic encryption (HE) | Scoped aggregates/analytics only. |
| Trust Profiles by Deployment Tier | Bridge for tier subsections. |
| 8. Shared Cloud | Baseline encryption, checkpoints, exports; optional attestation. |
| 9. Regulated Cloud | Stronger keys, attestation, zk exports, threshold/MPC. |
| 10. Dedicated / Private Instance | Customer roots, MPC, external notarization, residency. |
| Client/Server Data Flow | Bridge for happy/conflict paths. |
| 11. Happy path | Local edit → server → checkpoints → audit → export. |
| 12. Conflict / merge path | Server arbiter; merge events; client-claimed vs server-accepted. |
| Data Model Implications | Encrypted refs, commitments, proofs, DIDs, threshold metadata. |
| Security and Privacy Consequences | Bridge for consequence lists. |
| Positive consequences | Trust, continuity, selective disclosure, regulated fit. |
| Negative consequences | Complexity, key/proof burden, tooling. |
| Rejected Alternatives | Four rejected patterns (UX-only history, server-only no proofs, universal heavy crypto, public chain). |
| Configuration Profiles | Profiles 1–3; stable event semantics. |
| Rollout Plan | Phases 1–5 toward HE analytics. |
| Acceptance Criteria | Numbered satisfaction checklist. |
| Open Questions | Proof standards, tier mandates, attestation cost, MPC/HE placement. |
| Relationship to Other ADRs | Extends 0003, 0007, 0009, 0012, 0013, 0015. |
| Final Rationale | Layered crypto: ambitious but not on every save. |

---

## `thoughts/formspec/proposals/user-side-audit-ledger-add-on-proposal.md`

**H1 — User-Side Audit Change Tracking Ledger Add-On Proposal**  
Optional respondent-facing append-only ledger companion; aligns with platform audit direction without bloating core `Response`.

| H2 | Summary |
|----|---------|
| 1. Executive summary | Optional ledger; Definition + Response stay clean; companion spec. |
| 2. What the existing Formspec design already gives us | Pinning, lifecycle, hooks, changelog, ADRs → parallel ledger not event-sourced Response. |
| 3. Problem to solve on the user side | Diffs, provenance, attachments, validation, appeals; no keystroke surveillance. |
| 4. Design principles | Optional; material events; path-native; human-first then crypto; tiered identity. |
| 5. Proposed architecture | Intro to 5.1 / 5.2. |
| 5.1 Add a companion spec | Paths, `$formspecRespondentLedger`, additive packaging. |
| 5.2 Keep three layers distinct | Definition, Response snapshot, Respondent ledger. |
| 6. Canonical objects in the add-on | Four object types. |
| 6.1 `RespondentLedger` | Header fields. |
| 6.2 `RespondentLedgerEvent` | Event fields, hashes, validation hooks. |
| 6.3 `ChangeSetEntry` | Atomic change fields and enums. |
| 6.4 `LedgerCheckpoint` | Sealing aligned with platform audit. |
| 7. Event model focused on respondents | Required/optional types; non-goals. |
| 8. How it reaches down into the bottom of the spec | Pinning, paths, migrations, non-relevance, prepop, three-plane decoupling, identity adapters. |
| 9. Recommended storage model | Bridge for 9.1–9.3. |
| 9.1 Store snapshot and ledger separately | Logical separation; submission canonical. |
| 9.2 Use event coalescing for autosave | Coalesce to save boundaries. |
| 9.3 Separate value retention from proof retention | Hashes/redaction vs raw values. |
| 10. Proposed schema shape | Minimal JSON example. |
| 11. UX expectations for elegance | Timeline, plain language, progressive disclosure. |
| 12. Integration with the platform audit ledger | Feeds ADR-0003 material audit/checkpoints. |
| 13. Rollout plan | Phases 1–4. |
| 13A. Where configuration should differ | Stable events; varying policy by profile. |
| 14. Recommended boundaries | vs studio/analytics/support ledgers. |
| 15. Concrete implementation recommendation | Extension pointer; standalone schemas; coalescing; bridge checkpoints. |
| 16. Why this is the right fit | Engine/instance/platform separation; optional integrity. |

---

## `trellis/thoughts/reviews/2026-04-11-crypto-expert-concrete-solutions.md`

**H1 — Concrete Solutions for All 13 Findings**  
Rust-oriented crypto/protocol fixes for expert findings plus gaps; closes with consolidated **Header V2**.

| H2 | Summary |
|----|---------|
| FINDING 1: Multi-device causal ordering | HLC + causal deps, server topo sort, conflict policies for overlapping edits. |
| FINDING 2: Key rotation (complete protocol) | Lazy PRK re-wrap, BBS+/TMK versioning, tenant pubkey rotation, session lifecycle. |
| FINDING 3: Crate layout consolidation | Single `ledger-engine` + typestate `EventBuilder` as sole construction API. |
| FINDING 4: Tenant public key rotation grace period | Versioned keys, grace/re-wrap, hard reject on revoke only. |
| FINDING 5: Pedersen commitment fixed-position vector | Per-event-type schemas; commitments-to-zero for unused fields. |
| FINDING 6: Header tags — hash commitment | `tag_commitment`; tags in ciphertext to avoid header leakage (HIPAA-oriented). |
| FINDING 7: Self-correction (editorial) | Doc-only cleanup of self-referential ADR prose. |
| FINDING 8: Event granularity — batch at draft boundaries | Default `DraftSession` batching; optional finer granularity. |
| MISSED 1: AES-256-GCM nonce management — ECIES ephemeral keys | Per-wrap ephemeral X25519; safe GCM nonces. |
| MISSED 2: BBS+ / key bag grant asymmetry documentation | Three-tier access + `DisclosurePolicy`. |
| MISSED 3: WebAuthn PRF salt management | Server-stored salt, HKDF info versioning, recovery via new credential. |
| MISSED 4: Canonical hash construction | Unified `payload_content_id` vs `event_hash` domain separation. |
| MISSED 5: Crypto-shredding incomplete for GDPR | Pseudonymous `ledger_id`, mapping destruction, anchored shred protocol. |
| Summary of Header V2 Format | Final `EventHeaderV2` struct tying findings together. |

---

## `thoughts/formspec/adrs/0059-unified-ledger-as-canonical-event-store.md`

**H1 — ADR-0059: Unified Ledger as Canonical Event Store for the WOS + Formspec Case Lifecycle**  
One append-only Respondent Ledger as canonical portable ciphertext-hashed case record; Temporal for execution only.

| H2 | Summary |
|----|---------|
| Context | Split ledger vs WOS provenance + Postgres → two truths, weak sovereignty. |
| Decision | Single canonical store; platform processes but does not own; encrypt-then-hash; disposable projections. |
| Part 1: Comprehensive Requirements | Writers, integrity, identity, privacy tiers, regulations, lifecycle, WOS governance events. |
| Part 2: Encryption Architecture | TMK/PRK/DEK, access paths, BBS+ as third layer, crypto-shredding. |
| Part 3: Technology Composition | immudb/Trillian, COSE, Rekor/OTS, BBS+, KMS, DIDs/VCs, export shape; open research. |
| Part 4: Unified Event Taxonomy | `case.*`, `wos.*`, `ledger.*`, cross-case events. |
| Part 5: Temporal Integration | Temporal history vs ledger evidence; checkpoints; materialized views. |
| Part 6: Data Hosting Model | Shared / regulated / dedicated tiers. |
| Alternatives Considered | Rejects split stores, Temporal-as-canonical, second WOS ledger, ledger-only execution, enterprise chain. |
| Consequences | Pros (sovereignty, compliance, selective disclosure, Merkle, differentiation) vs cons (CQRS, latency, BBS+ maturity, dual replay). |
| Implementation Notes | Spec deltas; coprocessor; Temporal activities; Postgres → unified migration. |
| What we do NOT need | Non-goals: separate PoP framework, on-chain consensus here, routine ZK, realtime ledger SQL, custom crypto stacks. |
| References | Internal + external standards and systems. |
| Part 7: Client-First Local Ledger (Architectural Reframe) | Browser-origin ledger, OPFS/IndexedDB, WASM parity, sync, phased rollout. |

---

## `thoughts/research/tiered-privacy-white-paper-3-24-2025.md`

**H1 — Tiered Privacy: Restoring Trust in the Digital Age**  
TPIF: tiered decentralized identity framework addressing bots, surveillance, and centralized identity failure modes.

| H2 | Summary |
|----|---------|
| A Decentralized Framework for Identity and Authenticity Online | Subtitle/metadata: TPIF positioning, repo/author/affiliations. |
| Abstract | Five-tier verification from anonymous to transparent with minimal exposure. |
| Executive Summary | Pillars: tiered model, universal PoP VC, consortium chain, ZK/MPC/FHE, FHE audit log; societal stakes. |
| 1. Background: The Problem with Identity on Today’s Internet | Bots, failed authenticity programs, Worldcoin-style risks, DID/VC limits, chain transparency, ethics/legal/a11y. |
| 2. Problem Statement: The Urgent Need for Trust | Trust erosion; need decentralized privacy-first identity. |
| 3. Technical Framework: Balancing Privacy and Trust | Tiered DIDs, CA hierarchy, PoP VC, consortium chain, ZK/MPC/FHE/mix; architecture; tiers 4.4.1–4.4.5 (nested under this H2 in source). |
| 5. TPIF Privacy-Preserving Login Protocol | Normative login: mix routing, onion encryption, ZKPs, MPC, threshold signing, FHE commitments. |
| 6. Benefits of TPIF: A Transformative Approach | Benefits for users, platforms, governments/orgs. |
| 7. Call to Action and Future Vision | Collaboration, prototype/consortium/testing; deeper paper to follow. |
| 11. References | Bibliography. |

**Note:** In the source, there is no `## 4` heading; numbering jumps from section **3** to **5** at the H2 level. Deeper `###` / `####` headings under section 3 are not listed here.

---

*Regenerate or amend this file when adding documents under `trellis/`, especially material that affects the Formspec–wos-spec ledger contract.*
