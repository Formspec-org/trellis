# Derivation — `tamper/002-transition-from-mismatch`

## Header

**What this vector exercises.** Core §19 step 6.b — the state-continuity check for posture-transition events. A custody-model transition event advertises `from_custody_model = "CM-C"`, but the deployment's initial posture declaration pins `custody_model = "CM-B"`. A conforming verifier builds a shadow state from the initial declaration, observes the mismatch at step 6.b, accumulates `continuity_verified = false` for this transition's `PostureTransitionOutcome`, and drops `integrity_verified` to `false` via the step-9 conjunction.

Unlike `tamper/001-signature-flip`, which mutates a single signature byte leaving the payload untouched, this tamper lives *inside* the event payload: `from_custody_model` is the mutated field. Consequently the event is re-signed end-to-end — the cryptographic surfaces (signature, author_event_hash, canonical_event_hash) are all valid for the tampered bytes. §19 step 4.b's signature check passes; §19 step 6.b is the failure site.

**Scope.** Tamper of a single custody-model transition event. No export manifest, no checkpoint. The verifier-side surface exercised is: §19 step 4.a (kid resolution) + step 4.b (signature verify) + step 6.a (schema decode) + step 6.b (state continuity). Steps 6.c (declaration-digest resolve) and 6.d (attestation verify) pass on this vector — only 6.b fails — so the report's `PostureTransitionOutcome` has `declaration_resolved = true, attestations_verified = true, continuity_verified = false`.

**Core / Companion § roadmap.**

