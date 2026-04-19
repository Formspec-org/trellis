# Trellis O-5 Posture-Transition Event Schemas — Design

**Date:** 2026-04-18
**Scope:** Normative CDDL for the two canonical posture-transition events (custody-model, disclosure-profile); their event-type strings; verifier semantics; binding to the Posture Declaration (Companion §11); O-5 fixture plan.
**Closes:** O-5 (design); supplies the schemas Companion §10 has until now only described in prose and Companion Appendix A.5 has only sketched.
**Unblocks:** Companion §27.7 transition-auditability conformance tests; G-3 `append/` and `tamper/` coverage for posture transitions; Rust reference impl (G-4) of the transition path.
**Does not cover:** authoring the ~50 Core/Companion vectors (G-3 fixture-system design `thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md` owns the corpus plan), declaration-document template shape (O-4 stream owns Companion Appendix A.1–A.4), Phase 4 witness attestation of transitions (Companion §26 seam, deferred).

## Context

Ratification bar per `ratification/ratification-checklist.md` O-5 and Companion §10 (Posture-Transition Auditability): custody-model and disclosure-profile changes are canonical ledger events, not operator-side configuration changes. An implementation that silently edits its posture declaration without recording a transition event is NON-CONFORMANT per OC-08.

Companion §10.3 names six required fields generically (`transition_id`, `prior_posture_ref`, `new_posture_ref`, `transition_actor`, `policy_authority`, `effective_time`, `temporal_scope`, `attestations`). Appendix A.5 gives a structural sketch but no CDDL, no event-type string, no rule for how the event slots into Core §6 `EventPayload`, and no verifier output fields. An implementor reading Core + Companion + Appendix A.5 today cannot write byte-exact bytes. This brief closes that gap.

This design resolves six decisions needed before any transition fixture can be authored: (1) the two event-type strings, (2) the CDDL for each payload, (3) how the payload rides inside `EventPayload.extensions` without violating Core §6.7 strict-superset semantics, (4) the verifier's sequencing rule (from_* must match most-recent prior transition's to_*), (5) the rule binding transitions to declaration documents, (6) where the fixtures live under the G-3 layout.

## Decision 1 — Transition-vs-declaration rule

Three options for how transition events relate to the Posture Declaration (Companion §11 / Appendix A):

| Option | Rule | Operator burden | Audit clarity |
|---|---|---|---|
| **A. Co-publish required** | Every transition MUST cite a freshly published declaration document; `declaration_doc_digest` of the new declaration is mandatory. | High — every transition triggers a declaration-document revision even when the declaration text does not change (e.g., recovery authority added without touching the human-readable prose). | Highest — every transition has a self-contained `(from, to, declaration)` triple. Auditor never reads stale declarations. |
| **B. Reuse permitted** | A transition MAY reference an existing declaration document if that declaration is still accurate after the transition. `declaration_doc_digest` is still mandatory but MAY equal the prior transition's. | Low — operator revises the declaration only when it actually becomes wrong. | Medium — auditor must replay the chain to know which declaration was in force at event N. |
| **C. Snapshot-only** | Declaration is a separate cadence (e.g., quarterly); transitions cite whichever declaration was most recently published. | Lowest — no declaration revisions tied to transitions. | Lowest — by design, the declaration can be silently stale at a transition boundary; contradicts Companion §11.3 (declaration is part of verified export). |

**Picked: Option A — co-publish required.** Rationale:

