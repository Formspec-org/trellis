# Derivation — `export/010-certificate-of-completion-inline`

ADR 0007 §"Export manifest catalog" reference export. The chain carries
three events on `ledger_scope = b'trellis-cert:export-010'`:

1. **Event 0 (sequence 0).** `formspec.attachment.added` binding event
   (ADR 0072 mirror) — `payload_ref = PayloadExternal` whose
   `content_hash = SHA-256(presentation-artifact-bytes, domain
   "trellis-content-v1")`. The `trellis.evidence-attachment-binding.v1`
   extension declares `attachment_id = 'urn:trellis:attachment:cert-export-010'`.

2. **Event 1 (sequence 1).** `wos.kernel.signatureAffirmation` Facts-tier
   provenance record. `data.formspecResponseRef = "sha256:1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef"` —
   a `sha256:<hex>` digest text so ADR 0007 step 7 cross-check has
   parseable input. The certificate's `chain_summary.response_ref` echoes
   the same 32-byte digest.

3. **Event 2 (sequence 2).** `trellis.certificate-of-completion.v1` —
   `presentation_artifact.attachment_id` resolves through event 0's
   binding; `presentation_artifact.content_hash =
   SHA-256(presentation-artifact-bytes, domain
   "trellis-presentation-artifact-v1")`; `signing_events[0] = canonical_event_hash(event 1)`.

The export ZIP carries `065-certificates-of-completion.cbor` (one row),
bound through `trellis.export.certificates-of-completion.v1` with
`catalog_digest = SHA-256(catalog bytes)` (bare, no domain tag).

The presentation-artifact bytes are the **same** bytes the binding event's
`PayloadExternal.content_hash` covers — but the certificate's
`presentation_artifact.content_hash` is computed under a DIFFERENT domain
tag (`trellis-presentation-artifact-v1`), giving a different digest. The
verifier resolves this via `verify_certificate_attachment_lineage`:
`payload_blobs[binding.payload_content_hash]` returns the bytes,
`SHA-256(bytes, "trellis-presentation-artifact-v1")` recomputes the
certificate's content_hash, and equality flips
`outcome.attachment_resolved = true`.

Generator: `_generator/gen_export_010_certificate_of_completion.py`.
