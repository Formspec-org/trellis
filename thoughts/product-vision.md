# Formspec + WOS + Trellis — Product Vision & Roadmap

**Date:** 2026-04-17 (status updates 2026-04-18)
**Status:** Synthesized vision with sequenced delivery plan. Track A steps 1–5 landed; step 6 (test vectors) is in progress — see status table and Track A section below.

---

## The vision, in one sentence

**The open-spec platform for high-stakes public intake and adjudication — workflows where a government decision affects someone's rights, benefits, or safety — from the applicant's first keystroke to an air-gapped-verifiable appellate record.**

---

## The product

Three tiers, specified independently, composed as one artifact.

| Tier | Role | Owns |
|---|---|---|
| **Formspec** | Intake instrument | Adaptive, AI-native, document-aware data capture. Definition, Response, FEL, validation, relevance, calculation, respondent ledger, accessibility, locale, mapping. |
| **WOS** | Governance envelope | Deontic rules, due process, structured oversight, AI autonomy caps, authority-ranked reasoning, four-tier provenance, impact-level-dependent behavior. |
| **Trellis** | Integrity substrate | Cryptographic envelope, hash chain, signed checkpoints, offline-verifiable export package. What survives the system, the vendor, and the years. |

**The coherent claim.** Every record produced by the platform is simultaneously a valid Formspec response, a governed WOS event, and an attested Trellis entry. The three specs already point at each other's seams — the Respondent Ledger add-on (§13) explicitly defers signing and anchoring to a downstream layer, and the WOS Kernel (§10.5) names `custodyHook` as the seam for the same concern. Trellis is the concrete answer to two already-written deferrals, not a fourth layer invented above them.

---

## Who it's for

**Primary — federal and state public-sector intake.** Benefits adjudication (SSDI, SNAP, Medicaid, TANF, unemployment); grants (Grants.gov, pass-through, disaster recovery); licensing and permitting; compliance and regulatory filings; casework and investigations.

**Secondary — regulated enterprise with the same shape.** Healthcare intake and prior authorization; financial compliance and KYC/AML; education administration (FAFSA, Title IV); insurance claims with adjudication.

**Adjacent via the Sovereign variant.** Privacy-forward markets (EU eIDAS 2.0 wallet, HHS patient-held records); civil-liberties-sensitive workflows (asylum, journalism-oriented tools).

---

## Why it wins

Component prior art exists for every pillar. The moat is that nobody has specified them *together* as a machine-authorable, machine-validatable, JSON-native, multi-runtime stack.

- **Against Adobe AEM Forms.** Adobe built forms for the document era; we build for the decisioning era. Open JSON spec with TS/Rust/Python runtimes vs. proprietary XDP/XFA on Java. AI-native authoring vs. bolted-on Sensei. Cryptographic audit vs. application-level logs.
- **Against ServiceNow.** ServiceNow's catalog items are static field lists for internal requestors; our intake is adaptive, AI-native, document-aware, public-facing. Their workflow is proprietary lock-in; WOS is open governance with ServiceNow as one possible execution substrate.
- **Against DocuSign.** DocuSign is the last step of agreement. We are the full lifecycle of adjudication, with legal-grade evidence included rather than bolted on.

---

## How we build — the operating model

Three disciplines, running together.

### 1. Agreement-first sequencing

Every downstream artifact is strictly derived from the one above it. Parallel production of any two stages guarantees drift.

```
Agreement   (≤5 pages, signed off by product strategy)
    ↓ drives
Spec        (W3C-style prose, prompt-guidance surface)
    ↓ drives
Schema      (JSON Schema + CDDL for byte-level shapes)
    ↓ drives
Test vectors + lint rules  (machine-validation surface)
    ↓ drives
Runtime      (reference implementation)
    ↓ drives
Reference tool  (CLI, SDKs, second implementations)
    ↓ iteration loops back to
Spec edit    (only after interop failure or semantic ambiguity)
```

**Rule:** No schema edit without a spec edit. No spec edit without an agreement edit. No code change that doesn't trace back to a vector change traceable to a spec section.

### 2. Specs as prompt guidance

The spec is the primary authoring surface for humans and LLMs. It is W3C-style prose — self-contained within its concern, conceptually ordered, richly exampled, normatively precise (RFC 2119), composition-aware about adjacent systems. Schemas carry `x-lm.critical` + `intent` + `examples` annotations so LLMs can generate conformant documents directly from schema.

