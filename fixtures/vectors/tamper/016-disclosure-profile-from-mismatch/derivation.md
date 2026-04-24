# Derivation — `tamper/016-disclosure-profile-from-mismatch`

## Header

**What this vector exercises.** The disclosure-profile axis sibling of `tamper/002-transition-from-mismatch`. Together the two vectors exercise Core §19 step 6.b (state-continuity check) across BOTH Phase-1 posture-transition subtypes: custody-model (tamper/002) and disclosure-profile (this vector). The tampered `trellis.disclosure-profile-transition.v1` event claims `from_disclosure_profile = "rl-profile-C"` when the deployment's initial declaration pins `disclosure_profile = "rl-profile-A"`. A conforming verifier MUST track a shadow state per transition axis (Companion Appendix A.5.3 step 2) and detect the mismatch.

**Context — why this vector exists now.** The 2026-04-23 design-doc audit (`thoughts/audit-2026-04-23-design-docs-vs-specs-and-code.md`) surfaced that `trellis-verify`'s `decode_transition_details` handled only custody-model transitions; a tampered disclosure-profile transition was decoded to `None` and skipped the step-6.b check entirely. G-O-5 was retroactively reopened in `ratification/ratification-checklist.md` pending this fix. The companion Rust change in the same commit series extends the decode arm to `trellis.disclosure-profile-transition.v1`, adds a parallel `shadow_disclosure_profile` baseline parsed from the declaration's top-level `disclosure_profile` string, and routes the attestation rule through Appendix A.5.3 step 4's `scope_change` enum (`Narrowing` MAY be attested alone; `Widening` / `Orthogonal` MUST be dually attested). This vector is the negative oracle for that code path.

**Scope.** Verifier-side only. The event re-signs end-to-end so the signature is valid — the tamper is purely semantic. `scope_change = "Orthogonal"` with both `prior` and `new` authorities present exercises the attestation step positively, leaving step 6.b as the load-bearing failure.

**Core / Companion § roadmap.** Spec prose evidence for each emitted byte follows the `tamper/002` roadmap one-to-one with Appendix A.5.2 substituted for A.5.1 and the disclosure-profile field names substituted throughout.

---

## Body

### Step 1 — Initial posture declaration (shadow-state baseline)

Declaration in force BEFORE any transition. Pins:

- `custody_model.custody_model_id = "CM-B"` (unchanged across the transition; present for completeness)
- `disclosure_profile = "rl-profile-A"` — **the shadow-state baseline for step 6.b**

Committed as `input-initial-posture-declaration.bin` (301 B, SHA-256 `bbedf857…8bd7c0`). The verifier extracts `disclosure_profile` from this declaration as the initial value of its `shadow_disclosure_profile` baseline (Rust: `parse_disclosure_profile()` in `trellis-verify/src/lib.rs`). Note the asymmetry: the custody-model baseline reads `custody_model.custody_model_id` from a sub-map, while the disclosure-profile baseline reads `disclosure_profile` as a top-level string — mirroring the declaration shape emitted by the corresponding positive fixtures (`append/006`, `append/008`).

### Step 2 — Post-transition declaration

Declaration the transition event's `declaration_doc_digest` points at. Pins `disclosure_profile = "rl-profile-B"`. Digest under `trellis-posture-declaration-v1`:

`declaration_doc_digest = SHA-256(domain_preimage("trellis-posture-declaration-v1", <declaration bytes>))`

Committed as `input-posture-declaration.bin` (354 B). The digest the tampered event carries IS the correct digest of this declaration, so step 6.c (declaration-digest match) passes — the ONLY failing step is 6.b.

### Step 3 — Dual attestation (Appendix A.5.3 step 4 `Orthogonal` branch)

Companion Appendix A.5.3 step 4:

> "`scope_change = "Orthogonal"` — MUST be dually attested. `Orthogonal` is the non-narrowing default and does not qualify for the reduced-attestation carve-out."

Both authorities sign. Preimages are domain-separated under `trellis-transition-attestation-v1` and Ed25519-signed. The attestation check passes; the continuity check is the load-bearing failure.

| authority_class | signing key | role label |
|---|---|---|
| `prior` | `_keys/attestation-authority-cm-b-001.cose_key` | `urn:trellis:authority:test-profile-a-authority` |
| `new`   | `_keys/issuer-001.cose_key`                     | `urn:trellis:authority:test-profile-b-authority` |

### Step 4 — Tampered `DisclosureProfileTransitionPayload` (Appendix A.5.2)

