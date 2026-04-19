# Derivation — `append/008-disclosure-profile-transition-a-to-b`

## Header

**What this vector exercises.** Third Stream D / O-5 Posture-transition vector. Pins the byte shape of a `trellis.disclosure-profile-transition.v1` event (Core §6.7 extension registry entry; Companion Appendix A.5.2). The transition moves the Respondent Ledger Profile A/B/C axis from `rl-profile-A` to `rl-profile-B` under `scope_change = "Orthogonal"` — an axis swap (e.g., stricter privacy in exchange for weaker identity binding) rather than a strict narrowing or widening. Appendix A.5.3 step 4 classifies `Orthogonal` as the non-narrowing default; dual attestation is required.

The vector is structurally parallel to `append/006-custody-transition-cm-b-to-cm-a`: same ledger scope, same chain root (`append/001`), same dual-attestation discipline. The deltas versus 006 are (a) the event_type and extension-key string (`trellis.disclosure-profile-transition.v1` instead of `trellis.custody-model-transition.v1`), (b) the transition payload CDDL (`from_disclosure_profile`/`to_disclosure_profile`/`scope_change` in place of `from_custody_model`/`to_custody_model`), (c) the transition identifiers, and (d) `idempotency_key = idemp-append-008`.

**Scope.** Fact-producer side only. This vector covers invariant #11's disclosure-profile axis of the profile-namespacing rule (OC-06: `rl-profile-A/B/C` are namespace-qualified identifiers, never bare letters).

**Core / Companion § roadmap.** Identical to 006 with Appendix A.5.2 substituted for A.5.1 in Step 4.

---

## Body

### Step 1 — Prior head

