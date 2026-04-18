Below is a build-stack comparison matrix for the categories we’ve actually been discussing, not the entire 50+ artifact research corpus. The research keeps converging on the same architectural move: separate durable orchestration, policy/decisioning, authz, integration/eventing, provenance/audit, and agent/tool interop into distinct layers instead of asking one tool to cosplay as all of them.  ￼  ￼

1) Durable orchestration / long-running execution

The core requirement here is durable state, timers, retries, callbacks, replay or equivalent recovery semantics, and safe handling of nondeterministic agent/tool steps. The research is especially clear that agent calls should be treated as recorded external step results, not inline orchestration logic.  ￼  ￼  ￼

Tool Best role Strengths Weak spots Best fit Avoid using it as
Temporal Primary orchestration runtime Durable workflows, replay via event history, strong timer/retry/callback model, good for long-running casework Determinism/replay discipline is real engineering overhead Core execution engine for consequential workflows Decision engine, authz system, collaborative editor
Azure Durable Functions Similar durable orchestration model Strong event-sourced/replay semantics; good reference model for conformance thinking Azure-shaped operational footprint; less neutral if you want portable kernel semantics Azure-heavy shops or as a semantic comparison point Your portable workflow standard
AWS Step Functions / ASL Managed orchestration Clear state-machine vocabulary, callback token pattern, simple cloud-native operations Vendor lock-in; weaker human-task/case semantics AWS-native product surface or bounded integration flows Your cross-vendor semantic core
Inngest Simpler step-memoized workflow runtime Easier developer experience; avoids some replay pain by memoizing step results Not the same formal replay model as Temporal; less aligned with “audit by event history” as the kernel Less critical workflows or developer-friendly async jobs The backbone for regulated long-running case orchestration

My default: Temporal. It best matches the research thesis around durable execution, replay-safe orchestration, timers, callbacks, compensation, and explicit handling of nondeterministic agent steps.  ￼  ￼  ￼  ￼

1) Collaborative authoring / definition editing

This category is a little different: these are not perfect substitutes. The research points toward AI-native structured editing with typed patches, canonicalization, diagnostics, and multiplayer collaboration. Automerge is strongest for collaborative/offline editing; JSON Patch/JCS/LSP are the substrate and tooling conventions around it.  ￼  ￼  ￼

Tool Best role Strengths Weak spots Best fit Avoid using it as
Automerge Multiplayer/offline editing substrate Local-first, conflict-free collaboration, offline editing, document history Extra metadata/history cost; not a workflow runtime Collaborative authoring of workflow definitions, notes, annotations, draft policies Runtime source of truth for execution
JSON Schema + JSON Patch/JCS Canonical machine-edit substrate Validation, stable diffs/hashes, typed edits, reproducibility No collaboration model by itself Underlying definition format and patch protocol Human collaboration layer by itself
LSP-style tooling Diagnostics/refactors/editor UX Great for semantic errors, refactors, code actions, AI-assisted editing loops Not storage or sync Editor/workbench layer on top of schemas/docs Source of truth or runtime

My default: Automerge + JSON Schema/Patch/JCS + LSP-style tooling. That is a stack, not a winner-take-all choice. Automerge gives you collaboration; the JSON tooling gives you determinism and validation.  ￼  ￼  ￼

1) Fine-grained authorization

The research strongly separates authorization from workflow logic and treats Zanzibar-style relationship-based auth as the durable pattern for case/task/evidence permissions. OpenFGA and SpiceDB are filling essentially the same architectural slot.  ￼  ￼  ￼

Tool Best role Strengths Weak spots Best fit Avoid using it as
OpenFGA Relationship-based auth service Zanzibar-inspired, developer-friendly model language, supports ReBAC and can help with RBAC/ABAC patterns Still “just auth”; you must keep workflow and policy semantics elsewhere Product teams wanting fine-grained auth with a cleaner modeling experience Workflow engine or business-rule engine
SpiceDB Relationship-based auth database/service Zanzibar-inspired, purpose-built for real-time permissions, infra-style service More infrastructure-shaped; same conceptual limits as any authz engine Security-critical, graph-like permissions at case/task/evidence granularity Decision logic, routing, or eligibility engine

