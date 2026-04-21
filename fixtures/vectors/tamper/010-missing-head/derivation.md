# Derivation — `tamper/010-missing-head`

Checkpoint-aware export tamper. Starts from `export/001-two-event-chain`, removes the head checkpoint from `040-checkpoints.cbor`, updates the manifest's `checkpoints_digest`, and re-signs the manifest while leaving `head_checkpoint_digest` pinned to the missing head. The verifier therefore reaches `head_checkpoint_digest_mismatch` as the first integrity failure.
