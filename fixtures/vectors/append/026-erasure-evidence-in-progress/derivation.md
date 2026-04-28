# Derivation — `append/026-erasure-evidence-in-progress`

## Header

**What this vector exercises.** First/positive ADR 0005 cryptographic-
erasure-evidence vector under `subject_scope.kind = per-subject` with
`cascade_scopes = [CS-03]` and `completion_mode = "in-progress"`.
The event carries a `trellis.erasure-evidence.v1` extension payload per
Core §6.7 (Extension Registry) and Companion §20.6.2 (wire shape — byte-
authoritative reference is ADR 0005 §"Wire shape"). The verifier's
obligations are pinned by the 10-step checklist in Core §19 step 6b
(anchored on ADR 0005 §"Verifier obligations"); this vector's bytes are
the positive corpus that step's per-step decoders run against.

**Scope of this vector.**

1. Fact-producer side only (`append` op). Tamper-side coverage of step 8
   chain-walk lives in `tamper/017` (post_erasure_use) and `tamper/018`
   (post_erasure_wrap).
2. Single non-genesis event. Chains from `append/001-minimal-inline-payload`
   with `sequence = 1`. The erasure payload rides inside
   `EventPayload.extensions` per Core §6.5 strict-superset semantics; the
   surrounding event envelope is identical in shape to 001's and 006's.
3. No export manifest, no checkpoint, no inclusion proof. Step 9 (cascade
   cross-check against export contents) is best-effort in Phase 1 and
   rides O-3 evolution (ADR 0005 step 9).

**Core / Companion / ADR § roadmap (in traversal order).**

1. ADR 0005 §"Wire shape" — the `ErasureEvidencePayload` CDDL.
2. Core §6.7 — registers `trellis.erasure-evidence.v1` under
   `EventPayload.extensions`.
3. Companion §20.6 — Documentation and Evidence; OC-78 promotion +
   §20.6.1 reason-code table + §20.6.2 schema-conformance + §20.6.3
   OC-141..146 obligations.
4. Companion Appendix A.5 — shared `Attestation` rule reused under
   `trellis-transition-attestation-v1` (Core §9.8).
5. Core §19 step 6b — verifier 10-step checklist anchored on ADR 0005.
6. Core §6.1 / §6.8 — three event surfaces (authored / canonical / signed).
7. Core §9.5 / §9.2 — author_event_hash + canonical_event_hash.
8. Core §7.1 / §7.4 — COSE_Sign1 envelope.
9. Core §10.6 — AppendHead post-append state.

---

## Body

### Step 1 — Resolve the prior head (Core §10.2, §10.6)

Same as `append/006`: chain from `append/001-minimal-inline-payload` head.
`prev_hash = ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb`.

### Step 2 — Build the erasure-evidence payload (ADR 0005 §"Wire shape")

Field-by-field against the ADR 0005 CDDL:

| Field | Value | Source |
|---|---|---|
| `evidence_id` | `"urn:trellis:erasure:test:026"` | Operator-minted stable id; idempotent across retries. |
| `kid_destroyed` | `a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4` (16 bytes) | ADR 0005 §"Field semantics" — Phase-1 opaque-kid path; not in any committed signing-key registry, so the verifier skips registry-bind in step 2 but step 8 (chain consistency) still runs for `norm_key_class ∈ {"signing", "subject"}`. |
| `key_class` | `"subject"` | ADR 0006 `KeyEntry.kind` aligned. Wire `"subject"` is the normalized form (Core §8.7.6). |
| `destroyed_at` | `1745000100` | Unix seconds UTC; **strictly less than** host event `authored_at = 1745000200` (ADR 0005 step 4 / OC-144). |
| `cascade_scopes` | `[CS-03]` | Companion Appendix A.7 enumeration. ADR 0005 step 1 requires non-empty; step 9 (deep cross-check against export contents) is Phase-2 best-effort. |
| `completion_mode` | `"in-progress"` | ADR 0005 §"Field semantics". |
| `destruction_actor` | `"urn:trellis:principal:test-operator"` | Opaque URI. |
| `policy_authority` | `"urn:trellis:authority:test-governance"` | Opaque URI. |
| `reason_code` | `1` | `retention-expired` per Companion §20.6.1 reason-code table. |
| `subject_scope` | `per-subject` with `subject_refs = ['urn:trellis:subject:test-applicant-003']`. | ADR 0005 step 3 (cross-field shape). |
| `attestations` | 1-element array (classes: ['new']) | Companion §A.5 shared rule (see Step 3 below). |
| `extensions` | `null` | No extension fields. |

- `hsm_receipt = null` and `hsm_receipt_kind = null` (Phase-1 catch-all).

### Step 3 — Build attestations (Companion Appendix A.5 shared rule)

Each attestation signs `dCBOR([transition_id, effective_at, authority_class])`
under domain tag `trellis-transition-attestation-v1` (Core §9.8). For this
vector `transition_id = evidence_id = "urn:trellis:erasure:test:026"` and
`effective_at = destroyed_at = 1745000100`. Single `authority_class="new"` attestation signed under `_keys/issuer-001.cose_key`. Phase-1 lower bound; per-deployment SHOULD-grade dual rule (OC-143) does not apply to this scope/reason.

### Step 4 — Nest the payload into `EventPayload.extensions` (Core §6.5, §6.7)

