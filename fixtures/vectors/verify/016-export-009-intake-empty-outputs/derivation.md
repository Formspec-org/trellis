# Derivation — `verify/016-export-009-intake-empty-outputs`

This fixture starts from `export/009-intake-handoffs-public-create-empty-outputs`.
The archive is structurally sound, but Trellis intake verification must reject
the first admitted WOS payload because `outputs` is empty on the
`intakeAccepted` record.
