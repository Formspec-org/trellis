---
title: Trellis Core Specification (Phase 1)
version: 1.0.0-draft.1
date: 2026-04-17
status: draft
editors: Formspec / WOS / Trellis Working Group
---

# Trellis Core Specification (Phase 1)

**Version:** 1.0.0-draft.1
**Date:** 2026-04-17
**Editors:** Formspec / WOS / Trellis Working Group
**Companion to:** Formspec Core v1.0, Formspec Respondent Ledger v0.1, WOS Kernel v1.0

---

## Abstract

Trellis is the cryptographic integrity substrate for the Formspec / WOS / Trellis stack. It specifies a byte-exact, append-only, offline-verifiable envelope format for individual **events** (atomic append units); hash-chained **response ledgers** (scoped to one Formspec Response); signed **checkpoints** over contiguous event ranges; and a deterministic **export package** that a stranger with only the package in hand can verify on an air-gapped laptop decades after it was produced. This document is the Phase 1 Core: it normatively fixes the wire format, the Ed25519/COSE_Sign1 signature profile, the dCBOR canonical encoding, the SHA-256 hash construction, the signing-key registry, the HPKE Base-mode payload-key wrap, the reserved commitment and causal-dependency slots, the append idempotency contract, the export ZIP layout, and the verification algorithm. Phase 2 runtime-integrity behavior and Phase 3 case-ledger / agency-log composition are strict supersets of this core by using reserved extension containers; they do not redefine the event payload or checkpoint payload. The Trellis Operational Companion covers projection discipline, metadata budgets, and posture-transition auditability; it is a separate normative document.

## Status of This Document

This is the **Phase 1 Core** deliverable of the Trellis specification family: the constitutional byte-level substrate on which Phase 2 runtime integrity and Phase 3 portable case files are built. Implementors are encouraged to build against this specification and file interop reports. The single success criterion for Phase 1 is that an independent implementor, reading only this document and the cited normative references, can produce a second implementation whose byte output matches the reference fixtures (§29).

Operational guarantees that depend on live system behavior — projection snapshot policy, metadata-budget declaration tables, posture-transition auditability, rights-impacting evaluator rebuild — are normatively delegated to the **Trellis Operational Companion** (Phase 2 deliverable), which is a separate document. This core does not block on the companion: Phase 1 ships complete without it.

This core is the first Trellis document that may be cited in production procurement. Previous `DRAFTS/*` material and earlier split-out companion drafts now archived under `specs/archive/` are consolidated and superseded by this file for all Phase 1 normative purposes. Where the earlier documents contained material that is operational rather than constitutional, that material migrates to the Operational Companion and not to this document.

## Table of Contents

1. Introduction
2. Status and Non-Goals *(combined into §1)*
3. Conformance
4. Terminology
5. Canonical Encoding
6. Event Format
7. Signature Profile
8. Signing-Key Registry
9. Hash Construction
10. Chain Construction
11. Checkpoint Format
12. Header Policy
13. Commitment Slots Reserved
14. Registry Snapshot Binding
15. Snapshot and Watermark Discipline
16. Verification Independence Contract
17. Append Idempotency Contract
18. Export Package Layout
19. Verification Algorithm
20. Trust Posture Honesty
21. Posture / Custody / Conformance-Class Vocabulary
22. Composition with Respondent Ledger
23. Composition with WOS `custodyHook`
24. Agency Log (Phase 3 Superset Preview)
25. Security and Privacy Considerations
26. IANA Considerations
27. Test Vector Requirements
28. Appendix A — Full CDDL
29. Appendix B — Example Events and Exports
30. References

---

## 1. Introduction

### 1.1 What Trellis is

Formspec captures what a respondent answered. WOS governs the workflow that adjudicates it. Trellis is the layer that makes the resulting record **survive the system, the vendor, and the years**. A Trellis envelope is the smallest atomic unit of attested record in the stack: one COSE_Sign1 signature over a deterministically encoded CBOR object whose content hash is taken over ciphertext, whose signing key is resolvable against an export-embedded key registry, and whose append position is attested by a signed checkpoint over the preceding tree.

Trellis is not a workflow engine. It is not an identity platform. It is not an orchestrator. It is the integrity substrate against which three levels of Formspec / WOS structure compose.

### 1.2 Three scopes of append-only structure

This stack uses three nested scopes of append-only structure. Every requirement in this specification is qualified by scope.

- **Event.** An atomic append: one field edit, one governance action, one signature. Defined in §6.
- **Response ledger.** A hash-chained sequence of events for one Formspec Response. Sealed at submission. Phase 1's primary scope.
- **Case ledger.** A hash-chained sequence of governance events composing one or more sealed response-ledger heads with WOS governance events into one adjudicatory matter. Phase 3. Its event format is this specification's event format; its head format is a strict superset of this specification's checkpoint format (§24).
- **Agency log.** An operator-maintained log whose *entries* are case-ledger heads plus arrival metadata and optional witness signatures. Phase 3. Structurally CT-style; it proves that a case existed at time T and was not quietly deleted.
- **Federation log.** A log of agency-log heads witnessed by an independent operator. Phase 4.

Throughout this document, "ledger" is always qualified by scope. "Log" is reserved for structures whose entries are other ledgers' heads. All five are Trellis-shaped: same envelope, same hash, same signing profile, different scopes.

### 1.3 Non-goals

This specification does not define:

- workflow semantics (those live in WOS),
- form data-capture semantics (those live in Formspec),
- storage backends (Postgres, blob stores, object stores are implementation choices),
- transport protocols (HTTPS, mTLS, gRPC, IPFS are transport choices),
- post-quantum cryptography as a Phase 1 shipping requirement (§7.3 reserves the seam; Phase 1 ships Ed25519),
- a BBS+ / Pedersen selective-disclosure implementation (§13 reserves the slots; implementation deferred to Phase 2+),
- legal admissibility in any jurisdiction (§20).

### 1.4 Phase supersetting commitment

**The Phase 1 envelope IS the Phase 3 case-ledger event format.** Phase 2 and Phase 3 are strict supersets: they add runtime attestation (Phase 2) and case-scoped composition (Phase 3); they do not redefine the event. Later phases add data only through reserved extension containers defined in §6 and §11, and preserve the Phase 1 payload fields byte-for-byte. Phase 1 verifiers reject unknown top-level payload fields; forward compatibility comes from registered extension containers, not from accepting arbitrary top-level data. This is a normative commitment (§6.5 MUST-clause).

---

## 2. Conformance

The key words **MUST**, **MUST NOT**, **REQUIRED**, **SHALL**, **SHALL NOT**, **SHOULD**, **SHOULD NOT**, **RECOMMENDED**, **NOT RECOMMENDED**, **MAY**, and **OPTIONAL** in this document are to be interpreted as described in BCP 14 ([RFC 2119], [RFC 8174]) when, and only when, they appear in all capitals as shown here.

### 2.1 Conformance classes

An implementation MAY claim one or more of the following conformance classes. A claim MUST satisfy every requirement applicable to each claimed class.

1. **Fact Producer.** Produces signed events. Typical: Formspec respondent client, WOS governance runtime.
2. **Canonical Append Service.** Admits events into canonical order, forms a Merkle tree over event hashes, issues signed checkpoints.
3. **Verifier.** Validates an export package offline.
4. **Derived Processor.** Builds projections from canonical truth. (Snapshot and rebuild discipline in §15; full operational detail in the Operational Companion.)
5. **Export Generator.** Assembles export packages that a Verifier can validate without network access.

Phase 1 conformance for each class requires satisfying every MUST in every section of this document tagged to that class. A second implementation that passes the fixture suite of §29 demonstrates conformance as the class(es) it implements.

### 2.2 RFC 2119 scope

RFC 2119 keywords in this document govern byte-level wire format (§§5–11), verifier behavior (§19), export contents (§18), and the trust-posture honesty floor (§20). They do not govern UX, transport, storage backend choice, or any matter explicitly delegated to a companion.

---

## 3. Terminology

- **Event.** The atomic append unit defined by §6.
- **Canonical event hash.** The SHA-256 digest over the dCBOR-encoded `CanonicalEventHashPreimage` under domain separator `trellis-event-v1` (§9.2).
- **Prior event hash.** The canonical event hash of the immediately preceding event in the same response ledger (§10.2).
- **Content hash.** The SHA-256 digest over the ciphertext body of an event's payload (§9.3). Computed over ciphertext so that per-subject key destruction (crypto-shredding) erases the payload without invalidating the chain.
- **`suite_id`.** Registered identifier for the full signature suite (curve + signature algorithm + digest) used on a given event, checkpoint, or manifest (§7). It is carried in the COSE protected header.
- **`kid`.** Signing-key identifier, unique within the signing-key registry of an export (§8.2). It is carried in the COSE protected header.
- **Payload reference.** A tagged reference to encrypted payload bytes: either inline ciphertext with nonce, or a content-addressed external ciphertext reference (§6.4).
- **Signing-key registry.** The append-only registry of `SigningKeyEntry` records embedded in every export (§8).
- **Key bag.** Per-event set of HPKE-wrapped content-encryption keys, one per recipient (§9.4).
- **`LedgerServiceWrapEntry`.** Append-only record of a service-side re-wrap after a Long-lived Authority Key rotation (§8.6); does not mutate the original event's `key_bag` or canonical event hash.
- **Canonical append attestation.** A signed checkpoint plus an inclusion proof that a given event hash appears at a given position in the canonical tree.
- **Checkpoint.** A COSE_Sign1 whose payload is a `CheckpointPayload` over a ledger scope and tree head (§11).
- **Tree head hash.** The Merkle root over the sequence of canonical event hashes up to `tree_size`, under the RFC-6962-compatible construction of §11.3.
- **Watermark.** The tuple `(tree_size, tree_head_hash)` identifying the canonical state from which a derived artifact or agency-log entry was built (§15).
- **Idempotency key.** A stable, caller-supplied, opaque identifier of up to 64 bytes, or a UUIDv7, that identifies an append attempt (§17).
- **Export package.** A deterministic ZIP with the contents of §18.
- **Response ledger / case ledger / agency log / federation log.** See §1.2.
- **Conformance class.** See §2.1. Legacy drafts called these "profiles"; this document renames them to eliminate the tri-namespace overload (§21).
- **Custody model.** See §21. The legacy companion draft's "Profiles A–E" (provider-readable / reader-held / delegated / threshold / organizational) are renamed "Custody Models" here.
- **Posture.** See §21. The Respondent Ledger spec's "Profile A/B/C" (privacy × identity × integrity-anchoring) is renamed "Posture" in the Trellis vocabulary. Its normative home remains Respondent Ledger §15A.

When a normative clause below uses one of these terms, it means exactly what is defined here — not the same term as it may appear in `DRAFTS/` material or in superseded drafts.

---

## 4. Non-goals and authority boundaries

This Phase 1 core does not extend or reinterpret semantics owned by other specifications. The following cross-repo authority boundary is normative.

- **Formspec** is authoritative for Definition structure, Response semantics, FEL evaluation, validation algorithm, relevance and calculation, version pinning, and the four-phase processing model. Trellis does not restate those; a `formspec.authored` event is a Trellis envelope wrapping a Formspec artifact whose meaning is fixed by Formspec.
- **WOS** is authoritative for actor model, case state, deontic governance, autonomy caps, and workflow runtime semantics. A `wos.governance` event is a Trellis envelope wrapping a WOS artifact whose meaning is fixed by WOS.
- **Trellis** is authoritative for the envelope bytes, the hash chain, the checkpoint format, the export package, and the verification algorithm — and for nothing else. The Trellis verifier does not evaluate Formspec validation rules, it does not run WOS deontic checks, and it does not decide workflow outcomes. It verifies integrity and provenance distinctions; the admissibility of what it attests to remains bound to Formspec and WOS.

Trellis-bound Formspec processors MUST implement at least Formspec Core conformance. Trellis-bound WOS runtimes MUST implement at least WOS Kernel conformance. Where Trellis behavior depends on Formspec Definition or Response semantics (for example, version-pinning determinism, calculated-field provenance), processing MUST be delegated to a Formspec-conformant processor; Trellis MUST NOT restate the Formspec rule.

---

## 5. Canonical Encoding

**Requirement class:** Fact Producer, Canonical Append Service, Verifier, Export Generator.

### 5.1 Pinned encoding: dCBOR

All Trellis byte-level structures — events, checkpoints, signing-key registry entries, export manifests, inclusion proofs, consistency proofs — are serialized as **deterministic CBOR (dCBOR)**, which for this specification means the Core Deterministic Encoding profile of [RFC 8949] §4.2.2:

- Integers encoded in the smallest possible representation (no leading zero-length prefixes).
- Map keys sorted in byte-wise lexicographic order of their canonical CBOR encoding; duplicate keys rejected.
- No indefinite-length items (all arrays, maps, byte strings, text strings use definite-length encoding).
- Floating-point values, if used, encoded as the shortest form that round-trips to the same value; NaN and infinities rejected in all fields normatively defined here.
- Byte strings (`bstr`) used for all binary material; text strings (`tstr`) used only for human-readable identifiers.

Implementations MUST NOT emit non-deterministic CBOR. A record that does not round-trip byte-for-byte through a conformant dCBOR encoder is not a canonical record.

### 5.2 Reproducibility requirement

Every byte-level test vector in the fixture suite of §29 MUST reproduce byte-for-byte on a second, independent implementation. Cross-implementation byte-match is the success criterion for this encoding choice. An implementation that produces semantically-equivalent-but-byte-different output is non-conformant; dCBOR exists precisely to remove that latitude.

### 5.3 CDDL grammar fragment

All structured types in this specification are described by [RFC 8610] CDDL. The authoritative grammar is in Appendix A (§28); each section below gives the fragment relevant to that section.

