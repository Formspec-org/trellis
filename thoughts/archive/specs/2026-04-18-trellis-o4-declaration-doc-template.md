# Trellis O-4 Delegated-Compute Declaration Document — Template Design

**Date:** 2026-04-18
**Scope:** Template shape, schema, lint surface, and minimum-viable close for ratification gate **O-4** (delegated-compute honesty) per `ratification/ratification-checklist.md`.
**Closes:** O-4 at the *design* level; one reference declaration document + a passing lint pass closes the gate at the *evidence* level.
**Does not cover:** authoring lint rules into `scripts/check-specs.py`, authoring the reference declaration itself (follow-on `S`), or extending the template beyond Phase 1 obligations.

## Context

Companion §19 (Delegated-Compute Honesty) requires that every agent-in-the-loop deployment record, *before compute begins*, an explicit grant covering scope, authority attestation, audit trail, and attribution — with no scope drift and no silent extension (OC-64..OC-70). Companion §11 (Posture-Declaration Honesty) requires a separate, export-scoped Posture Declaration enumerating access taxonomy, custody model, and crypto-shredding scope. Core §20 (Trust Posture Honesty) lifts invariant #15 to a MUST NOT: implementations MUST NOT describe posture more strongly than behavior supports, and `PostureDeclaration.delegated_compute = true` is the export-manifest bit that flags "an agent touches plaintext here."

These three obligations fit together but are not interchangeable. Core §20 gates the bit. Companion §11 produces the export-scoped posture document. Companion §19 produces the *per-deployment grant-family declaration* — what a given agent is authorized to do, on whose authority, and how its actions show up in the ledger. O-4 closes when a reference declaration of this third kind exists, is machine-checkable, and passes lint.

Appendix A of the Companion already carries a partial template: A.1 top-level `PostureDeclaration`, A.2 access taxonomy row, A.3 metadata budget row, A.4 custody-model registry entry, A.5 posture-transition event. Appendix B.4 carries a `DelegatedComputeGrant` *minimal* shape (grantor, grantee, policy authority, scope, time/purpose bound, agent provenance, canonical-fact reference, signature). B.4 is the right seed — it names the fields §19 requires — but it is non-normative and undersized for a machine-checkable audit surface. This design extends B.4 into a full per-deployment declaration document; Appendix A's `PostureDeclaration` is preserved as the §11 sibling and referenced, not replaced.

## Template structure — three candidates

### Option 1 — Flat YAML single file

One YAML document per deployment. Flat-ish keys, mild nesting. Pros: highest operator-authoring ergonomics (comments, anchors, most legible for hand-editing), diff-friendly, no tooling needed. Cons: YAML's type ambiguity (`on`/`off`, timestamp parsing, octal surprises) is a hazard for lint; two YAML parsers can disagree on edge-cases in a way TOML/JSON do not.

### Option 2 — Hierarchical JSON with schema

One JSON document per deployment, JSON Schema alongside. Pros: unambiguous types, canonical ecosystem for machine-lint, aligns with Companion §11's "machine- and human-readable … Operators SHOULD publish a machine-readable form (JSON or CBOR)." Cons: no comments, hand-authoring friction, operator will reach for a tool to maintain it.

### Option 3 — Markdown with TOML frontmatter

One `.md` file per deployment. TOML frontmatter carries the machine-checkable fields; Markdown body carries operator narrative (governance references, human-readable scope description, supply-chain caveats). Pros: mirrors `fixtures/vectors/` manifest discipline already established in the G-3 fixture-system design (TOML for machine-lint, Markdown siblings for human prose); operator-ergonomic (the narrative lives next to the machine bits instead of in a second file); diff-reviewable; TOML parses deterministically in Rust/Python/Go. Cons: two formats in one file; extraction requires a frontmatter parser (trivial, already present in the Python ecosystem).

**Chosen: Option 3 — Markdown with TOML frontmatter.** Three reasons. First, Trellis has already picked TOML as its machine-lint substrate (`fixtures/vectors/*/manifest.toml`) — reusing that choice keeps the toolchain narrow and lets `scripts/check-specs.py` share TOML-parsing machinery. Second, declaration documents are governance artifacts: the human-audience prose (why this agent, what the grantor authorizes, which WOS tier) is load-bearing and should not live in a separate file operators forget to update. Third, Companion §11.1 explicitly wants human- and machine-readable side-by-side; Markdown-with-frontmatter delivers that in one reviewable artifact rather than two.

