# Derivation — `shred/002-backup-refusal`

## Header

**What this vector exercises.** This vector is the SECOND O-3 shred-conformance fixture and the first to exercise the **backup-refusal** sub-scenario of Test 4 in `thoughts/specs/2026-04-18-trellis-o3-projection-conformance.md`. A minimal two-event canonical chain is constructed over a fresh ledger scope `test-shred-backup-ledger`: event 0 appends a plaintext-bearing `PayloadInline`; event 1 is a canonical crypto-shred event binding event 0's §9.3 `content_hash` into its own payload. Alongside the chain the fixture ships `input-backup-snapshot.cbor` — a pre-shred snapshot that materialized event 0's plaintext (Companion §16.4 "A snapshot MAY be used to accelerate recovery or rebuild of derived artifacts"). The byte-compare target `expected-cascade-report.cbor` asserts that for every declared Appendix A.7 cascade-scope class (`CS-01`, `CS-03`, `CS-04`, `CS-05`) post-cascade MUST hold BOTH `invalidated_or_plaintext_absent = true` AND `backup_resurrection_refused = true`. Companion §16.5 / §20.5 second sentence / §27.3 OC-135 row 3 / §28.6 all pin the refusal rule; this fixture turns it into byte-exact conformance evidence. Exercises TR-OP-004 in its backup-refusal dimension.

**Scope of this vector.** Structural-only, no HPKE wrap. `PayloadInline.ciphertext` carries opaque pinned bytes, matching `shred/001` / `projection/001` conventions. The fixture pins:

1. the **canonical inputs** — the chain plus the shred event in isolation, byte-identical to their counterparts under this fixture's pinned ledger scope,
2. a **backup snapshot** — a dCBOR map capturing what a post-shred restore would reintroduce,
3. the **expected post-state** — the byte-exact cascade report a conforming runner MUST reach once the shred fact is applied and a restore of `input-backup-snapshot.cbor` into a live derived artifact of any in-scope class is attempted.

The declared cascade scope deliberately mirrors `shred/001` (`CS-01`, `CS-03`, `CS-04`, `CS-05`). CS-02 (evaluator state) and CS-06 (respondent-facing / workflow-export views) require materializing further derived artifacts and are out of scope for this minimal fixture — they belong to later fixtures in the series. The goal of this fixture is the *refusal* axis of TR-OP-004, not scope expansion.

**Why a pre-shred snapshot is load-bearing.** Companion §16.4 names snapshots as a legitimate recovery substrate. §16.5 carves out the one case where recovery is forbidden: "A snapshot MUST NOT be used to resurrect canonically-destroyed plaintext into live derived artifacts." §20.5 second sentence generalizes the rule to any backup. §27.3 OC-135 row 3 makes it conformance-testable: "that backup-resurrection is prevented — no live derived artifact may be restored from backup to a state containing destroyed plaintext." The snapshot here is the simplest artifact that realizes that rule: a dCBOR map that materializes event 0's plaintext, taken at canonical append height `tree_size = 1` (after event 0, before event 1). Once event 1 appends, any restore of this snapshot into a live derived artifact is a §16.5 violation.

**Companion / Core § roadmap (in traversal order).**

