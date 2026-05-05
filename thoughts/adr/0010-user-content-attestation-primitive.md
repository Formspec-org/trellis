# ADR 0010 — User-Content Attestation Primitive

**Date:** 2026-04-28
**Status:** Accepted (pending implementation)
**Supersedes:** —
**Superseded by:** —
**Related:** ADR 0001-0004 (DAG envelope + Rust byte authority); ADR 0006 (KeyEntry signing class); ADR 0007 (certificate-of-completion — top-level artifact that cites this primitive in its `attestations[]`); WOS [Signature Profile §2.8 SignatureAffirmation](../../../work-spec/specs/profiles/signature.md) (workflow-level provenance record that carries this primitive); proposed parent-repo identity-attestation stack ADR (PLN-0381); proposed `work-spec/specs/profiles/signature.md` §1.3 reopen + extension (PLN-0380).

## Decision

Trellis defines a canonical event extension `trellis.user-content-attestation.v1` that records one user (non-operator) attesting to one piece of in-chain content with a declared `signing_intent`. The attestation binds three things by hash: the host event being attested (chain-position binding), the identity attestation that proves the attestor's identity at attestation time (claim-graph binding), and the user's intent (URI-encoded; Trellis owns the bytes, WOS owns the meaning). A new domain-separation tag `trellis-user-content-attestation-v1` (Core §9.8) covers the signature preimage.

This is the byte-level primitive. Higher-level events compose it: a `SignatureAffirmation` (WOS workflow-level provenance) carries one or more user-content attestations as its proof; a `trellis.certificate-of-completion.v1` event references them in its `attestations[]` slot when the actor is a user attesting to user content (rather than an operator attesting to a posture change). The primitive may also stand alone — a notary attesting to a document outside any WOS workflow.

The rejected alternative — folding the byte primitive into `SignatureAffirmation` directly without a Trellis-side CDDL — would couple WOS workflow semantics to the byte format and prevent non-workflow-driven attestations from sharing the same wire shape, the same domain tag, the same verifier path, and the same fixtures.

## Context

### The gap

ADR 0007 ratifies certificate-of-completion as the top-level artifact (PDF + chain summary + attestations). Its `attestations: [+ Attestation]` field currently reuses Companion §A.5's `Attestation` shape — but A.5 is the *operator-actor* attestation primitive used in posture transitions (custody-model, disclosure-profile, erasure-evidence). The semantics differ:

- **A.5 Attestation:** an operator or governance authority attests to a *posture change*. Authority class is `prior` / `new`. Signature preimage is `dCBOR([transition_id, effective_at, authority_class])` under `trellis-transition-attestation-v1`. No identity-attestation reference; authority is the principal.

- **User-content attestation:** a user (applicant, signer, witness, notary, respondent) attests to *user content* (a form response, a document, a finding). Authority isn't a posture-axis side; the attestor is a person whose identity the verifier must resolve through a separate identity-attestation event. Intent is per-attestation, not per-axis-side.

WOS's `SignatureAffirmation` (Signature Profile §2.8) sits one layer up — it's the workflow-level provenance record that names which signer participated in which signing task, and currently folds the byte-level signature into its payload informally. Without a Trellis-side CDDL, the byte format leaks into WOS's namespace, prevents non-workflow attestations from sharing the wire shape, and forecloses the verifier composition that ADR 0007 already calls for.

### What the primitive carries

Three bindings, one signature, one intent URI:

1. **Chain-position binding.** `attested_event_hash` (the canonical event hash of the host event being attested) plus `attested_event_position` (its sequence within `ledger_scope`). A verifier MUST resolve both and confirm consistency — preventing attestation-event-detachment attacks (signing a hash without committing to its position).
2. **Identity binding.** `identity_attestation_ref` (canonical event hash of an identity-attestation event whose subject equals `attestor`). Forecloses claim-graph attacks where the attestor principal is unverifiable.
3. **Intent binding.** `signing_intent` is a URI string. Trellis verifies it is syntactically a URI (RFC 3986); meaning is owned by WOS Signature Profile + jurisdiction-specific intent registries (PLN-0380).