## Template — the machine-checkable shape

Files live at `deployments/<deployment-id>/declaration.md`. The path is declared inside the document as `declaration_uri` so lint can round-trip. Frontmatter is TOML. Body is Markdown narrative.

### Frontmatter skeleton

```toml
# Companion §19 per-deployment delegated-compute declaration.
# Sibling of the export-scoped Posture Declaration (Companion §11, Appendix A.1).

declaration_id   = "urn:trellis:declaration:ssdi-intake-triage:2026-04-18"
deployment_id    = "ssdi-intake-triage-v1"
operator_id      = "urn:gov:example-state:disability-determination-services"
declaration_uri  = "deployments/ssdi-intake-triage-v1/declaration.md"
effective_from   = "2026-04-18T00:00:00Z"
supersedes       = ""                                # prior declaration_id, or ""
posture_declaration_ref = "urn:trellis:posture:ssdi:2026-04-01"  # Companion §11 / Appendix A.1

# Scope — Companion §19.1, §19.2 (OC-64 §3 "scope"), §19.5 (OC-68 no scope drift)
[scope]
authorized_actions = ["read", "propose"]             # enum: read | propose | commit_on_behalf_of | decide
content_classes    = ["respondent_pii", "intake_narrative"]
fact_families      = ["intake-fact", "triage-recommendation"]
decision_class     = "advisory"                      # §19.4 item 5: advisory | recommendatory | decision_contributory
max_agents_per_case      = 1                         # cross-checkable vs observed
max_invocations_per_case = 8                         # cross-checkable vs observed
time_bound       = "2027-04-18T00:00:00Z"            # §19.2 OC-65; "" only if open_ended_permitted = true
purpose_bound    = "first-pass triage recommendation for adjudicator review"
open_ended_permitted = false                         # §19.2 OC-65 exception flag

# Authority attestation — Companion §19.3 (OC-66), §19.6 (WOS autonomy caps)
[authority]
grantor_principal    = "urn:gov:example-state:dds:role:intake-lead"
grantor_role_tier    = "WOS:tier:operator-adjudicator"   # must resolve in WOS role registry
policy_authority     = "urn:gov:example-state:dds:policy:ai-triage-2026-01"
wos_autonomy_cap_ref = "urn:gov:example-state:dds:wos:autonomy-cap:triage-tier-2"
delegation_chain     = [
  "urn:gov:example-state:dds:role:commissioner",
  "urn:gov:example-state:dds:role:intake-lead",
]

# Audit trail — Companion §19.2 (OC-64 auditable), §19.4 (OC-67 attribution)
[audit]
grant_event_type              = "trellis.grant.delegated-compute.v1"
invocation_event_type         = "trellis.compute.invocation.v1"
output_event_type             = "trellis.compute.output.v1"
revocation_event_type         = "trellis.revocation.delegated-compute.v1"
attribution_fields_emitted    = [
  "grantor_principal", "grantee_agent_id", "grant_ref", "scope_ref",
  "human_reviewer_principal", "decision_class",
]
human_reviewer_approval_link  = "reviewer_approval_event_ref"  # field name inside output events

# Attribution — Companion §19.4 (OC-67), Core §20 (invariant #15)
[attribution]
agent_identity          = "urn:vendor:example-ai:triage-model:v4.2"
agent_provenance_ref    = "urn:vendor:example-ai:model-card:triage-v4.2"
training_data_provenance = "declared-in-model-card"
human_actor_field       = "actor_human"              # set when a human commits
agent_actor_field       = "actor_agent_under_delegation"  # set when agent commits under grant
actor_discriminator_rule = "exactly_one_of(actor_human, actor_agent_under_delegation)"

# Supply chain — Companion §19.8 (OC-70)
[supply_chain]
model_identifier     = "example-ai/triage:v4.2"
model_version_hash   = "sha256:..."
runtime_enclave      = "tenant_operated"             # matches access_taxonomy row (§A.2)
model_signature_ref  = "urn:vendor:example-ai:signature:v4.2"

# Operator signature over the frontmatter preimage
[signature]
signer        = "urn:gov:example-state:dds:role:intake-lead"
alg           = "COSE:ES256"                         # declared; verified against operator key registry
signature_b64 = "..."
signed_fields_digest_alg = "SHA-256"
```