- Companion §11.3 (OC-15) says every export bundle MUST carry a declaration; a verifier processing a chain with a transition needs the declaration in force *after* the transition to gate downstream checks (§19.4, §28.10). Option B forces verifiers to walk backwards to resolve the applicable declaration; Option C breaks the contract outright.
- OC-09 says "corrections to past declarations MUST be represented as forward transitions." That rule is ambient-stated for transitions that *correct* declarations. Option A makes it uniform: every transition is a forward-declaration-producing act. Uniform is cheaper to verify than conditional.
- Operator burden is in practice low. The declaration document is structured (Appendix A.1–A.4) and content-addressed; re-publishing is a deterministic serialization of the new in-force state. Operators that claim the new state equals the prior state trivially produce a declaration identical to the prior one byte-for-byte — but a transition with `declaration_doc_digest == prior_declaration_doc_digest` is prima-facie evidence that the operator believes the declaration is unchanged, which the auditor can cross-check against what the transition actually changed. That is a stronger audit signal than "the operator said nothing about the declaration."
- Greenfield discipline (CLAUDE.md "no 'defer' on greenfield"): Option B's "may reuse" rule is exactly the kind of soft option that accumulates ambiguity. Option A is stricter and cheaper to enforce now.

**Normative rule (proposed):** A Posture Transition event's `declaration_doc_digest` MUST reference the declaration document in force *after* the transition's `effective_time`. The referenced declaration MUST be published (content-addressable) at or before the transition event is admitted. A digest that resolves to an unpublished document is a fatal verification failure.

## Decision 2 — Event-type strings

Per Core §6.7 extension-registration discipline, event-type identifiers begin with `trellis.` and are append-only, suffixed with a version slot mirroring Core §9.8's domain-tag convention (`-v1`). Per Core §12.4, event-type identifiers MUST be outcome-neutral; "custody-model-transition" says *what* not *which way*, which is compliant.

The registered identifiers for O-5:

- **`trellis.custody-model-transition.v1`** — change in Custody Model (Companion §9) or in any §9.3 required field of the active Custody Model (decryptors, recovery authorities, delegated-compute posture, attestation-control authorities).
- **`trellis.disclosure-profile-transition.v1`** — change in the Respondent Ledger Profile A/B/C claim (posture axes: privacy × identity × integrity-anchoring) for a declared scope, as it applies to Trellis-wrapped response ledgers.

Naming rationale: the `.v1` suffix parallels Core §9.8's `trellis-*-v1` domain tag pattern and matches the phase-superset commitment (§6.5) that Phase 2+ registrations use `.v2` without rewriting v1. Both identifiers are reserved in the Phase 1 registration table (Core §6.7) under Phase 1 — they are Phase 1 Companion obligations, not Phase 2 seams.

Invariant #11 compliance: `custody-model-transition` names the §9 namespace; `disclosure-profile-transition` names the Respondent Ledger Profile A/B/C namespace. Neither crosses into the other; neither reuses the word "profile" bare.

## Decision 3 — CDDL: custody-model transition

The transition event is an ordinary Core §6 event. Its envelope is unchanged: `Event = COSESign1Bytes` over `dCBOR(EventPayload)`. The transition-specific payload rides in a reserved `EventPayload.extensions` entry keyed by the registered identifier. This preserves Core §6.5 strict-superset semantics — the transition is additive, not a top-level field growth.

```cddl
; Registered under EventHeader.event_type = "trellis.custody-model-transition.v1"
; Payload rides in EventPayload.extensions["trellis.custody-model-transition.v1"]
; Phase: 1. Verification obligation: reject-if-unknown-at-version.

CustodyModelTransitionPayload = {
  transition_id:           URI,                 ; stable per Companion §10.3(1)
  from_custody_model:      CustodyModelId,      ; one of CM-A..CM-F, or registered extension
  to_custody_model:        CustodyModelId,
  effective_at:            uint,                ; Unix seconds UTC; Companion §10.3(6) "effective_time"
  reason_code:             ReasonCode,          ; enumerated; see below
  declaration_doc_digest:  digest,              ; SHA-256 over dCBOR(PostureDeclaration in force AFTER transition)
  transition_actor:        URI,                 ; Companion §10.3(4)
  policy_authority:        URI,                 ; Companion §10.3(5)
  temporal_scope:          TemporalScope,       ; Companion §10.3(7)
  attestations:            [* Attestation],     ; Companion §10.3(8); see §10.4 dual-attestation rule
  extensions:              { * tstr => any } / null,
}

CustodyModelId = tstr .regexp "^(CM-[A-Z])|(x-.+)$"    ; reserved CM-A..CM-F plus vendor extensions
ReasonCode     = &(
  GovernanceApproval:         0,
  RecoveryAuthorityChange:    1,
  DelegatedComputeChange:     2,
  AttestationAuthorityChange: 3,
  CorrectionOfPriorOverclaim: 4,
  ThresholdCompositionChange: 5,
  OrganizationalAuthorityChange: 6,
  ClientKeyAuthorityChange:   7,
  Other:                      255,
)
TemporalScope = &(
  Prospective:    0,
  Retrospective:  1,
  Both:           2,
)
Attestation = {
  authority:       URI,
  authority_class: &(Prior: 0, New: 1),
  signature:       bstr,                        ; COSE_Sign1 countersignature over (transition_id, effective_at)
}
```

