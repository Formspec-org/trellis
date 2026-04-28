# Trellis reference artifacts

Non-normative reference implementations published alongside Trellis ADRs
that ship operator-side adopter material. ADRs cite this directory by
path; the contents are NOT part of normative spec evidence.

| Path | ADR | Purpose |
|------|-----|---------|
| `certificate-of-completion/template-v1/` | ADR 0007 | Reference HTML + print stylesheet for `PresentationArtifact`; rendered to PDF via headless Chromium / WeasyPrint. `template_id = "trellis.reference.certificate-of-completion.v1"`. |

Operators are free to use, extend, or ignore each artifact. ADR text and
canonical fixtures are the load-bearing surface; this directory exists so
adopters have a working starting point.
