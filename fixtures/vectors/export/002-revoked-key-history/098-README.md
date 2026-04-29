# Trellis Export (Fixture) — export/002-revoked-key-history

- scope (manifest.scope): `test-revocation-ledger`
- tree_size (manifest.tree_size): `1`
- tree_head_hash: `321f70c72f3cdccfc3c38c2819494dc0ebfd22491a77e04ba277d7914f822b81`
- head_checkpoint_digest: `f659113f7eac906bf4062aec2e795269746a25ddf4f60f9abda9c1f8e20a0fe2`

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
