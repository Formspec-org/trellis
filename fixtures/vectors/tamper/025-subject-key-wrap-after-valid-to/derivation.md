# tamper/025-subject-key-wrap-after-valid-to — derivation

Spec-prose reproduction of the bytes committed under this directory. Authority
ladder: Rust > CDDL > prose; if the script and Core disagree, Core wins. The
generator is `fixtures/vectors/_generator/gen_tamper_023_to_025.py`.

## Pinned anchors

- **Core §8.7.3 step 4** bullet 3 — `subject`: `effective_from ≤ authored_at
  ≤ valid_to` (when non-null) for `KeyBagEntry` wraps referencing this
  subject-kid; NO wraps MAY reference this kid after `valid_to`. Phase-1
  verifiers MAY skip with a warning.
- **ADR 0006 *Phase-1 runtime discipline*** — `KeyBagEntry.ephemeral_pubkey`
  + recipient-pubkey path (Core §9.4) continues to use opaque bytes for
  the recipient; Phase-2+ lifts the Wrap-entry recipient reference to a
  registered `subject` kid.
- **ADR 0006 fixture plan** — `tamper/025-subject-key-wrap-after-valid-to`:
  detectable when the subject-kind classes activate.
- **Core §8.7.3 step 4** bullet 4 — A `subject` kid in a COSE_Sign1
  protected header for an ordinary event is a class-confusion attack;
  verifier MUST reject with `key_class_mismatch` (same dispatch as
  recovery-class).
- **TR-CORE-048** — Matrix anchor.

## Phase-1 detection vs Phase-2+ activation

Phase-1 `KeyBagEntry.recipient` is opaque bytes (Core §9.4); the verifier
cannot bind a `KeyBagEntry` to a registered `subject` kid until Phase-2+
activates the recipient-as-kid path per ADR 0006. The literal "wrap
references a `subject` kid whose `valid_to` has passed" check is
therefore Phase-2+ runtime work.

This vector lands the **wire bytes** today so the corpus carries a fixture
that exercises both:

1. **Today (Phase-1)**: Signing under a `subject`-class kid is a class
   violation per Core §8.7.3 step 4; verifier emits `key_class_mismatch`.
   This is the assertion the conformance harness checks today.
2. **Tomorrow (Phase-2+)**: When the recipient-as-kid path activates and
   `KeyBagEntry.recipient` resolves to a registered `subject` kid, the
   verifier additionally checks `details.authored_at ≤ subject.valid_to`
   for every wrap referencing that kid. The Rust verifier already
   captures `subject.valid_to` into
   `NonSigningKeyEntry.subject_valid_to` (see
   `crates/trellis-verify/src/lib.rs` `parse_key_registry` subject arm)
   so this future check runs without re-decoding the registry.

The fixture's wire shape carries the forward-compatible signal: the
`subject` row has `valid_to = 1745130100`; the event's `authored_at =
1745131200` is strictly greater. When Phase-2+ runtime activates, this
ordering will fire `subject_wrap_after_valid_to` (or whatever the
final code is), and a vector update at that time may upgrade the
expected `tamper_kind` accordingly.

## Construction (step-by-step)

Same as `tamper/023` but with `recovery` swapped for `subject` and the
subject row carries an explicit `valid_to`:

1. **Authored event** — Core §6.1 12-field map; `ledger_scope = "test-key-
   entry-025-ledger"`; `sequence = 0`; `prev_hash = null`; payload
   inline; `authored_at = 1745131200`. dCBOR-canonical map ordering per
   Core §5.1.
2. **`author_event_hash`** — Core §9.5 over the authored bytes under
   domain tag `trellis-author-event-v1`.
3. **EventPayload** — Core §6.1 13-field canonical map.
4. **Protected header (TAMPER)** — Core §7.4 dCBOR map with
   `kid = kid(subject) = 8ef652560863e5f9eb7e668c94e4428f`.
5. **`Sig_structure`** — RFC 9052 §4.4 over dCBOR-encoded protected header.
6. **Signature** — `Ed25519PrivateKey(seed_001).sign(sig_structure)`.
   The signature would verify against `pubkey_001`, but the verifier's
   class-dispatch rejects before signature verification.
7. **COSE_Sign1 envelope** — CBOR tag 18 wrapping `[protected, {},
   payload, signature]`; dCBOR-encoded.
8. **Single-event ledger** — `dcbor([cose_sign1])`.
9. **Registry** — `dcbor([signing_row, subject_row])`. `signing_row` is
   the legacy 8-field `SigningKeyEntry` for `issuer-001` per Core §8.2.
   `subject_row` is a `KeyEntryNonSigning` per §8.7.1 with `kind =
   "subject"`, `kid = kid(subject)`, `suite_id = 1`,
   `attributes = {pubkey, subject_ref: "urn:agency.gov:subject:fixture-025",
   authorized_for: [b"x-trellis-test/wrap-cap-1"], effective_from:
   1745130000, valid_to: 1745130100, supersedes: null}` per
   `SubjectKeyAttributes` in §8.7.2, `extensions = null`.

## Expected report (Phase-1)

| Field | Value | Source |
|---|---|---|
| `structure_verified` | `false` | Class-dispatch fatal — Core §8.7.3 step 4 bullet 4 (subject kid signing ordinary event). |
| `integrity_verified` | `false` | Cannot be true when structure_verified is false. |
| `readability_verified` | `false` | No payload decoded once dispatch rejects. |
| `tamper_kind` | `"key_class_mismatch"` | Core §8.7.3 step 4 / TR-CORE-048. |
| `failing_event_id` | `"structure"` | Failure surfaces at registry-dispatch step before any per-event canonical_event_hash is computed. |

## Cross-vector relationships

- Shares `kid(issuer-001) = af9dff525391faa75c8e8da4808b1743` with `append/001`
  / `append/032..035` / `tamper/001` / `tamper/023` / `tamper/024`.
- Shares `kid(subject) = 8ef652560863e5f9eb7e668c94e4428f` with `append/032`
  and `tamper/024`. (`append/032` and `tamper/025` use the same subject
  pubkey but different `valid_to` — `null` in the positive, `1745130100`
  in this tamper.)
- `tamper/023` covers `recovery`-class kid signing; this vector covers
  `subject`-class kid signing — same dispatch path, different reserved class.