A well-structured spec answers integrator questions without requiring five companion documents. One well-sectioned document per concern, not a constitutional taxonomy of subordinate companions.

### 3. The closed loop, center-of-gravity adjusted per spec

Each spec runs the full loop, but the artifact weight differs by the kind of thing being specified.

| Spec | Kind | Center of gravity |
|---|---|---|
| Formspec | semantic | many constraints; weight in schema + lint + conformance fixtures |
| WOS | semantic | many constraints (197); weight in tiered lint matrix + conformance fixtures |
| Trellis | byte-level protocol | few constraints, byte-exact; weight in **test vectors** + cross-implementation interop |

Treating all three specs with the same lint-heavy discipline overbuilds Trellis. Treating Trellis with an RFC-terse discipline underbuilds its integrator-facing surface. Each spec gets the same loop, weighted correctly.

---

## Current state (honest assessment)

| Spec | Agreement | Spec | Schema | Lint | Conformance | Runtime | Reference tool |
|---|---|---|---|---|---|---|---|
| **Formspec** | clear | mature (Core, Respondent Ledger, Mapping, References, Ontology, Locale, Screener, Assist, Theme, Component) | mature | present | present | TS engine shipped, Rust + Python reference | webcomponent, studio (partial), MCP server |
| **WOS** | clear (`POSITIONING.md`) | 19 specs, ~6.7k lines | 19 schemas | 197-rule matrix (T1/T2/T3) | 105 fixtures | `wos-core` + `wos-runtime` in active development | `wos-cli` pending, `wos-formspec-binding` shipped |
| **Trellis** | reached (`specs/trellis-agreement.md`) | two normative W3C-style specs (`trellis-core.md` + `trellis-operational-companion.md`, ~30k words total) plus requirements matrix (79 TR-CORE + 49 TR-OP) | CDDL inline in Core Appendix A | `scripts/check-specs.py` with 4 coverage rules + 7-test harness | fixture system + 1 byte-exact reference vector (`append/001-minimal-inline-payload`); ~49 more vectors pending | open (G-4) | open (G-4 CLI + WASM) |

The diagnostic has moved. **All three specs are running the loop.** Trellis entered the loop mid-April 2026: the eight-spec family converged to two normative docs; Phase 1 envelope invariants #1–#15 landed in Core; a fixture system design + coverage lint + first byte-exact reference vector are committed; the ratification bar ("reproducible from Core prose alone") is operationally testable and has already driven Core amendments (COSE header labels, named event surfaces, AppendHead struct). Remaining Track A work is ~49 more vectors, the Rust reference implementation, and the stranger-test second implementation.

---

## Delivery arc

Five Trellis architectural visions → four sequenced product phases, each a strict superset of the prior.

**Terminology — ledger vs log.** This stack uses three nested scopes of append-only structure; the phase descriptions and invariants below rely on these distinctions.

- **Event** — an atomic append (one field edit, one governance action, one signature).
- **Response ledger** — a hash-chained sequence of events for one Formspec response. Scoped to a single respondent session; sealed at submission.
- **Case ledger** — a hash-chained sequence of governance events for one case, composing one or more sealed response-ledger heads with WOS governance events. Scoped to a single adjudicatory matter. This is the "portable case file" of Phase 3.
- **Agency log** — an append-only log of case-ledger heads (plus metadata and witness timestamps) maintained by an operator. Proves that a case existed at time T and was not quietly deleted. CT-style log-of-cases; structurally what Trillian builds.
- **Federation log** — a log of agency-log heads witnessed by an independent operator (Phase 4). Detects cross-operator equivocation.

"Ledger" is always qualified by scope (response ledger, case ledger). "Log" is reserved for structures whose entries are other ledgers' heads. All five are Trellis-shaped — same envelope format, same hash construction, same signing profile — applied at different scopes.

### Phase 1 — **Attested exports** (Trellis Vision 4: export-only)

Trellis is a signed export bundle format. Runtimes use whatever storage they like (Postgres default). At submission, sealing, case close, or FOIA boundary, the runtime serializes a COSE-signed bundle with hash chain, inclusion proofs, and provenance distinctions intact. Auditors, IGs, journalists, opposing counsel verify offline via `trellis verify`.

