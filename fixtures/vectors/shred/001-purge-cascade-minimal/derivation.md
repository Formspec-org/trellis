# Derivation — `shred/001-purge-cascade-minimal`

## Header

**What this vector exercises.** This vector is the FIRST O-3 shred-conformance fixture. It exercises Test 4 of `thoughts/specs/2026-04-18-trellis-o3-projection-conformance.md` — purge-cascade verification. A minimal 2-event canonical chain is constructed over a fresh ledger scope `test-shred-ledger`: event 0 appends a plaintext-bearing `PayloadInline` ciphertext; event 1 is a canonical crypto-shred event that binds event 0's §9.3 `content_hash` into its own payload. The fixture declares four Appendix A.7 cascade-scope classes (`CS-01`, `CS-03`, `CS-04`, `CS-05`) that MUST report `invalidated_or_plaintext_absent = true` once the shred event is applied. The byte-compare target is `expected-cascade-report.cbor`; the crypto-shred event is also committed in isolation as `input-shred-event.cbor` for runners that process the shred fact directly.

**Scope of this vector.** Purge-cascade attestation only, structural-only. No HPKE wrap — `PayloadInline.ciphertext` carries opaque pinned bytes as in `append/001` and `projection/001`. The cascade-enforcement runner is NOT exercised here; this fixture pins the *inputs the runner consumes* and the *expected post-state the runner must reach*, not a byte-compare of an implementation's runtime cascade. The declared scope deliberately omits `CS-02` (evaluator state) and `CS-06` (respondent-facing / workflow-export views) because those classes require materializing a further derived artifact whose bytes are not in-scope for the first-batch minimal fixture. A follow-on fixture will add those classes and exercise them against a pre-shred view artifact.

**Companion / Core § roadmap (in traversal order).**