```cddl
; Canonical encoding base type.
; All bstr fields are raw bytes; tstr fields are UTF-8.
; All maps are encoded dCBOR per §5.1.
canonical-bytes = bytes  ; dCBOR-encoded
digest = bstr .size 32   ; SHA-256, 32 bytes
```

---

## 6. Event Format

**Requirement class:** Fact Producer (produces), Canonical Append Service (admits), Verifier (validates), Export Generator (includes).

### 6.1 Normative structure

An **event** is the atomic append unit. Every append to any Trellis-shaped structure (response ledger, case ledger, agency log, federation log) is one event. On the wire an event is a COSE_Sign1 object whose protected headers identify the signing suite and key, and whose payload is the dCBOR encoding of `EventPayload`.

```cddl
Event = COSESign1Bytes ; RFC 9052 COSE_Sign1 tagged CBOR value.

EventPayload = {
  version:           uint .size 1,          ; wire-format version, = 1 for Phase 1
  ledger_scope:      bstr,                  ; replay boundary, §10.4
  sequence:          uint,                  ; monotonic within ledger scope
  prev_hash:         digest / null,         ; §10; null only for sequence == 0
  causal_deps:       [* digest] / null,     ; §10.3 reserved; null or [] in Phase 1
  author_event_hash: digest,                ; §9.5; excludes itself and signatures
  content_hash:      digest,                ; SHA-256 over ciphertext, §9.3
  header:            EventHeader,           ; §12
  commitments:       [* Commitment] / null, ; §13 reserved
  payload_ref:       PayloadRef,            ; inline or external ciphertext, §6.4
  key_bag:           KeyBag,                ; §9.4
  idempotency_key:   bstr .size (1..64),    ; §17
  extensions:        { * tstr => any } / null, ; reserved top-level extension container
}
```

An `EventPayload` is a CBOR map with exactly these keys. Additional top-level keys are reserved for future phases and MUST NOT be emitted by Phase 1 producers; Phase 1 verifiers encountering an unknown top-level key MUST reject the event. Additive extension is performed via `EventPayload.extensions` and `EventHeader.extensions`, which are explicitly reserved for forward-compatible growth.

### 6.2 Sequence and prev_hash

- `sequence` is a non-negative integer, monotonic within ledger scope. The first event in a ledger has `sequence = 0`.
- `ledger_scope` is the response-ledger, case-ledger, agency-log, or federation-log identifier. It is signed and hashed so a genesis event or copied event cannot replay into another scope.
- `prev_hash` is `null` if and only if `sequence == 0`. Otherwise it MUST be the canonical event hash (§9.2) of the immediately preceding event in the same `ledger_scope`.
- Phase 1 chain construction is strict linear (§10). `causal_deps` MUST be either `null` or the empty array `[]` in Phase 1 events.

### 6.3 author_event_hash

`author_event_hash` is the integrity digest of the author-originated portion of the event: the signed envelope, the ciphertext, and the key bag at the moment of signing. It is computed per §9.5 and MUST NOT change after signing, even under service-side key rotation (§8.6).

### 6.4 content_hash over ciphertext

`content_hash` is `SHA-256` over the exact ciphertext bytes named by `payload_ref`. This hash is over **ciphertext, not plaintext** (§9.3). This choice is load-bearing for crypto-shredding: destroying the per-subject content-encryption key erases the plaintext without invalidating the hash chain, because verification checks the ciphertext hash and the signature — neither of which depends on plaintext.

`payload_ref` is one of two tagged structures:

```cddl
PayloadRef = PayloadInline / PayloadExternal

PayloadInline = {
  ref_type:   "inline",
  ciphertext: bstr,
  nonce:      bstr,
}

PayloadExternal = {
  ref_type:       "external",
  content_hash:   digest,
  availability:   AvailabilityHint,
  retrieval_hint: tstr / null,
}

AvailabilityHint = &(
  InExport:     0,  ; bytes are present in the export payload directory
  External:     1,  ; bytes are retrievable from a content-addressed external store
  Withheld:     2,  ; bytes intentionally omitted for disclosure/readability reasons
  Unavailable:  3,  ; bytes unavailable; verifier must report omitted checks
)
```

For `PayloadInline`, `content_hash` MUST equal the hash of `ciphertext`. For `PayloadExternal`, `EventPayload.content_hash` MUST equal `PayloadExternal.content_hash`; if ciphertext bytes are not present in the export, an offline verifier reports that payload integrity and readability checks could not run (§19) rather than pretending they succeeded.

### 6.5 Phase-superset extension points

The following extension points are reserved for Phase 2 / Phase 3 superset growth. Phase 1 producers MUST NOT populate them except as `null` or empty maps/lists; Phase 1 verifiers MUST accept them as empty. Phase 2+ producers MUST populate them according to later-phase specifications; Phase 1+Phase 2 bridge verifiers MUST reject records whose extension-point usage violates this specification.

- `causal_deps` — Phase 2 HLC / DAG causal ordering.
- `commitments` — Phase 2+ per-field redaction-aware commitments (§13).
- `EventPayload.extensions` — additive top-level event metadata without top-level field growth.
- `header.extensions` — additive header fields without a version bump (§12.3).
- `header.witness_ref` — Phase 4 transparency-witness references.

**Normative phase-superset commitment.** The byte shape produced by a Phase 1 Export Generator MUST be byte-identical to what a Phase 3 case-ledger event of the same logical content would produce at Phase 1 semantic equivalence. A later-phase event MAY add fields under the reserved extension points above, but MUST NOT rename, remove, reorder, or redefine any field of §6.1 in a way that breaks Phase 1 verification. "Strict superset" means semantic and structural preservation through reserved extension containers. It does not mean Phase 1 verifiers accept unknown top-level fields.

### 6.6 Signature scope

An Event signature is the RFC 9052 COSE_Sign1 signature over `EventPayload` using COSE's standard `Sig_structure` preimage:

```text
["Signature1", protected, external_aad, payload]
```

`payload` is the exact dCBOR bytes of `EventPayload`. `external_aad` is the zero-length byte string for Phase 1. The COSE protected header MUST contain `alg`, `kid`, and `suite_id` (§7.4). No signature bytes are present in `EventPayload`, so event signing is non-circular by construction.

---

## 7. Signature Profile

**Requirement class:** Fact Producer, Verifier.

Every signed artifact in Trellis — events, checkpoints, manifests, and signing-key-registry administrative entries — is a COSE_Sign1 value and carries an explicit `suite_id` identifying the signature suite used. A verifier that encounters an unregistered `suite_id` MUST reject the artifact. The `suite_id` registry (§26.2) is part of the IANA considerations.

### 7.1 Pinned Phase 1 suite

**Phase 1 pins `suite_id = 1` to Ed25519-over-COSE_Sign1.** Concretely: the signature is COSE_Sign1 ([RFC 9052]) with `alg = -8` (EdDSA) and the signing key a 32-byte Ed25519 public key ([RFC 8032]). The digest algorithm used by dependent constructions (canonical event hash, content hash, checkpoint tree head) is SHA-256 ([FIPS 180-4]).

A Phase 1 Fact Producer MUST use `suite_id = 1` in the COSE protected header. A Phase 1 Verifier MUST accept any registered `suite_id` whose suite it recognizes, and MUST reject any unregistered `suite_id`.

### 7.2 `suite_id` IANA-style registry

The registry is a content-addressed append-only list; the Phase 1 initial contents are:

| `suite_id` | Signature suite | Digest | Status | Notes |
|---|---|---|---|---|
| 0 | Reserved | — | Reserved | MUST NOT appear on any canonical artifact. |
| 1 | Ed25519 / COSE_Sign1 / EdDSA | SHA-256 | **Active (Phase 1)** | Phase 1 mandatory suite. |
| 2 | Reserved for ML-DSA-65 ([FIPS 204]) | SHA-256 | Reserved | Phase 2+ post-quantum suite candidate. |
| 3 | Reserved for SLH-DSA-128s ([FIPS 205]) | SHA-256 | Reserved | Phase 2+ hash-based signature candidate. |
| 4 | Reserved for hybrid (Ed25519 + ML-DSA-65) | SHA-256 | Reserved | Phase 2+ migration-period hybrid. |
| 5–15 | Reserved | — | Reserved | IANA codepoint reservation, see §26. |

Future suites MUST be registered in the `suite_id` registry before any verifier is required to accept them. A record with an unregistered `suite_id` is not a canonical record.

### 7.3 Migration obligation (multi-decade verifiability)

A verifier running in 2045 MUST be able to resolve a signature produced in 2026 after intervening key rotations, and MUST be able to resolve a signature produced with `suite_id = 1` after the Trellis family has migrated its active default to a post-quantum suite. This imposes three obligations on Phase 1 implementations:

1. Every signed artifact MUST carry its `suite_id` in the COSE protected header (§7.4). A verifier MUST use that `suite_id`, not the "current" suite.
2. Every export MUST include a signing-key registry (§8) capable of resolving the signing `kid` referenced by every event, checkpoint, and manifest in the export, as that key existed at signing time.
3. Crypto parameters (curve, digest, length constraints) MUST be determined by `suite_id`, not by the verifier's current default. A verifier that "upgrades" a 2026 signature to 2045 semantics by reinterpreting it is broken.

Phase 1 does not ship a post-quantum suite. It pins the migration seam and the obligation.

### 7.4 COSE protected headers and Sig_structure

Trellis uses RFC 9052 COSE_Sign1 directly. Implementations MUST use a normal COSE library's signing preimage, not a Trellis-specific self-reference workaround.

For every Trellis COSE_Sign1 artifact, the protected header MUST contain:

| Header | Value |
|---|---|
| `alg` | COSE algorithm identifier. Phase 1: `-8` (EdDSA). |
| `kid` | 16-byte signing-key identifier resolvable in `signing-key-registry.cbor` (§8). |
| `suite_id` | Trellis signature-suite identifier. Phase 1: `1`. |

The protected header MAY additionally carry `artifact_type` with values `event`, `checkpoint`, `manifest`, or another registered value. If present, a verifier MUST check that it matches the containing artifact. If absent, the containing archive member or enclosing structure supplies the artifact type.

To sign an Event, Checkpoint, or Export Manifest:

1. Build the artifact payload map (`EventPayload`, `CheckpointPayload`, or `ExportManifestPayload`) with no signature field.
2. Serialize the payload map as dCBOR (§5).
3. Construct a COSE_Sign1 object whose payload is those bytes.
4. Populate the protected header with `alg`, `kid`, and `suite_id`.
5. Sign the RFC 9052 `Sig_structure` array `["Signature1", protected, external_aad, payload]`, with `external_aad` equal to the zero-length byte string for Phase 1.

A verifier uses the protected-header `kid` to resolve the public key via the signing-key registry (§8), uses protected-header `suite_id` and `alg` to select the suite, and verifies the COSE_Sign1 signature over the standard `Sig_structure`. Signature verification is independent of all other verification steps.

---

## 8. Signing-Key Registry

**Requirement class:** Canonical Append Service (maintains), Export Generator (snapshots), Verifier (resolves).

### 8.1 Why this exists

A COSE signature without a resolvable key is unverifiable after rotation. Phase 1 exports MUST include a signing-key registry snapshot so that verification is self-contained at any future date, including dates after every key the registry references has been rotated out of operational use.

### 8.2 `SigningKeyEntry`

```cddl
SigningKeyEntry = {
  kid:           bstr .size 16,           ; unique key identifier; §8.3
  pubkey:        bstr,                    ; raw public key bytes per suite_id
  suite_id:      uint,                    ; §7.2
  status:        SigningKeyStatus,        ; §8.4
  valid_from:    uint,                    ; RFC 3339 timestamp as Unix seconds UTC
  valid_to:      uint / null,             ; null for currently-active keys
  supersedes:    bstr / null,             ; kid this entry replaces, if any
  attestation:   bstr / null,             ; optional HSM / KMS attestation, suite-defined
}

SigningKeyStatus = &(
  Active:   0,
  Rotating: 1,
  Retired:  2,   ; no new signatures, historical verification permitted
  Revoked:  3,   ; hard-reject events signed under this key after valid_to
)
```

### 8.3 `kid` format

`kid` is a 16-byte opaque identifier. An implementation MAY derive it as the first 16 bytes of `SHA-256(suite_id || pubkey)`, or MAY assign it by administrative policy; either MUST produce a `kid` unique within the registry.

### 8.4 Lifecycle

Legal `status` transitions:

- `Active → Rotating`: a successor key has been provisioned; both keys accept signatures.
- `Rotating → Retired`: successor is fully deployed; the old key accepts no new signatures but remains verifiable for historical material.
- `Rotating → Revoked`: compromise detected during rotation.
- `Active → Revoked`: compromise detected without intermediate Rotating state.
- `Retired → Revoked`: late detection of compromise.

`Revoked` is terminal. `Retired` is terminal for signature issuance but not for verification of historical records. `Destroyed` is out of scope for Phase 1 signing keys (signing-key destruction is an operational action represented in the agency log, §24); the private key material MAY be destroyed, but the `SigningKeyEntry` MUST remain in the registry to preserve historical verifiability.

### 8.5 Registry snapshot in every export

Every export package (§18) MUST include a complete registry snapshot resolvable for every `kid` referenced by any event, checkpoint, or `LedgerServiceWrapEntry` in the export. "Complete" means: for any `kid` cited, the entry for that `kid`, every entry that `supersedes` points to transitively, and every entry that is a supersession ancestor of the cited `kid`. A verifier encountering a `kid` that cannot be resolved against the embedded registry MUST reject the export.

### 8.6 `LedgerServiceWrapEntry` under LAK rotation

A service-side Long-lived Authority Key (LAK) used to wrap payload content-encryption keys (§9.4) MAY rotate independently of event signing keys. Rotation that changes the LAK-wrapping of historical payloads MUST NOT modify any Phase 1 `Event` (including its `key_bag`, `author_event_hash`, or `content_hash`). Instead, a service-side re-wrap produces an append-only `LedgerServiceWrapEntry`:

