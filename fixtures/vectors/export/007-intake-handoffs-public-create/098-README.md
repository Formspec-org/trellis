# Trellis Export (Fixture) — export/007-intake-handoffs-public-create

ADR 0073 public-intake export fixture. `063-intake-handoffs.cbor` binds the
Formspec `IntakeHandoff`, the canonical Response bytes used for
`responseHash`, and the Trellis event hashes of the WOS `intakeAccepted`
and `caseCreated` records so offline verification can replay the whole
submission → intake acceptance → governed-case birth path.
