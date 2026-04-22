# Derivation — `tamper/013-attachment-manifest-digest-mismatch`

This fixture starts from `export/005-attachments-inline`, increments the
`byte_length` field inside `061-attachments.cbor`, and leaves the signed
`000-manifest.cbor` unchanged. The verifier must localize the failure to the
ADR 0072 attachment-manifest digest bound by
`trellis.export.attachments.v1.attachment_manifest_digest`.