```cddl
LedgerServiceWrapEntry = {
  ledger_id:          bstr,                ; scope identifier
  author_event_hash:  digest,              ; target event
  lak_version:        uint,                ; new LAK version
  ephemeral_pubkey:   bstr .size 32,       ; fresh X25519 ephemeral public key
  wrapped_dek:        bstr,                ; HPKE-wrapped DEK, §9.4
  created_at:         uint,                ; Unix seconds UTC
  signature:          COSESign1Bytes,      ; service signature, same COSE suite as §7
}
```

This is load-bearing for invariant #7 (key-bag immutability under rotation). Historical `author_event_hash` values MUST reproduce after any LAK rotation, because no field covered by `author_event_hash` ever changes; the new wrap is a separate canonical artifact.

---

## 9. Hash Construction

**Requirement class:** Fact Producer, Canonical Append Service, Verifier.

### 9.1 Domain separation discipline

Every hash computation in Trellis is domain-separated by a length-prefixed UTF-8 byte tag. The generic form is:

```
digest = SHA-256(
    len(tag)       as 4-byte big-endian unsigned ||
    tag            as UTF-8 bytes ||
    len(component) as 4-byte big-endian unsigned ||
    component      as raw bytes
)
```

Multi-component inputs repeat the `len(component) || component` pair for each input, in the fixed order specified for the construction. Length-prefixing (not delimiter-based framing) removes component-boundary ambiguity. The generic procedure is referenced below by domain tag only.

### 9.2 Canonical event hash (`trellis-event-v1`)

The canonical event hash is:

```
canonical_event_hash = SHA-256(
    "trellis-event-v1" domain-separated per §9.1 over
    dCBOR(CanonicalEventHashPreimage)
)
```

```cddl
CanonicalEventHashPreimage = {
  version:       uint .size 1,
  ledger_scope:  bstr,
  event_payload: EventPayload,
}
```

`event_payload` is the decoded `EventPayload` carried as the COSE payload. It contains `author_event_hash`, but it contains no signature bytes. The domain tag `trellis-event-v1` reserves the `-v1` slot for this construction; a future version bump would use `-v2`. Verifiers MUST match the domain tag to the event payload's `version` field.

### 9.3 Content hash (`trellis-content-v1`, over ciphertext)

```
content_hash = SHA-256(
    "trellis-content-v1" domain-separated per §9.1 over
    the exact ciphertext bytes named by payload_ref
)
```

`content_hash` is over **ciphertext**, never plaintext. Hashing plaintext forecloses crypto-shredding: once the DEK is destroyed, plaintext cannot be recovered, so a plaintext hash binds the chain to something that no longer exists — forcing either non-erasure (violating GDPR Article 17 / FOIA redaction obligations) or chain breakage (violating the append-only invariant). Hashing ciphertext preserves both: erasure destroys the DEK, the ciphertext remains hash-verifiable, and the chain stands.

### 9.4 Key bag and HPKE wrap

The payload content-encryption key (DEK) is wrapped for one or more recipients in the event's `key_bag`:

```cddl
KeyBag = {
  entries: [* KeyBagEntry],
}
KeyBagEntry = {
  recipient:        bstr,                ; stable recipient identifier (e.g., "ledger-service", DID)
  suite:            uint,                ; HPKE suite identifier, Phase 1 fixed to 1
  ephemeral_pubkey: bstr .size 32,       ; X25519 ephemeral public key, unique per wrap
  wrapped_dek:      bstr,                ; HPKE-sealed DEK
}
```

Phase 1 HPKE suite 1 ([RFC 9180]) is **Base mode**, KEM = `DHKEM(X25519, HKDF-SHA256)`, KDF = `HKDF-SHA256`, AEAD = `ChaCha20-Poly1305`.

Choice of ChaCha20-Poly1305 over AES-256-GCM for payload AEAD is for constant-time implementability on WASM and non-AES-NI hardware; it is not a security claim about one over the other. Both satisfy the confidentiality requirement. Deployments that require AES-256-GCM MAY register an additional HPKE suite in Phase 2.

Every wrap MUST use a **fresh ephemeral X25519 keypair**, generated, used once, and destroyed. The `ephemeral_pubkey` is persisted in the envelope so the recipient can perform ECDH. Reusing an ephemeral keypair across wraps is a non-conformance.

Authentication note: HPKE Base mode provides no sender authentication at the wrap layer. This is adequate because (a) `author_event_hash` covers the key bag, (b) the event is Ed25519-signed per §7, and (c) any modification to the key bag invalidates the signature. Use of HPKE Auth mode would add sender-key management at the wrap layer without strengthening the envelope; the envelope signature is the authentication boundary.

### 9.5 `author_event_hash` construction

```
author_event_hash = SHA-256(
    "trellis-author-event-v1" domain-separated per §9.1 over
    dCBOR(AuthorEventHashPreimage)
)
```

```cddl
AuthorEventHashPreimage = {
  version:         uint .size 1,
  ledger_scope:    bstr,
  sequence:        uint,
  prev_hash:       digest / null,
  causal_deps:     [* digest] / null,
  content_hash:    digest,
  header:          EventHeader,
  commitments:     [* Commitment] / null,
  payload_ref:     PayloadRef,
  key_bag:         KeyBag,
  idempotency_key: bstr .size (1..64),
  extensions:      { * tstr => any } / null,
}
```

`author_event_hash` binds the envelope payload, payload reference, and key bag at the moment of signing. It excludes itself and all signature material by construction: `AuthorEventHashPreimage` has no `author_event_hash` field and no COSE signature field. It is immutable under rotation because none of its inputs is altered by service-side re-wraps (§8.6); re-wraps produce append-only `LedgerServiceWrapEntry` records outside the author-event scope.

### 9.6 Checkpoint digest (`trellis-checkpoint-v1`)

```
checkpoint_digest = SHA-256(
    "trellis-checkpoint-v1" domain-separated per §9.1 over
    dCBOR(CheckpointHashPreimage)
)
```

```cddl
CheckpointHashPreimage = {
  version:            uint .size 1,
  scope:              bstr,
  checkpoint_payload: CheckpointPayload,
}
```

### 9.7 Export manifest digest (`trellis-export-manifest-v1`)

```
export_manifest_digest = SHA-256(
    "trellis-export-manifest-v1" domain-separated per §9.1 over
    dCBOR(ExportManifestHashPreimage)
)
```

```cddl
ExportManifestHashPreimage = {
  version:          uint .size 1,
  scope:            bstr,
  manifest_payload: ExportManifestPayload,
}
```

The export manifest (§18.3) is also signed as a COSE_Sign1 object. The digest construction above is used when another artifact needs to refer to the manifest by content address.

### 9.8 Domain-tag registry

Phase 1 reserves these domain tags. An implementation MUST NOT use any of these tags for any purpose other than its defined construction. A future version's constructions MUST bump the version suffix and register the new tag.

- `trellis-event-v1` — canonical event hash (§9.2)
- `trellis-author-event-v1` — author event hash (§9.5)
- `trellis-content-v1` — content hash (§9.3)
- `trellis-checkpoint-v1` — checkpoint digest (§11.2)
- `trellis-export-manifest-v1` — export manifest digest (§9.7)
- `trellis-merkle-leaf-v1` — Merkle tree leaf hash (§11.3)
- `trellis-merkle-interior-v1` — Merkle interior-node hash (§11.3)

---

## 10. Chain Construction

**Requirement class:** Fact Producer (computes prev_hash), Canonical Append Service (enforces order), Verifier (replays chain).

### 10.1 Phase 1: strict linear chain

Phase 1 canonical order is **strict linear** per ledger scope. Events are totally ordered by `sequence`, and each event's `prev_hash` references the canonical event hash of the immediately preceding event. There is exactly one canonical order per ledger scope.

### 10.2 `prev_hash` requirements

- For `sequence == 0`: `prev_hash` MUST be `null`.
- For `sequence == N > 0`: `prev_hash` MUST equal the `canonical_event_hash` (§9.2) of the event with `sequence == N-1` in the same ledger.
- A Canonical Append Service MUST reject any submission whose `prev_hash` does not satisfy this constraint.
- A Verifier MUST verify the chain by recomputing each event's `canonical_event_hash` and checking that it appears as `prev_hash` in the next event.

### 10.3 Reserved: causal dependencies (Phase 2)

Phase 2 MAY upgrade to an HLC-ordered causal DAG. The `causal_deps` field (§6.1) reserves the wire slot. Phase 1 events MUST emit `causal_deps` as `null` or `[]`. Phase 2+ producers populating `causal_deps` emit a new `version` value; Phase 1 verifiers MUST reject events whose `causal_deps` is non-empty at `version == 1`. This reservation exists so that Phase 2 does not require a header-version break.

### 10.4 Ledger scope and partitioning

Canonical order is scoped to a declared **ledger scope** — one Formspec Response for response ledgers, one case for case ledgers, one operator for agency logs, and so on. An implementation MAY partition a logical system into multiple ledgers by scope, but MUST NOT allow competing canonical orders for the same scope. There MUST be exactly one `sequence = 0`, `sequence = 1`, …, per scope.

### 10.5 Append-only invariant

A Canonical Append Service MUST NOT rewrite a canonical event once it has been admitted. "Admitted" means: `canonical_event_hash` is computed, the append attestation is issued, and any subscribed verifier is entitled to expect byte-for-byte reproduction of the event on demand. Correction is always a new event; there is no in-place edit.

---

## 11. Checkpoint Format

**Requirement class:** Canonical Append Service (issues), Verifier (validates).

### 11.1 Purpose

A **checkpoint** is a COSE_Sign1 signed tree head over the Merkle tree of canonical event hashes in a ledger scope up to some `tree_size`. The COSE payload is `CheckpointPayload`. Checkpoints enable:

- inclusion proofs (a given event hash appears at a given position),
- consistency proofs (a later tree is an append-only extension of an earlier tree),
- optional external anchoring (Phase 2+ transparency-log witnessing).

### 11.2 `Checkpoint` structure

```cddl
Checkpoint = COSESign1Bytes ; payload is dCBOR(CheckpointPayload)

CheckpointPayload = {
  version:       uint .size 1,          ; = 1 for Phase 1
  scope:         bstr,                  ; ledger scope identifier
  tree_size:     uint,                  ; count of events committed to
  tree_head_hash: digest,               ; Merkle root, §11.3
  timestamp:     uint,                  ; Unix seconds UTC at issuance
  anchor_ref:    bstr / null,           ; §11.5; Phase 1 optional
  prev_checkpoint_hash: digest / null,  ; previous checkpoint's digest, or null for the first
  extensions:    { * tstr => any } / null, ; §11.6; reserved for Phase 3+ heads
}
```

The checkpoint **digest** under domain tag `trellis-checkpoint-v1` is:

```
checkpoint_digest = SHA-256(
    "trellis-checkpoint-v1" domain-separated per §9.1 over
    dCBOR(CheckpointHashPreimage)
)
```

`prev_checkpoint_hash` chains checkpoints so a verifier can validate an ordered sequence without retrieving every event.

### 11.3 Merkle tree construction

The tree is RFC-6962-compatible with domain-separated leaf and interior hashes:

- **Leaf hash:** `SHA-256("trellis-merkle-leaf-v1" domain-separated per §9.1 over canonical_event_hash)`.
- **Interior hash:** `SHA-256("trellis-merkle-interior-v1" domain-separated per §9.1 over (left_hash || right_hash))`.
- **Odd-node promotion:** when a level has an odd number of nodes, the final node is carried up to the next level unchanged (RFC 6962 §2.1 semantics).

The tree is constructed over the sequence `[canonical_event_hash(0), canonical_event_hash(1), ..., canonical_event_hash(tree_size - 1)]` in canonical order.

### 11.4 Inclusion and consistency proofs

An **inclusion proof** for `canonical_event_hash` at position `leaf_index` under a checkpoint with `tree_size = N` is an audit path allowing the verifier to recompute `tree_head_hash` from the leaf. A verifier MUST recompute the root and check bitwise equality with the checkpoint's `tree_head_hash`.

A **consistency proof** between an earlier checkpoint `(m, head_m)` and a later checkpoint `(n, head_n)` where `m < n` is a proof path allowing the verifier to recompute both heads and confirm that the tree at size `n` is an append-only extension of the tree at size `m`.

Both proofs are included in exports (§18.3). A Canonical Append Service that publishes a checkpoint whose consistency proof against any prior head fails is in violation of the append-only invariant (§10.5) and MUST be treated by a verifier as a tampered source.

### 11.5 `anchor_ref` (reserved for Phase 2+ external witnessing)

`anchor_ref` is an optional opaque reference to an external witness or anchor (for example, an OpenTimestamps Bitcoin anchor, a transparency-log cosignature, an RFC 3161 TSA receipt). Phase 1 MUST accept `null`, and MUST NOT require a non-null value. Phase 1 producers MAY emit a value; Phase 1 verifiers MUST NOT fail verification solely on the absence of an anchor. Phase 4 (federation) elevates anchoring to MUST under a separate registered deployment class.

Exports that bundle anchor-proof material (for example, `bitcoin/headers.cbor` bundling Bitcoin block headers) do so under a Phase 2+ registered deployment class; Phase 1 verification MUST NOT depend on such material.

### 11.6 Head-format extension container

`CheckpointPayload.extensions` is the only Phase 1 reservation for Phase 3 case-ledger head data. Phase 1 producers MUST emit `extensions` as `null` or an empty map. Phase 1 verifiers MUST reject unknown top-level fields in `CheckpointPayload`, but MUST preserve and ignore unknown registered keys inside `extensions`.

Phase 3 case-ledger heads MUST embed or preserve the Phase 1 checkpoint payload unchanged. Additional head data is carried only in `extensions`, for example:

