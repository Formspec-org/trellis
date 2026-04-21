# Trellis Phase-1 MVP — Principles and Format ADRs

**Status:** Draft, 2026-04-20. Pending user validation.

Captures the vision model used to decide the three open format lock-ins
(event topology, anchor cardinality, federation hooks) and the
Rust-vs-Python authority question from `TODO.md`. The principles below are
load-bearing for ADRs 0001–0004 that follow. Revise the principles before
revising the ADRs; downstream decisions flow from them mechanically.

**User posture (captured 2026-04-20):**
- Zero records will be issued under Phase-1-shape before G-5 ratifies it.
- PROD-MVP deployable as fast as possible; G-5 is soon-next.
- **Maximalist envelope, restrictive Phase-1 runtime.** Reserve architectural
  capacity in the wire format; enforce Phase-1 scope via lint rules, not
  via absence-of-capacity.
- Integrity maintained via two independent byte-level implementations
  (Rust + Python) that must agree.

Authority: this posture overrides vision-model:279 ("minimalism over
reservation"), which was captured as a placeholder before the maximalist
directive landed. Vision-model will be updated after validation.

---

## Principles (the vision model)

1. **Phase-1 scope is Phase-1 scope.** Ship the Phase-1 runtime fast. Do not
   widen Phase-1 runtime to accommodate Phase 2+ use cases; do not narrow
   Phase 1 by deferring prerequisites. "MVP" is about runtime scope and
   delivery speed, not about envelope expressiveness.

2. **Zero Phase-1-shape records before G-5.** The revision window is now
   through G-5 commissioning. After commissioning: stranger re-work. After
   issuance: data migration. Exploit the cheap window for ENVELOPE decisions;
   rely on it less for RUNTIME decisions (runtime evolves under SemVer).

3. **"Format-breaking" is lifecycle-dependent.** Three cost tiers, often
   conflated:
   - **Pre-G-5 commissioning** — free. Regenerate fixtures via CI.
   - **Post-G-5, pre-issuance** — moderate. Stranger re-reads the spec
     delta and re-implements affected surface.
   - **Post-issuance** — expensive. Data migration across deployed records.
     Not applicable pre-deployment.

   Envelope additions post-issuance are especially expensive if the existing
   envelope can't carry them. Hence Principle 5.

4. **Rust is the byte authority.** The reference architecture is G-4 Rust.
   For decisions spec prose can't pin (CBOR ordering, COSE headers, ZIP
   metadata, Merkle step composition), the Rust implementation is canonical.
   The spec prose and Rust impl are co-authorities per vision-model:278;
   when they disagree, spec wins and Rust is fixed.

5. **Maximalist envelope, restrictive Phase-1 runtime.** Reserve envelope
   capacity for Phase 2/3/4 use cases now — fields, hash slots, extension
   hooks. Enforce Phase-1 scope at the RUNTIME layer via lint rules
   ("MUST NOT populate", "length MUST equal 1", etc.). Reserving is
   architecturally cheap; format revision post-issuance is architecturally
   expensive (Principle 3, tier three). When Phase 3/4 runtime capability
   lands, lint rules relax and the envelope is unchanged — no format break.

   Reconciles user_profile:65 (prefer architectural right over cheap now)
   with user_profile:59 (default to more restrictive): envelope maximally
   expressive, runtime maximally constrained in Phase 1.

6. **G-5 is the integrity anchor; Python/Rust cross-check is secondary
   integrity surface.** G-5 is the only external witness of spec-prose ↔
   implementation correctness. Internal Python/Rust agreement adds value
   (catches typos, catches spec-ambiguity bugs resolved differently between
   team members working from the same spec). Per Principle 5 applied to
   implementation: maintain two implementations where both are cheap, not
   one where cost-of-retirement is low.

7. **Architectural-capacity tie-break.** When two paths are both defensible
   under Principles 1–6, prefer the one that reserves more architectural
   capacity (broader envelope, more extension seams, stronger abstraction
   boundaries). DRY/KISS applies only when architectural capacity is
   already tied.

---

## ADR 0001 — Event topology: multi-parent DAG envelope, single-parent Phase-1 runtime

**Decision.** Envelope: `priorEventHash: [Hash]` (list form, DAG-capable).
Phase-1 runtime lint: `len(priorEventHash) MUST equal 1` for all events
in Phase-1 scope.

**Principles applied.** #5 maximalist envelope + restrictive runtime,
#3 cost-at-moment (envelope cheap now, expensive post-issuance), #4 Rust
is byte authority (implements both envelope and Phase-1 lint rule).

**Cost analysis.**
- Single-parent cases under list form: envelope carries `[hash]` instead
  of `hash`. Trivial runtime cost; zero semantic cost (list-of-one
  degenerates).
- Phase 3/4 arrival (consolidated adjudications, cross-case merges,
  federation composition): lint relaxes; envelope unchanged; no format
  break; no fixture regeneration; stranger impl already handles list.
