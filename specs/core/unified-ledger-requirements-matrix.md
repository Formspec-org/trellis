# Unified Ledger Core — Feature / Requirements Matrix

This document extracts normative requirements from the legacy draft [`../../DRAFTS/unified_ledger_core.md`](../../DRAFTS/unified_ledger_core.md) into a traceability matrix with stable IDs. Use it for migration into [`trellis-core.md`](trellis-core.md), assurance mapping, and implementation checklists.

**Synthesis rows** **ULCR-095–103** record integrator-critical guarantees from normalized [`trellis-core.md`](trellis-core.md), [`shared-ledger-binding.md`](shared-ledger-binding.md), [`../trust/trust-profiles.md`](../trust/trust-profiles.md), [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md), and [`../../DRAFTS/trellis_spec_family_normalization_plan.md`](../../DRAFTS/trellis_spec_family_normalization_plan.md) where the legacy omnibus was silent or less explicit.

**IDs:** `ULCF-###` = feature area; `ULCR-###` = requirement (through **ULCR-103**).

**Requirement class** (from the source draft): **CS** = constitutional semantic, **PC** = profile constraint, **BR** = binding or reference choice.

---

## Legend

| Column | Meaning |
|--------|---------|
| **ULCF** | Feature / capability area |
| **ULCR** | Normative requirement (one obligation or prohibition where practical) |
| **§** | Section in `unified_ledger_core.md` or other cited spec |

Keywords follow BCP 14 as used in the source draft.

---

## Feature index (ULCF)

| ID | Name | § (primary) |
|----|------|-------------|
| ULCF-001 | Conformance & profile subordination | 2.3 |
| ULCF-002 | Core Profile & conformance roles | 2.4–2.5 |
| ULCF-003 | Core–implementation contracts | 3 |
| ULCF-004 | Controlled vocabulary (normative terminology) | 4.10 |
| ULCF-005 | Core ontology discipline | 5.2 |
| ULCF-006 | Canonical truth scope | 6.1 |
| ULCF-007 | Fundamental invariants | 7.1 |
| ULCF-008 | Fact admission, objects, admissibility | 8.1–8.2 |
| ULCF-009 | Admission state machine | 8.3 |
| ULCF-010 | Canonical order | 9.1 |
| ULCF-011 | Canonical append attestation | 9.2 |
| ULCF-012 | Serialization/proof binding boundary | 9.3 |
| ULCF-013 | Trust Profile minimum semantics | 10.1 |
| ULCF-014 | Disclosure posture vs assurance | 10.2 |
| ULCF-015 | Trust honesty | 11.1 |
| ULCF-016 | Trust Profile transitions | 11.2 |
| ULCF-017 | Export packages | 12.1–12.2 |
| ULCF-018 | Offline verification capabilities | 12.3 |
| ULCF-019 | Export provenance & verification independence | 12.4–12.5 |
| ULCF-020 | Generic profile discipline | 13.1 |
| ULCF-021 | Profile trust inheritance & export honesty | 13.2–13.3 |
| ULCF-022 | Offline Authoring Profile | 14.1 |
| ULCF-023 | Reader-Held Decryption Profile | 14.2 |
| ULCF-024 | Delegated Compute Profile | 14.3 |
| ULCF-025 | Disclosure and Export Profile | 14.4 |
| ULCF-026 | User-Held Record Reuse Profile | 14.5 |
| ULCF-027 | Respondent History Profile | 14.6 |
| ULCF-028 | Bindings, vocabulary placement, sidecars | 15 |
| ULCF-029 | Derived artifacts & evaluators | 16.1 |
| ULCF-030 | Metadata minimization | 16.2 |
| ULCF-031 | Idempotency & rejection | 16.3 |
| ULCF-032 | Durable storage & snapshots | 16.4 |
| ULCF-033 | Lifecycle & cryptographic inaccessibility | 16.5 |
| ULCF-034 | Versioning & algorithm agility | 16.6 |
| ULCF-035 | Trust & privacy disclosure obligations | 17.3 |
| ULCF-036 | Integrator-critical ledger guarantees (normalized core + family boundaries) | [`trellis-core.md`](trellis-core.md) §5–9; [`shared-ledger-binding.md`](shared-ledger-binding.md); plan §4 |
| ULCF-037 | Canonical receipt immutability (binding) | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Canonical receipt immutability |

