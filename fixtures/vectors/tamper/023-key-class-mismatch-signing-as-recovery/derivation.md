# tamper/023-key-class-mismatch-signing-as-recovery — derivation

Spec-prose reproduction of the bytes committed under this directory. Authority
ladder: Rust > CDDL > prose; if the script and Core disagree, Core wins. The
generator is `fixtures/vectors/_generator/gen_tamper_023_to_025.py`.

## Pinned anchors

- **Core §8.7** — Unified `KeyEntry` taxonomy. Five reserved classes; verifier
  dispatch on the top-level `kind` field; class-specific invariants in
  §8.7.3 step 4.
- **Core §8.7.3 step 4** — Recovery-class kids are NOT acceptable as a
  signing kid for any non-recovery event. A `recovery`-class kid in a
  COSE_Sign1 protected header for an ordinary event is a class-confusion
  attack; verifier MUST reject with `key_class_mismatch`.
- **ADR 0006** — *Adversary model* bullet 1: forged signature claim under a
  recovery-only kid fails class-dispatch validation.
- **TR-CORE-048** — Matrix anchor: a `KeyEntry` whose `kind` is one of the
  five reserved literals MUST decode against the matching CDDL group; a
  `recovery`-class kid in a COSE_Sign1 protected header for an ordinary
  event MUST fail with `key_class_mismatch`.

## Construction (step-by-step)

The vector reuses `append/035-key-entry-recovery-reservation`'s registry
shape — a two-entry array carrying `issuer-001` (legacy `SigningKeyEntry`
shape) and a `recovery`-class `KeyEntryNonSigning` row whose
`authorizes_recovery_for` references `kid(issuer-001)`. The non-signing
pubkey for the recovery class is the deterministic
`SHA-256("trellis-fixture-non-signing-pubkey-recovery")` 32-byte value
shared with `gen_append_032_to_035.py`, so `kid(recovery)` is byte-equal
across `append/035` and `tamper/023`.

The ledger carries one event in `test-key-entry-recovery-ledger`:

1. **Authored event** — Core §6.1 12-field map; `ledger_scope = "test-key-
   entry-recovery-ledger"`; `sequence = 0`; `prev_hash = null`; payload
   inline (mirrors `append/032`'s shape exactly except for the ledger scope
   and idempotency_key); `authored_at = 1745130800`. dCBOR-canonical map
   ordering per Core §5.1.
2. **`author_event_hash`** — Core §9.5 over the authored bytes under
   domain tag `trellis-author-event-v1`.
3. **EventPayload** — Core §6.1 13-field canonical map; same fields as
   authored plus `author_event_hash` appended last.
4. **Protected header (TAMPER)** — Core §7.4 dCBOR map `{1: -8, 4: kid,
   -65537: 1}` with `kid = kid(recovery) = 35a6bf49bc0f202ab88841503a831e04`.
   This is the only delta vs an `append/032`-shape positive: the kid
   points at the recovery-class kid, not at `kid(issuer-001)`.
5. **`Sig_structure`** — RFC 9052 §4.4 `["Signature1", protected, "",
   payload]` over the dCBOR-encoded protected header from step 4.
6. **Signature** — `Ed25519PrivateKey(seed_001).sign(sig_structure)`.
   Note: this signature would VERIFY against `pubkey_001`, but the
   verifier's class-dispatch obligation rejects before reaching
   signature verification — the kid resolves to a non-signing class.
7. **COSE_Sign1 envelope** — CBOR tag 18 wrapping `[protected, {},
   payload, signature]`; dCBOR-encoded.
8. **Single-event ledger** — `dcbor([cose_sign1])` per Core §18.4
   (`010-events.cbor` shape).
9. **Registry** — `dcbor([signing_row, recovery_row])`. `signing_row` is
   the legacy 8-field `SigningKeyEntry` (no `kind`) for `issuer-001` per
   Core §8.2. `recovery_row` is a `KeyEntryNonSigning` per §8.7.1 with
   `kind = "recovery"`, `kid = kid(recovery)`, `suite_id = 1`,
   `attributes = {pubkey, authorizes_recovery_for: [kid_001],
   activation_quorum: 1, activation_quorum_set: null,
   effective_from: 1745130000, supersedes: null}` per `RecoveryKeyAttributes`
   in §8.7.2, `extensions = null`.

## Expected report

| Field | Value | Source |
|---|---|---|
| `structure_verified` | `false` | Class-dispatch fatal — Core §8.7.3 step 4. |
| `integrity_verified` | `false` | Cannot be true when structure_verified is false. |
| `readability_verified` | `false` | No payload decoded once dispatch rejects. |
| `tamper_kind` | `"key_class_mismatch"` | Core §8.7.3 step 4 / TR-CORE-048. |
| `failing_event_id` | `"structure"` | Failure surfaces at registry-dispatch step before any per-event canonical_event_hash is computed. |

## Cross-vector relationships

- Shares `kid(issuer-001) = af9dff525391faa75c8e8da4808b1743` with `append/001`
  / `append/032..035` / `tamper/001`.
- Shares `kid(recovery) = 35a6bf49bc0f202ab88841503a831e04` with `append/035`.
- The signed event's `canonical_event_hash` is unique (different
  `idempotency_key` and `ledger_scope` than any other vector); committed for
  audit reference: `eb093ba4cc9ce2d2a8020d9333f5f451ef73e617d0e146f8159bde9e77f1ddc0`.