| Field | Value | Note |
|---|---|---|
| `transition_id` | `"urn:trellis:transition:test:tamper-016"` | |
| `from_disclosure_profile` | `"rl-profile-C"` | **TAMPER** — initial declaration pins `"rl-profile-A"` |
| `to_disclosure_profile` | `"rl-profile-B"` | |
| `effective_at` | `1745000500` | |
| `reason_code` | `4` (governance-policy-change) | |
| `declaration_doc_digest` | Step 2 output — 32 bytes | correct digest, step 6.c passes |
| `scope_change` | `"Orthogonal"` | dual attestation required |
| `transition_actor` | `"urn:trellis:principal:test-operator"` | |
| `policy_authority` | `"urn:trellis:authority:test-governance"` | |
| `temporal_scope` | `"prospective"` | |
| `attestations` | 2-element array (Step 3) | classes `{"prior", "new"}` |
| `extensions` | `null` | |

### Steps 5–8 — Event envelope, COSE_Sign1, canonical hash

Identical discipline to `tamper/002`. The event is re-signed end-to-end so the signature verifies cleanly; step 6.b is the sole failing step. Pinned salient values:

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| `input-initial-posture-declaration.bin` | 301 | `bbedf8572858eeb46509cdc2a841d8f721e93beb82dab1c490ec692a6e8bd7c0` |
| `input-posture-declaration.bin` | 354 | `ed994bb5dd9601a894a2d173c914a0057b8c61ef4f99fbac324e226ce54974b4` |
| `input-tampered-event.cbor` | 1461 | `b2c72d0c660e99c7790474868ebacfe71a03d58ec972282c693d956a41d20336` |
| `input-tampered-ledger.cbor` | 1462 | `15a9fa670c99245501dce14a6131d3dce4198755e76be3df25ed31629675e4ce` |
| `input-signing-key-registry.cbor` | 133 | `4f0efcbe40658fe661406d686007c3b8f1abf66132b3271f1e02799d72b41d08` |

Other pinned values:

- `kid` = `af9dff525391faa75c8e8da4808b1743`.
- `prev_hash` = `ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb` (from `append/001`).
- `failing_event_id` (= canonical_event_hash of the tampered event) = `fe79a51c60b9538a4157df34d7a15baa356c2c1ad9dd48a65ab87320ea480f6f`.

---

## Expected verifier behavior

Per Core §19 step 9 and Companion Appendix A.5.3:

| Field | Value | Reason |
|---|---|---|
| `structure_verified` | `true` | Event decodes cleanly and payload CDDL matches Appendix A.5.2 |
| `integrity_verified` | `false` | step-9 AND conjunction sees `posture_transitions[0].continuity_verified = false` |
| `readability_verified` | `true` | No decryption attempted; `PayloadInline` marker bytes are opaque |
| `tamper_kind` | `"state_continuity_mismatch"` | Matches tamper/002's custody sibling — verifier output contract stable across both axes |
| `failing_event_id` | `fe79a51c…480f6f` | canonical_event_hash of the tampered event (payload IS the mutation site) |

`posture_transitions[0]` field detail:

- `kind` = `"disclosure-profile"` (vs tamper/002's `"custody-model"`) — this is the verifier's reported dispatch and is itself verification-relevant: a verifier that misses the dispatch would report `kind = "custody-model"` or fail to produce an outcome entry at all, both of which are regressions from the G-O-5 reopen fix.
- `from_state` = `"rl-profile-C"` (the claimed value from the tampered payload, echoed into the report)
- `to_state` = `"rl-profile-B"`
- `continuity_verified` = `false`
- `declaration_resolved` = `true` (step 6.c passes)
- `attestations_verified` = `true` (Orthogonal dual attestation present)
- `failures` = `["state_continuity_mismatch"]`

---

## Invariant → byte mapping

| Invariant / capability | Where in this vector's bytes |
|---|---|
| Core §19 step 6.b state-continuity check on disclosure-profile axis | `from_disclosure_profile = "rl-profile-C"` vs initial declaration's `disclosure_profile = "rl-profile-A"`. |
| Companion Appendix A.5.2 CDDL (disclosure-profile transition) | Value map under `extensions["trellis.disclosure-profile-transition.v1"]`, including the `scope_change` field. |
| Companion Appendix A.5.3 step 4 Orthogonal dual-attestation rule | `scope_change = "Orthogonal"` with `attestations` length = 2 and classes `{"prior", "new"}`; step passes positively. |
| Invariant #11 — Respondent Ledger Profile A/B/C namespace-qualified | `from_disclosure_profile`/`to_disclosure_profile` use full `rl-profile-*` identifiers, never bare letters. |
| O-5 gate dual-axis verifier coverage | Together with `tamper/002`, proves the verifier handles BOTH custody-model AND disclosure-profile transitions symmetrically — the gap that caused the G-O-5 retroactive reopen. |

## Core-gap notes

None. The Rust fix that shipped alongside this vector mirrors spec semantics one-to-one; no Core or Companion prose change was needed.
