# Derivation — `verify/025-signed-acts-manifest-extension-wrong-catalog-ref`

Starts from `export/006-signature-affirmations-inline`. Rewrites the
`trellis.export.signed-acts.manifest.v1.catalog_ref` field from
`"068-signed-acts-manifest.cbor"` to `"wrong-member.cbor"`, leaving every
other extension field (`manifest_digest`, `derivation_rule`) and the 068
member bytes untouched. Re-signs `000-manifest.cbor`.

Per Trellis Core §6.7 and the verifier admission gate at Rust
`trellis-verify-wos/src/signed_acts.rs:125-133`:

> if extension.catalog_ref != SIGNED_ACTS_MANIFEST_MEMBER {
>     findings.push(finding("signed_acts_manifest_extension_invalid", …));
> }

Python mirror: `verify_wos._validate_signed_acts_manifest_extension`
(`verify_wos.py:804-812`). Both runtimes emit one finding of kind
`signed_acts_manifest_extension_invalid` and the relying-party verdict becomes
invalid via the `domain_admissibility` blocking-reason.
