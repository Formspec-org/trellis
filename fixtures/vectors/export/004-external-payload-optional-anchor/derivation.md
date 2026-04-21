# Derivation — `export/004-external-payload-optional-anchor`

Single-event export that bundles a `PayloadExternal` body under `060-payloads/<content_hash>.bin` and declares optional external anchoring (`external_anchor_required = false`) in the manifest posture declaration. Exercises export ZIP member variety and manifest-variant coverage without forcing an anchor-resolution failure.

This fixture was generated deterministically from committed append bytes and the pinned key / registry inputs named in the generator source. The archive members, manifest digests, checkpoint chain, and deterministic ZIP root are the evidence of record.