The enclosing `EventPayload.header.event_type` is the bstr UTF-8 encoding of `"trellis.custody-model-transition.v1"`. `EventPayload.header.classification` is operator-chosen but MUST be outcome-neutral per §12.4. No commitment slots are required for transition events (Core §13 slots are reserved for payload-level field commitments, not governance events); `EventPayload.commitments` is `null` or `[]`.

**Field constraints:**

- `from_custody_model` MUST NOT equal `to_custody_model`; a no-op transition is a conformance violation (silence satisfies the same purpose).
- `reason_code` is plaintext and MUST NOT leak outcome information per §12.4. The enumeration is outcome-neutral by construction.
- `declaration_doc_digest` is a SHA-256 digest computed via the generic procedure of Core §9.1 with domain tag `trellis-posture-declaration-v1` (see Follow-ons — new domain-tag registration).
- `attestations` MUST satisfy Companion §10.4: an expansion transition (for example, CM-B → CM-A, or any transition widening the `provider_readable` decryptor set) MUST carry ≥1 attestation with `authority_class=Prior` AND ≥1 with `authority_class=New`. A narrowing transition MAY carry only `authority_class=New`. A verifier enforces this by cross-referencing the prior declaration's custody model against the new one.

## Decision 4 — CDDL: disclosure-profile transition

Parallel shape, distinct namespace. The "disclosure profile" here refers to the Respondent Ledger Profile A/B/C posture claim for a declared scope — it is how the Trellis-wrapped ledger represents the posture axes the Respondent Ledger spec owns at §15A (privacy × identity × integrity-anchoring). Invariant #11: Trellis does not redefine Profile A/B/C; it carries the field value as a qualified string.

```cddl
; Registered under EventHeader.event_type = "trellis.disclosure-profile-transition.v1"
; Payload rides in EventPayload.extensions["trellis.disclosure-profile-transition.v1"]
; Phase: 1. Verification obligation: reject-if-unknown-at-version.

DisclosureProfileTransitionPayload = {
  transition_id:           URI,
  from_disclosure_profile: RespondentLedgerProfile,  ; owned by Formspec Respondent Ledger §15A
  to_disclosure_profile:   RespondentLedgerProfile,
  effective_at:            uint,
  reason_code:             ReasonCode,
  declaration_doc_digest:  digest,                   ; as in §3 above
  scope_change:            ScopeChange,              ; narrowing vs widening, see below
  transition_actor:        URI,
  policy_authority:        URI,
  temporal_scope:          TemporalScope,
  attestations:            [* Attestation],
  extensions:              { * tstr => any } / null,
}

RespondentLedgerProfile = tstr .regexp "^rl-profile-[ABC]$"
  ; Always namespace-qualified. Bare "A" / "B" / "C" forbidden per OC-06 and invariant #11.

ScopeChange = &(
  ScopeNarrowing: 0,   ; post-transition disclosure scope is a subset of pre-transition
  ScopeWidening:  1,   ; post-transition disclosure scope is a superset of pre-transition
  ScopeOrthogonal: 2,  ; neither subset nor superset (e.g., privacy-tier swap for identity-tier)
)
```

