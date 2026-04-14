# Trellis Spec Family Normalization Plan (Convergence Pass)

## 1) Assessment of the proposed split

The current split is directionally correct and should be preserved.

- **Keep one small core** for constitutional invariants only.
- **Keep companion specs** for trust/key/projection/export/monitoring details.
- **Demote research/reviews/ADRs** to rationale and incubation.
- **Promote profile material** into profile-oriented artifacts that can later become machine-readable sidecars.

### Direct answers on structure

1. **Is the current Trellis Core still too large?**
   - Slightly yes. The core should be reduced to constitution-level semantics only:
     canonical plane, append/attestation contract, one canonical hash construction, canonical vs derived boundary, and independent verification envelope.

2. **Should Trust Profiles and Key Lifecycle stay separate?**
   - Yes. Keep separate.
   - Trust Profiles = externally honest custody/readability posture.
   - Key Lifecycle = operational cryptographic state machine.

3. **Should Projection and Runtime Discipline partly defer to WOS runtime?**
   - Yes. Trellis should define only cross-system invariants:
     projection watermarking, rebuildability, snapshot provenance, purge cascade obligations.
   - WOS should own runtime orchestration semantics.

4. **Should Export and Disclosure be one companion or two?**
   - Split into two companions.
   - Export Verification Package and Disclosure Semantics are coupled but not identical maturity domains.

5. **Is Monitoring and Witnessing worth making a named companion now, even if minimal?**
   - Yes. Keep it minimal and seam-oriented now to prevent architecture drift later.

## 2) Recommended final Trellis spec family

## Normative set (priority order)

1. **Trellis Core Specification**
   - Constitutional semantics only.
   - Owns canonical append-attested order per governed scope, canonical vs derived separation, canonical event hash construction, append idempotency baseline, and independent verification obligations.

2. **Shared Ledger Binding Specification**
   - Family bindings and canonization discipline.
   - Defines family IDs, minimum record fields, schema/version rules, and binding constraints across Formspec, WOS, trust/access, and release families.

3. **Trust Profile Specification**
   - Reader-held default, provider-readable, tenant-operated key plane.
   - Includes metadata budget, disclosure authority, delegated compute honesty requirements.

4. **Key Lifecycle Operating Model Specification**
   - Key classes, lifecycle states, rotation, grace periods, recovery, destruction/crypto-shredding, historical verification under key evolution.

5. **Projection & Rebuild Discipline Specification**
   - Projection watermarking, rebuild contract, snapshot rules, purge cascades for plaintext derivatives, runtime boundary constraints.

6. **Export Verification Package Specification**
   - Offline-verifiable package shape, trust profile declaration carriage, payload readability declaration, schema/version references, optional anchoring seam.

7. **Disclosure Manifest Specification**
   - Audience-scoped disclosure manifests, claim class declarations, selective disclosure semantics, provenance preservation rules.

8. **Monitoring & Witnessing Companion** (minimal now)
   - Checkpoint publication seams, anti-equivocation-compatible publication, append growth verification hooks, monitor/witness interoperability targets.

## Profile/sidecar artifacts (non-constitutional)

- **Trust profile instance catalog** (profile sidecars; later machine-readable).
- **Deployment profile catalog** (reader-held cloud, provider-readable regulated, tenant-operated dedicated).
- **Metadata budget matrices by fact family**.
- **Capability declaration sidecars** (anchoring mode, delegated compute mode, disclosure mode).

## Rationale/incubation set

- ADRs, technology surveys, expert reviews, risk memos, and advanced crypto exploration (BBS/MLS/FHE/PHE/MPC) remain rationale unless and until promoted.

## 3) Mapping from current Trellis docs into the new family

### 3.1 Major cluster mapping (decision level)

