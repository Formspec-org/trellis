# Derivation — `append/006-custody-transition-cm-b-to-cm-a`

## Header

**What this vector exercises.** First Stream D / O-5 posture-transition vector. It pins the wire shape of a `trellis.custody-model-transition.v1` Posture-transition event per Core §6.7 and Companion Appendix A.5.1. The transition widens the provider-readable decryptor set (CM-B → CM-A), which is a Posture-expanding change under OC-11: Companion §10.4 / Appendix A.5.3 step 4 therefore require *dual* attestation — one `authority_class="prior"` signature from the retiring CM-B authority, one `authority_class="new"` signature from the incoming CM-A authority. The `declaration_doc_digest` field binds the event to a freshly published Posture Declaration representing the state in force AFTER the transition per OC-15 / Decision 1 of the design brief (`thoughts/specs/2026-04-18-trellis-o5-posture-transition-schemas.md`).

**Scope of this vector.**

1. Fact-producer side only (`append` op). Verifier-side observations of the same transition live in the sibling tamper vectors 002-004.
2. Single non-genesis event. Chains from `append/001-minimal-inline-payload` with `sequence = 1`. The transition payload rides inside `EventPayload.extensions` (Core §6.5 strict-superset semantics); the surrounding event envelope is identical in shape to 001's and to 005's.
3. No export manifest, no checkpoint, no inclusion proof. Core §19's state-continuity check (step 6.b) is exercised by tamper/002; this vector's job is to produce the canonical bytes a conforming verifier would see walking the chain normally.

**Core / Companion § roadmap (in traversal order).**

1. Core §6.7 — the `EventPayload.extensions` registry entry for `trellis.custody-model-transition.v1` (Phase 1; reject-if-unknown-at-version).
2. Companion Appendix A.5.1 — the CDDL for the payload riding in that slot.
3. Companion Appendix A.5 shared rule — the `Attestation` struct shape.
4. Core §9.1 + §9.8 — domain-separation discipline for `declaration_doc_digest` and attestation signatures (`trellis-posture-declaration-v1`, `trellis-transition-attestation-v1`).
5. Companion §10.4 / OC-11 — the dual-attestation obligation for expansions.
6. Core §6.1 / §6.8 — the three event surfaces (authored, canonical, signed).
7. Core §9.5 / §9.2 — author_event_hash and canonical_event_hash construction.
8. Core §7.1 / §7.4 — COSE_Sign1 envelope.
9. Core §10.6 — AppendHead post-append state.

---

## Body

### Step 1 — Resolve the prior head (§10.2, §10.6)

`append/006` chains from `append/001`'s head. The `AppendHead` from 001 carries `scope = "test-response-ledger"`, `sequence = 0`, and a 32-byte `canonical_event_hash`. Core §10.2 requires `prev_hash` of event `sequence = N` to equal `canonical_event_hash` of event `sequence = N - 1` in the same scope. The generator copies `../001-minimal-inline-payload/expected-append-head.cbor` verbatim into this vector as `input-prior-append-head.cbor` so the vector is self-contained for stranger-test review.