1. Core §5.1 — dCBOR encoding profile. Every CBOR structure below is serialized per §5.1.
2. Core §8.3 — `kid` derivation, reused unchanged from `shred/001`.
3. Core §9.3 — `content_hash` over the event-0 ciphertext bytes. This is the binding handle the shred event references AND the handle the backup snapshot binds to.
4. Core §9.5 + §6.8 + §7.4 + §6.1 — event 0 (plaintext-bearing append) construction: authored → canonical → signed forms. Identical shape to `shred/001` Step 1.
5. Companion §16.4 (Snapshot as Recovery Substrate) — the backup snapshot's legitimate role. `taken_at_tree_size = 1` pins the height at which the snapshot materialized.
6. Companion §16.5 (Retention and Purge Cascade) — the refusal rule. Load-bearing sentence pinned below in Step 2.
7. Companion §20.3 (OC-75) + §20.4 (OC-76) + §20.5 (OC-77) + Appendix A.7 — cascade scope + obligation. Identical to `shred/001`.
8. Companion §20.5 second sentence — "Backups are governed by the Operator's retention and recovery policy; backups MUST NOT be used to resurrect destroyed plaintext into live derived artifacts." The generalization of §16.5 from snapshots to any backup.
9. Companion §27.3 OC-135 row 3 — "that backup-resurrection is prevented — no live derived artifact may be restored from backup to a state containing destroyed plaintext." Makes the refusal rule a conformance-test requirement, which is what this fixture pins.
10. Companion §28.6 — "Purge-cascade operations (§20.3) MUST NOT leave residual plaintext in system projections, caches, backups, evaluator state, or rebuild fixtures." The completeness obligation against which the cascade report's flags are verified.
11. Cascade-report construction — `expected-cascade-report.cbor` = dCBOR map binding the target `content_hash`, the declared scope list, a SHA-256 digest of the backup snapshot bytes, and the per-class expected post-state (`invalidated_or_plaintext_absent` + `backup_resurrection_refused` + refusal rationale).

**Pinned inputs.**

| Input | Value | Source |
|---|---|---|
| `signing_key` | Ed25519 COSE_Key (same issuer as `append/001` / `shred/001`) | `../../_keys/issuer-001.cose_key` |
| `ledger_scope` | bstr `"test-shred-backup-ledger"` (24 bytes) | Core §10.4 |
| Event 0 `sequence` | `0` (genesis) | §10.2 |
| Event 0 `event_type` | bstr `"x-trellis-test/backup-target-append"` (35 bytes) | §14.6 |
| Event 0 `authored_at` | `1745200000` | §12.1 |
| Event 0 `classification` | bstr `"x-trellis-test/shreddable"` (25 bytes) | §14.6 |
| Event 0 `idempotency_key` | bstr `"idemp-bkp-tgt-00"` (16 bytes) | §6.1 |
| Event 0 `payload_bytes` | `"backup-refusal-plaintext-bytes"` + 2 × `0x00` (32 bytes) | PayloadInline ciphertext |
| Event 1 `sequence` | `1` | §10.2 |
| Event 1 `prev_hash` | `canonical_event_hash[0]` | §10.2 |
| Event 1 `event_type` | bstr `"x-trellis-test/crypto-shred"` (27 bytes) | §14.6 |
| Event 1 `authored_at` | `1745200060` | §12.1 |
| Event 1 `classification` | bstr `"x-trellis-test/shred-fact"` (25 bytes) | §14.6 |
| Event 1 `idempotency_key` | bstr `"idemp-bkp-shr-00"` (16 bytes) | §6.1 |
| Event 1 `payload_bytes` | dCBOR map `{reason: "key-destroyed", target_content_hash: <bstr>}` | Companion §20.3 binding |
| Backup `schema_id` | `"urn:trellis:test:backup-snapshot:v1"` | this fixture (reviewer-oriented) |
| Backup `origin_class` | `"CS-03"` (snapshots retained for performance or recovery) | Appendix A.7 |
| Backup `taken_at_tree_size` | `1` | §16.4 (pre-shred height) |
| Backup `materialized_plaintext` | event 0's `payload_bytes` | §16.5 refusal target |
| `declared_scope` | `["CS-01", "CS-03", "CS-04", "CS-05"]` | Companion Appendix A.7 |

The ledger scope, idempotency keys, authored_at timestamps, and event-type / classification byte-strings are byte-disjoint from `shred/001` so the two fixtures' content_hashes and canonical_event_hashes differ. Each fixture's intermediates remain independently reader-verifiable.

---

## Body

### Step 1: Event 0 — plaintext-bearing append

**Core § citation:** §6.8, §9.3, §9.5, §9.2, §7.4, §6.1.