```cddl
CaseLedgerHeadExtensions = {
  ? composed_response_heads: [* digest],
  ? case_scope_metadata: CaseScope,
  ? witness_signatures: [* WitnessSignature],
}
```

The extension container is load-bearing for invariant #12: agency-log adoption is not a wire-format break for any Phase 1 checkpoint already in the field.

---

## 12. Header Policy

**Requirement class:** Fact Producer, Canonical Append Service, Verifier.

The event header is where Trellis makes an explicit, normatively-enumerated trade-off between what is plaintext (available for routing, classification, retention) and what is a commitment to an encrypted or private value. Implementations MUST NOT move fields across this boundary without a `version` bump.

### 12.1 `EventHeader` shape

```cddl
EventHeader = {
  event_type:    bstr,                    ; registered event-type identifier (§14)
  authored_at:   uint,                    ; Unix seconds UTC; plaintext
  retention_tier: uint,                   ; 0..3; plaintext
  classification: bstr,                   ; registered classification identifier; plaintext
  outcome_commitment: digest / null,      ; §12.2; commitment, NOT plaintext outcome
  subject_ref_commitment: digest / null,  ; §12.2; commitment, NOT plaintext subject
  tag_commitment: digest / null,          ; §12.2; commitment, NOT plaintext tags
  witness_ref:   bstr / null,             ; reserved for Phase 4
  extensions:    { * tstr => any } / null, ; additive; §12.3
}
```

### 12.2 Plaintext vs. committed declaration table

The following declaration is normative. Phase 1 events MUST place each field in the named layer; Phase 1 verifiers MUST reject an event whose header places a field in the wrong layer.

| Field | Layer | Rationale |
|---|---|---|
| `event_type` | Plaintext header | Structural verification, registry lookup (§14). MUST be outcome-neutral (see §12.4). |
| `authored_at` | Plaintext header | Routing, retention computation, audit timelines. MAY be coarsened per Operational Companion. |
| `retention_tier` | Plaintext header | Required for retention enforcement without decryption. |
| `classification` | Plaintext header | Required for routing and access-decision prefiltering. MUST NOT reveal outcome. |
| `outcome_commitment` | Commitment (plaintext is in ciphertext payload) | Determinations (granted / denied / eligible / ineligible) are HIPAA / adjudication sensitive; outcome in plaintext is a metadata disclosure. Commitment allows verifiers with payload access to check; observers learn nothing. |
| `subject_ref_commitment` | Commitment | Subject identity is sensitive in most jurisdictions. Plaintext subject in the header enables passive re-identification via envelope metadata alone. |
| `tag_commitment` | Commitment | Tags like `adverse-decision`, `appeal-pending` leak outcome indirectly. |
| `witness_ref` | Plaintext header (when present) | Phase 4 external witness references are structural. |
| `extensions` | Plaintext additive | Additive header fields (§12.3); MUST NOT carry outcome or subject material. |

The commitment construction for each `*_commitment` field is:

```
commitment = SHA-256(
    "trellis-header-commitment-v1" domain-separated per §9.1 over
    dCBOR([committed_value, commitment_nonce])
)
```

where `commitment_nonce` is a fresh 16-byte random value stored inside the encrypted payload (never in the envelope). Without the nonce, low-entropy committed values (for example, boolean eligibility) are trivially recoverable by hash-candidate brute force; the nonce makes the commitment hiding against external observers while remaining verifiable by payload holders.

### 12.3 `extensions` sub-map

`extensions` is a map from text-string keys to arbitrary CBOR values. Implementations MAY use it for deployment-specific additive metadata. A key that begins with `trellis-` is reserved for this specification family and MUST NOT be defined by third parties. Unknown keys MUST be preserved by intermediaries (round-tripped) and MAY be ignored by verifiers. `extensions` MUST NOT be used to smuggle fields that belong in the committed layer (outcome, subject, sensitive tags); doing so is a trust-posture-honesty violation (§20).

### 12.4 Event-type granularity

Registered `event_type` values MUST be outcome-neutral. Concretely: `wos.determination` is registered; `wos.determination.adverse` and `wos.determination.favorable` MUST NOT be. Outcome is carried in `outcome_commitment` and the encrypted payload. Event types within the same classification SHOULD be indistinguishable by commitment count or envelope shape so that the fixed plaintext fields do not leak outcome signal.

---

## 13. Commitment Slots Reserved

**Requirement class:** Fact Producer, Verifier.

Phase 1 reserves wire-level slots for per-field cryptographic commitments that enable redaction-aware export and selective disclosure in Phase 2+. The implementation of selective disclosure (BBS+, Pedersen with range proofs, Merkle-leaf revelation) is deferred. The slots are normative now because retrofitting them requires a wire-format break, and because Phase 3 portable case files without selective disclosure force either all-or-nothing disclosure or envelope reissue.

### 13.1 `Commitment` structure

```cddl
Commitment = {
  scheme:        uint,                ; commitment scheme identifier, §13.3
  slot:          uint,                ; position in fixed-per-event-type vector
  value:         bstr,                ; scheme-defined commitment bytes
  metadata:      bstr / null,         ; scheme-defined auxiliary data
}
```

### 13.2 Fixed-position vectors

For each registered `event_type`, the event-type registry (§14) MAY declare a fixed number of commitment slots and their intended field associations. Every event of a given type MUST carry exactly that number of commitments. Unused slots MUST be filled with a scheme-defined identity value (for example, a commitment to zero with random blinding for Pedersen; an all-zeros digest for Merkle) that is indistinguishable from populated slots.

### 13.3 Scheme registry (initial)

| `scheme` | Meaning | Status |
|---|---|---|
| 0 | Reserved | Reserved |
| 1 | Merkle leaf over dCBOR-serialized field value with salt | Reserved (Phase 2+) |
| 2 | Pedersen commitment over Ristretto255 | Reserved (Phase 2+) |
| 3 | BBS+ attribute commitment | Reserved (Phase 2+) |

Phase 1 producers MUST emit `commitments` as `null` or `[]`. Phase 1 verifiers MUST accept either. Phase 2+ implementations fill the slots under the registered scheme, and Phase 2+ verifiers accept according to the registry.

### 13.4 Why slots, not implementation, in Phase 1

Fixing the scheme in Phase 1 would lock in a scheme that the selective-disclosure field is still standardizing (BBS+ draft cadence, post-quantum disclosure schemes). Fixing the slot reservation now costs nothing; fixing the scheme now risks an obsolete choice. Slot reservation makes Phase 2 additive; not reserving would make Phase 2 a break.

---

## 14. Registry Snapshot Binding

**Requirement class:** Export Generator (embeds), Verifier (resolves historical meaning).

### 14.1 The problem

Signature verification without registry binding proves **byte integrity only** — it proves the signed bytes are unmodified, but it does not prove what those bytes **meant** at signing time. A 2045 verifier looking at a 2026 event needs to know: what did `event_type = "wos.determination"` signify in 2026? What was the commitment schema for that type? What classification vocabulary was in force?

Resolving those questions against a live registry in 2045 is insufficient, because the registry may have evolved, been migrated, or ceased to exist. Resolving against the registry **snapshot** that was in force at signing time is what preserves semantic verifiability.

### 14.2 Bound registry

Every export package (§18) MUST embed the domain registry in force at signing time, content-addressed by its SHA-256 digest. The registry covers at minimum:

- event-type taxonomy: `event_type` → semantic name, commitment schema, privacy classification,
- role vocabulary: actor roles admissible under the bound WOS and Formspec versions,
- governance rules: the WOS governance ruleset identifier and its digest,
- classification vocabulary: allowed values of `header.classification`.

### 14.3 `RegistryBinding`

```cddl
RegistryBinding = {
  registry_digest: digest,             ; SHA-256 over canonical registry bytes
  registry_format: uint,               ; 1 = dCBOR; 2+ reserved
  registry_version: tstr,              ; human-readable version string
  bound_at_sequence: uint,             ; sequence in the ledger where binding took effect
}
```

An export manifest MUST contain one or more `RegistryBinding` entries. The first binding MUST cover `sequence = 0`. Subsequent bindings, if any, MUST appear in monotonic `bound_at_sequence` order and each MUST be preceded by an explicit binding event in the canonical order.

### 14.4 Verifier obligation

A verifier resolving `event_type` (or any other registry-bound field) for an event at sequence `S` MUST resolve against the binding whose `bound_at_sequence` is the largest value `≤ S`, using the embedded registry bytes, not a live lookup. A verifier that performs a live registry lookup to interpret a historical event is non-conformant.

### 14.5 Registry migration discipline

Registry changes that affect interpretation (event-type semantics, commitment layout, privacy tier) MUST emit a new `RegistryBinding` fact in the canonical ledger before events using the new interpretation are admitted. This is the same rule that governs `construction_id` migration in the prior binding draft, applied to the registry-snapshot layer here.

---

## 15. Snapshot and Watermark Discipline

**Requirement class:** Derived Processor, Export Generator.

### 15.1 Core rule

Every derived artifact and every agency-log entry MUST carry a **watermark** `(tree_size, tree_head_hash)` identifying the canonical state from which it was derived, plus a deterministic **rebuild path** from the canonical chain to the derived artifact. Full-replay from `sequence = 0` is not a valid Phase 1 implementation for any system holding more than trivial quantities of case data — at case-file scale, full replay is operationally infeasible, and retrofitting snapshots invalidates every derived view already shipped.

This section owns the **core rule** (watermark + rebuild-path requirement). The **operational elaboration** — snapshot cadence, purge cascade after crypto-shredding, staleness handling, index rebuild guarantees — belongs to the Operational Companion. The core rule belongs here because the watermark is part of the canonical export and the rebuild path is part of the verification contract.

### 15.2 `Watermark`

```cddl
Watermark = {
  scope:          bstr,
  tree_size:      uint,
  tree_head_hash: digest,
  checkpoint_ref: digest,              ; checkpoint_digest (§11.2)
  built_at:       uint,                ; Unix seconds UTC when the artifact was built
  rebuild_path:   tstr,                ; implementation-defined deterministic identifier
}
```

A derived artifact's `Watermark` MUST be verifiable against the canonical chain: the named `checkpoint_ref` MUST exist in the export (or be resolvable against a referenced checkpoint chain), and the chain of events up to `tree_size` under that checkpoint's `tree_head_hash` MUST verify.

### 15.3 Rebuild path

`rebuild_path` is a deterministic identifier that, combined with the canonical events up to `tree_size` and with the declared configuration history of the derived processor, allows a recipient to rebuild the derived artifact and confirm byte-for-byte equivalence. The rebuild path is not a guarantee of performance; it is a guarantee that the derived artifact is not authoritative and can be regenerated.

### 15.4 Rule applies to agency-log entries

Phase 3 agency-log entries (§24) are themselves derived artifacts in the sense that they compose case-ledger heads with arrival metadata. They MUST carry a watermark. An agency log that cannot be rebuilt from the canonical underlying case ledgers is non-conformant — regardless of how well its own internal integrity is preserved.

---

## 16. Verification Independence Contract

**Requirement class:** Verifier, Export Generator.

### 16.1 Normative requirement

Export verification MUST NOT depend on:

- derived artifacts (projections, evaluator state, caches, indexes, timelines),
- workflow runtime state (task queues, orchestration state, session state),
- mutable service databases,
- live access to the producing service's APIs, beyond what the export package explicitly names as optional external proof material.

The verifier MUST be able to complete every obligation in §19 on an air-gapped laptop, given only the export ZIP and whatever optional external proof material (for example, a Bitcoin header bundle for OpenTimestamps anchor verification) the package names.

### 16.2 No live registry lookups

A Phase 1 verifier MUST NOT perform a live registry lookup to interpret an event. Registry meaning is bound per §14 at signing time and embedded in the export per §18.

### 16.3 Optional external anchors

A package MAY reference external anchoring (transparency log URL, Bitcoin block anchor, RFC 3161 TSA receipt). Such references MUST be marked explicitly as optional external proof material in the manifest and MUST NOT be required for baseline Phase 1 verification. A Phase 1 verifier that the package's signed tree head and consistency-proof material verify MUST return "verified" even if the external anchor cannot be fetched. Phase 4 deployments MAY register a deployment class that elevates external anchoring to required; absent such a class, anchoring is additive.

### 16.4 Omitted-payload honesty

If a package omits ciphertext bytes or payload readability material (because payloads are reader-held and the verifier is not a reader, or because payloads are intentionally redacted), the package MUST still verify the structure, signatures, provenance, and append claims that are verifiable from included bytes. The verification algorithm (§19) returns `structure_verified`, `integrity_verified`, and `readability_verified` separately, plus a list of omitted payload checks. A package that silently fails — that omits both material and the declaration of omission — is non-conformant.

---

## 17. Append Idempotency Contract

**Requirement class:** Fact Producer, Canonical Append Service, Verifier.

### 17.1 Why this is wire contract, not operator convention

Every `append` call carries a stable `idempotency_key`. Without a wire-contract idempotency semantic, every operator implements dedup locally, implementations diverge exactly at the boundary where they matter (network retry at submission), and interop between a primary and a second implementation is blocked. Idempotency at the envelope layer is the only construction that survives multiple operators composing one agency log.

### 17.2 `idempotency_key`

`idempotency_key` is a byte string of 1–64 bytes. It MAY be a UUIDv7 ([RFC 9562]), a stable caller-assigned identifier, or any value chosen by the Fact Producer so long as the producer guarantees that equivalent authored submissions produce equal keys.

The recommended convention is UUIDv7, because:

- it is globally unique without coordination,
- it encodes wall-time in the first bytes (useful for archival debugging),
- RFC 9562 pins its format, avoiding legacy UUIDv4 randomness variance.

Callers may substitute a deterministic hash of the authored fact's causal identity (for example, `SHA-256(session_id || field_path || proposed_value)`) if that better matches their retry semantics.

