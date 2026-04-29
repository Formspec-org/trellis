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
- `author_event_hash`: `38d5cb021db56a20346930dde43c0fb174db237709155f3ec47628bfe4641195`
- `canonical_event_hash`: `deef7001a81a7133d5cd9a1ec0e9db27e05883022eef3b7fab5d6d5faf2abd0b`