### Schema sketch (JSON Schema dialect, applied to the parsed TOML object)

Key fields, with cross-check surfaces marked. Lint-surface fields are the ones that can be compared to on-ledger evidence without human judgment.

- `declaration_id`: URI, unique across the deployment corpus.
- `operator_id`: URI; **cross-check** — MUST match `PostureDeclaration.operator_id` on the referenced posture document.
- `posture_declaration_ref`: URI; **cross-check** — the referenced posture document MUST have `delegated_compute = true` (Core §20.2). A `false` there with a Companion §19 declaration present is a posture-honesty violation.
- `scope.authorized_actions`: enum set; **cross-check** — if `commit_on_behalf_of` or `decide` is absent, no canonical fact in the ledger under this grant may carry `actor_agent_under_delegation` as the sole actor for a decision-contributory event.
- `scope.max_agents_per_case` / `max_invocations_per_case`: uint; **cross-check** — declared ceiling vs observed maximum across the ledger window.
- `scope.time_bound` / `purpose_bound`: required non-empty unless `open_ended_permitted = true` (OC-65).
- `authority.grantor_role_tier`: opaque URI; **cross-check** — MUST resolve in the declared WOS role registry.
- `authority.wos_autonomy_cap_ref`: URI; **cross-check** — MUST resolve and MUST NOT be in revoked state at `effective_from`.
- `audit.grant_event_type` / `invocation_event_type` / `output_event_type` / `revocation_event_type`: event-type strings; **cross-check** — each MUST be in the Operator's published event-type registry; each MUST appear in the ledger within the declaration's temporal scope (absence of the revocation type is acceptable only while the grant is active).
- `audit.attribution_fields_emitted`: string set; **cross-check** — every emitted event of the declared types MUST carry all listed fields (Companion §19.4 OC-67).
- `attribution.actor_discriminator_rule`: fixed string `exactly_one_of(actor_human, actor_agent_under_delegation)`; **cross-check** — no event may set both; no decision-contributory event may set neither. Core invariant #15 bites here.
- `supply_chain.runtime_enclave`: enum `{ provider_operated, tenant_operated, client_side, isolated_enclave }`; **cross-check** — MUST match the `delegated_compute_exposure` value for the relevant `content_class` row in the posture document's `AccessTaxonomyTable` (Appendix A.2).
- `signature`: COSE-declared operator signature over a dCBOR preimage of the normalized frontmatter. Verifying this ties the declaration to the operator key registry the same way envelopes are tied, and closes the "who signed off on this claim" loop.

Body-side (Markdown) fields are operator narrative: governance rationale, human-readable scope description, supply-chain caveats, review history. Not lint-gated; read by humans during audit.

## Worked example

**Deployment scenario:** LLM-assisted first-pass triage for SSDI (Social Security Disability Insurance) intake — the agent reads the respondent's intake narrative plus declared PII, drafts a triage recommendation (complete / needs-follow-up / out-of-scope), and writes the draft as an advisory canonical fact. A human adjudicator reads the recommendation, either accepts it (committing their own decision fact that cites the agent's draft as input) or rejects it (committing a contradicting decision fact). The agent never commits a decision-contributory fact on its own; `scope.authorized_actions = ["read", "propose"]` and `scope.decision_class = "advisory"`.

Filled-out declaration (excerpt; full frontmatter mirrors the skeleton above with values populated):

