# Derivation — `append/020-wos-intake-accepted-workflow-attach`

This fixture wraps a WOS `intakeAccepted` facts-tier record in a Trellis Phase-1
envelope. The authored payload models the ADR 0073 workflow-initiated path:
the Formspec handoff already names the governed case, so the accepted outcome
is `attachToExistingCase` and no `caseCreated` record is emitted.
