# Derivation — `append/021-wos-intake-accepted-public-create`

This fixture wraps a WOS `intakeAccepted` facts-tier record in a Trellis Phase-1
envelope for the ADR 0073 public-intake path. The accepted outcome is
`createGovernedCase`, so the record carries the created case ref and the pinned
Definition URL/version that justified the new governed case.
