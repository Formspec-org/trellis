# Archived Spec Provenance Analysis: `specs/archive/`

**Date:** 2026-04-23
**Status:** Non-normative reference document
**Purpose:** Trace every significant concept from the 13 archived spec files under `specs/archive/` to its current home in the ratified `trellis-core.md` v1.0.0 and `trellis-operational-companion.md` v1.0.0, and assess whether `specs/cross-reference-map.md` accounts for all of them.

---

## Method

For each archived file, every normative concept is mapped to one of five dispositions:

1. **Absorbed into Core** — now lives in a specific section of `trellis-core.md` v1.0.0.
2. **Absorbed into Companion** — now lives in a specific section of `trellis-operational-companion.md` v1.0.0.
3. **Removed to upstream** — the normative home is now WOS Assurance, WOS Governance, WOS Kernel, or Formspec Respondent Ledger.
4. **Deferred** — explicitly reserved for Phase 2/3/4 with a seam or wire-slot in the current specs.
5. **Dropped** — not present in current specs, no upstream home, no deferral slot.

---

## Complete Inventory of `specs/archive/` (13 files across 8 subdirectories)

| # | Subdirectory | File |
|---|---|---|
| 1 | `core/` | `unified-ledger-requirements-matrix.md` |
| 2 | `core/` | `unified-ledger-companion-requirements-matrix.md` |
| 3 | `core/` | `trellis-core.md` |
| 4 | `core/` | `shared-ledger-binding.md` |
| 5 | `workflow/` | `workflow-governance-provenance.md` |
| 6 | `projection/` | `projection-runtime-discipline.md` |
| 7 | `export/` | `export-verification-package.md` |
| 8 | `export/` | `disclosure-manifest.md` |
| 9 | `trust/` | `key-lifecycle-operating-model.md` |
| 10 | `trust/` | `trust-profiles.md` |
| 11 | `operations/` | `monitoring-witnessing.md` |
| 12 | `forms/` | `forms-respondent-history.md` |
| 13 | `assurance/` | `assurance-traceability.md` |

---

## Cross-reference map coverage verdict

The cross-reference map (`specs/cross-reference-map.md`) covers **only removed requirement rows** — concepts extracted from the two requirements-matrix files and relocated to upstream specs during the Plan 3 three-spec reorganization. It does **not** trace concepts that were absorbed into the current Core or Companion, concepts that were restructured or renamed in place, or concepts deferred to future phases.

| Status | Count | Files |
|---|---|---|
| Explicitly named (source docs for removed rows) | 2 | `core/unified-ledger-requirements-matrix.md`, `core/unified-ledger-companion-requirements-matrix.md` |
| Fully accounted by concept (upstream removals) | 2 | `forms/forms-respondent-history.md`, `workflow/workflow-governance-provenance.md` |
| Partially accounted (some concepts mapped upstream, rest untraced in map) | 5 | `export/disclosure-manifest.md`, `trust/trust-profiles.md`, `trust/key-lifecycle-operating-model.md`, `assurance/assurance-traceability.md`, `projection/projection-runtime-discipline.md` |
| Not accounted at all in cross-reference map | 4 | `core/trellis-core.md`, `core/shared-ledger-binding.md`, `export/export-verification-package.md`, `operations/monitoring-witnessing.md` |

**Bottom line:** The cross-reference map accounts for 2 of 13 files explicitly, covers upstream-removal concepts from 7 more partially, and leaves 4 files entirely untraced. However, provenance analysis against the current ratified specs shows that **all 13 files' concepts have been absorbed, removed, deferred, or intentionally dropped** — the gaps are in the cross-reference map's scope, not in the current spec coverage.

---

## 1. `core/trellis-core.md` (v0.1.0-draft.1, 2026-04-14)

This was the "semantic constitution" — no wire format, no CDDL, no byte-level pins. It defined invariants, trust semantics, canonical truth boundaries, and verification boundaries at a constitutional level.

### 1.1 Absorbed into Core

