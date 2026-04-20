# Derivation — `export/001-two-event-chain`

## Header

**What this vector exercises.** This is the first `export/` vector. It is sized to be *optimal* rather than minimal: a two-event chain with two checkpoints, inclusion proofs for both leaves, and a single consistency proof from 1 → 2. The export is a deterministic ZIP per Core §18.1 and contains the full Phase-1 archive spine per §18.2, with a signed manifest binding digests per §18.3 / §19.

**Core § roadmap (in traversal order).**

1. Core §18.1 — deterministic ZIP requirements (ordering, timestamps, attributes, STORED).
2. Core §18.2–§18.6 — required archive members and their dCBOR shapes.
3. Core §18.3 + §7.4 — `000-manifest.cbor` COSE_Sign1 signing rules.
4. Core §19 steps 1–5 — digest bindings and per-event / checkpoint verification surfaces.
5. Core §9.2 + §11.3 — canonical event hashes, Merkle leaf/interior hashes, and checkpoint roots.
6. Core §14 — registry snapshot binding and verifier obligation to resolve against embedded bytes.

---

## Body

### Step 1: Select the canonical chain to export

**Core § citation:** §18.4 `010-events.cbor`; §19 step 4 (event loop).

**Operation.** The export’s `010-events.cbor` is a dCBOR array of `Event` COSE_Sign1 records in canonical order. This vector uses the two already-derived canonical events from:

- `append/001-minimal-inline-payload` (`sequence = 0`)
- `append/005-prior-head-chain` (`sequence = 1`, with `prev_hash` linking to sequence 0)

The bytes are concatenated under the CBOR array header `0x82`, preserving the byte-exact COSE_Sign1 envelopes.

**Result:** `010-events.cbor` (2-element array).

---

### Step 2: Compute canonical event hashes and Merkle leaf hashes

**Core § citation:** §9.2 (canonical event hash `trellis-event-v1`); §11.3 (Merkle leaf hash `trellis-merkle-leaf-v1`).

**Operation.**

1. Decode each COSE payload as `EventPayload`.
2. Compute `canonical_event_hash` for each payload under the §9.2 preimage.
3. Compute the Merkle leaf hash for each `canonical_event_hash`.

**Result:** leaf hashes for indices 0 and 1 are the inputs to the §11.3 Merkle root.

---

### Step 3: Compute the 2-leaf Merkle root (tree_size = 2)

**Core § citation:** §11.3 (Merkle interior hash `trellis-merkle-interior-v1` over `left_hash || right_hash`).

**Operation.** For a two-leaf tree, the `tree_head_hash` is:

`H_interior( leaf_hash_0 || leaf_hash_1 )`

where `H_interior` is the domain-separated interior-hash construction pinned by §11.3.

**Result:** the final checkpoint’s `tree_head_hash` is the 2-leaf root.

---

### Step 4: Build two checkpoints and link them

**Core § citation:** §11.2 (`CheckpointPayload`), §9.6 / §11.2 (checkpoint digest `trellis-checkpoint-v1`), §19 step 5.

**Operation.**

- Checkpoint #1 (`tree_size = 1`) commits `tree_head_hash = leaf_hash_0` and sets `prev_checkpoint_hash = null`.
- Checkpoint #2 (`tree_size = 2`) commits `tree_head_hash = root_2` and sets `prev_checkpoint_hash = checkpoint_digest(checkpoint_1_payload)`.

Both checkpoint payloads are serialized as dCBOR and signed as COSE_Sign1 per §7.4.

**Result:** `040-checkpoints.cbor` (2-element array of COSE_Sign1 checkpoints).

---

### Step 5: Build inclusion proofs and the 1 → 2 consistency proof

**Core § citation:** §18.5 (`InclusionProof` and `ConsistencyProof`).

**Operation.**

- `020-inclusion-proofs.cbor` maps `leaf_index → InclusionProof`.
  - For leaf 0, the audit path is `[leaf_hash_1]`.
  - For leaf 1, the audit path is `[leaf_hash_0]`.
- `025-consistency-proofs.cbor` contains one record linking `from_tree_size = 1` to `to_tree_size = 2` with `proof_path = [leaf_hash_1]` (RFC 6962 semantics for the two-leaf extension).

**Result:** inclusion and consistency proof files committed.

---

### Step 6: Embed signing-key registry snapshot

**Core § citation:** §8.2 (`SigningKeyEntry`), §8.3 (derived `kid`), §8.5 (self-contained verification).

**Operation.** `030-signing-key-registry.cbor` is a one-entry dCBOR array containing the issuer-001 Ed25519 public key and its derived `kid`, sufficient to resolve the `kid` referenced by the manifest and both checkpoints.

**Result:** `030-signing-key-registry.cbor` committed and digest-bound by the manifest.

---

### Step 7: Embed and bind the registry snapshot

**Core § citation:** §14.2 (bound registry), §14.3 (`RegistryBinding`), §19 step 3.f.

**Operation.**

1. Commit the domain registry bytes under `050-registries/<registry_digest>.cbor`.
2. Put a matching `RegistryBinding` entry in the manifest payload’s `registry_bindings`.

**Result:** the manifest binds the registry by SHA-256 digest; verifiers resolve meaning from the embedded bytes.

---

### Step 8: Build and sign the export manifest

**Core § citation:** §18.3 (`ExportManifestPayload`), §7.4 (COSE_Sign1 signing), §19 steps 2–3 (manifest verification + digest bindings).

**Operation.**

1. Compute SHA-256 digests of each required archive member.
2. Populate `ExportManifestPayload` with those digests, scope, tree_size, registry bindings, and `PostureDeclaration` (§20).
3. Serialize as dCBOR and sign as COSE_Sign1 to produce `000-manifest.cbor`.

**Result:** `000-manifest.cbor` committed.

---

### Step 9: Assemble the deterministic ZIP

**Core § citation:** §18.1 (deterministic ZIP).

**Operation.** Pack the archive members into a ZIP with:

- entries in lexicographic order by UTF-8 filename,
- compression method `STORED` for every entry,
- fixed modification time `1980-01-01T00:00:00Z` (DOS epoch minimum),
- extra fields empty and external attributes zero.

**Result:** `expected-export.zip` is the byte-level acceptance artifact for this vector.

---

## Footer — committed artifacts

- `expected-export.zip` — authoritative byte output for `export/` conformance.
- `000-manifest.cbor`, `010-events.cbor`, `020-inclusion-proofs.cbor`, `025-consistency-proofs.cbor`, `030-signing-key-registry.cbor`, `040-checkpoints.cbor`, `050-registries/...` — decomposed archive members used to assemble the ZIP and referenced by the manifest digests.
- `input-ledger-state.cbor` — non-normative runner convenience describing this vector’s build inputs.