```toml
declaration_id   = "urn:trellis:declaration:ssdi-intake-triage:2026-04-18"
deployment_id    = "ssdi-intake-triage-v1"
operator_id      = "urn:gov:example-state:disability-determination-services"
effective_from   = "2026-04-18T00:00:00Z"
posture_declaration_ref = "urn:trellis:posture:ssdi:2026-04-01"

[scope]
authorized_actions       = ["read", "propose"]
content_classes          = ["respondent_pii", "intake_narrative"]
fact_families            = ["intake-fact", "triage-recommendation"]
decision_class           = "advisory"
max_agents_per_case      = 1
max_invocations_per_case = 8
time_bound               = "2027-04-18T00:00:00Z"
purpose_bound            = "first-pass triage recommendation for adjudicator review"
open_ended_permitted     = false

[authority]
grantor_principal    = "urn:gov:example-state:dds:role:intake-lead"
grantor_role_tier    = "WOS:tier:operator-adjudicator"
policy_authority     = "urn:gov:example-state:dds:policy:ai-triage-2026-01"
wos_autonomy_cap_ref = "urn:gov:example-state:dds:wos:autonomy-cap:triage-tier-2"

[audit]
grant_event_type           = "trellis.grant.delegated-compute.v1"
invocation_event_type      = "trellis.compute.invocation.v1"
output_event_type          = "trellis.compute.output.v1"
revocation_event_type      = "trellis.revocation.delegated-compute.v1"
attribution_fields_emitted = [
  "grantor_principal", "grantee_agent_id", "grant_ref", "scope_ref",
  "human_reviewer_principal", "decision_class",
]

[attribution]
agent_identity          = "urn:vendor:example-ai:triage-model:v4.2"
agent_provenance_ref    = "urn:vendor:example-ai:model-card:triage-v4.2"
actor_discriminator_rule = "exactly_one_of(actor_human, actor_agent_under_delegation)"

[supply_chain]
runtime_enclave    = "tenant_operated"
model_identifier   = "example-ai/triage:v4.2"
model_version_hash = "sha256:7f3e..."
```

The Markdown body then carries: (1) a plain-English summary of what the agent can and cannot do; (2) the delegation chain from commissioner → intake-lead as narrative; (3) the explicit statement that every triage recommendation is advisory and that adjudicators hold the commit authority for the decision fact; (4) the supply-chain caveats — model version, vendor, update cadence — per OC-70.

## Lint rule surface (follow-on, not implemented here)

A declaration-doc validator — either a new `scripts/check-declarations.py` or a declaration-aware pass inside `scripts/check-specs.py`, TBD — should enforce, at minimum:

1. **File-exists-at-declared-path.** The file containing the declaration resolves to `declaration_uri`.
2. **Schema-validates.** The TOML frontmatter parses and conforms to the declared schema; unknown top-level tables fail loud (mirrors the F1 "fail on unknown shape" discipline from the G-3 fixture-system design).
3. **Posture-cross-check.** The document referenced by `posture_declaration_ref` exists and its `PostureDeclaration.delegated_compute == true`. A declaration with a posture reference that declares `delegated_compute = false` is a Core §20 / invariant #15 violation.
4. **Operator-match.** `operator_id` in the declaration matches `PostureDeclaration.operator_id` on the referenced posture document.
5. **Grantor-resolves.** Every `authority_grantor` and `delegation_chain` URI resolves to a known WOS role in the Operator's declared WOS role registry.
6. **WOS-cap-active.** `authority.wos_autonomy_cap_ref` resolves and is not revoked at `effective_from`.
7. **Audit-event-types-registered.** Every `audit.*_event_type` value appears in the Operator's event-type registry.
8. **Audit-event-types-emitted.** Every declared event type appears at least once in the ledger within the declaration's temporal scope (grant-type at start; invocation/output as observed; revocation permitted to be absent while active).
9. **Attribution-fields-present.** Every emitted event of the declared types carries every field listed in `audit.attribution_fields_emitted`.
10. **Actor-discriminator.** No event under the grant sets both `actor_human` and `actor_agent_under_delegation`; no decision-contributory event sets neither.
11. **Scope-ceilings-honored.** Observed `max_agents_per_case` and `max_invocations_per_case` across the ledger window do not exceed declared values.
12. **Scope-actions-honored.** If `commit_on_behalf_of` or `decide` is absent from `scope.authorized_actions`, no decision-contributory event under the grant has `actor_agent_under_delegation` as sole actor.
13. **Time-bound-honored.** No invocation event under the grant has a timestamp outside `[effective_from, time_bound]` unless `open_ended_permitted = true`.
14. **Supply-chain-matches-taxonomy.** `supply_chain.runtime_enclave` matches the `delegated_compute_exposure` on the matching Appendix A.2 row.
15. **Operator-signature-verifies.** `signature` verifies against the Operator's key registry as of `effective_from`.

Checks 1–2 are static and always runnable. Checks 3–4 and 14 need read access to the posture-declaration registry but not to the ledger. Checks 5–6 need the WOS role / autonomy-cap registry. Checks 7–13 need read access to the ledger. Check 15 needs the operator key registry. The tiers matter: 1–6 and 14–15 are cheap CI checks; 7–13 are ledger-replay checks that run in an operator-run conformance suite. For Phase 1 ratification, checks 1–6 and 14–15 are the minimum-viable lint surface; ledger-replay checks can land with the Rust conformance crate.