**Operation.** Identical pipeline to `shred/001` Step 1, parameterized with the Event 0 pins from the table above. Using the issuer seed + pubkey from `../../_keys/issuer-001.cose_key` and `kid = af9dff525391faa75c8e8da4808b1743`:

1. `content_hash[0] = SHA-256("trellis-content-v1" domain-sep over 32-byte payload_bytes)` per §9.3.
2. Build `AuthorEventHashPreimage`, dCBOR-encode, compute `author_event_hash[0]` per §9.5.
3. Build `EventPayload`, dCBOR-encode.
4. Build COSE protected-header bstr (`{1:-8, 4:kid, -65537:1}`), wrap, sign with Ed25519 over `Sig_structure` per RFC 9052 §4.4.
5. Assemble COSE_Sign1 tag-18 envelope.
6. Compute `canonical_event_hash[0]` per §9.2.

**Results.**

| Output | Value |
|---|---|
| `content_hash[0]` | `2b81a765657e96ee25b5ce278025d6d8287014eeda98b98256ae810303247a7e` |
| `canonical_event_hash[0]` | `eb2121c2352308277cd64dd9fa1d1bd53f39fc5456852dc2926e96fbbc2239bf` |

**Committed as:** embedded in `input-chain.cbor` as event 0. The content_hash is also byte-pinned in `expected-cascade-report.cbor` under `target_content_hash` (Step 4) and in `input-backup-snapshot.cbor` under `target_content_hash` (Step 2).

---

### Step 2: Backup snapshot

**Spec citation:** Companion §16.4 (Snapshot as Recovery Substrate, OC-48); Companion §16.5 (Retention and Purge Cascade); Core §9.3 (binding by digest).

**Load-bearing sentence (§16.4, OC-48):**

> "A snapshot MAY be used to accelerate recovery or rebuild of derived artifacts. Where a snapshot is used as a recovery substrate, its rebuild equivalence (§15.3) MUST have been established — by sampled rebuild, checkpoint-bound equivalence, or both — before it is relied upon."

**Load-bearing sentence (§16.5):**

> "Snapshots that contain plaintext or plaintext-derived material are subject to the purge-cascade rules of §20.3. A snapshot MUST NOT be used to resurrect canonically-destroyed plaintext into live derived artifacts."

**Snapshot structure (dCBOR map).** The snapshot is a 5-entry dCBOR map keyed:

| Key | Value |
|---|---|
| `schema_id` | `"urn:trellis:test:backup-snapshot:v1"` |
| `origin_class` | `"CS-03"` (snapshot retained for recovery) |
| `taken_at_tree_size` | `1` (after event 0, before event 1) |
| `target_content_hash` | event 0's `content_hash` from Step 1 |
| `materialized_plaintext` | event 0's `payload_bytes` (32 bytes) |

dCBOR key ordering (byte-wise over UTF-8 key bytes): `schema_id` (9 chars, `0x69` prefix) < `origin_class` (12 chars, `0x6c` prefix) < `taken_at_tree_size` (18 chars, `0x72` prefix) < `target_content_hash` (19 chars, `0x73` prefix, `t` > `m`) … but dCBOR first sorts by encoded key length ascending, then lexicographically within length. Serialized order: `schema_id`, `origin_class`, `taken_at_tree_size`, `target_content_hash`, `materialized_plaintext` (see footer hex dump for the exact byte layout).

**Operation.** dCBOR-encode (§5.1). Resulting bytes are written to `input-backup-snapshot.cbor`.

**Result (198 bytes, sha256 `314ac9d52baa1b82e2171ca6cce9c15ad23622c03f6a69b49ba4c789d487aefb`):** first byte `0xa5` (map of 5). Full hex in the footer.

**Committed as:** `input-backup-snapshot.cbor`. The SHA-256 digest is byte-pinned in `expected-cascade-report.cbor` under `backup_snapshot_ref` (Step 4) so a runner can bind its refusal report to the specific snapshot bytes under test.

