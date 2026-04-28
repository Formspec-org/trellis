# Derivation — `export/013-intake-handoffs-public-create-empty-outputs`

This fixture mirrors `export/007-intake-handoffs-public-create`, but the
embedded WOS `intakeAccepted` and `caseCreated` records carry empty `outputs`
arrays. The archive must still verify structurally, but Trellis intake
verification must reject the payloads before catalog matching.
