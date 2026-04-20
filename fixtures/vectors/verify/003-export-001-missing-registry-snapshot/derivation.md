# Derivation — `verify/003-export-001-missing-registry-snapshot`

## Header

**What this vector exercises.** This is a negative-non-tamper verify vector: the export ZIP’s manifest binds a registry snapshot by digest, but the ZIP omits the corresponding `050-registries/<digest>.cbor` member. Per Core §19 step 3.f, this is a digest-binding failure and is treated as a fatal archive-integrity failure.

**Core § roadmap.**

1. Core §19 step 1 — open deterministic ZIP (§18.1).
2. Core §19 step 2 — verify `000-manifest.cbor` COSE_Sign1.
3. Core §19 step 3.f — verify each bound registry snapshot digest.
4. Abort path: Core §19 step 3 (`archive integrity failure`).

---

## Body

### Step 1: Start from the happy-path export

Input ZIP is identical to `export/001-two-event-chain/expected-export.zip` except that it omits the single `050-registries/<digest>.cbor` file.

### Step 2: Omit the registry member while keeping the binding

**Core § citation:** §14.2 bound registry; §19 step 3.f digest binding.

**Operation.** Remove the registry file from the ZIP, but do not change the manifest. The manifest still includes a `RegistryBinding` and still binds the digest of the missing file.

**Expected result.** A verifier attempting Core §19 step 3.f cannot compute SHA-256 over the missing registry member, so the digest-binding requirement cannot be satisfied. The verifier aborts with `structure_verified = false` (and therefore `integrity_verified = false`, `readability_verified = false` by convention for aborted verification).