**Unlocks.** The "cryptographic audit infrastructure" moat the enterprise-feature-gaps doc names. FOIA and litigation production get an evidence-grade answer. Not on the critical path for first gov sale (leaves FedRAMP engineering capacity for reviewer dashboard, document storage).

**Shipping shape.** ~60–90 pages of Trellis spec, ~50 test vectors, ~3–5k lines of Rust across a small crate workspace, one CLI binary. Weeks–months, not quarters.

**Trust posture, stated honestly.** Trellis does not claim system-owner-proof tamper resistance in Phase 1. It claims: tampering requires the operator to replace signed export bundles already in third-party hands, and any reissue is detectable by checkpoint divergence against prior recipients. Transparency witnessing (Phase 4) raises this bar to equivocation-proof; Phase 1 sets it at "difficult and obvious," which is the actual product requirement.

### Phase 1 envelope invariants (non-negotiable)

Phase 1 ships a small spec. That smallness is a discipline, not an excuse to defer architecture. The following decisions MUST land in the first envelope format and its manifest, because every one of them is cheap to include now and requires a wire-format break to retrofit. If any cannot be committed to, the agreement document is not ready to sign.

1. **Canonical CBOR profile is pinned.** Trellis specifies one deterministic encoding (dCBOR or an explicitly named equivalent) in §"Canonical encoding." Byte-exact test vectors are meaningless without this. No "pre-implementation spike" wording.

2. **Signature suite is identified, not assumed.** Every signed artifact carries a `suite_id`. The spec names the Phase 1 suite (e.g. Ed25519/COSE_Sign1), reserves `suite_id` space for hybrid and post-quantum suites (ML-DSA, SLH-DSA), and states the migration obligation — a 2045 verifier MUST be able to resolve a 2026 signature after key and suite rotations. Phase 1 does not ship a PQ suite; it reserves the seam and names the obligation.

3. **Signing-key registry is part of the export.** A COSE signature without a resolvable key is unverifiable after rotation. Exports include a signing-key registry snapshot (`SigningKeyEntry`, Active/Revoked lifecycle) so verification is self-contained at any future date.

4. **Hashes are over ciphertext, not plaintext.** Payloads are encrypted before hashing so that per-subject key destruction ("crypto-shredding") is possible without invalidating the chain. This is the only GDPR Art. 17 / FOIA-redaction story that survives an append-only chain; hashing plaintext forecloses it permanently.

5. **Ordering model is named.** The spec states whether `prev_hash` denotes strict linear sequence or a causal DAG (HLC + explicit dependencies). Phase 2 runtime-time integrity across concurrent devices cannot add causal ordering later without a header-version break. If linear-only is chosen in Phase 1, the header reserves the causal-dependency field.

6. **Registry-snapshot binding in the manifest.** The export manifest includes a content-addressed digest of the domain registry (event taxonomy, role vocabulary, governance rules) in force at the time of signing. Signature verification without this proves byte integrity only — not semantic verification. A 2045 verifier needs to know *what the fields meant* in 2026.

7. **`key_bag` / author-event-hash is immutable under rotation.** Any key re-wrap that would change `author_event_hash` is forbidden; re-wrapping produces an append-only `LedgerServiceWrapEntry`. Historical hashes MUST reproduce after a Long-lived Authority Key rotation, or the chain breaks across its own lifecycle.

8. **Redaction-aware commitment slots are reserved.** The envelope header has field positions for per-field commitments (Pedersen, Merkle leaves, or equivalent). BBS+ / selective-disclosure *implementation* is deferred; the *slot* is not. Without this, Phase 3 portable case files force either all-or-nothing disclosure or envelope reissue.

9. **Plaintext-vs-committed header policy is explicit.** The spec lists which header fields are plaintext (routing, audit classification) and which are commitments to encrypted or private values (determination outcomes, subject metadata). Header-tag leakage of HIPAA- or adjudication-sensitive values is a spec decision, not an implementation choice.

10. **Phase 1 envelope IS the Phase 3 case-ledger event format.** The continuity commitment is stated normatively: the byte shape produced by Phase 1 export is the byte shape of a Phase 3 case-ledger event. Phase 2 and 3 are strict supersets — they add runtime attestation and case-scoped composition; they do not redefine the event. Without this commitment, the "strict superset" claim of the phase arc is false.

