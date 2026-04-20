# Derivation — `verify/001-export-001-two-event-chain`

## Header

**What this vector exercises.** This is the first `verify/` success vector. It verifies a complete Phase-1 export ZIP (produced by `export/001-two-event-chain`) under Core §19. The acceptance check is the report’s three booleans (`structure_verified`, `integrity_verified`, `readability_verified`) all `true`.

**Core § roadmap.**

1. Core §19 step 1 — open the deterministic ZIP per §18.1.
2. Core §19 step 2 — verify `000-manifest.cbor` COSE_Sign1 (kid resolution + signature).
3. Core §19 step 3 — verify all manifest-bound digests (events, checkpoints, proofs, signing-key-registry, registries).
4. Core §19 step 4 — verify each Event: signature, hashes, scope, payload integrity, chain link, registry resolution.
5. Core §19 step 5 — verify each Checkpoint: signature, Merkle root, prev_checkpoint_hash chain, and consistency proof.

---

## Body

### Step 1: Input artifact

`input-export.zip` is a byte-identical copy of `export/001-two-event-chain/expected-export.zip`. The verifier consumes the ZIP as the `E` input to Core §19.

---

### Step 2: Expected report booleans

Under Core §19 step 9:

- `structure_verified = true` because every required structure decodes and the manifest signature and digest bindings validate.
- `integrity_verified = true` because every per-event signature/hash/chain check, checkpoint root/chain check, and proof check validates, and there are no omitted payload checks.
- `readability_verified = true` because no payload required by the export scope is missing and no readability checks fail for this vector’s scope.

These booleans are the only fields pinned in `[expected.report]` for this initial happy-path verify vector.

