# Trellis Export (Fixture) — export/009-intake-handoffs-public-create-empty-outputs

Negative ADR 0073 export fixture. `063-intake-handoffs.cbor` still
binds the same Formspec handoff and canonical Response bytes, but the
embedded WOS `intakeAccepted` and `caseCreated` payloads carry empty
`outputs` arrays so verifier parsing must fail before handoff matching.