---

## Requirements matrix (ULCR)

| ULCR | ULCF | Feature name | Requirement summary | Keyword | Class | § |
|------|------|----------------|----------------------|---------|-------|---|
| ULCR-001 | ULCF-001 | Conformance & profile subordination | Profiles/bindings MUST stay subordinate to core canonical truth, append, trust-profile, export-verification, and core–implementation contracts. | MUST | PC | 2.3 |
| ULCR-002 | ULCF-001 | Conformance & profile subordination | Profiles/bindings MUST narrow or specialize the core, not reinterpret it. | MUST | PC | 2.3 |
| ULCR-003 | ULCF-001 | Conformance & profile subordination | Profiles/bindings MUST NOT define a second canonical order. | MUST NOT | PC | 2.3 |
| ULCR-004 | ULCF-001 | Conformance & profile subordination | Profiles/bindings MUST NOT redefine canonical truth established by the Core Profile. | MUST NOT | PC | 2.3 |
| ULCR-005 | ULCF-001 | Conformance & profile subordination | On conflict with core, the core specification governs. | MUST (governance) | PC | 2.3 |
| ULCR-006 | ULCF-002 | Core Profile & roles | Core Profile implementation MUST produce or accept author-originated facts, canonical facts, canonical records, and canonical append attestations as applicable to its role. | MUST | CS | 2.4 |
| ULCR-007 | ULCF-002 | Core Profile & roles | Core Profile MUST preserve append-only semantics for canonical records. | MUST | CS | 2.4 |
| ULCR-008 | ULCF-002 | Core Profile & roles | Core Profile MUST distinguish canonical truth from derived artifacts. | MUST | CS | 2.4 |
| ULCR-009 | ULCF-002 | Core Profile & roles | Core Profile MUST support independently verifiable export for at least one declared export scope. | MUST | CS | 2.4 |
| ULCR-010 | ULCF-002 | Fact Producer | Fact Producer MUST produce author-originated or other attributable facts admitted per spec under active profile/binding. | MUST | CS | 2.5.1 |
| ULCR-011 | ULCF-002 | Fact Producer | Fact Producer MUST sign/authenticate facts where profile/binding requires. | MUST | CS | 2.5.1 |
| ULCR-012 | ULCF-002 | Fact Producer | Fact Producer MUST preserve causal references when applicable. | MUST | CS | 2.5.1 |
| ULCR-013 | ULCF-002 | Fact Producer | Fact Producer MUST NOT mutate previously produced facts. | MUST NOT | CS | 2.5.1 |
| ULCR-014 | ULCF-002 | Canonical Append Service | CAS MUST validate admissible facts under active profile/binding. | MUST | CS | 2.5.2 |
| ULCR-015 | ULCF-002 | Canonical Append Service | CAS MUST form canonical records for admitted facts. | MUST | CS | 2.5.2 |
| ULCR-016 | ULCF-002 | Canonical Append Service | CAS MUST append canonical records to canonical order. | MUST | CS | 2.5.2 |
| ULCR-017 | ULCF-002 | Canonical Append Service | CAS MUST issue canonical append attestations. | MUST | CS | 2.5.2 |
| ULCR-018 | ULCF-002 | Canonical Append Service | CAS MUST NOT rewrite prior canonical records. | MUST NOT | CS | 2.5.2 |
| ULCR-019 | ULCF-002 | Canonical Append Service | CAS MUST NOT treat workflow state, projections, or caches as canonical truth. | MUST NOT | CS | 2.5.2 |
| ULCR-020 | ULCF-002 | Verifier | Verifier MUST verify authored authentication where required. | MUST | CS | 2.5.3 |
| ULCR-021 | ULCF-002 | Verifier | Verifier MUST verify canonical append attestations and inclusion proofs. | MUST | CS | 2.5.3 |
| ULCR-022 | ULCF-002 | Verifier | Verifier MUST distinguish author facts, canonical records, attestations, and disclosure/export artifacts. | MUST | CS | 2.5.3 |
| ULCR-023 | ULCF-002 | Verifier | Verifier MUST NOT require derived artifacts to verify canonical integrity. | MUST NOT | CS | 2.5.3 |
| ULCR-024 | ULCF-002 | Derived Processor | Derived Processor MUST consume canonical truth as only authoritative input. | MUST | CS | 2.5.4 |
| ULCR-025 | ULCF-002 | Derived Processor | Derived Processor MUST record sufficient provenance to support rebuild. | MUST | CS | 2.5.4 |
| ULCR-026 | ULCF-002 | Derived Processor | Derived Processor MUST be discardable/rebuildable without changing canonical truth. | MUST | CS | 2.5.4 |
| ULCR-027 | ULCF-002 | Export Generator | Export Generator MUST package records, attestations, and verification material per declared export scope. | MUST | CS | 2.5.5 |
| ULCR-028 | ULCF-002 | Export Generator | Export Generator MUST preserve provenance distinctions. | MUST | CS | 2.5.5 |
| ULCR-029 | ULCF-002 | Export Generator | Export Generator MUST include enough material for offline verifier to validate export scope. | MUST | CS | 2.5.5 |
| ULCR-030 | ULCF-003 | Core–implementation contracts | Canonical Append Contract: implementation MAY vary mechanisms but MUST preserve admission, order, record formation, attestation semantics. | MUST | CS | 3 |
| ULCR-031 | ULCF-003 | Core–implementation contracts | Derived Artifact Contract: derived artifacts MUST be rebuildable from canonical truth and MUST NOT be authoritative for canonical facts. | MUST / MUST NOT | CS | 3 |
| ULCR-032 | ULCF-003 | Core–implementation contracts | Workflow Contract: workflow state MUST remain operational unless later represented as canonical facts under profile/binding. | MUST | CS | 3 |
| ULCR-033 | ULCF-003 | Core–implementation contracts | Authorization Contract: grant/revocation MUST remain canonical; evaluator state MUST remain derived. | MUST | CS | 3 |
| ULCR-034 | ULCF-003 | Core–implementation contracts | Trust Contract: implementation MAY vary custody/KM/delegation but active Trust Profile MUST describe who can read/recover/delegate/attest/administer. | MUST | CS | 3 |
| ULCR-035 | ULCF-003 | Core–implementation contracts | Export Contract: implementation MAY vary packaging but exports MUST preserve required provenance distinctions and verification claims. | MUST | CS | 3 |
| ULCR-036 | ULCF-003 | Core–implementation contracts | Bindings/implementations MUST preserve all these contracts when mechanisms change. | MUST | CS | 3 |
| ULCR-037 | ULCF-004 | Controlled vocabulary | Normative sections MUST avoid casual alternatives when a preferred term exists. | MUST | CS | 4.10 |
| ULCR-038 | ULCF-005 | Core ontology | Normative sections MUST preserve distinctions among primary object classes. | MUST | CS | 5.2 |
| ULCR-039 | ULCF-005 | Core ontology | Normative sections MUST NOT collapse derived or disclosure/export artifacts into canonical truth. | MUST NOT | CS | 5.2 |
| ULCR-040 | ULCF-006 | Canonical truth scope | Implementation MUST NOT treat excluded artifacts (derived, workflow state, auth evaluator state, indexes, caches, unrecorded delegated outputs) as authoritative for canonical facts. | MUST NOT | CS | 6.1 |
| ULCR-041 | ULCF-007 | Named invariants | Invariant 1: author-originated fact and canonical append attestation MUST remain distinguishable. | MUST | CS | 7.1 |
| ULCR-042 | ULCF-007 | Named invariants | Invariant 2: canonical fact and canonical record MUST remain distinguishable. | MUST | CS | 7.1 |
| ULCR-043 | ULCF-007 | Named invariants | Invariant 3: derived artifacts MUST remain non-canonical. | MUST | CS | 7.1 |
| ULCR-044 | ULCF-007 | Named invariants | Invariant 4: provider-readable and reader-held access MUST remain distinct. | MUST | CS | 7.1 |
| ULCR-045 | ULCF-007 | Named invariants | Invariant 5: delegated compute MUST NOT be treated as blanket provider plaintext access. | MUST NOT | CS | 7.1 |
| ULCR-046 | ULCF-007 | Named invariants | Invariant 6: disclosure posture and assurance MUST remain distinct and MUST NOT be conflated. | MUST / MUST NOT | CS | 7.1 |
| ULCR-047 | ULCF-008 | Fact admission & objects | Implementation MUST keep distinguishable: author fact, canonical fact, canonical record, attestation, disclosure/export artifact. | MUST | CS | 8.1 |
| ULCR-048 | ULCF-008 | Fact admission & objects | Canonical record MUST remain distinguishable from underlying canonical fact. | MUST | CS | 8.1 |
| ULCR-049 | ULCF-008 | Fact admission & objects | Disclosure/export artifact MUST NOT be treated as identical to underlying canonical record. | MUST NOT | CS | 8.1 |
| ULCR-050 | ULCF-008 | Admissibility | Profiles MAY narrow admissibility (subset, predicates, actors); MUST NOT reinterpret categories to change truth or create alternate order. | MAY / MUST NOT | CS | 8.2 |
| ULCR-051 | ULCF-009 | Admission state machine | Fact becomes canonical only when canonical record crosses declared durable-append boundary. | MUST (semantic) | CS | 8.3 |
| ULCR-052 | ULCF-009 | Admission state machine | Attestation proves inclusion/order under append model; does not prove substantive correctness beyond admission scope (declared limitation). | (framing) | CS | 8.3 |
| ULCR-053 | ULCF-010 | Canonical order | Records past durable-append boundary MUST be bound into canonical ordered append structure. | MUST | CS | 9.1 |
| ULCR-054 | ULCF-010 | Canonical order | Canonical order MUST have declared scope; claims apply only within scope. | MUST | CS | 9.1 |
| ULCR-055 | ULCF-010 | Canonical order | Append-attestation stream (or equivalent) MUST be single ordered source of truth for inclusion/sequence. | MUST | CS | 9.1 |
| ULCR-056 | ULCF-010 | Canonical order | No workflow/projection/auth/collaboration layer MUST NOT define alternate canonical order. | MUST NOT | CS | 9.1 |
| ULCR-057 | ULCF-011 | Append attestation | CAS MUST return attestation for records past durable-append boundary. | MUST | CS | 9.2 |
| ULCR-058 | ULCF-011 | Append attestation | CAS MUST NOT issue attestation before durable-append boundary crossed. | MUST NOT | CS | 9.2 |
| ULCR-059 | ULCF-011 | Append attestation | Attestation MUST include or reference: position/index, inclusion proof material, append-head reference, sufficient verifier metadata. | MUST | CS | 9.2 |
| ULCR-060 | ULCF-012 | Serialization binding | If binding declares encodings/proofs/APIs, conforming implementations for that binding MUST follow it. | MUST | BR | 9.3 |
| ULCR-061 | ULCF-013 | Trust Profile object | Trust Profile MUST semantically include minimum fields listed in source (identifier, scope, postures, authorities, metadata visibility, etc.). | MUST | CS | 10.1 |
| ULCR-062 | ULCF-013 | Trust Profile object | Bindings MAY define wire shape but MUST preserve minimum semantic fields and meanings. | MAY / MUST | CS | 10.1 |
| ULCR-063 | ULCF-014 | Disclosure vs assurance | Implementation MUST distinguish assurance from disclosure posture; MUST NOT treat higher assurance as greater identity disclosure by default; MAY support subject continuity; MUST preserve distinctions across profiles/exports/disclosures. | MUST / MUST NOT / MAY / MUST | CS | 10.2 |
| ULCR-064 | ULCF-015 | Trust honesty | For each deployment mode handling protected content: MUST publish Trust Profile; MUST state provider-readable vs reader-held vs reader-held+delegated compute; MUST state runtime plaintext access in ordinary processing; MUST state recovery without user; MUST state whether delegated compute exposes plaintext to ordinary service components; MUST NOT collapse delegated compute into provider-readable unless declared; MUST NOT overstate trust posture. | MUST / MUST NOT | CS | 11.1 |
| ULCR-065 | ULCF-016 | Trust transitions | On custody/readability/recovery/delegated-compute change for protected content: MUST treat as Trust Profile transition; MUST make auditable; MUST define prospective/retrospective/both; MUST NOT expand reader-held or delegated compute into provider-readable without explicit transition. | MUST / MUST NOT | CS | 11.2 |
| ULCR-066 | ULCF-017 | Export | Conforming implementations MUST support independently verifiable exports for ≥1 declared scope. | MUST | CS | 12.1 |
| ULCR-067 | ULCF-017 | Export package contents | Export MUST include sufficient material listed in source (records/representations, attestations/proofs, keys/refs, append proofs, schema digests, payloads/refs, canonical facts for claims). | MUST | CS | 12.2 |
| ULCR-068 | ULCF-017 | Export package contents | References required for offline verification MUST be immutable, content-addressed, or included. | MUST | CS | 12.2 |
| ULCR-069 | ULCF-018 | Verification | Verifier MUST be able to perform verification steps 1–5 in source (auth, inclusion, append-head, digests/refs, disclosure artifacts). | MUST | CS | 12.3 |
| ULCR-070 | ULCF-019 | Export provenance | Exports MUST preserve distinction among author facts, canonical records, attestations, later disclosure/export artifacts. | MUST | CS | 12.4 |
| ULCR-071 | ULCF-019 | Verification independence | Export verification MUST NOT depend on derived artifacts, workflow runtime, mutable DBs, or ordinary APIs beyond optional external proof material. | MUST NOT | CS | 12.5 |
| ULCR-072 | ULCF-019 | Verification independence | If export omits payload readability, MUST still disclose which integrity/provenance/append claims remain verifiable. | MUST | CS | 12.5 |
| ULCR-073 | ULCF-020 | Profile discipline | Profile layers MUST remain profiles; MUST NOT alter core truth/admission/order/attestation/trust honesty/export semantics; MUST NOT define alternate canonical source of truth. | MUST / MUST NOT | PC | 13.1 |
| ULCR-074 | ULCF-021 | Profile trust & export | Profiles/bindings inherit active Trust Profile; MUST distinguish provider-readable, reader-held, delegated compute when protected content involved; MUST NOT imply stronger confidentiality than Trust Profile; MUST NOT weaken Trust Profile via profile-local wording. | MUST / MUST NOT | PC | 13.2 |
| ULCR-075 | ULCF-021 | Profile-scoped export | Profile-scoped export MAY present profile-specific views; MUST preserve object-class distinctions; MUST NOT imply broader coverage than declared scope. | MAY / MUST / MUST NOT | PC | 13.3 |
| ULCR-076 | ULCF-022 | Offline Authoring Profile | MAY require delayed submission, authored time/context, local pending, auth before admission; MUST preserve §8 state machine and provenance distinctions. | MAY / MUST | PC | 14.1 |
| ULCR-077 | ULCF-023 | Reader-Held Decryption Profile | MAY require no general plaintext for protected content in ordinary operation; MUST identify decrypting principals; MUST be consistent with Trust Profile; MUST preserve reader-held vs provider-readable distinction. | MAY / MUST | PC | 14.2 |
| ULCR-078 | ULCF-024 | Delegated Compute Profile | MAY define scoped delegated compute requirements; MUST NOT imply general provider readability; if workflow relies materially on delegated output MUST require canonical fact or canonical reference to stable artifact. | MAY / MUST NOT / MUST | PC | 14.3 |
| ULCR-079 | ULCF-025 | Disclosure and Export Profile | MAY define scopes, postures, claim classes, presentation rules; MUST remain subordinate to §12. | MAY / MUST | PC | 14.4 |
| ULCR-080 | ULCF-026 | User-Held Record Reuse Profile | MAY define reference/submission of user-held material; MUST distinguish reusable records from canonical facts; MUST bind what was reused; MUST NOT treat entire user-held layer as canonical workflow state by default. | MAY / MUST / MUST NOT | PC | 14.5 |
| ULCR-081 | ULCF-027 | Respondent History Profile | MAY define respondent-visible history; MUST treat timelines as derived over canonical truth; MUST NOT define second append model; MUST NOT imply full coverage unless in scope. | MAY / MUST / MUST NOT | PC | 14.6 |
| ULCR-082 | ULCF-028 | Bindings | Bindings MAY define concrete serializations/APIs/proofs/mappings; MUST preserve constitutional semantics and contracts. | MAY / MUST | BR | 15.1 |
| ULCR-083 | ULCF-028 | Vocabulary placement | Domain vocabularies SHOULD be in profiles not constitutional core. | SHOULD | PC | 15.2 |
| ULCR-084 | ULCF-028 | Family bindings | Binding MAY map core to family; MAY define path/item-key/validation/amendment semantics; remain binding/profile unless adopted higher. | MAY | BR | 15.3 |
| ULCR-085 | ULCF-028 | Sidecars | Sidecar MAY collect family/deployment material; MUST NOT alter constitutional semantics. | MAY / MUST NOT | BR | 15.4 |
| ULCR-086 | ULCF-029 | Derived artifacts | Derived artifact MUST NOT be authoritative for canonical facts; MUST be rebuildable from canonical truth + declared config history; MUST record enough provenance for source canonical state; MUST treat lag/rebuild/loss as operational not truth change. | MUST | CS | 16.1 |
| ULCR-087 | ULCF-029 | Rights-impacting evaluators | If derived evaluator used for rights-impacting decisions: MUST trace inputs to canonical facts; MUST define rebuild behavior; MUST define stale/missing/inconsistent behavior. | MUST | CS | 16.1 |
| ULCR-088 | ULCF-030 | Metadata minimization | Visible metadata SHOULD be limited to stated purposes; SHOULD NOT keep metadata merely to accelerate derived artifacts; MUST NOT retain visible append metadata merely for convenience when derived/scoped mechanisms suffice. | SHOULD / MUST NOT | CS | 16.2 |
| ULCR-089 | ULCF-031 | Idempotency | Canonical append MUST define idempotency semantics for retries; MUST define stable idempotency key or equivalent causal identity; MUST define retry outcome (reject/no-op/existing ref); rejected MUST NOT be treated as appended; successful retries MUST resolve consistently per identity/scope. | MUST / MUST NOT | CS | 16.3 |
| ULCR-090 | ULCF-032 | Storage & snapshots | Canonical records MUST be stored durably/immutably from ordinary participants’ perspective; implementation MUST declare durable-append boundary; snapshots MAY be used but MUST be derived not canonical; replica completion MUST remain operational not canonical. | MUST / MAY | CS | 16.4 |
| ULCR-091 | ULCF-033 | Lifecycle | If operation is canonical/compliance-relevant among listed lifecycle types, MUST represent as lifecycle fact; if affects compliance/retention/recoverability claims MUST be canonical fact. | MUST | CS | 16.5 |
| ULCR-092 | ULCF-033 | Crypto destruction | If key destruction/erasure used, MUST document irrecoverable content, who retains access, destruction evidence, remaining metadata; affected derived plaintext MUST be invalidated/purged/unusable per declared policy. | MUST | CS | 16.5 |
| ULCR-093 | ULCF-034 | Versioning | MUST version algorithms and schema/semantic references; MUST version listed semantic areas where profile/binding-specific; MUST preserve enough info to verify historical records under rules in effect; MUST NOT silently reinterpret historical records without migration; MUST NOT silently invalidate prior export verification via evolution; MUST NOT rely on out-of-band operator knowledge for historical interpretation. | MUST / MUST NOT | CS | 16.6 |
| ULCR-094 | ULCF-035 | Trust/privacy disclosure | Handling protected content: MUST disclose visible metadata and which parties observe it; MUST disclose provider-readable ordinary operation; MUST disclose delegated-compute plaintext exposure to ordinary components; MUST NOT equate ciphertext storage with provider blindness when decryption paths exist. | MUST / MUST NOT | CS | 17.3 |
| ULCR-095 | ULCF-036 | Governed scope & order | Exactly one canonical append-attested order per governed scope; implementations MAY partition into multiple ledgers by scope but MUST NOT allow competing canonical orders for the same governed scope. | MUST / MAY / MUST NOT | CS | [`trellis-core.md`](trellis-core.md) §5.2 (inv. 3), §6.2 |
| ULCR-096 | ULCF-036 | Canonical event hash | Canonical append semantics MUST use exactly one authoritative canonical event hash construction over the sealed canonical record package; deterministic canonical serialization is REQUIRED; subordinate hashes MAY exist for specialized purposes but MUST NOT redefine canonical append semantics. | MUST / REQUIRED / MAY / MUST NOT | CS | [`trellis-core.md`](trellis-core.md) §7 |
| ULCR-097 | ULCF-036 | Verifier obligations | A conforming verifier MUST be able to validate canonical record integrity, append attestation validity, inclusion and consistency claims, and export-package canonical provenance claims, without requiring derived runtime state. | MUST | CS | [`trellis-core.md`](trellis-core.md) §8 |
| ULCR-098 | ULCF-036 | Cross-repository authority | Trellis core semantics MUST NOT be interpreted to redefine Formspec or WOS semantic authority. | MUST NOT | CS | [`trellis-core.md`](trellis-core.md) §9 |
| ULCR-099 | ULCF-036 | Substrate binding | Trellis MUST bind Formspec-family and WOS-family facts (and related trust/release families per binding spec) into one governed canonical substrate with shared append, hash, and verification rules; binding MUST NOT reinterpret Formspec or WOS meaning. | MUST / MUST NOT | CS | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Substrate binding, §Deferral Rules, §Canonization rules |
| ULCR-100 | ULCF-036 | Baseline scope (advanced crypto) | Baseline Trellis Core conformance MUST NOT be interpreted to require advanced selective disclosure, threshold custody, group-sharing protocols, advanced homomorphic or privacy-preserving computation, or cross-agency analytic protocols unless a declared profile, binding, or implementation specification explicitly requires them. | MUST NOT | CS | [`trellis-core.md`](trellis-core.md) §10.1; [`../../DRAFTS/trellis_spec_family_normalization_plan.md`](../../DRAFTS/trellis_spec_family_normalization_plan.md) §4; aligns [`unified-ledger-companion-requirements-matrix.md`](unified-ledger-companion-requirements-matrix.md) ULCOMP-R-213–214 |
| ULCR-101 | ULCF-036 | Admission prerequisites | Canonical Append Service MUST NOT issue canonical append attestation until binding-declared admission prerequisites are satisfied, including resolution of causal or logical dependencies required for that record class. | MUST NOT | CS | [`trellis-core.md`](trellis-core.md) §6.1 |
| ULCR-102 | ULCF-036 | Order independence from operations | Canonical order MUST be determined solely by this specification and applicable binding; MUST NOT depend on wall-clock receipt time, queue depth, worker identity, or other operational accidents. | MUST / MUST NOT | CS | [`trellis-core.md`](trellis-core.md) §6.2 |
| ULCR-103 | ULCF-037 | Receipt immutability | Binding-defined ingest-time verification or payload-readiness fields on the canonical append attestation (or equivalent receipt) MUST NOT be rewritten in place after issuance; posture changes MUST be new canonical facts or attestations per binding. | MUST NOT | BR | [`shared-ledger-binding.md`](shared-ledger-binding.md) §Canonical receipt immutability |