`scope_change` is a single enum, not two sibling flags: "narrowing" and "widening" are mutually exclusive at the transition boundary, and a `ScopeOrthogonal` swap exists for Profile changes that exchange one axis for another (e.g., stricter privacy in exchange for weaker identity binding). Making it one enum forecloses the bug where both flags get set or both get cleared.

**Attestation rule.** `ScopeWidening` requires dual attestation (Companion §10.4); `ScopeNarrowing` MAY carry only the new-authority signature; `ScopeOrthogonal` requires dual attestation by default because at least one axis widens. A verifier computes the required attestation set from `scope_change` and enforces.

## Decision 5 — Verification semantics

A conforming verifier processing a chain that contains ≥1 posture-transition event MUST perform the following, in addition to Core §19 algorithm steps 1–9:

1. **Schema validation.** Decode `EventPayload.extensions["trellis.custody-model-transition.v1"]` (or `.disclosure-profile-transition.v1`) as the CDDL type above. Reject unknown keys inside the payload per Core §6.7. Fatal failure on structural mismatch.

2. **State-continuity check.** Maintain two per-scope shadow states while walking the chain: `last_custody_model` (initial value = the custody model declared in the deployment's initial Posture Declaration per Companion §11.3, or `null` if the chain has no prior declaration) and `last_disclosure_profile` (initial value = the deployment's Respondent Ledger Profile claim, or `null`). For each transition event:
   - `from_custody_model` (or `from_disclosure_profile`) MUST equal the current shadow state.
   - After validation, update the shadow state to `to_*`.
   A mismatch is a **localizable failure** per Core §19 (not fatal — accumulate and continue so a single misordered transition does not mask other tampering).

3. **Declaration-document resolution.** Resolve `declaration_doc_digest` against the export's declaration material (Companion §11.3: embedded or content-addressed). If the declaration is absent, record `posture_declaration_unresolved` and continue; if the declaration is present but digest mismatches, fatal failure (tampering).

4. **Attestation count check.** For expansion / orthogonal transitions, verify dual attestation per §10.4. Missing required attestation is a localizable failure.

5. **Downstream obligation surfacing.** Record each transition in the report's `posture_transitions` list so relying parties can see the posture history of the chain.

**VerificationReport additions (CDDL extension to Core §19):**

```cddl
VerificationReport =/ {
  ...,
  posture_transitions: [* PostureTransitionObservation],
}

PostureTransitionObservation = {
  event_sequence:          uint,                   ; sequence within scope
  transition_kind:         &(CustodyModel: 0, DisclosureProfile: 1),
  from_value:              tstr,                   ; stringified from_* field
  to_value:                tstr,
  effective_at:            uint,
  declaration_resolved:    bool,                   ; false if declaration material was unavailable
  continuity_verified:     bool,                   ; from_* matched shadow state
  attestation_sufficient:  bool,                   ; §10.4 rule satisfied
  reason_code:             ReasonCode,
}
```

A verifier that encounters `continuity_verified = false` MUST set `integrity_verified = false` at the report level. `declaration_resolved = false` does NOT by itself invalidate `integrity_verified` — it is reported through the existing §19 `omitted_payload_checks` / `warnings` channel so that an export intentionally shipped without its declaration document is still byte-verifiable, consistent with §11.3.

## Decision 6 — Fixture plan

Vectors slot into the existing G-3 layout (`fixtures/vectors/{append,verify,export,tamper}/NNN-slug/`) per the G-3 fixture-system design. No new op-dir. No new runner. Coverage claims add to existing TR-CORE / TR-OP rows without new matrix rows (TR-CORE-102 already covers the invariant; new TR-OP-* rows for the event schemas themselves are a follow-on — see below).

Target coverage for O-5:

| Op-dir | Vector | Purpose | Approximate invariant / TR coverage |
|---|---|---|---|
| `append/` | `NNN-custody-transition-cm-b-to-cm-a` | Happy path: CM-B → CM-A (widening), dual attestation, fresh declaration document digest. | TR-CORE-102; Companion §10.1, §10.3, §10.4, §11.3; invariant #9 (header outcome-neutrality), #15 (posture honesty). |
| `append/` | `NNN-custody-transition-cm-c-narrowing` | CM-C → CM-B with single new-authority attestation; `reason_code = DelegatedComputeChange`; `temporal_scope = Prospective`. | TR-CORE-102; §10.3, §10.4 narrowing branch. |
| `append/` | `NNN-disclosure-profile-transition-a-to-b` | Respondent Ledger Profile rl-profile-A → rl-profile-B; `scope_change = ScopeOrthogonal`; dual attestation. | TR-CORE-102; invariant #11 (profile namespacing); RL §15A binding. |
| `tamper/` | `NNN-transition-from-mismatch` | Chain has three events: initial declaration, transition claiming `from_custody_model = CM-A` when shadow state is `CM-B`. Expected report: `continuity_verified = false`, `integrity_verified = false`, failure code `posture_transition_continuity_violation`. | Verifier §5 state-continuity rule; OC-09 (no retroactive rewrite). |
| `tamper/` | `NNN-transition-missing-dual-attestation` | Widening transition with only `authority_class = New` attestation. Expected: `attestation_sufficient = false`, `integrity_verified = false`. | §10.4 dual-attestation enforcement. |
| `tamper/` | `NNN-transition-declaration-digest-mismatch` | `declaration_doc_digest` does not resolve to a digest matching any published declaration in the export. Expected: fatal failure `posture_declaration_digest_mismatch`. | §11.3 export-bundle binding; Decision 1 rule (co-publish required). |

Six vectors total (three happy-path, three tamper). This is the minimum to exercise: both event types, both attestation-count branches, the state-continuity rule, the declaration-digest resolution rule, and the scope-change enum. Sized **S** per the G-3 design's vector-batch convention; fits one authoring session after the `append/` residue batch (TODO.md critical-path step 2) lands the prerequisite chain-linkage vectors.

**Time this batch to land after `append/005-prior-head-chain`.** Every transition vector requires a non-genesis chain (the first event establishes initial posture; the transition is a subsequent event). Without `append/005`, the transition vectors would either duplicate prior-head mechanics or depend on an uncommitted convention. The TODO "Author O-5 fixtures" entry already notes this dependency.

**Derivation evidence** per G-3: each vector ships `derivation.md` citing Companion §10.3 (field-by-field), §10.4 (attestation rule), §11.3 (declaration binding), Core §6.7 (extension-registration), and §9.1 (digest construction for `declaration_doc_digest`). Intermediates (the fresh `AuthorEventHashPreimage`, the CanonicalEventHashPreimage with the transition extension entry, the Sig_structure) are committed as sibling `.bin` files per G-3's derivation-evidence convention.

## Appendix A.5 disposition

The existing Appendix A.5 ("Posture Transition Event") contains a generic structural sketch:

```
PostureTransition {
  transition_id:        URI
  prior_posture_ref:    URI
  new_posture_ref:      URI
  transition_actor:     URI
  policy_authority:     URI
  effective_time:       timestamp
  temporal_scope:       enum { prospective, retrospective, both }
  attestations:         [ { authority, authority_class, signature } ]
}
```

**Preserve** the generic-parent shape as context — it maps one-to-one to the eight generic §10.3 fields and is informative for the Appendix. **Supersede** its role as the authoritative transition schema: the two CDDL blocks above (custody-model and disclosure-profile) are the normative payloads. Proposed Appendix A.5 rewrite: re-title "Posture Transition Event Families," keep the generic struct as the shared parent, and add two sub-sections A.5.1 and A.5.2 carrying the CDDL from Decisions 3 and 4 verbatim. The `prior_posture_ref` / `new_posture_ref` URI fields in the parent are realized by `declaration_doc_digest` + the implicit `from_*` / `to_*` values in the subtypes. No other Appendix A content is affected.