### 17.3 Resolution semantics

An idempotency identity is the pair `(ledger_scope, idempotency_key)`. The identity is permanent within that ledger scope. A Canonical Append Service MUST NOT reuse the same idempotency identity for a different authored payload after a clock interval, API TTL, dedup-store compaction, or operator lifecycle event. Retry budgets, API TTLs, and dedup-store lifecycle are operational policy owned by the Trellis Operational Companion; they do not relax this Core identity rule.

For a given `idempotency_key` within a declared ledger scope, a Canonical Append Service MUST resolve every successful retry to exactly one of:

1. **Same canonical reference.** The exact canonical event hash that was admitted on the first successful submission. The service returns the same `canonical_event_hash`, and the payload `content_hash` is byte-equal to the original.
2. **Declared no-op.** A successful retry against a key that was admitted but whose subsequent retry carries a payload that is byte-identical (post-dCBOR canonicalization) returns a structured no-op response referencing the original canonical event hash.
3. **Reject on conflict.** A retry that shares `idempotency_key` but whose payload would produce a different `content_hash`, `author_event_hash`, or `canonical_event_hash` MUST be rejected with the structured error `IdempotencyKeyPayloadMismatch` (§17.5). This is invariant #13 of the vision document, lifted to normative text: same key, different payload means deterministic rejection, auditable.

The service MUST NOT, on retry, create a new canonical order position with a different canonical event hash under the same `idempotency_key`. Duplication at the same `idempotency_key` with a different hash is undefined canonical order.

### 17.4 Operational retry policy boundary

Core defines the permanent idempotency identity and deterministic replay/rejection semantics. The Operational Companion defines retry budgets, API-facing TTLs, dedup-store retention lifecycle, and how operators document storage compaction. No operational policy may cause `(ledger_scope, idempotency_key)` to accept a different payload after any expiry.

### 17.5 Rejection codes

The following rejection codes are normative for Phase 1. Each is a structured, verifiable response, not a free-form error string.

| Code | Meaning |
|---|---|
| `IdempotencyKeyPayloadMismatch` | Same `(ledger_scope, idempotency_key)`, different payload/hash material — see §17.3. |
| `prev_hash_mismatch` | `prev_hash` does not match the predecessor's canonical event hash (§10.2). |
| `sequence_gap` | `sequence` is not `prev.sequence + 1`. |
| `unknown_suite_id` | `suite_id` is not registered (§7.2). |
| `unresolvable_kid` | COSE protected-header `kid` cannot be resolved in the active registry (§8.5). |
| `registry_digest_mismatch` | Event's bound registry does not match export manifest (§14). |
| `hash_construction_mismatch` | Event uses a hash construction not registered. |
| `missing_required_field` | Envelope is missing a required field. |
| `header_layer_violation` | A field appears in the wrong layer per §12.2. |
| `chain_break` | General category for chain-integrity failures at verification time. |

Additional codes MAY be registered in Phase 2+.

---

## 18. Export Package Layout

**Requirement class:** Export Generator (produces), Verifier (consumes).

### 18.1 Deterministic ZIP

An export package is a deterministic ZIP archive. "Deterministic" means:

- entries ordered in byte-wise lexicographic order of their UTF-8 filename,
- entry names are prefixed so the single lexicographic order is the required processing order,
- compression method is `STORED` (ZIP method 0) for every entry; DEFLATE is not conformant because library parameters are not deterministic across implementations,
- local file headers have extra-field length zero,
- file modification time is fixed to `1980-01-01T00:00:00Z`,
- external file attributes are zero,
- no ZIP64 unless the package exceeds 4 GiB,
- `000-manifest.cbor` appears first in the archive so that a truncated-read verifier can bail fast.

The archive name SHOULD follow the pattern `trellis-export-<scope>-<tree_size>-<short_head_hash>.zip`.

A conforming implementation can reproduce the archive layout with a `zip -0` style invocation over files already named with the prefixes below, provided it suppresses extra fields and normalizes timestamps and attributes as required above.

### 18.2 Required archive members

```
trellis-export-<scope>-<tree_size>-<shorthash>/
  000-manifest.cbor               ; §18.3 — COSE_Sign1 over ExportManifestPayload
  010-events.cbor                 ; §18.4 — dCBOR array of Event
  020-inclusion-proofs.cbor       ; §18.5 — dCBOR map leaf_index → proof
  025-consistency-proofs.cbor     ; §18.5 — dCBOR array of consistency proofs
  030-signing-key-registry.cbor   ; §8.5 — dCBOR array of SigningKeyEntry and LedgerServiceWrapEntry
  040-checkpoints.cbor            ; §18.6 — dCBOR array of Checkpoint
  050-registries/                 ; §14 — embedded domain-registry bytes
    <registry_digest_hex>.cbor    ; one file per distinct RegistryBinding
  060-payloads/                   ; OPTIONAL — encrypted payloads if inlined or included
    <content_hash_hex>.bin
  090-verify.sh                   ; §18.8 — self-contained verifier invocation
  098-README.md                   ; §18.9 — human-readable orientation
  099-trellis-verify-linux-x86_64 ; OPTIONAL — statically linked verifier binary
  099-trellis-verify-darwin-arm64 ; OPTIONAL — statically linked verifier binary
  099-trellis-verify-windows-x86_64.exe ; OPTIONAL — statically linked verifier binary
```

Files marked OPTIONAL may be omitted; a verifier MUST NOT fail solely on their absence.

### 18.3 `ExportManifest`

```cddl
SignedExportManifest = COSESign1Bytes ; payload is dCBOR(ExportManifestPayload)

ExportManifestPayload = {
  format:           tstr,                 ; "trellis-export/1"
  version:          uint .size 1,         ; = 1 for Phase 1
  generator:        tstr,                 ; generator identifier
  generated_at:     uint,                 ; Unix seconds UTC
  scope:            bstr,                 ; ledger scope
  tree_size:        uint,                 ; events covered
  head_checkpoint_digest: digest,         ; §11.2
  registry_bindings: [+ RegistryBinding], ; §14
  signing_key_registry_digest: digest,    ; SHA-256 of 030-signing-key-registry.cbor
  events_digest:    digest,               ; SHA-256 of 010-events.cbor
  checkpoints_digest: digest,             ; SHA-256 of 040-checkpoints.cbor
  inclusion_proofs_digest: digest,        ; SHA-256 of inclusion-proofs.cbor
  consistency_proofs_digest: digest,      ; SHA-256 of consistency-proofs.cbor
  payloads_inlined: bool,                 ; true if 060-payloads/ is present
  external_anchors: [* ExternalAnchor],   ; §16.3; optional
  posture_declaration: PostureDeclaration, ; §20
  head_format_version: uint,              ; §18.7; Phase 1 = 1
  omitted_payload_checks: [* OmittedPayloadCheck], ; §16.4, §19
  extensions:       { * tstr => any } / null,
}
```

The manifest binds every other archive member by digest. A verifier MUST check that every digest in the manifest matches the actual archive contents.

```cddl
OmittedPayloadCheck = {
  content_hash: digest,
  reason:       tstr,
}
```

### 18.4 `010-events.cbor`

A dCBOR array of `Event` COSE_Sign1 records in canonical order, starting at `sequence = 0` up to `sequence = tree_size - 1`. Concatenation and ordering are canonical; byte-match reproducibility is mandatory.

### 18.5 `inclusion-proofs.cbor` and `consistency-proofs.cbor`

- **Inclusion proofs.** A dCBOR map from `leaf_index` (uint) to `InclusionProof` under the final head. Every event in the export MUST have an inclusion proof.
- **Consistency proofs.** A dCBOR array of `ConsistencyProof` records linking each intermediate checkpoint to the next, plus a proof from the first checkpoint's head to the final head. This enables a verifier to confirm append-only growth without storing every intermediate tree.

```cddl
InclusionProof = {
  leaf_index:  uint,
  tree_size:   uint,
  leaf_hash:   digest,
  audit_path:  [* digest],
}
ConsistencyProof = {
  from_tree_size: uint,
  to_tree_size:   uint,
  proof_path:     [* digest],
}
```

### 18.6 `040-checkpoints.cbor`

A dCBOR array of all `Checkpoint` COSE_Sign1 records issued for this scope up to and including the final checkpoint of the export. Checkpoints are ordered by `tree_size` ascending. Each checkpoint payload's `prev_checkpoint_hash` MUST match the previous checkpoint's digest.

### 18.7 Head format version and superset commitment

The `head_format_version` field identifies the checkpoint / head format. Phase 1 ships version 1. Phase 3 case-ledger heads are a strict superset at a later version number: they preserve the Phase 1 `CheckpointPayload` fields unchanged and carry additional fields only in `CheckpointPayload.extensions` (§11.6). A Phase 1 verifier reading a later head format under a Phase-1-declared scope MAY return `unknown_head_format`, but it MUST NOT accept unknown top-level checkpoint fields as though they were Phase 1.

This is invariant #12 lifted normatively: **agency-log adoption is not a wire-format break for any Phase 1 export already in the field.**

### 18.8 `verify.sh`

`090-verify.sh` is a POSIX shell script that invokes the verifier binary appropriate for the current platform (detected via `uname`) and exits with `0` on verification success. The script MUST NOT require network access. Its full source SHOULD be at most a few dozen lines; its authority is the verifier binary, not the script.

### 18.9 `README.md`

`098-README.md` is a human-readable orientation file. Normative only in that it MUST state: the scope, the `tree_size`, the final `tree_head_hash`, the posture declaration (§20), which payload checks were omitted offline, and the verification invocation. It SHOULD NOT describe the export as legally admissible (§20.4).

---

## 19. Verification Algorithm

**Requirement class:** Verifier.

Given an export ZIP `E`, a verifier MUST implement the following algorithm. All steps MUST run without network access. Time and memory bounds: linear in the number of events for structure and integrity, O(log N) per inclusion proof, O(log N) per consistency proof. The output separates structure, ciphertext integrity, and payload readability because exports may intentionally omit ciphertext bytes or decryption material.

```text
VERIFY(E) -> VerificationReport

1. Open E as a deterministic ZIP (§18.1). If the ZIP layout is non-deterministic,
   abort with report.layout_error.

2. Read 000-manifest.cbor as SignedExportManifest. Verify its COSE_Sign1:
     a. Resolve protected-header kid via embedded 030-signing-key-registry.cbor.
        If unresolvable, abort with report.unresolvable_manifest_kid.
     b. Verify protected-header alg and suite_id are registered and consistent.
     c. Verify the RFC 9052 Sig_structure signature over the manifest payload.
        If invalid,
        abort with report.manifest_signature_invalid.
     d. Decode the COSE payload as ExportManifestPayload; reject unknown top-level fields.

3. Verify digests bound by the manifest:
     a. SHA-256(010-events.cbor) == manifest.events_digest
     b. SHA-256(040-checkpoints.cbor) == manifest.checkpoints_digest
     c. SHA-256(020-inclusion-proofs.cbor) == manifest.inclusion_proofs_digest
     d. SHA-256(025-consistency-proofs.cbor) == manifest.consistency_proofs_digest
     e. SHA-256(030-signing-key-registry.cbor) == manifest.signing_key_registry_digest
     f. For each RegistryBinding rb in manifest.registry_bindings:
          SHA-256(050-registries/<rb.registry_digest>.cbor) == rb.registry_digest
   Any mismatch ⇒ abort with report.archive_integrity_failure.

4. For each Event COSE_Sign1 e in 010-events.cbor (in order):
     a. Resolve protected-header kid via 030-signing-key-registry.cbor.
     b. Verify protected-header alg and suite_id, then verify the COSE Sig_structure (§7.4).
     c. Decode the COSE payload as EventPayload; reject unknown top-level fields.
     d. Recompute author_event_hash(payload) per §9.5. Check equals payload.author_event_hash.
     e. Recompute canonical_event_hash(payload) per §9.2.
     f. Check payload.ledger_scope == manifest.scope.
     g. If payload.payload_ref is PayloadInline:
          check SHA-256(payload_ref.ciphertext) under §9.3 == payload.content_hash.
        If payload.payload_ref is PayloadExternal and 060-payloads/<content_hash>.bin exists:
          check SHA-256(file bytes) under §9.3 == payload.content_hash.
        If payload.payload_ref is PayloadExternal and bytes are absent:
          record report.omitted_payload_checks[payload.content_hash] and continue
          with the remaining structure and chain checks for this event.
     h. If payload.sequence == 0: check payload.prev_hash == null. Else check
        payload.prev_hash == canonical_event_hash(events[payload.sequence - 1]).
     i. Check payload.causal_deps is null or [] (Phase 1 strict-linear, §10.3).
     j. Resolve the RegistryBinding applicable to payload.sequence per §14.4;
        check payload.header.event_type and related fields against the bound registry.
     k. On any failure, record in report.event_failures and continue — do NOT abort;
        the final verdict is false, but the report enumerates every failure.

5. For each Checkpoint COSE_Sign1 c in 040-checkpoints.cbor (in order):
     a. Resolve protected-header kid and verify COSE Sig_structure.
     b. Decode the COSE payload as CheckpointPayload; reject unknown top-level fields.
     c. Recompute Merkle root over canonical_event_hash(events[0..payload.tree_size])
        per §11.3. Check bit-equal to payload.tree_head_hash.
     d. If not the first checkpoint: check payload.prev_checkpoint_hash == digest of prior c.
     e. Verify consistency proof from prior c to current c (§11.4).
   Any failure ⇒ record in report.checkpoint_failures.

6. For each inclusion proof ip in 020-inclusion-proofs.cbor:
     a. Recompute Merkle root per ip.audit_path, ip.leaf_hash, ip.leaf_index.
     b. Check it matches the head checkpoint's tree_head_hash.
   Any failure ⇒ record in report.proof_failures.

7. If posture declaration indicates external anchoring:
     - IF the required external material is present: verify per §16.3.
     - IF external material is declared optional: skip without failure.
     - IF required but missing: record report.anchor_unresolved (NOT a verification failure
       under Phase 1, see §16.3, unless the posture declaration itself
       claims external anchoring is required).

8. Compute:
     structure_verified =
       manifest signature valid AND every COSE/CBOR/CDDL structure decoded and signed
       AND no unknown top-level Phase 1 fields were accepted.

     integrity_verified =
       archive digests valid AND event hashes, prev_hash links, checkpoint roots,
       inclusion proofs, consistency proofs, and every available ciphertext hash valid
       AND report.omitted_payload_checks is empty.

     readability_verified =
       every payload required by the export scope was decrypted and schema-validated
       under the bound registry and upstream Formspec/WOS semantics.

9. Return report with structure_verified, integrity_verified,
   readability_verified, failures, warnings, and omitted_payload_checks.
```

