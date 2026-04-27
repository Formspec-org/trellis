# Derivation — `append/031-key-entry-signing-lifecycle`

## Header

**What this vector exercises.** Re-pins the rotation lifecycle of
`append/002-rotation-signing-key` under the executed unified `KeyEntry`
encoding (Core §8.7 / ADR 0006). The semantic claim is the same as 002's
TR-CORE-036 + TR-CORE-038 — Event A's `author_event_hash` is invariant
under signing-key rotation; both pre- and post-rotation events verify
against the post-rotation registry snapshot — but the registry CBOR is
the new flat `KeyEntrySigning` arm with a top-level `kind: "signing"`
discriminator (Core §8.7.1).

Per ADR 0006 *Wire preservation*: this vector is the load-bearing positive
for TR-CORE-039 (unified taxonomy + verifier dispatch on `kind`). The
registry CBOR bytes are NOT byte-equal to 002's flat-shape `SigningKeyEntry`
encoding because the new `kind` field changes the dCBOR map header from
8 entries (`0xa8`) to 9 (`0xa9`) and adds the canonical-ordered key
`kind: "signing"`. The migration is an explicit registry-snapshot wire
evolution; 002's pre-migration bytes remain valid in their own corpus
slot (verifier dispatch on absence of `kind` lands them on the legacy
`SigningKeyEntry` path, Core §8.2).

**Ledger scope.** `test-key-entry-ledger` (21 bytes) is distinct from every
other fixture's ledger scope: per §10.1 + invariant #5 each ledger scope
admits exactly one canonical event at each sequence position; isolating
031 in its own scope keeps existing fixtures' invariant #5 claims
untouched.

**Pinned inputs.**

| Input | Value | Source |
|---|---|---|
| `signing_key_a` (pre-rotation) | Ed25519 COSE_Key for `issuer-001` (shared with 001/002/005) | `../../_keys/issuer-001.cose_key` |
| `signing_key_b` (post-rotation) | Ed25519 COSE_Key for `issuer-002` (shared with 002/009) | `../../_keys/issuer-002.cose_key` |
| `payload` | 64 bytes ASCII `"Trellis fixture payload #001"` + `0x00` padding | `../../_inputs/sample-payload-001.bin` |
| `ledger_scope` | bstr `"test-key-entry-ledger"` (21 bytes) | §10.4 |
| Event A `sequence` | `0` | §10.2 genesis |
| Event A `authored_at` | `1745120000` | §12.1 narrative |
| Event A `idempotency_key` | bstr `"idemp-append-031a"` (17 bytes) | §6.1 |
| Rotation timestamp | `1745120060` (= issuer-001's `valid_to`, issuer-002's `valid_from`) | §8.2 |
| Event B `sequence` | `1` | §10.2 |
| Event B `authored_at` | `1745120120` | §12.1 narrative |
| Event B `idempotency_key` | bstr `"idemp-append-031b"` (17 bytes; distinct from A per §17.3) | §6.1 / §17.3 |
| `suite_id` | `1` (Phase-1 pin, shared by both issuers) | §7.1 |

**Core § roadmap.**

1. §9.3 + §9.1 — `content_hash` over the 64-byte ciphertext (identical to 001/002).
2. §6.8 (authored), §9.5, Appendix A — Event A's `AuthorEventHashPreimage`.
3. §9.5 + §9.1 — Event A's `author_event_hash`.
4. §6.8 (canonical) + §6.1 — Event A's `EventPayload`.
5. §7.4 — Event A protected header + `Sig_structure`; §7.1 Ed25519; §6.1 + §7.4 tag-18 envelope.
6. §9.2 + §10.6 — Event A `canonical_event_hash` + `AppendHead`.
7. **§8.7.1 — `signing-key-registry-before.cbor` (new shape).** One `KeyEntrySigning` entry: `{kind: "signing", attestation: null, kid, pubkey, status: 0, suite_id: 1, supersedes: null, valid_from, valid_to: null}`. Map header `0xa9` (nine entries; legacy 002 uses `0xa8` for eight).
8. **§8.7.1 + §8.4 — `signing-key-registry-after.cbor` (new shape).** Two-element array: issuer-001 transitions Active → Retired with `valid_to = ROTATION_TIMESTAMP`; issuer-002 entered Active with `supersedes = kid(issuer-001)`. Both rows carry `kind: "signing"`.
9. **Invariant #7 reproduction (TR-CORE-038).** §9.5 has no `kid` / no signing-key-registry reference; the registry transition between Step 7 and Step 8 changes zero bytes of Event A's `AuthorEventHashPreimage`. Re-running Steps 2–3 after the new-shape registry transition yields byte-identical `author_event_hash(A)`. The generator asserts this in-script (`a_author_hash == a_author_hash_2`); a failing assertion blocks emission.
10. §10.2 — Event B `prev_hash = canonical_event_hash(A)`. The §10.2 linkage pins to canonical-event-hash bytes, not to a signing kid; the registry shape change does not affect it.
11. §6.8, §9.5, §9.2, §7.4, §7.1, §6.1, §10.6 — Event B's authored / canonical / signed surfaces under issuer-002.

