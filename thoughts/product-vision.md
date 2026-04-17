# Formspec + WOS + Trellis — Product Vision & Roadmap

**Date:** 2026-04-17
**Status:** Synthesized vision with sequenced delivery plan

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
| **Trellis** | **not reached** (11 mutually arguing drafts) | 15 fragmented files, 5.6k lines of constitutional prose | none | none | none | none | none |

The diagnostic: **Formspec and WOS are running the loop. Trellis has not entered it.** The Trellis work so far has produced valuable research (ADRs, expert-panel reviews, crypto-solutions memo, risk-reduction analysis) but no convergence — the existing spec files are attempting to codify agreement that doesn't yet exist.

---

## Delivery arc

Five Trellis architectural visions → four sequenced product phases, each a strict superset of the prior.

### Phase 1 — **Attested exports** (Trellis Vision 4: export-only)

Trellis is a signed export bundle format. Runtimes use whatever storage they like (Postgres default). At submission, sealing, case close, or FOIA boundary, the runtime serializes a COSE-signed bundle with hash chain, inclusion proofs, and provenance distinctions intact. Auditors, IGs, journalists, opposing counsel verify offline via `trellis verify`.

**Unlocks.** The "cryptographic audit infrastructure" moat the enterprise-feature-gaps doc names. FOIA and litigation production get an evidence-grade answer. Respondent Ledger Profile A. Not on the critical path for first gov sale (leaves FedRAMP engineering capacity for reviewer dashboard, document storage).

**Shipping shape.** ~60–90 pages of Trellis spec, ~50 test vectors, ~3–5k lines of Rust across a small crate workspace, one CLI binary. Weeks–months, not quarters.

### Phase 2 — **Runtime-time integrity** (Trellis Vision 1: shared library)

Trellis becomes a Rust crate both runtimes link against. Every write is attested as it happens, not just at export boundaries. Formspec uses it through the Respondent Ledger extension point; WOS uses it through `custodyHook`. Respondent Ledger Profile B is the operating mode.

**Unlocks.** SOC 2 / ISO 27001 enterprise expansion. "Tamper-evident by default" is procurement-language literal.

### Phase 3 — **Portable case files** (Trellis Vision 2: unified canonical ledger)

One Trellis ledger per case, spanning Formspec responses and WOS governance events. A complete case — drafts, submissions, amendments, determinations, appeals — exports as a single verifiable bundle that travels across agencies, across years, across operators. Respondent Ledger Profile C is the operating mode.

**Unlocks.** Multi-agency federal programs (Medicaid across states, SSDI federal↔state DDS, grants federal↔pass-through). The strategic Adobe/ServiceNow displacement narrative at the product-category level. "A case is a file — hand it over."

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

1. **Write the Trellis agreement document** (≤5 pages). Scope, primitives (envelope, chain, checkpoint, export), non-goals, seams (Respondent Ledger `LedgerCheckpoint`, WOS `custodyHook`), invariants (append-only, content-addressed, offline-verifiable, storage-agnostic). The synthesis from the preceding strategy conversation is ~80% of it. **Owner: product strategy. Timeline: 1–2 weeks.**
2. **Sign it off.** Explicit yes/no from the people who own "are we building this thing." Not a committee exercise — a gate.
3. **Archive the eleven existing drafts** to `thoughts/archive/`. They're valuable research; they're not converged artifacts. Stop allowing them to be cited as normative.
4. **Write one W3C-style Trellis spec.** Phase 1 scope only: envelope format, canonical CBOR, hash construction, signature profile, chain construction, checkpoint format, export package layout, verification algorithm, security and privacy considerations, composition with Respondent Ledger and `custodyHook`. ~60–90 pages, one file, well-sectioned, richly exampled. **Timeline: 3–4 weeks.**
5. **Derive `envelope.cddl` + `export-manifest.schema.json`.** Referenced explicitly by spec sections. Schema does not lead; it follows.
6. **Author ~50 test vectors.** Language-neutral JSON files under `fixtures/vectors/{append,verify,export,tamper}/`. Every byte-level claim in the spec corresponds to at least one vector.
7. **Write the reference implementation.** Rust crates: `trellis-core`, `trellis-cose`, `trellis-store-postgres`, `trellis-store-memory`, `trellis-verify`, `trellis-cli`, `trellis-conformance`. Public API is three functions: `append`, `verify`, `export`. Passes every fixture vector.
8. **Ship the CLI + WASM bindings.** `trellis verify | append | export`. WASM for browser-side verification (respondent-facing).
9. **Stand up a second implementation.** `trellis-py` or `trellis-go`, written by someone who only reads the spec. Passes every vector byte-for-byte. This is the proof the spec works.