**`prev_hash` = `ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb`** (= 001's `canonical_event_hash`).

### Step 2 — Build the post-transition PostureDeclaration preimage (Companion §11, Appendix A.1)

Companion §11 defines the PostureDeclaration fields in prose; Appendix A.1 gives a structural sketch. The declaration's interior CDDL is not normatively pinned (flagged in `thoughts/specs/2026-04-18-trellis-core-gaps-surfaced-by-g3.md` if the doc exists; not load-bearing for this vector). Per Core §19 step 6.c the verifier's obligation is to **recompute** `declaration_doc_digest` under domain tag `trellis-posture-declaration-v1` and compare byte-for-byte to the `declaration_doc_digest` field; it does not parse the declaration interior. The generator therefore emits a minimal dCBOR map with the Appendix A.1 named fields so a reviewer can read the bytes, and computes the digest over them.

The declaration describes the state **after** the transition: `custody_model = CM-A`. It also carries a `supersedes` link identifying the prior declaration, consistent with OC-09's no-silent-overwrite rule — the prior declaration remains addressable; this one is an additive successor.

- `input-posture-declaration.bin` = `dCBOR(PostureDeclaration{scope, custody_model=CM-A, disclosure_profile=rl-profile-A, effective_from, supersedes, posture_honesty_statement, declaration_id, operator_id})` — 320 bytes.
- `declaration_doc_digest = SHA-256(domain_preimage("trellis-posture-declaration-v1", <declaration bytes>))` per Core §9.1. = **`fced7843312f6fd3f9a61e10068f9cd8d7a095d479278a1307d4b584d66aa3e1`** (32 bytes).

Domain-separation preimage shape (Core §9.1): `len(tag_utf8).to_bytes(4, "big") || tag_utf8 || len(component).to_bytes(4, "big") || component`. The tag is `"trellis-posture-declaration-v1"` (Core §9.8 registry entry).

### Step 3 — Build both attestations (Companion Appendix A.5 shared rule + §10.4)

OC-11 Posture-widening rule: CM-B → CM-A widens the provider-readable decryptor set (CM-A is Provider-Readable Custodial per Companion §9.2), so this transition **requires** dual attestation.

The shared `Attestation` rule in Companion Appendix A.5 pins the signature preimage:

> `attestation.signature = Ed25519(sk, domain_preimage("trellis-transition-attestation-v1", dCBOR([transition_id, effective_at, authority_class])))`

Two attestation preimages are therefore emitted, one per `authority_class`:

| authority_class | preimage bytes (dCBOR array) | file |
|---|---|---|
| `prior` | `dCBOR(["urn:trellis:transition:test:006", 1745000100, "prior"])` | `input-attestation-preimage-prior.cbor` (45 B) |
| `new`   | `dCBOR(["urn:trellis:transition:test:006", 1745000100, "new"])`   | `input-attestation-preimage-new.cbor`   (43 B) |

Each preimage is domain-separated under `trellis-transition-attestation-v1` (Core §9.8) and then signed with the corresponding Ed25519 key. `authority_class="prior"` is signed with `_keys/attestation-authority-cm-b-001.cose_key`; `authority_class="new"` is signed with `_keys/issuer-001.cose_key` (which also signs the outer COSE_Sign1 envelope — the fixture reuses the issuer key for the CM-A authority role to avoid authoring a third keypair for signals that do not depend on signer distinctness).

### Step 4 — Assemble the `CustodyModelTransitionPayload` (Appendix A.5.1)

Field-by-field against Companion A.5.1's CDDL:

| Field | Value | Source |
|---|---|---|
| `transition_id` | `"urn:trellis:transition:test:006"` | Pinned; opaque URI scoped by ledger_scope. |
| `from_custody_model` | `"CM-B"` | Companion §9.2 identifier. |
| `to_custody_model` | `"CM-A"` | Companion §9.2 identifier. |
| `effective_at` | `1745000100` | Unix seconds UTC. |
| `reason_code` | `3` | `operator-boundary-change`, registered in A.5.1 reason-code table. |
| `declaration_doc_digest` | Step 2 output — 32 bytes. | Core §9.1 + §9.8. |
| `transition_actor` | `"urn:trellis:principal:test-operator"` | Opaque URI. |
| `policy_authority` | `"urn:trellis:authority:test-governance"` | Opaque URI. |
| `temporal_scope` | `"prospective"` | Companion §10.3 field 6. |
| `attestations` | 2-element array (Step 3) | §10.4 dual-attestation. |
| `extensions` | `null` | No extension fields. |

### Step 5 — Nest the payload into `EventPayload.extensions` (Core §6.5, §6.7)

Per Core §6.5 strict-superset semantics, registered extensions are keyed by their event-type string. The transition payload therefore rides under the key `"trellis.custody-model-transition.v1"` in the enclosing `EventPayload.extensions` map. `EventHeader.event_type` is also set to the UTF-8 encoding of that same identifier (the `event_type` field names the payload family; the extension slot carries its concrete CDDL).

`EventPayload.header.classification` is set to `"x-trellis-test/unclassified"` — outcome-neutral per Core §12.4. `EventPayload.payload_ref` carries an 18-byte marker ciphertext `"custody-transition"` under `ref_type="inline"`; §6.1 requires a `payload_ref` value for every event, and the transition's semantic content is entirely in `extensions`, so the marker payload is narrative only (its `content_hash` still participates in canonical-event-hash construction via the normal §9.3 path).

### Step 6 — Compute `author_event_hash` (§9.5)

Build `AuthorEventHashPreimage` per Appendix A §28 (13 fields: `version, ledger_scope, sequence, prev_hash, causal_deps, content_hash, header, commitments, payload_ref, key_bag, idempotency_key, extensions`). Serialize under dCBOR; this is `input-author-event-hash-preimage.cbor` (1214 bytes).

Domain-separate under `trellis-author-event-v1` and hash: `author_event_hash = SHA-256(domain_preimage("trellis-author-event-v1", <authored bytes>))` = **`ca4453e7295ab1b3431d10c5e7268c92406175b11f1d529a47fc01a612788c00`** (32 bytes). Committed as `author-event-hash.bin`.

### Step 7 — Build `EventPayload` and compute `canonical_event_hash` (§6.1, §9.2)

`EventPayload` has the same fields as the authored preimage plus `author_event_hash` from Step 6. Serialize under dCBOR: this is `expected-event-payload.cbor` (1266 bytes).

`CanonicalEventHashPreimage = {version, ledger_scope, event_payload}`; serialize, domain-separate under `trellis-event-v1`, hash. `canonical_event_hash = `**`7222eb378b82a5c47458a207cafde0da0d48ec8bff7a9f2b8fb528b4ed647023`**.

### Step 8 — COSE_Sign1 envelope (§7.1, §7.4)

Protected header (dCBOR map): `{1: -8, 4: <kid>, -65537: 1}` — alg = EdDSA, kid = derived per §8.3 from issuer-001 pub, suite_id = 1. Same kid as `append/001`/`005` (`af9dff525391faa75c8e8da4808b1743`).

`Sig_structure = dCBOR(["Signature1", <protected_bstr>, h'', <payload_bstr>])` per RFC 9052 §4.4 with Core §6.6 pinning `external_aad = h''`. Committed as `sig-structure.bin` (1311 bytes). Ed25519-sign with the issuer-001 seed; signature is 64 bytes.

Final COSE_Sign1 tag-18 envelope: `[<protected_bstr>, {}, <payload_bstr>, <signature>]`. Committed as `expected-event.cbor` (1367 bytes).

### Step 9 — `AppendHead` (§10.6)

`AppendHead = {scope, sequence, canonical_event_hash}`. `scope = "test-response-ledger"`, `sequence = 1`, `canonical_event_hash = 7222eb37…647023`. Committed as `expected-append-head.cbor` (93 bytes).

---

## Invariant → byte mapping

| Invariant / capability | Where in this vector's bytes |
|---|---|
| Core §6.7 registered extension `trellis.custody-model-transition.v1` | `EventPayload.extensions` key in `expected-event-payload.cbor`. |
| Companion A.5.1 CDDL (custody-model transition) | The value map under that key. |
| Companion A.5 shared `Attestation` rule | Two-element `attestations` array inside the transition payload; each element's `signature` bstr is Ed25519 over `domain_preimage("trellis-transition-attestation-v1", dCBOR([transition_id, effective_at, authority_class]))`. |
| OC-11 / §10.4 widening → dual attestation | `attestations` length = 2 with classes `{"prior", "new"}`. |
| OC-15 + Decision 1 co-publish | `declaration_doc_digest` resolves to the bytes of `input-posture-declaration.bin`. |
| Core §9.8 domain tag `trellis-posture-declaration-v1` | Used once — for `declaration_doc_digest`. |
| Core §9.8 domain tag `trellis-transition-attestation-v1` | Used twice — once per attestation signature. |

## Core-gap notes

The PostureDeclaration's interior CDDL is unpinned. The verifier's byte obligation (§19 step 6.c) is to recompute the digest over whatever bytes live in `input-posture-declaration.bin` and compare — the interior structure is opaque to the check. A follow-on Core amendment pinning the CDDL would let fixtures cite a byte-shape beyond "minimal dCBOR map matching the Appendix A.1 named fields"; it is not required for this vector's byte-level signal.

## Footer — summary digests

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| `input-prior-append-head.cbor` | 93 | `dc0fc83406bc87364b8beeebb4b8c867e68e9e5a025e24817c542f91da3772db` |
| `input-posture-declaration.bin` | 320 | `dfe2dd26f70cc63288259e7b0bcfaaf4dd77720ee72bdb5d1fa32ecc5385cfc2` |
| `input-attestation-preimage-prior.cbor` | 45 | `85b5e0dadd2b143390dab58c277c3ea2346444e623e341419557b79bb4da2732` |
| `input-attestation-preimage-new.cbor` | 43 | `e9cd4156ae1266b238f19ee88abbd4305a67f60b0aee7ce5fea527dd0c32ccd2` |
| `input-author-event-hash-preimage.cbor` | 1214 | `08ab474c164975f03e7a681cb3fdbaede9cd3cbf5ed127868955a82b841b0632` |
| `author-event-hash.bin` | 32 | `ca4453e7295ab1b3431d10c5e7268c92406175b11f1d529a47fc01a612788c00` |
| `expected-event-payload.cbor` | 1266 | `a321a76eb109fa8a1cc642f8658b76b6f54603a8257464c0bd4a4e4d273de67c` |
| `expected-event.cbor` (COSE_Sign1) | 1367 | `1b22adf49e2ec12bae7d77fd3f0f37534683c816cf0dff7a683abceaa5079a5c` |
| `expected-append-head.cbor` | 93 | `183f4f125117f5b6d207f751edb318edf5b8e82b8aa51c409dc8f8d6d37c7798` |

Pinned salient values (hex):

- `kid` = `af9dff525391faa75c8e8da4808b1743` (shared with `append/001`/`005` — same issuer, same suite).
- `prev_hash` = `ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb` (= `append/001`'s canonical_event_hash).
- `declaration_doc_digest` = `fced7843312f6fd3f9a61e10068f9cd8d7a095d479278a1307d4b584d66aa3e1`.
- `author_event_hash` = `ca4453e7295ab1b3431d10c5e7268c92406175b11f1d529a47fc01a612788c00`.
- `canonical_event_hash` = `7222eb378b82a5c47458a207cafde0da0d48ec8bff7a9f2b8fb528b4ed647023`.