| Current cluster | New home | Level | Action |
|---|---|---|---|
| `DRAFTS/unified_ledger_core.md` | Trellis Core + split-outs | Normative | **Split**: remove profile-heavy and operational sections to companions. |
| `DRAFTS/unified_ledger_companion.md` | Companion set | Companion | **Decompose** into Shared Binding, Trust Profiles, Key Lifecycle, Projection/Rebuild, Export, Disclosure, Monitoring. |
| `thoughts/formspec/specs/respondent-ledger-spec.md` | Shared Ledger Binding (Formspec family) | Companion binding | **Retain as bound family spec**; defer authored semantics authority to Formspec. |
| `thoughts/formspec/adrs/0059-*.md` | Rationale for unified-canonical direction | Rationale | **Demote** from primary surface; mine for binding constraints and migration rationale. |
| `thoughts/formspec/adrs/0054-*.md` | Trust/Profile and Key Lifecycle rationale | Rationale | **Demote**; preserve chain model insights but avoid normative overlap with Formspec. |
| `thoughts/specs/2026-04-10-unified-ledger-concrete-proposal.md` | Implementation profile + rationale extracts | Rationale / profile seed | **Split** into (a) normative candidate clauses and (b) implementation playbook. |
| `thoughts/research/*` surveys and risk memo | Architecture rationale and option records | Rationale | **Retain as research baseline**; do not treat as normative. |
| `thoughts/reviews/*` expert and crypto reviews | Assurance rationale + hardening backlog | Rationale | **Retain**; promote only stable constraints to normative companions. |

### 3.2 File-by-file normalization actions

| Current file | Target classification | Target home | Decision |
|---|---|---|---|
| `DRAFTS/unified_ledger_core.md` | Normative core | `specs/trellis-core.md` | Keep, but reduce to constitutional invariants only. |
| `DRAFTS/unified_ledger_companion.md` | Companion source material | Split across `specs/companions/*` | Decompose into named companions; remove omnibus role. |
| `thoughts/specs/2026-04-10-unified-ledger-concrete-proposal.md` | Rationale + implementation profile seed | `rationale/implementation-baseline.md` + excerpts into companions | Split; keep non-normative as baseline profile guidance. |
| `thoughts/research/2026-04-10-unified-ledger-technology-survey.md` | Rationale | `rationale/technology-survey.md` | Retain as option-analysis record. |
| `thoughts/research/ledger-risk-reduction.md` | Rationale | `rationale/risk-reduction.md` | Retain; use as anti-bespoke design guardrail. |
| `thoughts/research/tiered-privacy-white-paper-3-24-2025.md` | Rationale (cross-cutting) | `rationale/tiered-privacy-background.md` | Retain; do not make Trellis normative text. |
| `thoughts/reviews/2026-04-10-expert-panel-unified-ledger-review.md` | Rationale + assurance input | `rationale/expert-review-2026-04-10.md` | Retain; promote only stable requirements. |
| `thoughts/reviews/2026-04-11-crypto-expert-concrete-solutions.md` | Rationale + hardening backlog | `rationale/crypto-solutions-2026-04-11.md` | Retain; extract normative clauses selectively. |
| `thoughts/formspec/specs/respondent-ledger-spec.md` | Companion binding (defer authority) | Trellis shared-ledger binding annex for Formspec family | Keep as binding input; Formspec remains semantic authority. |
| `thoughts/formspec/proposals/user-side-audit-ledger-add-on-proposal.md` | Rationale/proposal | `rationale/formspec-add-on-proposal.md` | Demote from normative status. |
| `thoughts/formspec/adrs/0054-privacy-preserving-client-server-ledger-chain.md` | Rationale/ADR | `rationale/formspec-adr-0054.md` | Demote; preserve trust-chain conclusions. |
| `thoughts/formspec/adrs/0059-unified-ledger-as-canonical-event-store.md` | Rationale/ADR | `rationale/formspec-adr-0059.md` | Demote; preserve canonical substrate conclusions. |

## 4) Explicit repo-boundary decisions (Trellis vs WOS vs Formspec)

### Formspec owns