11. **"Profile" is not overloaded across three namespaces.** Three prior-draft namespaces currently use the letters A–E/F: the Respondent Ledger spec's Profile A/B/C (privacy × identity × integrity-anchoring posture); the legacy core draft's seven profiles (Core/Offline/Reader-Held/Delegated-Compute/Disclosure/User-Held/Respondent-History); the legacy companion draft's Profiles A–E (provider-readable/reader-held/delegated/threshold/organizational trust-custody models). These are three orthogonal concerns and must not share a namespace. The Respondent Ledger spec unambiguously owns **"Profile A/B/C"** for posture axes. The legacy core draft's profiles are renamed **"Conformance Classes"** (what they semantically are). The legacy companion draft's profiles are renamed **"Custody Models."** The product-vision refers to Trellis capability tiers by phase name ("attested-export tier," "runtime-integrity tier," "portable-case tier"). The case ledger is defined normatively as a composition of sealed response-ledger heads plus governance events; the agency log is defined normatively as the log-of-case-ledger-heads; these are three distinct structures at three distinct scopes, not one term used three ways.

12. **Head formats compose forward; agency log is a Phase 3 superset.** The case-ledger head format in Phase 3 is a strict superset of Phase 1's checkpoint format — same fields, additional fields only (e.g. references to sealed response-ledger heads, case-scope metadata). The agency log is introduced in Phase 3, but its entries are case-ledger heads as produced in Phase 1 plus arrival metadata and optional witness signatures. Without this commitment, agency-log adoption is a wire-format break for every Phase 1 export already in the field.

13. **Append idempotency is part of the wire contract.** Every `append` call carries a stable idempotency key. Retries with the same key and payload MUST return the same canonical record reference; retries with the same key and a different payload MUST be rejected with a defined error. Without this, every operator implements dedup locally, divergence between implementations is guaranteed, and network-retry behavior is underspecified at exactly the boundary where it matters.

14. **Snapshots and watermarks are day-one, not retrofitted.** Every derived artifact (projections, materialized views, indexes) and every agency-log entry carries a watermark `(tree_size, tree_head_hash)` identifying the canonical state it was derived from, plus a rebuild path from the canonical chain. Full-replay-only is not a valid Phase 1 implementation; at case-file scale it is operationally infeasible, and retrofitting snapshots invalidates every derived view already shipped.

15. **Implementations MUST NOT describe trust posture more strongly than behavior supports.** Promoted from prose to normative invariant. If payloads are provider-readable in ordinary operation, the implementation's posture declaration MUST say so; if "tamper-evident" depends on an external anchor or witness, the declaration MUST name the dependency; cryptographic controls alone MUST NOT be described as legal admissibility. This is the spec-level floor underneath the "difficult and obvious" product claim.

### Phase 2 — **Runtime-time integrity** (Trellis Vision 1: shared library)

Trellis becomes a Rust crate both runtimes link against. Every write is attested as it happens, not just at export boundaries. Formspec uses it through the Respondent Ledger extension point; WOS uses it through `custodyHook`.

**Unlocks.** SOC 2 / ISO 27001 enterprise expansion. "Tamper-evident by default" is procurement-language literal.

### Phase 3 — **Portable case files** (Trellis Vision 2: unified canonical ledger)

One case ledger per case, composing sealed response-ledger heads with WOS governance events. A complete case — drafts, submissions, amendments, determinations, appeals — exports as a single verifiable bundle that travels across agencies, across years, across operators. Alongside the case ledger, the operating agency maintains an **agency log** of case-ledger heads; this is what proves that a case existed at time T and was not quietly deleted, and what FOIA/litigation completeness claims actually rest on.

**Unlocks.** Multi-agency federal programs (Medicaid across states, SSDI federal↔state DDS, grants federal↔pass-through). The strategic Adobe/ServiceNow displacement narrative at the product-category level. "A case is a file — hand it over."

**Locked narrative (2026-04-22).** Phase 3 is the **implementation home** for [ADR-0059: unified ledger as canonical case store](../../thoughts/adr/0059-unified-ledger-as-canonical-event-store.md) (one append-only spine per case, encrypt-then-hash, disposable projections). Phase 1–2 are **strict subsets** of that story: Phase 1 establishes the **byte-exact envelope and offline verifier** without populating reserved federation fields ([Phase-1 MVP principles](specs/2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md)). WOS-facing sequencing summary: [`../../wos-spec/thoughts/plans/0059-unified-ledger-as-canonical-event-store.md`](../../wos-spec/thoughts/plans/0059-unified-ledger-as-canonical-event-store.md).