1. Core §5.1 — dCBOR encoding profile. Every CBOR structure below is serialized per §5.1.
2. Core §8.3 — `kid` derivation, reused unchanged.
3. Core §9.3 — `content_hash` over the event-0 ciphertext bytes. This is the binding handle the shred event references.
4. Core §9.5 + §6.8 + §7.4 + §6.1 — event 0 (plaintext-bearing append) construction: authored → canonical → signed forms. Identical shape to `append/001` Steps 3–10.
5. Companion §20.3 (Crypto-Shredding Scope) + Core §6.1 — event 1 (crypto-shred) construction. The shred event's own payload is a small dCBOR declaration map `{target_content_hash: <event-0 content_hash>, reason: "key-destroyed"}`. This map is the inline payload whose bytes §9.3 hashes as event 1's `content_hash`. (Erasure **facts** are not encrypted; the canonical event itself is plaintext — what erasure destroys is event 0's DEK in a production deployment; at the §6/§9 layer this fixture exercises, the shred-event *payload* is a plaintext declaration identifying which prior content is erased.)
6. Companion §20.4 (OC-76) + §20.5 (OC-77) + Appendix A.7 — cascade-scope enumeration. The fixture declares a subset of the six A.7 classes; each declared class asserts `invalidated_or_plaintext_absent = true` post-cascade.
7. Cascade-report construction — `expected-cascade-report.cbor` = dCBOR map binding the target content_hash, the declared scope list, and the expected post-state per class.

**Pinned inputs.**

| Input | Value | Source |
|---|---|---|
| `signing_key` | Ed25519 COSE_Key (same issuer as `append/001` / `projection/001`) | `../../_keys/issuer-001.cose_key` |
| `ledger_scope` | bstr `"test-shred-ledger"` (17 bytes) | Core §10.4 |
| Event 0 `sequence` | `0` (genesis) | §10.2 |
| Event 0 `event_type` | bstr `"x-trellis-test/shred-target-append"` (34 bytes) | §14.6 |
| Event 0 `authored_at` | `1745100000` | §12.1 |
| Event 0 `classification` | bstr `"x-trellis-test/shreddable"` (25 bytes) | §14.6 |
| Event 0 `idempotency_key` | bstr `"idemp-shred-tgt0"` (16 bytes) | §6.1 |
| Event 0 `payload_bytes` | `"shred-target-plaintext-bytes-v1"` + 1 × `0x00` (32 bytes) | PayloadInline ciphertext |
| Event 1 `sequence` | `1` | §10.2 |
| Event 1 `prev_hash` | `canonical_event_hash[0]` | §10.2 |
| Event 1 `event_type` | bstr `"x-trellis-test/crypto-shred"` (27 bytes) | §14.6 |
| Event 1 `authored_at` | `1745100060` | §12.1 |
| Event 1 `classification` | bstr `"x-trellis-test/shred-fact"` (25 bytes) | §14.6 |
| Event 1 `idempotency_key` | bstr `"idemp-shred-evt1"` (16 bytes) | §6.1 |
| Event 1 `payload_bytes` | dCBOR map `{reason: "key-destroyed", target_content_hash: <bstr>}` | Companion §20.3 binding |
| `declared_scope` | `["CS-01", "CS-03", "CS-04", "CS-05"]` | Companion Appendix A.7 |

---

## Body

### Step 1: Event 0 — plaintext-bearing append

**Core § citation:** §6.8, §9.3, §9.5, §9.2, §7.4, §6.1.

**Operation.** Identical to `append/001` Steps 3–10, parameterized with the Event 0 pins from the table above. Using the issuer seed + pubkey from `../../_keys/issuer-001.cose_key` and `kid = af9dff525391faa75c8e8da4808b1743`:

1. `content_hash[0] = SHA-256("trellis-content-v1" domain-sep over 32-byte payload_bytes)` per §9.3.
2. Build `AuthorEventHashPreimage`, dCBOR-encode, compute `author_event_hash[0]` per §9.5.
3. Build `EventPayload`, dCBOR-encode.
4. Build COSE protected-header bstr (`{1:-8, 4:kid, -65537:1}`), wrap, sign with Ed25519 over `Sig_structure` per RFC 9052 §4.4.
5. Assemble COSE_Sign1 tag-18 envelope.
6. Compute `canonical_event_hash[0]` per §9.2.

**Results.**

| Output | Value |
|---|---|
| `content_hash[0]` | `c995d0f05e0505cf4f6c1d330552508275f804ef0c74aa612dc00640e8f8484d` |
| `canonical_event_hash[0]` | `faf815c2caf4c6ebe0de1ab7d7817a3f364580f45ca11c33bf2943b8ffd9765f` |

**Committed as:** embedded in `input-chain.cbor` as event 0. The content_hash is also byte-pinned in `expected-cascade-report.cbor` under `target_content_hash` (Step 3).

---

### Step 2: Event 1 — canonical crypto-shred event

**Spec citation:** Companion §20.3 (Crypto-Shredding Scope, OC-75); Core §6.1 (events are ordinary COSE_Sign1 / EventPayload records regardless of semantic kind); Core §9.3 (content_hash runs over ciphertext bytes).

**Load-bearing sentence (§20.3):**

> "Trellis Core owns the cryptographic mechanics that make crypto-shredding work: Core §9 (Hash Construction) requires `content_hash` over ciphertext so that destroying the payload DEK leaves the chain verifiable, and the HPKE key-bag wrap defined therein holds the DEK that erasure destroys. This companion adds the **operational** obligation: cryptographic erasure is **incomplete** until the **purge cascade** completes across every derived artifact holding plaintext or plaintext-derived material subject to the erasure event."

**Shred-event payload (dCBOR map).** The fixture binds the shred fact to its target by embedding event 0's `content_hash[0]` in the shred event's payload, under a `target_content_hash` key. The shred payload map also carries a minimal `reason` tstr. The payload is plaintext — the shred event itself is not encrypted. §9.3's content_hash construction runs over these bytes regardless of whether they are plaintext or ciphertext; the field is named `ciphertext` in `PayloadInline` but is defined as "the exact bytes named by payload_ref" (§9.3), and a plaintext declaration is valid as long as the downstream obligations honor it.

```
shred_declaration = {
  "reason":              "key-destroyed",
  "target_content_hash": content_hash[0],  ; 32 bytes from Step 1
}
event_1.payload_bytes = dCBOR(shred_declaration)
```

dCBOR key ordering: `reason` (`0x66` prefix) < `target_content_hash` (`0x73` prefix), so the serialized order is `reason`, `target_content_hash`.

**Operation.** With `prev_hash = canonical_event_hash[0]` (Step 1), apply the identical construction as Step 1 (the §6.8 / §9.5 / §6.1 / §7.4 pipeline) using the Event 1 pins from the table above and `event_1.payload_bytes` from this step.

**Results.**

| Output | Value |
|---|---|
| `canonical_event_hash[1]` | `a1fd83d043c3473f4a302d22fb84f2fbcbd6a275c6f20cc0037bdc95d0033f4b` |

**Committed as:** embedded in `input-chain.cbor` as event 1, AND committed as the standalone file `input-shred-event.cbor` (729 bytes). A reader may confirm that `input-shred-event.cbor` is byte-identical to chain bytes `0xd2 0x84 …` starting immediately after event 0's COSE_Sign1 terminates in `input-chain.cbor`.

---

### Step 3: Cascade report

**Spec citation:** Companion §20.4 (Purge-Cascade Obligation, OC-76), §20.5 (Cascade Scope, OC-77), Appendix A.7 (Cascade-Scope Enumeration).

**Load-bearing sentence (§20.4 / OC-76):**

> "If canonical lifecycle facts declare that protected content has been cryptographically destroyed, sealed, or otherwise made inaccessible, every derived artifact that holds plaintext or plaintext-derived material subject to that declaration MUST be invalidated, purged, or otherwise made unusable according to the Operator's declared policy."

**Load-bearing sentence (§20.5 / OC-77):**

> "The purge cascade MUST reach every class in the cascade-scope enumeration (Appendix A.7). [...] new conformance fixtures exercising purge-cascade verification MUST reference the class by its enumerated identifier rather than by prose description."

**Appendix A.7 enumeration (from which the declared subset is drawn).**

| identifier | class | reference |
|---|---|---|
| `CS-01` | consumer-facing and system projections | §15 |
| `CS-02` | evaluator state that incorporated the destroyed material | §25 |
| `CS-03` | snapshots retained for performance or recovery | §16 |
| `CS-04` | caches, indexes, and materialized views | §15 |
| `CS-05` | rebuild fixtures that contain the destroyed material | §14 / §20.4 |
| `CS-06` | respondent-facing history views and workflow export views | §§23–24 |

**Declared scope for this fixture.** `["CS-01", "CS-03", "CS-04", "CS-05"]`. CS-02 and CS-06 are omitted (not NOT-in-scope in principle, but out-of-scope for the first-batch fixture — see "Scope of this vector" above).

**Cascade-report structure.**

```
cascade_report = {
  "declared_scope":      ["CS-01", "CS-03", "CS-04", "CS-05"],    ; dCBOR list of tstr
  "expected_post_state": {
    "CS-01": { "invalidated_or_plaintext_absent": true,
               "rationale": "CS-01-in-declared-cascade-scope" },
    "CS-03": { "invalidated_or_plaintext_absent": true,
               "rationale": "CS-03-in-declared-cascade-scope" },
    "CS-04": { "invalidated_or_plaintext_absent": true,
               "rationale": "CS-04-in-declared-cascade-scope" },
    "CS-05": { "invalidated_or_plaintext_absent": true,
               "rationale": "CS-05-in-declared-cascade-scope" },
  },
  "target_content_hash": <32-byte content_hash[0] from Step 1>,
}
```

**Operation.** dCBOR-encode (§5.1 byte-wise lex key ordering). Resulting dCBOR key order for the top-level map: `declared_scope` (14 chars, `0x6e` prefix), `expected_post_state` (19 chars, `0x73` prefix), `target_content_hash` (19 chars, `0x73` prefix). The two `0x73`-prefixed keys tie-break by continuing byte-wise comparison: `e` < `t` so `expected_post_state` precedes `target_content_hash`.

**Result (452 bytes):** first byte `0xa3` (map of 3). Full hex in the footer.

**Committed as:** `expected-cascade-report.cbor` (452 bytes). This is the byte-compare target Test 4's runner MUST reach once the cascade is applied.

---

## Footer — full hex dumps

Each block below is the byte-exact content of the named sibling file.

### `expected-cascade-report.cbor` (452 bytes, sha256 `55b390d1a8409401679593668822ed3c244b2fe74d08145f424590b9db59b8ed`)

```
a36e6465636c617265645f73636f7065846543532d30316543532d3033654353
2d30346543532d30357365787065637465645f706f73745f7374617465a46543
532d3031a269726174696f6e616c65781f43532d30312d696e2d6465636c6172
65642d636173636164652d73636f7065781f696e76616c6964617465645f6f72
5f706c61696e746578745f616273656e74f56543532d3033a269726174696f6e
616c65781f43532d30332d696e2d6465636c617265642d636173636164652d73
636f7065781f696e76616c6964617465645f6f725f706c61696e746578745f61
6273656e74f56543532d3034a269726174696f6e616c65781f43532d30342d69
6e2d6465636c617265642d636173636164652d73636f7065781f696e76616c69
64617465645f6f725f706c61696e746578745f616273656e74f56543532d3035
a269726174696f6e616c65781f43532d30352d696e2d6465636c617265642d63
6173636164652d73636f7065781f696e76616c6964617465645f6f725f706c61
696e746578745f616273656e74f5737461726765745f636f6e74656e745f6861
73685820c995d0f05e0505cf4f6c1d330552508275f804ef0c74aa612dc00640
e8f8484d
```

Structure: `0xa3` (map of 3); keys in the dCBOR order `declared_scope`, `expected_post_state`, `target_content_hash`. The `expected_post_state` inner map has 4 entries (`0xa4`) keyed `CS-01`, `CS-03`, `CS-04`, `CS-05`. Each value is a 2-entry map (`0xa2`) carrying `rationale` (0x69-prefix text) and `invalidated_or_plaintext_absent` (0x78-prefix text, longer than 23 chars). Each boolean is `0xf5` (true).

### `input-shred-event.cbor` (729 bytes, sha256 `d7235d83eb684c0c5dc0d3d80499de49d4db76ddd02b71ad987059560755ecaf`)

The crypto-shred event's COSE_Sign1 envelope in isolation. Structure: `0xd2` (tag 18); `0x84` (array of 4); `0x58 0x1b <27-byte protected header>` (identical to `append/001` / `projection/001`); `0xa0` (empty unprotected); `0x59 0x02 0x74 <628-byte EventPayload>`; `0x58 0x40 <64-byte Ed25519 signature>`.

The EventPayload's `payload_ref.ciphertext` field at the offset marked by the key `0x6a 6369706865727465787478` (= "ciphertext" text key) carries the 76-byte dCBOR-encoded shred declaration (`0xa2 66 72 65 61 73 6f 6e 6d 6b 65 79 2d 64 65 73 74 72 6f 79 65 64 73 74 61 72 67 65 74 5f 63 6f 6e 74 65 6e 74 5f 68 61 73 68 58 20 <32-byte content_hash[0]>`). A reader can confirm `target_content_hash` inside this embedded bstr equals event 0's `content_hash` from Step 1.

### `input-chain.cbor` (1389 bytes, sha256 `1b48180ff50beaa188cea9abe895993d8f1a639cc27b3982adc4a0853812cfa6`)

CBOR array of two COSE_Sign1 envelopes (`0x82` = array of 2, then `0xd2 …` for each envelope). The concatenation layout is identical to `projection/001`'s `input-chain.cbor`. Event 0 (plaintext append) occupies bytes 1..N, event 1 (crypto-shred) occupies bytes N+1..end, where event 1's bytes are byte-identical to `input-shred-event.cbor`.

Reader-verifiable intermediates (embedded in the chain bytes):

- Event 0 `content_hash` = `c995d0f05e0505cf4f6c1d330552508275f804ef0c74aa612dc00640e8f8484d`
- Event 0 `canonical_event_hash` = `faf815c2caf4c6ebe0de1ab7d7817a3f364580f45ca11c33bf2943b8ffd9765f`
- Event 1 `prev_hash` = Event 0 `canonical_event_hash` (§10.2), matching the 32 bytes at event 1's EventPayload `prev_hash` offset.
- Event 1 `canonical_event_hash` = `a1fd83d043c3473f4a302d22fb84f2fbcbd6a275c6f20cc0037bdc95d0033f4b`

The shred declaration embedded inside event 1's `PayloadInline.ciphertext` carries `target_content_hash` = event 0's `content_hash` above, proving the binding between the erasure fact and its target.