My default: choose one, not both. If you want the cleaner product/dev experience, I’d lean OpenFGA. If you want the more infra-flavored Zanzibar service posture, I’d lean SpiceDB. Conceptually, they solve the same problem.  ￼

1) Policy / decision layer

This is where people make a mess by shoving everything into one engine. The research argues for separating decision services, policy evaluation, and date-effective public-program logic. OpenFisca is weirdly important here because it handles temporal parameter versioning, which most workflow stacks conveniently pretend does not exist.  ￼  ￼  ￼  ￼

Tool Best role Strengths Weak spots Best fit Avoid using it as
OpenFisca Date-effective program rules / benefits logic Temporal parameters, reforms, microsimulation mindset, good for statutory thresholds/rates Not a general workflow engine or generic authz engine Benefits, taxes, entitlements, regulated numeric logic that changes over time Fine-grained app authorization
OPA / Rego General policy evaluation Policy-as-code, external data, testing, decoupled evaluation/enforcement Rego is developer-centric; explanation portability is weaker than you may want Routing, guardrails, environment/context-aware decisions, enforcement policy Human-readable policy authoring for nontechnical domain experts
Cedar Analyzable authorization policy PARC model, schema validation, strong analyzability for auth-like decisions Narrower focus; authorization-oriented rather than full workflow policy Safety-critical permissioning and analyzable policy checks Full statutory rules engine
DMN-style tables Business-readable decisions Good for tabular decisions and traceable hit policies Needs runtime/integration around it; not great for full authz or date-versioned legislation by itself Eligibility tables, routing tables, approval thresholds Runtime orchestration or auth graph
Catala / Legal-rule style tools Law-heavy, exception-rich rules Strong for “general rule + exception” reasoning and legislative traceability Niche ecosystem, more specialized Highly legalistic public-sector domains Broad application policy platform

My default split:
 • OpenFisca for date-effective public-program rules
 • OPA for general policy evaluation and workflow guardrails
 • Cedar if you need tighter analyzable auth-like policy than OPA
 • DMN-style tables for business-readable decisions above those engines

That is slightly more complex, but it stops one engine from becoming a haunted basement full of unrelated rules.  ￼  ￼  ￼

1) Integration / eventing / interface contracts

The research lands on a clean split here: CloudEvents for envelopes, OpenAPI for sync contracts, AsyncAPI for async contracts, Arazzo for reusable API-call sequences, and webhook conventions for callbacks. None of these is a workflow engine; they are the plumbing that keeps your workflow engine from inventing its own dialect of sadness.  ￼  ￼

Tool Best role Strengths Weak spots Best fit Avoid using it as
CloudEvents Event envelope Standard metadata, cross-platform event portability, clean lifecycle/event wrapper No domain semantics by itself Workflow lifecycle events, callbacks, external signals Business workflow semantics
OpenAPI Sync API contract Typed interfaces, strong tooling, good contract surface for tools/services Not for async/event streams Synchronous service/tool contracts Event bus or workflow model
AsyncAPI Async contract Channel/message schemas, event-driven integration contracts Not a lifecycle/orchestration model Event-driven integrations Decision or orchestration logic
Arazzo API sequence / integration subflow Standardizes call sequences/dependencies Narrow scope; not full workflow Reusable integration subgraphs Whole workflow/case model
Standard Webhooks Callback delivery pattern Signatures, idempotency, delivery semantics Narrow but essential External callbacks into long-running workflows General event model

My default: CloudEvents + OpenAPI + AsyncAPI, with Arazzo when you want reusable integration subflows and webhook conventions for callback edges.  ￼  ￼

1) Provenance / audit / observability

This category needs a hard separation: audit/provenance is not the same thing as observability. The research is emphatic here. PROV gives you the conceptual model for “what happened and why”; OpenTelemetry gives you operational traces; XES/OCEL are export/interchange formats for mining and conformance; tamper-evident logs are optional if your domain needs stronger non-repudiation.  ￼  ￼  ￼