---

## User-value themes (traceability to ULCR)

High-impact guarantees for builders, operators, and auditors; most were already present in legacy rows—**ULCR-095–103** add normalized-core, binding, and admission explicitness.

| User-value theme | Primary ULCR IDs |
|------------------|------------------|
| One append-attested order per governed scope (incl. partitioning) | ULCR-054, ULCR-055, ULCR-056, **ULCR-095** |
| One canonical event hash + deterministic serialization | **ULCR-096** |
| Independent verification (inclusion, consistency, export provenance) | ULCR-069, ULCR-071, ULCR-072, **ULCR-097** |
| Append idempotency and explicit rejection | ULCR-089 |
| Canonical vs derived; evaluators do not override grants | ULCR-031, ULCR-040, ULCR-086, ULCR-087 |
| Honest trust posture (who reads what, metadata, delegation) | ULCR-064, ULCR-074, ULCR-094 |
| Destruction / shred + derived plaintext handling | ULCR-092 |
| Verifiable export / disclosure packages | ULCR-066, ULCR-067, ULCR-068, ULCR-070 |
| Cross-family authority (Trellis does not redefine Formspec/WOS) | **ULCR-098** |
| Single substrate (Formspec + WOS families bound to one ledger) | **ULCR-099** |
| Baseline does not mandate late-phase privacy/crypto mechanisms | **ULCR-100** |
| No attestation while causal/binding prerequisites unresolved | **ULCR-101** |
| Order not driven by receipt time, queues, or workers | **ULCR-102** |
| Ingest-time receipt fields not silently rewritten | **ULCR-103** |

