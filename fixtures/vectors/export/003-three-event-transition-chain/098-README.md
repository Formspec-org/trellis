# Trellis Export (Fixture) — export/003-three-event-transition-chain

- scope (manifest.scope): `test-response-ledger`
- tree_size (manifest.tree_size): `3`
- tree_head_hash: `27e0ff0f0749934d52d07988a797424a9bef8eace9ea99292ba0717297c2b3bb`
- head_checkpoint_digest: `be906b349e3fcb1e9ae49289bfe01be0a62537a62ea0b467965f312c5b794d84`

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
