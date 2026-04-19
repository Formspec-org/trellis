---
# Delegated-Compute Declaration — SSDI Intake Triage (reference)
#
# On-disk format per Companion §A.6: TOML frontmatter carries the
# machine-checkable claim; the Markdown body below carries operator narrative.
# This artifact is a REFERENCE illustration, not a conformance fixture;
# signature bytes are pinned placeholder values clearly marked below.
#
# Instance-path convention (proposed — see Markdown body §"Instance path convention"):
#   fixtures/declarations/<deployment-slug>/declaration.md
#   fixtures/declarations/<deployment-slug>/posture-declaration.stub.md  (companion posture stub)
# The <deployment-slug> is a short kebab-case identifier stable across
# revisions; successive versions of the SAME deployment live under the same
# slug directory and chain via `supersedes`.

declaration_id          = "urn:example:ssdi-intake-triage/declaration/v1"
operator_id             = "urn:example:operator/ssa-example-adjudication-unit"
posture_declaration_ref = "urn:example:ssdi-intake-triage/posture-declaration/v1"
effective_from          = 2026-05-01T00:00:00Z
# supersedes omitted — key absence denotes null for URI|null fields per Companion A.6

[scope]
# Phase 1 authorized actions. "decide" is reserved and NON-CONFORMANT per A.6 rule 4
# (invariant #15). This deployment deliberately withholds "decide" authority from
# the agent; the human adjudicator remains the sole commit_on_behalf_of principal.
authorized_actions      = ["read", "propose"]
content_classes         = [
  "ssdi.intake.questionnaire_response",
  "ssdi.intake.supporting_document_text",
]
# Phase 1 deployments without a case ledger MUST set max_agents_per_case = null.
# TOML has no explicit null; empty-integer is disallowed, so the A.6 convention
# uses key absence to encode null. See "A.6 authoring conventions applied here"
# in the body.
# max_agents_per_case   = (absent — treated as null per A.6 Phase 1 convention)
max_invocations_per_day = 500
time_bound              = 2027-01-01T00:00:00Z
purpose_bound           = "draft adjudication triage narrative"

[authority]
grantor_principal       = "urn:example:wos:role/adjudication-oversight"
grantor_role_tier       = "wos.governance.tier/senior-reviewer"
wos_autonomy_cap_ref    = "urn:example:wos:autonomy-cap/ssdi-triage-assistive-v1"
delegation_chain        = [
  "urn:example:wos:role/adjudication-oversight",
  "urn:example:wos:role/program-integrity-director",
]

[audit]
# Each identifier MUST appear in the operator's event-type registry (Core §6.7).
registry_ref = "urn:example:wos/event-registry/v2"
event_types = [
  "wos.agent.delegated_compute.read.v1",
  "wos.agent.delegated_compute.propose.v1",
  "wos.agent.delegated_compute.grant.v1",
]

[attribution]
agent_identity                  = "urn:example:agent/ssdi-triage-drafter/v1"
actor_discriminator_rule        = "exactly_one_of(actor_human, actor_agent_under_delegation)"
attribution_fields_emitted      = [
  "actor_agent_under_delegation",
  "agent_identity",
  "delegation_grant_id",
  "wos_autonomy_cap_ref",
]

[supply_chain]
# MUST match the A.2 access_taxonomy row's delegated_compute_exposure enum value
# for every listed content_class. Enum domain: {provider_operated, tenant_operated,
# client_side, isolated_enclave}. See narrative justification in body.
runtime_enclave  = "isolated_enclave"
model_identifier = "claude-sonnet-4-6@v1"

[signature]
# PLACEHOLDER signature for reference purposes — this is NOT a conformance fixture.
# A conforming declaration carries a COSE_Sign1 signature over the canonical bytes
# of the frontmatter (domain-separated per Core §9.1). Replace with a real signer
# output before use in any conformance run.
cose_sign1_b64 = "AAAA-placeholder-signature-for-reference-purposes-only-AAAA"
signer_kid     = "urn:example:operator/ssa-example-adjudication-unit#key-2026-05"
alg            = "EdDSA"
---

# SSDI Intake Triage — Delegated-Compute Declaration (reference)

## Deployment context

This deployment is operated by a notional Social Security Disability Insurance
(SSDI) adjudication unit using the Formspec/Trellis/WOS stack end-to-end. An
LLM triage agent reads inbound intake questionnaire responses and scanned
supporting-document text extracts, then drafts a recommendation narrative that
is attached to the adjudicator's fact record as a proposed — not committed —
artifact. A human adjudicator reviews the draft, edits as needed, and issues
the binding decision. The adjudicator is the sole principal authorized to
`commit_on_behalf_of` the claimant's case record. The agent never decides; it
only reads and proposes.

The governing WOS autonomy cap is `assistive` (WOS AI Integration §5.2):
every admission of agent-authored content requires explicit human
confirmation. No admission occurs from the agent's own authority.

## Rationale for `supply_chain.runtime_enclave = "isolated_enclave"`

The agent runtime is hosted in a single-tenant confidential-compute enclave
operated by the model-inference provider but attested to the operator. The
enclave's sealing key is bound to the operator's posture KMS root, and
plaintext intake payloads are decrypted only inside the enclave boundary.
Neither the provider's ordinary service components nor the operator's general
application tier see plaintext intake content for this deployment. This
matches the A.2 `delegated_compute_exposure` value `isolated_enclave` for
both listed content classes in the referenced Posture Declaration.

