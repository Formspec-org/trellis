# Derivation — `append/007-custody-transition-cm-c-narrowing`

## Header

**What this vector exercises.** Second Stream D / O-5 Posture-transition vector. It pins the *narrowing* branch of Companion §10.4 / OC-11 / Appendix A.5.3 step 4: a custody-model transition that contracts access surface (here CM-C Delegated Compute → CM-B Reader-Held with Recovery Assistance) MAY be attested by the new authority alone. The event envelope, domain-tag discipline, and `EventPayload.extensions` shape are structurally identical to `append/006-custody-transition-cm-b-to-cm-a`. The load-bearing deltas are:

1. `from_custody_model = "CM-C"`, `to_custody_model = "CM-B"` (direction reversed vs 006, and narrowing rather than widening).
2. `reason_code = 2` (`key-custody-change` per Appendix A.5.1 reason-code table).
3. `attestations` carries a single element with `authority_class = "new"` — the narrowing carve-out under OC-11.
4. Fresh `idempotency_key` (`idemp-append-007`) so the vector identity is distinct from 006 under Core §17.3's `(ledger_scope, idempotency_key)` uniqueness rule.

**Scope.** Fact-producer side only; the verifier-side counterpart (that a narrowing transition with a single `authority_class="new"` attestation is accepted without triggering `attestation_insufficient`) is implicit — tamper/003 exercises the *failure* branch (widening with missing dual attestation). Together they pin both sides of the §10.4 rule.

**Core / Companion § roadmap.** Identical to 006; see that derivation's Header for the full ordered traversal. The only new normative anchor in 007 is Appendix A.5.3 step 4 narrowing-branch prose.

---

## Body

### Step 1 — Prior head (§10.2, §10.6)

Same `ledger_scope` as 006, same chain root (`append/001`). `prev_hash = ef2622f1…724ddb`. Copied into the vector as `input-prior-append-head.cbor`.

### Step 2 — Post-transition PostureDeclaration (Companion §11, Appendix A.1)

`custody_model = CM-B` and `disclosure_profile = rl-profile-B` in the declaration in force AFTER the transition. Fresh `declaration_id = urn:trellis:declaration:test:007-post`; `supersedes = urn:trellis:declaration:test:007-pre`. Bytes committed as `input-posture-declaration.bin` (320 B).

`declaration_doc_digest = SHA-256(domain_preimage("trellis-posture-declaration-v1", <declaration bytes>)) = 3a9b0d0fc0071f8af22db191e96e9075dfc290992d374186762f90025b965807`.

### Step 3 — Single attestation (Companion Appendix A.5 shared rule + §10.4 narrowing branch)

Companion Appendix A.5.3 step 4 narrowing branch:

> "`scope_change = "Narrowing"` — MAY be attested by the new authority alone. For Custody-Model transitions (which have no `scope_change` field), the Posture-expanding cases named in OC-11 (transitions expanding provider-readable access) MUST be dually attested; narrowing cases MAY be attested by the new authority alone."

CM-C → CM-B is a custody-model narrowing: CM-C admits delegated-compute plaintext exposure; CM-B does not. The transition therefore qualifies for the single-attestation branch. This vector emits exactly one `Attestation`:

| authority_class | preimage | signing key |
|---|---|---|
| `new` | `dCBOR(["urn:trellis:transition:test:007", 1745000200, "new"])` | `_keys/issuer-001.cose_key` |

The preimage is domain-separated under `trellis-transition-attestation-v1` (Core §9.8) and Ed25519-signed. Preimage bytes committed as `input-attestation-preimage-new.cbor` (43 B). The `_keys/attestation-authority-cm-b-001.cose_key` key is NOT required for this vector — narrowing does not need the prior authority to co-attest.

### Step 4 — `CustodyModelTransitionPayload` (Appendix A.5.1)

