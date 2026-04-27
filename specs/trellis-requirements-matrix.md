---
title: Trellis Requirements Matrix (Consolidated)
version: 1.0.0
date: 2026-04-21
status: released
supersedes:
  - specs/archive/core/unified-ledger-requirements-matrix.md
  - specs/archive/core/unified-ledger-companion-requirements-matrix.md
  - thoughts/archive/drafts/unified-ledger-requirements-matrix.md
  - thoughts/archive/drafts/unified-ledger-companion-requirements-matrix.md
---

# Trellis Requirements Matrix (Consolidated)

## Purpose

This is the consolidated traceability matrix for the Trellis spec family. It replaces the four legacy matrices — the archived `specs/archive/core/` and `thoughts/archive/drafts/` copies of both the core matrix (`ULCR-*`) and the companion matrix (`ULCOMP-R-*`) — and adds rows for the Phase 1 envelope invariants (#1-#15 of `thoughts/product-vision.md`) that the prior matrices predated. Legacy `ULCR-*` and `ULCOMP-R-*` citations in the `Legacy` column refer to the archived-core versions in `specs/archive/core/` (which is the more complete of the two legacy sources: `ULCR-104..115` and `ULCOMP-R-215..223` exist only there; the `thoughts/archive/drafts/` versions stop at `ULCR-103` / `ULCOMP-R-214`). Trellis Core and the Trellis Operational Companion are the normative prose specifications; this matrix is not an independent source of conformance obligations. Where this matrix conflicts with normative prose, the prose governs and the matrix row is a bug to fix.

---

## Column Schema (traceability)

| Column | Definition |
|---|---|
| **ID** | Stable identifier. `TR-CORE-NNN` = core-scope. `TR-OP-NNN` = operational-companion-scope. IDs are monotonic and never reused. |
| **Scope** | `core` (lands in Trellis Core) or `operational` (lands in Trellis Operational Companion). |
| **Invariant** | Phase 1 envelope invariant #1–#15 from `thoughts/product-vision.md`. `—` = constitutional requirement not covered by a specific invariant. |
| **Requirement** | Normative MUST/MUST NOT/SHOULD/SHOULD NOT/MAY statement (BCP 14), one sentence. |
| **Rationale** | One sentence on why the requirement exists. |
| **Verification** | How the requirement is tested. Values: `test-vector` (fixture under `fixtures/vectors/...`), `projection-rebuild-drill`, `declaration-doc-check` (inspection of a required declaration document, e.g. Posture Declaration), `model-check` (property-based / protocol fuzz / state-machine model), `spec-cross-ref` (enforced by cross-spec lint), `manual-review`. |
| **Legacy** | Prior matrix IDs (`ULCR-*`, `ULCOMP-R-*`). `—` = no antecedent. |
| **Notes** | Optional: merge notes, conflict resolutions, renames. |

> **Scoped-vocabulary note (applies to every row that mentions "canonical truth", "canonical record", or "canonical order").** Per §3.3, these terms are always resolved within the governed ledger scope in force for the event (response ledger, case ledger, agency log, or federation log). Rows that do not qualify the scope explicitly are to be read as "within any Trellis ledger scope (response / case / agency / federation) as determined by the active binding" and are scope-agnostic by construction. Rows below that add parenthetical qualifications (e.g. "within the event's ledger scope") do so for clarity, not to introduce a new semantic.

---

## Section 1 — Core-Scope Requirements (`TR-CORE-NNN`)

### 1.1 Contracts (Append / Derived / Workflow / Authorization / Trust / Export)

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-001 | core | — | Implementations MAY vary append, proof, or storage mechanisms but MUST preserve admission, canonical order, canonical record formation, and canonical append attestation semantics (Canonical Append Contract). | The contract is what interop rests on; mechanism is implementation detail. | test-vector, model-check | ULCR-030 | Constitutional. |
| TR-CORE-002 | core | — | Derived artifacts MUST remain rebuildable from canonical truth (within the event's ledger scope per §3.3) and MUST NOT be authoritative for canonical facts (Derived Artifact Contract). | Prevents a second source of truth creeping in through indexes or caches. | projection-rebuild-drill | ULCR-031, ULCR-086 | Scope qualification per the scoped-vocabulary note. |
| TR-CORE-003 | core | — | Workflow state MUST remain operational unless later represented as canonical records under the active binding or declared conformance class (Workflow Contract). | Keeps orchestration out of the ledger. | spec-cross-ref, manual-review | ULCR-032 | Cross-ref: Core §4. |
| TR-CORE-004 | core | — | Grant and revocation semantics MUST remain canonical; authorization evaluator state MUST remain derived (Authorization Contract). | Evaluators are caches; the grants are the truth. | projection-rebuild-drill | ULCR-033, ULCOMP-R-095, ULCOMP-R-096, ULCOMP-R-098, ULCOMP-R-099 | Merges core + companion rows on the same contract. |
| TR-CORE-005 | core | — | Implementations MAY vary custody, key management, or delegated-compute mechanisms but the active Posture Declaration MUST describe who can read, recover, delegate, attest, or administer access (Trust Contract). | Custody is pluggable; honesty about who holds the keys is not. | declaration-doc-check | ULCR-034 | |
| TR-CORE-006 | core | — | Implementations MAY vary export packaging but exports MUST preserve required provenance distinctions and verification claims (Export Contract). | Export packaging is negotiable; what the package proves is not. | test-vector | ULCR-035 | |
| TR-CORE-007 | core | — | Bindings and implementations MUST preserve all core-to-implementation contracts when underlying mechanisms change. | Mechanism swaps must not silently relax contracts. | spec-cross-ref | ULCR-036 | Cross-ref: Core §1. |

### 1.2 Ontology, Canonical Truth, and Companion Subordination

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-010 | core | — | Normative sections MUST preserve distinctions among the primary object classes: author-originated fact, canonical record, canonical append attestation, derived artifact, disclosure/export artifact. | Every downstream guarantee depends on these five being distinguishable. | spec-cross-ref | ULCR-038, ULCR-047 | Cross-ref: Core §3 and Core §6. |
| TR-CORE-011 | core | — | Normative sections MUST NOT collapse derived or disclosure/export artifacts into canonical truth (within any Trellis ledger scope — response / case / agency / federation; see §3.3). | The substrate is what survives; presentations are not canonical. | spec-cross-ref | ULCR-039 | Scope qualification per the scoped-vocabulary note. Cross-ref: Core §3 and Core §15. |
| TR-CORE-012 | core | — | Implementations MUST NOT treat derived artifacts, workflow state, authorization evaluator state, indexes, caches, or unrecorded delegated-compute outputs as authoritative for canonical facts. | Closes every known "shortcut-as-truth" pattern. | projection-rebuild-drill, manual-review | ULCR-040 | |
| TR-CORE-013 | core | — | A canonical record MUST remain distinguishable from the underlying authored content it represents. | Records carry admission and ordering metadata the content does not. | test-vector | ULCR-048 | |
| TR-CORE-014 | core | — | A disclosure or export artifact MUST NOT be treated as identical to the underlying canonical record it references. | Disclosure-time assembly differs from canonical admission. | test-vector | ULCR-049 | |
| TR-CORE-015 | core | — | Companion specifications MUST narrow or specialize core semantics and MUST NOT reinterpret them; on conflict, core governs. | Keeps companion-local wording from silently rewriting the constitution. | spec-cross-ref | ULCR-001, ULCR-002, ULCR-005 | Merges three subordination rows. Cross-ref: Core §1 and Companion §6. |
| TR-CORE-016 | core | — | Companion specifications MUST NOT redefine canonical truth (in any Trellis ledger scope per §3.3) or define a second canonical order for the same governed scope. | Paired with TR-CORE-011, TR-CORE-020. | spec-cross-ref | ULCR-003, ULCR-004 | Scope qualification per the scoped-vocabulary note. Cross-ref: Core §3 and Core §10. |
| TR-CORE-017 | core | — | Normative sections MUST use the controlled vocabulary defined in Trellis Core when discussing canonical truth, records, attestations, and derived artifacts. | Terminology drift has been the largest source of prior-draft disputes. | spec-cross-ref | ULCR-037 | Vocabulary reconciliation: see Core §3. |
| TR-CORE-018 | core | — | Every Trellis event MUST be expressible in three distinct CDDL-level surfaces per Core §6.8: the authored form (`AuthorEventHashPreimage`, the input to `author_event_hash`); the canonical form (`EventPayload`, the COSE payload); and the signed form (`Event = COSESign1Bytes`, the wire envelope). Implementations MUST NOT treat any two surfaces as interchangeable. | Fixture filenames and tooling need unambiguous names for each surface; collapsing them hides byte-level differences. | test-vector, spec-cross-ref | — | Added 2026-04-18 to track Core §6.8 (G-3 gap B2). |

### 1.3 Canonical Order and One-Per-Scope Invariant

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-020 | core | #5 | Exactly one canonical append-attested order MUST exist per governed scope; implementations MAY partition into multiple ledgers by scope but MUST NOT allow competing canonical orders for the same governed scope. | Invariant #5 ("ordering model is named") pins a single authoritative order; partitioning is permitted only by disjoint scope. | test-vector, model-check | ULCR-043, ULCR-055, ULCR-056, ULCR-095, ULCOMP-R-120, ULCOMP-R-121 | |
| TR-CORE-021 | core | #5 | Canonical order MUST have a declared scope; inclusion, consistency, position, and export claims apply only within that scope. | Scope declaration is what makes claims falsifiable. | test-vector | ULCR-053, ULCR-054 | |
| TR-CORE-022 | core | #5 | No workflow runtime, projection, authorization evaluator, or collaboration layer MAY define an alternate canonical order for the same governed scope. | Closes lateral "order" coups from operational layers. | spec-cross-ref | ULCR-056 | Cross-ref: Core §10. |
| TR-CORE-023 | core | #5 | Canonical order MUST be determined solely by this specification and the applicable binding; MUST NOT depend on wall-clock receipt time, queue depth, worker identity, or other operational accidents. | Makes order independently reproducible. | test-vector, model-check | ULCR-102 | |
| TR-CORE-024 | core | #5 | The spec MUST name whether `prev_hash` denotes strict linear sequence or a causal DAG (HLC + explicit dependencies); if linear-only is chosen in Phase 1, the header MUST reserve the causal-dependency field. | Invariant #5 — runtime concurrency across devices cannot add causal order later without a wire break. | spec-cross-ref, test-vector | — | New invariant row. Cross-ref: Core §10. |
| TR-CORE-025 | core | #5 | Bindings SHOULD specify deterministic tie-breaking where concurrent admissible records could otherwise admit more than one total order consistent with declared causal constraints. | Makes order independently reproducible under concurrency. | test-vector, model-check | ULCR-115 | |

### 1.4 Canonical Hash Construction

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-030 | core | #1, #4 | Canonical append semantics MUST use exactly one authoritative canonical event hash construction over the sealed canonical record package; deterministic canonical serialization (the pinned CBOR profile, dCBOR or explicitly named equivalent) is REQUIRED; subordinate hashes MAY exist for specialized purposes but MUST NOT redefine canonical append semantics. | Invariant #1 (pinned canonical CBOR) + invariant #4 (hashes over ciphertext): byte-exact vectors demand both a single construction and a pinned encoding. | test-vector | ULCR-096 | Merges the one-hash-construction rule with the pinned-encoding requirement. |
| TR-CORE-031 | core | #4 | Hashes MUST be computed over ciphertext, not plaintext, for payloads subject to per-subject key destruction ("crypto-shredding"). | Invariant #4 — the only GDPR Art. 17 / FOIA-redaction story that survives an append-only chain. | test-vector | — | New invariant row. |
| TR-CORE-032 | core | #1 | The Phase 1 canonical encoding MUST be deterministic CBOR (dCBOR) per RFC 8949 §4.2.2 paired with SHA-256 over the Core-defined Trellis hash preimages (§9); future canonical hash constructions MUST be registered via a dedicated registry companion before verifiers are required to accept them. | Invariant #1 pins dCBOR as the one deterministic encoding; byte-exact cross-implementation fixtures require a single named construction and pre-empt registry-less silent extension. | test-vector, spec-cross-ref | ULCR-109 | Corrects the legacy JCS contradiction. Verification covers fixtures under `fixtures/vectors/encoding/` proving byte-exact round-trip through any conformant dCBOR encoder (see Core §5.1 and §9.2). |
| TR-CORE-033 | core | — | Every HPKE `KeyBagEntry` MUST use a fresh X25519 ephemeral keypair, unique across every wrap in the containing ledger scope (Core §9.4); within a single event with N recipients the `key_bag` MUST contain N distinct `ephemeral_pubkey` values; reuse within an event, across events in the same scope, or across scopes is non-conformance. | Producer-side freshness obligation; the persisted `ephemeral_pubkey` IS the encapsulated key derived from the single-shot private scalar, so byte-equality across wraps proves scalar reuse. Detected at corpus-authoring time, not at verify time (§19.1 has no `ephemeral_reuse` `tamper_kind`; verifiers are stateless across events per §16). | test-vector | — | Added 2026-04-27 to anchor Core §9.4 ephemeral-uniqueness MUST. Enforced by `scripts/check-specs.py` rule R17 against every event payload in the corpus; production paths use `OsRng` via `trellis-hpke::wrap_dek`, the lint closes the fixture-pinned ephemeral path under the §9.4 test-vector carve-out. |

### 1.5 Signature Suite and Signing-Key Registry

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-035 | core | #2 | Every signed artifact MUST carry a `suite_id`; the spec MUST name the Phase 1 suite (Ed25519/COSE_Sign1 or equivalent) and MUST reserve `suite_id` space for hybrid and post-quantum suites (ML-DSA, SLH-DSA). | Invariant #2 — a 2045 verifier must resolve a 2026 signature after key and suite rotations. | test-vector, declaration-doc-check | — | New invariant row. |
| TR-CORE-036 | core | #2 | The spec MUST state the migration obligation that a verifier at any future date MUST be able to resolve a prior signature after key and suite rotations. | Closes the "rotation breaks history" failure. | spec-cross-ref, test-vector | — | New invariant row. Cross-ref: Core §7. |
| TR-CORE-037 | core | #3 | Exports MUST include a signing-key registry snapshot (`SigningKeyEntry`, Active/Revoked lifecycle) so verification is self-contained at any future date. | Invariant #3 — a COSE signature without a resolvable key is unverifiable after rotation. | test-vector | — | New invariant row. |
| TR-CORE-038 | core | #7 | The `key_bag` / `author_event_hash` MUST be immutable under rotation; any key re-wrap that would change `author_event_hash` is forbidden; re-wrapping MUST produce an append-only `LedgerServiceWrapEntry`. | Invariant #7 — historical hashes must reproduce after Long-lived Authority Key rotation. | test-vector | — | New invariant row. |

### 1.6 Fact-Admission State Machine and Durable-Append Boundary

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-040 | core | — | Implementations MUST keep distinguishable: author-originated fact, canonical record, canonical append attestation, derived artifact, disclosure/export artifact (object distinction, admission state). | The fact-admission state machine rests on the five classes. | test-vector | ULCR-047 | |
| TR-CORE-041 | core | — | Companions MAY narrow admissibility (subset, predicates, actors) but MUST NOT reinterpret categories in a way that changes canonical truth (within the affected ledger scope per §3.3) or creates an alternate canonical order for the same governed scope. | Scope-specific narrowing is allowed; reinterpretation is not. | spec-cross-ref | ULCR-050 | Scope qualification per the scoped-vocabulary note. Cross-ref: Core §2 and Core §10. |
| TR-CORE-042 | core | — | A fact becomes canonical only when its canonical record has crossed the binding-declared durable-append boundary. | Canonicity is defined at the boundary, not at receipt. | test-vector | ULCR-051, ULCOMP-R-150, ULCOMP-R-151, ULCOMP-R-152 | |
| TR-CORE-043 | core | — | A canonical append attestation proves inclusion and order under the active append model; by itself it does not prove the substantive correctness of the underlying content beyond the scope of admission and attestation. | Prevents overclaiming from attestation alone. | spec-cross-ref | ULCR-052 | Cross-ref: Core §11 and Core §16. |
| TR-CORE-044 | core | — | A Canonical Append Service MUST return a canonical append attestation for canonical records that have crossed the durable-append boundary, and MUST NOT issue one before that boundary is crossed. | Pairs admission timing with attestation timing. | test-vector | ULCR-057, ULCR-058 | |
| TR-CORE-045 | core | — | A canonical append attestation MUST include or reference the canonical append position, inclusion-oriented proof material, an append-head reference, and sufficient verifier metadata to validate canonical inclusion. | Receipt must carry enough to verify offline. | test-vector | ULCR-059 | |
| TR-CORE-046 | core | — | A Canonical Append Service MUST NOT issue a canonical append attestation until binding-declared admission prerequisites are satisfied, including resolution of causal or logical dependencies required for that record class. | Attestation implies prerequisites are met. | model-check | ULCR-101 | |

### 1.7 Append Idempotency and Rejection Semantics

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-050 | core | #13 | Every `append` call MUST carry a stable idempotency key; retries with the same key and payload MUST return the same canonical record reference; retries with the same key and a different payload MUST be rejected with a defined error. | Invariant #13 — dedup belongs in the wire contract, not per-operator. | test-vector, model-check | — | New invariant row. |
| TR-CORE-051 | core | #13 | Canonical append operations MUST define idempotency semantics for retried or replayed submissions; rejected submissions MUST NOT be treated as canonically appended; for a given idempotency identity within a declared scope, every successful retry MUST resolve to the same canonical record reference or the same declared no-op outcome. | Operationalizes the idempotency contract. | test-vector | ULCR-089, ULCOMP-R-114, ULCOMP-R-115, ULCOMP-R-116, ULCOMP-R-117, ULCOMP-R-118, ULCOMP-R-119 | Merges six companion rows. |
| TR-CORE-052 | core | — | Rejections of canonical append submissions MUST be explicit and auditable. | Silent rejection is indistinguishable from acceptance. | test-vector | ULCR-108, ULCOMP-R-198, ULCOMP-R-199, ULCOMP-R-200 | Merges companion rejection rows. |
| TR-CORE-053 | core | #13 | If idempotent acceptance is supported, the spec MUST define verifier-visible consequences. | Verifier needs to tell newly-appended from already-appended. | test-vector | ULCOMP-R-119 | |

### 1.8 Verification Independence and Export

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-060 | core | — | A conforming Verifier MUST verify authored authentication where required, canonical append attestation validity and inclusion consistency, and distinguish author-originated facts, canonical records, canonical append attestations, and disclosure/export artifacts. | Verifier role is the root of independent trust. | test-vector | ULCR-020, ULCR-021, ULCR-022, ULCR-097 | |
| TR-CORE-061 | core | — | A Verifier MUST NOT require access to derived runtime state to verify canonical integrity. | Verification independence floor. | test-vector | ULCR-023, ULCR-071 | |
| TR-CORE-062 | core | — | A conforming implementation claiming the Export Generator role MUST produce independently verifiable exports for at least one declared Trellis ledger scope of canonical truth (response / case / agency / federation per §3.3). | Floor of the export contract. | test-vector | ULCR-009, ULCR-027, ULCR-066 | Scope qualification per the scoped-vocabulary note. |
| TR-CORE-063 | core | — | An export MUST include sufficient material for an offline verifier to validate the declared scope (canonical records or declared representations, attestations/proofs, verification keys or immutable key references, append proofs, schema/semantic digests plus embedded copies or immutable references, protected-payload references or payloads, and canonical facts required for claim verification). | Enumerates the minimum export package. | test-vector | ULCR-067 | |
| TR-CORE-064 | core | — | Any reference required for offline verification MUST be immutable, content-addressed, or included in the export package. | Prevents verification depending on mutable backends. | test-vector | ULCR-068 | |
| TR-CORE-065 | core | — | Exports MUST preserve the distinction among author-originated facts, canonical records, canonical append attestations, and later-assembled disclosure or export artifacts. | Export provenance is auditable only if the classes stay separate. | test-vector | ULCR-028, ULCR-070 | |
| TR-CORE-066 | core | — | Where an export omits payload readability, the export MUST still disclose which integrity, provenance, and append claims remain verifiable. | Honesty about what is and isn't verifiable. | declaration-doc-check | ULCR-072 | |
| TR-CORE-067 | core | — | A Verifier MUST be able to (1) verify authored signatures, (2) verify canonical inclusion within the declared append scope, (3) verify append-head consistency when required, (4) verify schema/semantic digests and any embedded copies or immutable references, and (5) verify any included disclosure/export artifacts. | Baseline verifier capability matrix. | test-vector | ULCR-069 | |
| TR-CORE-068 | core | — | When a Verifier reports a localizable or fatal failure, it MUST classify the dominant failure under one of the values in the Core §19.1 `tamper_kind` enum; the enum is append-only and changes MUST land in §19.1 prose first (with matching matrix row + tamper vector) before a verifier or fixture references the value. | Pins the verifier-output failure-category vocabulary so a stranger reading two independent verifier reports gets stable category names; closes the de-facto fixture convention with normative prose. | test-vector | — | Added 2026-04-27 to anchor Core §19.1 tamper_kind enum. Enforced by `scripts/check-specs.py` rule R13 against the tamper corpus. |
| TR-CORE-069 | core | — | Trellis event payloads carrying a `reason_code: uint` field MUST draw values from the per-family registry tables under Core §6.9. The integer `255` is reserved across every family as `Other` (append-only catch-all); codes `1..254` are family-local and MUST NOT be reinterpreted across families. Phase-1 verifiers MUST reject unregistered codes as a structure failure. | Per-family namespace prevents code `3` meaning "operator-boundary-change" in custody-model contexts and "legal-order-compelling-erasure" in erasure contexts from drifting into a single shared table; the `255 = Other` floor is the only cross-family invariant. | spec-cross-ref | — | Added 2026-04-27 to anchor Core §6.9 ReasonCode Registry. |

### 1.9 Manifest Bindings (Registry-Snapshot, Redaction, Plaintext-vs-Committed)

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-070 | core | #6 | The export manifest MUST include a content-addressed digest of the domain registry (event taxonomy, role vocabulary, governance rules) in force at the time of signing. | Invariant #6 — byte-integrity without semantic binding does not verify meaning. | test-vector | — | New invariant row. |
| TR-CORE-071 | core | #8 | The envelope header MUST reserve field positions for per-field commitments (Pedersen, Merkle leaves, or equivalent); BBS+ / selective-disclosure *implementation* is deferred but the *slots* are not. | Invariant #8 — retrofitting slots forces a wire-format break. | spec-cross-ref, test-vector | — | New invariant row. Cross-ref: Core §13. |
| TR-CORE-072 | core | #9 | The spec MUST list which header fields are plaintext (routing, audit classification) and which are commitments to encrypted or private values. | Invariant #9 — header-tag leakage of sensitive values is a spec decision, not an implementation choice. | declaration-doc-check, spec-cross-ref | — | New invariant row. Cross-ref: Core §12. |
| TR-CORE-073 | core | — | Event-type and classification identifiers beginning with `x-trellis-test/` MUST be reserved for conformance fixtures per Core §14.6; production deployments MUST reject events bearing `x-trellis-test/*` identifiers; such identifiers MUST NOT be resolvable against any deployed registry binding. | Allows fixture authoring without requiring a registry snapshot that resolves these identifiers, while preventing test prefixes from leaking into production data. | test-vector, spec-cross-ref | — | Added 2026-04-18 to track Core §14.6 (G-3 gaps S1/S2). |

### 1.10 Head Format, Case Ledger, Agency Log (Forward Composition)

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-080 | core | #10 | The byte shape produced by Phase 1 export MUST be the byte shape of a Phase 3 case-ledger event; Phase 2 and 3 MUST be strict supersets (additional fields only, no redefinition). | Invariant #10 — the continuity commitment underlying the phase arc. | test-vector | — | New invariant row. |
| TR-CORE-081 | core | #12 | The case-ledger head format in Phase 3 MUST be a strict superset of Phase 1's checkpoint format (same fields, additional fields only). | Invariant #12 — agency-log adoption must not be a wire-format break. | test-vector | — | New invariant row. |
| TR-CORE-082 | core | #12 | Agency-log entries MUST be case-ledger heads as produced in Phase 1 plus arrival metadata and optional witness signatures. | Invariant #12 — the log-of-case-ledgers composes forward. | test-vector | — | New invariant row. |
| TR-CORE-083 | core | #12 | The `append` operation MUST return an `AppendHead` CBOR structure per Core §10.6 containing `scope`, `sequence`, and `canonical_event_hash` fields encoded per dCBOR; this is the in-process API return artifact and MUST NOT appear in Phase 1 export packages. | Pins the byte-level shape of the `append` return so fixtures and stranger implementations produce identical outputs; keeps the API surface distinct from on-wire export. | test-vector | — | Added 2026-04-18 to track Core §10.6 (G-3 gap B3). |

### 1.11 Snapshots, Watermarks, and Rebuild

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-090 | core | #14 | Every derived artifact (projections, materialized views, indexes) and every agency-log entry MUST carry a watermark `(tree_size, tree_head_hash)` identifying the canonical state it was derived from, plus a rebuild path from the canonical chain. | Invariant #14 — full-replay-only is not valid at case-file scale. | projection-rebuild-drill | — | New invariant row; upstream for TR-OP-010, TR-OP-011. |
| TR-CORE-091 | core | — | Canonical records MUST be stored durably and immutably from the perspective of ordinary append participants; implementations MUST declare the durable-append boundary; snapshots MAY be used for performance but MUST be treated as derived artifacts; replica completion state MUST remain operational rather than canonical. | Unified storage-and-snapshot discipline. | projection-rebuild-drill, declaration-doc-check | ULCR-090, ULCOMP-R-148, ULCOMP-R-149, ULCOMP-R-150, ULCOMP-R-151, ULCOMP-R-152, ULCOMP-R-153, ULCOMP-R-154 | |

### 1.12 Posture Honesty and Companion Subordination

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-100 | core | #15 | Implementations MUST NOT describe trust posture more strongly than behavior supports; if payloads are provider-readable in ordinary operation, the declaration MUST say so; if "tamper-evident" depends on an external anchor or witness, the declaration MUST name the dependency; cryptographic controls alone MUST NOT be described as legal admissibility. | Invariant #15 — promoted from prose to normative floor. | declaration-doc-check, manual-review | ULCR-064, ULCR-074, ULCR-094, ULCOMP-R-106, ULCOMP-R-107, ULCOMP-R-108, ULCOMP-R-109, ULCOMP-R-110, ULCOMP-R-111, ULCOMP-R-163, ULCOMP-R-167, ULCOMP-R-175, ULCOMP-R-176 | Merges trust-honesty rows. |
| TR-CORE-101 | core | — | A Posture Declaration MUST semantically include declaration identifier, scope, ordinary-operation readability posture, reader-held and delegated-compute postures, current and historical decryption authorities, recovery authorities and conditions, append-attestation control authorities, exceptional-access authorities, and metadata visibility. | Minimum semantic fields for every deployment mode. | declaration-doc-check | ULCR-061, ULCR-062 | |
| TR-CORE-102 | core | — | On custody, readability, recovery, or delegated-compute change affecting protected content, implementations MUST treat the change as a posture transition, MUST make it auditable, MUST define whether it applies prospectively/retrospectively/both, and MUST NOT expand reader-held or delegated-compute access into provider-readable access without such an explicit transition. | Posture-transition auditability. | test-vector, declaration-doc-check | ULCR-065 | |
| TR-CORE-103 | core | — | Conformance classes, custody models, and bindings MUST remain subordinate to the active Posture Declaration: distinguish provider-readable / reader-held / delegated-compute; MUST NOT imply stronger confidentiality than the Posture Declaration supports; MUST NOT weaken posture requirements via local wording. | Companion-subordination rule. | spec-cross-ref | ULCR-074, ULCOMP-R-088, ULCOMP-R-089, ULCOMP-R-090, ULCOMP-R-091 | Cross-ref: Core §20 and Companion §11. |

### 1.13 Versioning, Lifecycle, and Metadata Minimization

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-110 | core | — | Implementations MUST version canonical algorithms and schema/semantic references; MUST version author-fact, canonical-record, append, export-verification, and posture semantics where custody-model- or binding-specific; MUST preserve enough immutable interpretation material to verify historical records under the rules in effect when produced; MUST NOT silently reinterpret historical records without an explicit migration mechanism; MUST NOT silently invalidate prior export verification via evolution; MUST NOT rely on out-of-band operator knowledge to interpret historical records. | Versioning discipline unified across drafts. | test-vector | ULCR-093, ULCOMP-R-201, ULCOMP-R-202, ULCOMP-R-203, ULCOMP-R-204, ULCOMP-R-205, ULCOMP-R-206, ULCOMP-R-207, ULCOMP-R-208 | |
| TR-CORE-111 | core | — | Where an implementation supports ledger-specific cryptographic lifecycle operations (key destruction, export issuance) as part of canonical or compliance-relevant behavior, it MUST represent the operation as a lifecycle fact; where the fact affects recoverability claims, it MUST be a canonical fact. | Ledger-specific portion of lifecycle; generic lifecycle is upstream at WOS. | test-vector | ULCR-091 | |
| TR-CORE-112 | core | — | Where cryptographic erasure or key destruction is used, implementations MUST document which content becomes irrecoverable, who retains access, what evidence of destruction is preserved, and what metadata remains; affected derived plaintext state MUST be invalidated, purged, or made unusable per declared policy. | Ties erasure to derived-view invalidation. | declaration-doc-check | ULCR-092, ULCOMP-R-159, ULCOMP-R-160 | |
| TR-CORE-113 | core | — | Visible metadata SHOULD be limited to canonical verification, schema/semantic lookup, required audit-visible declarations, conflict gating, and append processing; SHOULD NOT be kept merely to accelerate derived artifacts; MUST NOT be retained merely for operational convenience where derived or scoped mechanisms suffice. | Metadata-minimization rule. | declaration-doc-check | ULCR-088, ULCOMP-R-169, ULCOMP-R-170, ULCOMP-R-171, ULCOMP-R-172 | |

### 1.14 Cross-Repository Authority and Baseline Scope

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-120 | core | — | Trellis Core semantics MUST NOT be interpreted to redefine Formspec or WOS semantic authority. | Keeps the three-spec stack stable. | spec-cross-ref | ULCR-098 | Cross-ref: Core §4, Core §22, and Core §23. |
| TR-CORE-121 | core | — | When Trellis behavior depends on Formspec Definition or Response semantics (field values, relevance, validation, calculation), processing MUST be delegated to a Formspec-conformant processor; Trellis MUST NOT specify bind, FEL, or validation rules. | Formspec delegation. | spec-cross-ref | ULCR-104 | Cross-ref: Core §22. |
| TR-CORE-122 | core | — | A Formspec processor that ignores all Trellis sidecars, bindings, and artifacts MUST remain fully conformant to Formspec and MUST produce identical data and validation results (additive-invariant rule). | Trellis is additive, not invasive. | spec-cross-ref, test-vector | ULCR-105 | Cross-ref: Core §22. |
| TR-CORE-123 | core | — | Trellis-bound Formspec processors MUST implement at least Formspec Core conformance; roles that present Formspec-backed tasks to end users additionally require Component conformance. | Sets the Formspec conformance floor. | declaration-doc-check | ULCR-106 | |
| TR-CORE-124 | core | — | When a Trellis-bound deployment uses Formspec Screener evaluation, it MUST delegate to a Formspec-conformant Screener processor and MUST NOT alter the Screener evaluation algorithm. | Screener delegation. | spec-cross-ref | ULCR-107 | Cross-ref: Core §22. |
| TR-CORE-125 | core | — | Trellis MUST bind Formspec-family and WOS-family facts (and related trust/release families per binding spec) into one governed canonical substrate with shared append, hash, and verification rules; the binding MUST NOT reinterpret Formspec or WOS meaning. | Substrate binding. | spec-cross-ref | ULCR-099 | Legacy "canonical substrate" → "governed canonical substrate" (response ledger / case ledger / agency log / federation log per vocabulary in §3.3). Cross-ref: Core §22 and Core §23. |
| TR-CORE-126 | core | — | Baseline Trellis Core conformance MUST NOT be interpreted to require advanced selective disclosure, threshold custody, group-sharing protocols, advanced homomorphic or privacy-preserving computation, or cross-agency analytic protocols unless a declared conformance class, custody model, binding, or implementation specification explicitly requires them. | Keeps the baseline small. | spec-cross-ref | ULCR-100, ULCOMP-R-213, ULCOMP-R-214 | Cross-ref: Core §2 and Companion §6. |

### 1.15 Conformance Roles

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-130 | core | — | A conforming implementation MUST claim one or more of the five conformance roles (Fact Producer, Canonical Append Service, Verifier, Derived Processor, Export Generator) and MUST satisfy all requirements applicable to each claimed role. | Declaration of role set is the conformance unit. | declaration-doc-check | ULCR-006 | |
| TR-CORE-131 | core | — | A Fact Producer MUST emit attributable facts admissible under the active conformance class or binding; MUST sign or authenticate facts where required; MUST preserve causal references when applicable; MUST NOT rewrite previously emitted facts. | Producer role duties. | test-vector | ULCR-010, ULCR-011, ULCR-012, ULCR-013 | |
| TR-CORE-132 | core | — | A Canonical Append Service MUST preserve append-only semantics, validate admissibility, form canonical records for admitted facts, append to canonical order within the governed ledger scope it serves (response / case / agency / federation per §3.3), issue canonical append attestations, and MUST NOT rewrite prior canonical records or treat workflow state / projections / caches as canonical truth within that scope. | CAS role duties. | test-vector | ULCR-007, ULCR-014, ULCR-015, ULCR-016, ULCR-017, ULCR-018, ULCR-019 | Scope qualification per the scoped-vocabulary note. |
| TR-CORE-133 | core | — | A Derived Processor MUST treat canonical records (of its declared source ledger scope per §3.3) as its only authoritative input; MUST record sufficient provenance to support rebuild from that canonical state; MUST be discardable and rebuildable from canonical state without altering canonical truth in any Trellis ledger scope. | Derived-processor role duties. | projection-rebuild-drill | ULCR-024, ULCR-025, ULCR-026 | Scope qualification per the scoped-vocabulary note. |
| TR-CORE-134 | core | — | An Export Generator MUST package canonical records, attestations, and verification material per declared export scope; MUST preserve provenance distinctions; MUST include enough material for an offline verifier to validate the declared export scope. | Export-generator role duties. | test-vector | ULCR-027, ULCR-028, ULCR-029 | |

### 1.16 Binding / Sidecar Boundary

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-CORE-140 | core | — | Where a binding declares deterministic encodings, canonical byte sequences, exact proof formats, or API procedures, conforming implementations for that binding MUST follow it. | Binding-declared exactness is normative within its scope. | test-vector | ULCR-060, ULCR-082 | |
| TR-CORE-141 | core | — | Domain vocabularies, respondent-history vocabularies, forms vocabularies, workflow-family vocabularies, and similar interpretation layers SHOULD be defined in companion specifications rather than in core. | Keeps the core small. | spec-cross-ref | ULCR-083 | Cross-ref: Core §22, Core §23, and Companion §23. |
| TR-CORE-142 | core | — | A sidecar MAY collect family-specific, deployment-specific, or implementation-adjacent material subordinate to core; MUST NOT alter constitutional semantics. | Sidecar discipline. | spec-cross-ref | ULCR-085, ULCOMP-R-173, ULCOMP-R-174 | Cross-ref: Companion §23 and Companion §24. |
| TR-CORE-143 | core | — | Binding-defined ingest-time verification or payload-readiness fields on the canonical append attestation (or equivalent receipt) MUST NOT be rewritten in place after issuance; posture changes MUST be recorded as new canonical facts or attestations per binding. | Receipt immutability. | test-vector | ULCR-103 | |

---

## Section 2 — Operational-Companion-Scope Requirements (`TR-OP-NNN`)

### 2.1 Projection Discipline (Watermark, Stale Indication, Rebuild)

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-OP-001 | operational | #14 | Every staff-facing projection MUST carry a watermark indicating canonical append/checkpoint state it was derived from. | Invariant #14 — projections must identify the canonical state they reflect. | projection-rebuild-drill | ULCOMP-R-215 | |
| TR-OP-002 | operational | #14 | Every staff-facing projection MUST expose: canonical checkpoint identifier; canonical append height/sequence at build time; projection build timestamp; projection schema/version identifier. | Minimum watermark field set. | projection-rebuild-drill | ULCOMP-R-216 | |
| TR-OP-003 | operational | #14 | If a projection is stale relative to a newer canonical checkpoint, the view MUST indicate stale status. | Protects against stale-read decisions. | projection-rebuild-drill, manual-review | ULCOMP-R-217 | |
| TR-OP-004 | operational | — | Crypto-shredding MUST cascade: plaintext-derived projections and caches MUST be purged according to policy. | Otherwise erasure is incomplete. | projection-rebuild-drill | ULCOMP-R-218 | |
| TR-OP-005 | operational | #14 | Rebuilding a projection from canonical records for the same checkpoint MUST yield semantically equivalent output for declared projection fields; rebuild output MUST be byte-equal over the declared-deterministic portion (Core §15.3). | Rebuild equivalence is what makes projections discardable. | projection-rebuild-drill, test-vector | ULCOMP-R-219 | `Verification` flipped to include `test-vector` 2026-04-18 per Wave 1 Stream B O-3 design; unlocks byte-level conformance fixtures for rebuild equivalence. |
| TR-OP-006 | operational | #14 | Projection conformance tests MUST validate watermark presence (fields in Core §15.2 `Watermark`) and stale-status behavior. | Makes the discipline testable. | projection-rebuild-drill, test-vector | ULCOMP-R-220 | `Verification` flipped to include `test-vector` 2026-04-18 per Wave 1 Stream B O-3 design. |
| TR-OP-007 | operational | — | Each conforming deployment MUST define ongoing projection correctness checks including at least sampled rebuild comparison or checkpoint-bound equivalence; access-grant or authorization-expanding projections SHOULD be checked more frequently than general read models. | Projection integrity policy. | projection-rebuild-drill | ULCOMP-R-223 | |
| TR-OP-008 | operational | — | Operators MUST emit Trellis checkpoints at a cadence declared in the Posture Declaration (Companion §16 (Snapshot-from-Day-One)); checkpoint-cadence gaps MUST be detected and surfaced either by the implementation at runtime or by the verifier/auditor via fixture-bound checks. | OC-46 makes snapshot-from-day-one testable; absent this row the cadence obligation has no matrix anchor for G-3 lint coverage. | test-vector, manual-review | — | Added 2026-04-18 per Wave 1 Stream B O-3 design (gap #2: OC-46 anchor). |

### 2.2 Custody Models (CM-A … CM-F)

Canonical list: Companion §9 (Custody Models), in particular §9.2 (The Six Standard Custody Models). Rows below cite Companion §9 by anchor; each row names one `CM-*` identifier. A sixth identifier `CM-F` (Client-Origin Sovereign) is tracked in §4.3 and Core §21.3 but has no per-model requirement row here beyond the general Custody-Model honesty obligations in TR-OP-015, TR-OP-016, and Companion §9.4 / §9.6.

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-OP-010 | operational | — | A Posture Declaration using the `CM-A` (Provider-Readable Custodial) Custody Model MUST say so plainly and MUST NOT imply provider blindness. | Companion §9.2, §9.4 CM-A obligations (legacy companion letter: A). | declaration-doc-check | ULCOMP-R-176 | See §4.3 renaming. |
| TR-OP-011 | operational | — | A Posture Declaration using the `CM-B` (Reader-Held with Recovery Assistance) Custody Model MUST describe who can assist recovery and under what conditions. | Companion §9.2, §9.4 CM-B obligations (legacy companion letter: B). | declaration-doc-check | ULCOMP-R-177 | |
| TR-OP-012 | operational | — | A Posture Declaration using the `CM-C` (Delegated Compute) Custody Model MUST state whether plaintext is visible to any provider-operated components during delegation. | Companion §9.2, §9.4 CM-C obligations (legacy companion letter: C). | declaration-doc-check | ULCOMP-R-178 | |
| TR-OP-013 | operational | — | A Posture Declaration using the `CM-D` (Threshold-Assisted Custody) Custody Model MUST declare recovery conditions, quorum thresholds, and exceptional access; threshold participation MUST NOT be overstated. | Companion §9.2, §9.4 CM-D obligations (legacy companion letter: D). | declaration-doc-check | ULCOMP-R-179 | |
| TR-OP-014 | operational | — | A Posture Declaration using the `CM-E` (Organizational Trust) Custody Model MUST identify the scope of organizational authority and exceptional-access controls; MUST distinguish provider-readable from organization-controlled access where they differ. | Companion §9.2, §9.4 CM-E obligations (legacy companion letter: E). | declaration-doc-check | ULCOMP-R-180 | |
| TR-OP-017 | operational | — | A Posture Declaration using the `CM-F` (Client-Origin Sovereign) Custody Model MUST identify the client-origin key authority, the presence or absence of operator recovery, and the consequences of client key loss; MUST NOT imply legal or operational availability beyond what the key-custody design supports. | Companion §9.2, §9.4 CM-F obligations (no legacy companion letter; CM-F is new to the unified namespace). | declaration-doc-check | — | See §4.3 renaming. |
| TR-OP-015 | operational | — | A reader-held custody model MUST declare ordinary service operation does not require general plaintext access for declared protected content; MUST identify which principals may decrypt within scope; MUST identify whether the provider can assist recovery; MUST remain consistent with the active Posture Declaration; MUST distinguish reader-held access from provider-readable access and from delegated-compute access. | Unified reader-held custody rules. | declaration-doc-check | ULCOMP-R-029, ULCOMP-R-030, ULCOMP-R-031, ULCOMP-R-032, ULCOMP-R-033, ULCOMP-R-034 | |
| TR-OP-016 | operational | — | Reader-held access MUST NOT be described as provider-readable ordinary operation; MAY coexist with recovery assistance if the Posture Declaration declares it honestly; MAY coexist with delegated compute if delegation remains explicit, scoped, and auditable. | Reader-held honesty rule. | declaration-doc-check | ULCOMP-R-035, ULCOMP-R-036, ULCOMP-R-037 | |

### 2.3 Delegated Compute

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-OP-020 | operational | — | A delegated-compute custody model MUST distinguish delegated compute from provider-readable access; MUST make delegation explicit, attributable, and auditable; MUST define delegation scope and authority; SHOULD define purpose or time bounds; MUST define what audit facts or events are emitted for delegation and use; MUST NOT imply delegated compute grants general service readability. | Unified delegated-compute custody rules. | declaration-doc-check, test-vector | ULCOMP-R-038, ULCOMP-R-039, ULCOMP-R-040, ULCOMP-R-041, ULCOMP-R-042, ULCOMP-R-043, ULCOMP-R-044 | |
| TR-OP-021 | operational | — | A delegated-compute grant MUST be explicit, attributable to a principal or policy authority, scoped to declared content or content classes, auditable, and MUST NOT be interpreted as conferring standing plaintext access; SHOULD be time- or purpose-bounded. | Grant discipline. | declaration-doc-check | ULCOMP-R-045, ULCOMP-R-046, ULCOMP-R-047, ULCOMP-R-048, ULCOMP-R-049, ULCOMP-R-050 | |
| TR-OP-022 | operational | — | If a system relies materially on delegated-compute output, it MUST record the output as a canonical fact or canonical reference to a stable artifact; MUST preserve auditable links to the authorizing principal, compute agent identity, and scope of delegated access relevant to that output; MUST define whether the relied-upon output is advisory, recommendatory, or decision-contributory. | Material-reliance discipline. | test-vector | ULCOMP-R-051, ULCOMP-R-052, ULCOMP-R-053, ULCOMP-R-054, ULCOMP-R-055 | |

### 2.4 Grants, Revocations, Evaluator Rebuild

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-OP-030 | operational | — | Access grants and revocations affecting canonical authorization semantics MUST be recorded as append-only canonical facts. | Grants are truth; evaluators are caches. | test-vector | ULCOMP-R-095 | |
| TR-OP-031 | operational | — | Authorization evaluators MAY be derived; if derived, MUST be rebuildable from canonical grant and revocation facts; MUST NOT be authoritative for grant existence, grant history, or revocation history; MUST preserve canonical grant/revocation semantics when evaluator absent, stale, or rebuilding. | Evaluator discipline. | projection-rebuild-drill | ULCOMP-R-096, ULCOMP-R-097, ULCOMP-R-098, ULCOMP-R-099 | |
| TR-OP-032 | operational | — | If delegation affects authorization, legal authority, or access posture, delegation grants and revocations MUST be canonical facts. | Delegation is canonical when rights-impacting. | test-vector | ULCOMP-R-100 | |
| TR-OP-033 | operational | — | Where both narrow sharing and long-lived collaborative membership are supported, SHOULD avoid forcing both into one mechanism if doing so increases KM/audit complexity. | Sharing-mode hygiene. | manual-review | ULCOMP-R-101 | |
| TR-OP-034 | operational | — | If a derived evaluator is used for rights-impacting decisions, implementations MUST trace evaluator inputs to canonical facts; MUST define evaluator rebuild behavior; MUST define behavior when evaluator state is stale, missing, or inconsistent with canonical facts; MUST preserve the rule that evaluator state does not override canonical grant/revocation semantics. | Rights-impacting evaluator rebuild. | projection-rebuild-drill | ULCOMP-R-102, ULCOMP-R-103, ULCOMP-R-104, ULCOMP-R-105, ULCR-087 | |

### 2.5 Metadata Budget and Verification Posture

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-OP-040 | operational | #9 | Each declared Posture Declaration MUST include a metadata budget by canonical fact family (visible fields, observer classes, timing/access-pattern leakage, linkage stability, delegated-compute effects). | Invariant #9 — metadata-leakage is a spec decision. | declaration-doc-check | ULCOMP-R-221 | |
| TR-OP-041 | operational | — | Tiered verification deployments MUST declare verification posture classes and which downstream workflow or release classes each posture MAY feed; MUST NOT attach high-stakes outcomes to records below the declared minimum posture for that class; posture escalation MUST NOT be silent (MUST use explicit canonical facts or binding-defined attestations). | Verification-posture tiering. | declaration-doc-check, test-vector | ULCOMP-R-222 | |
| TR-OP-042 | operational | #11 | Custody-model Posture-transition events MUST conform to the `trellis.custody-model-transition.v1` CDDL schema (Companion Appendix A.5.1); `from_custody_model` and `to_custody_model` MUST name entries in the Custody Model Registry (Appendix A.4). | Invariant #11 — transitions are the temporal axis of the custody-model namespace. Without a pinned schema, deployments drift. | test-vector, spec-cross-ref | — | Added 2026-04-18 per Wave 1 Stream D O-5 design. Cross-ref: Companion §10. |
| TR-OP-043 | operational | #11 | Disclosure-profile Posture-transition events MUST conform to the `trellis.disclosure-profile-transition.v1` CDDL schema (Companion Appendix A.5.2); `from_disclosure_profile` and `to_disclosure_profile` MUST be Respondent Ledger Profile A/B/C values (`rl-profile-A`, `rl-profile-B`, `rl-profile-C`). | Invariant #11 — Respondent Ledger Profile A/B/C is the disclosure-profile namespace. | test-vector, spec-cross-ref | — | Added 2026-04-18 per Wave 1 Stream D O-5 design. Cross-ref: Companion §10. |
| TR-OP-044 | operational | #15 | A Trellis verifier processing a Posture-transition event MUST verify state continuity (`from_*` matches the most-recent-prior transition of the same kind in `ledger_scope`, or the deployment's initial declaration), declaration-digest resolution under `trellis-posture-declaration-v1` (Core §9 (Hash Construction) §9.8), and attestation-count semantics per Companion §10.4 (widening = dual attestation required; narrowing MAY be attested by the new authority alone). Failures MUST be reported per Core §19 (Verification Algorithm) step 6. | Invariant #15 — verification MUST detect overclaiming via transition drift. | test-vector | — | Added 2026-04-18 per Wave 1 Stream D O-5 design. |
| TR-OP-045 | operational | #15 | Every Posture-transition event's `declaration_doc_digest` MUST resolve to the Posture Declaration in force AFTER the transition (co-publish rule, OC-15 / OC-09); the referenced declaration's content digest under `trellis-posture-declaration-v1` MUST equal the stored digest. A digest mismatch is tamper evidence and is fatal (Core §19 step 6.c). | Invariant #15 — transitions that do not co-publish the declaration leave posture honesty ambiguous and break audit. | test-vector | — | Added 2026-04-18 per Wave 1 Stream D O-5 design. |
| TR-OP-046 | operational | — | Custody-Model Transition `reason_code` values MUST be drawn from Companion §A.5.1's registered table (codes 1–5 plus 255 = Other); Disclosure-Profile Transition `reason_code` values MUST likewise be drawn from Companion §A.5.2's registered table. Both tables are append-only under Core §6.9 ReasonCode Registry discipline; new codes land in the table first with a matrix update before a vector or runtime references them. | Per-family registration under Core §6.9 keeps custody-model and disclosure-profile reason vocabularies independently extensible without cross-namespace collision. | spec-cross-ref | — | Added 2026-04-27 to anchor Companion §A.5.1 / §A.5.2 reason-code registries under Core §6.9. |
| TR-OP-047 | operational | — | Every O-4 delegated-compute declaration document's `[signature]` block MUST be structurally valid (no crypto required): `alg` is a registered Phase-1 value (`EdDSA` per Core §7.1 / §7.2); `signer_kid` is a non-empty URI string resolving to a key registry entry; `cose_sign1_b64` is a non-empty base64-shaped string. Crypto verification (rule 14 in the design-doc lint surface) is not required for this static check. | The declaration's signature is the operator's commitment that the rest of the frontmatter accurately describes the deployment; verifying the field shapes is independent of holding the key, so it can run in CI without a key registry snapshot. | spec-cross-ref, declaration-doc-check | — | Added 2026-04-27. Anchored at Companion §19 (Delegated-Compute Honesty) §19.9 OC-70d. Enforced by `scripts/check-specs.py` rule R14 within `check_declaration_docs`. Closes the static-lint half of O-4 design rule 14. |
| TR-OP-048 | operational | — | Across all O-4 declaration documents, the `supersedes` graph keyed by `declaration_id → supersedes` MUST be acyclic and resolvable: every non-empty `supersedes` value MUST appear as some declaration's `declaration_id`; no chain MUST revisit a node; `declaration_id` values MUST be unique across the corpus. Empty-string and absent `supersedes` denote "no predecessor." | A cyclic `supersedes` graph would let a deployment claim two declarations succeed each other in a loop, defeating the audit-trail purpose of the pointer; dangling references would break the auditor's ability to walk back to the prior declaration in force. | spec-cross-ref, declaration-doc-check | — | Added 2026-04-27. Anchored at Companion §19 (Delegated-Compute Honesty) §19.9 OC-70e. Enforced by `scripts/check-specs.py` rule R15 (`check_declaration_supersedes_acyclic`). Closes the static-lint half of O-4 design rule 15. |

### 2.6 Offline Authoring Conformance Class

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-OP-050 | operational | — | An offline-authoring conformance class MUST permit author-originated facts to exist prior to canonical append; MUST preserve authored authentication semantics across delayed submission; MUST preserve authored time or authored context where available; MUST distinguish authored time from canonical append time unless equivalence is established explicitly; MUST define how local pending facts remain non-canonical until admitted; MUST define duplicate-submission and replay behavior for delayed submissions; MUST preserve provenance distinctions among authored fact, canonical record, and canonical append attestation. | Unified offline-authoring baseline. | test-vector | ULCOMP-R-011, ULCOMP-R-012, ULCOMP-R-013, ULCOMP-R-014, ULCOMP-R-015, ULCOMP-R-016, ULCOMP-R-017 | |
| TR-OP-051 | operational | — | An offline-authoring conformance class SHOULD minimize local pending state to what is necessary for user-authoring continuity; SHOULD avoid treating broad local collaboration state as canonical truth in any Trellis ledger scope (response / case / agency / federation per §3.3); SHOULD define how rejected offline submissions are surfaced without implying canonical admission. | Offline-authoring hygiene. | manual-review | ULCOMP-R-018, ULCOMP-R-019, ULCOMP-R-020 | Scope qualification per the scoped-vocabulary note. |
| TR-OP-052 | operational | — | Offline-originated facts MAY be submitted after delay; if accepted, MUST preserve authored authentication semantics; MUST distinguish later admission and later append attestation from earlier authorship; MUST NOT imply canonical append time is identical to authorship time unless equivalence is established. | Offline submission rules. | test-vector | ULCOMP-R-021, ULCOMP-R-022, ULCOMP-R-023, ULCOMP-R-024 | |
| TR-OP-053 | operational | — | Local pending state before admission MUST remain non-canonical; MUST NOT define alternate canonical order; SHOULD remain separable from draft-collaboration state; MUST be transformable into submitted facts without silently rewriting prior authored facts. | Pending-state discipline. | test-vector | ULCOMP-R-025, ULCOMP-R-026, ULCOMP-R-027, ULCOMP-R-028 | |

### 2.7 Durable-Append Boundary, Storage, Conflict Handling

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-OP-060 | operational | — | Canonical acceptance MUST define which durable write conditions are required; implementations MUST declare the durable-append boundary governing attestation, retry, and export issuance; proof or referenced state needed to recover or verify within export scope MUST be durably recoverable no later than that boundary. | Companion-scope durable-append boundary rules. | declaration-doc-check, test-vector | ULCOMP-R-148, ULCOMP-R-150, ULCOMP-R-151, ULCOMP-R-152, ULCOMP-R-153 | Core-scope invariant is TR-CORE-042; these are operational elaborations. |
| TR-OP-061 | operational | — | Implementations MAY define conflict-sensitive fact categories; conflict handling MUST be evaluated within the declared append scope of affected canonical facts; append in unaffected scopes MUST continue; affected derived systems MAY gate on explicit resolution facts; derived artifacts MUST NOT silently rewrite canonical facts to resolve conflicts; conflict resolution SHOULD be via later canonical facts, explicit rejection, or binding-defined admission rules; MUST NOT stall unrelated append scopes solely because a conflict is unresolved in another scope. | Conflict-handling discipline. | test-vector, model-check | ULCOMP-R-128, ULCOMP-R-129, ULCOMP-R-130, ULCOMP-R-131, ULCOMP-R-132, ULCOMP-R-133, ULCOMP-R-134 | |

### 2.8 Protected Payloads, Selective Disclosure, Disclosure Artifacts

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-OP-070 | operational | — | Sensitive content SHOULD reside in protected payloads when the Posture Declaration or binding requires protection; implementations MUST define which data is visible for canonical verification vs payload-protected; canonical records with protected payloads MUST include or reference sufficient access material for authorized recipients per custody/binding; conforming representation MUST preserve the semantic distinction among author fact, payload content, access/key material, and append-attestation material. | Protected-payload discipline. | test-vector | ULCOMP-R-144, ULCOMP-R-145, ULCOMP-R-146, ULCOMP-R-147 | |
| TR-OP-071 | operational | #8 | Selective disclosure SHOULD occur through disclosure or export artifacts rather than overloading canonical records. | Invariant #8 — selective-disclosure slots live in the envelope; selective disclosure happens above. | test-vector | ULCOMP-R-063 | |
| TR-OP-072 | operational | — | A disclosure-oriented artifact MAY present an audience-specific subset or presentation; MUST preserve provenance distinctions; MUST NOT be treated as a rewrite of canonical truth in the source ledger scope (per §3.3) or in any other Trellis ledger scope. | Disclosure-artifact discipline. | test-vector | ULCOMP-R-064, ULCOMP-R-065, ULCOMP-R-066 | Scope qualification per the scoped-vocabulary note. |
| TR-OP-073 | operational | — | A disclosure/export conformance class MUST support at least one verifiable disclosure or export form; MUST preserve distinction among author facts, canonical records, attestations, and later disclosure/export artifacts; MUST define which claims remain verifiable when payload readability is absent; MUST define audience scope where relevant; MUST remain subordinate to export guarantees of the core specification. | Disclosure/export rules. | test-vector | ULCOMP-R-056, ULCOMP-R-057, ULCOMP-R-058, ULCOMP-R-059, ULCOMP-R-060 | |
| TR-OP-074 | operational | — | A disclosure/export conformance class SHOULD state which claim classes are verifiable within that class; implementations MUST NOT imply an export supports a claim class unless the export contains sufficient material to verify that class. | Claim-class honesty. | declaration-doc-check, test-vector | ULCOMP-R-061, ULCOMP-R-062 | |

### 2.9 Privacy / Metadata Minimization (operational elaboration)

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-OP-080 | operational | — | Implementations handling protected content MUST document what is protected from whom; payload confidentiality MUST NOT be described as equivalent to metadata privacy; if provider-readable in ordinary operation, MUST say so plainly; if delegated compute operates without general provider readability, MUST distinguish that mode from provider-readable custody. | Operational privacy disclosure. | declaration-doc-check | ULCOMP-R-165, ULCOMP-R-166, ULCOMP-R-167, ULCOMP-R-168 | |

### 2.10 CAS Operational Obligations and Proof Model

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-OP-090 | operational | — | A CAS MUST validate append admissibility, preserve append-only, issue attestations, and retain/reference sufficient proof material for verification; by canonical role alone, a CAS MUST NOT be required to decrypt payloads, evaluate workflow policy, resolve workflow runtime, compute projections/indexes, or inspect protected content unless the Posture Declaration permits/requires it. | CAS operational scope. | declaration-doc-check, test-vector | ULCOMP-R-112, ULCOMP-R-113 | |
| TR-OP-091 | operational | — | Implementations MUST present one verifier-facing canonical append proof model per declared append scope at a time; MUST NOT require verifiers to reconcile multiple overlapping append-attestation semantics for the same scope; if the proof model changes, MUST define an explicit migration boundary; SHOULD use transparency-log-style append with order, inclusion proofs, and consistency proofs between append heads. | Proof-model discipline. | test-vector | ULCOMP-R-120, ULCOMP-R-121, ULCOMP-R-122, ULCOMP-R-123 | |
| TR-OP-092 | operational | — | Implementations MAY support external witnessing or anchoring; external witnessing MUST remain subordinate to core canonical append semantics; MUST NOT be required for correctness unless a registered deployment class or binding explicitly states otherwise; MAY strengthen equivocation detection or independent audit posture. | Witnessing discipline; supports Phase 4. | declaration-doc-check | ULCOMP-R-124, ULCOMP-R-125, ULCOMP-R-126, ULCOMP-R-127 | |

### 2.11 Lifecycle, Erasure, Sealing, Legal Sufficiency (ledger-scoped)

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-OP-100 | operational | — | If cryptographic erasure or key destruction is used, implementations MUST document what content becomes irrecoverable, who retains access, what evidence of destruction is preserved, and what metadata remains; affected derived plaintext state MUST be invalidated, purged, or made unusable per declared policy. | Operational elaboration of TR-CORE-112. | declaration-doc-check | ULCOMP-R-159, ULCOMP-R-160 | |
| TR-OP-101 | operational | — | Implementations MUST NOT imply that cryptography alone guarantees admissibility or legal sufficiency in all jurisdictions; MAY claim stronger evidentiary posture only to the extent supported by process, signatures, attestations, records practice, and law. | Legal-sufficiency honesty. | manual-review, declaration-doc-check | ULCOMP-R-163, ULCOMP-R-164 | |
| TR-OP-104 | operational | — | Erasure-Evidence `reason_code` values MUST be drawn from the registered table accompanying the `trellis.erasure-evidence.v1` event family (ADR 0005 §"Reason codes"; Companion §20 once promoted from ADR). The table is append-only under Core §6.9 ReasonCode Registry discipline and shares the `255 = Other` floor with all other reason-code families. The numeric values 1–5 in this family are not interchangeable with the same numeric values in Custody-Model or Disclosure-Profile Transition tables — cross-family reinterpretation is forbidden. | Anchors the erasure-evidence reason-code vocabulary under Core §6.9 alongside the transition families, so verifier and operator workflows have one place to look for "which families register reason codes." | spec-cross-ref | — | Added 2026-04-27. Co-lands with sequence item #4 (ADR 0005 execution); reservation now keeps the ADR-promotion path lint-clean when Companion §20 absorbs the table. |

### 2.12 Versioning / Algorithm Agility (operational elaboration)

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-OP-110 | operational | #2 | Implementations MUST preserve enough immutable interpretation material to verify historical records without live registry lookups, mutable references, or out-of-band operator knowledge. | Operational restatement of TR-CORE-110 tied to invariant #2 (signature-suite migration). | test-vector | ULCOMP-R-208 | |
| TR-OP-111 | operational | — | Implementations SHOULD test canonical invariants via model checking, replay, property-based tests, and protocol fuzzing. | Operational testing guidance. | model-check | ULCOMP-R-209 | |
| TR-OP-112 | operational | — | Implementers SHOULD reduce offline coordination scope where possible; offline capabilities SHOULD be reserved for authoring, signing, and bounded local transitions not requiring broad multi-party reconciliation; SHOULD separate draft collaboration semantics from canonical semantics. | Operational migration guidance. | manual-review | ULCOMP-R-210, ULCOMP-R-211, ULCOMP-R-212 | |

### 2.13 Companion-Scope Companion-Subordination Restatements

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-OP-120 | operational | — | Nothing in the operational companion creates a second canonical order, alters the definition of canonical truth in any Trellis ledger scope (response / case / agency / federation per §3.3), collapses derived artifacts into canonical truth, weakens trust-honesty requirements, or weakens export-verification guarantees. | Operational document-property rule. | spec-cross-ref | ULCOMP-R-006, ULCOMP-R-007, ULCOMP-R-008, ULCOMP-R-009, ULCOMP-R-010 | Scope qualification per the scoped-vocabulary note. Cross-ref: Companion §14, Companion §15, and Companion §27. |
| TR-OP-121 | operational | — | The operational companion MAY define custody-model-specific constraints, binding/sidecar interpretation layers, and reusable companion requirements that refine but do not reinterpret the core; additional requirements MUST be interpreted consistently with the core specification; MUST remain subordinate to the constitutional semantics of Trellis Core. | Operational-scope subordination. | spec-cross-ref | ULCOMP-R-001, ULCOMP-R-002, ULCOMP-R-003, ULCOMP-R-004, ULCOMP-R-005 | Cross-ref: Companion §6 and Companion §9. |
| TR-OP-122 | operational | — | Custody-model, binding, and sidecar exports MUST preserve author / canonical-record / attestation / disclosure distinctions and provenance distinctions when presenting scoped timelines, deltas, or interpretations; MUST NOT imply broader workflow/governance/custody/compliance/disclosure coverage than the declared scope includes. | Scoped-export honesty. | test-vector | ULCOMP-R-092, ULCOMP-R-093, ULCOMP-R-094 | |

### 2.14 Versioned Registries (Operator-Side Complement to Invariant #6)

| ID | Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes |
|---|---|---|---|---|---|---|---|
| TR-OP-130 | operational | #6 | Implementations SHOULD define versioned registries for the identifier and kind categories referenced by canonical records (event taxonomy, role vocabulary, governance rules, and any binding-declared registries); each registry MUST be resolvable to a content-addressed digest for inclusion in the export manifest per TR-CORE-070. | Invariant #6 fixes a snapshot digest in the manifest but presumes the registries themselves exist and are versioned; without this operator-side duty the manifest digest has nothing semantically meaningful to bind to. | declaration-doc-check, test-vector | ULCOMP-R-197 | Reinstated from gap-log §5.4 — the prior "owned upstream by WOS Governance App. A" justification was incorrect (WOS Governance has no Appendix A; `cross-reference-map.md` confirms no upstream home). |

---

## Section 3 — Mapping Tables

### 3.1 Invariants #1–#15 → `TR-CORE-NNN`

Every invariant gets at least one row. Invariants that generate operational duties additionally reference `TR-OP-NNN`.

| Invariant | Name (short) | TR-CORE rows | TR-OP rows |
|---|---|---|---|
| #1 | Canonical CBOR profile pinned | TR-CORE-030, TR-CORE-032 | — |
| #2 | Signature suite identified, migration obligation | TR-CORE-035, TR-CORE-036 | TR-OP-110 |
| #3 | Signing-key registry in export | TR-CORE-037 | — |
| #4 | Hashes over ciphertext | TR-CORE-030, TR-CORE-031 | — |
| #5 | Ordering model named (linear vs causal DAG) | TR-CORE-020, TR-CORE-021, TR-CORE-022, TR-CORE-023, TR-CORE-024, TR-CORE-025 | — |
| #6 | Registry-snapshot binding in manifest | TR-CORE-070 | TR-OP-130 |
| #7 | `key_bag` / author-event-hash immutable under rotation | TR-CORE-038 | — |
| #8 | Redaction-aware commitment slots reserved | TR-CORE-071 | TR-OP-071 |
| #9 | Plaintext-vs-committed header policy explicit | TR-CORE-072 | TR-OP-040 |
| #10 | Phase 1 envelope = Phase 3 case-ledger event | TR-CORE-080 | — |
| #11 | "Profile" namespace disambiguation | (spec-prose) | §4 of this matrix |
| #12 | Head formats compose forward; agency log superset | TR-CORE-081, TR-CORE-082, TR-CORE-083 | — |
| #13 | Append idempotency in wire contract | TR-CORE-050, TR-CORE-051, TR-CORE-053 | — |
| #14 | Snapshots and watermarks day-one | TR-CORE-090 | TR-OP-001, TR-OP-002, TR-OP-003, TR-OP-005, TR-OP-006 |
| #15 | Trust posture honesty floor | TR-CORE-100 | (inherits via TR-OP-010..014, TR-OP-040) |

### 3.2 Legacy ID → `TR-NNN` (Traceability)

Every load-bearing legacy ID appears in the `Legacy` column of exactly one consolidated row. A shorthand index:

| Legacy ID range | Maps to |
|---|---|
| ULCR-001..005 (subordination) | TR-CORE-015, TR-CORE-016 |
| ULCR-006..029 (conformance roles) | TR-CORE-130..134, TR-CORE-060..067 |
| ULCR-030..036 (contracts) | TR-CORE-001..007 |
| ULCR-037..040 (terminology / ontology / canonical truth) | TR-CORE-010, TR-CORE-011, TR-CORE-012, TR-CORE-017 |
| ULCR-041..046 (named invariants) | TR-CORE-020, TR-CORE-030, TR-CORE-051, TR-CORE-061, TR-CORE-132 (and see §5 drops) |
| ULCR-047..060 (fact admission, order, attestation) | TR-CORE-040..046, TR-CORE-020..025 |
| ULCR-061..065 (posture declaration) | TR-CORE-100..102 |
| ULCR-066..072 (export) | TR-CORE-062..067 |
| ULCR-073..085 (companion discipline, bindings, sidecars) | TR-CORE-015, TR-CORE-103, TR-CORE-140..143 (and see §5 drops) |
| ULCR-086..094 (supplementary constitutional) | TR-CORE-002, TR-CORE-091, TR-CORE-111..113, TR-OP-034 |
| ULCR-095..103 (integrator-critical) | TR-CORE-020, TR-CORE-030, TR-CORE-060, TR-CORE-120, TR-CORE-125, TR-CORE-126, TR-CORE-046, TR-CORE-023, TR-CORE-143 |
| ULCR-104..109 (Formspec/WOS integration, hash registry) | TR-CORE-121..124, TR-CORE-052, TR-CORE-032 |
| ULCR-110, ULCR-111 (legacy trust-posture invariants) | TR-CORE-100 (via merge) |
| ULCR-115 (determinism) | TR-CORE-025 |
| ULCOMP-R-001..010 (scope) | TR-OP-120, TR-OP-121 |
| ULCOMP-R-011..028 (offline authoring) | TR-OP-050..053 |
| ULCOMP-R-029..037 (reader-held) | TR-OP-015, TR-OP-016 |
| ULCOMP-R-038..055 (delegated compute) | TR-OP-020..022 |
| ULCOMP-R-056..066 (disclosure / export conformance class, selective disclosure) | TR-OP-071..074 |
| ULCOMP-R-088..094 (trust inheritance, scoped export) | TR-CORE-103, TR-OP-122 |
| ULCOMP-R-095..105 (grants, revocations, evaluators) | TR-OP-030..034 |
| ULCOMP-R-106..111 (provider/reader/delegated honesty) | TR-CORE-100 |
| ULCOMP-R-112..127 (CAS, idempotency, proof model, witnessing) | TR-OP-090..092, TR-CORE-051, TR-CORE-053 |
| ULCOMP-R-128..134 (conflict handling) | TR-OP-061 |
| ULCOMP-R-144..147 (protected payloads) | TR-OP-070 |
| ULCOMP-R-148..154 (storage / snapshots) | TR-CORE-091, TR-OP-060 |
| ULCOMP-R-159..168 (erasure, sealing, legal sufficiency, privacy) | TR-CORE-111, TR-CORE-112, TR-OP-080, TR-OP-100, TR-OP-101 |
| ULCOMP-R-169..172 (metadata minimization) | TR-CORE-113 |
| ULCOMP-R-173..180 (sidecar discipline, custody-model examples) | TR-CORE-142, TR-OP-010..014 |
| ULCOMP-R-197 (versioned registries) | TR-OP-130 (reinstated; see §5.4) |
| ULCOMP-R-198..208 (rejection, versioning) | TR-CORE-052, TR-CORE-110, TR-OP-110 |
| ULCOMP-R-209..212 (security testing, migration guidance) | TR-OP-111, TR-OP-112 |
| ULCOMP-R-213..214 (conformance boundary) | TR-CORE-126 |
| ULCOMP-R-215..220 (projection watermark, stale, rebuild) | TR-OP-001..006 |
| ULCOMP-R-221 (metadata budget) | TR-OP-040 |
| ULCOMP-R-222 (verification posture) | TR-OP-041 |
| ULCOMP-R-223 (projection integrity policy) | TR-OP-007 |

### 3.3 Terminology Reconciliation

The legacy matrices use "canonical substrate," "canonical truth," and "canonical record" interchangeably for three different scopes. This matrix uses the scoped vocabulary from `thoughts/product-vision.md`:

| Legacy term | Scoped term (normative) | Definition |
|---|---|---|
| "canonical substrate" (as whole) | *governed canonical substrate* | the union of response ledgers, case ledgers, agency logs, and federation logs maintained by conforming implementations |
| "canonical truth" / "canonical record" (session-scoped) | *response ledger* | hash-chained sequence of events for one Formspec response, scoped to a single respondent session; sealed at submission |
| "canonical truth" / "canonical record" (case-scoped) | *case ledger* | hash-chained sequence of governance events for one case, composing one or more sealed response-ledger heads with WOS governance events |
| "canonical record" (operator-scoped) | *agency log* | append-only log of case-ledger heads (plus metadata and witness timestamps) maintained by an operator |
| (new, Phase 4) | *federation log* | log of agency-log heads witnessed by an independent operator; detects cross-operator equivocation |

All four are Trellis-shaped — same envelope format, same hash construction, same signing profile — applied at different scopes. "Ledger" is always qualified by scope (response ledger / case ledger); "log" is reserved for structures whose entries are other ledgers' heads.

---

## Section 4 — Profile-Namespace Disambiguation (Invariant #11)

Three legacy namespaces shared the letters A–E/F. This matrix applies the renaming required by invariant #11 of `thoughts/product-vision.md`.

### 4.1 Respondent Ledger posture axes — *retain "Profile A/B/C"*

Owner: Formspec Respondent Ledger (upstream, `thoughts/formspec/specs/respondent-ledger-spec.md` §15A Recommended deployment profiles). The letters are kept because they denote orthogonal posture axes (privacy × identity × integrity-anchoring), not custody modes. Within Trellis prose these are always qualified as **"Respondent Ledger Profile A/B/C"** to avoid collision with the legacy companion letters (renamed in §4.3) and the legacy core-draft names (renamed in §4.2).

| Letter | Axis | Retained? |
|---|---|---|
| Respondent Ledger Profile A | Privacy posture | yes |
| Respondent Ledger Profile B | Identity posture | yes |
| Respondent Ledger Profile C | Integrity-anchoring posture | yes |

### 4.2 Legacy core-draft profiles — *renamed "Conformance Classes"*

Owner: Trellis Core (this spec family). Semantically these are conformance tiers, not profiles.

| Legacy core-draft name | New name (Conformance Class) |
|---|---|
| Core profile | Conformance Class: Core |
| Offline profile | Conformance Class: Offline |
| Reader-Held profile | Conformance Class: Reader-Held |
| Delegated-Compute profile | Conformance Class: Delegated-Compute |
| Disclosure profile | Conformance Class: Disclosure |
| User-Held profile | Conformance Class: User-Held (now owned upstream by Formspec Respondent Ledger) |
| Respondent-History profile | Conformance Class: Respondent-History (now owned upstream by Formspec Respondent Ledger) |

### 4.3 Legacy companion-draft Profiles A–E — *renamed "Custody Models" (CM-A … CM-F)*

Owner: Trellis Operational Companion (this spec family). Semantically these are custody arrangements. The canonical identifier set and definitions live in Operational Companion §9.2 (The Six Standard Custody Models); the table below records the legacy→current rename and cross-references the relevant `TR-OP-*` row. CM-F is new to the unified namespace — it has no legacy companion letter.

| Legacy companion letter | Posture described | Current identifier (Custody Model) | Row |
|---|---|---|---|
| Legacy Profile A | Provider-readable | `CM-A` — Provider-Readable Custodial (Companion §9.2) | TR-OP-010 |
| Legacy Profile B | Reader-held with recovery | `CM-B` — Reader-Held with Recovery Assistance (Companion §9.2) | TR-OP-011 |
| Legacy Profile C | Delegated compute | `CM-C` — Delegated Compute (Companion §9.2) | TR-OP-012 |
| Legacy Profile D | Threshold / quorum | `CM-D` — Threshold-Assisted Custody (Companion §9.2) | TR-OP-013 |
| Legacy Profile E | Organizational trust | `CM-E` — Organizational Trust (Companion §9.2) | TR-OP-014 |
| *(no legacy letter — new)* | Client-origin sovereign | `CM-F` — Client-Origin Sovereign (Companion §9.2) | TR-OP-017 |

### 4.4 Phase-scoped Trellis capability tiers — *referred to by phase name*

The product-vision refers to Trellis capability tiers by phase name, not by "profile" letter:

| Phase | Capability tier name |
|---|---|
| Phase 1 | Attested-export tier |
| Phase 2 | Runtime-integrity tier |
| Phase 3 | Portable-case tier |
| Phase 4 | Federation tier (witness / Sovereign variants) |

---

## Section 5 — Gap Log (Legacy Rows Dropped)

Every legacy row not migrated into a `TR-*` row is listed here with a one-sentence justification and a disposition annotation:

- `[confirmed]` — drop is sound against invariants #1–#15 and/or an upstream spec §N.
- `[corrected]` — drop is sound but the original justification cited the wrong invariant, `TR-*` row, or upstream section; the cell below supplies the correct citation.
- `[reinstated as TR-*-NNN]` — drop was unsound; the requirement has been reinstated as a new row in Section 1 or Section 2.

Legacy IDs remain permanently retired (not reused) regardless of drop category.

### 5.1 Dropped: superseded by a product-vision invariant or consolidated `TR-*` row

Legacy-ID citations in this table (and throughout the matrix) resolve to the `specs/archive/core/` version of the core matrix, which is the more complete of the two legacy sources and the one that assigned ULCR-041..046 to structural invariants (`specs/archive/core/unified-ledger-requirements-matrix.md` §6.5, cross-referencing legacy `trellis-core.md` §6.2). The older `thoughts/archive/drafts/` version of the same matrix used those six IDs for a different set of invariants (author-fact/attestation distinguishability, canonical fact/record distinguishability, derived-not-canonical, provider-readable/reader-held distinction, delegated-compute scope, disclosure/assurance distinction); where the DRAFTS meaning differs, the archived-core meaning governs because it is the one Plan 3 consolidated against. DRAFTS coverage for those invariants lands at the TR-CORE rows noted in the "DRAFTS-meaning coverage" column below.

| Legacy ID | Why dropped (archived-core meaning) | DRAFTS-meaning coverage | Disposition |
|---|---|---|---|
| ULCR-041 | Append-only canonical history ("canonical records MUST NOT be rewritten in-place") is covered by TR-CORE-132 (Canonical Append Service MUST NOT rewrite prior canonical records) and TR-CORE-131 (Fact Producer MUST NOT rewrite previously emitted facts). | DRAFTS ULCR-041 ("author-originated fact and canonical append attestation MUST remain distinguishable") is covered by TR-CORE-010 and TR-CORE-040 (five object classes include author-originated fact and canonical append attestation as distinct categories). | [corrected] |
| ULCR-042 | "Derived artifacts MUST NOT be treated as canonical truth" is covered by TR-CORE-002 (Derived Artifact Contract) and TR-CORE-011 / TR-CORE-012 (derived and disclosure artifacts MUST NOT collapse into canonical truth; implementations MUST NOT treat derived artifacts as authoritative). | DRAFTS ULCR-042 ("canonical fact and canonical record MUST remain distinguishable") is covered by TR-CORE-013 (canonical record MUST remain distinguishable from underlying authored content). | [corrected] |
| ULCR-044 | "Canonical append semantics MUST bind to exactly one canonical hash construction" is merged into TR-CORE-030 (which unifies the one-hash-construction rule with the invariant #1 / #4 pinned-encoding and ciphertext-hashing requirements). | DRAFTS ULCR-044 ("provider-readable and reader-held access MUST remain distinct") is covered by TR-CORE-100 and TR-CORE-103 (posture-honesty floor and reader-held vs provider-readable distinction); operational elaboration at TR-OP-015, TR-OP-016. | [corrected] |
| ULCR-045 | "Canonical verification MUST NOT depend on workflow runtime internals" is covered by TR-CORE-061 (Verifier MUST NOT require access to derived runtime state to verify canonical integrity). | DRAFTS ULCR-045 ("delegated compute MUST NOT be treated as blanket provider plaintext access") is covered by TR-CORE-100 / TR-CORE-103 and TR-OP-020..022 (delegated-compute custody rules). | [corrected] |
| ULCR-046 | "Append idempotency — equivalent admitted canonical inputs MUST NOT create duplicate canonical order positions" is covered by invariant #13 and TR-CORE-050 / TR-CORE-051 (idempotency key contract and merged companion idempotency rows). | DRAFTS ULCR-046 ("disclosure posture and assurance MUST remain distinct and MUST NOT be conflated") is out-of-scope for Trellis per §5.2 (owned upstream by WOS Assurance §4 Invariant 6 and Formspec Respondent Ledger §6.4 / §6.6 `assuranceLevel`, `privacyTier`). | [corrected] |

### 5.2 Dropped: out of scope (owned upstream by Formspec Respondent Ledger or WOS)

Citations below are verified against the current upstream spec files; see `specs/cross-reference-map.md` for the authoritative concept-to-section index.

| Legacy ID | Why dropped | Disposition |
|---|---|---|
| ULCR-063 | Disclosure-vs-assurance taxonomy; owned upstream by WOS Assurance §2 (Assurance Levels) + §4 (Invariant 6: Disclosure Posture Is Not Assurance Level) and Formspec Respondent Ledger §6.6 (`privacyTier`) / §6.6.1 (`assuranceLevel`). | [confirmed] |
| ULCR-080 | User-held record reuse capability; owned upstream by Formspec Respondent Ledger §6.6A (Identity and implementation decoupling) + §6.7 (Disclosure tier and assurance are independent). | [confirmed] |
| ULCR-081 | Respondent-history capability; owned upstream by Formspec Respondent Ledger §5 (RespondentLedger object), §6 (RespondentLedgerEvent), and §8 (Event taxonomy). Previous citation ("§6.7") was imprecise; the respondent-history capability is the whole purpose of that spec and not localized to §6.7. | [corrected] |
| ULCR-112 | Legacy Invariant 6 (Disclosure posture and assurance posture MUST remain distinct and MUST NOT be conflated); owned upstream by WOS Assurance §4 (Invariant 6). | [confirmed] |
| ULCOMP-R-067..075 | User-held record reuse; owned upstream by Formspec Respondent Ledger §6.6A + §6.7. | [confirmed] |
| ULCOMP-R-076..087 | Respondent history; owned upstream by Formspec Respondent Ledger §5, §6, §6.6A, §6.7, §8. | [corrected] |
| ULCOMP-R-135..138 | Identity / signing mechanics; owned upstream by WOS Assurance §3 (Subject Continuity) + §5 (Provider-Neutral Attestation). Previous citation ("WOS Assurance" bare) was insufficiently specific. | [corrected] |
| ULCOMP-R-139 | User-signing evidence distinction (authored authentication vs canonical append attestation); the ledger-side distinction is carried by TR-CORE-040 (five object classes include author-originated fact and canonical append attestation as distinct categories). Upstream identity-signing semantics are owned by WOS Assurance §3 + §5. | [corrected] |
| ULCOMP-R-140..143 | Assurance-vs-disclosure taxonomy and subject continuity; owned upstream by WOS Assurance §2 + §3 + §4 (Invariant 6) and Formspec Respondent Ledger §6.6 + §6.6A + §6.6.1. Previous citation was insufficiently specific. | [corrected] |
| ULCOMP-R-155..158 | Generic lifecycle (retention, legal hold, archival, sealing, schema upgrade); owned upstream by WOS Governance §2.9 (Schema Upgrade as Named Lifecycle Operation) + §7.15 (Legal Hold). Ledger-specific cryptographic lifecycle (key destruction, export issuance) is retained at TR-CORE-111 / TR-CORE-112. | [confirmed] |
| ULCOMP-R-161..162 | Sealing / retention precedence; owned upstream by WOS Governance §7.15 (Legal Hold) which establishes precedence over retention policies. Previous citation ("WOS Governance" bare) was insufficiently specific. | [corrected] |
| ULCOMP-R-181..188 | Forms respondent-history sidecar (stable paths, item keys, validation snapshots, amendment cycles, migration outcomes, change sets, history moments, respondent export views); owned upstream by Formspec Respondent Ledger §6.6A + §6.7 (and §7 ChangeSetEntry for change-set semantics, §9 for materiality, §8 for event taxonomy). | [corrected] |
| ULCOMP-R-189..196 | Workflow governance sidecar (workflow mapping, governance facts, review semantics, approval/recovery, provenance family, conflict families, workflow export views); owned upstream by WOS Governance workflow-governance.md §§3–4 (Due Process, Review Protocols) + §8 (Rejection and Remediation) + §11 (Delegation of Authority). Previous citation ("WOS Governance" bare) was insufficiently specific. | [corrected] |

### 5.3 Dropped: duplicate of another consolidated row

| Legacy ID | Why dropped | Disposition |
|---|---|---|
| ULCR-073 | "Companion discipline MUST NOT alter core semantics" is entirely restated by TR-CORE-015 + TR-CORE-016. | [confirmed] |
| ULCR-113 | "Author-fact vs append-attestation distinction" is subsumed by TR-CORE-010 + TR-CORE-040 (object-class distinction includes author-originated fact and canonical append attestation as two of the five classes). | [confirmed] |
| ULCR-114 | "Canonical fact vs canonical record distinction" is subsumed by TR-CORE-013 (canonical record MUST remain distinguishable from underlying authored content). | [confirmed] |
| ULCR-075 | Scoped-export rule folded into TR-OP-122 (custody-model / binding / sidecar exports MUST preserve author / canonical-record / attestation / disclosure distinctions, and MUST NOT imply broader coverage than the declared scope includes). | [confirmed] |

### 5.4 Reinstated: drop was unsound

The following legacy rows were flagged during the gap-log soundness audit as lacking both (a) coverage by any of invariants #1–#15 and (b) a verified upstream home, and have been reinstated as new `TR-*` rows. The original legacy IDs remain retired; reinstated rows carry new IDs in Section 1 or Section 2.

| Legacy ID | Original justification (rejected) | Reinstated as | New home |
|---|---|---|---|
| ULCOMP-R-197 | "Registry conventions; owned upstream by WOS Governance App. A." WOS Governance has no Appendix A, and `specs/cross-reference-map.md` confirms "no confirmed upstream home — registry conventions are not defined in current WOS specs." The obligation (implementations SHOULD define versioned registries for listed identifier/kind categories) is a companion-scope operator duty complementing invariant #6 (which covers the manifest-side snapshot-binding but presumes versioned registries exist). | TR-OP-130 | New row added in §2.14. |

### 5.5 Contradictions between legacy matrices (resolution recorded inline)

Prior-draft conflicts resolved in favor of product-vision invariants, per the authoring contract:

- **Legacy "canonical substrate" as global-scope record vs scoped "response/case/agency/federation ledger."** Resolved in §3.3: scoped vocabulary governs; the singular "canonical substrate" is now "governed canonical substrate" and is always decomposable into the four scope tiers.
- **Legacy companion Profiles A–E as "trust profiles" vs Respondent Ledger "Profile A/B/C" as posture axes.** Resolved in §4: companion letters become Custody Models; posture axes keep the letters because they denote orthogonal axes, not custody arrangements.
- **Legacy core-draft profiles (Core / Offline / Reader-Held / Delegated-Compute / Disclosure / User-Held / Respondent-History) as "Profiles" vs current Trellis Core "Conformance Classes."** Resolved in §4: these become Conformance Classes; the last two are owned upstream.

---

## References

- `thoughts/product-vision.md` — Phase 1 envelope invariants (#1–#15), terminology block, Track E §21.
- `specs/trellis-core.md` — Trellis Core Specification.
- `specs/trellis-operational-companion.md` — Trellis Operational Companion.
- `specs/cross-reference-map.md` — per-row upstream-destination index for rows dropped to Formspec Respondent Ledger or WOS.
- Legacy (non-normative): `thoughts/archive/drafts/unified-ledger-requirements-matrix.md`, `thoughts/archive/drafts/unified-ledger-companion-requirements-matrix.md`, `specs/archive/core/unified-ledger-requirements-matrix.md`, `specs/archive/core/unified-ledger-companion-requirements-matrix.md`. Citations in the `Legacy` column of this matrix are resolved against the `specs/archive/core/` versions.
