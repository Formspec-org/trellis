# Derivation — `verify/008-export-001-prev-checkpoint-hash-mismatch`

## Header

**What this vector exercises.** This negative-non-tamper verify vector makes
the head checkpoint's `prev_checkpoint_hash` disagree with the digest of the
prior checkpoint, while keeping the ZIP, manifest digest bindings, checkpoint
roots, and inclusion proofs self-consistent. Per Core §19 step 5.d, this is a
localizable checkpoint-chain failure that flips `integrity_verified = false`
via the step-9 conjunction.

**Core § roadmap.**

1. Core §19 steps 1–4 — ZIP layout, manifest signature, digest bindings, and
   per-event checks all succeed.
2. Core §19 step 5.c — checkpoint root recomputation still succeeds; the
   vector is not a root-mismatch case.
3. Core §19 step 5.d — the head checkpoint's `prev_checkpoint_hash` does not
   equal the digest of the prior checkpoint.
4. Core §19 step 9 — integrity conjunction fails because checkpoint-chain
   continuity is not valid.

---

## Body

### Step 1: Start from the happy-path export

Input ZIP is based on `export/001-two-event-chain/expected-export.zip`.

### Step 2: Rewrite the head checkpoint's `prev_checkpoint_hash` and re-sign

**Core § citation:** §11.2 (`CheckpointPayload.prev_checkpoint_hash`),
§11.3 checkpoint digest, §19 step 5.d.

**Operation.**

- Decode `040-checkpoints.cbor` and select the head checkpoint (the second
  checkpoint, `tree_size = 2`).
- Decode its COSE payload to `CheckpointPayload`.
- Flip a single bit in `prev_checkpoint_hash`, preserving 32-byte length. The
  reference generator XORs the least-significant bit of the final byte
  (`prev_checkpoint_hash[-1] ^= 0x01`); byte-exact reproduction requires the
  same LSB flip.
- Re-serialize the payload as dCBOR and re-sign the checkpoint as COSE_Sign1.

### Step 3: Update manifest digest bindings and re-sign

**Core § citation:** §19 step 3 digest bindings; §18.3 manifest.

**Operation.**

- Update `manifest.checkpoints_digest` and `manifest.head_checkpoint_digest` to
  match the mutated `040-checkpoints.cbor`.
- Re-sign the manifest so §19 step 2 continues to pass.

**Expected result.** The verifier reaches §19 step 5.d and records a
checkpoint-chain failure; `structure_verified = true` and
`integrity_verified = false`.