- If we had shipped `priorEventHash: Hash` and Phase 3 demanded DAG:
  envelope change, fixture regeneration, stranger re-commissioning,
  record migration.

**Counter-argument considered.** Single-parent chain is simpler to read
and implement; list-of-one is visual noise for Phase-1 readers. Accepted
trade-off: one extra bracket in fixtures in exchange for eliminating the
Phase 3 format-break risk.

**Phase-1 lint rule.** `TR-CORE-R{next}`: *Every event MUST have exactly
one parent hash in `priorEventHash` (array length = 1). Phase-1 lint;
relaxes at Phase 3 scoping.*

**Revisit when.**
- Real Phase-1 consolidation use case emerges (relax lint; runtime
  semantics specified in an amendment).
- Phase 3 scoping begins (formal DAG-semantics spec).

---

## ADR 0002 — Anchor cardinality: list-form envelope, single-anchor Phase-1 default

**Decision.** Envelope: `anchor_refs: [AnchorRef]` (list form). Phase-1
runtime lint: `len(anchor_refs) MUST be ≥ 1` (no upper bound);
operators SHOULD populate with one substrate at Phase-1 deployment.
Multi-anchor is legal from day 1; threshold verification semantics defer
to the Phase 4 Federation Profile.

**Substrate choice for Phase-1 deployment.** Adapter-tier concrete choice
per user_profile:89 and vision-model ε:306. Candidates: Bitcoin
OpenTimestamps, Sigstore Rekor, agency-operated Trillian. Deferred to a
bounded spike; not in scope for this ADR.

**Principles applied.** #5, #3, #7 (architectural-capacity tie-break:
list form beats scalar on capacity).

**Cost analysis.**
- List-of-one in Phase-1 deployments: trivial envelope cost.
- Phase 4 multi-witness federation: envelope ready; threshold semantics
  added via the Federation Profile without format break.
- Single-slot alternative: Phase 4 envelope revision + record migration.

**Counter-argument considered.** Threshold-of-N verification semantics
are not specified in Phase 1. True — but the envelope carries the anchors;
verifiers can enforce Phase-1 "first-anchor-valid" semantics until the
Profile lands.

**Phase-1 lint rule.** `TR-CORE-R{next+1}`: *`anchor_refs` MUST have at
least one entry. Verifiers MUST accept a checkpoint if any anchor in the
list verifies. Multi-anchor threshold semantics defer to the Federation
Profile.*

**Revisit when.**
- Operational experience reveals substrate fragility (add a second anchor
  per-deployment).
- Phase 4 Federation Profile scoping begins.
- Regulatory / procurement forces specific multi-substrate semantics.

---

## ADR 0003 — Federation extension points: envelope-reserved, Phase-1-locked

**Decision.** Reserve envelope fields for Core §22 case ledger and §24
agency log as optional-but-validated fields. Phase-1 runtime lint:
`MUST NOT populate` (the reserved fields must be absent in Phase-1
records).

**Reserved fields (proposed; confirm in Core prose):**
- §22: `case_ledger_ref: Optional[CaseLedgerRef]` with content-addressable
  ID + cross-case-ledger reference shape + version pinning slots.
- §24: `agency_log_head: Optional[AgencyLogHeadRef]` with head digest +
  log identifier.

**Principles applied.** #5 maximalist envelope + restrictive runtime,
#4 Rust is byte authority (implements both envelope fields and lint),
#7 architectural-capacity tie-break.

**Cost analysis.**
- Reserved-absent fields in Phase-1 records: zero bytes on the wire
  (optional CBOR encoding omits absent fields). Zero runtime cost.
- Reader-cognitive cost of reserved fields: nonzero but bounded; mitigated
  by Phase-1 lint + explicit "Phase 4 use" docstring in Core prose.
- Phase 4 federation arrival: lint relaxes; envelope carries the
  references; no format break.
- Without reservation (prior minimalist ADR): Phase 4 envelope bump +
  record migration for cross-agency records.

**Federation Profile co-defers.** Shape A (cooperative trust-anchor
network, per vision-model:310) remains the inherited default for Phase 4.
This ADR reserves the envelope surface but NOT the Profile design;
Profile specification defers with Phase 4 scoping.

**Counter-argument considered.** "Reserved-unused is a few optional
envelope fields, no runtime effect" — true of the wire, nonzero of the
reader. Per Principle 5, the architectural-capacity gain dominates the
cognitive cost.

**Phase-1 lint rule.** `TR-CORE-R{next+2}`: *`case_ledger_ref` and
`agency_log_head` MUST NOT be populated in Phase-1 records. Phase-1
lint; relaxes at Phase 4 Federation Profile scoping.*

**Revisit when.** Phase 4 federation enters scoping.

---

