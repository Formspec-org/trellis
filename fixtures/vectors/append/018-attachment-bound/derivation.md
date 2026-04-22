# append/018 attachment-bound

This vector pins the Trellis half of ADR 0072 for a Formspec-originated
attachment binding.

## Inputs

- Formspec authored fixture: `input-formspec-respondent-ledger-event.json`,
  copied from the Formspec workspace file `fixtures/respondent-ledger/attachment-added-binding.json` (repository root; not under `trellis/fixtures/vectors/`).
- Attachment ciphertext bytes: `input-attachment-ciphertext.bin`.
- Binding metadata: `input-evidence-attachment-binding.cbor`, the dCBOR
  encoding of `attachmentBinding` from the Formspec fixture.

## Contract

The Trellis event is a genesis append with:

- `EventHeader.event_type = "formspec.attachment.added"`
- `EventPayload.payload_ref = PayloadExternal`
- `EventPayload.extensions["trellis.evidence-attachment-binding.v1"] =
  EvidenceAttachmentBinding`
- `EventPayload.content_hash = PayloadExternal.content_hash =
  EvidenceAttachmentBinding.payload_content_hash`

The payload hash is over the attachment ciphertext bytes named by
`PayloadExternal`, not over the binding metadata.

## Pinned hashes

- `content_hash`: `e796c89473ca5db95c63050683cb05b971b8815380e847ef560d32f805459fe6`
- `idempotency_key`: `1ddbdf32dcb1cbf5b2fa379a358757cb95e1863426a3243bbf92bac4ef1a82fc`
- `author_event_hash`: `801d0ca7b66ecaec41a9876671633ad015a0614eb5f058482924dd66a67d1511`
- `canonical_event_hash`: `03c83d06c490a28e3101bdf30887da2242c4c1a44078aab633b7eb4d2c4b4934`
