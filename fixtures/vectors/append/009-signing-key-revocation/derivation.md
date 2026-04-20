# Derivation — `append/009-signing-key-revocation`

## Header

**What this vector exercises.** The `append/` residue batch closure — the
final two entries of `fixtures/vectors/_pending-invariants.toml`
(`pending_invariants = [3, 6]`). Both invariants are discharged byte-exactly
inside one append-context fixture.

* **Invariant #3 (TR-CORE-037) — signing-key registry Active/Revoked
  lifecycle.** `append/002-rotation-signing-key` already landed the
  Active→Retired half of §8.4's `SigningKeyStatus` enum. This vector lands
  the Active→Revoked half: a single genesis event signed by `issuer-002`
  paired with `input-signing-key-registry-before.cbor` (one entry:
  `issuer-002` `Active`, `valid_to = null`) and
  `input-signing-key-registry-after.cbor` (one entry: `issuer-002`
  `Revoked (3)`, `valid_to = COMPROMISE_TIMESTAMP`). Per §8.4 "Revoked is
  terminal" and per §8.5 the entry MUST persist so the historical event
  remains verifiable; the post-compromise snapshot still resolves
  `kid(issuer-002)` to the same `pubkey` bytes, only with terminal status.
  Together with 002, the two vectors span the §8.4 enum byte-exactly
  (`Active → Retired`, `Active → Revoked`).

* **Invariant #6 (TR-CORE-070) — registry-snapshot binding in manifest.**
  Commits a minimal §14.2-conformant `input-domain-registry.cbor` (one
  `event_type`, one role, one governance-ruleset pointer with its SHA-256
  digest, one classification — all under §14.6's reserved `x-trellis-test/`
  prefix) and a matching §14.3 `input-registry-binding.cbor`
  (`RegistryBinding` CDDL struct) whose `registry_digest` byte-exactly
  equals SHA-256 of the `input-domain-registry.cbor` bytes. The binding
  fields are pinned: `registry_format = 1` (dCBOR per §14.3),
  `registry_version = "x-trellis-test/registry-009-v1"`,
  `bound_at_sequence = 0` per §14.3 "The first binding MUST cover
  `sequence = 0`."

**Scope of this vector.** Structural-only for the payload layer —
single-event, no-rotation, no-re-wrap, empty `KeyBag` (§9.4 admits zero
entries), `PayloadInline` carrying the pinned 64-byte plaintext opaquely.
The revocation claim lands entirely on the signing-key-registry side of §8;
the registry-binding claim lands on a committed `RegistryBinding` artifact
adjacent to the event. No export manifest is assembled here — the §19.2
export-manifest layout is the subject of the `export/` suite (TODO.md
critical-path step 4). §14.3's `registry_digest` byte construction can be
verified end-to-end against the committed input files without a full
export.

**Ledger-scope choice.** `test-revocation-ledger` is deliberately distinct
from every other fixture scope (`test-response-ledger` 001/005,
`test-rotation-ledger` 002, `test-external-ledger` 003, `test-hpke-ledger`
004, `test-posture-ledger` 006/007/008) to avoid collision at
`sequence = 0` per §10.1 / invariant #5 (each ledger scope admits exactly
one canonical event at each sequence position).

**Pinned inputs.**

| Input | Value | Source |
|---|---|---|
| `signing_key` | Ed25519 COSE_Key for `issuer-002`, seed `…00cc` | `../../_keys/issuer-002.cose_key` |
| `payload` | 64 bytes, ASCII `"Trellis fixture payload #001"` + `0x00` padding | `../../_inputs/sample-payload-001.bin` |
| `ledger_scope` | bstr `"test-revocation-ledger"` (22 bytes) | §10.4 |
| `sequence` | `0` | §10.2 genesis |
| `authored_at` | `1745110000` | §12.1; narrative-only |
| `idempotency_key` | bstr `"idemp-append-009"` (16 bytes) | §6.1 |
| Compromise timestamp | `1745110120` — pinned as `valid_to` on `issuer-002`'s post-compromise registry entry | §8.2 |
| `suite_id` | `1` (Phase-1 pin) | §7.1 |
| `PayloadInline.nonce` | 12 bytes of `0x00` | §6.4 |
| `registry_format` | `1` (dCBOR) | §14.3 |
| `registry_version` | `"x-trellis-test/registry-009-v1"` | §14.3; §14.6 prefix |
| `bound_at_sequence` | `0` | §14.3 "first binding MUST cover sequence = 0" |

**Core § roadmap (in traversal order).**