- Authored/spec/runtime semantics for form responses.
- Respondent authoring semantics, authored event meaning, and authored continuity behavior (including browser-meaningful authoring).
- Form-specific validation and response object semantics.

### WOS owns

- Workflow/governance semantic meaning and runtime envelope.
- Case lifecycle execution semantics, orchestration semantics, and governance interpretation.
- Runtime/coproc behaviors that consume canonical facts.

### Trellis owns

- Canonical ledger semantics and append/attestation contract.
- Canonical order per governed scope constraints.
- Canonical event hash construction discipline.
- Trust/custody/disclosure semantics at shared substrate level.
- Export/verifier package semantics.
- Projection/rebuild/snapshot/purge discipline shared across Formspec and WOS consumers.

### Trellis must bind

- Formspec-family facts and WOS-family facts into one governed canonical substrate.
- Trust/access/release fact families into consistent canonization and verification rules.

### Trellis must NOT redefine

- Formspec authored meaning.
- WOS workflow meaning.
- Product UX and runtime orchestration specifics.
- Late-phase privacy-tech requirements as baseline obligations.

## 5) Top 5 drafting priorities

1. **Shrink core further** to constitutional invariants only, with zero profile prose leakage.
2. **Finalize one canonical event hash construction** with deterministic serialization and subordinate-hash discipline.
3. **Stabilize Trust Profiles + Key Lifecycle pair** including metadata budget and grace-period semantics.
4. **Lock Projection/Rebuild discipline** including provenance watermark requirement for staff-facing derived views and purge-cascade requirement for crypto-shredding completeness.
5. **Split and stabilize release surface** into Export Verification Package + Disclosure Manifest with offline verification as first-class.

## 5.1 Immediate repo normalization sequence (recommended)

1. Create `specs/` and `specs/companions/` scaffold with final filenames.
2. Move trimmed constitutional text from `DRAFTS/unified_ledger_core.md` into `specs/trellis-core.md`.
3. Split `DRAFTS/unified_ledger_companion.md` into named companion drafts.
4. Move research/reviews/ADRs into a clearly marked `rationale/` tree.
5. Add profile sidecar templates (`profiles/`) for trust-profile and metadata-budget declarations.
6. Add an assurance traceability artifact mapping each invariant to verification methods.

## 6) Anything structurally wrong in the current draft

- Core still carries too much profile and companion content.
- Companion is overloaded as omnibus; this raises semantic debt and review friction.
- Export and disclosure are currently under-separated for independent maturity.
- Assurance methodology is present conceptually but not yet anchored as a durable spec-family artifact with traceability from requirements to checks.
- Repo boundaries are implied but not explicit enough to prevent future overlap with Formspec and WOS.

## 7) Technical conclusions still not captured strongly enough

1. **Append idempotency contract** should be explicit in core normative language, not only research/review narrative.
2. **Snapshot-from-day-one requirement** should be explicit in Projection/Rebuild companion.
3. **Assurance traceability matrix** should map each hard invariant to model checking, fuzzing, property tests, vectors, and recovery drills.
4. **Metadata budget as a mandatory profile declaration table** should be formalized, not prose-only.
5. **Phase-1 implementation profile** should be written explicitly as non-normative but recommended baseline:
   Postgres-first canonical plane, direct encrypted blobs, derived workflow/authz, offline verifier, transparency-log semantics without CT personality lock-in.

## 8) What the current canvas-style restructuring got right vs what should still change

### Got right

- Correct core/companion family direction.
- Correct emphasis on transparency-log semantics.
- Correct preservation of canonical vs derived rigidity.
- Correct inclusion of trust profiles and key lifecycle as first-class.
- Correct treatment of export/disclosure and monitoring seams.

### Should still change

- Reduce core further.
- Split omnibus companion into named, independently reviewable companions.
- Split Export vs Disclosure.
- Add explicit assurance artifact and metadata-budget tables.
- Publish hard repo-boundary clauses to minimize future semantic duplication.
