# Derivation — `export/005-attachments-inline`

This fixture realizes the Trellis side of ADR 0072 for export bundles.

It starts from `append/018-attachment-bound`, packages that canonical event
as the only event in the export, includes the attachment ciphertext at
`060-payloads/<payload_content_hash>.bin`, derives `061-attachments.cbor`
from the chain-authored `EvidenceAttachmentBinding`, and binds that derived
manifest through `ExportManifestPayload.extensions["trellis.export.attachments.v1"]`.

The attachment manifest is a dCBOR array of `AttachmentManifestEntry` maps.
The entry's `binding_event_hash` is the canonical event hash of `append/018`,
and its `payload_content_hash` equals both the event `content_hash` and the
`PayloadExternal.content_hash`.
