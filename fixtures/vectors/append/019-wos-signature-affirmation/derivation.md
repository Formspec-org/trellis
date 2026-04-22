# append/019 — WOS `SignatureAffirmation` through `custodyHook`

Genesis append vector for a WOS Signature Profile `SignatureAffirmation`
Facts-tier provenance record. Same §23 composition contract as
`append/010-wos-custody-hook-state-transition`, but the payload family is
`wos.kernel.signatureAffirmation` and the readable inline bytes are the
WOS-authored `signatureAffirmation` record (dCBOR).

## Pinned WOS inputs

`input-wos-record.dcbor` pins a minimal schema-shaped record aligned with the
stack’s signed-response example: signer, role, document hash, consent reference
paths into `authoredSignatures[…]`, identity binding, provider, ceremony,
profile ref, and `formspecResponseRef` for the shared fixture URL.

`input-wos-idempotency-tuple.cbor` carries the ADR-0061 tuple
`(caseId, recordId)` used to derive `idempotency_key`.

## Outputs

Byte-exact `expected-event.cbor`, `expected-event-payload.cbor`, and
`expected-append-head.cbor` are produced by `fixtures/vectors/_generator/gen_append_019.py`.
