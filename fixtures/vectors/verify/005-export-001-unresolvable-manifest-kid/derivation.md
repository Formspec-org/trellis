# Derivation — `verify/005-export-001-unresolvable-manifest-kid`

## Header

**What this vector exercises.** This negative-non-tamper verify vector makes the manifest’s `kid` unresolvable against the embedded signing-key registry. Per Core §19 step 2.a, an unresolvable manifest `kid` is a fatal failure (abort with `structure_verified = false`).

**Core § roadmap.**

1. Core §19 step 1 — open deterministic ZIP (§18.1).
2. Core §19 step 2.a — resolve manifest protected-header `kid` via embedded `030-signing-key-registry.cbor`.
3. Abort path: Core §19 step 2.a (`unresolvable_manifest_kid`).

---

## Body

### Step 1: Start from the happy-path export

Input ZIP is identical to `export/001-two-event-chain/expected-export.zip` except for `000-manifest.cbor`.

### Step 2: Rewrite the manifest protected header `kid` to a non-existent value

**Core § citation:** §8.5 registry must resolve every `kid`; §19 step 2.a kid resolution.

**Operation.**

- Decode `000-manifest.cbor` as a COSE_Sign1 tag-18 4-array.
- Decode the protected header bstr to a CBOR map.
- Set `kid` (`4`) to a 16-byte value not present in the embedded registry (this vector uses `0x00` repeated 16 times).
- Re-encode the protected header per dCBOR and re-sign the manifest.

The COSE is structurally valid, but the verifier must fail at kid resolution because the registry does not contain the `kid`.

**Expected result.** Verification aborts at Core §19 step 2.a with `structure_verified = false`.