1. §14.2 + §14.3 — build the minimal domain-registry map
   (`event_types`, `role_vocabulary`, `governance`, `classifications`); dCBOR-
   encode; commit as `input-domain-registry.cbor`. Compute
   `registry_digest = SHA-256(input-domain-registry.cbor)`. Build the
   `RegistryBinding` map with that digest and the pinned format/version/
   bound-sequence fields; dCBOR-encode; commit as
   `input-registry-binding.cbor`. **Byte claim (invariant #6):**
   `SHA-256(input-domain-registry.cbor) == RegistryBinding.registry_digest`
   inside `input-registry-binding.cbor`.
2. §8.2 + §8.5 — build the signing-key-registry snapshots. `registry-before`
   is a dCBOR array with one `SigningKeyEntry` for `issuer-002`
   (`status = Active (0)`, `valid_from = authored_at`, `valid_to = null`,
   `supersedes = null`). `registry-after` is a dCBOR array with the same
   `kid` / `pubkey` / `suite_id`, `status = Revoked (3)`,
   `valid_to = COMPROMISE_TIMESTAMP`. Per §8.4 "Revoked is terminal"; per
   §8.5 the entry MUST persist so historical material remains verifiable.
   Commit both as `input-signing-key-registry-{before,after}.cbor`. **Byte
   claim (invariant #3):** the `kid` / `pubkey` / `suite_id` fields in both
   snapshots are byte-identical; only `status` and `valid_to` differ, and
   the after-snapshot's `status` field carries the integer `3` (the CDDL
   `Revoked` discriminant per §8.4).
3. §9.3 + §9.1 — `content_hash` over the 64-byte `PayloadInline.ciphertext`.
4. §6.8 (authored) + §9.5 + Appendix A — build the `AuthorEventHashPreimage`;
   dCBOR-encode; commit as `input-author-event-hash-preimage.cbor`.
5. §9.5 + §9.1 — `author_event_hash = SHA-256(len-prefix || "trellis-author-
   event-v1" || len || authored-bytes)`; commit as `author-event-hash.bin`
   (and the 4+24+4+540 = 571-byte domain-separated preimage as
   `author-event-preimage.bin`).
6. §6.1 — build the canonical-form `EventPayload` including the computed
   `author_event_hash`; dCBOR-encode; commit as
   `expected-event-payload.cbor`.
7. §7.4 + RFC 9052 §4.4 — protected-header map
   `{1: -8, 4: kid(issuer-002), -65537: 1}`; dCBOR-encode; `Sig_structure =
   ["Signature1", protected, h'', EventPayload]`; dCBOR-encode. Commit as
   `sig-structure.bin`.
8. §7.1 — Ed25519 signature over `Sig_structure` under `issuer-002`'s seed;
   64 bytes per RFC 8032 §5.1.6.
9. §6.1 + §7.4 — tag-18 `COSE_Sign1` envelope `[protected, {}, payload,
   signature]`; dCBOR-encode. Commit as `expected-event.cbor`.
10. §9.2 + §10.6 — build `CanonicalEventHashPreimage`; dCBOR-encode;
    domain-separated SHA-256 with tag `"trellis-event-v1"` produces
    `canonical_event_hash`. Wrap in `AppendHead = {scope, sequence,
    canonical_event_hash}`; dCBOR-encode. Commit as
    `expected-append-head.cbor`.

## Construction-to-matrix-row mapping

* **TR-CORE-037** — "Exports MUST include a signing-key registry snapshot
  (`SigningKeyEntry`, Active/Revoked lifecycle) so verification is
  self-contained at any future date."
  Covered by steps (2) — the `registry-after` snapshot carries a
  `SigningKeyEntry` whose `status = Revoked (3)` and whose `kid` resolves the
  event's protected-header `kid`; per §8.5 the entry persists and preserves
  historical verifiability.
* **TR-CORE-070** — "The export manifest MUST include a content-addressed
  digest of the domain registry (event taxonomy, role vocabulary, governance
  rules) in force at the time of signing."
  Covered by step (1) — `input-registry-binding.cbor` contains a
  content-addressed SHA-256 `registry_digest` of `input-domain-registry.cbor`
  whose §14.2 categories include event taxonomy, role vocabulary, governance
  rules (ruleset identifier + digest), and classification vocabulary.

## Invariant reproduction checklist

* `SHA-256(input-domain-registry.cbor)` equals the `registry_digest` bstr
  inside `input-registry-binding.cbor` (invariant #6 primary claim).
* `input-registry-binding.cbor` is a dCBOR map with exactly the four §14.3
  fields (`registry_digest`, `registry_format`, `registry_version`,
  `bound_at_sequence`); `bound_at_sequence == 0` per §14.3 first-binding
  rule.
* `input-signing-key-registry-after.cbor` decodes to a single-element array
  whose `SigningKeyEntry.status == 3` (Revoked) and whose `valid_to` equals
  `COMPROMISE_TIMESTAMP`.
* `kid` / `pubkey` / `suite_id` are byte-identical across `input-signing-
  key-registry-before.cbor` and `-after.cbor` — the revocation changes
  `status` and `valid_to`, not identity.
* `expected-event.cbor` is a tag-18 `COSE_Sign1` whose protected-header `kid`
  equals the `kid` in both registry snapshots; verification against the
  `registry-after` snapshot succeeds for events with
  `authored_at < valid_to` (per §8.4 "verification of historical records"
  rule for terminal-status entries).

## What this vector does NOT cover

* LAK rotation and `LedgerServiceWrapEntry` append-only mechanics (§8.6) —
  exercised by the HPKE-wrap vectors in `append/004` and tamper-case
  successors.
* Registry migration discipline (§14.5 — a second `RegistryBinding` bound at
  a later sequence in the same scope) — deferred to the `export/` suite that
  TODO.md critical-path step 4 will build.
* Full §19.2 export-manifest assembly — this vector commits the
  `RegistryBinding` artifact byte-exactly without assembling the enclosing
  manifest. The `export/` suite will assemble manifests end-to-end.

---

**Traceability.** This vector is the closure of `fixtures/vectors/
_pending-invariants.toml` `pending_invariants`: with 009 landed the two
remaining invariant numbers (#3, #6) have byte-testable vector coverage and
the allowlist entry `pending_invariants` drops to `[]`.
