# Derivation - `projection/003-cadence-positive-height`

## Header

**What this vector exercises.** This vector is the positive half of O-3 Test 3: snapshot cadence. It exercises TR-OP-008 and Companion §16.2 OC-46 by declaring a height-based cadence of every two events and committing checkpoints at every required height: 2, 4, and 6.

**Scope of this vector.** Cadence only. It does not test watermark fields, rebuild equivalence, stale-status behavior, or purge-cascade semantics. Those are covered by neighboring projection/shred fixtures.

**Runner contract.** A conforming runner loads `input-chain.cbor`, reads the declared `[cadence]` table, enumerates the checkpoint files listed in `[inputs].checkpoints`, and verifies that each required height has a checkpoint whose payload binds the same `tree_size` and the Merkle `tree_head_hash` for the chain prefix at that height.

## Body

### Step 1: Canonical chain

The generator builds six structural-only Trellis events using the same EventPayload and COSE_Sign1 shape as `projection/001-watermark-attestation` and `projection/002-rebuild-equivalence-minimal`.

Each event uses:

- `ledger_scope = b"test-cadence-ledger"`
- `event_type = b"x-trellis-test/cadence-event"` under the Core §14.6 reserved test prefix
- `PayloadInline` with 32 bytes of opaque ciphertext
- empty `key_bag.entries`
- `prev_hash` equal to the previous event's canonical event hash, except event 0 where `prev_hash = null`

The six signed events are written as a definite-length dCBOR array in `input-chain.cbor`.

### Step 2: Height cadence declaration

The fixture declares:

- `cadence.kind = "height-based"`
- `cadence.interval = 2`
- `cadence.required_tree_sizes = [2, 4, 6]`

This is a concrete instance of Companion §16.2 OC-46, which requires each deployment to declare a snapshot cadence and makes absent snapshots at required cadence points a conformance violation.

### Step 3: Checkpoints at every required height

For each required height, the generator computes the Merkle root over the first `tree_size` canonical event hashes and signs a Core §11.2 `CheckpointPayload`:

- `input-checkpoint-002.cbor` binds `tree_size = 2`
- `input-checkpoint-004.cbor` binds `tree_size = 4`
- `input-checkpoint-006.cbor` binds `tree_size = 6`

Each checkpoint is a COSE_Sign1 envelope over the checkpoint payload. The height-4 checkpoint links to the height-2 checkpoint via `prev_checkpoint_hash`; the height-6 checkpoint links to height 4.

### Step 4: Expected cadence report

`expected-cadence-report.cbor` is dCBOR over:

```cddl
CadenceReport = {
  cadence_kind:        "height-based",
  interval:            2,
  expected_tree_sizes: [2, 4, 6],
  observed_tree_sizes: [2, 4, 6],
  missing_tree_sizes:  [],
  cadence_satisfied:   true,
  failure_code:        null,
}
```

The runner passes this fixture when the observed checkpoint heights exactly satisfy the declared cadence.