| Archived Concept | Current Location | Notes |
|---|---|---|
| Core object classes (author-originated fact, canonical record, canonical append attestation, derived artifact, disclosure/export artifact) | Core §3 (Terminology), §6 (Event Format) | Renamed/refined: "event" replaces "canonical record" as the atomic append unit; three event surfaces (authored/canonical/signed) in §6.8 |
| Five conformance roles (Fact Producer, CAS, Verifier, Derived Processor, Export Generator) | Core §2.1 | Identical role names, same five classes |
| Append-only canonical history (Invariant 1) | Core §10.5 | "A Canonical Append Service MUST NOT rewrite a canonical event once admitted" |
| No second canonical truth (Invariant 2) | Core §1.2 ("three scopes of append-only structure"), Companion §14.2 (OC-33) | Split: byte-level in Core, operational in Companion |
| One canonical order per governed scope (Invariant 3) | Core §10.4 ("MUST NOT allow competing canonical orders for the same scope") | |
| One canonical event hash construction (Invariant 4) | Core §9.1–9.2 (domain-separated SHA-256) | Now pinned to dCBOR + SHA-256 with domain tags |
| Verification independence (Invariant 5) | Core §16 (Verification Independence Contract) | Explicit normative section |
| Append idempotency (Invariant 6) | Core §17 (Append Idempotency Contract) | Expanded significantly with wire-level details |
| Canonical truth scope (what is/isn't canonical) | Core §1.2, §4 | Explicitly enumerates excluded categories |
| Fact admission categories | Core §6.7 (Extension Registration) | Event types registered per family |
| Canonical order requirements | Core §10 (Chain Construction) | Pinned to strict linear for Phase 1 with DAG reservation |
| Canonical append attestation requirements | Core §11 (Checkpoint Format) | Now COSE_Sign1 over Merkle tree head |
| Export requirement | Core §18 (Export Package Layout) | Now deterministic ZIP with CDDL-pinned manifest |
| Verification requirement | Core §19 (Verification Algorithm) | Full algorithm with fatal/localizable failure classes |
| Export contents | Core §18.2 (Required archive members) | Deterministic ZIP member list |
| Provenance distinction requirement | Core §19 (verification report separates structure/integrity/readability) | Three-boolean output |
| Export verification independence | Core §16 | Normative section |
| Cross-repository authority boundaries (Formspec/WOS/Trellis) | Core §4 (Non-goals and authority boundaries) | Identical delegation structure |
| Core-to-implementation contracts | Core §10.5, §15.1, §16.1 | Distributed into specific sections |
| Controlled vocabulary | Core §3 (Terminology) | Expanded with Trellis-specific terms |

### 1.2 Absorbed into Companion

| Archived Concept | Current Location | Notes |
|---|---|---|
| Semantic invariants D (provider-readable ≠ reader-held) and E (delegated compute ≠ general readability) | Companion §8 (Access Taxonomy) | OC-01 through OC-06 |
| Semantic invariant F (disclosure posture ≠ assurance level) | Companion §11 (Posture-Declaration Honesty) | OC-14, Core §20.4 |
| Derived artifact requirements (rebuild, provenance, evaluator behavior) | Companion §14 (Derived-Artifact Discipline) | OC-32 through OC-36 |
| Versioning and algorithm agility at constitutional level | Companion §22 (Versioning and Algorithm Agility) | OC-86 through OC-93 |
| Metadata minimization (Core §13.2) | Companion §12 (Metadata Budget Discipline) | OC-18 through OC-25 |
| Idempotency and rejection at constitutional level | Companion §18 (Append Idempotency Operational), §21 (Rejection Taxonomy) | OC-53 through OC-85 |
| Storage, snapshots, durable-append boundary | Companion §16 (Snapshot-from-Day-One) | OC-45 through OC-48 |

### 1.3 Removed to Upstream

| Archived Concept | Upstream Home | Notes |
|---|---|---|
| Trust Profile semantics (§4.7, §13.4) | WOS Assurance §4–§6, Formspec Respondent Ledger §15A | Trellis now defines Custody Models, not generic trust profiles |
| Disclosure posture taxonomy (§4.8) | WOS Assurance §4 | Core §20 references it; does not redefine |
| Subject continuity (§4.9) | WOS Assurance §3, Formspec Respondent Ledger §6.6A | Not restated in Trellis |

### 1.4 Deferred

| Archived Concept | Current Reservation | Notes |
|---|---|---|
| DAG-capable chain topology (hinted in §7.4 determinism note) | Core §6.1 `causal_deps`, §10.3 | Phase 2 reservation |
| Alternate proof models (hinted in §7.7) | Core §7.2 (suite_id registry) | Phase 2+ via suite_id registration |

### 1.5 Dropped

| Archived Concept | Notes |
|---|---|
| "Conformance Profiles" as a separate concept | Renamed to "Conformance Classes" (Core §21.2) |
| Three-tier requirement-class markers (Constitutional/Profile/Binding) | Simplified to role-based "Requirement class" labels |
| "Core Profile" as a distinct conformance entity | Dropped; Core §2.1 just lists conformance classes |
| §7.3 Fact Admission State Machine (7 states in a table) | Not reproduced as an explicit state machine. Concepts distributed across Core §6, §17, §19. The explicit table of 7 named states (Originated → Submitted → Admissible → Accepted for Durable Append → Canonical Record Formed → Canonical Append Attested → Exported) is gone. |
| §13.3 Baseline scope constraint ("baseline conformance MUST NOT be interpreted to require advanced selective disclosure, threshold custody...") | Dropped as a standalone section. The non-goals in Core §1.3 partially capture this. Companion §13.6 defers selective disclosure crypto. |
| Semantic invariants A-F as explicitly labeled invariants | Not labeled as invariant letters in current specs. Absorbed into prose requirements. |
| The 8-companion specification table | Consolidated to 2 ratified specs (Core + Companion). |
| §14 Non-normative guidance sections | Not carried forward as a section. The practical reduction rule and implementation guidance are absorbed into CLAUDE.md and inline notes. |

---

## 2. `core/shared-ledger-binding.md` (v0.1.0-draft.2, 2026-04-14)

The wire-level binding spec. JSON/JCS/SHA-256 canonical serialization. This entire spec was **replaced** by the dCBOR/CBOR/COSE architecture in current Core.

### 2.1 Absorbed into Core (with significant re-architecture)

| Archived Concept | Current Location | Notes |
|---|---|---|
| Canonical record envelope | Core §6 (EventPayload CDDL) | **Completely redesigned**: JSON → CBOR; flat fields → structured map with typed sub-maps |
| Canonical serialization (JCS per RFC 8785) | Core §5 (dCBOR per RFC 8949 §4.2.2) | **Encoding changed**: JCS → dCBOR |
| Canonical event hash (SHA-256 over JCS) | Core §9.2 (domain-separated SHA-256 over dCBOR) | **Hash construction changed**: bare SHA-256 → domain-tagged SHA-256 with `CanonicalEventHashPreimage` wrapper |
| `construction_id` field | Core §7.2 (`suite_id` registry) | **Renamed and restructured**: string identifier → integer registry with suite semantics |
| Append head (JSON with root_hash, tree_size, signature) | Core §11 (CheckpointPayload as COSE_Sign1) | **Redesigned**: JSON → COSE_Sign1 over dCBOR Merkle tree head |
| Merkle tree construction (RFC 6962 compatible) | Core §11.3 (RFC-6962-compatible with domain separation) | **Enhanced**: added domain-separated leaf and interior hashes |
| Inclusion proof | Core §11.4, §18.5 (InclusionProof CDDL) | Retained conceptually; CDDL shape changed from JSON to CBOR |
| Consistency proof | Core §11.4, §18.5 (ConsistencyProof CDDL) | Retained conceptually; CDDL shape changed from JSON to CBOR |
| Idempotency identity (`governed_scope`, `idempotency_key`) | Core §17 (`ledger_scope`, `idempotency_key`) | **Field renamed**: `governed_scope` → `ledger_scope` |
| Idempotency-key mismatch rejection | Core §17.5 (`IdempotencyKeyPayloadMismatch`) | Retained |
| Four-layer separation | Core §6.8 (three event surfaces), §9.4 (KeyBag), §12 (EventHeader) | **Restructured**: 4 layers → 3 surfaces (authored/canonical/signed) + header layer policy |
| Family binding matrix | Core §22 (Respondent Ledger), §23 (WOS custodyHook) | **Restructured**: generic family matrix → two specific composition sections |
| Formspec-authored admission path | Core §22 (Composition with Respondent Ledger) | Now a specific composition seam |
| WOS governance admission path | Core §23 (Composition with WOS custodyHook) | Now a specific composition seam |
| Trust fact admission path | Companion §10 (Posture-Transition Auditability) | Now posture transitions as canonical events |
| Release fact admission path | Core §18 (Export Package Layout) | Export is now a deterministic ZIP, not a separate fact family |
| Schema/version compatibility | Core §14 (Registry Snapshot Binding) | **Restructured**: `schema_ref` per record → `RegistryBinding` per export |
| Canonization rules | Core §17.5 (rejection codes), §10.2 (prev_hash), §12.2 (header layer) | Distributed across multiple sections |
| Rejection codes | Core §17.5 | **Code set changed**: `missing_required_field`, `invalid_schema_ref`, `hash_construction_mismatch`, `idempotency_conflict`, etc. → `IdempotencyKeyPayloadMismatch`, `prev_hash_mismatch`, `sequence_gap`, `unknown_suite_id`, `unresolvable_kid`, etc. |
| Canonical receipt immutability | Core §10.5 | Absorbed into append-only invariant |
| Construction ID on every record | Core §7.4 (`suite_id` in COSE protected header) | **Renamed**: `construction_id` → `suite_id`; moved from envelope payload to COSE header |
| Self-contained interpretation material | Core §14 (Registry Snapshot Binding) | **Enhanced**: now embedded in export, content-addressed |
| No silent reinterpretation | Core §14.5 (registry migration discipline) | Retained |
| Construction fixity per scope | Core §7.2, §9 (single hash construction per event) | Retained |
| Registries (family IDs, construction IDs, custody modes, lifecycle kinds, rejection codes) | Core §7.2 (suite_id), §14 (RegistryBinding), §26 (IANA), Companion §9 | **Restructured**: in-spec lists → CDDL registry + IANA considerations |

### 2.2 Absorbed into Companion

| Archived Concept | Current Location | Notes |
|---|---|---|
| Custody mode field semantics | Companion §9 (Custody Models CM-A through CM-F) | **Expanded**: 4 custody modes → 6 custody models with detailed per-model obligations |
| Trust fact admission (profile validation) | Companion §10 (Posture-Transition Auditability) | Now posture transitions with Appendix A.5 CDDL |
| Proof portability (`construction_id` on every proof) | Core §7.4 (`suite_id` in COSE header) + Companion §22 (versioning) | Split between Core wire and Companion operational obligations |
| Protected payload separation obligations | Companion §8 (Access Taxonomy) | OC-01 through OC-06 |
| Security considerations (11 items) | Core §25, Companion §28 | Distributed between both specs |
| Privacy considerations | Core §25.2, Companion §12 (Metadata Budget) | Split between both specs |
| Legal-sufficiency subordination | Core §20.4 | Retained |

### 2.3 Dropped

| Archived Concept | Notes |
|---|---|
| JCS (RFC 8785) as canonical serialization | **Dropped entirely** in favor of dCBOR. No JSON anywhere in current wire format. |
| `payload_digest` as a nested object with `construction_id` | **Dropped**. Replaced by `content_hash` as a direct `digest` field (Core §9.3). |
| `access_material_ref` as a separate envelope field | **Dropped**. Replaced by `key_bag` inline in the event (Core §9.4). |
| `references` array for cross-family references | **Dropped** as explicit envelope field. Cross-family binding now through composition rules in §22–§23. |
| `custody_mode` as an envelope field | **Dropped** from envelope. Custody model declared in PostureDeclaration (Core §20.2) and Companion §9. |
| `schema_ref` envelope field | **Dropped**. Replaced by registry bindings (Core §14) and `event_type` in header (Core §12.1). |
| `author_ref` envelope field | **Dropped**. Replaced by signing key resolution via `kid` (Core §7.4, §8). |
| `governed_scope` field name | **Renamed** to `ledger_scope` (Core §6.1). |
| `family_id` field | **Dropped**. Replaced by `event_type` in `EventHeader` (Core §12.1) with registered `wos.*`, `formspec.*`, `trellis.*` namespaces. |
| `trellis.release` family | **Dropped** as a separate fact family. Export is a ZIP, not an admitted fact. |
| `trellis.trust` family | **Dropped** as a separate fact family. Trust transitions ride in `EventPayload.extensions` (Core §6.7). |
| `trellis-jcs-sha256-v1` construction ID | **Dropped**. No JCS-based construction exists in current specs. |
| §13.3 Trust fact admission path | **Restructured**. Trust facts are now posture-transition events in extensions. |
| §13.4 Release fact admission path | **Dropped**. No separate release fact family. |
| §16.1 Family IDs registry | **Dropped**. Event types are registered per Core §6.7, §14. |
| §16.2 Construction IDs registry | **Dropped**. Replaced by `suite_id` registry (Core §7.2). |
| §16.3 Custody Modes registry | **Restructured**. Replaced by Custody Models registry (Core §26.3, Companion §9). |
| §16.4 Lifecycle Fact Kinds registry | **Dropped** as separate registry. Lifecycle operations absorbed into Companion §20. |
| Annex A.1 Worked Envelope Example (JSON) | **Dropped**. Replaced by Core §29 (CBOR examples). |
| Annex A.3 Relation to Recommended Technology Shape | **Dropped**. The binding instantiated a JSON/JCS shape that no longer exists. |

---

## 3. `core/unified-ledger-requirements-matrix.md` (v0.3.0-draft.1, 2026-04-14)

Explicitly named in the cross-reference map as a source document for removed ULCR rows.

### 3.1 Absorbed into Core

| Archived Concept | Current Location | Notes |
|---|---|---|
| ULCR-001–005 (companion subordination) | Core §2.3 (implicit), §4, §5.2 | Subordination rules absorbed into Core architecture |
| ULCR-006–029 (conformance roles) | Core §2.1, §2.2 | Same five roles |
| ULCR-030–036 (core-to-implementation contracts) | Core §10.5, §15.1, §16.1 | Distributed into specific sections |
| ULCR-037–040 (terminology, ontology, canonical truth scope) | Core §3, §5.1 | Same concepts |
| ULCR-041–046 (named core invariants 1–6) | Core §10.5, §9.1–9.2, §16, §17 | Distributed into specific sections |
| ULCR-047–060 (fact admission, object distinction, canonical order) | Core §6, §10, §11 | Distributed |
| ULCR-069–071 (verification capabilities, provenance, independence) | Core §16, §19 | Distributed |

### 3.2 Absorbed into Companion

| Archived Concept | Current Location | Notes |
|---|---|---|
| ULCR-061–065 (Trust Profile semantics, honesty, transitions) | Companion §9–§11 | |
| ULCR-074–075 (profile trust inheritance, profile-scoped export) | Companion §8, §11, §13 | |
| ULCR-076–079 (standard profiles: offline, reader-held, delegated, disclosure) | Companion §9 (CM-A through CM-E), §13 | **Renamed** from profiles to custody models |
| ULCR-086–092 (derived artifacts, evaluators, metadata, lifecycle, crypto) | Companion §14–§20 | |

### 3.3 Removed to Upstream

| Archived Concept | Upstream Home | Notes |
|---|---|---|
| ULCR-080–081 (User-Held Record Reuse, Respondent History profiles) | Formspec Respondent Ledger §6.6A, §6.7 | Plan 3 refactor moved these upstream |
| ULCF-013–016 (Trust Profile semantics, disclosure vs assurance) | WOS Assurance §4–§6 | |

### 3.4 Dropped

| Archived Concept | Notes |
|---|---|
| ULCR/ULCF identifier namespace | **Replaced** by TR-CORE-*/TR-OP-* in current `trellis-requirements-matrix.md` |
| ULCF-001–040 feature families | **Replaced** by different organization in current matrix |
| ULCR-073 (superseded — companion discipline collapsed into ULCR-001–005) | Confirmed superseded |
| The "Status" column values (Core/Delegated/Companion) | Replaced by simpler tracking in current matrix |

---

## 4. `core/unified-ledger-companion-requirements-matrix.md` (v0.3.0, 2026-04-15)

Explicitly named in the cross-reference map as a source document for removed ULCOMP-R rows.

### 4.1 Absorbed into Core

| Archived Concept | Current Location | Notes |
|---|---|---|
| ULCOMP-R-001–010 (companion scope, subordination) | Core §4, §5.2 | |
| ULCOMP-R-112–122 (CAS obligations, append idempotency, proof model) | Core §10, §17 | |

### 4.2 Absorbed into Companion

| Archived Concept | Current Location | Notes |
|---|---|---|
| ULCOMP-R-029–037 (reader-held profile) | Companion §9 (CM-B) | |
| ULCOMP-R-038–055 (delegated compute) | Companion §9 (CM-C), §19 | |
| ULCOMP-R-056–066 (disclosure/export) | Companion §13, Core §18 | |
| ULCOMP-R-088–094 (trust inheritance, scoped export honesty) | Companion §8, §11, §24 | |
| ULCOMP-R-095–105 (grants, revocations, evaluators) | Companion §25 | |
| ULCOMP-R-106–111 (access categories, profile honesty) | Companion §8, §9, §11 | |
| ULCOMP-R-114–119 (append idempotency detail) | Core §17, Companion §18 | |
| ULCOMP-R-128–134 (conflict handling, rejection) | Companion §21 | |
| ULCOMP-R-144–147 (protected payloads) | Core §9, §12 | |
| ULCOMP-R-148–154 (storage, snapshots) | Companion §16 | |
| ULCOMP-R-159–164 (cryptographic erasure, legal sufficiency) | Core §9.3, §20.4, Companion §20 | |
| ULCOMP-R-165–172 (privacy, metadata minimization) | Companion §12 | |
| ULCOMP-R-173–180 (sidecar discipline, example profiles) | Companion §9, §23, §24 | |
| ULCOMP-R-198–200 (rejection semantics) | Companion §21 | |
| ULCOMP-R-201–208 (versioning) | Companion §22 | |
| ULCOMP-R-215–218 (projection watermark, stale, purge cascade) | Companion §14–§15, §20 | |
| ULCOMP-R-221–223 (metadata budget, verification posture, projection integrity) | Companion §12, §15 | |

### 4.3 Removed to Upstream

| Archived Concept | Upstream Home | Notes |
|---|---|---|
| ULCOMP-R-067–075 (User-Held Reuse) | Formspec Respondent Ledger | Plan 3 |
| ULCOMP-R-076–087 (Respondent History) | Formspec Respondent Ledger | Plan 3 |
| ULCOMP-R-135–138 (identity/signing mechanics) | WOS Assurance | Plan 3 |
| ULCOMP-R-140–142 (assurance vs disclosure taxonomy) | WOS Assurance + Formspec Respondent Ledger | Plan 3 |
| ULCOMP-R-155–162 (generic lifecycle, sealing/precedence) | WOS Governance | Plan 3 |
| ULCOMP-R-181–196 (forms sidecar, workflow sidecar) | Formspec Respondent Ledger, WOS Governance | Plan 3 |
| ULCOMP-R-197 (registry conventions) | WOS Governance | Plan 3 |

### 4.4 Dropped (legacy-only rows with no current owner)

25 rows marked "legacy only — no current owner" in the archived matrix. These fall into:

| Area | Archived IDs | Notes |
|---|---|---|
| Offline authoring profile (18 rows) | ULCOMP-R-011–028 | No current spec owns an explicit "Offline Authoring Profile." Core §6 handles authored form; Companion §18 handles retry semantics. The explicit profile with local pending state, authored-time preservation, and delayed-submission behavior is **unowned**. |
| Conflict handling discretionary rules (3 rows) | ULCOMP-R-128, ULCOMP-R-131, ULCOMP-R-133 | Core §10.5 and Companion §21 handle rejection, but the specific conflict-sensitive fact categories, cross-scope isolation, and resolution-via-later-facts are **unowned**. |
| Sharing-mode discipline (1 row) | ULCOMP-R-101 | **Dropped.** No current spec addresses narrow sharing vs long-lived collaborative membership. |
| Migration guidance SHOULDs (3 rows) | ULCOMP-R-210–212 | **Dropped.** Non-normative migration guidance was not carried forward. |

---

## 5. `projection/projection-runtime-discipline.md` (v0.1.0-draft.3, 2026-04-15)

The cross-reference map only captured the single respondent-history concept (ULCOMP-R-078); the full projection discipline now lives in Companion §14–§15, §16, §20, §24, §25.

### 5.1 Absorbed into Companion

| Archived Concept | Current Location | Notes |
|---|---|---|
| Canonical truth boundary | Companion §14.1–14.2 | OC-32, OC-33 |
| Derived artifact requirements (rebuild, provenance, non-authoritative) | Companion §14.1–14.6 | OC-32 through OC-36 |
| Projection categories (Consumer-Facing, System) | Companion §15.1 | OC-37 |
| Projection watermark contract | Companion §14.1, Core §15.2 | Watermark CDDL in Core; operational obligations in Companion |
| Rebuild contract and verification | Companion §15.3 | OC-39, OC-40 |
| Projection integrity policy (integrity sampling) | Companion §15.5–15.6 | OC-42 through OC-44 |
| Authorization evaluator behavior | Companion §25 | OC-115 through OC-124 |
| Workflow state and canonical fact mapping | Companion §24 | OC-104 through OC-113 |
| Storage, snapshots, availability | Companion §16 | OC-45 through OC-48 |
| Purge-cascade requirement | Companion §20.3–20.5 | OC-75 through OC-77, Appendix A.7 |
| Runtime boundary | Companion §24.8 | OC-111 |

### 5.2 Absorbed into Core

| Archived Concept | Current Location | Notes |
|---|---|---|
| Watermark CDDL shape | Core §15.2 (Watermark) | Pinned CDDL |
| Rebuild-output encoding (dCBOR) | Core §15.3 | "Rebuilt derived artifacts MUST use dCBOR" |

### 5.3 Dropped

| Archived Concept | Notes |
|---|---|
| Three explicit conformance roles (Projection Producer, Projection Verifier, Authorization Evaluator) | **Restructured**. Projection Producer and Authorization Evaluator are now Companion §6.4 roles. Projection Verifier role is not separately named. |
| §12 Provenance family semantics | **Dropped** as a standalone concept. Provenance is handled through Core §22–§23 composition and Companion §24 sidecar. |
| §11 Workflow state and canonical fact mapping (detailed) | **Absorbed** into Companion §24 at a higher level; some specific mapping detail lost. |

---

## 6. `export/export-verification-package.md` (v0.1.0-draft.2, 2026-04-14)

The cross-reference map does not trace this file at all. Export verification concepts are substantially absorbed into Core §16, §18, §19 and Companion §13.

### 6.1 Absorbed into Core

| Archived Concept | Current Location | Notes |
|---|---|---|
| Export requirement | Core §18 | Deterministic ZIP |
| Export package contents | Core §18.2 | Required + optional archive members |
| Verifier obligations | Core §19 | Full algorithm |
| Export verification independence | Core §16 | Normative section |
| Provenance distinction requirement | Core §19 | Three-boolean verification report |
| Claim classes | Core §19 (VerificationReport CDDL) | `structure_verified`, `integrity_verified`, `readability_verified` |
| Algorithm agility and historical verifiability | Core §7.3, §14, Companion §22 | Split between both specs |
| Self-contained verifiable object design goal | Core §16.1 ("air-gapped laptop") | |
| Profile-scoped export honesty | Core §20 (PostureDeclaration) | |

### 6.2 Absorbed into Companion

| Archived Concept | Current Location | Notes |
|---|---|---|
| Selective disclosure discipline | Companion §13 | OC-26 through OC-31 |
| Export claim classes as a named concept | Companion §13 (`commitment_proofs`) | Restructured into selective disclosure |

### 6.3 Dropped

| Archived Concept | Notes |
|---|---|
| "Export Verification Package" as a named artifact class | **Renamed** to "export package" (Core §18) |
| Separate conformance roles (Export Generator, Verifier) as companion-specific | **Absorbed** into Core §2.1 conformance classes |
| Profile subordination section (§2.3) | **Dropped** as separate section. Core §4 handles authority boundaries. |

---

## 7. `export/disclosure-manifest.md` (v0.1.0-draft.2, 2026-04-14)

The cross-reference map only traced disclosure posture and Invariant 6; the manifest structure and companion-level concepts are untraced in the map but live in Companion §13 and Core §18, §20.

### 7.1 Absorbed into Companion

| Archived Concept | Current Location | Notes |
|---|---|---|
| Manifest structure | Companion §13.3 | OC-27 |
| Audience scope declaration | Companion §13.3 (`audience` field) | |
| Disclosure posture | Companion §13 (OC-26 through OC-31) | |
| Selective disclosure discipline | Companion §13.1–13.6 | Including deferral of advanced crypto |
| Coverage honesty | Companion §13.4 (OC-28), §23.9 (OC-102) | |
| Posture and assurance non-conflation | Core §20.4, Companion §28 | |
| Relationship to export verification package | Core §18 (embedded in export) | |

### 7.2 Dropped

| Archived Concept | Notes |
|---|---|
| Separate conformance roles (Disclosure Producer, Disclosure Verifier, Disclosure Consumer) | **Dropped** as named roles. Absorbed into Operator, Auditor in Companion §6.4. |
| §11 Posture and Assurance Non-Conflation as standalone section | **Distributed** across Core §20.4 and Companion §28.8. |

---

## 8. `trust/key-lifecycle-operating-model.md` (v0.1.0-draft.2, 2026-04-14)

The cross-reference map only traced the generic-lifecycle rows that moved upstream to WOS Governance; the crypto-specific key-management concepts are untraced in the map.

### 8.1 Absorbed into Core

| Archived Concept | Current Location | Notes |
|---|---|---|
| Signing key lifecycle states | Core §8.4 (SigningKeyStatus: Active/Rotating/Retired/Revoked) | Simplified: 5 archived states → 4 current states |
| Key rotation and versioning | Core §8.4 (lifecycle transitions), §8.6 (LedgerServiceWrapEntry) | LAK rotation handled via append-only wrap entries |
| Key identifier construction | Core §8.3 (derived kid from SHA-256 of suite_id \|\| pubkey) | Pinned derivation formula |
| Registry snapshot in export | Core §8.5 | Every export includes complete registry snapshot |
| Algorithm agility | Core §7.3 | Migration obligation |

### 8.2 Absorbed into Companion

| Archived Concept | Current Location | Notes |
|---|---|---|
| Lifecycle operations (retention, legal hold, archival, sealing) | Companion §20 | OC-71 through OC-74 |
| Erasure and key destruction | Companion §20.3–20.6 | OC-75 through OC-78 |
| Sealing and later lifecycle facts | Companion §20.2 | OC-73 |
| Legal sufficiency statement | Core §20.4 | |
| Threshold and quorum custody | Companion §9 (CM-D) | |
| Recovery authorities | Companion §9.3 (`recovery_authorities`) | |
| Required completeness rule (purge cascade) | Companion §20.5, Appendix A.7 | OC-77 |

### 8.3 Deferred

| Archived Concept | Current Reservation | Notes |
|---|---|---|
| Threshold custody proof-of-quorum format | Companion §26 (Phase 4 seam) | |

### 8.4 Dropped

| Archived Concept | Notes |
|---|---|
| Five key classes (Tenant root, Scope, Subject, Signing, Recovery-only) | **Simplified**. Current Core §8 only defines `SigningKeyEntry`. Tenant root, scope, subject, and recovery-only keys are not distinct CDDL types. |
| Key Lifecycle Manager and Key Lifecycle Auditor as conformance roles | **Dropped** as explicit roles. Absorbed into Operator and Auditor in Companion §6.4. |
| Explicit key destruction evidence format | **Dropped**. Crypto-shredding addressed through `content_hash` over ciphertext + key bag + Companion purge cascade. No separate evidence format. |
| Grace periods for key rotation | **Dropped**. Core §8.4 handles rotation via Active → Rotating → Retired transitions without explicit grace-period parameters. |

---

## 9. `trust/trust-profiles.md` (v0.1.0-draft.3, 2026-04-14)

The largest archived companion (647 lines). Almost entirely absorbed into Companion Part I (§8–§13) and Core §20. The cross-reference map only traced the Posture Declaration seam; the full custody model taxonomy, profile transitions, metadata budget, and verification posture classes are untraced in the map.

### 9.1 Absorbed into Companion

| Archived Concept | Current Location | Notes |
|---|---|---|
| Trust Profile object semantics (§3) | Companion §9 (Custody Models) | **Renamed**: "Trust Profile" → "Custody Model" |
| Trust honesty requirements (§4.1) | Companion §11 (Posture-Declaration Honesty) | OC-13 through OC-17 |
| Trust profile transitions (§4.2) | Companion §10 (Posture-Transition Auditability) | OC-07 through OC-12 |
| Baseline Profile A (provider-readable) | Companion §9 (CM-A: Provider-Readable Custodial) | **Renamed** |
| Baseline Profile B (reader-held with recovery) | Companion §9 (CM-B: Reader-Held with Recovery Assistance) | **Renamed** |
| Baseline Profile C (delegated compute) | Companion §9 (CM-C: Delegated Compute) | **Renamed** |
| Baseline Profile D (threshold) | Companion §9 (CM-D: Threshold-Assisted Custody) | **Renamed** |
| Baseline Profile E (organizational trust) | Companion §9 (CM-E: Organizational Trust) | **Renamed** |
| Metadata Budget (§6) | Companion §12 (Metadata Budget Discipline) | OC-18 through OC-25 |
| Profile declaration schema (§8) | Companion Appendix A | A.1 through A.6 |
| Access semantics and profile honesty detail (§9) | Companion §8, §9.4 | OC-01 through OC-06 |
| Standard profiles (§10) | Companion §9.2–9.4 | Six standard models with per-model obligations |
| Example profiles A-E (§13) | Companion §9.4 | Model-specific obligations |
| Audit and conformance hooks (§12) | Companion §27 | OC-134 through OC-139 |

### 9.2 Absorbed into Core

| Archived Concept | Current Location | Notes |
|---|---|---|
| Trust Profile wire shape (referenced from Core) | Core §20.2 (PostureDeclaration CDDL) | Compact CDDL in export manifest |
| Verification posture classes (§7) | Core §20 (PostureDeclaration CDDL) | Absorbed into Core wire format |
| Verification posture as export metadata | Core §18.3 (`posture_declaration` field) | |

### 9.3 New in Current Specs (not in archive)

| New Concept | Current Location | Notes |
|---|---|---|
| CM-F (Client-Origin Sovereign) | Companion §9.2 | **Added**. Not in archived profiles. |
| Disclosure-profile axis (Respondent Ledger Profile A/B/C) | Core §21.1–21.2, Companion Appendix A.5.2 | **Added**. Separate posture axis from custody models. |
| Delegated-Compute Declaration Document | Companion §19.9, Appendix A.6 | **Added**. Much more detailed than archived delegated compute profile. |
| Cascade-Scope Enumeration (CS-01 through CS-06) | Companion Appendix A.7 | **Added**. Machine-checkable purge cascade scope. |
| Conformance tiers (OP-1, OP-2, OP-3, OP-W) | Companion §6.3 | **Added**. Replaces profile-level conformance claims. |

### 9.4 Dropped

| Archived Concept | Notes |
|---|---|
| "Profile" terminology for custody models | **Renamed** to "Custody Model" per Core §21.2 |
| Trust Profile Publisher, Verifier, Auditor as separate conformance roles | **Dropped**. Absorbed into Companion §6.4 (Operator, Auditor). |
| Profile conformance (claiming a specific profile) | **Restructured** as Companion §6.3 conformance tiers (OP-1, OP-2, OP-3). |
| §11 Relationship to Export Claim Classes | **Dropped** as standalone section. Absorbed into Core §19 verification report structure. |
| Example Profile D (threshold) with detailed proof-of-quorum format | **Deferred** to Phase 4 (Companion §26). Current CM-D has obligations but no quorum wire format. |

---

## 10. `operations/monitoring-witnessing.md` (v0.1.0-draft.2, 2026-04-14)

The cross-reference map does not trace this file at all. Witnessing concepts are absorbed into Companion §26 (Phase 4 seams) and Core §11.5, §16.3, §18.3 (anchor infrastructure).

### 10.1 Absorbed into Companion (Part V, §26)

| Archived Concept | Current Location | Notes |
|---|---|---|
| Witness subordination to canonical correctness | Companion §26.2 | OC-125 |
| Monitor and witness sub-roles (4 sub-roles) | Companion §26.5 | OC-128 |
| Checkpoint publication interface | Companion §26.3 | OC-126 |
| Append-head consistency checking | Companion §26.4 | OC-127 |
| Witness attestation semantics | Companion §26.6 | OC-129 (seam) |
| Equivocation definition and evidence format | Companion §26.7 | OC-130 (seam) |
| Detection vs enforcement separation | Companion §26.8 | OC-131 |

### 10.2 Absorbed into Core

| Archived Concept | Current Location | Notes |
|---|---|---|
| `anchor_ref` field | Core §11.5 (CheckpointPayload.anchor_ref) | Optional opaque reference for external anchoring |
| External anchors in export | Core §18.3 (external_anchors array), §16.3 | |

### 10.3 Deferred

| Archived Concept | Current Reservation | Notes |
|---|---|---|
| Full witness network topology | Companion §26 (Phase 4) | Seam only |
| Consensus protocols | Companion §26 (Phase 4) | Seam only |
| Quorum election mechanisms | Companion §26 (Phase 4) | Seam only |
| Equivocation enforcement (beyond detection) | Companion §26.8 (Phase 4) | Detection defined; enforcement deferred |
| Wire formats for witness attestations | Core §6.7 (`trellis.witness_signature.v1` registered) | Registration exists; implementation deferred |

### 10.4 Specificity Regressions

| Archived Concept | Archive Detail | Current Detail | Notes |
|---|---|---|---|
| Checkpoint publication interface | 8-row table with specific resources (scope ID, checkpoint ID, append position, append-head ref, checkpoint time, consistency proof, inclusion proof, pagination) | Companion §26.3 table with 8 rows | **Comparable specificity**. Table structure similar. |
| Witness attestation semantics | Detailed semantics for 4 attestation properties | Companion §26.6: "SEAM DEFINED; IMPLEMENTATION DEFERRED TO PHASE 4" | **Regression**: archive had detailed semantics; current is seam-only |
| Equivocation evidence format | Detailed definition and evidence structure | Companion §26.7: "SEAM DEFINED" | **Regression**: archive had more detail |

---

## 11. `assurance/assurance-traceability.md` (v0.1.0-draft.3, 2026-04-14)

The cross-reference map only traced assurance taxonomy and Invariant 6 upstream; the companion's unique content (verification methodology, evidence retention) is untraced.

### 11.1 Absorbed into Companion

| Archived Concept | Current Location | Notes |
|---|---|---|
| Invariant scope and assurance methodology | Companion §27 (Operational Conformance Tests) | OC-134 through OC-139 |
| Minimum CI expectations | Companion §27 | |
| Evidence retention policy | Companion §27 | |

### 11.2 Absorbed into Core (via requirements matrix)

| Archived Concept | Current Location | Notes |
|---|---|---|
| Operational traceability matrix (Appendix A) | Current `trellis-requirements-matrix.md` (TR-CORE, TR-OP rows) | Replaced by current requirements matrix |

### 11.3 Dropped

| Archived Concept | Notes |
|---|---|
| "Assurance Artifact" as a formal concept | **Dropped**. Companion §27 describes test categories but not a formal "assurance artifact" type. |
| Primary/Secondary method distinction | **Dropped**. |
| Explicit evidence retention policy section | **Dropped** as standalone concept. Companion §27 mentions evidence but doesn't specify retention. |

---

## 12. `workflow/workflow-governance-provenance.md` (v0.1.0-draft.0, stub, 2026-04-14)

68-line stub. Planned content now lives in Companion §24 (Workflow Governance Sidecar).

| Archived Concept | Current Location | Notes |
|---|---|---|
| Workflow sidecar subordination | Companion §24.1–24.4 | |
| Workflow governance sidecar shape (planned) | Companion Appendix B.2 | Minimal shape |
| custodyHook binding | Companion §24.9 | OC-112, OC-113 |
| Inbound requirements (ULCOMP-R-189–196) | **Removed to upstream** (WOS Governance) per Plan 3 | |

---

## 13. `forms/forms-respondent-history.md` (v0.1.0-draft.0, stub, 2026-04-14)

68-line stub. Planned content now lives in Companion §23 (Respondent History Sidecar).

| Archived Concept | Current Location | Notes |
|---|---|---|
| Forms sidecar subordination | Companion §23.1–23.2 | |
| Respondent History sidecar shape (planned) | Companion Appendix B.1 | Minimal shape |
| Inbound requirements (ULCOMP-R-181–188) | **Removed to upstream** (Formspec Respondent Ledger) per Plan 3 | |

---

## Summary: Dropped Without Replacement

Concepts present in the archived spec family that have **no current home** in any ratified spec, no upstream home, and no deferral slot:

| Concept | Archived Source | Notes |
|---|---|---|
| Fact Admission State Machine (7 named states) | `trellis-core.md` §7.3 | The explicit 7-state table is gone. The semantics survive in distributed form, but the state machine is not reproduced as a named artifact. |
| Offline Authoring Profile (18 requirements) | companion matrix ULCOMP-R-011–028 | No current spec owns an explicit offline-authoring profile. Core handles authored form; Companion handles retry. The specific local-pending-state, authored-time-preservation, and delayed-submission concepts are **unowned**. |
| Conflict-sensitive fact categories and cross-scope isolation | companion matrix ULCOMP-R-128, ULCOMP-R-131, ULCOMP-R-133 | Core §10.5 and Companion §21 handle rejection, but specific conflict categories and cross-scope isolation rules are **unowned**. |
| Sharing-mode discipline | companion matrix ULCOMP-R-101 | Narrow sharing vs long-lived collaborative membership. **Unowned**. |
| Non-normative migration guidance (3 SHOULDs) | companion matrix ULCOMP-R-210–212 | Offline coordination scope reduction, offline capability reservation, draft/canonical separation. **Unowned** (non-normative). |
| Five key classes (Tenant root, Scope, Subject, Signing, Recovery-only) | `key-lifecycle-operating-model.md` §2 | Current Core only defines `SigningKeyEntry`. The other four classes have no CDDL type. |
| Key destruction evidence format | `key-lifecycle-operating-model.md` §6 | No separate evidence format in current specs. |
| Grace periods for key rotation | `key-lifecycle-operating-model.md` §4 | No explicit grace-period parameters in current Core §8.4. |
| Primary/Secondary assurance method distinction | `assurance-traceability.md` §2.2 | **Dropped.** |
| "Assurance Artifact" formal concept | `assurance-traceability.md` §2.1 | **Dropped.** |

---

## Summary: Deferred with Reservation

Concepts that have an explicit wire slot, registered identifier, or seam in current specs but whose full implementation is Phase 2+:

| Concept | Current Reservation | Phase | Archived Source |
|---|---|---|---|
| DAG-capable chain topology | Core §6.1 `causal_deps`, §10.3 | Phase 2 | `trellis-core.md` §7.4 |
| Per-field cryptographic commitments (Pedersen, BBS+, Merkle) | Core §13 (Commitment slots) | Phase 2+ | `shared-ledger-binding.md`, `disclosure-manifest.md` |
| Post-quantum signature suites (ML-DSA, SLH-DSA, hybrid) | Core §7.2 (suite_id 2–4 reserved) | Phase 2+ | `shared-ledger-binding.md` |
| HLC-ordered causal DAG | Core §6.1 `causal_deps` | Phase 2 | `trellis-core.md` §7.4 |
| Case ledger composition | Core §22.4–22.5, §24 | Phase 3 | `trellis-core.md` §1.2 |
| Agency log | Core §24 (Phase 3 preview) | Phase 3 | `trellis-core.md` §1.2 |
| Federation log | Core §1.2 | Phase 4 | `trellis-core.md` §1.2 |
| Witness cosignatures | Core §6.7 `trellis.witness_signature.v1` | Phase 4 | `monitoring-witnessing.md` |
| Equivocation enforcement | Companion §26.8 | Phase 4 | `monitoring-witnessing.md` |
| Threshold custody quorum format | Companion §9 (CM-D) | Phase 4 | `key-lifecycle-operating-model.md` §9 |
| OpenTimestamps / transparency-log anchoring | Core §11.5 (`anchor_ref`) | Phase 2+ | `monitoring-witnessing.md` §8 |

---

## Summary: Specificity Regressions

Where current specs are **less detailed** than the archived drafts on the same topic:

| Topic | Archived Detail | Current Detail | Impact |
|---|---|---|---|
| Witness attestation semantics | `monitoring-witnessing.md` §7: detailed 4-property attestation semantics | Companion §26.6: "SEAM DEFINED; IMPLEMENTATION DEFERRED TO PHASE 4" | Low — Phase 4 content, seam is intentional |
| Equivocation evidence format | `monitoring-witnessing.md` §9: detailed evidence structure | Companion §26.7: seam only | Low — Phase 4 content |
| Key lifecycle state machine | `key-lifecycle-operating-model.md` §3: explicit state/transition table for all 5 key classes | Core §8.4: `SigningKeyStatus` 4-value enum with legal transitions listed in prose | Medium — signing keys well-specified; other key classes lack lifecycle specification |
| Fact admission state machine | `trellis-core.md` §7.3: 7-state table | No explicit state machine table in current specs | Low — semantics survive in distributed form |
| Offline authoring profile | 18 detailed requirements in companion matrix | No dedicated section in current specs | Medium — no current spec owns this concept |

---

## Summary: Key Renames

| Archived Term | Current Term | Source |
|---|---|---|
| "Profile A" (provider-readable) | "CM-A" (Provider-Readable Custodial) | Core §21, Companion §9 |
| "Profile B" (reader-held) | "CM-B" (Reader-Held with Recovery Assistance) | Core §21, Companion §9 |
| "Profile C" (delegated compute) | "CM-C" (Delegated Compute) | Core §21, Companion §9 |
| "Profile D" (threshold) | "CM-D" (Threshold-Assisted Custody) | Core §21, Companion §9 |
| "Profile E" (organizational) | "CM-E" (Organizational Trust) | Core §21, Companion §9 |
| — (no equivalent) | "CM-F" (Client-Origin Sovereign) | **New in current** |
| "Conformance Profiles" | "Conformance Classes" | Core §21.2 |
| `construction_id` | `suite_id` | Core §7.2 |
| `governed_scope` | `ledger_scope` | Core §6.1 |
| `family_id` | `event_type` (registered) | Core §12.1 |
| "Trust Profile" | "Custody Model" | Core §21, Companion §9 |
| "Append Head" (JSON) | "Checkpoint" (COSE_Sign1) | Core §11 |
| "Canonical Record Envelope" (JSON) | "EventPayload" (CBOR) | Core §6 |
| "ULCR-###" | "TR-CORE-###" | Requirements matrix |
| "ULCOMP-R-###" | "TR-OP-###" | Requirements matrix |

---

*End of archived spec provenance analysis.*
