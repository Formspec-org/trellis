# Derivation — `tamper/042-timestamp-legacy-uint-rejected`

Mutates `append/001`'s `authored_at` from the protobuf-pattern `[1745000000, 0]` to legacy bare `uint` `1745000000`, re-encodes the payload as dCBOR, and re-signs under issuer-001's real seed.

Signatures verify. The COSE envelope is well-formed. The tamper surfaces exclusively at the ADR 0069 D-2.1 timestamp-decode check: `map_lookup_timestamp()` encounters `Value::Integer` where `Value::Array` is required, triggering `legacy_timestamp_format` rejection.

§19 verifier walk:

- Step 4.a — kid resolves. PASS.
- Step 4.b — signature verifies (re-signed with real key). PASS.
- Step 4.c — payload decodes from COSE. PASS.
- Step 4.c-header — `map_lookup_timestamp(header, "authored_at")` sees bare `uint` instead of `[uint, uint]` array. **FAIL** — `legacy_timestamp_format`.
- Step 9 — `readability_verified` = false (structural decode failure); `integrity_verified` = false.

Distinguishes from prior tamper cases:

- tamper/041 = temporal order violation; timestamps decode correctly but decrease.
- tamper/042 = THIS vector; timestamps fail to decode at all (legacy bare uint rejected).