## Verifier dispatch (TR-CORE-039)

A verifier loads `input-signing-key-registry-after.cbor`, decodes it as a
dCBOR array, and dispatches per Core §8.7.3 step 1. For each entry the
top-level `kind` field is present (text-string `"signing"`); the verifier
applies the §8.7.3 step 3 signing-arm field-set validation (§8.2 parity)
and inserts the kid into the resolvable signing-key map. Both Event A
(kid = `af9dff525391faa75c8e8da4808b1743`) and Event B (kid =
`3d05ee9ced8f29b60ef84b17d4712e24`) resolve to entries whose pubkey
verifies their COSE_Sign1 signature; lifecycle (`Retired` for A — §8.4
permits historical verification; `Active` for B) does not reject either.

This is the byte-level claim of TR-CORE-039: a verifier reading the unified
shape produces the same verification outcome as one reading the legacy
shape (002), with the only on-wire difference being the `kind` field.

## Invariant → byte mapping

| Invariant / TR-CORE row | Where in 031's bytes |
|---|---|
| TR-CORE-039 (KeyEntry taxonomy + dispatch) | Both registry rows in `input-signing-key-registry-{before,after}.cbor` carry the canonical-ordered text key `kind: "signing"`; the dCBOR map header is `0xa9` (9 entries); the per-entry byte sequence proves the verifier dispatch on `kind` resolves to the signing-arm validator and produces signing-equivalent output to the legacy shape. |
| TR-CORE-038 (key_bag / author_event_hash immutable) | `input-pre-rotation-author-event-hash.bin` (32 bytes) is the SHA-256 of the domain-separated dCBOR of Event A's `AuthorEventHashPreimage`. Steps 7–8 mutate the registry snapshot (new-shape this time); Steps 2–3 re-execute and produce byte-identical bytes. The generator assertion (`a_author_hash == a_author_hash_2`) is the mechanical restatement; the normative source is Core §9.5's last sentence ("immutable under rotation because none of its inputs is altered by service-side re-wraps"). |
| TR-CORE-036 (verifier resolves prior signature after rotation) | `input-pre-rotation-event.cbor` (kid = `af9dff525391faa75c8e8da4808b1743`) and `expected-event.cbor` (kid = `3d05ee9ced8f29b60ef84b17d4712e24`) both resolve their kids against `input-signing-key-registry-after.cbor`. The new registry shape carries `issuer-001` with `kind: "signing"`, `status: 2` (Retired); §8.4 admits historical verification under Retired status. |
| TR-CORE-037 (signing-key registry snapshot is self-contained) | `input-signing-key-registry-after.cbor` carries every kid referenced by the events plus `issuer-002`'s `supersedes = kid(issuer-001)`; §8.5 transitive resolvability holds for both kids. |

## Core-gap notes

The load-bearing Core claim for this vector — TR-CORE-039 dispatch + the
new-shape registry CBOR — reproduces unambiguously from §8.7's prose and
§28's CDDL. No Core gap is claimed from 031.

Two observations worth logging:

- **Map header byte.** `0xa9` (9 entries) is the discriminator between the
  new flat-signing arm and the legacy `0xa8` (8 entries) shape. A verifier
  detecting `kind` field presence rather than counting map entries
  produces identical behavior; the `kind` test is the normative dispatch
  per §8.7.3 step 1.
- **`SigningKeyStatus` encoding unchanged.** Both shapes carry the same
  enum values (Active=0, Rotating=1, Retired=2, Revoked=3). 031 exercises
  Active and Retired; the legacy 002 vector covers the same path under the
  flat shape. The Active→Revoked path is exercised by 009 under the legacy
  shape; a follow-on may re-pin 009 in the new shape if/when ADR 0006
  Phase-2+ activation requires it.
