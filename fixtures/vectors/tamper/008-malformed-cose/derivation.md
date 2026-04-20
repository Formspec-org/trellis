# Derivation — `tamper/008-malformed-cose`

## Header

**What this vector exercises.** Fourth expanded-tamper case (TODO.md task
#12). Exercises §19's structural-identification surface via the
tamper-kind enum row `malformed_cose` pinned in `tamper/001`'s derivation:

> `malformed_cose` | COSE_Sign1 envelope is structurally invalid (wrong
> array length, wrong tag, malformed bstr length prefix, absent or nil
> payload) | step 4.c (and step 4.b if decode proceeds) | Fatal-
> classification candidate; §19's "Failure classes" prose names decode
> failures as aborts if they corrupt `structure_verified` globally.

**The tamper.** `append/001-minimal-inline-payload/expected-event.cbor`
byte 0 is flipped from `0xd2` (CBOR tag 18 = COSE_Sign1 per RFC 9052
§4.2) to `0xd1` (CBOR tag 17 = COSE_Mac0). Every other byte — including
the protected-header bstr, the unprotected-header map, the payload bstr,
and the signature bstr — is byte-identical to the upstream envelope.

## Construction discipline

RFC 9052 §4.4's `Sig_structure = ["Signature1", protected, external_aad,
payload]` is the input to signature computation. The outer CBOR tag
number is NOT in `Sig_structure`. Therefore:

  1. The tampered bytes' protected+payload surface is byte-identical to
     the upstream.
  2. The signature bytes (array index 3) are byte-identical to the
     upstream.
  3. If a lenient verifier ignored the outer tag and proceeded to §19
     step 4.b signature verification, the signature would still verify.

This makes the tamper an isolation test of the verifier's **structural-
identification** obligation: §19 identifies each item in `010-events.cbor`
as a "COSE_Sign1", which per RFC 9052 §4.2 is tag 18. A tag-17 item is
not a COSE_Sign1; the verifier MUST reject. The tamper surfaces *before*
step 4.a — at the "identify as COSE_Sign1" gate that every §19
event-loop iteration performs implicitly.

## §19 verifier walk

| Step | Action | Result |
|---|---|---|
| pre-4 | Identify item at `010-events.cbor[0]` as COSE_Sign1 per RFC 9052 §4.2 (tag 18). | **FAIL** — item is tag 17. |
| 4.a | Resolve protected-header kid. | Not reached. |
| 4.b | Verify COSE signature. | Not reached. (Would have PASSED had it been reached — signature is the real issuer-001 signature over the upstream payload.) |
| 4.c | Decode payload as EventPayload. | Not reached. |
| 4.d, 4.e, 4.f, 4.h | Hash / chain / scope checks. | Not reached. |
| 9 | Final verdict conjunction. | `structure_verified = false` already. |

`integrity_verified = false` and `readability_verified = false` follow by
§19 step 9's AND-conjunction when `structure_verified` is false — none of
the integrity / readability inputs were computed, so the conjunctive
predicates cannot hold.

## Construction-to-matrix-row mapping

* **TR-CORE-035** — "Every signed artifact MUST carry a `suite_id`; the
  spec MUST name the Phase 1 suite (Ed25519/COSE_Sign1 or equivalent)".
  COSE_Sign1 identity is carried at two layers: the CBOR tag number
  (RFC 9052 §4.2) and the protected-header `suite_id` (Core §7.4). This
  vector pins the outer-tag layer: a tag-17 item carrying valid tag-18
  signed bytes is still not a COSE_Sign1 for §19's purposes.

* **TR-CORE-060** — "A conforming Verifier MUST verify authored
  authentication where required, canonical append attestation validity
  and inclusion consistency, and distinguish author-originated facts,
  canonical records, canonical append attestations, and disclosure/export
  artifacts." A verifier that cannot distinguish COSE_Sign1 from
  COSE_Mac0 at the structural-identification layer fails this
  distinguishability requirement (COSE_Mac0 is a MAC'd structure, not a
  signed one — a verifier would be attesting authorship where none is
  present in the RFC 9052 sense).

* **TR-CORE-061** (shared with `tamper/001` and `tamper/007`) — verifier-
  integrity independence: the tag check is offline, deterministic, and
  does not depend on any runtime state. The tamper is catchable by
  byte-pattern inspection.

## Distinguishes from prior tamper cases

* **tamper/001** (`signature_invalid`) — signature bytes flipped; §19
  step 4.b fails. Here the signature is UNCHANGED; step 4.b would pass
  if reached.
* **tamper/005** (`event_truncation`) — ledger missing an event; step
  4.h dangling prev_hash. Here the ledger has its one event; the tamper
  is on that event, not on the set.
* **tamper/006** (`event_reorder`) — two events swapped; step 4.h. Here
  only one event.
* **tamper/007** (`hash_mismatch`) — stored `author_event_hash` field
  flipped and re-signed; step 4.d fails but step 4.b passes. Here the
  stored hash is unchanged; structure fails before step 4.d.
* **tamper/008** (this vector) — outer CBOR tag flipped; structure fails
  before any step-4.x check. The earliest tamper detection in the §19
  algorithm.

## Invariant reproduction checklist

* `input-tampered-event.cbor` is 691 bytes; byte 0 is `0xd1`; bytes 1..690
  are byte-identical to `append/001/expected-event.cbor` bytes 1..690.
* `sha256(input-tampered-event.cbor)` =
  `029603f0cf90aa28a8d43f4a07406dadfdc68ccc1f5653db707241780330abb4`.
* Decoding `input-tampered-event.cbor` as generic CBOR succeeds and
  returns a `CBORTag(tag=17, value=[protected, {}, payload, signature])`.
  The CBOR layer is NOT malformed; only the COSE-layer identity is.
* Decoding as COSE_Sign1 (`RFC 9052 §4.2`, tag 18) MUST fail because the
  tag number is 17.
* The embedded payload bstr (array index 2) and signature bstr (array
  index 3) are byte-identical to `append/001`'s envelope's
  corresponding positions. If extracted and re-tagged as tag 18, they
  would form a valid COSE_Sign1.

## What this vector does NOT cover

* Malformed array length (e.g., 3-element array instead of 4) — catchable
  at a different point in decode; future residue-of-residue vector
  candidate.
* Malformed bstr length prefix — would produce a CBOR-level decode
  failure (not just COSE-level). Different failure class than this
  vector's semantic-identity tamper.
* Absent or nil payload (`payload == nil` per Core §7.4) — Core §7.4
  explicitly says "verifier MUST reject `payload == nil`"; that's a
  separate tamper case targeting a different paragraph.
* Wrong `suite_id` in the protected header — `tamper_kind =
  suite_unsupported`, a separate enum row not yet vectored.

---

**Traceability.** TR-CORE-035 (signature-suite identity), TR-CORE-060
(verifier fact-class distinguishability), TR-CORE-061 (verifier
integrity independence); RFC 9052 §4.2 (COSE_Sign1 tag number); Core
§7.4 (protected-header layer); Core §19 (structural-identification
pre-step); tamper-kind enum row `malformed_cose` as pinned in
`tamper/001/derivation.md`.
