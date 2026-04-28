# Derivation — `append/023-erasure-evidence-per-subject-cs-03`

## Header

**What this vector exercises.** First/positive ADR 0005 cryptographic-
erasure-evidence vector under `subject_scope.kind = per-subject` with
`cascade_scopes = [CS-03]` and `completion_mode = "complete"`.
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
| `evidence_id` | `"urn:trellis:erasure:test:023"` | Operator-minted stable id; idempotent across retries. |
| `kid_destroyed` | `a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1` (16 bytes) | ADR 0005 §"Field semantics" — Phase-1 opaque-kid path; not in any committed signing-key registry, so the verifier skips registry-bind in step 2 but step 8 (chain consistency) still runs for `norm_key_class ∈ {"signing", "subject"}`. |
| `key_class` | `"subject"` | ADR 0006 `KeyEntry.kind` aligned. Wire `"subject"` is the normalized form (Core §8.7.6). |
| `destroyed_at` | `1745000100` | Unix seconds UTC; **strictly less than** host event `authored_at = 1745000200` (ADR 0005 step 4 / OC-144). |
| `cascade_scopes` | `[CS-03]` | Companion Appendix A.7 enumeration. ADR 0005 step 1 requires non-empty; step 9 (deep cross-check against export contents) is Phase-2 best-effort. |
| `completion_mode` | `"complete"` | ADR 0005 §"Field semantics". |
| `destruction_actor` | `"urn:trellis:principal:test-operator"` | Opaque URI. |
| `policy_authority` | `"urn:trellis:authority:test-governance"` | Opaque URI. |
| `reason_code` | `1` | `retention-expired` per Companion §20.6.1 reason-code table. |
| `subject_scope` | `per-subject` with `subject_refs = ['urn:trellis:subject:test-applicant-001']`. | ADR 0005 step 3 (cross-field shape). |
| `attestations` | 1-element array (classes: ['new']) | Companion §A.5 shared rule (see Step 3 below). |
| `extensions` | `null` | No extension fields. |

- `hsm_receipt = null` and `hsm_receipt_kind = null` (Phase-1 catch-all).

### Step 3 — Build attestations (Companion Appendix A.5 shared rule)

Each attestation signs `dCBOR([transition_id, effective_at, authority_class])`
under domain tag `trellis-transition-attestation-v1` (Core §9.8). For this
vector `transition_id = evidence_id = "urn:trellis:erasure:test:023"` and
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
`idempotency_key = "idemp-append-023"`. Serialize under dCBOR; this
is `input-author-event-hash-preimage.cbor`.

`author_event_hash = SHA-256(domain_preimage("trellis-author-event-v1",
<authored bytes>)) =` **`f1c01c3f06a78d393c2df367ccfcb257aa310caaf787e9d2091e04eba5664042`** (32 bytes). Committed as
`author-event-hash.bin`.

### Step 6 — Build `EventPayload` and compute `canonical_event_hash` (§6.1, §9.2)

`EventPayload` is the authored-preimage shape plus `author_event_hash` from
Step 5. Serialize under dCBOR: `expected-event-payload.cbor`.

`canonical_event_hash =` **`e18e146824d31cfdc9ec7fca98573ba7a4325739d985f9eeb59e32fd569f10c9`**.

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
| `author-event-hash.bin` | 32 | `ebbba9787fdd3f5fb683d9eb490b17984d67e90f31c64e61413fba9fe57b4509` |
| `expected-append-head.cbor` | 93 | `d4abd134aa641847c480fed6103b869f19ba24db9a6ff872985e89afb9ed72b2` |
| `expected-event-payload.cbor` | 1200 | `adc792e707639aa86098dcabdf16b765b2572799beba410ad0a3aa0f94f83eae` |
| `expected-event.cbor` | 1301 | `0ca51508cb057efbae98a74031223aa251a393becba0a404932b6a4d3d931b1f` |
| `input-attestation-preimage-new.cbor` | 40 | `831b60e35c42e9e507c256f4bc79b71f1e3642f13f524dc3ef9892b81dd652cf` |
| `input-author-event-hash-preimage.cbor` | 1148 | `13e07bc56f8208b8822d1cbe6f63802377fb4d7216ed980397e7deafd7e11a5c` |
| `input-prior-append-head.cbor` | 93 | `dc0fc83406bc87364b8beeebb4b8c867e68e9e5a025e24817c542f91da3772db` |

Pinned salient values (hex):

- `kid` = `af9dff525391faa75c8e8da4808b1743` (shared with `append/001`/`006`).
- `kid_destroyed` = `a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1a1` (Phase-1 opaque-kid path; not in any registry).
- `prev_hash` = `ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb`.
- `destroyed_at` = `1745000100`.
- `host authored_at` = `1745000200`.
- `author_event_hash` = `f1c01c3f06a78d393c2df367ccfcb257aa310caaf787e9d2091e04eba5664042`.
- `canonical_event_hash` = `e18e146824d31cfdc9ec7fca98573ba7a4325739d985f9eeb59e32fd569f10c9`.
