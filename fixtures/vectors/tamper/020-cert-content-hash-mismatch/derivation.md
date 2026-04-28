# Derivation — `tamper/020-cert-content-hash-mismatch`

Starts from `export/010-certificate-of-completion-inline`. The certificate event's `presentation_artifact.content_hash` is XOR-flipped from the correct value (`SHA-256(presentation-artifact-bytes, "trellis-presentation-artifact-v1")`). The bound attachment bytes in `060-payloads/` are unchanged.

`verify_certificate_attachment_lineage` resolves the attachment via `presentation_artifact.attachment_id` → binding event → `payload_blobs`, then recomputes the digest under the certificate's domain tag and finds it disagrees with the certificate's claim. ADR 0007 step 4 fails closed with `presentation_artifact_content_mismatch`, distinct from `presentation_artifact_attachment_missing` (which fires when bytes are absent or lineage is unresolvable).

Generator: `_generator/gen_export_010_certificate_of_completion.py`.