---

### Step 3: Event 1 — canonical crypto-shred event

**Spec citation:** Companion §20.3 (Crypto-Shredding Scope, OC-75); Core §6.1 (events are ordinary COSE_Sign1 / EventPayload records regardless of semantic kind); Core §9.3 (`content_hash` over ciphertext).

**Load-bearing sentence (§20.3, OC-75):** identical to `shred/001` Step 2.

**Shred-event payload (dCBOR map).** Same shape as `shred/001` — a 2-entry map binding `reason` (`"key-destroyed"`) and `target_content_hash` (event 0's `content_hash[0]`). dCBOR key ordering places `reason` before `target_content_hash`.

```
shred_declaration = {
  "reason":              "key-destroyed",
  "target_content_hash": content_hash[0],  ; 32 bytes from Step 1
}
event_1.payload_bytes = dCBOR(shred_declaration)
```

**Operation.** With `prev_hash = canonical_event_hash[0]` (Step 1), apply the Step-1 construction pipeline against the Event 1 pins.

**Results.**

| Output | Value |
|---|---|
| `canonical_event_hash[1]` | `b004206ce289020738ea0e6ee8b3c948872b1b49e90ac287434e731e3fb052b4` |

**Committed as:** embedded in `input-chain.cbor` as event 1, AND committed as the standalone file `input-shred-event.cbor` (737 bytes). A reader may confirm that `input-shred-event.cbor` is byte-identical to the chain bytes for event 1 under this fixture's pins.

---

### Step 4: Cascade report

**Spec citation:** Companion §16.5, §20.4 (OC-76), §20.5 (OC-77) second sentence, §27.3 OC-135 row 3, §28.6; Appendix A.7.

**Load-bearing sentence (§20.5 second sentence):**

> "Backups are governed by the Operator's retention and recovery policy; backups MUST NOT be used to resurrect destroyed plaintext into live derived artifacts."

**Load-bearing sentence (§27.3 OC-135 row 3):**

> "An implementation MUST pass a crypto-shred-cascade test suite that verifies, for each declared purge-cascade scope (§20.5): [...] 3. that backup-resurrection is prevented — no live derived artifact may be restored from backup to a state containing destroyed plaintext."

**Cascade-report structure (extends shred/001).** Top-level dCBOR map with four entries:

| Key | Value |
|---|---|
| `declared_scope` | `["CS-01", "CS-03", "CS-04", "CS-05"]` |
| `backup_snapshot_ref` | 32-byte SHA-256 of `input-backup-snapshot.cbor` bytes |
| `expected_post_state` | per-class map (see below) |
| `target_content_hash` | 32-byte `content_hash[0]` from Step 1 |

dCBOR key ordering: bytewise ascending over UTF-8 key bytes with length-first sorting. Serialized order: `declared_scope` (14 chars, `0x6e` prefix) < `backup_snapshot_ref` (19 chars, `0x73` prefix, `b` < `e` < `t`) < `expected_post_state` (19 chars, `0x73` prefix, `e`) < `target_content_hash` (19 chars, `0x73` prefix, `t`). The footer hex dump confirms.

**Per-class inner map (3 entries — one more than shred/001's 2):**

| Key | Value | Anchor |
|---|---|---|
| `invalidated_or_plaintext_absent` | `true` | §20.4 OC-76 (cascade invalidation obligation) |
| `backup_resurrection_refused` | `true` | §16.5 / §20.5 second sentence / §27.3 OC-135 row 3 |
| `rationale` | `"<CS-NN>-backup-restore-refused-per-§16.5"` | §16.5 citation |

Rationale byte-strings encode `§` as the 2-byte UTF-8 sequence `c2 a7` (39 decoded bytes per rationale). Each inner map is `0xa3` (map of 3); each boolean is `0xf5` (true).

**Operation.** dCBOR-encode the top-level structure (§5.1). Resulting bytes are written to `expected-cascade-report.cbor`.

**Result (658 bytes, sha256 `5171ad4785d4dd4b0f04f36d954336d9ad0a3a55e4c5d144217d5c17d8e73f27`):** first byte `0xa4` (map of 4). Full hex in the footer.

**Committed as:** `expected-cascade-report.cbor`. This is the byte-compare target Test 4's backup-refusal runner MUST reach once the cascade is applied and a restore of `input-backup-snapshot.cbor` is attempted against a live derived artifact of each declared scope class.

---

## Footer — full hex dumps

Each block below is the byte-exact content of the named sibling file.

### `expected-cascade-report.cbor` (658 bytes, sha256 `5171ad4785d4dd4b0f04f36d954336d9ad0a3a55e4c5d144217d5c17d8e73f27`)

```
a46e6465636c617265645f73636f7065846543532d30316543532d3033654353
2d30346543532d3035736261636b75705f736e617073686f745f726566582031
4ac9d52baa1b82e2171ca6cce9c15ad23622c03f6a69b49ba4c789d487aefb73
65787065637465645f706f73745f7374617465a46543532d3031a36972617469
6f6e616c65782743532d30312d6261636b75702d726573746f72652d72656675
7365642d7065722dc2a731362e35781b6261636b75705f726573757272656374
696f6e5f72656675736564f5781f696e76616c6964617465645f6f725f706c61
696e746578745f616273656e74f56543532d3033a369726174696f6e616c6578
2743532d30332d6261636b75702d726573746f72652d726566757365642d7065
722dc2a731362e35781b6261636b75705f726573757272656374696f6e5f7265
6675736564f5781f696e76616c6964617465645f6f725f706c61696e74657874
5f616273656e74f56543532d3034a369726174696f6e616c65782743532d3034
2d6261636b75702d726573746f72652d726566757365642d7065722dc2a73136
2e35781b6261636b75705f726573757272656374696f6e5f72656675736564f5
781f696e76616c6964617465645f6f725f706c61696e746578745f616273656e
74f56543532d3035a369726174696f6e616c65782743532d30352d6261636b75
702d726573746f72652d726566757365642d7065722dc2a731362e35781b6261
636b75705f726573757272656374696f6e5f72656675736564f5781f696e7661
6c6964617465645f6f725f706c61696e746578745f616273656e74f573746172
6765745f636f6e74656e745f6861736858202b81a765657e96ee25b5ce278025
d6d8287014eeda98b98256ae810303247a7e
```

Structure: `0xa4` (map of 4); keys in dCBOR order `declared_scope`, `backup_snapshot_ref`, `expected_post_state`, `target_content_hash`. The `expected_post_state` inner map has 4 entries (`0xa4`) keyed `CS-01`, `CS-03`, `CS-04`, `CS-05`. Each inner value is a 3-entry map (`0xa3`) carrying `rationale` (0x69-prefix text), `backup_resurrection_refused` (0x78 0x1b-prefix text), and `invalidated_or_plaintext_absent` (0x78 0x1f-prefix text). Each boolean is `0xf5` (true).

### `input-backup-snapshot.cbor` (198 bytes, sha256 `314ac9d52baa1b82e2171ca6cce9c15ad23622c03f6a69b49ba4c789d487aefb`)

```
a569736368656d615f6964782375726e3a7472656c6c69733a746573743a6261
636b75702d736e617073686f743a76316c6f726967696e5f636c617373654353
2d30337274616b656e5f61745f747265655f73697a6501737461726765745f63
6f6e74656e745f6861736858202b81a765657e96ee25b5ce278025d6d8287014
eeda98b98256ae810303247a7e766d6174657269616c697a65645f706c61696e
7465787458206261636b75702d7265667573616c2d706c61696e746578742d62
797465730000
```

Structure: `0xa5` (map of 5); dCBOR key order: `schema_id`, `origin_class`, `taken_at_tree_size`, `target_content_hash`, `materialized_plaintext`. `materialized_plaintext` is the 32-byte plaintext from Event 0 — these are the bytes §16.5 forbids restoring into a live derived artifact post-shred.

### `input-shred-event.cbor` (737 bytes, sha256 `81e2020d1eb530732ab1e4798f7ff755fb4ed7a9178b71c0e9f657a3ac4ae337`)

```
d284581ba301270450af9dff525391faa75c8e8da4808b17433a0001000001a0
59027cad66686561646572a96a6576656e745f74797065581b782d7472656c6c
69732d746573742f63727970746f2d73687265646a657874656e73696f6e73f6
6b617574686f7265645f61741a6805a3bc6b7769746e6573735f726566f66e63
6c617373696669636174696f6e5819782d7472656c6c69732d746573742f7368
7265642d666163746e726574656e74696f6e5f74696572006e7461675f636f6d
6d69746d656e74f6726f7574636f6d655f636f6d6d69746d656e74f676737562
6a6563745f7265665f636f6d6d69746d656e74f6676b65795f626167a167656e
7472696573806776657273696f6e016873657175656e63650169707265765f68
6173685820eb2121c2352308277cd64dd9fa1d1bd53f39fc5456852dc2926e96
fbbc2239bf6a657874656e73696f6e73f66b63617573616c5f64657073f66b63
6f6d6d69746d656e7473f66b7061796c6f61645f726566a3656e6f6e63654c00
0000000000000000000000687265665f7479706566696e6c696e656a63697068
657274657874584ca266726561736f6e6d6b65792d64657374726f7965647374
61726765745f636f6e74656e745f6861736858202b81a765657e96ee25b5ce27
8025d6d8287014eeda98b98256ae810303247a7e6c636f6e74656e745f686173
685820db3fb1393fa25ddc6ea81d37ffec7e7681b317bdb19a9bf0082521fdae
8676726c6c65646765725f73636f70655818746573742d73687265642d626163
6b75702d6c65646765726f6964656d706f74656e63795f6b6579506964656d70
2d626b702d7368722d303071617574686f725f6576656e745f68617368582024
9fee7313d56bbc5ec235f895d5a45f6fc576cf36293fe7df12b1125ebe7ede58
40a6caf6d11325191d746612b39c393af29bcf8c15e25bb401c62b5baa0c4523
ffc2c35de2d49d2486d083a4119a8572f2fd9f5065309f66c58f06ae4b66b064
0f
```

The crypto-shred event's COSE_Sign1 envelope in isolation. Structure: `0xd2` (tag 18); `0x84` (array of 4); 27-byte protected header (identical layout to `shred/001` / `projection/001`); empty unprotected; 636-byte EventPayload; 64-byte Ed25519 signature.

### `input-chain.cbor` (1406 bytes, sha256 `b325a18710a6224c3441ed779869db7e488f8b5fa5afafcfb27343784ff8f2e6`)

```
82d284581ba301270450af9dff525391faa75c8e8da4808b17433a0001000001
a0590237ad66686561646572a96a6576656e745f747970655823782d7472656c
6c69732d746573742f6261636b75702d7461726765742d617070656e646a6578
74656e73696f6e73f66b617574686f7265645f61741a6805a3806b7769746e65
73735f726566f66e636c617373696669636174696f6e5819782d7472656c6c69
732d746573742f73687265646461626c656e726574656e74696f6e5f74696572
006e7461675f636f6d6d69746d656e74f6726f7574636f6d655f636f6d6d6974
6d656e74f6767375626a6563745f7265665f636f6d6d69746d656e74f6676b65
795f626167a167656e7472696573806776657273696f6e016873657175656e63
650069707265765f68617368f66a657874656e73696f6e73f66b63617573616c
5f64657073f66b636f6d6d69746d656e7473f66b7061796c6f61645f726566a3
656e6f6e63654c000000000000000000000000687265665f7479706566696e6c
696e656a6369706865727465787458206261636b75702d7265667573616c2d70
6c61696e746578742d627974657300006c636f6e74656e745f6861736858202b
81a765657e96ee25b5ce278025d6d8287014eeda98b98256ae810303247a7e6c
6c65646765725f73636f70655818746573742d73687265642d6261636b75702d
6c65646765726f6964656d706f74656e63795f6b6579506964656d702d626b70
2d7467742d303071617574686f725f6576656e745f6861736858206c4df5f0b4
1972eecfe554dc6ba006c86e6f5d45cfd8aaacd5f3789aadd4daf15840abce2e
b76cb0f7e7e9f59fb59f9d842bf800886635266991b958633a2a3800e48fe6d9
b232339f447807e63bf648e8424fd8acfa42713da6efe6e15d58c69f0cd28458
1ba301270450af9dff525391faa75c8e8da4808b17433a0001000001a059027c
ad66686561646572a96a6576656e745f74797065581b782d7472656c6c69732d
746573742f63727970746f2d73687265646a657874656e73696f6e73f66b6175
74686f7265645f61741a6805a3bc6b7769746e6573735f726566f66e636c6173
73696669636174696f6e5819782d7472656c6c69732d746573742f7368726564
2d666163746e726574656e74696f6e5f74696572006e7461675f636f6d6d6974
6d656e74f6726f7574636f6d655f636f6d6d69746d656e74f6767375626a6563
745f7265665f636f6d6d69746d656e74f6676b65795f626167a167656e747269
6573806776657273696f6e016873657175656e63650169707265765f68617368
5820eb2121c2352308277cd64dd9fa1d1bd53f39fc5456852dc2926e96fbbc22
39bf6a657874656e73696f6e73f66b63617573616c5f64657073f66b636f6d6d
69746d656e7473f66b7061796c6f61645f726566a3656e6f6e63654c00000000
0000000000000000687265665f7479706566696e6c696e656a63697068657274
657874584ca266726561736f6e6d6b65792d64657374726f7965647374617267
65745f636f6e74656e745f6861736858202b81a765657e96ee25b5ce278025d6
d8287014eeda98b98256ae810303247a7e6c636f6e74656e745f686173685820
db3fb1393fa25ddc6ea81d37ffec7e7681b317bdb19a9bf0082521fdae867672
6c6c65646765725f73636f70655818746573742d73687265642d6261636b7570
2d6c65646765726f6964656d706f74656e63795f6b6579506964656d702d626b
702d7368722d303071617574686f725f6576656e745f686173685820249fee73
13d56bbc5ec235f895d5a45f6fc576cf36293fe7df12b1125ebe7ede5840a6ca
f6d11325191d746612b39c393af29bcf8c15e25bb401c62b5baa0c4523ffc2c3
5de2d49d2486d083a4119a8572f2fd9f5065309f66c58f06ae4b66b0640f
```

CBOR array of two COSE_Sign1 envelopes (`0x82` = array of 2, then `0xd2 …` for each envelope). Event 0 (plaintext append) occupies bytes 1..N; event 1 (crypto-shred) occupies bytes N+1..end and is byte-identical to `input-shred-event.cbor` above.

Reader-verifiable intermediates (embedded in the chain bytes):

- Event 0 `content_hash` = `2b81a765657e96ee25b5ce278025d6d8287014eeda98b98256ae810303247a7e`
- Event 0 `canonical_event_hash` = `eb2121c2352308277cd64dd9fa1d1bd53f39fc5456852dc2926e96fbbc2239bf`
- Event 1 `prev_hash` = Event 0 `canonical_event_hash` (§10.2).
- Event 1 `canonical_event_hash` = `b004206ce289020738ea0e6ee8b3c948872b1b49e90ac287434e731e3fb052b4`

The shred declaration embedded inside event 1's `PayloadInline.ciphertext` carries `target_content_hash` = event 0's `content_hash` above. The backup snapshot in `input-backup-snapshot.cbor` binds the same `target_content_hash` AND materializes the 32-byte plaintext that §16.5 forbids restoring once event 1 has been appended.
