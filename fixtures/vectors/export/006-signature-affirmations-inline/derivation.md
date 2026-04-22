# Derivation — `export/006-signature-affirmations-inline`

This fixture realizes the Trellis side of the WOS-T4 signature export contract.

It starts from `append/019-wos-signature-affirmation`, packages that canonical
event as the only event in the export, and derives
`062-signature-affirmations.cbor` from the readable WOS-authored
`SignatureAffirmation` payload already carried inside the signed event.

The catalog is chain-derived rather than independently authored: each row names
the admitting `canonical_event_hash` and repeats the WOS evidence fields needed
for a certificate-of-completion renderer to summarize the signing act without
redefining canonical authority. The human-facing certificate remains a derived
artifact; the signed Trellis export remains the authority.