Tool Best role Strengths Weak spots Best fit Avoid using it as
W3C PROV Provenance model Entity-Activity-Agent backbone, good conceptual fit for “who/what/why” Needs a workflow-specific profile to be practical Portable audit/provenance model Runtime tracing system
OpenTelemetry Observability Traces, spans, context propagation, good ops tooling ecosystem Does not answer the full audit/accountability question Runtime ops/debugging and trace correlation Regulated audit record by itself
XES / OCEL Audit log interchange / process mining export Useful for conformance and mining; OCEL handles multi-object histories better Needs stronger semantics around evidence/authority/policy Analytics export from your audit log Core source-of-truth audit model
Merkle / CT-style tamper evidence Optional audit integrity profile Strong append-only integrity story Extra operational complexity High-assurance domains needing proof of non-tampering Default day-one requirement for every deployment

My default: PROV-shaped audit model + OpenTelemetry + optional XES/OCEL export. Add tamper-evident logging only if your regulatory context actually justifies the pain.  ￼  ￼  ￼

1) Agent / tool interoperability boundary

This category is the easiest to misuse. MCP and A2A are transport/interoperability layers, not governance. The research says that very plainly: no single agent protocol gives you authority modes, propose/commit, override semantics, or idempotent governance by itself.  ￼  ￼  ￼

Tool Best role Strengths Weak spots Best fit Avoid using it as
MCP Model-to-tools/data boundary Standard tool exposure, lifecycle, capability negotiation, typed tool schemas Security/governance still needs extra layers; not a workflow kernel Connecting agents/models to tools, data, prompts, workflows Governance model for consequential actions
A2A Agent-to-agent collaboration Agent discovery, task lifecycle, async/streaming, long-running collaboration Still not the governance layer; more moving parts Cross-agent collaboration across subsystems/orgs Substitute for orchestration + policy + audit
Provider tool/function calling In-model tool invocation contract Practical, widely supported, JSON-schema shaped Provider-specific and not cross-vendor governance Single-model tool invocation loops Standard kernel for interoperability
WoT Thing Description concepts Capability-descriptor inspiration Nice affordance model beyond HTTP-only APIs Not the thing you should force into core Designing richer tool descriptors Workflow semantics

My default: MCP now, A2A later if you truly need multi-agent collaboration. Provider tool calling is a profile at the edge, not your semantic center of gravity.  ￼  ￼

1) Core persistence / storage

This is not glamorous, but you need it.

Tool Best role Strengths Weak spots Best fit Avoid using it as
Postgres Operational source of truth for metadata/indexes Reliable, transactional, flexible enough for cases/tasks/indexes Not ideal for large evidence blobs Case/task metadata, policy/version refs, audit indexes Blob store
Object storage Evidence and large artifacts Cheap, durable, good for claim-check pattern and content-addressed blobs Not a relational query layer Documents, images, attachments, receipts Primary workflow state engine

The research keeps pointing toward claim-check patterns, typed case files, and evidence integrity rather than shoving giant artifacts into the orchestration layer. So in practice you want both: Postgres for operational truth and object storage for evidence blobs.  ￼  ￼

My practical default stack

If I had to freeze the stack today, I would use this:

Category Default choice Why
Durable orchestration Temporal Best fit for long-running, auditable, replay-safe workflows
Collaborative authoring Automerge + JSON Schema/Patch/JCS Multiplayer editing plus machine-valid structure
Fine-grained authz OpenFGA or SpiceDB Zanzibar-style relationship auth at case/task/evidence level
Decisioning OpenFisca + OPA Date-effective public-program logic plus general policy evaluation
Optional analyzable auth policy Cedar Stronger auth-like analyzability when needed
Eventing/contracts CloudEvents + OpenAPI + AsyncAPI Clean sync/async integration surfaces
Provenance/audit PROV-shaped audit model “Who/what/why” instead of just logs
Observability OpenTelemetry Runtime traces and ops visibility
Agent boundary MCP Cleanest tool/data interoperability layer today
Multi-agent interop A2A later Only when real cross-agent collaboration justifies it
Operational store Postgres + object storage Metadata/indexes plus evidence/artifact storage

That combination best matches the layered architecture the research keeps converging on: Temporal for execution, FGA for permissions, OpenFisca/OPA for rules, CloudEvents/OpenAPI for interop, PROV/OTel for accountability and operations, and Automerge for collaborative authoring.  ￼  ￼  ￼

If you want, I can turn this into a single-sheet decision table with columns like “day-1 pick / day-2 option / avoid / why / migration path.”