The verifier's output is a structured report enumerating every integrity observation. The overall convenience boolean MAY be computed as all three booleans true, but implementations MUST expose the three booleans independently. A package that omits ciphertext bytes can still be structurally verified, but it cannot claim payload integrity or readability were verified offline for the omitted payloads.

```cddl
VerificationReport = {
  structure_verified:   bool,
  integrity_verified:   bool,
  readability_verified: bool,
  event_failures:       [* VerificationFailure],
  checkpoint_failures:  [* VerificationFailure],
  proof_failures:       [* VerificationFailure],
  omitted_payload_checks: [* OmittedPayloadCheck],
  warnings:             [* tstr],
}

VerificationFailure = {
  location: tstr,
  code:     tstr,
  detail:   tstr,
}
```

### 19.1 Tamper evidence

When any verification boolean is false, the report identifies specifically **which** canonical bytes or payload checks do not reconcile. This is the "difficult and obvious" property: tampering that rewrites history after an export has been published is detectable by any verifier holding a prior export copy, because the tampered re-export's head will not be a consistent extension of the prior export's head (§11.4). The verifier does not require the tampering party to self-report; consistency-proof failure is the signal.

### 19.2 No network, no fallbacks

The verifier MUST NOT fetch external resources. It MUST NOT fall back to heuristic interpretation of malformed data. It MUST NOT silently skip checks that it lacks material to perform; it MUST record each skipped check in the report.

### 19.3 Time and memory

For an export with `N` events, the verifier runs in time `O(N)` for integrity (one pass over events, one pass over checkpoints) plus `O(M log N)` for the M inclusion proofs. Memory is `O(N)` in the worst case (the canonical-event-hash array to rebuild the tree), or `O(log N)` with streaming tree construction. A Phase 1 reference implementation that does not verify an N = 1,000,000 export on a laptop in under 60 seconds is not a good reference implementation; this is an engineering requirement, not a normative one.

---

## 20. Trust Posture Honesty

**Requirement class:** Fact Producer, Canonical Append Service, Export Generator.

### 20.1 Normative requirement

Implementations MUST NOT describe their trust posture more strongly than behavior supports. This is invariant #15 lifted normatively.

### 20.2 Required `PostureDeclaration`

Every export manifest (§18.3) MUST include a `PostureDeclaration` with at least the following fields:

```cddl
PostureDeclaration = {
  provider_readable:         bool,                   ; can the service operator decrypt
                                                    ; payloads in ordinary operation?
  reader_held:               bool,                   ; are decryption keys held by the subject or
                                                    ; a subject-designated reader?
  delegated_compute:         bool,                   ; is there any delegated-compute path that
                                                    ; exposes plaintext to a compute agent?
  external_anchor_required:  bool,                   ; does "tamper-evident" depend on an external
                                                    ; anchor beyond this package?
  external_anchor_name:      tstr / null,            ; if required, the name of the dependency
  recovery_without_user:     bool,                   ; can recovery occur without the user's
                                                    ; participation?
  metadata_leakage_summary:  tstr,                   ; human-readable short description of what
                                                    ; metadata remains visible
}
```

### 20.3 Honest field semantics

- `provider_readable = true` means the operator CAN decrypt payloads during ordinary operation. An implementation whose operator holds a reader-wrap copy in the `key_bag` MUST declare `true`. Declaring `false` under these circumstances is a trust-posture-honesty violation.
- `reader_held = true` means the subject or a subject-designated reader holds decryption capability, AND the operator does not hold it in ordinary operation. Both conditions are required; implementations that let the operator hold a "for emergencies" wrap MUST set `reader_held = false` unless the "emergency" is declared in `recovery_without_user` and in the Posture Declaration.
- `delegated_compute = false` means no compute agent, including AI services, is given decryption capability. An LLM-agent workflow that requires payload access to operate MUST declare `true`.
- `external_anchor_required = true` means the "tamper-evident" claim depends on the external anchor named in `external_anchor_name`. Packages that claim tamper-evidence via only the internal signed tree head MUST set `external_anchor_required = false`.
- `recovery_without_user = true` means the service can recover payload access without user participation. This is a key disclosure obligation.
- `metadata_leakage_summary` is non-normative prose; it MUST NOT be used to obscure any of the structured fields.

### 20.4 Legal claims

Cryptographic controls alone MUST NOT be described as legal admissibility. A Phase 1 export verifies integrity and provenance; legal sufficiency in any jurisdiction is governed by upstream obligations (WOS Assurance §6 and analogous regulatory regimes) and is outside the scope of this specification. An implementation MUST NOT embed in its export, its manifest prose, its README, or any accompanying documentation, a claim that the package's Phase 1 verification establishes legal admissibility.

### 20.5 Downgrade protocol

If a deployment discovers that it has overstated its posture (for example, an earlier declaration of `reader_held = true` when the operator had decryption capability), the correction is itself a canonical fact recorded in the response or case ledger, and an update to the posture-transition record. The deployment MUST NOT silently rewrite prior `PostureDeclaration` values; prior exports remain accurate as of their production time, and the correction is a forward event.

---

## 21. Posture / Custody / Conformance-Class Vocabulary

**Requirement class:** All.

### 21.1 The vocabulary problem

Three prior-draft namespaces used the letters A–E/F for three different concerns:

- the Respondent Ledger spec's Profile A/B/C (privacy × identity × integrity-anchoring posture),
- the legacy core draft's seven profiles (Core / Offline / Reader-Held / Delegated-Compute / Disclosure / User-Held / Respondent-History),
- the legacy companion draft's Profiles A–E (provider-readable / reader-held / delegated / threshold / organizational trust-custody models).

These are three orthogonal concerns and MUST NOT share a namespace.

### 21.2 Normative renames

- The Respondent Ledger spec unambiguously owns **"Profile A/B/C"** for posture axes (privacy tier × identity binding × integrity anchoring). Its definitions (Respondent Ledger §15A) are cited here; Trellis does not redefine them.
- The legacy core draft's profiles are renamed **"Conformance Classes"** (what they semantically are). This document defines them in §2.1.
- The legacy companion draft's profiles are renamed **"Custody Models"**. Their definitions move to the Operational Companion; Core only cites the identifier namespace.
- Trellis capability tiers use **phase names** — "attested-export tier" (Phase 1), "runtime-integrity tier" (Phase 2), "portable-case tier" (Phase 3), "federated tier" (Phase 4) — not letters.

### 21.3 Custody models enumerated

Phase 1 recognizes the following custody-model identifiers as values in the Custody Models registry (§26.3). Each identifier is a text string; semantics are defined by the Operational Companion §9.

- `CM-A` — Provider-readable.
- `CM-B` — Reader-held with recovery.
- `CM-C` — Delegated compute.
- `CM-D` — Threshold or quorum custody.
- `CM-E` — Organizational trust.
- `CM-F` — Client-origin sovereign / respondent-held custody.

Phase 2+ MAY register additional models. Registration is append-only; semantics do not change after registration.

### 21.4 Scope distinction: event / response ledger / case ledger / agency log / federation log

These are three distinct structures at three distinct scopes, not one term used three ways.

- **Event** (§1.2, §6) — atomic append.
- **Response ledger** — hash-chained sequence of events for one Formspec Response. Owned by Formspec Respondent Ledger semantics for authored meaning; owned by Trellis for envelope integrity.
- **Case ledger** — hash-chained sequence of governance events composing one or more sealed response-ledger heads with WOS governance events. Defined in §24.
- **Agency log** — operator-maintained log of case-ledger heads. Defined in §24.
- **Federation log** — log of agency-log heads witnessed by an independent operator. Phase 4, out of scope for this document.

---

## 22. Composition with Respondent Ledger

**Requirement class:** Fact Producer (Formspec-family), Verifier.

### 22.1 The seam

The Formspec Respondent Ledger specification (`specs/audit/respondent-ledger-spec.md`) defines the per-response event model, including `eventHash` and `priorEventHash` fields at §6.2 (per-event integrity chaining) and `LedgerCheckpoint` objects at §13 (per-range integrity checkpoints). Respondent Ledger §13.4 explicitly defers "specific signature suite or external anchor" to a downstream layer. Trellis is that downstream layer.

### 22.2 Per-event binding (Track E §21(a))

**When a Trellis envelope wraps a Respondent-Ledger event, Respondent Ledger §6.2 `eventHash` and `priorEventHash` are promoted from SHOULD to MUST.** Concretely:

- The Respondent-Ledger event, serialized per Respondent Ledger §14, MUST appear as the event's plaintext-committed authored-fact material (within the encrypted payload if `reader_held`; within the plaintext audit material if `provider_readable` per the declared posture).
- The Respondent-Ledger event's `eventHash` MUST equal the Trellis event's `canonical_event_hash` (§9.2).
- The Respondent-Ledger event's `priorEventHash` MUST equal the Trellis event's `prev_hash` (§6.2).

This is the integrity-chaining binding that the Respondent Ledger spec names but does not implement. Trellis implements it normatively for Trellis-wrapped Respondent-Ledger events.

### 22.3 Per-range binding (Track E §21(b))

Respondent Ledger §13 `LedgerCheckpoint` and Trellis §11 `Checkpoint` are **different hashes covering different scopes**. The binding between them:

- A Respondent-Ledger `LedgerCheckpoint` (§13.2 minimum fields: `checkpointId`, `ledgerId`, `fromSequence`, `toSequence`, `batchHash`, `signedAt`) is the per-range sealing artifact from the Formspec side.
- A Trellis `Checkpoint` (§11.2) is the signed tree head from the Trellis side.
- When a Trellis envelope wraps a Respondent Ledger, the Trellis `Checkpoint.tree_head_hash` MUST cover exactly the sequence range `[fromSequence, toSequence]` declared by the Respondent Ledger `LedgerCheckpoint`, and the Respondent Ledger `batchHash` MUST be reproducible from the canonical event hashes in that range under the construction of §11.3.

They are not the same hash, and MUST NOT be conflated; they attest to the same events at different layers (Formspec range vs. Trellis tree).

### 22.4 Case ledger as composition

The **case ledger** is defined normatively as: a Trellis-shaped hash-chained sequence of events whose admitted facts are (a) sealed response-ledger heads (one per Formspec Response associated with the case) plus (b) WOS governance events. The case-ledger event format IS the event format of §6. The composition rule: a sealed response-ledger head appears in a case ledger as an event of `event_type = "trellis.response-head"` whose payload references the sealed response head and whose `commitments` bind the response-ledger's final `tree_head_hash`. A WOS governance event appears as `event_type = "wos.*"` per the WOS-family event taxonomy.

Case-ledger head format is the head format of §11 at a version number (Phase 3) that is a strict superset of Phase 1's version 1 (§18.7).

### 22.5 Response → case composition rule

When a Formspec Response is sealed at submission:

1. The Respondent-Ledger final `LedgerCheckpoint` is produced per Respondent Ledger §13.
2. A Trellis `Checkpoint` is produced per §11 covering the same range.
3. The Trellis `Checkpoint.tree_head_hash` is bound into a new case-ledger event of type `trellis.response-head` whose payload references the Checkpoint.
4. The case ledger's `prev_hash` chain extends by one event.

This is the normative composition rule. It makes the response ledger a *named tributary* of the case ledger without redefining either.

---

## 23. Composition with WOS `custodyHook`

**Requirement class:** Fact Producer (WOS-family), Verifier.

### 23.1 The seam

WOS Kernel §10.5 names `custodyHook` as the seam through which a WOS runtime delegates cryptographic custody to a downstream layer. Trellis is that downstream layer for WOS deployments that adopt it.

### 23.2 Binding

A WOS runtime using Trellis as its custody backend MUST:

- Emit each provenance record (WOS Kernel §8 Facts tier) as a Trellis event of `event_type` drawn from the `wos.*` family in the bound registry (§14).
- Use Trellis `canonical_event_hash` (§9.2) as the provenance record's integrity hash wherever WOS Kernel §8 calls for one.
- Chain WOS provenance records via Trellis `prev_hash` per §10.2.
- Use Trellis `Checkpoint` (§11) for any WOS per-range integrity artifact.

### 23.3 Non-redefinition

Trellis does not alter WOS semantic authority. A WOS runtime's case-state model, deontic ruleset, autonomy caps, and governance logic remain WOS-spec bound. Trellis specifies only how the WOS record, once produced, is envelope-wrapped and integrity-bound. A WOS-conformant runtime and a Trellis-conformant canonical append service compose without either spec changing.

### 23.4 Delegation

When Trellis behavior depends on WOS evaluation semantics — whether a proposed state transition is permitted, whether a deontic check passes — Trellis MUST delegate to a WOS-conformant processor. Trellis does not evaluate WOS rules; it attests to the results WOS produces.

---

## 24. Agency Log (Phase 3 Superset Preview)

**Requirement class:** Canonical Append Service (for Phase 3 agency operators), Verifier.

### 24.1 Normative definition

