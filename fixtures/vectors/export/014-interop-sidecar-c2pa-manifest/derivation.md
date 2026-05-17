# `export/014-interop-sidecar-c2pa-manifest` derivation

Wave 25 positive — c2pa-manifest@v1 dispatched verifier.

Built by re-emitting `export/012-interop-sidecars-empty-list`'s
manifest with `interop_sidecars: [<one c2pa-manifest@v1 entry>]`,
adding the sidecar file at
`interop-sidecars/c2pa-manifest/cert-wave25-001.c2pa`, and re-
signing the manifest under `_keys/issuer-001.cose_key`.

The sidecar bytes are a synthetic five-field Trellis assertion
dCBOR map (mirrors `trellis-interop-c2pa::emit_c2pa_manifest_for_certificate`).
For path-(b) digest-only verification, the wrapping format is
irrelevant — the verifier only recomputes
`SHA-256(trellis-content-v1, file-bytes)` and compares to
`manifest.interop_sidecars[0].content_digest`.

Op is `export` per the directory naming rule (`check-specs.py:check_vector_manifest_identity` requires the
directory's first segment match `manifest.op`). The conformance
walker rebuilds the ZIP byte-for-byte from
`input-ledger-state.cbor` + per-member files; the result MUST
equal `expected-export.zip` (Core §18.1 deterministic ZIP).
Sidecar member at `interop-sidecars/c2pa-manifest/cert-wave25-001.c2pa`
is included in the member list.

`zip_sha256 = e6c3947f394dc135932cabeab12a4b2c2d890ba2cceee18d9ff2f95352db27e4`
