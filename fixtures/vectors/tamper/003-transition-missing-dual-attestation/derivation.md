# Derivation — `tamper/003-transition-missing-dual-attestation`

## Header

**What this vector exercises.** Companion §10.4 (OC-11) / Appendix A.5.3 step 4 — the dual-attestation rule for Posture-widening transitions — enforced via Core §19 step 6.d's attestation-count + signature-verify check. The tampered event advertises a widening custody-model transition (CM-B → CM-A: the provider-readable decryptor surface expands). OC-11 mandates dual attestation for Posture-widening transitions. The event ships only the `authority_class = "new"` signature; no `"prior"` attestation is present. A conforming verifier's step 6.d check records `attestations_verified = false` and appends `attestation_insufficient` to the outcome's failures list. §19 step 9's integrity conjunction drops to `false` via the "no entry in report.posture_transitions has attestations_verified = false" clause.

Unlike `tamper/002`, `from_custody_model = "CM-B"` matches the initial declaration — step 6.b passes. Unlike `tamper/004`, the declaration digest resolves correctly — step 6.c passes. The failure is isolated to step 6.d.

**Scope.** Identical surface to `tamper/002` except that (a) the tamper is in the *attestation count*, not the `from_custody_model` value, and (b) no prior-authority key is needed since the prior attestation is absent from the event.

**Core / Companion § roadmap.**

1. Core §6.7 / Companion Appendix A.5.1 — event shape.
2. Companion §10.4 (OC-11) — widening requires dual attestation.
3. Companion Appendix A.5.3 step 4 — attestation-count rule's realization.
4. Core §19 step 6.d — the verifier check site.

---

## Body

### Step 1 — Initial posture declaration (Companion §11, Appendix A.1)

Pins `custody_model = "CM-B"`. Committed as `input-initial-posture-declaration.bin` (301 B). The event's `from_custody_model = "CM-B"` matches this baseline, so step 6.b passes.

### Step 2 — Post-transition posture declaration

Pins `custody_model = "CM-A"` (matches the event's `to_custody_model`). Committed as `input-posture-declaration.bin` (354 B). Declaration digest under `trellis-posture-declaration-v1` matches `declaration_doc_digest`, so step 6.c passes.

### Step 3 — Single (insufficient) attestation

Companion §10.4 (OC-11):

> "Posture-expanding transitions — transitions that expand the access or disclosure surface along any Posture axis, including but not limited to any Custody-Model transition from a reader-held model to a provider-readable model — MUST be dually attested by both authorities where both exist; a unilateral expansion by the party gaining access is NON-CONFORMANT."

CM-A is provider-readable; CM-B is reader-held. CM-B → CM-A is therefore the canonical OC-11 example of a widening transition. The event SHOULD carry both a `"prior"` and a `"new"` attestation. This vector ships only the `"new"` attestation (signed with `_keys/issuer-001.cose_key`).

### Step 4 — Assemble the tampered transition payload (Appendix A.5.1)

| Field | Value |
|---|---|
| `transition_id` | `"urn:trellis:transition:test:tamper-003"` |
| `from_custody_model` | `"CM-B"` (honest; matches initial) |
| `to_custody_model` | `"CM-A"` (widening) |
| `effective_at` | `1745000500` |
| `reason_code` | `3` |
| `declaration_doc_digest` | SHA-256 per Step 2. |
| `transition_actor` | `"urn:trellis:principal:test-operator"` |
| `policy_authority` | `"urn:trellis:authority:test-governance"` |
| `temporal_scope` | `"prospective"` |
| `attestations` | **1-element array** — `[new]` only. **← TAMPER**. |
| `extensions` | `null` |

### Step 5 — Re-sign the event end-to-end

Identical discipline to `append/006` / `tamper/002`. `canonical_event_hash` of the tampered event = **`a14060ff664e4c229c1e832e78c5072f430a709005e4c38821f94a976b2eb4c0`**.

### Step 6 — Ledger + signing-key registry

`input-tampered-ledger.cbor` = `dCBOR([<tag-18 event>])` (1228 B). `input-signing-key-registry.cbor` carries the `issuer-001` entry (133 B, byte-identical to `tamper/001` and `tamper/002`).

### Step 7 — Pin `[expected.report]`

| Field | Value | Reasoning |
|---|---|---|
| `structure_verified` | `true` | Event decodes cleanly; step 4.c + step 6.a pass. |
| `integrity_verified` | `false` | Step 9: `report.posture_transitions[0].attestations_verified = false`. |
| `readability_verified` | `true` | No payload decryption attempted. |
| `tamper_kind` | `"attestation_insufficient"` | tamper/001's tamper-kind enum. |
| `failing_event_id` | `a14060ff…b2eb4c0` | Tampered event's `canonical_event_hash`. |

The `PostureTransitionOutcome` carries: `continuity_verified = true`, `declaration_resolved = true`, `attestations_verified = false`, `failures = ["attestation_insufficient"]`.

---

## Invariant → byte mapping

| Invariant / capability | Where in this vector's bytes |
|---|---|
| OC-11 / §10.4 widening dual-attestation rule | `attestations` array inside `input-tampered-event.cbor`'s `EventPayload.extensions["trellis.custody-model-transition.v1"]` has length 1. |
| Core §19 step 6.d attestation-count + signature check | A conforming verifier computes the required attestation set from the Posture-widening direction (`from = CM-B`, `to = CM-A`) and observes `len(attestations) = 1 < 2`. |
| §19 step 9 `integrity_verified` conjunction | Drops to false via the "no entry has attestations_verified = false" clause. |

## Core-gap notes

None. OC-11 is unambiguous; §19 step 6.d names the check site explicitly.