## Follow-ons

Spec edits this design demands, scoped for separate landings:

- **Core §6.7 extension table:** register `trellis.custody-model-transition.v1` and `trellis.disclosure-profile-transition.v1` under `EventPayload.extensions` with Phase 1 / reject-if-unknown-at-version obligation. Cite Companion §10 as the normative semantic anchor.
- **Core §9.8 domain-tag registry:** add `trellis-posture-declaration-v1` for the `declaration_doc_digest` construction. The digest is computed via the generic §9.1 procedure over `dCBOR(PostureDeclaration)` — the Appendix A.1 CDDL struct, once that is turned into proper CDDL (Appendix A is currently prose-structured; coordinate with O-4 stream).
- **Core §19 (Verification Algorithm):** add a step 5.5 for state-continuity check and attestation-count check; extend the `VerificationReport` CDDL with `posture_transitions` per Decision 5. This is an additive change per §6.5 strict-superset semantics.
- **Matrix new rows:** `TR-OP-042` (custody-model transition event schema), `TR-OP-043` (disclosure-profile transition event schema), `TR-OP-044` (verifier state-continuity rule), `TR-OP-045` (declaration-digest co-publish rule). Each cites Companion §10 and this brief. TR-CORE-102 remains the cross-cutting anchor.
- **Lint checks (`scripts/check-specs.py`):** (a) event-type registry appends only — a PR removing `trellis.custody-model-transition.v1` or its sibling from the §6.7 table fails; (b) CDDL cross-reference — the two payload types appear in Core §28 Appendix A and match this brief's CDDL byte-for-byte; (c) a fixture under `append/*-transition-*` or `tamper/*-transition-*` must declare one of the two registered event-types in its manifest.
- **Companion §27.7 ("Transition-Auditability Tests"):** reword to cite the six O-5 fixtures above by ID once authored, replacing the current prose-only list.

## Open items

- **Reason-code registry governance.** The enumeration in Decision 3 covers the obvious cases but is not exhaustive. Proposal: treat `ReasonCode` as append-only per the Core §6.7 extension-registration discipline; `Other = 255` is a reserved catch-all for transitions that do not fit registered codes, and auditors flag `Other` for manual review. Warrants confirmation before landing as CDDL.
- **Attestation signature scope.** Decision 3's `Attestation.signature` is stated to be a COSE_Sign1 countersignature over `(transition_id, effective_at)`. This wants a precise `Sig_structure` preimage like Core §6.6 defines for event signatures — "over the tuple" is under-specified. Propose: `Sig_structure` payload = `dCBOR([transition_id, effective_at, authority_class])`, with domain-separation via a new tag `trellis-transition-attestation-v1`. Mechanically simple; needs review before citing in a TR-OP row.
- **Disclosure-profile scope granularity.** A deployment may declare Respondent Ledger Profile A/B/C at response-scope, case-scope, or deployment-scope. This brief assumes deployment-scope (the Posture Declaration covers one scope per OC-03). If an operator wants to transition profile at finer granularity (a single case upgrades to stricter privacy posture without revising the deployment declaration), the current design forces a new declaration at the deployment level. That may be the right answer — greenfield discipline — or it may be too coarse. Warrants brainstorming before fixtures commit.
- **Interaction with `trellis.external_anchor.v1`** (Core §6.7 Phase 2 reservation). A Phase 2 deployment with external anchoring may want to anchor transition events with higher priority than ordinary events. The current brief does not address this; Phase 1 has no anchoring obligation. Non-blocking.

## Non-goals

- Defining the declaration-document template (Companion Appendix A.1–A.4). O-4 stream owns that.
- Authoring the six fixtures. Separate plan, blocked on `append/005-prior-head-chain`.
- Defining transition semantics for Conformance Classes (Core §2.1). Conformance Classes are Core byte-level implementation classes; they do not transition in the operational sense. Invariant #11 keeps that namespace separate.
- Phase 4 witness attestation of transitions. Companion §26 seam, implementation deferred.
