# Derivation — `verify/026-signed-acts-manifest-extension-wrong-derivation-rule`

Starts from `export/006-signature-affirmations-inline`. Rewrites the
`trellis.export.signed-acts.manifest.v1.derivation_rule` field from
`"signed-acts-manifest-v1"` to `"signed-acts-manifest-unsupported"`, leaving
every other extension field and the 068 member bytes untouched. Re-signs
`000-manifest.cbor`.

Per Trellis Core §6.7 and the verifier admission gate at Rust
`trellis-verify-wos/src/signed_acts.rs:135-145`:

> if extension.derivation_rule != SIGNED_ACTS_MANIFEST_DERIVATION_RULE_V1 {
>     findings.push(finding("signed_acts_manifest_extension_invalid", …));
> }

Python mirror: `verify_wos._validate_signed_acts_manifest_extension`
(`verify_wos.py:813-822`). Both runtimes emit one finding of kind
`signed_acts_manifest_extension_invalid` and the relying-party verdict becomes
invalid via the `domain_admissibility` blocking-reason.

Distinct from `verify/020-export-006-signed-acts-unsupported-rule`, which
mutates the 066 catalog's `trellis.export.signed-acts.v1.derivation_rule`
(render projection) and emits `signed_acts_catalog_invalid` under
`projection_integrity`. This fixture exercises the 068 manifest (substrate)
admission gate, routing to `domain_admissibility`.
