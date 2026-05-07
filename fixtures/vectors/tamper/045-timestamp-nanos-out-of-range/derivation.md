# Derivation — `tamper/045-timestamp-nanos-out-of-range`

This vector pins the canonical negative for the ADR 0069 D-2.1 Trellis CBOR
timestamp shape after the stack blessed `[seconds, nanos]` instead of a
single nanosecond counter.

The generator starts from `append/001-minimal-inline-payload`, changes
`header.authored_at` from `[1745000000, 0]` to
`[1745000000, 1000000000]`, re-encodes the event payload as dCBOR, and
re-signs the COSE_Sign1 envelope under issuer-001. The event is therefore
well-formed COSE with a valid signature, but `map_lookup_timestamp()` cannot
construct a Trellis timestamp because Core §28 requires
`nanos_within_second <= 999999999`.

Verifier walk:

- Step 4.a — kid resolves in the signing-key registry. **PASS**.
- Step 4.b — COSE_Sign1 signature verifies. **PASS**.
- Step 4.c — payload bytes decode as CBOR. **PASS**.
- Step 4.c-header — `authored_at` is array-shaped, but the nanosecond
  component is `1000000000`. **FAIL** —
  `timestamp_nanos_out_of_range`.

Expected report:

- `structure_verified = false`
- `integrity_verified = false`
- `readability_verified = false`
- `tamper_kind = "timestamp_nanos_out_of_range"`
- `failing_event_id = "structure"`

Fixture distinction:

- `tamper/041` covers timestamp ordering after successful timestamp decode.
- `tamper/042` covers legacy bare-uint timestamp encoding.
- `tamper/045` covers malformed `[seconds, nanos]` array content.
