# Derivation — `tamper/022-cert-signing-event-unresolved`

Starts from `export/010-certificate-of-completion-inline`. The certificate event's `signing_events[0]` is rewritten to `0xff…ff` (32 bytes). The chain still carries the original SignatureAffirmation event at sequence 1, but its `canonical_event_hash` does not match the rewritten digest.

`finalize_certificates_of_completion` runs step 5 (signing-event resolution) over the full chain context provided by the export-bundle path, fails to find a matching event in `event_by_hash`, and emits `signing_event_unresolved` localized to the unresolvable digest hex. ADR 0007 step 5 also covers wrong-event-type resolution (a digest pointing at a non-SignatureAffirmation event); this vector exercises the missing-event sub-case.

Generator: `_generator/gen_export_010_certificate_of_completion.py`.
