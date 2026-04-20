# Event Registry — SSDI Intake Triage (stub)

This stub represents the operator event-type registry referenced by
`audit.registry_ref = "urn:example:wos/event-registry/v2"` in
`declaration.md`. It is intentionally narrow: it exists so the A.6 static
declaration-doc validator can resolve the declared delegated-compute event
types without replaying a ledger.

Registered event types:

- `wos.agent.delegated_compute.read.v1`
- `wos.agent.delegated_compute.propose.v1`
- `wos.agent.delegated_compute.grant.v1`
