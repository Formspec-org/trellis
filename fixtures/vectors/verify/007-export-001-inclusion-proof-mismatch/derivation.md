# Derivation — `verify/007-export-001-inclusion-proof-mismatch`

## Header

**What this vector exercises.** This negative-non-tamper verify vector makes one inclusion proof fail while keeping the ZIP and manifest digest bindings self-consistent. Per Core §19 step 7, any inclusion-proof failure is recorded in `report.proof_failures` and flips `integrity_verified = false` via the step-9 conjunction.

**Core § roadmap.**

1. Core §19 steps 1–3 — ZIP layout + manifest signature + digest bindings all succeed.
2. Core §19 step 7 — recompute Merkle root from the inclusion proof and compare to head checkpoint root.
3. Core §19 step 9 — integrity conjunction fails because inclusion proofs are not all valid.

---

## Body

### Step 1: Start from the happy-path export

Input ZIP is based on `export/001-two-event-chain/expected-export.zip`.

### Step 2: Flip one bit in InclusionProof.leaf_hash for leaf_index 0

**Core § citation:** §18.5 (`InclusionProof`), §19 step 7.

**Operation.**

- Decode `020-inclusion-proofs.cbor`.
- Select the proof for leaf index 0.
- Flip a single bit in `leaf_hash`, preserving 32-byte length.
- Re-serialize `020-inclusion-proofs.cbor` as dCBOR.

### Step 3: Update manifest digest bindings and re-sign

**Core § citation:** §19 step 3 digest bindings; §18.3 manifest.

**Operation.**

- Update `manifest.inclusion_proofs_digest` to match the mutated `020-inclusion-proofs.cbor`.
- Re-sign the manifest so §19 step 2 continues to pass.

**Expected result.** The verifier reaches §19 step 7 and records a proof failure; `structure_verified = true` and `integrity_verified = false`.