The signature is detached Ed25519 over the dCBOR-encoded preimage `dCBOR([attestation_id, attested_event_hash, attested_event_position, attestor, identity_attestation_ref, signing_intent, attested_at])` under domain tag `trellis-user-content-attestation-v1`.

## Wire shape

Under `EventPayload.extensions["trellis.user-content-attestation.v1"]`:

```cddl
UserContentAttestationPayload = {
  attestation_id:           tstr,                  ; stable within ledger_scope; idempotent re-emit by hash
  attested_event_hash:      digest,                ; canonical_event_hash of the host event being attested
  attested_event_position:  uint,                  ; 0-based sequence within ledger_scope
  attestor:                 tstr,                  ; principal URI
  identity_attestation_ref: digest / null,         ; canonical_event_hash of an identity-attestation event;
                                                   ; null only when the deployment Posture Declaration admits
                                                   ; unverified attestors (default REQUIRED non-null)
  signing_intent:           tstr,                  ; URI per RFC 3986; meaning per WOS Signature Profile registry
  attested_at:              uint,                  ; Unix seconds UTC; MUST equal envelope authored_at exactly
  signature:                bstr,                  ; detached Ed25519 over the user-content attestation preimage
                                                   ; (dCBOR([attestation_id, attested_event_hash,
                                                   ;         attested_event_position, attestor,
                                                   ;         identity_attestation_ref, signing_intent,
                                                   ;         attested_at])
                                                   ;  under domain tag trellis-user-content-attestation-v1)
  signing_kid:              tstr,                  ; Core §8 KeyEntry signing-class kid; MUST be Active at attested_at
  extensions:               { * tstr => any } / null,
}
```

### Field semantics

- **`attestation_id`** — operator- or runtime-minted identifier, stable within `ledger_scope`. Re-emission with the same `attestation_id` MUST carry a byte-identical canonical payload (idempotency-by-hash); divergent re-emission flips `integrity_verified = false` with `user_content_attestation_id_collision`. First-seen wins is non-normative.
- **`attested_event_hash` / `attested_event_position`** — the chain-position binding. Verifier MUST resolve `attested_event_position` to a chain-present event in `ledger_scope` and confirm its `canonical_event_hash` equals `attested_event_hash`. Position-without-hash or hash-without-position attestations are rejected; both fields are normative and disagreement is `user_content_attestation_chain_position_mismatch`.
- **`attestor`** — principal URI of the user attesting. NOT an operator URI; per Companion §6.4 the operator is a distinct role and its attestations belong in A.5's `Attestation` shape. Lint enforcement and conformance fixtures forbid operator URIs in this slot.
- **`identity_attestation_ref`** — canonical event hash of a chain-present event whose `event_type` is registered as an identity-attestation kind. The initial registration target is `wos.identity.*` (PLN-0381 + PLN-0384); until those ratify, deployments MAY register a deployment-local identity-attestation extension type per Core §6.7. Resolution rules: the resolved event's payload MUST carry a subject equal to `attestor`, MUST be in the same `ledger_scope`, and MUST be at `sequence < attested_event_position` (identity proof temporally precedes the attestation). Unresolved or wrong-subject reference flags `user_content_attestation_identity_unresolved` / `_subject_mismatch`.
  - **`null` admission:** permitted only when the Posture Declaration in force at `attested_at` declares `admit_unverified_user_attestations: true`. Default is REQUIRED non-null. Verifier checks the posture and flags `user_content_attestation_identity_required` if the posture forbids null.
