# Derivation — `verify/024-signed-acts-manifest-extension-parse-failure`

Starts from `export/006-signature-affirmations-inline`. Replaces the value of
`trellis.export.signed-acts.manifest.v1` in the manifest's `extensions` map
with a CBOR text string (`"not-a-map"`) instead of the expected map of
`catalog_ref` / `manifest_digest` / `derivation_rule`. Re-signs
`000-manifest.cbor`; the 068 member bytes are left untouched.

The verifier's extension-parse step fails at
`value.as_map().ok_or_else(|| "signed acts manifest extension is not a map")` —
Rust `parse_signed_acts_manifest_extension` (`signed_acts.rs:200`), surfaced via
`validate_bound_signed_acts_manifest_extension` (`signed_acts.rs:114-122`).
Python mirror: `_parse_signed_acts_manifest_export_extension`
(`trellis-py/src/trellis_py/verify_wos.py:487-488`). Both runtimes emit one
finding of kind `signed_acts_manifest_extension_invalid` (Severity::Failure)
and the relying-party verdict becomes invalid via the `domain_admissibility`
blocking-reason (these kinds are NOT `is_projection_finding`).