## ADR 0004 — Rust is the byte authority; Python generators retained as cross-check

**Decision.** Rust is the byte authority (Principle 4). Python generators
in `fixtures/vectors/_generator/` are **retained** as a parallel
implementation, not retired. Both implementations MUST produce
byte-identical output for every committed vector; disagreement is a
high-priority investigation (spec ambiguity OR impl bug).

**Principles applied.** #4 Rust is byte authority, #6 dual-impl
cross-check is secondary integrity surface, #5 applied to implementations
(maintain architectural capacity; don't retire what's cheap).

**Integrity model.**
- **G-5 stranger** — the primary external integrity anchor. Reads spec
  prose alone; byte-matches Rust output.
- **Python ↔ Rust cross-check** — CI-enforced on every PR. Catches:
  - Rust impl typos (cheap to fix, caught immediately).
  - Spec-ambiguity bugs where the same spec prose is reasonably read two
    different ways by two team members writing the two impls.
  - Both impls agreeing but being wrong — only G-5 catches this.
- **Authority resolution.** When Rust and Python disagree on a
  byte-level decision the spec prose does not pin: Rust wins (it's the
  reference impl); Python updates to match; the disagreement becomes a
  spec-prose clarification ticket.

**CI discipline.**
- `python3 scripts/check-specs.py` runs both generators for every vector;
  asserts byte-identity.
- Disagreements fail the PR. Investigation ticket is load-bearing; don't
  silence the check.

**Principles applied to retention (why NOT retire despite DRY/KISS
appeal).**
- Principle 5: maintaining two byte-level implementations is
  architectural capacity (two independent witnesses of byte correctness).
  Retiring one loses that capacity irreversibly once muscle memory
  departs.
- user_profile:59: "default to the more restrictive" — two impls
  constrain the byte surface more than one.
- user_profile:65: "prefer architectural decisions over engineering
  speed" — engineering cost of maintaining Python in parallel is low
  under minutes-not-days; architectural value of dual-impl cross-check
  is high.

**Counter-argument considered.** Python duplicates byte-level logic —
not DRY. Accepted: DRY applies when the duplication is likely to drift
unsupervised. Here CI guarantees byte-identity, so the "duplication" is
actively verified to be semantically identical. Not drift; cross-check.

**Revisit when.**
- After G-5 commissioning + first month of production deployment: if the
  Python/Rust cross-check has caught zero interesting bugs (only noise)
  across a representative sample of PRs, reconsider retirement. Decision
  criterion is empirical.
- If Python maintenance burden grows disproportionately to its
  integrity contribution.

---

## Implications for TODO.md

- ADRs 0001–0003 resolve the **Blocker — this week** section. Replace each
  bullet with a one-line pointer to this doc.
- ADR 0004 resolves the **Architecture fork** section. Remove the fork;
  replace with a pointer: "Dual-impl cross-check is the lock-in; Python
  is not scheduled for retirement."
- G-3's acceptance criterion stays as-is: "byte-exact vectors across
  Rust + Python generators." The allowlist-free coverage matrix stays.
- New Phase-1 lint rules (R-series: DAG-length-1, anchor-length-≥1,
  federation-fields-absent) land in `scripts/check-specs.py` as part of
  the corresponding vector authoring.

## Implications for vision-model.md

- Line 279 (principle 5 in vision-model's Trellis section) needs
  reversal: "minimalism over reservation" → "maximalist envelope,
  restrictive Phase-1 runtime."
- ADR 0001–0003 rewrites (lines 284–286) flip from minimalist to
  maximalist-envelope decisions.
- ADR 0004 rewrite (line 287) flips from "Python retires" to "Python
  retained as cross-check."
- Cross-references elsewhere (changelog, handoff) unchanged; these
  describe the session, not the current decision.

Offer: I can apply this vision-model.md update after this doc is
validated.

---

## Validation checklist (for the human)

Tick each item if you agree; redirect if not.

- [ ] Principles 1–7 match your intent under the maximalist directive.
- [ ] ADR 0001 (DAG envelope, length-1 runtime) is the correct synthesis
      of "be maximalist" for event topology.
- [ ] ADR 0002 (list-form anchor envelope, multi-anchor legal from day 1,
      substrate choice deferred to spike) matches.
- [ ] ADR 0003 (§22 + §24 envelope hooks reserved, MUST-NOT-populate at
      Phase 1) is the right reservation scope (or name missing fields).
- [ ] ADR 0004 (retain Python as cross-check, not retire) is the right
      reading of "maximalist" applied to implementations.
- [ ] "Revisit when" triggers on each ADR are the right conditions to
      re-open the decision.

If all six tick, mark this doc **Accepted**, commit it, update
`vision-model.md`, and pull pointers into `TODO.md` so the Blocker section
collapses to "done — see this doc" and the Architecture fork section
collapses to "dual-impl cross-check locked in."