**Invariant → verification methods:** see [`../assurance/assurance-traceability.md`](../assurance/assurance-traceability.md).

Staff-facing **projection watermarks**, **stale indication**, **purge cascades**, and **rebuild/conformance expectations** are tracked in [`unified-ledger-companion-requirements-matrix.md`](unified-ledger-companion-requirements-matrix.md) (**ULCOMP-R-215–220**) and [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md).

**Metadata budget** (per fact family) is tracked in the companion matrix (**ULCOMP-R-221**) and [`../trust/trust-profiles.md`](../trust/trust-profiles.md).

---

## Coverage notes

1. **§2.1–2.2** define conformance classes and profiles by name; they are structural and inform ULCF-002 and profile features rather than separate MUST rows.
2. **§8.2** enumerates admissible categories; normative constraints are ULCR-050 plus admission semantics elsewhere.
3. **§8.3** state machine table is definitional; ULCR-051–052 capture the durable-append rule and attestation scope.
4. **§17.1–17.2** are non-normative considerations; only **§17.3** yields ULCR-094.
5. **§18** is explicitly non-normative—no ULCR rows.
6. **ULCR-095–098** are anchored in [`trellis-core.md`](trellis-core.md); update them if that document’s §§5–9 change.
7. **ULCR-099** tracks [`shared-ledger-binding.md`](shared-ledger-binding.md) §Substrate binding; **ULCR-100** tracks [`trellis-core.md`](trellis-core.md) §10.1 and the normalization plan and aligns companion **ULCOMP-R-213–214** (legacy App. G).
8. **ULCR-101–102** track [`trellis-core.md`](trellis-core.md) §6.1–6.2; **ULCR-103** tracks [`shared-ledger-binding.md`](shared-ledger-binding.md) §Canonical receipt immutability.
9. Tiered **verification posture** for workflows and **projection integrity policy** are in the companion matrix (**ULCOMP-R-222–223**) with [`../trust/trust-profiles.md`](../trust/trust-profiles.md) and [`../projection/projection-runtime-discipline.md`](../projection/projection-runtime-discipline.md).

When `unified_ledger_core.md` text moves into `trellis-core.md` and companion drafts, keep **ULCR** IDs stable in migrated sections or add a “Migrated as ULCR-NNN” line for traceability.
