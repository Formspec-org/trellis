# Derivation — `verify/004-export-001-unsupported-suite`

## Header

**What this vector exercises.** This negative-non-tamper verify vector makes the manifest’s `suite_id` unregistered. Per Core §19 step 2.b, a verifier must reject an unregistered or inconsistent `suite_id` as a fatal failure (abort with `structure_verified = false`).

**Core § roadmap.**

1. Core §19 step 1 — open deterministic ZIP (§18.1).
2. Core §19 step 2 — verify `000-manifest.cbor` COSE protected header and signature:
   - step 2.a resolves `kid`
   - **step 2.b checks `alg` and `suite_id` are registered and consistent**
3. Abort path: Core §19 step 2.b.

---

## Body

### Step 1: Start from the happy-path export

Input ZIP is identical to `export/001-two-event-chain/expected-export.zip` except for `000-manifest.cbor`.

### Step 2: Rewrite the manifest protected header `suite_id` and re-sign

**Core § citation:** §7.4 protected header is signed; §19 step 2.b suite-id registration check.

**Operation.**

- Decode `000-manifest.cbor` as a COSE_Sign1 tag-18 4-array.
- Decode the protected header bstr to a CBOR map.
- Set `suite_id` (`-65537`) to an unregistered value (this vector uses `999`).
- Re-encode the protected header per dCBOR and re-sign the manifest under Ed25519.

This ensures the manifest remains CBOR-decodable and structurally a COSE_Sign1, but a conforming verifier must reject it at the suite-registration check.

**Expected result.** Verification aborts at Core §19 step 2.b with `structure_verified = false`.

