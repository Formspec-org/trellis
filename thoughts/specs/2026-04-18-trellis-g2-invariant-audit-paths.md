# Trellis G-2 Invariant Audit Paths — Design

**Date:** 2026-04-18
**Scope:** Per-invariant audit channel assignment for Phase 1 envelope invariants #1–#15; per-channel audit mechanism; G-2 closure conditions.
**Closes:** G-2 (non-byte-testable half). The byte-testable half is already audited by `scripts/check-specs.py check_invariant_coverage` against `fixtures/vectors/_pending-invariants.toml`.
**Does not cover:** authoring the declaration-doc template (Stream C, concurrent), authoring the projection-rebuild-drill fixtures (Stream B, concurrent), implementing new lint rules, or ratifying invariant #11's §4 text beyond what the matrix already states.

## Context

`ratification/ratification-checklist.md` G-2 requires that every Phase 1 envelope invariant #1–#15 (`specs/trellis-agreement.md` §5) appears as normative MUST text in Core and is cross-referenced from ≥1 `TR-CORE-*` row. The byte-testable half of that bar is enforced today: `check_invariant_coverage` fails CI when an invariant that has ≥1 matrix row with `Verification=test-vector` has no vector claiming such a row (`scripts/check-specs.py:263`). The non-byte-testable half — invariants whose matrix rows carry `Verification` in `{model-check, declaration-doc-check, spec-cross-ref, projection-rebuild-drill, manual-review}` — is explicitly out of scope for that lint (`scripts/check-specs.py:276–280`).

The G-3 fixture-system design (`thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md` "Invariant audit paths") names those five non-byte channels and defers the assignment pass to "a follow-on audit pass … tracked as the remaining G-2 work, not part of G-3. That pass must complete before G-2 closes." This brief is that pass.