### Phase 4 — **Federation + Sovereign** (Trellis Visions 5 and 3)

Two variants, same core.

- **Transparency witness (Vision 5).** Independent log operator witnesses runtime checkpoints; gossip protocol detects equivocation. Cross-jurisdictional programs get structural integrity — no single operator, not even the operating agency, can rewrite history.
- **Sovereign / client-origin (Vision 3).** The respondent's device holds a cryptographic record of their own session, signed with a key only they control. Sold as a tier (**Formspec Sovereign**) to privacy-forward and civil-liberties markets.

---

## Non-goals

- Not a general BPM engine. Temporal, Camunda, Flowable, Step Functions keep running orchestration.
- Not a generic identity platform. We integrate ID.me, Login.gov, DIDs via provider-neutral adapters. We don't issue identities.
- Not a document-management system. Storage is pluggable; Trellis signs over opaque blob references.
- Not a BI / analytics tool. We emit clean provenance (PROV-O, XES, OCEL 2.0) to downstream tools.
- Not a cost play. We are not "Adobe but cheaper."

---

## Next steps — the concrete plan

Sequenced by what unblocks what, not by what sounds most strategic.

### Track A — Trellis (from zero)

Not on the critical path for first gov sale; runs in parallel with certification/engineering tracks.

Status legend: ✅ done · 🟡 in progress · ⬜ open.

1. ✅ **Write the Trellis agreement document.** Landed as `specs/trellis-agreement.md` — non-normative decision gate naming scope, primitives, seams, and the Phase 1 stranger-test success criterion.
2. ✅ **Sign off.** Agreement accepted as the decision gate for Track A.
3. ✅ **Mine, then archive the prior drafts.** Eight-spec family and DRAFTS mined and relocated: constitutional content folded into Core and Operational Companion, requirements matrices consolidated into `specs/trellis-requirements-matrix.md`, operational content split into the Companion, Track B/D substrate selection moved out of Trellis. Originals live under `specs/archive/` and `thoughts/archive/drafts/` for provenance; not cited as normative.
4. ✅ **Write two W3C-style Trellis specs.** `specs/trellis-core.md` (~16k words — envelope format, dCBOR, hash construction, signature profile with pinned COSE header labels, chain construction, checkpoint format, export package layout, verification algorithm, append idempotency, AppendHead return artifact, security and privacy considerations, composition with Respondent Ledger and `custodyHook`) and `specs/trellis-operational-companion.md` (~14k words — projections and derived-artifact discipline, metadata-budget declarations, delegated-compute honesty, trust-profile transition auditability, snapshot watermarks, rebuild semantics).
5. ✅ **CDDL derived inline in Core Appendix A (§28).** Every normative CDDL type — `EventPayload`, `AuthorEventHashPreimage`, `Event = COSESign1Bytes`, `PayloadInline` / `PayloadExternal`, `AppendHead`, `Checkpoint`, `ExportManifest`, etc. — is consolidated there. Schema follows prose, not the reverse.
6. 🟡 **Author ~50 test vectors** under `fixtures/vectors/{append,verify,export,tamper}/`. Fixture system design + 12-task scaffold plan committed; first byte-exact reference vector (`append/001-minimal-inline-payload`) is live and covers invariants #1/#2/#4/#5. Remaining ~49 vectors authored in follow-on batches — tracked in `TODO.md` and the ratification checklist (G-3). Coverage enforced by `scripts/check-specs.py` lint rules; every byte-level claim resolves to at least one vector once the batch rollout completes.
7. ⬜ **Write the reference implementation.** Rust crates: `trellis-core`, `trellis-cose`, `trellis-store-postgres`, `trellis-store-memory`, `trellis-verify`, `trellis-cli`, `trellis-conformance`. Public API is three functions: `append`, `verify`, `export`. Passes every fixture vector. Byte-matching the first vector alone is a legitimate first milestone; full corpus match closes ratification gate G-4.
8. ⬜ **Ship the CLI + WASM bindings.** `trellis verify | append | export`. WASM for browser-side verification (respondent-facing). Same crate workspace as step 7.
9. ⬜ **Stand up a second implementation** (`trellis-py` or `trellis-go`), written by someone who only reads the specs. Passes every vector byte-for-byte. This is the stranger test — the proof the spec works. Closes ratification gate G-5.

