# Derivation — `verify/006-export-001-checkpoint-root-mismatch`

## Header

**What this vector exercises.** This negative-non-tamper verify vector makes the head checkpoint’s `tree_head_hash` disagree with the Merkle root recomputed from `010-events.cbor`, while keeping the ZIP and manifest digest bindings self-consistent. Per Core §19 step 5.c, this is a checkpoint verification failure recorded in `report.checkpoint_failures`, which flips `integrity_verified = false` via the step-9 conjunction.

Secondary surface. Because every inclusion proof is root-checked against `head.tree_head_hash` (now mutated), Core §19 step 7.b (root recomputation for each inclusion proof) also fails and records entries in `report.proof_failures`. Both 5.c and 7.b are localizable failures — the verifier continues, and the declared booleans in `manifest.toml` stay correct (`structure_verified = true`, `integrity_verified = false`, `readability_verified = true`).

**Core § roadmap.**

1. Core §19 steps 1–3 — ZIP layout + manifest signature + digest bindings all succeed.
2. Core §19 step 5.c — recompute Merkle root and compare to `CheckpointPayload.tree_head_hash`.
3. Core §19 step 7.b — inclusion-proof roots also disagree with the mutated `head.tree_head_hash`; entries accumulate in `report.proof_failures`.
4. Core §19 step 9 — integrity conjunction fails because checkpoint roots and/or inclusion proofs are not all valid.

---

## Body

### Step 1: Start from the happy-path export

Input ZIP is based on `export/001-two-event-chain/expected-export.zip`.

### Step 2: Rewrite the head checkpoint’s `tree_head_hash` and re-sign

**Core § citation:** §11.2 (`CheckpointPayload.tree_head_hash`), §11.3 Merkle root, §19 step 5.c.

**Operation.**

- Decode `040-checkpoints.cbor` and select the head checkpoint (the second checkpoint, `tree_size = 2`).
- Decode its COSE payload to `CheckpointPayload`.
- Flip a single bit in `tree_head_hash`, preserving 32-byte length. The reference generator XORs the least-significant bit of the final byte (`tree_head_hash[-1] ^= 0x01`); byte-exact reproduction requires the same LSB flip.
- Re-serialize the payload as dCBOR and re-sign the checkpoint as COSE_Sign1.

### Step 3: Update manifest digest bindings and re-sign

**Core § citation:** §19 step 3 digest bindings; §18.3 manifest.

**Operation.**

- Update `manifest.checkpoints_digest` and `manifest.head_checkpoint_digest` to match the mutated `040-checkpoints.cbor`.
- Re-sign the manifest so §19 step 2 continues to pass.

**Expected result.** The verifier reaches §19 step 5.c and records a checkpoint failure; `structure_verified = true` and `integrity_verified = false`.

