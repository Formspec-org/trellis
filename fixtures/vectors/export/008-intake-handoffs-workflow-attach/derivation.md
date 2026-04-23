# Derivation — `export/008-intake-handoffs-workflow-attach`

This fixture realizes the Trellis side of ADR 0073 for the workflow-initiated
attach path. The export carries one admitted WOS `intakeAccepted` event and a
catalog row in `063-intake-handoffs.cbor` whose `case_created_event_hash` is
null because no governed-case birth occurred.
