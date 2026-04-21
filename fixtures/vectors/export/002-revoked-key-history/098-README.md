# Trellis Export (Fixture) — export/002-revoked-key-history

- scope (manifest.scope): `test-revocation-ledger`
- tree_size (manifest.tree_size): `1`
- tree_head_hash: `b3429cc786c88170ecabd6a56dda555c424ad2f9f69abaa6237f03ae66a7fa3a`
- head_checkpoint_digest: `9c9e0c0723c30efda5d5bd7096423392ced2e84525c76aeed74f50a2856c346b`

## Posture Declaration (manifest.posture_declaration)
```json
{
  "delegated_compute": false,
  "external_anchor_name": null,
  "external_anchor_required": false,
  "metadata_leakage_summary": "Historical revoked-key export fixture.",
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
