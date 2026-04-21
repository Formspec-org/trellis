# Trellis Export (Fixture) — export/003-three-event-transition-chain

- scope (manifest.scope): `test-response-ledger`
- tree_size (manifest.tree_size): `3`
- tree_head_hash: `3263ce16ed5de941732c1402cf80c8e2b54f0e020b169dea96426027fafd6e9a`
- head_checkpoint_digest: `c41d8c2bae82909858564b44f1819c5699db611a27a5883e83ab2c983a074621`

## Posture Declaration (manifest.posture_declaration)
```json
{
  "delegated_compute": false,
  "external_anchor_name": null,
  "external_anchor_required": false,
  "metadata_leakage_summary": "Transition-chain export fixture.",
  "provider_readable": true,
  "reader_held": false,
  "recovery_without_user": true
}
```

## Omitted payload checks
```json
[]
```

Run `./090-verify.sh` from this directory.
