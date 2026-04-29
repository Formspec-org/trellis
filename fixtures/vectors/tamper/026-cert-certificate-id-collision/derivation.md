# Derivation — `tamper/026-cert-certificate-id-collision`

Two-event chain on a single ledger_scope. Both events share
`certificate_id = "urn:trellis:certificate:test:028"`. Event 0 is a byte-
exact clone of `append/028-certificate-of-completion-minimal-pdf`'s payload
(idempotency_key tweaked to dodge §17.3 collision under combined replay).
Event 1 mutates `presentation_artifact.content_hash` to a 32-byte all-`0xff`
value, making its canonical certificate payload byte-different from event
0's. The `prev_hash` chain links event 1 to event 0 normally so
`_verify_event_set` admits the structural form.

Per ADR 0007 §"Field semantics" `certificate_id` clause:

> If the operator re-emits the same certificate_id with a different payload
> (different content_hash, signing_events, or chain_summary), that is a
> chain policy violation: the verifier treats the duplicate as
> certificate_id_collision and flips integrity_verified = false.

`finalize_certificates_of_completion` collects all in-scope certificate
events and runs the collision pass; it reports
`certificate_id_collision` localized to event 1's canonical_event_hash.

Event 0 canonical_event_hash: `eca2892b2c4d1ccf973067aa12a5aeb633baaaf89a9765e76c03964684e5ac5c`
Event 1 canonical_event_hash: `a8e0bd7ab927c61a0ee144702d540c96fbafc6a0eae264ec932610881f737f06` (failing event)

Generator: `_generator/gen_tamper_021_023_025_026.py`.
