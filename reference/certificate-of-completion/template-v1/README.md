# Reference Certificate-of-Completion Template v1

**Non-normative.** Operators MAY use, extend, or ignore this template.
ADR 0007 §"Reference template" pins the file location and the template
identifier; the rendered bytes are an operator concern.

## Identity

- **template_id:** `trellis.reference.certificate-of-completion.v1`
- **template_hash (SHA-256 over `template.html` bytes):**
  `ca2e669d67bc4c1fa87a832b7463f99757ca3c81486f23909db25bb4bf5cb74e`
- **Bare-hex copy for tooling:** `template_hash.txt`

Operators using this template set both fields on
`PresentationArtifact`:

```cddl
PresentationArtifact = {
  ...
  template_id:   "trellis.reference.certificate-of-completion.v1",
  template_hash: <32 bytes — SHA-256 of template.html>,
  ...
}
```

A third-party auditor can recompute the hash from the committed
`template.html` bytes and confirm the operator did not silently render
through a different template.

## What it is

Single-file HTML + a colocated `template.css` print stylesheet. Renders
to PDF via any standard headless pipeline:

```sh
# Chromium headless (Linux / macOS):
chromium --headless --disable-gpu \
  --print-to-pdf=certificate.pdf \
  template.html

# WeasyPrint (pure-Python alternative):
weasyprint template.html certificate.pdf
```

The HTML uses `{{placeholders}}` and `{{#each ...}}` blocks for
operator-side templating. ADR 0007 does NOT pin a templating engine;
the chosen syntax is Mustache/Handlebars-shaped because that family
ports cleanly to most server runtimes. Operators wiring their own
template engine substitute the placeholders before rendering.

## Placeholder reference

Top-level fields (one substitution each):

| Placeholder                       | Source field                                             |
|-----------------------------------|----------------------------------------------------------|
| `{{certificate_id}}`              | `CertificateOfCompletionPayload.certificate_id`          |
| `{{completed_at_iso}}`            | `completed_at` rendered as ISO-8601 (operator format)    |
| `{{workflow_status}}`             | `chain_summary.workflow_status`                          |
| `{{impact_level_or_dash}}`        | `chain_summary.impact_level` (`—` when null)             |
| `{{case_ref_or_dash}}`            | `case_ref` (`—` when null)                               |
| `{{workflow_ref_or_dash}}`        | `workflow_ref` (`—` when null)                           |
| `{{response_ref_hex_or_dash}}`    | `chain_summary.response_ref` hex (`—` when null)         |
| `{{presentation_content_hash_hex}}` | `presentation_artifact.content_hash` hex                |
| `{{media_type}}`                  | `presentation_artifact.media_type`                       |
| `{{byte_length}}`                 | `presentation_artifact.byte_length`                      |
| `{{attachment_id}}`               | `presentation_artifact.attachment_id`                    |
| `{{template_id_or_dash}}`         | `presentation_artifact.template_id` (`—` when null)      |
| `{{template_hash_hex_or_dash}}`   | `presentation_artifact.template_hash` hex (`—` when null)|

Repeated blocks:

| Block                       | Iterates over                  |
|-----------------------------|--------------------------------|
| `{{#each signer_display}}`  | `chain_summary.signer_display` |
| `{{#each signing_events}}`  | `signing_events` (digest list) |
| `{{#each attestations}}`    | `attestations`                 |

Inside `signer_display`: `{{principal_ref}}`, `{{display_name}}`,
`{{display_role_or_dash}}`, `{{signed_at_iso}}`, `{{@index_plus_one}}`.

Inside `signing_events`: `{{this_hex}}` (32-byte digest as hex).

Inside `attestations`: `{{authority}}`, `{{authority_class}}`.

## What this template does NOT do

- It does not embed PAdES / eIDAS / ESIGN signature blocks. Operators
  needing those produce a PAdES-signed PDF themselves and bind it via
  ADR 0007's `presentation_artifact` (the chain reference holds either
  way).
- It does not localize date or name formats. `{{completed_at_iso}}` and
  `{{signed_at_iso}}` take operator-formatted strings; date locale is
  the operator's responsibility.
- It does not assert WCAG accessibility tagging. Tagged-PDF output is a
  separate adopter concern (see ADR 0007 §"Open questions" item 2).

## Trust boundary

A verifier holding the export bundle + this rendered PDF can confirm:

1. The PDF's bytes hash to `presentation_artifact.content_hash` under
   domain tag `trellis-presentation-artifact-v1`.
2. The signer count printed in the table matches
   `chain_summary.signer_count`.
3. Each `signing_events[i]` digest resolves to a chain-present
   `SignatureAffirmation` event.
4. (When `template_id` and `template_hash` are both set) the rendered
   bytes were produced from this template and not silently substituted.

A verifier cannot detect rendering-level divergence inside the
operator-supplied fields (e.g., `display_name` typo, locale drift,
header text edits). ADR 0007 §"Adversary model" names this trust
boundary explicitly.