**Phase 1 success criterion:** a stranger writes a second conformant implementation from the spec alone, and every fixture matches byte-for-byte.

### Track B — Finish the in-flight specs and runtimes

10. **Complete the WOS runtime.** `wos-runtime` lands all 105 conformance fixtures green. `wos-cli` ships. `wos-export` (PROV-O, XES, OCEL 2.0) stabilizes.
11. **Ship the Formspec Coprocessor handoff** (Runtime Companion §15). This is the binding that lets Formspec responses flow into WOS case instances cleanly.
12. **Iterate Formspec specs** for remaining spec-complete-but-unimplemented items per the enterprise-feature-gaps doc (References, Ontology, Locale, Screener, Assist, Mapping). Continue the existing loop.

### Track C — First-sale blockers (calendar-gated, start clock now)

These are not specs or engineering — they are certifications with 12–18 month timelines. Starting the clock matters more than perfecting any single artifact.

13. **Start FedRAMP Moderate authorization.** Partner with a 3PAO, select an ATO sponsor, begin posture work. The spec suite's mechanisms (data minimization, version pinning, PII tracing, regulatory references) are the foundation of the narrative.
14. **Start SOC 2 Type II.** Requires 6–12 months of operating history, so the clock starts when a production system does.
15. **File for a GSA Schedule.** Without this, agencies cannot easily buy.
16. **Commission a formal WCAG 2.1 AA audit + VPAT.** The Component spec's per-component ARIA mandates, Theme spec's WCAG guidance, and Locale spec's `@accessibility` context suffix are strong substrate; a production audit converts substrate into procurement evidence.

### Track D — First-sale engineering (the actual gaps)

Per enterprise-feature-gaps.md, these are the genuinely-unspecified critical paths that are not closed by WOS or Formspec specs.

17. **Reviewer dashboard.** The UI that consumes WOS governance and displays it to adjudicators. Purely implementation; the data model is spec-complete.
18. **Document storage backend.** File upload exists client-side; no storage service. Blob store + preview + virus scanning + bulk upload.
19. **Webhook infrastructure.** Outbound event delivery — roadmap Phase 1, currently unimplemented.
20. **Notification delivery.** WOS Notification Template sidecar is spec-complete; email/SMS delivery is not built.

### Track E — Cross-cutting bindings

21. **Close the Respondent Ledger ↔ Trellis binding.** One sentence in the Respondent Ledger spec: *"When an event is wrapped by a Trellis envelope, `eventHash` MUST equal the envelope canonical hash at that sequence."* Plus the one-paragraph nesting note for response-scoped vs case-scoped ledgers.
22. **Close the WOS `custodyHook` ↔ Trellis binding.** Document how a WOS runtime uses Trellis as its custody backend without redefining either spec.

---

## The unifying test

**A stranger reads the three spec documents, writes conformant implementations of each in their preferred language, and passes every conformance vector.** When that's true, the platform works. When it's not, one of the stages in the loop is broken — find it, fix it upstream, let the fix propagate down.

No other test matters more than that one. Not ratification checklists, not constitutional hierarchies, not requirements matrices. Just: can a stranger build it, and does it interop.

---

## The single-sentence pitch

**"When a public decision affects someone's rights, the work — from the applicant's first keystroke to the appellate record — should run on open specifications, governed by machine-enforceable due-process rules, and produce evidence that verifies on an air-gapped laptop in 2045."**

Formspec captures it. WOS governs it. Trellis proves it.
