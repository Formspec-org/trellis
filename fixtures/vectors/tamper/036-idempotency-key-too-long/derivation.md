# Derivation — `tamper/036-idempotency-key-too-long`

## Header

**What this vector exercises.** Core §6.1 + §17.2 pin `idempotency_key` to `bstr .size (1..64)` (CDDL grammar in §28). This vector commits a single-event ledger whose authored event has a 65-byte `idempotency_key` — exactly one byte over the upper bound. The signature over the EventPayload is valid; the chain is trivially intact (single event, sequence=0, prev_hash=null); the canonical hash recomputation matches the stored value. The **only** violation is the §6.1 / §17.2 structural size bound. The verifier MUST surface `tamper_kind = "idempotency_key_length_invalid"` (§17.5) and flip `structure_verified = false`.

**Why structural-reject, not §17.3 conflict.** The §6.1 length bound is a CDDL grammar obligation; an out-of-bound key fails CDDL conformance before any §17.3 retry-conflict resolution runs. Per the spec prose addition in §17.2 ("a structural-reject path orthogonal to the §17.3 retry-vs-conflict resolution"), the two paths do not overlap: an empty key OR a 65+ byte key is rejected with `idempotency_key_length_invalid` regardless of whether any in-scope twin exists.

**Pinned inputs.**

| Input | Value |
|---|---|
| `signing_key` | `_keys/issuer-001.cose_key` (Ed25519, suite_id=1) |
| `ledger_scope` | bstr `"test-response-ledger"` |
| `idempotency_key` | bstr `b"x" * 65` (65 bytes — VIOLATION) |
| plaintext | `"Trellis tamper/036 payload" + 38 zero bytes` (64 bytes) |
| sequence / prev_hash / authored_at | `0` / `null` / `1745000360` |

The CBOR encoding of the 65-byte byte string is `0x58 0x41 + 65 × 0x78`. The map prefix `0xac` (12-entry authored) and `0xad` (13-entry canonical) are unchanged from `append/001` and `append/041` — only the `idempotency_key` value-bytes section grows.

---

## Body

### Construction

Steps 1–10 of the standard append pipeline (see `append/001-minimal-inline-payload`'s derivation for the per-step framing) apply unchanged. The Ed25519 signature is computed over the actual `Sig_structure` bytes that include the over-length authored map. From the issuer's perspective, the signature is valid and the canonical hash is honest. The bytes the issuer signed are:

| Field | Value |
|---|---|
| `idempotency_key` | bstr length 65, repeated `0x78` |
| `content_hash` | SHA-256 over plaintext under `trellis-content-v1` domain |
| (other fields) | mirror `append/001`'s shape |

So `tamper/036` is **not** a forgery — it is a legitimate signature over a structurally-noncompliant authored event. The verifier's job is to reject the structural noncompliance regardless of signature validity.

### Verifier obligations

The verifier MUST:

1. Decode each canonical event's `EventPayload` per §6.1 / §28 CDDL.
2. Validate `idempotency_key` against `bstr .size (1..64)`.
3. On length out of bounds (empty or > 64 bytes), flip `structure_verified = false` and emit `tamper_kind = "idempotency_key_length_invalid"`.
4. The check is orthogonal to §17.3 conflict resolution; running it does not require state from the Canonical Append Service.

In this vector, step 2 fails immediately (length 65 > 64). Steps 4-onward of §19 (signature verification, hash recomputation, chain linkage) are skipped or surface as further failures depending on implementation; the report's primary `tamper_kind` is `idempotency_key_length_invalid`.

### Why `structure_verified = false` (not just `integrity_verified = false`)

Compare to `tamper/035` where the events are individually structurally valid; the violation there is at the inter-event idempotency layer (`integrity_verified = false`, `structure_verified = true`). Here the single event itself is **structurally** non-conformant to the §28 CDDL grammar — `bstr .size (1..64)` is part of the CDDL that defines what is and is not a valid `EventPayload`. So the verifier's `structure_verified` flag reflects the CDDL grammar fail.

---

## Footer — committed intermediates

| File | Size | SHA-256 |
|---|---:|---|
| `input-tampered-event.cbor` | 741 | `efc9a6ac0d91cc8ce089552562d52b83a254a1ff008a978e9c920b193be64e51` |
| `input-tampered-ledger.cbor` | 742 | `1a40376e42be9f15087d0880329fb746319a70b80122f73d7a74012c3bc27fc8` |
| `input-signing-key-registry.cbor` | 133 | `4f0efcbe40658fe661406d686007c3b8f1abf66132b3271f1e02799d72b41d08` |

| Cryptographic intermediates | Value |
|---|---|
| `kid` (issuer-001) | `af9dff525391faa75c8e8da4808b1743` |
| `idempotency_key` length | 65 bytes (bound: 1..64) |
| `canonical_event_hash` | `4feb3b7b06f630f79df90c8736ef0c66dee7efcb1806c665844774c0cbcceee6` |
| signature validity (over over-length payload) | VALID (this is the load-bearing point — signature ≠ structure) |
