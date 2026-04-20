# Derivation — `verify/002-export-001-manifest-sigflip`

## Header

**What this vector exercises.** This is a negative-non-tamper verify vector: the export ZIP is structurally intact, but the manifest’s COSE signature is invalid. Per Core §19, an invalid manifest signature is a **fatal failure** and the verifier MUST abort with `structure_verified = false`.

**Core § roadmap.**

1. Core §19 step 1 — open deterministic ZIP (§18.1).
2. Core §19 step 2 — verify `000-manifest.cbor` COSE_Sign1.
3. Abort path: Core §19 step 2.c (`manifest signature invalid`).

---

## Body

### Step 1: Start from the happy-path export

Input ZIP is identical to `export/001-two-event-chain/expected-export.zip` except for `000-manifest.cbor`.

### Step 2: Flip one bit in the manifest signature

**Core § citation:** §7.4 COSE_Sign1 structure; §19 step 2.c manifest signature verification.

**Operation.** COSE_Sign1 is tag-18 4-array `[protected, unprotected, payload, signature]`. The signature is the final element, so flipping any single bit of the final byte of `000-manifest.cbor` mutates only the signature while preserving CBOR decodability.

**Byte-exact pin.** The reference generator XORs the least-significant bit of the final byte of `000-manifest.cbor` (`byte[-1] ^= 0x01`). Any verifier reproducing this vector MUST apply the same LSB flip so the resulting `input-export.zip` is byte-identical to the committed fixture (G-5 stranger test).

**Expected result.** A verifier reaches Core §19 step 2.c, signature verification fails, and the verifier aborts with `structure_verified = false` (and therefore `integrity_verified = false`, `readability_verified = false` by convention for aborted verification).