Every assignment below is derivable from the `Verification` column of `specs/trellis-requirements-matrix.md` §1 and §2, cross-referenced via §3.1 (invariant → TR-CORE row) and §4 (invariant #11 routing). Nothing here is new semantics; the brief consolidates what the matrix already declares into a per-invariant closure table.

## Classification approaches considered

Three approaches were on the table:

- **Channel-per-invariant, primary only.** One primary channel per invariant, chosen as the strictest available. Simple; legible. Under-represents hybrid invariants (e.g. #2, whose migration obligation is both byte-testable and spec-cross-ref) and makes the closure checklist brittle — a single missing channel would falsely close an invariant that is still half-audited elsewhere.
- **Multi-channel default — every invariant audited ≥2 channels.** Redundancy as signal. Strong against drift (two independent witnesses per invariant) but expensive: forces synthetic declaration docs or manual reviews onto byte-testable invariants that G-3 already covers exhaustively. Generates busywork for the orchestrator.
- **Hybrid — byte-testable invariants audited by G-3 alone; non-byte gets ≥1 channel; invariants whose matrix rows declare multiple verification modes get every mode they declare.** Honors the matrix's existing `Verification` column literally: if a row says `test-vector, spec-cross-ref`, both must have a live audit. No synthetic channels; no dropped channels.

**Chosen: hybrid.** The matrix is the source of truth for verification mode per row (`specs/trellis-requirements-matrix.md` §"Column Schema — Verification"). Adding synthetic channels would recreate the drift the matrix was consolidated to eliminate; dropping declared channels would lose audit signal already encoded in the rows. The byte-testable subset is already zero-maintenance via the G-3 lint; the incremental cost of assigning non-byte channels only where the matrix declares them is minimal and keeps the closure checklist honest.

## Per-invariant audit table

Columns:

- **Channel(s)** — every distinct `Verification` value appearing in the TR-CORE / TR-OP rows the matrix §3.1 maps this invariant to, plus §4 routing for #11.
- **Evidence artifact** — the row ID(s), Companion §, or matrix §N carrying the verification obligation.
- **Cadence** — one-off (passes at audit time, not re-run per release) vs per-release (re-run every time Core / Companion / matrix changes).
- **Owner** — byte-testable channels: fixture authors + `check-specs.py`. Non-byte: the orchestrator signing ratification (`ratification/ratification-checklist.md` §11).

| Inv | Channel(s) | Evidence artifact(s) | Cadence | Owner |
|---|---|---|---|---|
| **#1** Canonical CBOR profile pinned | `test-vector`; `spec-cross-ref` | TR-CORE-030 (`test-vector`); TR-CORE-032 (`test-vector, spec-cross-ref`) → Core §5 "Canonical Encoding" | per-release | fixture authors; lint |
| **#2** Signature suite identified, migration obligation | `test-vector`; `declaration-doc-check`; `spec-cross-ref` | TR-CORE-035 (`test-vector, declaration-doc-check`); TR-CORE-036 (`spec-cross-ref, test-vector`); TR-OP-110 (`test-vector`) → Core §7 "Signature Profile"; Companion §22 "Versioning and Algorithm Agility" | per-release | fixture authors; lint; orchestrator (decl-doc) |
| **#3** Signing-key registry in export | `test-vector` | TR-CORE-037 → Core §8 "Signing-Key Registry" | per-release | fixture authors; lint |
| **#4** Hashes over ciphertext | `test-vector` | TR-CORE-030, TR-CORE-031 → Core §9 "Hash Construction" | per-release | fixture authors; lint |
| **#5** Ordering model named (linear vs causal DAG) | `test-vector`; `model-check`; `spec-cross-ref` | TR-CORE-020 (`test-vector, model-check`); TR-CORE-021 (`test-vector`); TR-CORE-022 (`spec-cross-ref`); TR-CORE-023 (`test-vector, model-check`); TR-CORE-024 (`spec-cross-ref, test-vector`); TR-CORE-025 (`test-vector, model-check`) → Core §10 "Chain Construction" | per-release | fixture authors; lint; orchestrator (model-check) |
| **#6** Registry-snapshot binding in manifest | `test-vector`; `declaration-doc-check` | TR-CORE-070 (`test-vector`); TR-OP-130 (`declaration-doc-check, test-vector`) → Core §14 "Registry Snapshot Binding"; Companion §12 "Metadata Budget Discipline" | per-release | fixture authors; lint; orchestrator (decl-doc) |
| **#7** `key_bag` / author-event-hash immutable under rotation | `test-vector` | TR-CORE-038 → Core §8 "Signing-Key Registry" (rotation subsection) | per-release | fixture authors; lint |
| **#8** Redaction-aware commitment slots reserved | `test-vector`; `spec-cross-ref` | TR-CORE-071 (`spec-cross-ref, test-vector`); TR-OP-071 (`test-vector`) → Core §13 "Commitment Slots Reserved" | per-release | fixture authors; lint; orchestrator (cross-ref) |
| **#9** Plaintext-vs-committed header policy explicit | `declaration-doc-check`; `spec-cross-ref` | TR-CORE-072 (`declaration-doc-check, spec-cross-ref`); TR-OP-040 (`declaration-doc-check`) → Core §12 "Header Policy"; Companion §12 "Metadata Budget Discipline" | per-release | orchestrator |
| **#10** Phase 1 envelope IS Phase 3 case-ledger event | `test-vector` | TR-CORE-080 → Core §18 "Export Package Layout" | per-release | fixture authors; lint |
| **#11** "Profile" namespace disambiguation | `spec-cross-ref` | Matrix §4 "Profile-Namespace Disambiguation" (§4.1–§4.4) — no TR-CORE row; Core §21 "Posture / Custody / Conformance-Class Vocabulary" carries the normative prose | one-off (matrix §4 reviewed at ratification; re-triggered only on vocabulary change) | orchestrator |
| **#12** Head formats compose forward; agency log superset | `test-vector` | TR-CORE-081, TR-CORE-082, TR-CORE-083 → Core §11 "Checkpoint Format"; Core §24 "Agency Log" | per-release | fixture authors; lint |
| **#13** Append idempotency in wire contract | `test-vector`; `model-check` | TR-CORE-050 (`test-vector, model-check`); TR-CORE-051 (`test-vector`); TR-CORE-053 (`test-vector`) → Core §17 "Append Idempotency Contract" | per-release | fixture authors; lint; orchestrator (model-check) |
| **#14** Snapshots and watermarks day-one | `projection-rebuild-drill` | TR-CORE-090 (`projection-rebuild-drill`); TR-OP-001, TR-OP-002, TR-OP-003 (`projection-rebuild-drill, manual-review`), TR-OP-005, TR-OP-006 (`projection-rebuild-drill`) → Core §15 "Snapshot and Watermark Discipline"; Companion §14–§17 (Derived-Artifact Discipline, Projection Runtime Rules, Snapshot-from-Day-One, Staff-View Integrity) | per-release | Stream B (projection fixtures) |
| **#15** Trust posture honesty floor | `declaration-doc-check`; `manual-review` | TR-CORE-100 (`declaration-doc-check, manual-review`) → Core §20 "Trust Posture Honesty"; Companion §11 "Posture-Declaration Honesty" | per-release | Stream C (decl-doc template); orchestrator |

**Summary of channel assignments:**

- **Byte-testable only** (G-3 lint closes): #1, #3, #4, #7, #10, #12 — 6 invariants.
- **Byte-testable + non-byte secondary channel(s)**: #2, #5, #6, #8, #13 — 5 invariants.
- **Non-byte only**: #9, #11, #14, #15 — 4 invariants.

**By channel (unique invariant counts):**

- `test-vector` (G-3 lint): 11 invariants — {#1, #2, #3, #4, #5, #6, #7, #8, #10, #12, #13}.
- `declaration-doc-check`: 4 invariants — {#2, #6, #9, #15}.
- `spec-cross-ref`: 5 invariants — {#1, #2, #5, #8, #9, #11} (6 listed; note #1's cross-ref applies only to future-registry gating under TR-CORE-032 and is satisfied by matrix §3.1 + Core §5; see "Channel audit mechanisms" below).
- `model-check`: 2 invariants — {#5, #13}.
- `projection-rebuild-drill`: 1 invariant — {#14}.
- `manual-review`: 1 invariant — {#15} (inherits as TR-CORE-100's paired channel with declaration-doc-check); also implicit on #14 via TR-OP-001/002/003.

No sixth channel was added. The five G-3 channels cover every `Verification` value appearing in any TR-CORE or TR-OP row whose invariant column points to #1–#15.

## Channel audit mechanisms

For G-2 to close, each non-byte channel needs a concrete "done-for-one-invariant" check. Byte-testable channel is already concrete via the G-3 lint.

### `test-vector` (byte-testable)

**Done** iff `scripts/check-specs.py check_invariant_coverage` reports zero errors for the invariant and the invariant is not listed in `fixtures/vectors/_pending-invariants.toml` `pending_invariants`. Fully automated; per-release.

### `declaration-doc-check`

**Done for one invariant** iff (a) a reference declaration document exists at a declared path (Stream C output), (b) that declaration document contains the fields the invariant's TR-CORE / TR-OP row demands — for #2 the signature-suite-in-force + reserved successor identifiers; for #6 the domain-registry snapshot digest; for #9 the plaintext-vs-committed header enumeration; for #15 the posture declaration per Core §20 + Companion §11, and (c) `scripts/check-specs.py` has a lint rule that parses the declaration and fails if a required field is absent. Today only (a) is missing (Stream C is authoring the template concurrently) and (c) is missing (see Follow-ons). Cadence: per-release (re-lint on every Core / Companion edit).

### `spec-cross-ref`

**Done for one invariant** iff for every TR-CORE / TR-OP row whose `Verification` contains `spec-cross-ref` and whose `Invariant` cell names this invariant, the matrix's `Rationale` or `Notes` column cites a Core / Companion § that contains the normative MUST and that § resolves in the live document. `scripts/check-specs.py check_core_section_references` already verifies that every `Core §N` reference in the Companion resolves (`scripts/check-specs.py:375`). Extension needed: the same resolution check against matrix-row text, plus a lookup asserting the cited § actually carries a MUST about the claim. Cadence: per-release. For invariant #11 specifically, the audit target is matrix §4 prose + Core §21; the lint need only assert §4 and Core §21 both exist.

### `model-check`

**Done for one invariant** iff a property-based or state-machine model exists (owned by G-4 Rust workspace or the G-5 stranger impl) that exercises the row's claim and passes. For #5: a chain-construction model that asserts there is exactly one canonical order per scope under arbitrary concurrent admission. For #13: an append-idempotency model asserting same-key-same-payload retries converge and same-key-different-payload retries reject. Cadence: per-release of the reference impl. No local lint possible — audit is by test-suite pass on the reference impl, reported in G-4 evidence. Manual cross-check at ratification time: "does the reference impl ship a named model test for this row?"

### `projection-rebuild-drill`

**Done for one invariant** iff Stream B's projection conformance fixtures include a drill that rebuilds the watermarked artifact from the canonical chain and byte-compares (per `thoughts/product-vision.md` §"Phase 1 envelope invariants" #14). Invariant #14 is the only invariant on this channel. Audit mechanism once Stream B lands: a fixture-walk like G-3's but over `fixtures/projections/` (or equivalent), with a lint rule asserting each watermark row in the matrix has ≥1 rebuild fixture. Cadence: per-release. Stream B is the dependency.

### `manual-review`

**Done for one invariant** iff the orchestrator signs an attestation in `ratification/ratification-checklist.md` naming the invariant and the reviewed artifact (declaration document, prose, or fixture). One-off per ratification cycle; re-triggered on any edit to the invariant's Core / Companion section. No automation. Applicable today to #15 (paired with declaration-doc-check on TR-CORE-100) and implicitly to #14 (paired with projection-rebuild-drill on TR-OP-001/002/003).

## G-2 closure conditions

G-2 flips from partial to closed when all of the following are true:

- [ ] Every invariant #1–#15 has ≥1 channel assigned in the table above. *(true today, via this brief; awaiting ratification cite.)*
- [ ] `test-vector` channel: `check_invariant_coverage` reports zero errors AND `_pending-invariants.toml` `pending_invariants = []`. Drives closure via G-3 corpus rollout (`TODO.md` "Critical path" step 2).
- [ ] `declaration-doc-check` channel: Stream C's template landed; one reference declaration document authored covering #2, #6, #9, #15 fields; lint rule authored that fails if a required field is absent. *(all three are Stream C / Follow-ons.)*
- [ ] `spec-cross-ref` channel: for every matrix row whose `Verification` contains `spec-cross-ref` and whose `Invariant` cell names #1, #2, #5, #8, #9, or #11, the cited Core / Companion § resolves and carries the claimed MUST. Lint extension to `check-specs.py` (Follow-on) enforces this.
- [ ] `model-check` channel: G-4 reference impl ships named property-based tests for #5 and #13 in `trellis-conformance`. Audit is "tests present and passing," asserted in G-4 evidence.
- [ ] `projection-rebuild-drill` channel: Stream B's fixtures land; lint rule covers #14 the way G-3 covers #1. See Stream B design.
- [ ] `manual-review` channel: orchestrator attestation in `ratification/ratification-checklist.md` for #15 (and any #14 rows still claiming manual-review after Stream B).
- [ ] `fixtures/vectors/_pending-invariants.toml` has `pending_invariants = []` and `pending_tr_core = []`. (This is the byte-testable half already tracked in `TODO.md`; reiterated here because G-2 depends on both halves.)

When every box is checked, the G-2 row in `ratification/ratification-checklist.md` flips from `[ ]` to `[x]` with inline evidence: commit SHA for this brief, SHA for the `check-specs.py` lint extensions, SHA for the Stream B/C fixture landings, commit SHA for each declaration document, and the G-4 SHA proving the model-check tests pass.

## Non-goals

- Authoring the declaration-doc template itself. Stream C owns that.
- Authoring projection-rebuild-drill fixtures. Stream B owns that.
- Implementing lint rules. The brief lists them as Follow-ons; another agent is extending `check-specs.py` concurrently per the F6 guidance in the G-3 fixture-system design.
- Re-classifying invariants into channels the matrix does not declare. The matrix's `Verification` column is authoritative; if a channel is missing, the fix is to add it to the row and re-run `docs:check`, not to annotate it here.
- Adding new invariants. The 15 Phase 1 invariants are frozen per `specs/trellis-agreement.md` §5.

## Follow-ons

Listed for the orchestrator; not implemented here.

- **Lint: declaration-doc-check enforcement.** New rule in `scripts/check-specs.py`: given a declaration-doc path (per Stream C's template), parse it and fail if the fields required by invariants #2 (signature-suite-in-force + reserved successors), #6 (domain-registry snapshot digest), #9 (plaintext-vs-committed enumeration), or #15 (posture declaration per Core §20 + Companion §11) are missing. Feeds off Stream C's template schema.

- **Lint: spec-cross-ref row resolution.** Extend `check_core_section_references` (`scripts/check-specs.py:375`) to scan the matrix as well. For every row with `Verification` containing `spec-cross-ref`, parse any `Core §N` / `Companion §N` citation in the `Rationale` or `Notes` cell and assert the § exists. Already-covered companion-side logic generalizes.

- **Lint: projection-rebuild-drill coverage.** Once Stream B fixtures land, mirror `check_invariant_coverage` for `Verification=projection-rebuild-drill`. Assert every such matrix row has ≥1 rebuild fixture under `fixtures/projections/` (or wherever Stream B pins it).

- **Lint: model-check evidence assertion.** At ratification close, `check-specs.py` reads a `model-check-evidence.toml` listing (row_id → test-path) for every matrix row with `Verification=model-check`, and fails if any row is missing. Populated from G-4 Rust workspace output.

- **Matrix row for invariant #11.** Currently covered only by matrix §4 prose (per `ratification-checklist.md` M-3 wording refined after initial check-in). If future ratification cycles find the prose-only route fragile, add a TR-CORE row with `Verification=spec-cross-ref` pointing at matrix §4 + Core §21. Not required for G-2 close — documented here for the next orchestrator to weigh.

- **Reconciliation with Companion §27 "Operational Conformance Tests."** Companion §27 enumerates operational conformance tests (projection rebuild, crypto-shred cascade, rejection semantics, metadata-budget compliance). Several overlap with this brief's non-byte channels (#14 projection-rebuild-drill, #9 metadata budget via declaration-doc-check). On ratification, audit that §27's test list is a superset of this brief's channel assignments, or annotate the gap. Not blocking; alignment pass only.

## Consumers

Once ratified, this brief is consumed by:

1. `ratification/ratification-checklist.md` G-2 row — cites this brief as evidence of the non-byte-testable half's audit plan.
2. `scripts/check-specs.py` follow-on plan — the four lint rules listed above draw their requirements from the "Channel audit mechanisms" section.
3. Stream C (declaration-doc template) — the declaration-doc-check section names the invariants a template must cover (#2, #6, #9, #15).
4. Stream B (projection fixtures) — the projection-rebuild-drill section names invariant #14 as the sole inhabitant of that channel.
5. G-4 / G-5 implementors — the model-check section names #5 and #13 as the invariants whose audit depends on reference-impl test suites.
