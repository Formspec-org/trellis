# Derivation — `append/025-erasure-evidence-per-tenant`

## Header

**What this vector exercises.** First/positive ADR 0005 cryptographic-
erasure-evidence vector under `subject_scope.kind = per-tenant` with
`cascade_scopes = [CS-01, CS-02, CS-03, CS-04, CS-05, CS-06]` and `completion_mode = "complete"`.
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
| `evidence_id` | `"urn:trellis:erasure:test:025"` | Operator-minted stable id; idempotent across retries. |
| `kid_destroyed` | `a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3` (16 bytes) | ADR 0005 §"Field semantics" — Phase-1 opaque-kid path; not in any committed signing-key registry, so the verifier skips registry-bind in step 2 but step 8 (chain consistency) still runs for `norm_key_class ∈ {"signing", "subject"}`. |
| `key_class` | `"subject"` | ADR 0006 `KeyEntry.kind` aligned. Wire `"subject"` is the normalized form (Core §8.7.6). |
| `destroyed_at` | `1745000100` | Unix seconds UTC; **strictly less than** host event `authored_at = 1745000200` (ADR 0005 step 4 / OC-144). |
| `cascade_scopes` | `[CS-01, CS-02, CS-03, CS-04, CS-05, CS-06]` | Companion Appendix A.7 enumeration. ADR 0005 step 1 requires non-empty; step 9 (deep cross-check against export contents) is Phase-2 best-effort. |
| `completion_mode` | `"complete"` | ADR 0005 §"Field semantics". |
| `destruction_actor` | `"urn:trellis:principal:test-operator"` | Opaque URI. |
| `policy_authority` | `"urn:trellis:authority:test-governance"` | Opaque URI. |
| `reason_code` | `4` | `operator-initiated-policy-change` per Companion §20.6.1 reason-code table. |
| `subject_scope` | `per-tenant` with `tenant_refs = ['urn:trellis:tenant:test-tenant-eu']`. | ADR 0005 step 3 (cross-field shape). |
| `attestations` | 2-element array (classes: ['prior', 'new']) | Companion §A.5 shared rule (see Step 3 below). |
| `extensions` | `null` | No extension fields. |

- `hsm_receipt = null` and `hsm_receipt_kind = null` (Phase-1 catch-all).

### Step 3 — Build attestations (Companion Appendix A.5 shared rule)

Each attestation signs `dCBOR([transition_id, effective_at, authority_class])`
under domain tag `trellis-transition-attestation-v1` (Core §9.8). For this
vector `transition_id = evidence_id = "urn:trellis:erasure:test:025"` and
`effective_at = destroyed_at = 1745000100`. Two attestations: one `authority_class="prior"` signed under `_keys/attestation-authority-cm-b-001.cose_key`, one `authority_class="new"` signed under `_keys/issuer-001.cose_key`. Companion OC-143 SHOULD-grade dual-attestation rule exercised structurally (`subject_scope.kind = per-tenant` triggers the SHOULD).

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
`idempotency_key = "idemp-append-025"`. Serialize under dCBOR; this
is `input-author-event-hash-preimage.cbor`.

`author_event_hash = SHA-256(domain_preimage("trellis-author-event-v1",
<authored bytes>)) =` **`bc887e8a9ad1788eb21393bdc246363b075f97f0d8a9304366a4e460ca96c9ec`** (32 bytes). Committed as
`author-event-hash.bin`.

### Step 6 — Build `EventPayload` and compute `canonical_event_hash` (§6.1, §9.2)

`EventPayload` is the authored-preimage shape plus `author_event_hash` from
Step 5. Serialize under dCBOR: `expected-event-payload.cbor`.

`canonical_event_hash =` **`1112f6507771b2046694ca4a914e87de6759b35bd8fda9222428b3f9ba913587`**.

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
| ADR 0005 step 3 (`subject_scope` cross-field shape) | The `subject_scope` map within that payload — `kind = per-tenant` with the matching ref array(s) per the table above. |
| ADR 0005 step 4 (`destroyed_at <= host authored_at` / OC-144) | `destroyed_at = 1745000100 <= 1745000200 = host authored_at`. |
| ADR 0005 step 6 (HSM null-consistency) | `hsm_receipt` and `hsm_receipt_kind` both null OR both non-null. |
| ADR 0005 step 7 (Phase-1 attestation structural) | Each attestation's `signature` is exactly 64 bytes; `authority_class` is one of `prior` / `new`. |
| Companion §20.6.1 reason-code (Core §6.9 family) | `reason_code = 4` (`operator-initiated-policy-change`); per-family namespace prevents cross-family collision. |
| Core §9.8 domain tag `trellis-transition-attestation-v1` | Used 2 time(s) — once per attestation. |

---

## Footer — summary digests

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| `author-event-hash.bin` | 32 | `0e722ebe8b59bb4fdfa5282ee9c531f60d796e7df2edb484c7cfb26c99e2c4aa` |
| `expected-append-head.cbor` | 93 | `b94490282a561f56c8d2911d59b0f628bea96d1095bc2b1d633ebc93b95e09ae` |
| `expected-event-payload.cbor` | 1376 | `68e1ea5de7efb156d070919262db0bcea59b5844474861c68dbbd74c4afd2654` |
| `expected-event.cbor` | 1477 | `f023b8ee8cae60b65be300f7d8ef37e54f7eec682c1b3ccea00c4254f5464dbe` |
| `input-attestation-preimage-new.cbor` | 40 | `add6a6f749ee2bd79d3d4f6019aa69e602fa5614c603266e5584953232df18bf` |
| `input-attestation-preimage-prior.cbor` | 42 | `bba332d29fe1943de1aecca47a6c1c7228feb235d277b6df527a43a34d6fc0f3` |
| `input-author-event-hash-preimage.cbor` | 1324 | `ef95f56a80ed93a3bb61b1b242f7e4aac1809bc693c2998c2f367af2e246cb66` |
| `input-prior-append-head.cbor` | 93 | `dc0fc83406bc87364b8beeebb4b8c867e68e9e5a025e24817c542f91da3772db` |

Pinned salient values (hex):

- `kid` = `af9dff525391faa75c8e8da4808b1743` (shared with `append/001`/`006`).
- `kid_destroyed` = `a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3a3` (Phase-1 opaque-kid path; not in any registry).
- `prev_hash` = `ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb`.
- `destroyed_at` = `1745000100`.
- `host authored_at` = `1745000200`.
- `author_event_hash` = `bc887e8a9ad1788eb21393bdc246363b075f97f0d8a9304366a4e460ca96c9ec`.
- `canonical_event_hash` = `1112f6507771b2046694ca4a914e87de6759b35bd8fda9222428b3f9ba913587`.
