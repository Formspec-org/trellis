# Trellis Export (Fixture) — export/004-external-payload-optional-anchor

- scope (manifest.scope): `test-response-ledger`
- tree_size (manifest.tree_size): `1`
- tree_head_hash: `8337cc091966bdb7bf27375de35330bcad500c125017364793b0ab85f648af9a`
- head_checkpoint_digest: `3378c74bdbbabf41ff5aeb43b56524528dc12342ef0bc63aeff375084a731475`

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