1. Core §6.7 — extension registry entry for `trellis.custody-model-transition.v1`.
2. Core §19 step 6.b — state-continuity rule.
3. Companion §11 / Appendix A.1 — the initial posture declaration shape (the verifier's shadow-state baseline).
4. Companion §10.3 field 1 (`transition_id`) — stable within `ledger_scope`.
5. Core §9.1 / §9.8 — domain-separation discipline (identical to `append/006`).
6. Core §7.1 / §7.4 — COSE_Sign1 envelope (same issuer key as `append/006`).

---

## Body

### Step 1 — Commit the initial posture declaration (Companion §11, Appendix A.1)

The verifier's shadow-state baseline at §19 step 6.b "from_* state equals ... the deployment's initial declaration if no prior transition exists." The initial declaration for this fixture pins `custody_model = "CM-B"`. Committed as `input-initial-posture-declaration.bin` (301 B). A conforming verifier walking this fixture's chain treats this as the authoritative prior-state source (equivalent to reading it from the export manifest's `posture_declaration` slot in a full §18 export).

### Step 2 — Commit the post-transition declaration and compute its digest

The event's `declaration_doc_digest` resolves cleanly against `input-posture-declaration.bin` (354 B; pins `custody_model = "CM-A"` = the `to_custody_model` value, consistent with the co-publish rule). Step 6.c passes on this vector — the declaration bytes are the ones the event claims they are. The tamper is in the event's `from_custody_model`, not in the declaration.

`declaration_doc_digest` = `SHA-256(domain_preimage("trellis-posture-declaration-v1", <post declaration bytes>))` = (pinned in the generator output; field value inside the tampered event payload).

### Step 3 — Build attestations (both classes present)

The widening claim (CM-? → CM-A) would require dual attestation per OC-11 if the claimed `from_*` were honest. The vector emits both `authority_class` signatures so `attestations_verified = true`; step 6.d passes. This isolates the failure to step 6.b.

### Step 4 — Assemble the tampered transition payload (Companion Appendix A.5.1)

| Field | Value |
|---|---|
| `transition_id` | `"urn:trellis:transition:test:tamper-002"` |
| `from_custody_model` | `"CM-C"` **← TAMPER** (initial declaration pins `CM-B`) |
| `to_custody_model` | `"CM-A"` |
| `effective_at` | `1745000400` |
| `reason_code` | `3` (operator-boundary-change) |
| `declaration_doc_digest` | SHA-256 per Step 2 — 32 bytes. |
| `transition_actor` | `"urn:trellis:principal:test-operator"` |
| `policy_authority` | `"urn:trellis:authority:test-governance"` |
| `temporal_scope` | `"prospective"` |
| `attestations` | 2-element array (prior + new). |
| `extensions` | `null` |

### Step 5 — Re-sign the event end-to-end

The event envelope is built exactly as in `append/006`: `AuthorEventHashPreimage` → `author_event_hash` (§9.5) → `EventPayload` → `CanonicalEventHashPreimage` → `canonical_event_hash` (§9.2) → protected header + `Sig_structure` → Ed25519 over the issuer-001 key → tag-18 COSE_Sign1. Because the payload bytes differ from 006 (the `from_custody_model` value and the attestation signatures are different), every downstream hash is different too.

`canonical_event_hash` of the tampered event = **`e024a548dfc0a00a9af2b62124ab04d184498f2795fa613b6ed854f0766c83d1`**. This is the `failing_event_id` pinned in the manifest — §19 populates `VerificationFailure.location` (and by extension the transition outcome's event identity) with the tampered event's own canonical hash.

### Step 6 — Wrap the event in a one-element ledger + ship the signing-key registry

Identical convention to `tamper/001`. `input-tampered-ledger.cbor` is `dCBOR([<tag-18 event>])` (1380 B). `input-signing-key-registry.cbor` carries the single `SigningKeyEntry` for `issuer-001` so step 4.a resolves the kid (133 B, byte-identical to `tamper/001`'s — same issuer).

### Step 7 — Pin `[expected.report]`

A conforming verifier walking this input produces:

| Field | Value | Reasoning |
|---|---|---|
| `structure_verified` | `true` | The tampered envelope decodes cleanly through §19 step 4.c, the transition payload decodes cleanly through step 6.a. |
| `integrity_verified` | `false` | Step 9's conjunction sees `continuity_verified = false` in `report.posture_transitions[0]`. |
| `readability_verified` | `true` | No payload decryption attempted; `PayloadInline` marker bytes are opaque. |
| `tamper_kind` | `"state_continuity_mismatch"` | tamper/001's tamper-kind enum, row `state_continuity_mismatch`. |
| `failing_event_id` | `e024a548…766c83d1` | Tampered event's `canonical_event_hash` per Step 5. |

The `PostureTransitionOutcome` emitted by step 6.e carries:

| Field | Value |
|---|---|
| `transition_id` | `"urn:trellis:transition:test:tamper-002"` |
| `kind` | `"custody-model"` |
| `from_state` | `"CM-C"` (as claimed by the event) |
| `to_state` | `"CM-A"` |
| `continuity_verified` | `false` |
| `declaration_resolved` | `true` (step 6.c passes) |
| `attestations_verified` | `true` (step 6.d passes) |
| `failures` | `["state_continuity_mismatch"]` |

---

## Invariant → byte mapping

| Invariant / capability | Where in this vector's bytes |
|---|---|
| Core §19 step 6.b state-continuity rule | Mismatch between `input-initial-posture-declaration.bin` (pins `CM-B`) and the `from_custody_model` value inside `input-tampered-event.cbor`'s `EventPayload.extensions["trellis.custody-model-transition.v1"]` (`"CM-C"`). |
| §19 step 9 `integrity_verified` conjunction | Drops to `false` via "no entry in report.posture_transitions has continuity_verified = false" clause. |
| §19 step 4.b signature verify passes | The event is re-signed with issuer-001; the Sig_structure covers the tampered payload bytes. |
| §19 step 6.c declaration-digest passes | `declaration_doc_digest` matches `input-posture-declaration.bin` under the `trellis-posture-declaration-v1` tag. |

## Core-gap notes

No Core gap. Step 6.b is unambiguous — the verifier MUST track a shadow state and check `from_*` against it. The initial-declaration baseline is named explicitly in §19 step 6.b ("equals the deployment's initial declaration if no prior transition exists").
