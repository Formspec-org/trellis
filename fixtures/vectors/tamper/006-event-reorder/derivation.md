# Derivation — `tamper/006-event-reorder`

## Header

**What this vector exercises.** Second expanded-tamper case (TODO.md
task #12 per the "Expanded `tamper/` suite" critical-path step). Exercises
Core §19 step 4.h `prev_hash` linkage, via the tamper-kind enum row
`event_reorder` pinned in `tamper/001`'s derivation:

> `event_reorder` | Two adjacent events are swapped; the later event's
> `prev_hash` no longer matches the now-earlier event's canonical hash |
> step 4.h | Variant of `prev_hash_break` with the additional property
> that `sequence` monotonicity is also broken.

The construction reuses byte-exact upstream events:

* `append/001-minimal-inline-payload` — genesis, `sequence = 0`,
  `prev_hash = null`, `canonical_event_hash =
  ef2622f1470ba3d9c24b47c0566cab8902b6500fbb3d47bdd77aae068e724ddb`.
* `append/005-prior-head-chain` — non-genesis, `sequence = 1`,
  `prev_hash = <append/001 canonical hash>`, `canonical_event_hash =
  3d3d5aeb5d4b8d972adbddfe0f339a94fffe01bf90ac1648be2eb98d4acc9f17`.

**The tamper.** Swap the order inside the ledger array — the tampered
ledger is `[append/005 event, append/001 event]` (follower at index 0,
genesis at index 1). No event bytes are mutated; every event individually
signs cleanly, decodes cleanly, and its recomputed `author_event_hash` /
`canonical_event_hash` match the upstream-pinned values.

## Core §19 verifier walk

* **Index 0** — survivor bytes are `append/005`'s envelope (`sequence = 1`,
  `prev_hash = ef2622f1…4ddb`).
    * Step 4.a — kid resolves in registry. PASS.
    * Step 4.b — COSE_Sign1 signature verifies. PASS.
    * Step 4.c — EventPayload decodes. PASS.
    * Step 4.d — `author_event_hash` recomputes to bytes already pinned
      upstream. PASS.
    * Step 4.e — `canonical_event_hash` recomputes to `3d3d5aeb…9f17`.
      PASS.
    * Step 4.h — `sequence = 1`, so verifier looks up `events[0]`.
      `events[0]` IS the survivor itself, whose canonical hash
      `3d3d5aeb…9f17` does NOT equal `payload.prev_hash = ef2622f1…4ddb`.
      **FAIL** — recorded in `event_failures`.
* **Index 1** — envelope is `append/001`'s genesis (`sequence = 0`,
  `prev_hash = null`).
    * Steps 4.a–4.e — all pass; byte-exact upstream fixture.
    * Step 4.h — `sequence = 0`, check `prev_hash == null`. PASS.

Step 9's `integrity_verified` AND-conjunction drops to `false` via
"prev_hash links … valid". `structure_verified = true`;
`readability_verified = true` (Phase 1 PayloadInline structural-only).

## Construction-to-matrix-row mapping

* **TR-CORE-020** — "… exactly one canonical append-attested order MUST
  exist per governed scope (invariant #5)." An ordered ledger whose
  indices do not realize monotonic `sequence` violates the
  single-canonical-order claim; §19 step 4.h surfaces it as
  `integrity_verified = false`. Shares this row with
  `append/005-prior-head-chain` (positive construction) and
  `tamper/005-chain-truncation` (truncation-side violation).
* **TR-CORE-023** — "Canonical order MUST be determined solely by this
  specification and the applicable binding; MUST NOT depend on wall-clock
  receipt time, queue depth, worker identity, or other operational
  accidents." §10.2 pins that order-determinant to the prev_hash
  linkage chain; a reorder severs that chain and leaves the ledger with
  no spec-determined canonical order. Parallel to append/005 which pins
  this row on the positive side (prev_hash-determined ordering). The
  symmetric TR-CORE-023 claim on `tamper/005-chain-truncation` landed
  alongside this vector's review-fix pass.

## Distinguishes from tamper/005-chain-truncation

* tamper/005 — one event absent. Ledger has length 1; step 4.h at the
  survivor finds `events[0]` = itself; failing_event_id = survivor;
  `tamper_kind = event_truncation`; auxiliary pin
  `omitted_event_id = <genesis canonical hash>`.
* tamper/006 — both events present, order swapped. Ledger has length 2;
  step 4.h at index 0 finds `events[0]` = itself; failing_event_id =
  event at index 0 (now the follower); `tamper_kind = event_reorder`;
  auxiliary pin `dangling_prev_hash = <genesis canonical hash>`.

Same step (§19 4.h) — different enum row in the conformance report so a
consumer can tell "one event is missing" from "events are present but
ordered wrong."

## Invariant reproduction checklist

* `sha256(input-tampered-event-at-index-0.cbor)` equals
  `416d5e6190d0ec8ad791437f7e4bdb369f751b11dcb3597a5f2911421529aac9` —
  byte-identical to `append/005/expected-event.cbor`.
* `sha256(input-tampered-event-at-index-1.cbor)` equals
  `8d18bcd820945b4c5575a44823d79685858914ee5893ac3c9e4b8ec183273815` —
  byte-identical to `append/001/expected-event.cbor`.
* `input-tampered-ledger.cbor` decodes as a 2-element CBOR array; array
  index 0 carries the `sequence = 1` envelope; array index 1 carries the
  `sequence = 0` envelope.
* The event at index 0's recorded `prev_hash` equals
  `ef2622f1…4ddb` — the genesis canonical hash — which does NOT equal the
  event at index 0's own recomputed `canonical_event_hash`
  (`3d3d5aeb…9f17`).
* The event at index 1's recorded `prev_hash` is `null`; its `sequence`
  is `0`; step 4.h passes trivially.

## What this vector does NOT cover

* `prev_hash_break` via mutated-bytes rather than swap — a separate vector
  would construct an event with a mutated `prev_hash` field, which the
  signature would then fail to verify first (step 4.b), so the tamper
  surfaces as `signature_invalid` unless a re-signing happens under a
  malicious key (which is the `signature_invalid` / `kid_misresolution`
  family, not `prev_hash_break`).
* Non-adjacent reorder — this vector swaps the only two events. A three-
  or-more-event chain with a non-adjacent swap is a future residue-of-
  residue case.
* Wrong-scope reorder — events with differing `ledger_scope` fields; that
  is a separate tamper family surfaced by §19 step 4.f (`ledger_scope`
  match), distinct from §19 step 4.h.

---

**Traceability.** TR-CORE-020 (invariant #5 — single canonical order per
governed scope); Core §19 step 4.h (prev_hash linkage check); tamper-kind
enum row `event_reorder` as pinned in `tamper/001/derivation.md`.