**Phase 1 success criterion:** a stranger writes a second conformant implementation from the spec alone, and every fixture matches byte-for-byte. The infrastructure to test this (vector corpus, coverage lint, deterministic generators) is in place; the corpus is being filled and the stranger implementation has not yet been commissioned.

### Track B — Finish the in-flight specs and runtimes

1. **Complete the WOS runtime.** `wos-runtime` lands all 105 conformance fixtures green. `wos-cli` ships. `wos-export` (PROV-O, XES, OCEL 2.0) stabilizes.
2. **Ship the Formspec Coprocessor handoff** (Runtime Companion §15). This is the binding that lets Formspec responses flow into WOS case instances cleanly.
3. **Iterate Formspec specs** for remaining spec-complete-but-unimplemented items per the enterprise-feature-gaps doc (References, Ontology, Locale, Screener, Assist, Mapping). Continue the existing loop.

### Track C — First-sale blockers (calendar-gated, start clock now)

These are not specs or engineering — they are certifications with 12–18 month timelines. Starting the clock matters more than perfecting any single artifact.

1. **Start FedRAMP Moderate authorization.** Partner with a 3PAO, select an ATO sponsor, begin posture work. The spec suite's mechanisms (data minimization, version pinning, PII tracing, regulatory references) are the foundation of the narrative.
2. **Start SOC 2 Type II.** Requires 6–12 months of operating history, so the clock starts when a production system does.
3. **File for a GSA Schedule.** Without this, agencies cannot easily buy.
4. **Commission a formal WCAG 2.1 AA audit + VPAT.** The Component spec's per-component ARIA mandates, Theme spec's WCAG guidance, and Locale spec's `@accessibility` context suffix are strong substrate; a production audit converts substrate into procurement evidence.

### Track D — First-sale engineering (the actual gaps)

Per enterprise-feature-gaps.md, these are the genuinely-unspecified critical paths that are not closed by WOS or Formspec specs.

1. **Reviewer dashboard.** The UI that consumes WOS governance and displays it to adjudicators. Purely implementation; the data model is spec-complete.
2. **Document storage backend.** File upload exists client-side; no storage service. Blob store + preview + virus scanning + bulk upload.
3. **Webhook infrastructure.** Outbound event delivery — roadmap Phase 1, currently unimplemented.
4. **Notification delivery.** WOS Notification Template sidecar is spec-complete; email/SMS delivery is not built.

### Track E — Cross-cutting bindings

1. **Close the Respondent Ledger ↔ Trellis binding.** Three parts, none of them a one-liner. (a) Promote §6.2 `eventHash`/`priorEventHash` from SHOULD to MUST when a Trellis envelope wraps the event, and define the binding against both the per-event layer (§6.2) and the per-range checkpoint layer (§13) — they are different hashes covering different scopes. (b) Define the **case ledger** as a new top-level object that composes sealed response-ledger heads with WOS governance events; specify the response→case composition rule. (c) Define the **agency log** as the operator-maintained log of case-ledger heads; specify the case→agency-log composition rule and the agency log's head format. This is a spec extension, not a nesting note.
2. **Close the WOS `custodyHook` ↔ Trellis binding.** Document how a WOS runtime uses Trellis as its custody backend without redefining either spec.

---

## The unifying test

**A stranger reads the three spec documents, writes conformant implementations of each in their preferred language, and passes every conformance vector.** When that's true, the platform works. When it's not, one of the stages in the loop is broken — find it, fix it upstream, let the fix propagate down.

No other test matters more than that one. Not ratification checklists, not constitutional hierarchies, not requirements matrices. Just: can a stranger build it, and does it interop.

---

## The single-sentence pitch

**"When a public decision affects someone's rights, the work — from the applicant's first keystroke to the appellate record — should run on open specifications, governed by machine-enforceable due-process rules, and produce evidence that verifies on an air-gapped laptop in 2045."**

Formspec captures it. WOS governs it. Trellis proves it.
