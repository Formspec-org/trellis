# Trellis Export (Fixture) — export/004-external-payload-optional-anchor

- scope (manifest.scope): `test-response-ledger`
- tree_size (manifest.tree_size): `1`
- tree_head_hash: `988b86f51b761fecc5248203a63541fe50907f0fc0e3345ef4429b22161f3d7a`
- head_checkpoint_digest: `85dba91dbe7d50cdde87b9cb77accab45dc4cfeae67cfe3719bba12b41b45e30`

## Posture Declaration (manifest.posture_declaration)
```json
{
  "delegated_compute": false,
  "external_anchor_name": "x-trellis-test/optional-anchor",
  "external_anchor_required": false,
  "metadata_leakage_summary": "PayloadExternal export fixture with optional anchor semantics.",
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
