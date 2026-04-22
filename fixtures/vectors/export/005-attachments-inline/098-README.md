# Trellis Export (Fixture) — export/005-attachments-inline

ADR 0072 attachment-binding export fixture. The attachment ciphertext is present under `060-payloads/`, and `061-attachments.cbor` is bound by `ExportManifestPayload.extensions["trellis.export.attachments.v1"]`.
