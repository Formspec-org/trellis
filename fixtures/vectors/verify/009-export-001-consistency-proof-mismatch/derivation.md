# Derivation — `verify/009-export-001-consistency-proof-mismatch`

## Header

**What this vector exercises.** This negative-non-tamper verify vector makes
the sole consistency proof disagree with the actual append-only relation
between the two published checkpoints, while keeping the ZIP, manifest digest
bindings, checkpoint roots, and inclusion proofs self-consistent. Per Core
§19 step 5.e, this is a localizable integrity failure that flips
`integrity_verified = false` via the step-9 conjunction.

**Core § roadmap.**

1. Core §19 steps 1–4 — ZIP layout, manifest signature, digest bindings, and
   per-event checks all succeed.
2. Core §19 step 5.c — checkpoint roots still recompute correctly.
3. Core §19 step 5.d — `prev_checkpoint_hash` continuity still holds.
4. Core §19 step 5.e — the consistency proof from the first checkpoint to the
   head checkpoint no longer reconstructs the later tree head.
5. Core §19 step 9 — integrity conjunction fails because consistency proofs are
   not all valid.

---

## Body

### Step 1: Start from the happy-path export

Input ZIP is based on `export/001-two-event-chain/expected-export.zip`.

### Step 2: Flip one bit in the consistency proof path

**Core § citation:** §11.4 consistency proofs; §19 step 5.e.

**Operation.**

- Decode `025-consistency-proofs.cbor`.
- Select the only consistency proof record (`from_tree_size = 1`,
  `to_tree_size = 2`).
- Flip a single bit in the only `proof_path` node, preserving 32-byte length.
  The reference generator XORs the least-significant bit of the final byte
  (`proof_path[0][-1] ^= 0x01`); byte-exact reproduction requires the same LSB
  flip.
- Re-serialize `025-consistency-proofs.cbor` as dCBOR.

### Step 3: Update manifest digest bindings and re-sign

**Core § citation:** §19 step 3 digest bindings; §18.3 manifest.

**Operation.**

- Update `manifest.consistency_proofs_digest` to match the mutated
  `025-consistency-proofs.cbor`.
- Re-sign the manifest so §19 step 2 continues to pass.

**Expected result.** The verifier reaches §19 step 5.e and records a
consistency-proof failure; `structure_verified = true` and
`integrity_verified = false`.
