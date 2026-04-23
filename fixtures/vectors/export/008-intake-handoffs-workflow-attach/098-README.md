# Trellis Export (Fixture) — export/008-intake-handoffs-workflow-attach

ADR 0073 workflow-initiated export fixture. `063-intake-handoffs.cbor`
binds the Formspec `IntakeHandoff`, the canonical Response bytes used
for `responseHash`, and the Trellis event hash of the admitted WOS
`intakeAccepted` record. No `caseCreated` event appears because the
handoff attaches to an existing governed case.
