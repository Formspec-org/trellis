# Derivation — `export/007-intake-handoffs-public-create`

This fixture realizes the Trellis side of ADR 0073 for the public-intake path.

The export carries two admitted WOS facts-tier events in canonical order:

1. `wos.kernel.intakeAccepted`
2. `wos.kernel.caseCreated`

`063-intake-handoffs.cbor` is chain-derived rather than independently authored:
it names the admitting event hashes, embeds the exact Formspec `IntakeHandoff`,
and carries the exact canonical Response envelope bytes whose SHA-256 digest was
stored in `handoff.responseHash`.