A `tenant_operated` posture was considered and rejected: the SSDI workload
needs model weights the operator does not host, so keeping inference on
tenant hardware is infeasible. A `provider_operated` posture was rejected as
too permissive: it would expose plaintext to provider-side observability,
violating the custody-model honesty claim (§11).

## Cross-check surface — expected evidence

The 15 cross-check surfaces of §A.6 are reconciled as follows. The first six
are statically checkable from this frontmatter plus the referenced posture
document; the remainder are checked by the Rust conformance crate against
on-ledger evidence once G-4 lands.

1. `posture_declaration_ref` resolves to `urn:example:ssdi-intake-triage/posture-declaration/v1`. That posture's A.2 access_taxonomy rows for `ssdi.intake.questionnaire_response` and `ssdi.intake.supporting_document_text` both declare `access_class = delegated_compute`. See `posture-declaration.stub.md`.
2. `operator_id` equals the posture declaration's `operator_id` (`urn:example:operator/ssa-example-adjudication-unit`).
3. Each string in `audit.event_types` appears in the operator's event-type registry at `urn:example:wos/event-registry/v2` — specifically the `wos.agent.delegated_compute.*` family registered under Core §6.7.
4. `scope.authorized_actions` is `["read", "propose"]`. It does not contain `"decide"`; this deployment deliberately withholds decide authority from the agent (Phase 1 non-conforming value per A.6 rule 4 / invariant #15).
5. `attribution.actor_discriminator_rule` is the exact literal string `"exactly_one_of(actor_human, actor_agent_under_delegation)"`.
6. `supply_chain.runtime_enclave = "isolated_enclave"` equals the `delegated_compute_exposure` value declared in A.2 for every listed content class.
7. Vacuously satisfied: this Phase 1 deployment exposes no case-scope ledger, so `max_agents_per_case` is null by the A.6 key-absence convention (see "A.6 authoring conventions applied here" below).
8. The invocation-budget pipeline enforces `max_invocations_per_day = 500` per agent identity per UTC day; breach increments a posture-honesty incident per §11.
9. `authority.wos_autonomy_cap_ref` resolves to a WOS autonomy cap whose scope is `{read, propose}` over the same content classes — a strict superset of this declaration's scope.
10. `delegation_chain` is monotonic: the senior reviewer role inherits its delegation authority from the program-integrity director role; both are resolvable under `policy_authority` at `effective_from`.
11. Every event emitted under this declaration honors the actor discriminator: triage-author events populate `actor_agent_under_delegation` only; adjudicator-confirmed admissions populate `actor_human` only.
12. Every emitted event's `agent_identity` attribution field equals `urn:example:agent/ssdi-triage-drafter/v1`.
13. Every emitted event type is contained in the `audit.event_types` list; unlisted types fail admission at the custody hook.
14. `signature` — placeholder only in this reference artifact; in a conformance instance, the signature MUST verify per Core §9.1.
15. `supersedes` is omitted (no predecessor). On a future revision, the chain MUST be acyclic and each predecessor MUST have been in force at the successor's `effective_from`.

## Posture-Declaration Honesty

Per Companion §11, the operator asserts that the declared custody model,
access taxonomy, metadata budget, and delegated-compute posture above
accurately reflect the behavior of the deployed system. A mismatch between
any declared field and observed control-plane behavior is a posture-honesty
violation (§11.2) and MUST be remediated by publishing a corrected Posture
Declaration and an incident record per §11.4.

## Phase 1 non-conformance of `"decide"`

A.6 rule 4 marks `"decide"` as a reserved, NON-CONFORMANT value for
`scope.authorized_actions` in Phase 1. This deployment acknowledges that
authority: the triage agent is authorized to `read` and `propose` only. Any
attempt to emit a decide-class event under this declaration's scope would
fail the custody-hook admission check and MUST be recorded as a
posture-honesty incident. Future phases may relax this restriction; until
then, human adjudication is the sole source of decide authority.

## Instance path convention (proposed for orchestrator adoption)

A.6 does not pin a filesystem path for Declaration instances. For Trellis
reference fixtures and for operator deployments that colocate declarations
with their posture docs, this artifact proposes:

```
<declarations-root>/<deployment-slug>/declaration.md
<declarations-root>/<deployment-slug>/posture-declaration.stub.md   # or full posture-declaration.md
<declarations-root>/<deployment-slug>/history/declaration.v<N>.md   # optional superseded revisions
```

Where `<deployment-slug>` is a short, stable, kebab-case identifier for the
deployment (here, `ssdi-intake-triage`). Successive revisions of the same
deployment live under the same slug directory; each revision's frontmatter
carries a monotonically increasing `declaration_id` version suffix and a
`supersedes` pointer to the prior declaration_id. This convention is
proposed, not normative; the orchestrator should ratify or replace it before
the Wave 1 lint-refactor plan lands A.6 schema linting.

## A.6 authoring conventions applied here

This artifact follows the now-pinned A.6 TOML conventions:

- Nullable frontmatter fields use key absence for `null`; `supersedes` and
  `scope.max_agents_per_case` are therefore omitted.
- The operator signature is serialized as `[signature]` with
  `cose_sign1_b64`, `signer_kid`, and `alg`.
- `audit.registry_ref = "urn:example:wos/event-registry/v2"` supplies the
  machine-readable event-type registry pointer used for rule 3.