## How O-4 closes

Companion §19 requires "a declaration document per agent-in-the-loop deployment." "Per deployment" means **per distinct (agent-identity, deployment-site, release) tuple**: if the same model version runs in two jurisdictions under two different grantors, that is two deployments and two declaration documents; if a new model version supersedes an old one at the same site, that is a new declaration with `supersedes` set to the old `declaration_id`. Versioning via `supersedes` mirrors the pattern already used for Posture Declarations (Appendix A.1), which is the right precedent — same-site model upgrades are an ordinary forward event, not a backfill of prior audit records.

**Minimum-viable for Phase 1 ratification (O-4 close):**

1. This design brief, reviewed and accepted.
2. One reference declaration document exists at `deployments/ssdi-intake-triage-v1/declaration.md` (the worked example above, filled out end-to-end).
3. Lint rules 1–6 and 14–15 pass on that document. Ledger-replay checks (rules 7–13) are listed as follow-ons and run when the Rust conformance crate lands.
4. A Core-§20 / Companion-§11 / Companion-§19 coherence check is documented: the reference posture document that `posture_declaration_ref` points to sets `delegated_compute = true`, and the two documents agree on `operator_id` and content classes.

That is one reference artifact plus a structural lint pass — comparable in shape to what `append/001-minimal-inline-payload` did for G-3. G-2 picks up the declaration-doc-check audit channel as a non-byte-testable path once this template lands (`thoughts/specs/…` Stream A design will cite this brief for invariants routed through delegated-compute honesty).

## Follow-ons

- **Author reference declaration document** — `S`. Populate the SSDI-intake-triage scenario end-to-end at `deployments/ssdi-intake-triage-v1/declaration.md`. Blocked only by this brief's acceptance.
- **Declaration-doc lint rules 1–6, 14–15 in `scripts/check-specs.py` (or a new `check-declarations.py`)** — `S`. Static checks only; no ledger dependency. Parallel with reference-doc authoring.
- **Ledger-replay lint rules 7–13 in the Rust conformance crate** — `M`. Blocked by G-4 (`trellis-conformance`). Not required for Phase 1 O-4 close but required for full §19 enforcement.
- **Integrate declaration-doc-check path into G-2 audit channel assignment** — `XS`. One row in the invariant audit-path table (Stream A design brief).
- **Appendix B.4 alignment** — `XS`. Add a Companion note pointing B.4 (`DelegatedComputeGrant` minimal shape) at this per-deployment declaration document as its canonical expansion, so future readers of the Companion find the connection.

## Ambiguity / review-before-citing

- **Appendix A status.** Appendix A already contains a substantive `PostureDeclaration` template (§11 sibling). This brief does not touch Appendix A — it introduces a *new* declaration-document type adjacent to A and cross-linked to A via `posture_declaration_ref`. If Working Group preference is instead to expand Appendix A with a new sub-appendix (e.g., A.6 `DelegatedComputeDeclaration`), the frontmatter schema above transplants directly; only the housing changes. Flagging for review before the reference declaration is written.
- **Event-type registry ownership.** Lint check 7 assumes an "Operator's event-type registry" exists. Core §6 defines the event format but does not pin an event-type-string registry shape. The Companion cites event families (grant, invocation, revocation) in §19 and §25 without naming a registry document. A downstream brief — probably the O-5 posture-transition-event design — will need to pin the registry shape; this template presumes its existence but does not define it. Flagging so that downstream authors align rather than diverge.
- **"Decide" as authorized-action.** The `scope.authorized_actions` enum includes `decide` as a possibility for completeness, but Companion §19 and Core §20 together strongly imply that decision-contributory acts by an agent *alone* are never Phase-1-conformant — a decide-capable deployment would at minimum need to set `decision_class = "decision_contributory"`, which is a posture that Core §20.3 says requires `delegated_compute = true` and careful §11.2 acknowledgement. Keeping `decide` in the enum but flagging for Working Group: if decide-alone is categorically out-of-scope for Phase 1, the enum should drop it and the schema should forbid it. Preferred resolution before reference-declaration authoring.
