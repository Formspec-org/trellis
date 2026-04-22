# Derivation — `verify/013-export-005-missing-attachment-body`

This fixture starts from `export/005-attachments-inline`, removes only the
`060-payloads/<payload_content_hash>.bin` member, and keeps the signed manifest
unchanged. The regular required-member digest checks still pass; the verifier
must fail the ADR 0072 inline-attachment obligation because the manifest
extension declares `inline_attachments = true`.
