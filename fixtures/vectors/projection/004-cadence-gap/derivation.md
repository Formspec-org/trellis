# Derivation - `projection/004-cadence-gap`

## Header

**What this vector exercises.** This vector is the negative half of O-3 Test 3: snapshot cadence. It exercises TR-OP-008 and Companion §16.2 OC-46 by declaring a height-based cadence of every two events while deliberately omitting the required checkpoint at height 4.

**Scope of this vector.** Cadence-gap detection only. The fixture does not assert a universal Trellis cadence; it asserts that a runner can compare observed checkpoints against the cadence declared by this vector.

**Runner contract.** A conforming runner loads the same six-event chain shape used by `projection/003-cadence-positive-height`, observes checkpoints at heights 2 and 6, computes that height 4 is required by the declared cadence, and reports `failure_code = "missing-required-checkpoint"`.

## Body

### Step 1: Canonical chain

The generator builds six structural-only Trellis events under `ledger_scope = b"test-cadence-ledger"`. Event shape matches `projection/003-cadence-positive-height`: COSE_Sign1 envelopes over EventPayload records, `PayloadInline` with opaque ciphertext, and `prev_hash` chained by canonical event hash.

The chain is committed as `input-chain.cbor`.

### Step 2: Declared cadence and observed checkpoints

The fixture declares:

- `cadence.kind = "height-based"`
- `cadence.interval = 2`
- `cadence.required_tree_sizes = [2, 4, 6]`
- `cadence.observed_tree_sizes = [2, 6]`
- `cadence.missing_tree_sizes = [4]`

Only two checkpoint files are committed:

- `input-checkpoint-002.cbor`
- `input-checkpoint-006.cbor`

The absence of `input-checkpoint-004.cbor` is intentional and is the behavior under test.

### Step 3: Expected failure report

`expected-cadence-report.cbor` is dCBOR over:

```cddl
CadenceReport = {
  cadence_kind:        "height-based",
  interval:            2,
  expected_tree_sizes: [2, 4, 6],
  observed_tree_sizes: [2, 6],
  missing_tree_sizes:  [4],
  cadence_satisfied:   false,
  failure_code:        "missing-required-checkpoint",
}
```

The runner passes this fixture only when it detects the gap. Treating the fixture as successful cadence compliance is incorrect; the expected success condition is correct failure detection.
