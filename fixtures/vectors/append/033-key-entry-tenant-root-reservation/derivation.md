# Derivation — `append/033-key-entry-tenant-root-reservation`

## Header

**What this vector exercises.** Phase-1 reservation positive for `kind = "tenant-root"` (ADR 0006 / Core §8.7.4).
A two-entry signing-key registry composes the **mixed-shape** acceptance
rule of Core §8.7: row 0 is the legacy `SigningKeyEntry` flat shape (no
`kind` field — verifier dispatches per §8.7.3 step 1 on absence of `kind`
to the §8.2 path); row 1 is a `KeyEntryNonSigning` row of `kind = "tenant-root"`
with the minimal `attributes` map per §8.7.2.

The genesis event in scope `test-key-entry-tenant-root-ledger` is signed by `issuer-001` and
verifies through the legacy-shape registry resolution path. The verifier
emits the non-signing row's kid into the parallel `NonSigningKeyEntry`
map (Core §8.7.3 step 3) — this map is consulted only on
`unresolvable_manifest_kid` paths to discriminate `key_class_mismatch`
from generic resolution failures (relevant to `tamper/023`); for this
positive vector the event's protected-header kid resolves to row 0 so
the non-signing row participates only in the structural-shape gate
(attributes-is-a-map check).

**TR-CORE-047 byte claim.** Verification of `expected-event.cbor` against
`input-signing-key-registry.cbor` yields `integrity_verified = true`
despite the non-signing row's presence: the Phase-1 verifier admits the
reservation without rejection. The Phase-1 lint
(`scripts/check-specs.py` rule R18) emits a warning naming this vector
and the `kind = "tenant-root"` declaration. Phase-2+ activation lifts the
warning per ADR 0006 §"Phase 2 evolution".

**Ledger scope.** `test-key-entry-tenant-root-ledger` is distinct from every other fixture's scope
so this vector's `sequence = 0` claim does not collide with any existing
genesis-event invariant under §10.1 + invariant #5.

## Pinned inputs

| Input | Value | Source |
|---|---|---|
| `signing_key` (legacy-shape signer) | Ed25519 COSE_Key for `issuer-001` (shared with 001/002/005/009/031) | `../../_keys/issuer-001.cose_key` |
| `payload` | 64 bytes ASCII `"Trellis fixture payload #001"` + `0x00` padding | `../../_inputs/sample-payload-001.bin` |
| `ledger_scope` | bstr `"test-key-entry-tenant-root-ledger"` | §10.4 |
| `sequence` | `0` | §10.2 genesis |
| `authored_at` | `1745130400` | §12.1 narrative |
| `idempotency_key` | bstr `"idemp-append-033"` | §6.1 |
| `suite_id` | `1` (Phase-1 pin) | §7.1 |
| Non-signing class | `tenant-root` | ADR 0006 / Core §8.7.1 |
| Non-signing pubkey | `SHA-256("trellis-fixture-non-signing-pubkey-tenant-root")` (32 bytes) | fixture-only deterministic seed; the non-signing key does not sign anything in Phase 1 |

## Construction (single-event path; identical to `append/001`)

The event-construction byte path is byte-equivalent to `append/001`
modulo `ledger_scope` / `authored_at` / `idempotency_key`. The
load-bearing claim of this vector is in the **registry shape**, not in
the event itself. See `append/001/derivation.md` for the §6.8 → §9.5 →
§6.1 → §7.4 → §7.1 → §6.1 → §9.2 → §10.6 pipeline; the same procedure
applies here with the per-vector pinned values above.

## Registry shape (load-bearing for 033-key-entry-tenant-root-reservation)

`input-signing-key-registry.cbor` is a two-element dCBOR array:

**Row 0 (legacy `SigningKeyEntry`, signing class).** Eight-field map
header `0xa8`; canonical-ordered keys `attestation`, `kid`, `pubkey`,
`status`, `suite_id`, `supersedes`, `valid_from`, `valid_to`. Verifier
dispatch on absence of `kind` lands on §8.2; `issuer-001` is registered
as Active, valid_from = event timestamp, valid_to = null, supersedes =
null, attestation = null. The event's protected-header kid resolves
here via byte equality on `kid`.

**Row 1 (`KeyEntryNonSigning`, kind = "tenant-root").** Five-field map header
`0xa5`; canonical-ordered keys `attributes`, `extensions`, `kid`,
`kind`, `suite_id`. Verifier dispatch on presence of `kind = "tenant-root"`
lands on §8.7.3 step 3:
  - validates the `attributes` field is a map (the structural-shape
    gate; per-field validation is Phase-2+ per ADR 0006 §"Verifier
    obligations" step 4),
  - inserts the kid (derived from the attributes' `pubkey` per §8.3) into
    the parallel `NonSigningKeyEntry` map.

The byte-level claim: a verifier reading both rows produces the same
`integrity_verified = true` outcome it would on a single-row legacy
registry, plus a populated non-signing kid map that surfaces only in
class-confusion diagnostics (`tamper/023..024`).

## Verifier dispatch trace

1. `parse_key_registry` (Rust `trellis-verify::parse_key_registry`,
   Python `_parse_key_registry`) decodes the two-element array.
2. Row 0: no `kind` field → legacy path; pubkey, status, valid_to all
   parsed; entry inserted into the signing-key map keyed by kid.
3. Row 1: `kind = "tenant-root"` → reserved-non-signing path;
   `attributes` map presence is checked; kid (derived from the
   attributes' `pubkey` per §8.3) is inserted into the
   `NonSigningKeyEntry` map keyed by kid.
4. `verify_event_set` resolves the event's protected-header kid against
   the signing-key map; row 0's pubkey verifies the COSE_Sign1
   signature; lifecycle (Active) and content_hash all pass.
5. `integrity_verified = true`; the Phase-1 lint reports the non-signing
   kid declaration as a warning per Core §8.7.4 / TR-CORE-047.

## Invariant → byte mapping

| TR-CORE row | Where in 033-key-entry-tenant-root-reservation's bytes |
|---|---|
| TR-CORE-039 (KeyEntry taxonomy + dispatch) | The two-row `input-signing-key-registry.cbor` array exercises both dispatch paths in §8.7.3 step 1: row 0 has no `kind` (legacy); row 1 has `kind = "tenant-root"` (reserved non-signing). Both paths produce identical verification outcomes for the genesis event because the kid-resolution map is unioned across paths. |
| TR-CORE-047 (Phase-1 reservation acceptance + lint warning) | `expected-event.cbor` verifies under the two-row registry without `integrity_verified` flipping to false; the Phase-1 lint emits a warning naming `kind = "tenant-root"`. |

## Core-gap notes

The load-bearing Core claim — admitting a non-signing reservation in the
registry without rejecting verification — reproduces directly from
§8.7.3 step 3 and §8.7.4. No Core gap is claimed.

The fixture-only non-signing pubkey (deterministic SHA-256 of a per-class
marker string) is a fixture artifact only; production deployments would
substitute the real per-class material once Phase-2+ activates the
class. ADR 0006 §"Phase alignment" makes this transition wire-stable
(the slot is already in the wire; the runtime activates).