An **agency log** is an operator-maintained append-only log whose entries are **case-ledger heads** plus arrival metadata and optional witness signatures. It proves that a case existed at time `T` and was not quietly deleted; it is structurally what CT logs are for certificates, applied to cases.

An agency-log entry is itself a Trellis event (§6) with:

- `event_type = "trellis.case-head"`,
- payload referencing the case-ledger head by digest,
- `commitments` (or header fields in Phase 3's declared construction) binding the case-scope metadata, the case-ledger's `tree_head_hash`, and optional witness cosignatures.

Agency-log heads are Trellis checkpoints (§11) at Phase 3's head format version. They preserve the Phase 1 checkpoint payload and carry Phase 3 additions in `CheckpointPayload.extensions`, per §11.6 and §18.7.

### 24.2 Phase 1 preservation obligation

Phase 1 MUST reserve head-format extension points that Phase 3 populates. The following Phase 3 fields extend the Phase 1 `CheckpointPayload` only inside `extensions`; a Phase 1 producer MUST NOT emit them, and a Phase 1 verifier MUST preserve and ignore them if encountered in a later-version head (§11.6, §18.7):

- `composed_response_heads: [* digest]` — references to sealed response-ledger heads composed into this case-ledger head.
- `case_scope_metadata: CaseScope` — arrival metadata (case ID, agency, adjudication phase).
- `witness_signatures: [* WitnessSignature]` — cosignatures by independent witnesses (Phase 4 federation).

### 24.3 Why this appears in Phase 1

The agency-log entry format is defined here, at Phase 1, because every Phase 1 checkpoint is a case-ledger-head-compatible artifact by construction. Without this normative reservation, Phase 3 agency-log adoption is a wire-format break for every Phase 1 export already in the field. By naming the reservations now, Phase 1 implementors know what Phase 3 will add and can validate their export shape against it.

### 24.4 Non-goal for Phase 1

Phase 1 does not specify agency-log gossip protocols, witness cosignature mechanics, or federation-log equivocation detection. Those belong to Phase 4. What Phase 1 guarantees is that when they arrive, they do not invalidate anything Phase 1 shipped.

---

## 25. Security and Privacy Considerations

**Requirement class:** Implementers.

### 25.1 Threat model

Trellis is designed against an adversary who controls the operating service, its database, its backup infrastructure, and its administrative console, but does **not** hold signing keys that have been distributed outside the service boundary (subject devices, independent witnesses, customer-held root keys). The goal is to make tampering by such an adversary **difficult and obvious**: difficult because it requires forging signatures, obvious because any reissued export's head is inconsistent with any prior export's head already in third-party hands.

This is not equivocation-proof. An adversary who never distributes exports externally, and who holds all signing keys, can rewrite history silently. Phase 4 transparency witnessing (out of scope for this document) raises the bar to equivocation-proof by requiring independent cosignature; Phase 1 ships the construction that makes Phase 4 additive, not reconstructive.

### 25.2 Metadata leakage

The envelope carries structural fields in plaintext (§12.2). An observer who sees envelopes without decrypting payloads learns:

- event type (limited by the outcome-neutral granularity rule, §12.4),
- timing (at `authored_at` granularity, which MAY be coarsened by the Operational Companion),
- COSE protected-header `kid` (which identifies the signer cohort but not, without correlation, the signer),
- ledger scope identifier (which may identify a case, a workflow, an agency),
- append-head position (which leaks cohort size for the scope).

Implementers MUST consult the metadata budget discipline of the Operational Companion before publishing envelopes to untrusted observers.

### 25.3 Equivocation and split-view

A service presenting different signed tree heads for the same scope to different verifiers is equivocating. Phase 1 detection is passive: verifiers holding different exports that share a scope can compare and detect divergence. Phase 4 elevates this to active detection via witness gossip. A Phase 1 deployment that does NOT distribute exports widely (for example, a single-tenant deployment) has weaker equivocation detection than one that does; this is a property of the deployment, not of the specification.

### 25.4 Side channels

Timing, access pattern, and inclusion-proof request patterns reveal information beyond what is in the envelope. Phase 1 does not mandate oblivious retrieval; deployments that require it declare so in their Posture Declaration and Operational Companion custody model.

### 25.5 Replay

`idempotency_key` (§17) prevents replay-as-duplicate. Replay-as-resubmission (a Fact Producer resubmits an event with the same key to force a retry) is handled by §17.3. Replay of an event from one ledger into another is prevented by the signed `ledger_scope`, the per-scope `prev_hash` chain, and the canonical event hash preimage (§9.2): an event hash includes the ledger scope, and the predecessor chain does not reproduce across scopes.

### 25.6 Key compromise

A compromised signing key invalidates every unrevoked signature the attacker can produce under it. Revocation (status → `Revoked`) is append-only; the compromised key MUST appear as `Revoked` in every subsequent export. Historical signatures under the key remain verifiable against the signing-key registry, but their interpretation depends on whether the compromise predates the signed event; this is a judgment the verifier cannot make from the envelope alone, and SHOULD be surfaced explicitly in the verification report (§19).

### 25.7 Post-quantum migration

Phase 1 ships Ed25519. A cryptographically-relevant quantum computer breaks Ed25519. The migration obligation (§7.3) requires that a 2045 verifier can still resolve 2026 signatures: the `suite_id` is in-band, the signing keys are in the registry, and the digest domain-separators are versioned. Migration to ML-DSA-65 or SLH-DSA-128s, or to a hybrid, registers a new `suite_id` and a new signing key; all prior records remain verifiable under their original suite by any verifier that retains Ed25519 validation code. Implementations that remove Ed25519 support after migration break their own history.

### 25.8 Crypto-shredding interaction with backups

Destroying the per-subject DEK erases plaintext locally. If the ciphertext has been backed up to off-site storage, the backup's plaintext is also erased (same DEK). This is the intended GDPR Art. 17 / FOIA-redaction behavior. An attacker with offline ciphertext and a quantum computer could theoretically recover plaintext if the AEAD succumbs to Shor or Grover; ChaCha20-Poly1305 AEAD has 256-bit key material and is believed to retain at least 128-bit post-quantum security under Grover. Against that adversary, crypto-shredding reduces to rotating the DEK class wholesale; this is a Phase 2 operational choice and does not affect Phase 1 wire format.

---

## 26. IANA Considerations

### 26.1 Content type

This specification requests registration of the media type `application/trellis-export+zip` for Phase 1 export packages. File extension: `.ztrellis` or `.zip`. Packages internally identify themselves via the `000-manifest.cbor` payload format field (`"trellis-export/1"`).

### 26.2 `suite_id` registry

A new IANA registry `Trellis Signature Suites` is requested. Registration policy: Specification Required. Initial contents per §7.2. Each registration MUST include: suite identifier, signature algorithm, digest algorithm, reference specification, status.

### 26.3 Custody Models registry

A new IANA-style sub-registry (maintained by the Trellis maintainers until IANA registration, under the shared governance of the Formspec/WOS/Trellis working group) tracks custody-model identifiers per §21.3.

### 26.4 Domain tags

The domain-separation tags enumerated in §9.8 are Trellis-internal and do not require IANA registration; they are documented here for implementor reference.

### 26.5 CBOR tag

No new CBOR tag is requested for Phase 1. Events and checkpoints are plain dCBOR maps; their type is established by their format member (`manifest.format`, explicit `version` fields) and by their archive placement.

---

## 27. Test Vector Requirements

**Requirement class:** All (fixture authorship), Verifier (conformance demonstration).

### 27.1 Coverage minimum

A conformant Phase 1 implementation MUST pass a minimum of **50 language-neutral test vectors** distributed across the following categories, placed under `fixtures/vectors/` in the reference distribution:

| Directory | Count minimum | Purpose |
|---|---|---|
| `fixtures/vectors/append/` | 10 | Valid append flows: first event, subsequent events, multi-event chains. |
| `fixtures/vectors/verify/` | 15 | Successful verification of complete exports: small, medium, large; single-scope; reader-held; provider-readable. |
| `fixtures/vectors/export/` | 10 | Export-package determinism: same input produces byte-identical ZIP output. |
| `fixtures/vectors/tamper/` | 15 | Detection of tampering: flipped bits in events.cbor, modified signatures, rewritten checkpoints, broken prev_hash chain, unresolvable kid, registry-digest mismatch, out-of-order checkpoints, consistency-proof forgery, substituted content, stripped inclusion proofs. |

### 27.2 Per-vector requirements

Each vector MUST include:

- input: the authored facts (plaintext), the signing keys, the registry snapshot, the idempotency keys, the timestamps,
- expected output: the exact dCBOR bytes of every event, the exact bytes of the checkpoint, the exact ZIP bytes of the export,
- expected verifier verdict: the exact `verified` boolean and a reference `VerificationReport`.

### 27.3 Byte-level claim coverage

Every byte-level claim in this specification MUST correspond to at least one vector. Concretely:

- every `suite_id` value used in Phase 1 has at least one vector,
- every rejection code in §17.5 has at least one negative-case vector,
- every commitment-scheme reservation in §13.3 has at least a shape-vector (not a full scheme implementation),
- every header layer-violation case in §12.2 has at least one negative-case vector.

### 27.4 Cross-implementation byte match

The Phase 1 success criterion (§1 Status) is that a second implementation, written from this specification alone, produces byte-identical output on every `export/` vector and byte-identical verification reports on every `verify/` and `tamper/` vector. Divergence at the byte level — even one-byte divergence — is a conformance failure in one of the two implementations and MUST be diagnosed against this specification, not papered over.

---

## 28. Appendix A — Full CDDL

```cddl
; Trellis Core Phase 1 CDDL grammar
; All types encoded as dCBOR (RFC 8949 §4.2.2).

digest     = bstr .size 32      ; SHA-256
suite_id   = uint
kid        = bstr .size 16
timestamp  = uint               ; Unix seconds UTC

; --- Event ------------------------------------------------------------

Event = COSESign1Bytes

EventPayload = {
  version:           uint .size 1,
  ledger_scope:      bstr,
  sequence:          uint,
  prev_hash:         digest / null,
  causal_deps:       [* digest] / null,
  author_event_hash: digest,
  content_hash:      digest,
  header:            EventHeader,
  commitments:       [* Commitment] / null,
  payload_ref:       PayloadRef,
  key_bag:           KeyBag,
  idempotency_key:   bstr .size (1..64),
  extensions:        { * tstr => any } / null,
}

EventHeader = {
  event_type:             bstr,
  authored_at:            timestamp,
  retention_tier:         uint .size 1,
  classification:         bstr,
  outcome_commitment:     digest / null,
  subject_ref_commitment: digest / null,
  tag_commitment:         digest / null,
  witness_ref:            bstr / null,
  extensions:             { * tstr => any } / null,
}

CheckpointHashPreimage = {
  version:            uint .size 1,
  scope:              bstr,
  checkpoint_payload: CheckpointPayload,
}

PayloadRef = PayloadInline / PayloadExternal

PayloadInline = {
  ref_type:   "inline",
  ciphertext: bstr,
  nonce:      bstr,
}

PayloadExternal = {
  ref_type:       "external",
  content_hash:   digest,
  availability:   AvailabilityHint,
  retrieval_hint: tstr / null,
}

AvailabilityHint = &(
  InExport:    0,
  External:    1,
  Withheld:    2,
  Unavailable: 3,
)

Commitment = {
  scheme:   uint,
  slot:     uint,
  value:    bstr,
  metadata: bstr / null,
}

KeyBag = {
  entries: [* KeyBagEntry],
}

KeyBagEntry = {
  recipient:        bstr,
  suite:            uint,
  ephemeral_pubkey: bstr .size 32,
  wrapped_dek:      bstr,
}

; --- Signature --------------------------------------------------------

COSESign1Bytes = bstr   ; RFC 9052 COSE_Sign1 tagged CBOR value as bytes.
                       ; Protected headers carry alg, kid, suite_id.

CanonicalEventHashPreimage = {
  version:       uint .size 1,
  ledger_scope:  bstr,
  event_payload: EventPayload,
}

AuthorEventHashPreimage = {
  version:         uint .size 1,
  ledger_scope:    bstr,
  sequence:        uint,
  prev_hash:       digest / null,
  causal_deps:     [* digest] / null,
  content_hash:    digest,
  header:          EventHeader,
  commitments:     [* Commitment] / null,
  payload_ref:     PayloadRef,
  key_bag:         KeyBag,
  idempotency_key: bstr .size (1..64),
  extensions:      { * tstr => any } / null,
}

; --- Signing-Key Registry --------------------------------------------

SigningKeyEntry = {
  kid:          kid,
  pubkey:       bstr,
  suite_id:     suite_id,
  status:       SigningKeyStatus,
  valid_from:   timestamp,
  valid_to:     timestamp / null,
  supersedes:   kid / null,
  attestation:  bstr / null,
}

SigningKeyStatus = &(
  Active:   0,
  Rotating: 1,
  Retired:  2,
  Revoked:  3,
)

LedgerServiceWrapEntry = {
  ledger_id:         bstr,
  author_event_hash: digest,
  lak_version:       uint,
  ephemeral_pubkey:  bstr .size 32,
  wrapped_dek:       bstr,
  created_at:        timestamp,
  signature:         COSESign1Bytes,
}

; --- Checkpoint -------------------------------------------------------

Checkpoint = COSESign1Bytes

CheckpointPayload = {
  version:                uint .size 1,
  scope:                  bstr,
  tree_size:              uint,
  tree_head_hash:         digest,
  timestamp:              timestamp,
  anchor_ref:             bstr / null,
  prev_checkpoint_hash:   digest / null,
  extensions:             { * tstr => any } / null,
}

InclusionProof = {
  leaf_index:  uint,
  tree_size:   uint,
  leaf_hash:   digest,
  audit_path:  [* digest],
}

ConsistencyProof = {
  from_tree_size: uint,
  to_tree_size:   uint,
  proof_path:     [* digest],
}

; --- Export Manifest --------------------------------------------------

SignedExportManifest = COSESign1Bytes

ExportManifestPayload = {
  format:                      tstr,         ; "trellis-export/1"
  version:                     uint .size 1,
  generator:                   tstr,
  generated_at:                timestamp,
  scope:                       bstr,
  tree_size:                   uint,
  head_checkpoint_digest:      digest,
  registry_bindings:           [+ RegistryBinding],
  signing_key_registry_digest: digest,
  events_digest:               digest,
  checkpoints_digest:          digest,
  inclusion_proofs_digest:     digest,
  consistency_proofs_digest:   digest,
  payloads_inlined:            bool,
  external_anchors:            [* ExternalAnchor],
  posture_declaration:         PostureDeclaration,
  head_format_version:         uint,
  omitted_payload_checks:      [* OmittedPayloadCheck],
  extensions:                  { * tstr => any } / null,
}

ExportManifestHashPreimage = {
  version:          uint .size 1,
  scope:            bstr,
  manifest_payload: ExportManifestPayload,
}

OmittedPayloadCheck = {
  content_hash: digest,
  reason:       tstr,
}

RegistryBinding = {
  registry_digest:   digest,
  registry_format:   uint,
  registry_version:  tstr,
  bound_at_sequence: uint,
}

ExternalAnchor = {
  kind:         tstr,
  anchor_ref:   bstr,
  required:     bool,
  description:  tstr,
}

PostureDeclaration = {
  provider_readable:        bool,
  reader_held:              bool,
  delegated_compute:        bool,
  external_anchor_required: bool,
  external_anchor_name:     tstr / null,
  recovery_without_user:    bool,
  metadata_leakage_summary: tstr,
}

; --- Watermark --------------------------------------------------------

Watermark = {
  scope:           bstr,
  tree_size:       uint,
  tree_head_hash:  digest,
  checkpoint_ref:  digest,
  built_at:        timestamp,
  rebuild_path:    tstr,
}

VerificationReport = {
  structure_verified:     bool,
  integrity_verified:     bool,
  readability_verified:   bool,
  event_failures:         [* VerificationFailure],
  checkpoint_failures:    [* VerificationFailure],
  proof_failures:         [* VerificationFailure],
  omitted_payload_checks: [* OmittedPayloadCheck],
  warnings:               [* tstr],
}

VerificationFailure = {
  location: tstr,
  code:     tstr,
  detail:   tstr,
}
```

---

## 29. Appendix B — Example Events and Exports

*This appendix is informative; the authoritative grammar is Appendix A and the authoritative bytes are the fixture vectors (§27).*

### 29.1 A minimal first event (hex-decoded dCBOR)

A Formspec `session.started` event, the first in a response ledger. Values are schematic: digest bytes truncated to `01 02 03 …` placeholders; real fixtures will use actual cryptographic outputs.

Decoded COSE payload structure:

```
EventPayload {
  version: 1,
  ledger_scope: h'7265732d303030303031',   ; "res-000001"
  sequence: 0,
  prev_hash: null,
  causal_deps: null,
  author_event_hash: h'01010101...32bytes',
  content_hash: h'02020202...32bytes',     ; SHA-256 over ciphertext
  header: {
    event_type: h'666f726d737065632e617574686f726564', ; "formspec.authored" bytes
    authored_at: 1744963200,               ; 2026-04-18T00:00:00Z
    retention_tier: 1,
    classification: h'7075626c6963',       ; "public"
    outcome_commitment: null,
    subject_ref_commitment: h'03030303...32bytes',
    tag_commitment: null,
    witness_ref: null,
    extensions: null
  },
  commitments: null,
  payload_ref: {
    ref_type: "inline",
    ciphertext: h'baadf00d...N-bytes',
    nonce: h'06060606...12bytes'
  },
  key_bag: {
    entries: [
      {
        recipient: h'7265616465722d686f6c64', ; "reader-hold"
        suite: 1,                             ; HPKE Base X25519 + HKDF-SHA256 + ChaCha20-Poly1305
        ephemeral_pubkey: h'04040404...32bytes',
        wrapped_dek: h'05050505...N-bytes'
      }
    ]
  },
  idempotency_key: h'0189b2c0...16bytes',  ; UUIDv7
  extensions: null
}
```

The wire event is a COSE_Sign1 value whose protected header includes `alg = -8`, `kid = h'deadbeef...16bytes'`, and `suite_id = 1`, and whose payload is the dCBOR bytes above.

Hex dump of the EventPayload dCBOR serialization (first bytes shown schematic):

```
aD                           ; map(13) — the EventPayload map
  67 76 65 72 73 69 6f 6e    ; "version"
  01                          ; 1
  6c 6c 65 64 67 65 72 ...    ; "ledger_scope"
  4a 72 65 73 2d ...          ; h'res-...'
  68 73 65 71 75 65 6e 63 65 ; "sequence"
  00                          ; 0
  ...                        ; remaining fields in lexicographic key order
```

(Full byte-level dump is in `fixtures/vectors/append/001-minimal-first-event.cbor.hex`.)

### 29.2 A signed checkpoint

```
CheckpointPayload {
  version: 1,
  scope: h'7265732d303030303031',          ; "res-000001"
  tree_size: 7,
  tree_head_hash: h'aabbccdd...32bytes',
  timestamp: 1744974000,
  anchor_ref: null,
  prev_checkpoint_hash: null,              ; first checkpoint of the scope
  extensions: null
}
```

The wire checkpoint is a COSE_Sign1 value over this payload with protected-header `alg`, `kid`, and `suite_id`.

### 29.3 Export manifest

```
ExportManifestPayload {
  format: "trellis-export/1",
  version: 1,
  generator: "trellis-cli/0.1.0",
  generated_at: 1744974100,
  scope: h'7265732d303030303031',
  tree_size: 7,
  head_checkpoint_digest: h'ffeeddcc...32bytes',
  registry_bindings: [
    {
      registry_digest: h'11223344...32bytes',
      registry_format: 1,
      registry_version: "2026.04",
      bound_at_sequence: 0
    }
  ],
  signing_key_registry_digest: h'...',
  events_digest: h'...',
  checkpoints_digest: h'...',
  inclusion_proofs_digest: h'...',
  consistency_proofs_digest: h'...',
  payloads_inlined: true,
  external_anchors: [],
  posture_declaration: {
    provider_readable: false,
    reader_held: true,
    delegated_compute: false,
    external_anchor_required: false,
    external_anchor_name: null,
    recovery_without_user: false,
    metadata_leakage_summary: "Envelope reveals event_type, authored_at (1s granularity), retention_tier, classification, and protected-header kid. Outcome, subject, and tags are committed, not plaintext."
  },
  head_format_version: 1,
  omitted_payload_checks: [],
  extensions: null
}
```

The wire `000-manifest.cbor` member is a COSE_Sign1 value over this payload with protected-header `alg`, `kid`, and `suite_id`.

### 29.4 Worked verification trace

Given `trellis-export-res-000001-7-ffeeddcc.zip` (the export of §29.1–29.3), a verifier runs:

```
1. Open ZIP. Layout deterministic? yes.
2. Read 000-manifest.cbor. Verify COSE_Sign1 ⇒ valid.
3. Check archive digests:
     SHA-256(010-events.cbor)       = manifest.events_digest        ✓
     SHA-256(040-checkpoints.cbor)  = manifest.checkpoints_digest   ✓
     (etc.)
4. For each event e_0..e_6:
     verify signature          ✓
     recompute canonical_event_hash ✓
     recompute author_event_hash   ✓
     verify content_hash       ✓
     verify prev_hash chain    ✓
     causal_deps null          ✓ (Phase 1)
     registry binding resolves ✓
5. For the single checkpoint c_7:
     verify signature          ✓
     recompute Merkle root     ✓ = c_7.tree_head_hash
     prev_checkpoint_hash null ✓ (first checkpoint)
6. For each inclusion proof:
     recompute root            ✓
7. No external anchors declared; skip.
8. verified = true.
9. Return (true, report with 0 failures).
```

A tamper fixture `fixtures/vectors/tamper/005-flipped-byte-in-event-3.zip` modifies one byte of `events[3].payload_ref.ciphertext`. On re-run:

```
4. Event e_3: content_hash mismatch ⇒ report.event_failures[3] = content_hash_mismatch
            : author_event_hash mismatch (covers ciphertext) ⇒ report.event_failures[3] += author_event_hash_mismatch
            : signature verification fails ⇒ report.event_failures[3] += signature_invalid
5. checkpoint c_7: Merkle root mismatch ⇒ report.checkpoint_failures[0] = tree_head_hash_mismatch
   (because event e_3's canonical_event_hash changed)
8. verified = false.
```

The report localizes the tamper to sequence 3 and to the ciphertext field specifically, across three independent checks.

---

## 30. Traceability Anchors

This non-normative section anchors the traceability matrix rows that correspond to Core obligations. The prose in §§1–29 is normative; `TR-CORE-*` rows in `trellis-requirements-matrix.md` are traceability aids and must be corrected if they conflict with this document.

Core traceability rows:

- TR-CORE-001, TR-CORE-002, TR-CORE-003, TR-CORE-004, TR-CORE-005, TR-CORE-006, TR-CORE-007
- TR-CORE-010, TR-CORE-011, TR-CORE-012, TR-CORE-013, TR-CORE-014, TR-CORE-015, TR-CORE-016, TR-CORE-017
- TR-CORE-020, TR-CORE-021, TR-CORE-022, TR-CORE-023, TR-CORE-024, TR-CORE-025
- TR-CORE-030, TR-CORE-031, TR-CORE-032, TR-CORE-035, TR-CORE-036, TR-CORE-037, TR-CORE-038
- TR-CORE-040, TR-CORE-041, TR-CORE-042, TR-CORE-043, TR-CORE-044, TR-CORE-045, TR-CORE-046
- TR-CORE-050, TR-CORE-051, TR-CORE-052, TR-CORE-053
- TR-CORE-060, TR-CORE-061, TR-CORE-062, TR-CORE-063, TR-CORE-064, TR-CORE-065, TR-CORE-066, TR-CORE-067
- TR-CORE-070, TR-CORE-071, TR-CORE-072
- TR-CORE-080, TR-CORE-081, TR-CORE-082
- TR-CORE-090, TR-CORE-091
- TR-CORE-100, TR-CORE-101, TR-CORE-102, TR-CORE-103
- TR-CORE-110, TR-CORE-111, TR-CORE-112, TR-CORE-113
- TR-CORE-120, TR-CORE-121, TR-CORE-122, TR-CORE-123, TR-CORE-124, TR-CORE-125, TR-CORE-126
- TR-CORE-130, TR-CORE-131, TR-CORE-132, TR-CORE-133, TR-CORE-134
- TR-CORE-140, TR-CORE-141, TR-CORE-142, TR-CORE-143

## 31. References

### 31.1 Normative references

- **[RFC 2119]** Bradner, S., "Key words for use in RFCs to Indicate Requirement Levels", BCP 14, RFC 2119, March 1997.
- **[RFC 8174]** Leiba, B., "Ambiguity of Uppercase vs Lowercase in RFC 2119 Key Words", BCP 14, RFC 8174, May 2017.
- **[RFC 8032]** Josefsson, S., Liusvaara, I., "Edwards-Curve Digital Signature Algorithm (EdDSA)", RFC 8032, January 2017.
- **[RFC 8610]** Birkholz, H., Vigano, C., Bormann, C., "Concise Data Definition Language (CDDL)", RFC 8610, June 2019.
- **[RFC 8949]** Bormann, C., Hoffman, P., "Concise Binary Object Representation (CBOR)", STD 94, RFC 8949, December 2020.
- **[RFC 9052]** Schaad, J., "CBOR Object Signing and Encryption (COSE): Structures and Process", STD 96, RFC 9052, August 2022.
- **[RFC 9180]** Barnes, R., Bhargavan, K., Lipp, B., Wood, C., "Hybrid Public Key Encryption", RFC 9180, February 2022.
- **[RFC 9562]** Davis, K., Peabody, B., Leach, P., "Universally Unique IDentifiers (UUIDs)", RFC 9562, May 2024.
- **[RFC 6962]** Laurie, B., Langley, A., Kasper, E., "Certificate Transparency", RFC 6962, June 2013. (Merkle tree construction model.)
- **[FIPS 180-4]** NIST, "Secure Hash Standard (SHS)", FIPS PUB 180-4, August 2015.
- **[Formspec Core]** Formspec Working Group, "Formspec Core Specification v1.0".
- **[Formspec Respondent Ledger]** Formspec Working Group, "Formspec Respondent Ledger Specification v0.1" (`specs/audit/respondent-ledger-spec.md`). Cited at §§22.2–22.5 for event and checkpoint binding.
- **[WOS Kernel]** WOS Working Group, "WOS Kernel Specification v1.0". Cited at §§23.1–23.4 for `custodyHook` binding.

### 31.2 Informative references

- **[FIPS 204]** NIST, "Module-Lattice-Based Digital Signature Standard (ML-DSA)", FIPS 204, August 2024. Reserved for Phase 2+ post-quantum `suite_id`.
- **[FIPS 205]** NIST, "Stateless Hash-Based Digital Signature Standard (SLH-DSA)", FIPS 205, August 2024. Reserved for Phase 2+ post-quantum `suite_id`.
- **[WOS Assurance]** WOS Working Group, "WOS Assurance Specification". Referenced for legal-sufficiency disclosure obligations (§20.4).
- **Trellis Operational Companion (Phase 2)** — separate normative document for projection and derived-artifact discipline, metadata-budget declarations, delegated-compute honesty, posture-transition auditability, snapshot watermarks, and rebuild semantics.
- **Formspec/WOS/Trellis Product Vision** — `thoughts/product-vision.md`, 2026-04-17. Phase roadmap and Phase 1 envelope invariants #1–#15.