Per Core §6.5 strict-superset semantics, the registered extension is keyed
by its event-type string. The erasure payload therefore rides under the
key `"trellis.erasure-evidence.v1"` in the enclosing `EventPayload.extensions`
map. `EventHeader.event_type` is also set to the UTF-8 encoding of that
same identifier.

`EventPayload.header.classification = "x-trellis-test/unclassified"`
(outcome-neutral per Core §12.4). `EventPayload.payload_ref` carries a
13-byte marker ciphertext `"erasure-event"` under `ref_type="inline"`;
§6.1 requires `payload_ref` for every event, and the erasure event's
semantic content is entirely in `extensions`, so the marker is narrative
only (its `content_hash` still participates in canonical-event-hash
construction via §9.3).

### Step 5 — Compute `author_event_hash` (§9.5)

Build `AuthorEventHashPreimage` (13 fields; see Core §28 Appendix A) with
`idempotency_key = "idemp-append-026"`. Serialize under dCBOR; this
is `input-author-event-hash-preimage.cbor`.

`author_event_hash = SHA-256(domain_preimage("trellis-author-event-v1",
<authored bytes>)) =` **`ca717e9339e2880a7eb7fcc44a901b5b9ded6ddcfb848c704596328daa2f389c`** (32 bytes). Committed as
`author-event-hash.bin`.

### Step 6 — Build `EventPayload` and compute `canonical_event_hash` (§6.1, §9.2)

`EventPayload` is the authored-preimage shape plus `author_event_hash` from
Step 5. Serialize under dCBOR: `expected-event-payload.cbor`.

`canonical_event_hash =` **`24ebece895df94d8c14897edc2890fd111dae4fafb760f8d0030db72c9711259`**.

### Step 7 — COSE_Sign1 envelope (§7.1, §7.4)

Protected header `{1: -8, 4: <kid>, -65537: 1}` (alg = EdDSA; kid =
`af9dff525391faa75c8e8da4808b1743` — same as `append/001` and `append/006`,
shared issuer key). `Sig_structure = dCBOR(["Signature1", <protected_bstr>,
h'', <payload_bstr>])` per RFC 9052 §4.4. Ed25519-sign with the issuer-001
seed; signature is 64 bytes. Final tag-18 envelope is committed as
`expected-event.cbor`.

### Step 8 — `AppendHead` (§10.6)

`AppendHead = {scope, sequence, canonical_event_hash}`. Committed as
`expected-append-head.cbor`.

---

## Invariant → byte mapping

| Invariant / capability | Where in this vector's bytes |
|---|---|
| ADR 0005 wire-shape CDDL (`ErasureEvidencePayload`) | The value map under `EventPayload.extensions["trellis.erasure-evidence.v1"]` in `expected-event-payload.cbor`. |
| ADR 0005 step 3 (`subject_scope` cross-field shape) | The `subject_scope` map within that payload — `kind = per-subject` with the matching ref array(s) per the table above. |
| ADR 0005 step 4 (`destroyed_at <= host authored_at` / OC-144) | `destroyed_at = 1745000100 <= 1745000200 = host authored_at`. |
| ADR 0005 step 6 (HSM null-consistency) | `hsm_receipt` and `hsm_receipt_kind` both null OR both non-null. |
| ADR 0005 step 7 (Phase-1 attestation structural) | Each attestation's `signature` is exactly 64 bytes; `authority_class` is one of `prior` / `new`. |
| Companion §20.6.1 reason-code (Core §6.9 family) | `reason_code = 1` (`retention-expired`); per-family namespace prevents cross-family collision. |
| Core §9.8 domain tag `trellis-transition-attestation-v1` | Used 1 time(s) — once per attestation. |

---

## Footer — summary digests

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| `author-event-hash.bin` | 32 | `aaa3707a98bab772d90bc956653a45c3366b04ca43cd5a37c3c39a2d5be9cb47` |
| `expected-append-head.cbor` | 93 | `e7eb2eec633d32b0cfd64f7d719f87ee3df01ad9b19dc57192780fc68772c432` |
| `expected-event-payload.cbor` | 1203 | `01c0229d1701a40ac58c1027fb5ea3e321ae3700874562df28a809ae54c6a533` |
| `expected-event.cbor` | 1304 | `a96517af7287de2d858d58ddf2456ce55ec79122fb9b7d9bcd306dd602c2697b` |
| `input-attestation-preimage-new.cbor` | 40 | `89b2d7b1c4d39d8a339f87543c444b5cc27c57c100bb5a45de18f1fba044b87e` |
| `input-author-event-hash-preimage.cbor` | 1151 | `99e0a948906e7d5560ea5ca899fa3a30988892ad2c4c5dc824c879ca9de19bbd` |
| `input-prior-append-head.cbor` | 93 | `dc0fc83406bc87364b8beeebb4b8c867e68e9e5a025e24817c542f91da3772db` |

Pinned salient values (hex):

- `kid` = `af9dff525391faa75c8e8da4808b1743` (shared with `append/001`/`006`).
- `kid_destroyed` = `a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4a4` (Phase-1 opaque-kid path; not in any registry).
- `prev_hash` = `ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb`.
- `destroyed_at` = `1745000100`.
- `host authored_at` = `1745000200`.
- `author_event_hash` = `ca717e9339e2880a7eb7fcc44a901b5b9ded6ddcfb848c704596328daa2f389c`.
- `canonical_event_hash` = `24ebece895df94d8c14897edc2890fd111dae4fafb760f8d0030db72c9711259`.