| Field | Value |
|---|---|
| `transition_id` | `"urn:trellis:transition:test:007"` |
| `from_custody_model` | `"CM-C"` |
| `to_custody_model` | `"CM-B"` |
| `effective_at` | `1745000200` |
| `reason_code` | `2` (key-custody-change) |
| `declaration_doc_digest` | Step 2 output — 32 bytes. |
| `transition_actor` | `"urn:trellis:principal:test-operator"` |
| `policy_authority` | `"urn:trellis:authority:test-governance"` |
| `temporal_scope` | `"prospective"` |
| `attestations` | 1-element array: `[Attestation{authority="urn:trellis:authority:test-cm-b-authority", authority_class="new", signature=<64-byte ed25519>}]` |
| `extensions` | `null` |

### Steps 5–9 — Event envelope, COSE_Sign1, AppendHead

Identical discipline to 006 (authored preimage → author_event_hash → canonical event payload → canonical_event_hash → protected header → Sig_structure → signature → tag-18 envelope → AppendHead). Only the `extensions` / `idempotency_key` / `authored_at` fields differ, which ripples through every downstream hash. Final outputs:

| Artifact | Bytes | SHA-256 |
|---|---:|---|
| `input-prior-append-head.cbor` | 93 | `dc0fc83406bc87364b8beeebb4b8c867e68e9e5a025e24817c542f91da3772db` |
| `input-posture-declaration.bin` | 320 | `8acce250c708a6f9bf3c8d2b5347e0f49a73aa33fe8a744b82c1e5fb08c385a4` |
| `input-attestation-preimage-new.cbor` | 43 | `3539dd948b844f7cab002f30c52d9e4c0f9f15ba53bfe059335011b972d167b6` |
| `input-author-event-hash-preimage.cbor` | 1062 | `24a712fc692ee6f06e747635ac715db37f11d79177709a08a5e97031dcd0b391` |
| `author-event-hash.bin` | 32 | `2ab59e20ac428369859bf88973aed06ee9f7d829f4eb432573551bf2356c35ff` |
| `expected-event-payload.cbor` | 1114 | `9f4a65c9f63a8dd80cda274272085985fe1bcd5c3cc40f1f7064a5a171c3bc62` |
| `expected-event.cbor` | 1215 | `a79dbe011786863dead8121d622d33f7f0b8ddee7c2682f3abc9801c70506079` |
| `expected-append-head.cbor` | 93 | `fa9bb47eed792c49478ff6d5a6842c200b9f8fc8fb840366ebdfe44104f94d46` |

Pinned salient values:

- `kid` = `af9dff525391faa75c8e8da4808b1743` (issuer-001).
- `prev_hash` = `ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb`.
- `declaration_doc_digest` = `3a9b0d0fc0071f8af22db191e96e9075dfc290992d374186762f90025b965807`.
- `author_event_hash` (raw bytes in `author-event-hash.bin`) = `94b5e0d7aff302f685328dbdc549cf378d218f10676160ea93be4573c850ab0a`.
- `canonical_event_hash` = `ceb269e6db61fcdb2fb118c53ce3e2f51d6c47b77c6e03c0e0d8a742b0adc159`.

---

## Invariant → byte mapping

| Invariant / capability | Where in this vector's bytes |
|---|---|
| Companion §10.4 / OC-11 narrowing carve-out | `attestations` array length = 1; sole element has `authority_class = "new"`. |
| Companion A.5.1 `reason_code` registry | Value `2` = `key-custody-change`. |
| Core §17.3 `(ledger_scope, idempotency_key)` uniqueness | `idempotency_key = b"idemp-append-007"` — distinct from 001's `b"idemp-append-001"` and 006's `b"idemp-append-006"`. |
| Core §6.7 registered extension `trellis.custody-model-transition.v1` | Same as 006; extension key unchanged. |

## Core-gap notes

None new beyond those flagged in `append/006`'s derivation (PostureDeclaration interior CDDL unpinned — not load-bearing here).