- **`signing_intent`** — URI string per RFC 3986. Trellis verifies syntactic validity only (scheme present, well-formed). Semantic registry lives under WOS Signature Profile §1.3 + extension (PLN-0380): the URI names which legal-effect class the attestation claims (signature, witness, notary, consent, attestation-of-fact). Unregistered URIs are admitted at the Trellis layer; semantic gating happens at the WOS verifier and at deployment configuration. Malformed URIs flag `user_content_attestation_intent_malformed`.
- **`attested_at`** — Unix seconds UTC, MUST exactly equal the envelope's `authored_at`. Exact equality is normative (no skew slack); the attestation timestamp is the act of authoring the envelope. Mismatch flags `user_content_attestation_timestamp_mismatch`.
- **`signature`** — detached Ed25519 over the dCBOR preimage under `trellis-user-content-attestation-v1`. Distinct domain tag from `trellis-transition-attestation-v1` so a wrongly-typed attestation cannot cross-validate against either family's verifier. Failure flags `user_content_attestation_signature_invalid`.
- **`signing_kid`** — Core §8 KeyEntry of class `signing` (ADR 0006). MUST be in lifecycle state `Active` (or `Rotating` per the rotation grace window once ratified — TODO item #5) at `attested_at`. Retired / Revoked / wrong-class kid flags `user_content_attestation_key_not_active`.

`Attestation` from Companion §A.5 is NOT reused here. The user-content primitive is a separate shape with a separate domain tag.

## Event-type registration (Core §6.7)

Add to the Core §6.7 Extension Registry:

| event_type | admitted payload | authority boundary |
|---|---|---|
| `trellis.user-content-attestation.v1` | `UserContentAttestationPayload` (this ADR) | User-authored (Companion §6.4 distinguishes user from operator); subject to identity-attestation resolution per Core §19 step 6d. Verifier obligations below. |

## Domain-separation tag (Core §9.8)

| tag | scope |
|---|---|
| `trellis-user-content-attestation-v1` | dCBOR preimage for `UserContentAttestationPayload.signature`. Distinct from `trellis-transition-attestation-v1` (operator-actor posture-transition signature) so wrongly-typed attestations cannot cross-validate. |

## Verifier obligations (Core §19 step 6d)

A conforming verifier processing an export bundle containing `trellis.user-content-attestation.v1` events MUST, in order:

1. **Decode** the payload against the CDDL above. Mismatch is a structure failure (Core §19 step 1).
2. **Validate** intra-payload invariants: `attested_at == envelope.authored_at`; `signing_intent` is a syntactically valid URI per RFC 3986. Each failure flips `integrity_verified = false` with `user_content_attestation_timestamp_mismatch` / `user_content_attestation_intent_malformed`.
3. **Resolve** `attested_event_position` to a chain-present event in `ledger_scope` and confirm its `canonical_event_hash` equals `attested_event_hash`. Mismatch flags `user_content_attestation_chain_position_mismatch`.
4. **Resolve** `identity_attestation_ref`:
   - If null: confirm the Posture Declaration in force at `attested_at` declares `admit_unverified_user_attestations: true`. Otherwise flag `user_content_attestation_identity_required`.
   - If non-null: resolve to a chain-present event of a registered identity-attestation event type, confirm its `ledger_scope` matches, confirm its `sequence < attested_event_position`, and confirm its payload subject equals `attestor`. Failure flags `user_content_attestation_identity_unresolved` (no resolve) / `user_content_attestation_identity_subject_mismatch` (resolved but wrong subject) / `user_content_attestation_identity_temporal_inversion` (sequence not strictly prior).
5. **Verify** `signature` over `dCBOR([attestation_id, attested_event_hash, attested_event_position, attestor, identity_attestation_ref, signing_intent, attested_at])` under domain tag `trellis-user-content-attestation-v1`, using the public key resolved from `signing_kid`. Failure flags `user_content_attestation_signature_invalid`.
6. **Validate key state** at `attested_at`: the Core §8 KeyEntry for `signing_kid` MUST be class `signing` and lifecycle state `Active` (or `Rotating` per ratified rotation grace; TODO item #5). Wrong class or wrong state flags `user_content_attestation_key_not_active`.
7. **Detect collision**: after decoding all user-content-attestation events in scope, two events sharing `attestation_id` with disagreeing canonical payload flag `user_content_attestation_id_collision`.
8. **Forbid operator-as-attestor**: when `attestor` resolves to a principal class registered as `operator` (Companion §6.4), flag `user_content_attestation_operator_in_user_slot`. Operator attestations belong in A.5's `Attestation` shape.
9. **Accumulate** outcomes into `VerificationReport.user_content_attestations`, parallel to `posture_transitions` / `erasure_evidence` / `certificates_of_completion`. Each entry carries: `attestation_id`, `attested_event_hash`, `attestor`, `signing_intent`, `chain_position_resolved`, `identity_resolved`, `signature_verified`, `key_active`, `failures`.

**Global integrity (user-content-attestation slice).** `integrity_verified = false` if any entry has any of `chain_position_resolved = false`, `identity_resolved = false` (when required), `signature_verified = false`, `key_active = false`, or any step 7–8 failure.

## Composition with existing primitives

- **Certificate-of-completion (ADR 0007).** `CertificateOfCompletionPayload.attestations[*]` MAY mix A.5 `Attestation` (operator closing the workflow per OC-11) with user-content-attestation references. ADR 0007 §"Wire shape" reads `Attestation` as Companion A.5's shape today; a follow-on amendment (sequenced under the next ADR slot, not a "phase" of this one) may extend the field to a discriminated union. Until then, certificates carry user signatures via the `signing_events` slot (which references `SignatureAffirmation` events) and operator attestations via `attestations`.
- **SignatureAffirmation (WOS Signature Profile §2.8).** WOS emits `SignatureAffirmation` provenance through `custodyHook`. The byte-level proof inside that record is a `UserContentAttestationPayload` — one per signer/document pair. The Signature Profile §1.3 reopen (PLN-0380) ratifies the binding so WOS authoring populates this CDDL directly.
- **Stand-alone use.** A notary signing a document outside any WOS workflow appends a bare `trellis.user-content-attestation.v1` event with `signing_intent` naming the notarial-attestation URI. No SignatureAffirmation, no certificate-of-completion. The verifier path is the same; verification independence per Core §16 holds.

## Operator workflow (CLI, non-normative)

`trellis-cli sign-user-content` — reference UX precedent mirrors `seal-completion` (ADR 0007) and `erase-key` (ADR 0005):

```
trellis-cli sign-user-content \
  --attested-event <digest>           # canonical_event_hash of the host event
  --attested-position <uint>          # sequence within ledger_scope
  --attestor <principal-uri>          # user URI
  --identity-attestation <digest>     # canonical_event_hash of identity event (or omit if posture admits null)
  --signing-intent <uri>              # RFC 3986 URI
  --signing-key <cose-key-file>       # signing-class kid (Core §8)
```

Atomic unit: (a) resolve key + identity references; (b) construct payload; (c) compute canonical hash + sign; (d) append.

## Adversary model

What this design catches:

- **Wrong-position attestation.** Attestor signs a hash without committing to its chain position; verifier resolves position and detects mismatch.
- **Detached identity claim.** Attestor signs without a resolvable identity-attestation reference; default-required posture rejects.
- **Cross-family signature confusion.** Wrong-domain-tag signature (e.g., A.5 Attestation bytes presented as user-content attestation) fails the domain-separated verification.
- **Operator masquerading as user.** Operator URI in `attestor` slot triggers `user_content_attestation_operator_in_user_slot`.
- **Key-state evasion.** Attestation issued under a Retired or Revoked kid fails `user_content_attestation_key_not_active`.
- **Idempotency collision.** Same `attestation_id`, divergent payload — fails `user_content_attestation_id_collision`.

What this design does NOT catch:

- **Semantic-intent fraud.** Trellis verifies the `signing_intent` is a syntactically valid URI; it does NOT validate that the URI's claimed legal effect (e.g., notarial intent) matches the signer's actual jurisdictional capacity. WOS Signature Profile + jurisdiction registry handle that. Mitigation: registered URIs in the Posture Declaration; lint refuses unregistered URIs in production deployments.
- **Identity-event forgery upstream.** If a deployment admits a forged identity-attestation event into the chain, this primitive will resolve to it and verify. Identity-event integrity is the identity-attestation ADR's problem (PLN-0381).
- **Collusive intent registration.** Operator registers a deceptive `signing_intent` URI in the deployment's Posture Declaration. Out of scope; jurisdictional procurement review handles that.

## Alternatives considered

### Option A — fold the byte primitive into `SignatureAffirmation` only (rejected)

Keep the byte format inside WOS's `SignatureAffirmation` payload, no Trellis-side CDDL. Declined: couples Trellis envelope discipline to WOS workflow semantics; prevents non-workflow attestations from sharing wire shape, domain tag, verifier path, fixtures. Forecloses ADR 0007 `attestations[]` from carrying user attestations cleanly. Conflates "what was signed" (Trellis) with "what workflow recorded the signing" (WOS).

### Option B — reuse Companion §A.5 `Attestation` (rejected)

Reuse the existing posture-transition attestation shape with new optional fields for identity reference + intent. Declined: A.5's `authority_class: "prior" / "new"` semantics don't apply to user-actor attestations; cross-family domain tag would conflate operator and user attestations under one verifier path; field-level extension would require breaking A.5's CDDL structure. Cleaner to ratify a separate primitive with a separate domain tag.

### Option C — extend the COSE_Sign1 envelope itself, not a payload extension (rejected)

Carry `attested_event_hash` / `attested_event_position` / `signing_intent` / `identity_attestation_ref` directly in the COSE_Sign1 protected header. Declined: violates ADR 0001-0004's "envelope is uniform; per-event-type variation lives in `EventPayload.extensions`"; would require COSE header registration; would prevent the same primitive from riding inside `SignatureAffirmation` records that WOS authors.

## Composition

- **Envelope compatible.** Rides `EventPayload.extensions` (Core §6.7). No envelope change; ADR 0003 preserved.
- **Runtime composes immediately.** Ships alongside SignatureAffirmation (live per WOS-T4) and certificate-of-completion (ADR 0007 — Wave 22 closed). Identity-attestation resolution degrades to deployment-local extension types until PLN-0381 ratifies the parent-repo stack ADR.
- **Forward-compat surfaces.** `signing_intent` URI registry expands without wire change. Posture Declaration field `admit_unverified_user_attestations` toggles default-required to default-permissive without wire change. Identity-attestation event types expand under Core §6.7's extension registry.
- **Case-ledger composition.** User-content attestations compose into case ledgers identically to other Trellis events. The host-event-binding and identity-binding fields tie attestations to the case lineage.

## Fixture plan

Eleven vectors total: four positive, seven tamper. Mirror ADR 0007's coverage proportions.

| Vector | Purpose |
|---|---|
| `append/036-user-content-attestation-minimal` | Single attestation, single attestor, identity-attestation reference present, well-formed signing_intent URI. |
| `append/037-user-content-attestation-multi-attestor` | Two attestation events on one host event (applicant + witness), distinct `attestation_id`, distinct `signing_intent` URIs. |
| `append/038-user-content-attestation-without-identity` | Posture Declaration admits unverified attestors; `identity_attestation_ref = null`; verifier accepts. |
| `append/039-user-content-attestation-stand-alone` | Bare attestation event with no SignatureAffirmation host (e.g., notarial attestation outside any workflow); verifier path unchanged. |
| `tamper/028-uca-signature-invalid` | Valid structure, signature computed under wrong domain tag (A.5's `trellis-transition-attestation-v1`). |
| `tamper/029-uca-chain-position-mismatch` | `attested_event_position` resolves to a chain event whose `canonical_event_hash` ≠ `attested_event_hash`. |
| `tamper/030-uca-identity-unresolved` | `identity_attestation_ref` digest does not resolve to any chain event. |
| `tamper/031-uca-identity-subject-mismatch` | Resolved identity-attestation event subject ≠ `attestor`. |
| `tamper/032-uca-identity-temporal-inversion` | Resolved identity-attestation event has `sequence ≥ attested_event_position` (identity must temporally precede). |
| `tamper/033-uca-intent-malformed` | `signing_intent` is not a syntactically valid URI per RFC 3986. |
| `tamper/034-uca-key-not-active` | `signing_kid` is class `signing` but lifecycle state is `Retired` at `attested_at`. |

Idempotency-collision (`attestation_id` reuse) and operator-as-attestor are covered by lint + conformance prose; if either surfaces an integrity failure mode the corpus does not catch, follow-on tamper vectors land under the next free slot.

## Open questions and downstream ADR slots

Each item below is a separate decision; none blocks ADR 0010's claim. When activated, each lands as its own ADR — none is a future "phase" of this one.

1. **Identity-attestation event taxonomy.** Parent-repo stack ADR (PLN-0381) ratifies `IdentityAttestation` shape and `wos.identity.*` namespace registration. Until ratified, `identity_attestation_ref` resolves to deployment-local extension types per Core §6.7. ADR 0010 does not block on PLN-0381.
2. **Signing-intent URI registry.** WOS Signature Profile §1.3 reopen (PLN-0380) ratifies the URI registry and meaning. Until ratified, Trellis admits any well-formed URI; deployments register intent URIs in their Posture Declaration. ADR 0010 does not block on PLN-0380.
3. **Certificate-of-completion `attestations[]` discriminated union.** ADR 0007's `attestations: [+ Attestation]` field reuses A.5's `Attestation`. A follow-on ADR may extend the field to a discriminated union admitting both A.5 and user-content shapes. Sequenced after ADR 0010 ratification; not a "phase" of this one.
4. **Rotation grace-window admission.** TODO item #5 ratifies the lifecycle `Rotating` overlap window. When that lands, step 6 of verifier obligations admits `Rotating` alongside `Active`. Until then, `Rotating` is not admitted.
5. **CLI ergonomics extension.** `trellis-cli sign-user-content` is non-normative and may grow flags (template-bound intent, identity-attestation auto-resolution, batch mode). Lands with adopter feedback; not a separate ADR.

## Cross-references

- **ADR 0007** — certificate-of-completion: top-level artifact citing this primitive in `attestations[]` (after the discriminated-union follow-on per open question 3).
- **ADR 0006** — KeyEntry signing class: `signing_kid` resolves through this taxonomy.
- **ADR 0001-0004** — envelope discipline (DAG-capable, single-parent runtime, reservation discipline). This ADR rides `EventPayload.extensions` per ADR 0003.
- **WOS Signature Profile §2.8** — SignatureAffirmation: workflow-level provenance record that carries this primitive as its byte-level proof.
- **WOS Signature Profile §1.3 reopen (PLN-0380)** — signing-intent URI registry; signer-authority claim shape; ESIGN/UETA/eIDAS posture mapping.
- **Identity attestation parent-repo stack ADR (PLN-0381)** — `IdentityAttestation` shape; `wos.identity.*` namespace; claim graph.
- **Companion §A.5** — operator-actor `Attestation` shape (distinct primitive, distinct domain tag).
- **Companion §6.4** — Operator role definition (the line between user and operator that step 8 of verifier obligations enforces).
- **Core §6.7** — Extension Registry (where `trellis.user-content-attestation.v1` registers).
- **Core §8** — KeyEntry taxonomy (signing-class lifecycle).
- **Core §9.8** — Domain separation (new `trellis-user-content-attestation-v1` tag).
- **Core §16** — Verification independence (this primitive's verifier path adds no derived-artifact dependency).
- **Core §19** — Verification Algorithm (new step 6d for user-content-attestation cross-checks).
- **Core §28** — Full CDDL appendix (where `UserContentAttestationPayload` appears).

## Implementation sequencing

1. **Spec amendments** — Core §6.7 registration row; Core §9.8 domain tag; Core §19 step 6d verifier obligations; Core §28 CDDL append; matrix rows TR-CORE-152..157; Companion §6.4 operator-vs-user reminder.
2. **Rust verifier** — extend `trellis-verify` with user-content-attestation decode, chain-position resolution, identity-resolution, signature verification, key-state check, report accumulation. `VerificationReport.user_content_attestations` field added.
3. **First positive vector** — `append/036-user-content-attestation-minimal` byte-matched end-to-end.
4. **Python stranger mirror** — `trellis-py` parity (G-5).
5. **Remaining positive vectors** — `append/037..039`.
6. **Tamper vectors** — `tamper/028..034` (per §Fixture plan).
7. **`trellis-cli sign-user-content`** command (ergonomics).

Steps 1–3 are the minimum for the ADR's claim to hold; steps 4–6 close the corpus; step 7 is adopter ergonomics.

---

*End of ADR 0010.*
