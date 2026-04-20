# Derivation — `tamper/007-hash-mismatch`

## Header

**What this vector exercises.** Third expanded-tamper case (TODO.md
task #12). Exercises Core §19 step 4.d — `author_event_hash`
recomputation disagreement with the payload's recorded value — per the
tamper-kind enum pinned in `tamper/001`'s derivation:

> `hash_mismatch` | `author_event_hash` or `canonical_event_hash`
> recomputation disagrees with the payload's recorded value | step
> 4.d / 4.e | Payload-field tamper that the author forgot to re-sign;
> §19's recomputation of the hash catches it.

The enum entry's "that the author forgot to re-sign" is the *intent* the
detection surface catches — but this vector DOES re-sign, which is what
isolates the test from `signature_invalid`. The signature is *legitimately
valid* under `issuer-001`'s real seed over the tampered payload bytes; §19
step 4.b therefore passes cleanly. The tamper surfaces only at step 4.d.

**The tamper.** `append/001-minimal-inline-payload`'s byte-exact
`EventPayload` has its `author_event_hash` field mutated at byte 0 by
`XOR 0x01` (the minimum-surface mutation, parallel to `tamper/001`'s
single-byte signature flip). The payload is then re-dCBOR-encoded and
re-signed under issuer-001's real seed.

  * Upstream `author_event_hash`   = `f1eeb3d6…dca6`
  * Tampered `author_event_hash`   = `f0eeb3d6…dca6`  (byte 0: `f1` → `f0`)

The `AuthorEventHashPreimage` inputs — `header`, `payload_ref`, `key_bag`,
`idempotency_key`, `extensions`, `content_hash`, `prev_hash`,
`causal_deps`, `sequence`, `ledger_scope`, `version` — are UNCHANGED vs.
`append/001`. §9.5 / §9.1's domain-separated SHA-256 recomputation over
those inputs therefore produces the UPSTREAM hash, not the tampered one.
Mismatch is surfaced by §19 step 4.d.

## Core §19 verifier walk

| Step | Action | Result |
|---|---|---|
| 4.a | Resolve protected-header `kid` via embedded registry. | **PASS** — `kid(issuer-001)` present (entry byte-identical to tamper/001/005/006). |
| 4.b | Verify COSE_Sign1 signature over Sig_structure (§7.4). | **PASS** — signature is a legitimate Ed25519 signature under `issuer-001`'s real seed over the *tampered* payload bytes. |
| 4.c | Decode payload as `EventPayload` (§6.1); reject unknown fields. | **PASS** — tampered payload is valid dCBOR with only known fields. |
| 4.d | Recompute `author_event_hash` per §9.5 from the preimage. | **FAIL** — recomputed `f1eeb3d6…` ≠ stored `f0eeb3d6…`. Recorded in `event_failures`. |
| 4.e | `canonical_event_hash` recomputation against optional `AppendHead`. | Not exercised in this vector — no `AppendHead` committed as expected input. |
| 4.h | `prev_hash` linkage. | PASS — `sequence = 0`, `prev_hash == null`. |
| 9 | Final verdict conjunction. | `integrity_verified = false` via "hash recomputations … match". |

`structure_verified = true` (payload decodes cleanly through step 4.c).
`readability_verified = true` (Phase 1 PayloadInline structural-only; no
payload decryption attempted).

## Construction-to-matrix-row mapping

* **TR-CORE-020** (invariant #5, single canonical order per scope) —
  the stored `author_event_hash` and the recomputation disagreeing would,
  if admitted, give the same payload two canonical identities inside one
  scope. Step 4.d surfaces this before admission; the matrix row's
  "exactly one canonical … order" claim is what step 4.d protects.
* **TR-CORE-023** (invariant #5, canonical order independent of wall-
  clock) — §10.2 pins canonical order to `prev_hash` linkage, which is
  ultimately anchored in `author_event_hash` and `canonical_event_hash`
  recomputations being stable functions of the payload bytes. Tampering
  the stored hash breaks that stability claim. Parallel to `append/005`
  which pins this row on the positive side.
* **TR-CORE-061** (verifier-integrity independence) — shared with
  `tamper/001`: §19 step 4.d's hash recomputation is offline,
  deterministic, and does not depend on any runtime state. A verifier
  catches the tamper without reference to derived or service-side state.

## Distinguishes from prior tamper cases

* **tamper/001** (`signature_invalid`) — signature byte flipped; step 4.b
  fails. Here step 4.b PASSES (we re-signed); step 4.d is the surface.
* **tamper/005** (`event_truncation`) — one-event ledger (follower only);
  step 4.h fails on dangling prev_hash. Here ledger is one-event too, but
  `sequence = 0` so step 4.h passes trivially.
* **tamper/006** (`event_reorder`) — two events in swapped order; step
  4.h fails at index 0. Here there is no second event.
* **tamper/007** (`hash_mismatch`, this vector) — step 4.d fails; all
  other steps pass.

## Invariant reproduction checklist

* `sha256(input-tampered-event.cbor)` = `f5e75213…7885` (692-byte tag-18
  envelope, distinct from `append/001`'s `expected-event.cbor` because
  the payload bytes and signature both changed).
* `input-tampered-event-payload.cbor`'s `author_event_hash` field decodes
  to `f0eeb3d6…dca6` — byte 0 is `0xf0`, not `0xf1`.
* Recomputing `author_event_hash` per §9.5 (domain-separated SHA-256
  tag `"trellis-author-event-v1"` over `dCBOR(AuthorEventHashPreimage)`)
  from the payload's preimage fields produces `f1eeb3d6…dca6` — byte 0
  is `0xf1`, matching `append/001`'s upstream value.
* The two hashes differ at byte 0 only (bits: `f1` vs `f0`); every other
  byte is identical. The one-bit XOR is the minimum-surface mutation.
* The signature in `input-tampered-event.cbor` verifies under
  `issuer-001` over `sig-structure.bin`.

## What this vector does NOT cover

* `canonical_event_hash` recomputation (step 4.e) surface — reachable by
  the same class of tamper against the stored `canonical_event_hash`
  field (which `CanonicalEventHashPreimage` in §9.2 hashes via a
  different preimage shape). A follow-up residue-of-residue vector could
  exercise step 4.e alone; this vector covers step 4.d.
* Payload-field tamper that the author *forgot* to re-sign — the enum
  entry's paradigm case. That is a conflation of `signature_invalid`
  and `hash_mismatch` and is covered implicitly by `tamper/001`.
* Multi-event chain hash mismatch — if a later event in the chain has a
  tampered `author_event_hash`, the tamper surfaces at step 4.d on that
  event, not a new failure class.

---

**Traceability.** TR-CORE-020 (invariant #5 single canonical order),
TR-CORE-023 (invariant #5 order determined solely by spec + binding),
TR-CORE-061 (verifier-integrity independence); Core §19 step 4.d (hash
recomputation); §9.5 / §9.1 (author_event_hash construction); tamper-kind
enum row `hash_mismatch` as pinned in `tamper/001/derivation.md`.