Same chain root and `prev_hash` as 006/007: `ef2622f1…724ddb` (= `append/001`'s `canonical_event_hash`).

### Step 2 — Post-transition PostureDeclaration

Declaration in force AFTER the transition sets `disclosure_profile = "rl-profile-B"` (and keeps `custody_model = CM-B` — the custody model is not affected by this transition). Committed as `input-posture-declaration.bin` (320 B).

`declaration_doc_digest = SHA-256(domain_preimage("trellis-posture-declaration-v1", <declaration bytes>)) = 1ee1374cf6ea096782dcd33e3c77bd5ec39c0aa7b0c2d3f005f618cec9f8813e`.

### Step 3 — Dual attestation (Appendix A.5.3 step 4 Orthogonal branch)

Companion Appendix A.5.3 step 4:

> "`scope_change = "Orthogonal"` — MUST be dually attested. `Orthogonal` is the non-narrowing default and does not qualify for the reduced-attestation carve-out."

Both authorities therefore sign. The `authority_class = "prior"` signer represents the prior `rl-profile-A` authority and uses `_keys/attestation-authority-cm-b-001.cose_key` (reused across O-5 fixtures as the generic "prior authority" key — the specific role identifier in the URI string is what names which axis the attestation covers). The `authority_class = "new"` signer represents the incoming `rl-profile-B` authority and uses `_keys/issuer-001.cose_key`.

| authority_class | preimage | file |
|---|---|---|
| `prior` | `dCBOR(["urn:trellis:transition:test:008", 1745000300, "prior"])` | `input-attestation-preimage-prior.cbor` (45 B) |
| `new`   | `dCBOR(["urn:trellis:transition:test:008", 1745000300, "new"])`   | `input-attestation-preimage-new.cbor`   (43 B) |

Each preimage is domain-separated under `trellis-transition-attestation-v1` and Ed25519-signed.

### Step 4 — `DisclosureProfileTransitionPayload` (Appendix A.5.2)

| Field | Value |
|---|---|
| `transition_id` | `"urn:trellis:transition:test:008"` |
| `from_disclosure_profile` | `"rl-profile-A"` |
| `to_disclosure_profile` | `"rl-profile-B"` |
| `effective_at` | `1745000300` |
| `reason_code` | `4` (governance-policy-change) |
| `declaration_doc_digest` | Step 2 output — 32 bytes. |
| `scope_change` | `"Orthogonal"` |
| `transition_actor` | `"urn:trellis:principal:test-operator"` |
| `policy_authority` | `"urn:trellis:authority:test-governance"` |
| `temporal_scope` | `"prospective"` |
| `attestations` | 2-element array (Step 3). |
| `extensions` | `null` |

### Step 5 — `EventPayload.extensions` key

The payload rides under the key `"trellis.disclosure-profile-transition.v1"` — the Core §6.7 registered event_type identifier. `EventHeader.event_type` is the UTF-8 encoding of that same string. No other `EventPayload` field differs from the 006/007 shape.

### Steps 6–9 — Event envelope, COSE_Sign1, AppendHead

Identical discipline to 006/007. Final outputs:

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| `input-prior-append-head.cbor` | 93 | `dc0fc83406bc87364b8beeebb4b8c867e68e9e5a025e24817c542f91da3772db` |
| `input-posture-declaration.bin` | 320 | `b62aeac63848233e6646b1038fc634621c79b22e2e155324c1fc2bd921257dc3` |
| `input-attestation-preimage-prior.cbor` | 45 | `7801c6d53c6c1b71dc0bf37f63072f85b7a74a5e325df665395c274da3b5eab5` |
| `input-attestation-preimage-new.cbor` | 43 | `fc3edc189e33d5be2e5f2a05811fa336fdc48fb8f8dcc7be53002c818e3df10d` |
| `input-author-event-hash-preimage.cbor` | 1302 | `6d2fb7589a4a54cf2181f84651c4280f3fbe44048a4e4ca24f2f3f7848d18ced` |
| `author-event-hash.bin` | 32 | `39e2ad4a563dee546943ecc3956d8c8f28430d102cbb047313677fabf0cd789a` |
| `expected-event-payload.cbor` | 1354 | `2e949b75982a61d5aea85cec23ce7b228c450ea02f6421d931fa5bb2e36c57a6` |
| `expected-event.cbor` | 1455 | `5cb665465604c7d11e5e90d90a90276975c404814cf544553789c10030262e37` |
| `expected-append-head.cbor` | 93 | `d63a97b607879915edb072954b8d74396a6c4688ecd1eccf78de8681efd9cf8e` |

Pinned salient values:

- `kid` = `af9dff525391faa75c8e8da4808b1743`.
- `prev_hash` = `ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb`.
- `declaration_doc_digest` = `1ee1374cf6ea096782dcd33e3c77bd5ec39c0aa7b0c2d3f005f618cec9f8813e`.
- `author_event_hash` = `a521259205c4cd6fb8c7a2373d1d74278fadee80f2955debc5eab11bececf8e8`.
- `canonical_event_hash` = `cd86e6861316583a50f31cf8f7172e2e62206a3b50ad828a169a60ec8a429011`.

---

## Invariant → byte mapping

| Invariant / capability | Where in this vector's bytes |
|---|---|
| Core §6.7 registered extension `trellis.disclosure-profile-transition.v1` | `EventPayload.extensions` key + `EventHeader.event_type` value. |
| Companion Appendix A.5.2 CDDL | The value map under that key, including the `scope_change = "Orthogonal"` field that distinguishes this CDDL from A.5.1. |
| Invariant #11 — Respondent Ledger Profile A/B/C namespace-qualified | `from_disclosure_profile = "rl-profile-A"` and `to_disclosure_profile = "rl-profile-B"`; bare `A` / `B` would be non-conformant under OC-06. |
| Appendix A.5.3 step 4 Orthogonal dual-attestation rule | `attestations` length = 2 with classes `{"prior", "new"}`. |

## Core-gap notes

None new beyond the PostureDeclaration-interior-CDDL note flagged in 006.
