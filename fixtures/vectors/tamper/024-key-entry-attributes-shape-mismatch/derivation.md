# tamper/024-key-entry-attributes-shape-mismatch — derivation

Spec-prose reproduction of the bytes committed under this directory. Authority
ladder: Rust > CDDL > prose; if the script and Core disagree, Core wins. The
generator is `fixtures/vectors/_generator/gen_tamper_023_to_025.py`.

## Pinned anchors

- **Core §8.7.1** — `KeyEntry` CDDL: `KeyEntryNonSigning` MUST carry an
  `attributes` map. Pairing a literal `kind` with the wrong inner-map
  shape (or with a missing `attributes` field) is a structure failure
  with code `key_entry_attributes_shape_mismatch`.
- **Core §8.7.3 step 3** — Verifier dispatch validates the entry shape:
  for reserved non-signing kinds, validate `attributes` against the
  matching CDDL group; structural mismatch → `key_entry_attributes_shape_mismatch`.
- **ADR 0006** — *Verifier obligations* step 3.
- **TR-CORE-048** — Matrix anchor: structural mismatch MUST fail with
  `key_entry_attributes_shape_mismatch`.

## Construction (step-by-step)

The tampered registry contains exactly two rows:

1. **Row 0** — Legacy `SigningKeyEntry` (no `kind` field) for
   `issuer-001`. Same shape as `append/001`'s registry entry; verifier
   dispatches via Core §8.7.3 step 1 onto the §8.2 path because no
   top-level `kind` is present.
2. **Row 1 (TAMPER)** — A `KeyEntryNonSigning` whose `kind = "subject"`
   and `kid = kid(subject) = 8ef652560863e5f9eb7e668c94e4428f` (same kid
   as `append/032`'s subject row, derived from
   `SHA-256("trellis-fixture-non-signing-pubkey-subject")` per `gen_append_032_to_035`).
   The `attributes` field is INTENTIONALLY OMITTED. Per Core §8.7.1 +
   §8.7.3 step 3 this is a structure failure.

The signed event in `010-events.cbor` is a clean `issuer-001`-signed
envelope (mirroring `append/032`'s genesis shape but with
`ledger_scope = "test-key-entry-024-ledger"` and
`idempotency_key = "tamper-key-entry-shape-024"`); its only role is to
provide a non-empty ledger so the verifier loads the registry. The
verifier's `parse_key_registry` returns
`Err(VerifyError::with_kind("key_entry_attributes_shape_mismatch"))` at the
row-1 dispatch arm before any signature verification runs.

The typed-kind tag is plumbed through `verify_tampered_ledger` to the
report's `event_failures[0].kind` field with location `"structure"` so
the conformance harness's `tamper_kind` assertion picks it up correctly.

## Expected report

| Field | Value | Source |
|---|---|---|
| `structure_verified` | `false` | Registry shape failure — Core §8.7.1 / §8.7.3 step 3. |
| `integrity_verified` | `false` | Cannot be true when structure_verified is false. |
| `readability_verified` | `false` | No payload decoded once registry parse fails. |
| `tamper_kind` | `"key_entry_attributes_shape_mismatch"` | Core §8.7.3 step 3 / TR-CORE-048. |
| `failing_event_id` | `"structure"` | Failure surfaces at registry-decode step before any per-event canonical_event_hash is computed. |

## Cross-vector relationships

- Shares `kid(issuer-001) = af9dff525391faa75c8e8da4808b1743` with `append/001`
  / `append/032..035` / `tamper/001` / `tamper/023` / `tamper/025`.
- Shares `kid(subject) = 8ef652560863e5f9eb7e668c94e4428f` with `append/032`
  and `tamper/025`.
- `tamper/023` and `tamper/025` test class-dispatch on a structurally
  VALID registry; `tamper/024` tests the structural-shape gate that runs
  BEFORE class dispatch.
