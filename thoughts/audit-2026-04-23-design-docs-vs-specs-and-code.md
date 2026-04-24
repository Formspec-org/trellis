# Design Doc Audit — 2026-04-23

Comprehensive status of every `thoughts/specs/` document against current `specs/` and code (`crates/`, `trellis-py/`, `fixtures/`, `scripts/`).

16 agents ran in parallel, one per design doc.

**Post-audit reader note (2026-04-24):** Summary row 9 and Actionable item #1 below describe an **O-5 disclosure-profile verifier gap** that was **re-closed on 2026-04-23** later the same day (after this audit prose was finalized). Both Rust and Python `decode_transition_details` handle `trellis.disclosure-profile-transition.v1`, with `tamper/016-disclosure-profile-from-mismatch` as the negative oracle. Do not re-do that work from Finding #1 alone — confirm against current `main` and [`ratification/ratification-checklist.md`](../ratification/ratification-checklist.md) O-5.

---

## Summary Table

| # | Document | Verdict | Key Finding |
|---|---|---|---|
| 1 | `core-gaps-surfaced-by-g3` | **FULLY RESOLVED** | All 3 blocking + 5 secondary gaps closed in spec, code, and matrix |
| 2 | `g2-invariant-audit-paths` | **FULLY RESOLVED** | All 15 invariants have audit channels; G-2 gate closed |
| 3 | `g3-first-batch-brainstorm` | **MOSTLY RESOLVED** | All 5 vectors landed; `tamper_kind` enum never pinned in Core §17.5 as recommended |
| 4 | `g3-fixture-scaffold-plan` | **FULLY RESOLVED** | All 12 tasks done; corpus grew from 1 planned to 45+ vectors |
| 5 | `g3-fixture-system-design` | **MOSTLY RESOLVED** | Core design fully implemented; `fixtures/vectors/README.md` stale (lists 4 ops, not 6); renumbering CI guard not built |
| 6 | `g4-rust-workspace-plan` | **MOSTLY RESOLVED** | All 10 crates built, G-4/G-5 closed. Drift: layout changed, API names diverged, `no_std` not attempted, HPKE not in Rust yet |
| 7 | `o3-projection-conformance` | **MOSTLY RESOLVED** | All 4 test types implemented, O-3 gate closed. Cadence only covers height-based kind; non-deterministic rebuild fixture deferred |
| 8 | `o4-declaration-doc-template` | **MOSTLY RESOLVED** | Reference doc + static lint landed, O-4 closed. Signature crypto verify (rule 14) and supersedes acyclicity (rule 15) not implemented; 9 ledger-replay rules have zero implementation |
| 9 | `o5-posture-transition-schemas` | **FULLY RESOLVED (post-audit 2026-04-23)** | Disclosure-profile axis re-closed same session: `decode_transition_details` + `tamper/016` — see ratification O-5 narrative. Row above described state before that re-close. |
| 10 | `wave1-consolidation-plan` | **FULLY RESOLVED** | Every spec edit, matrix row, lint rule, and fixture landed in sequence; all referenced gates closed |
| 11 | `wave1-lint-extension-plan` | **FULLY RESOLVED** | All 11 rules (R1–R11) implemented. File at 1818 lines exceeds the plan's own ~800-line revisit trigger |
| 12 | `hpke-freshness-decision` | **MOSTLY RESOLVED** | Core §9.4 amended, `append/004` fixture complete. No Rust HPKE implementation exists yet (spec-only); §8.6 wording weaker than §9.4 |
| 13 | `phase-1-mvp-principles-and-format-adrs` | **MOSTLY RESOLVED** | All 4 ADR intents achieved. **ADR 0001/0002/0003 mechanisms changed**: `prev_hash` is scalar (not list), `anchor_ref` is scalar (not list), named fields replaced by `extensions` containers. Architectural goals met, but CDDL-level prescriptions are stale |
| 14 | `evidence-integrity-attachment-binding` | **FULLY RESOLVED** | Extension registered, `061-attachments.cbor` manifest, verifier obligations, and all 4 fixtures fully implemented |
| 15 | `g5-stranger-commission-brief` | **FULLY RESOLVED** | All 6 deliverables present, 45/45 byte-match, G-5 closed |
| 16 | `wos-custody-hook-wire-format` | **MOSTLY RESOLVED** | All 8 proposals in spec/fixtures. Cross-repo WOS ADR 0061 acceptance not confirmable from Trellis alone |

---

## Actionable Items Surfaced

### Real gaps (not by-design deferrals)

1. ~~**O-5 disclosure-profile verifier gap**~~ — **Closed 2026-04-23 same session** (see banner above + `ratification-checklist.md` O-5). Original text retained for archaeology: Rust `decode_transition_details` only handled custody-model; disclosure-profile transitions passed verification without semantic checks (`thoughts/specs/2026-04-18-trellis-o5-posture-transition-schemas.md`).
2. **O-4 rules 14–15** — signature crypto verification and supersedes-chain acyclicity lint not implemented (`thoughts/specs/2026-04-18-trellis-o4-declaration-doc-template.md`)
3. **`tamper_kind` enum** — never formally pinned in Core §17.5; values are de-facto consistent across corpus but not normatively enumerated (`thoughts/specs/2026-04-18-trellis-g3-first-batch-brainstorm.md`)
4. **`check-specs.py` at 1818 lines** — exceeds its own revisit trigger at ~800 (`thoughts/specs/2026-04-18-trellis-wave1-lint-extension-plan.md`)
5. **`fixtures/vectors/README.md` stale** — lists 4 ops, should list 6 (`thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md`)
6. **HPKE not in Rust** — spec and fixtures exist, no reference implementation in crates (`thoughts/specs/2026-04-19-trellis-hpke-freshness-decision.md`)

### Design-doc hygiene

- ADRs 0001–0003 in `phase-1-mvp-principles-and-format-adrs.md` have stale CDDL prescriptions (mechanism changed, intent preserved)
- O-4 template doc has ~12 fields that were dropped/renamed in Companion A.6 without the design doc being annotated

---

## 1. `2026-04-18-trellis-core-gaps-surfaced-by-g3.md` — FULLY RESOLVED

### Doc Summary

This design doc catalogues six gaps (three blocking, three secondary) that prevented the G-3 fixture authoring Task 10 from producing byte-exact test vectors from Core prose alone. It recommends Path 1 (amend Core now, then retry Task 10). The doc itself is marked **resolved 2026-04-18** with specific commit hashes for each gap.

### Status per Item

#### B1 — COSE protected-header label for `suite_id`

**IMPLEMENTED** (spec + code).

- Spec: `trellis-core.md:390` pins `suite_id` → integer label `-65537`, `artifact_type` → `-65538`, plus serialization order rationale.
- Code: `crates/trellis-types/src/lib.rs:29` defines `COSE_LABEL_SUITE_ID: i128 = -65_537`. `crates/trellis-types/src/lib.rs:166` encodes the label bytes. `crates/trellis-verify/src/lib.rs:1396` parses it at verification time.
- Python generators use the same label value.

#### B2 — Three event surfaces (authored / canonical / signed)

**IMPLEMENTED** (spec + requirements matrix).

- Spec: `trellis-core.md:327–337` (§6.8) defines all three surfaces with CDDL type references: `AuthorEventHashPreimage`, `EventPayload`, `Event = COSESign1Bytes`.
- Requirements matrix: `trellis-requirements-matrix.md:64` adds `TR-CORE-018` tracking §6.8.
- Fixture filenames (`input-authored-event.cbor`, `expected-canonical-event.cbor`, `expected-signed-event.cbor`) align with the three-surface vocabulary.

#### B3 — `AppendHead` struct for `expected-next-head.cbor`

**IMPLEMENTED** (spec + code).

- Spec: `trellis-core.md:682–699` (§10.6) defines `AppendHead = { scope, sequence, canonical_event_hash }` with full CDDL.
- Code: `crates/trellis-types/src/lib.rs:89–121` implements `AppendHead` struct. `crates/trellis-cddl/src/lib.rs:184–185` implements `append_head_bytes()`. `crates/trellis-core/src/lib.rs:130` produces it on append. `crates/trellis-conformance/src/lib.rs:81` reads `expected-append-head.cbor` for byte-match.
- Requirements matrix: `trellis-requirements-matrix.md:144` adds `TR-CORE-083`.

#### S1 — `event_type` identifier registration

**IMPLEMENTED** (spec).

- Spec: `trellis-core.md:924–930` (§14.6) reserves `x-trellis-test/` prefix for conformance fixtures.
- Requirements matrix: `trellis-requirements-matrix.md:135` adds `TR-CORE-073`.
- Fixtures: Extensively used across all generators (e.g., `gen_append_001.py`, `gen_v3_remaining.py`, etc.) with `x-trellis-test/append-minimal`, `x-trellis-test/unclassified`, etc.

#### S2 — `classification` identifier registration

**IMPLEMENTED** (same mechanism as S1).

- The `x-trellis-test/` reservation in §14.6 applies to `classification` as well as `event_type`. Fixture generators consistently use `x-trellis-test/unclassified`.

#### S3 — `PayloadInline.nonce` length

**IMPLEMENTED** (spec + code).

- Spec: `trellis-core.md:253` pins `nonce: bstr .size 12` in the CDDL. `trellis-core.md:273` adds explanatory prose linking to the ChaCha20-Poly1305 AEAD nonce size.
- Code/fixtures: `gen_append_001.py:49` uses `b"\x00" * 12`. `gen_v3_remaining.py:688` uses `b"\x00" * 12`.

#### S4 — HPKE latitude for structural-only vectors

**RESOLVED** (separate decision doc).

- The design doc marks this as "deferred" (fixture-system concern, not Core).
- The Core spec addresses this at `trellis-core.md:571` via the **test-vector carve-out**: fixtures MAY pin ephemeral private keys under `fixtures/vectors/_keys/` with (a)/(b)/(c) procedural constraints.
- Separate decision doc `thoughts/specs/2026-04-19-trellis-hpke-freshness-decision.md` elaborates the carve-out rationale.
- Fixture `append/004-hpke-wrapped-inline` exercises real HPKE with pinned ephemerals; structural-only vectors (001, etc.) use zero-ciphertext with a pinned nonce and empty key_bag entries.

#### S5 — `kid` construction

**IMPLEMENTED** (spec + code).

- Spec: `trellis-core.md:444–458` (§8.3) pins derivation as `SHA-256(dCBOR_encode_uint(suite_id_integer) || pubkey_raw)[0..16]` with fixed byte order.
- Code: `crates/trellis-cose/src/lib.rs:18` implements `derive_kid(suite_id: u8, public_key: [u8; 32]) -> [u8; 16]`. `crates/trellis-core/src/lib.rs:121` calls it.

#### Section-numbering drift in the scaffold plan

**SUPERSEDED**.

- The doc notes the scaffold plan cited wrong Core section numbers. The current Core spec has been reorganized so that the sections now match: §6 covers Event Format (including §6.8 three surfaces), §7 covers Signature Profile (including §7.4 headers), §9 covers Hash Construction (§9.2 canonical, §9.5 author), §10 covers Chain Construction (including §10.6 AppendHead), §11 covers Checkpoint Format.
- The scaffold plan's section-citation drift is a historical artifact; the current plan/doc corpus references current section numbers.

### Drift

No drift detected between the design doc's resolved state and the current specs/code. The doc was written to describe gaps that *existed at the time* and then documents the specific commits that closed each gap. The current ratified specs (`trellis-core.md` v1.0.0, `trellis-operational-companion.md` v1.0.0, `trellis-requirements-matrix.md` v1.0.0, all dated 2026-04-21) incorporate all amendments listed in the resolution table. The doc's own resolution table accurately maps to the spec sections as they exist today.

### Verdict

**FULLY RESOLVED.** Every blocking gap (B1–B3) and every secondary gap (S1–S5) is addressed in the current ratified specs, tracked in the requirements matrix with stable TR-CORE IDs, and implemented in the Rust crate code and Python fixture generators. S4 was explicitly deferred to the fixture system (correct scope) and resolved via the Core §9.4 test-vector carve-out plus a dedicated HPKE freshness decision doc.

---

## 2. `2026-04-18-trellis-g2-invariant-audit-paths.md` — FULLY RESOLVED

### Doc Summary

The design doc assigns one or more audit channels (`test-vector`, `declaration-doc-check`, `spec-cross-ref`, `model-check`, `projection-rebuild-drill`, `manual-review`) to each of the 15 Phase 1 envelope invariants, derived mechanically from the `Verification` column of the requirements matrix. It proposes a hybrid classification: byte-testable invariants close via G-3 lint alone; non-byte invariants get at least one channel; every channel declared in a matrix row must be satisfied. It enumerates concrete closure conditions per channel and four follow-on lint rules for `check-specs.py`.

### Status per Item

#### Per-invariant audit table

| Inv | Channel(s) from doc | Status | Evidence |
|---|---|---|---|
| **#1** Canonical CBOR profile pinned | `test-vector`; `spec-cross-ref` | **IMPLEMENTED** | Vectors under `fixtures/vectors/{append,export,verify}/` exercise dCBOR round-trip. R6 lint `check_spec_cross_ref_rows` validates TR-CORE-030/TR-CORE-032 `Core §5` anchors (`scripts/check-specs.py:928`). |
| **#2** Signature suite identified, migration obligation | `test-vector`; `declaration-doc-check`; `spec-cross-ref` | **IMPLEMENTED** | `append/002-rotation-signing-key/` covers rotation. `evidence.toml` has no TR-CORE-035/036 model-check entry (none required — doc assigns `declaration-doc-check` not `model-check`). R6 lint covers `spec-cross-ref`. |
| **#3** Signing-key registry in export | `test-vector` | **IMPLEMENTED** | Export vectors include `010-signing-key-registry.cbor`; `trellis-verify` rejects missing/swap (tamper/012). |
| **#4** Hashes over ciphertext | `test-vector` | **IMPLEMENTED** | `append/001`, `append/004` exercise content-hash-over-ciphertext; TR-CORE-030/031 covered by fixture corpus. |
| **#5** Ordering model named | `test-vector`; `model-check`; `spec-cross-ref` | **IMPLEMENTED** | `model_checks.rs:335` (`tr_core_020_single_canonical_order_per_scope`), `model_checks.rs:345` (`tr_core_023_order_is_independent_of_operational_accidents`), `model_checks.rs:353` (`tr_core_025_concurrency_uses_deterministic_tie_breaking`). `evidence.toml` lines 3–5 map TR-CORE-020/023/025. R6 lint covers `spec-cross-ref` for TR-CORE-022/024. |
| **#6** Registry-snapshot binding in manifest | `test-vector`; `declaration-doc-check` | **IMPLEMENTED** | Export fixtures carry manifest with `registry_snapshot_digest`. TR-OP-130 `declaration-doc-check` satisfied by `ssdi-intake-triage/declaration.md` `audit.registry_ref` field. R11 lint validates. |
| **#7** `key_bag` / author-event-hash immutable under rotation | `test-vector` | **IMPLEMENTED** | `append/002-rotation-signing-key/` demonstrates immutability. |
| **#8** Redaction-aware commitment slots reserved | `test-vector`; `spec-cross-ref` | **IMPLEMENTED** | TR-CORE-071 `spec-cross-ref` cites Core §13; R6 lint validates. TR-OP-071 has `test-vector` coverage. |
| **#9** Plaintext-vs-committed header policy explicit | `declaration-doc-check`; `spec-cross-ref` | **IMPLEMENTED** | TR-CORE-072 and TR-OP-040 both carry `declaration-doc-check`. `ssdi-intake-triage/declaration.md` enumerates content classes and access taxonomy. R11 lint validates. R6 lint covers `spec-cross-ref` for TR-CORE-072 → Core §12. |
| **#10** Phase 1 envelope IS Phase 3 case-ledger event | `test-vector` | **IMPLEMENTED** | TR-CORE-080 covered by export corpus; strict-superset CDDL enforced by trellis-cddl. |
| **#11** "Profile" namespace disambiguation | `spec-cross-ref` | **IMPLEMENTED** | Matrix §4 (lines 418–469) carries the full rename table. Core §21 defines the vocabulary. R6 lint checks `Core §21` resolution. Doc explicitly says "no TR-CORE row; matrix §4 prose" is acceptable for G-2 close. |
| **#12** Head formats compose forward | `test-vector` | **IMPLEMENTED** | TR-CORE-081/082/083 covered by checkpoint and append-head fixtures. |
| **#13** Append idempotency in wire contract | `test-vector`; `model-check` | **IMPLEMENTED** | `model_checks.rs:438` (`tr_core_050_idempotency_keys_are_stable_across_retries`). `evidence.toml` line 7 maps TR-CORE-050. |
| **#14** Snapshots and watermarks day-one | `projection-rebuild-drill` | **IMPLEMENTED** | 5 projection vectors (`projection/001-005`) + 2 shred vectors (`shred/001-002`). R7 lint `check_projection_rebuild_drill_coverage` (`scripts/check-specs.py:859`) validates coverage. |
| **#15** Trust posture honesty floor | `declaration-doc-check`; `manual-review` | **IMPLEMENTED** | `ssdi-intake-triage/declaration.md` covers posture honesty (§11 cross-check). R11 lint validates. Manual-review attestation recorded in `ratification-checklist.md:14` (G-2 signed off). |

#### Channel audit mechanisms

| Channel | Doc closure condition | Status | Evidence |
|---|---|---|---|
| `test-vector` | `check_invariant_coverage` zero errors + `_pending-invariants.toml` empty | **IMPLEMENTED** | `check_invariant_coverage` at `scripts/check-specs.py:605`. `_pending-invariants.toml` removed per ratification evidence. |
| `declaration-doc-check` | Reference declaration doc + lint rule | **IMPLEMENTED** | `check_declaration_docs` at `scripts/check-specs.py:1550` (R11). Reference doc at `fixtures/declarations/ssdi-intake-triage/declaration.md`. |
| `spec-cross-ref` | Lint extension for matrix-row §N resolution | **IMPLEMENTED** | `check_spec_cross_ref_rows` at `scripts/check-specs.py:928` (R6). Scans `Rationale`/`Notes` for `Core §N` / `Companion §N` and validates heading exists. |
| `model-check` | Named property-based tests in `trellis-conformance` + `evidence.toml` | **IMPLEMENTED** | `crates/trellis-conformance/src/model_checks.rs` has 7 proptest tests + 2 deterministic tests. `thoughts/model-checks/evidence.toml` maps 8 rows. `_pending-model-checks.toml` has `pending_matrix_rows = []`. R8 lint `check_model_check_evidence` at `scripts/check-specs.py:1033`. |
| `projection-rebuild-drill` | Stream B fixtures + lint rule | **IMPLEMENTED** | 7 fixtures under `fixtures/vectors/{projection,shred}/`. R7 lint at `scripts/check-specs.py:859`. |
| `manual-review` | Orchestrator attestation in ratification checklist | **IMPLEMENTED** | `ratification/ratification-checklist.md:14` — G-2 checked `[x]` with inline evidence. |

### Drift

1. **`spec-cross-ref` invariant count mismatch (minor).** Doc line 63 lists 6 invariants for `spec-cross-ref` ({#1, #2, #5, #8, #9, #11}) but the parenthetical says "6 listed; note #1's cross-ref applies only to future-registry gating." The matrix currently has `spec-cross-ref` on TR-CORE-018 (no invariant) and several other constitutional rows not mapped in the doc's invariant-centric table.
2. **Model-check evidence exceeds doc scope.** The doc pins model-check to invariants #5 and #13 (TR-CORE-020/023/025/050). The actual `evidence.toml` also covers TR-CORE-001, TR-CORE-046, TR-OP-061, TR-OP-111 — rows the doc doesn't mention.
3. **Projection-rebuild-drill fixture count.** The doc names invariant #14 as the sole inhabitant of `projection-rebuild-drill`. The R7 lint now also covers shred fixtures (`shred/001-002`) which test the crypto-shredding cascade side.
4. **`_pending-invariants.toml` removed.** The doc references this file at lines 76 and 109. The file has been removed from the tree (per ratification checklist evidence). Consistent with the doc's closure condition.
5. **Declaration doc scope.** The doc (line 80) lists invariant #2 as needing "signature-suite-in-force + reserved successor identifiers" in the declaration doc. The ssdi-intake-triage declaration doc is a delegated-compute declaration (Companion §19 / A.6), not a signature-suite declaration. No real gap — the doc's wording over-indexes on a single artifact type.

### Verdict

**FULLY RESOLVED.** All 15 invariants have assigned audit channels. All six channel mechanisms are implemented in `scripts/check-specs.py` (R6–R8, R11) and backed by concrete artifacts (evidence.toml, projection/shred fixtures, declaration docs, model-check tests). G-2 is checked closed in `ratification/ratification-checklist.md:14` with commit SHAs. The two deferred follow-ons (matrix row for #11, §27 reconciliation) are explicitly marked non-blocking by the doc itself. No normative drift found.

---

## 3. `2026-04-18-trellis-g3-first-batch-brainstorm.md` — MOSTLY RESOLVED

### Doc Summary

This brainstorm is a pre-authoring design for five G-3 fixture vectors (`002-rotation-signing-key`, `003-external-payload-ref`, `004-hpke-wrapped-inline`, `005-prior-head-chain`, `tamper/001-signature-flip`). It identifies a mislabeling of invariant numbers in TODO.md, proposes corrected invariant-to-vector assignments, defines an authoring order (005→003→004→002→tamper), predicts Core-spec gaps, specifies generator helper extraction, and shapes the first tamper vector. It also flags five open questions for the orchestrator.

### Status per Item

#### §1 — Invariant assignment per vector

**Option A adopted (keep slot assignments, fix invariant labels). All five vectors implemented with corrected labels.**

| Vector | Status | Evidence |
|---|---|---|
| `append/002-rotation-signing-key` | **IMPLEMENTED** | `fixtures/vectors/append/002-rotation-signing-key/manifest.toml` claims `TR-CORE-036` (migration obligation) and `TR-CORE-038` (#7 key-bag/author_event_hash immutability). All expected artifacts committed. |
| `append/003-external-payload-ref` | **IMPLEMENTED** | `fixtures/vectors/append/003-external-payload-ref/manifest.toml` claims `TR-CORE-031` (#4 hashes over ciphertext) and `TR-CORE-071` (#8 reserved commitment slots). All artifacts committed. |
| `append/004-hpke-wrapped-inline` | **IMPLEMENTED** | `fixtures/vectors/append/004-hpke-wrapped-inline/derivation.md` demonstrates real HPKE wrap with pinned ephemeral X25519 key. All artifacts committed. |
| `append/005-prior-head-chain` | **IMPLEMENTED** | `fixtures/vectors/append/005-prior-head-chain/manifest.toml` claims `TR-CORE-020`, `TR-CORE-023`, `TR-CORE-024`, `TR-CORE-025`, `TR-CORE-050`–`053`, `TR-CORE-080`. All artifacts committed. |
| `tamper/001-signature-flip` | **IMPLEMENTED** | `fixtures/vectors/tamper/001-signature-flip/manifest.toml` carries `[expected.report]` with `integrity_verified = false`, `tamper_kind = "signature_invalid"`. All artifacts committed. |

#### §2 — Dependency order within the batch

**IMPLEMENTED.** The recommended order (005 → 003 → 004 → 002 → tamper) was followed. All generators exist in `_generator/`.

#### §3 — Core-spec gaps likely to surface

| Predicted Gap | Status | Evidence |
|---|---|---|
| §10.2 `prev_hash` linkage (author claim vs service check) | **RESOLVED IN SPEC** | `trellis-core.md:667`: "A Canonical Append Service MUST reject any submission whose `prev_hash` does not satisfy this constraint." |
| §9.4 HPKE freshness obligation | **RESOLVED IN SPEC** | `trellis-core.md:571`: "Test-vector carve-out" paragraph explicitly permits pinned ephemerals in fixtures. |
| §8 rotation + §9.5 `author_event_hash` binding | **RESOLVED IN SPEC** | `trellis-core.md:492`: "Historical `author_event_hash` values MUST reproduce after any LAK rotation." |
| §6.4 `PayloadExternal` integrity shape | **RESOLVED IN SPEC** | `trellis-core.md:256-268`: Full CDDL for `PayloadExternal`. |
| §17.5 `IdempotencyKeyPayloadMismatch` error shape | **RESOLVED IN SPEC** | `trellis-core.md:1044-1058`: Normative rejection codes table with 10 named codes. |
| §9.1 length-prefix uniformity | **RESOLVED IN SPEC** | `trellis-core.md:512-513`: Uniform length-prefix form regardless of component count. |

#### §4 — Generator work required

**IMPLEMENTED (Option B adopted).** `_generator/_lib/byte_utils.py` exists with `dcbor()`, `domain_separated_sha256()`, and `deterministic_zipinfo()`. Each generator imports from `_lib` but keeps spec-interpretive logic inline with `§N` citations.

#### §5 — First tamper vector shape

**IMPLEMENTED (Option B adopted).** Based on 005 (non-genesis chain) as recommended. Manifest carries inline `[expected.report]` with failure details.

#### §6 — Unknowns flagged for orchestrator

| Unknown | Status | Evidence |
|---|---|---|
| 1. `tamper_kind` enum pre-enumeration | **PARTIALLY RESOLVED** | Values are de-facto consistent across the corpus but no formal enum table exists in Core. The brainstorm's recommendation to "pin the enum in §17.5" was not done. |
| 2. HPKE freshness — Core amendment vs fixture convention | **RESOLVED** | Core amendment chosen (`trellis-core.md:571`). |
| 3. `prev_hash` linkage check locus | **RESOLVED** | Service MUST reject (`trellis-core.md:667`). |
| 4. Generator pre-work in its own commit | **NOT VERIFIED** | Cannot confirm from file state alone. |
| 5. Stream D `append/005` dependency / freeze | **IMPLEMENTED** | Vector 005 exists and is `status = "active"`. |

### Drift

1. **Brainstorm referenced `_pending-invariants.toml`** — this file does not exist. The pending-invariants tracking mechanism was either abandoned or renamed.
2. **Brainstorm claimed "this batch closes invariants {#7, #8 (partial), #10, #13}"** — the actual vector manifests claim coverage for these plus additional invariants.
3. **Additional vectors beyond the brainstorm's scope exist** — vectors 006–021 and tamper vectors 002–015 are now implemented, going well beyond the brainstorm's "first batch" scope.

### Verdict

**MOSTLY RESOLVED.** All five proposed vectors are implemented with committed artifacts, corrected invariant labels, generators, derivation documents, and passing bytes. Of the six items, only the `tamper_kind` enum pre-enumeration (unknown #1) remains partially open — no normative enum table exists in Core, though values are de-facto consistent across the tamper corpus.

---

## 4. `2026-04-18-trellis-g3-fixture-scaffold-plan.md` — FULLY RESOLVED

### Doc Summary

A 12-task implementation plan to scaffold `fixtures/vectors/` per the G-3 fixture system design, extend `scripts/check-specs.py` with four coverage-lint rules (TDD via synthetic test scenarios), and author one end-to-end reference vector (`append/001-minimal-inline-payload`) as proof the system works. Subsequent vector batches are deferred to follow-on plans.

### Status per Item

| Task | Status | Evidence |
|---|---|---|
| 1: Scaffold top-level fixture directories and READMEs | **IMPLEMENTED (exceeded)** | All planned files exist plus `projection/`, `shred/`, `_templates/` |
| 2: Add derivation-evidence template | **IMPLEMENTED** | `fixtures/vectors/_templates/derivation-template.md` exists |
| 3: Add lint test harness (TDD foundation) | **IMPLEMENTED (significantly exceeded)** | `scripts/test_check_specs.py` exists (1,129 lines, 19 test classes) with 40+ synthetic scenarios |
| 4: Extend `check-specs.py` to accept `TRELLIS_LINT_ROOT` | **IMPLEMENTED** | `check-specs.py:13` reads `TRELLIS_LINT_ROOT` env var |
| 5: Implement coverage rule 1 — testable rows must have vectors | **IMPLEMENTED** | `check_vector_coverage` at `check-specs.py:796`; `TRELLIS_SKIP_COVERAGE` blanket bypass fully removed |
| 6: Implement coverage rule 2 — declared coverage must equal matrix-derived | **IMPLEMENTED (exceeded)** | `check_vector_declared_coverage` at line 563 |
| 7: Implement rule 3 (invariant coverage) and rule 4 (generator import discipline) | **IMPLEMENTED (significantly exceeded)** | `check_invariant_coverage` at line 605, `check_generator_imports` at line 650 |
| 8: Generate and commit the pinned issuer key | **IMPLEMENTED** | `fixtures/vectors/_keys/issuer-001.cose_key` + 4 additional keys |
| 9: Commit pinned sample payload | **IMPLEMENTED (exceeded)** | `sample-payload-001.bin` + `sample-external-payload-003.bin` |
| 10: Author the first vector end-to-end — `append/001-minimal-inline-payload` | **IMPLEMENTED (exceeded)** | All 8 sibling files exist. Manifest covers 19 TR-CORE rows + 1 TR-OP row |
| 11: Link fixture scaffold from top-level docs | **IMPLEMENTED** | `README.md:83` has the pointer |
| 12: Final verification pass | **IMPLEMENTED** | All ratification gates checked; 45+ vectors across all six op dirs |

### Drift

| Aspect | Plan | Current State | Impact |
|---|---|---|---|
| `TRELLIS_SKIP_COVERAGE=1` bypass | Transitional env var | Fully removed; replaced by allowlist then cleaned | Positive drift |
| Op directories | 4 ops | 6 ops: +`projection`, `shred` | Positive drift |
| Vector count | 1 reference vector | 45+ across six op dirs | Massive positive drift |
| Lint rules | 4 coverage rules | 12+ rules (R1–R12) | Massive positive drift |
| Sibling file names | `expected-canonical-event.cbor`, `expected-signed-event.cbor` | `expected-event-payload.cbor`, `expected-event.cbor` | Minor naming drift |

### Verdict

**FULLY RESOLVED.** Every task in the 12-task plan was implemented. The codebase then significantly exceeded the plan's scope.

---

## 5. `2026-04-18-trellis-g3-fixture-system-design.md` — MOSTLY RESOLVED

### Doc Summary

The design doc (242 lines, dated 2026-04-18, amended same day for findings F1–F6) specifies the directory layout, TOML manifest schema, derivation-evidence convention, coverage-enforcement rules, conformance-runner contract, and authoring discipline for the `fixtures/vectors/` corpus. It resolves seven pre-implementation decisions and amends six review findings. The corpus is the ratification artifact for gates G-3, G-4, and G-5.

### Status per Item

| Item | Status | Notes |
|---|---|---|
| Directory layout (4 ops → 6 ops) | **IMPLEMENTED** | `projection/` and `shred/` added |
| TOML manifest format | **IMPLEMENTED** | Every vector has `manifest.toml` with validated fields |
| Coverage enforcement rules 1–3 | **IMPLEMENTED** | Rules 1–3 in `check-specs.py` |
| Pending-invariants allowlist (F5) | **IMPLEMENTED** | File removed (all coverage enforced) |
| Derivation evidence | **IMPLEMENTED** | Every vector has `derivation.md` with §-citations |
| Cryptographic intermediates | **IMPLEMENTED** | Sibling `.bin`/`.cbor` files in vectors |
| Vector lifecycle / deprecation (F6) | **IMPLEMENTED** | `check_vector_lifecycle_fields()` at line 715 |
| Vector naming convention | **IMPLEMENTED** | `VECTOR_NAMING_PATTERN` at line 445 |
| Generator discipline | **IMPLEMENTED** | `check_generator_imports()` at line 650 |
| Conformance runner contract | **IMPLEMENTED** | Rust and Python runners walk all ops |
| Inline `[expected.report]` for verify/tamper | **IMPLEMENTED** | Structured-data tables in manifests |
| `_keys/` and `_inputs/` README provenance | **IMPLEMENTED** | Both READMEs catalog entries |
| Overlap policy (duplicate coverage allowed) | **IMPLEMENTED** | Union-coverage in lint |
| Renumbering-forbidden rule (F6 follow-on) | **NOT STARTED** | CI-level pre-merge guard not implemented |
| `tr_op` coverage field | **IMPLEMENTED** (drift) | Not in original design; fully lint-enforced |
| `_templates/` directory | **IMPLEMENTED** (drift) | Not in original design |
| R6–R12 lint rules | **IMPLEMENTED** (drift) | 6 additional rules beyond the 4 in design |

### Drift

| Item | Doc says | Codebase does | Severity |
|---|---|---|---|
| Op set | 4 ops | 6 ops: +projection, +shred | Low — additive |
| `coverage.tr_op` | Not mentioned | Present, lint-enforced | Low — additive |
| `fixtures/vectors/README.md` | Lists 4 ops | Still lists 4 ops (stale — missing projection/shred) | Medium — reader-facing doc is stale |
| Manifest field names | `prior_head`, `signing_key`, `authored_event` | Different names in actual manifests | Medium — design doc was proposals |
| Renumbering guard | CI-level pre-merge check | Not implemented | Low — deferred |

### Verdict

**MOSTLY RESOLVED.** All core design elements are fully implemented and lint-enforced. Two minor gaps: (1) `fixtures/vectors/README.md` is stale (lists 4 ops, not 6), and (2) the renumbering-forbidden CI guard is not implemented.

---

## 6. `2026-04-18-trellis-g4-rust-workspace-plan.md` — MOSTLY RESOLVED

### Doc Summary

This plan defines the Cargo workspace layout, crate split, public API surface, milestone sequencing, and fixture-runner contract for the G-4 Rust reference implementation. It prescribes 10 crates across 5 layers, three public API functions (`append`, `verify`, `export`), two milestones (byte-match `append/001` then full corpus), and a stranger-test isolation discipline. Both milestones have been achieved (G-4 closed, G-5 closed at 45/45).

### Status per Item

#### Crate Graph

| Planned Crate | Status | Evidence |
|---|---|---|
| `trellis-cddl` (Layer 0) | **IMPLEMENTED** | 303 lines. CBOR parsing, authored-event decode, canonical-event construction. |
| `trellis-cose` (Layer 0) | **IMPLEMENTED** | 69 lines. `derive_kid`, `protected_header_bytes`, `sig_structure_bytes`, `sign_ed25519`, `sign1_bytes`. Hand-built COSE (no `coset`). |
| `trellis-types` (Layer 1) | **IMPLEMENTED** | 219 lines. Domain tags, `StoredEvent`, `AppendHead`, `AppendArtifacts`, encoding helpers. |
| `trellis-core` (Layer 2) | **IMPLEMENTED** | 160 lines. `LedgerStore` trait, `append_event` function. |
| `trellis-verify` (Layer 2) | **IMPLEMENTED** | 3726 lines. Full export-ZIP verification. |
| `trellis-export` (Layer 2) | **IMPLEMENTED** | 291 lines. Deterministic ZIP. Hand-rolled. |
| `trellis-store-memory` (Layer 3) | **IMPLEMENTED** | 36 lines. |
| `trellis-store-postgres` (Layer 3) | **IMPLEMENTED** | 351 lines. Sync `postgres` crate. |
| `trellis-conformance` (Layer 4) | **IMPLEMENTED** | 612 + 632 lines (model_checks). |
| `trellis-cli` (Layer 4) | **IMPLEMENTED** | 223 lines. Scaffold scope as planned. |

#### Public API

| Planned Function | Status | Notes |
|---|---|---|
| `append` | **IMPLEMENTED** (renamed `append_event`) | Signature differs from plan (no `RegistryBinding` parameter) |
| `verify` | **IMPLEMENTED** (split into 3 functions) | No single façade entry point; `VerifyPolicy` knob not implemented |
| `export` | **SUPERSEDED** | No standalone function. `ExportPackage::to_zip_bytes()` instead. No `ExportPolicy` type. |

#### Milestones

| Milestone | Status |
|---|---|
| M1.a — CDDL + dCBOR | **IMPLEMENTED** |
| M1.b — COSE_Sign1 signing/verification | **IMPLEMENTED** |
| M1.c — Hash construction + domain separation | **IMPLEMENTED** |
| M1.d — Event construction (append) | **IMPLEMENTED** |
| M1.e — Verification (single-event path) | **IMPLEMENTED** |
| M1.f — Export ZIP layout | **IMPLEMENTED** |
| M2 — Full corpus byte-match | **IMPLEMENTED** (G-4 and G-5 both closed, 45/45) |

### Drift

1. **Directory layout**: Plan prescribed `trellis/rust/crates/`. Actual: `trellis/crates/`. The `rust/` intermediate directory was dropped.
2. **Workspace boundary**: Workspace Cargo.toml is at `formspec/Cargo.toml` (parent repo), combining Trellis and Formspec crates into one workspace.
3. **Layer violations**: Plan had `trellis-cose` at Layer 0 with no Trellis-type dependency. Actual: depends on `trellis-types`.
4. **`coset` dropped**: COSE_Sign1 is hand-built, which is stricter.
5. **No `zip` crate for writing**: Hand-writes ZIP bytes. Uses `zip` for reading only.
6. **`no_std` not pursued**: No crate has `#![no_std]`.
7. **API surface divergence**: Split verify functions instead of single façade. No `VerifyPolicy`/`ExportPolicy`.
8. **HPKE not in Rust** (Open Item #6 remains open).

### Verdict

**MOSTLY RESOLVED.** All 10 crates exist and are functional. Both milestones are closed. Drift is concentrated in directory layout, API naming, `no_std`, and HPKE. None represents missing capability.

---

## 7. `2026-04-18-trellis-o3-projection-conformance.md` — MOSTLY RESOLVED

### Doc Summary

The design doc specifies how the O-3 ratification gate (projection discipline conformance) should be implemented: four test types (watermark attestation, rebuild equivalence, snapshot cadence, purge-cascade verification), their fixture layout under `fixtures/vectors/` using Option C (unified tree with op-specific manifest extensions), per-op runner contracts, and coverage-enforcement rules for `tr_op` matrix rows. It also surfaces three spec gaps and lists open naming/encoding items deferred to fixture-authoring time.

### Status per Item

| Proposal | Status | Evidence |
|---|---|---|
| Option C fixture layout | **IMPLEMENTED** | `projection/` (5 vectors) and `shred/` (2 vectors) |
| Test 1 — Watermark attestation | **IMPLEMENTED** | `projection/001-watermark-attestation/` |
| Test 2 — Rebuild equivalence | **IMPLEMENTED** | `projection/002-rebuild-equivalence-minimal/` |
| Test 3 — Snapshot cadence (positive) | **IMPLEMENTED** | `projection/003-cadence-positive-height/` |
| Test 3 — Snapshot cadence (negative/gap) | **IMPLEMENTED** | `projection/004-cadence-gap/` |
| Test 4 — Purge-cascade (in-scope) | **IMPLEMENTED** | `shred/001-purge-cascade-minimal/` |
| Test 4 — Purge-cascade (backup-refusal) | **IMPLEMENTED** | `shred/002-backup-refusal/` |
| Staff-view watermark subtype | **IMPLEMENTED** | `projection/005-watermark-staff-view-decision-binding/` |
| Coverage enforcement: `tr_op` lane | **IMPLEMENTED** | R5 + R8 rules in `check-specs.py` |
| O-3 ratification gate closure | **CLOSED** | `ratification-checklist.md:37` |
| Spec gaps #1–#3 | **RESOLVED** | All three spec gaps addressed |

### Drift

1. **Manifest layout is flatter than proposed.** `[chain]`/`[view]` nesting replaced by flat `[inputs]`/`[expected]` with flat file names.
2. **No explicit `subtype` field.** Dispatch is implicit by presence of expected keys.
3. **Cadence fixtures only cover height-based kind.** Time/event-driven/hybrid are untested.
4. **Non-deterministic field rebuild fixture is deferred.**

### Verdict

**MOSTLY RESOLVED.** All four test types implemented with fixtures and runners. O-3 gate closed. Minor gaps: cadence only height-based, non-deterministic rebuild deferred, manifest layout flatter than proposed.

---

## 8. `2026-04-18-trellis-o4-declaration-doc-template.md` — MOSTLY RESOLVED

### Doc Summary

This design doc defines the template shape, file format, schema, and lint surface for the O-4 delegated-compute declaration document — a per-deployment artifact mandated by Companion §19 (OC-70a). It chose Markdown-with-TOML-frontmatter (Option 3), extended the Companion Appendix B.4 minimal `DelegatedComputeGrant` into a full audit-surface declaration, specified 15 cross-check rules (6 static + 9 ledger-replay), and defined the minimum-viable close for ratification gate O-4: one reference declaration + passing static lint.

### Status per Item

#### Template design choices

| Item | Status |
|---|---|
| Format: Markdown + TOML frontmatter (Option 3) | **IMPLEMENTED** |
| Path convention: `<slug>/declaration.md` | **IMPLEMENTED** |
| Nullable fields via key absence | **IMPLEMENTED** |

#### 15 Cross-check rules

| Rule | Status |
|---|---|
| 1. Posture ref resolves, A.2 declares `delegated_compute` | **IMPLEMENTED (static)** |
| 2. `operator_id` matches posture | **IMPLEMENTED (static)** |
| 3. Event types in registry | **IMPLEMENTED (static)** |
| 4. `"decide"` not in `authorized_actions` | **IMPLEMENTED (static)** |
| 5. Actor discriminator rule exact literal | **IMPLEMENTED (static)** |
| 6. `runtime_enclave` matches A.2 `delegated_compute_exposure` | **IMPLEMENTED (static)** |
| 7. `max_agents_per_case` ceiling (ledger-replay) | **NOT STARTED** |
| 8. `max_invocations_per_day` ceiling (ledger-replay) | **NOT STARTED** |
| 9. WOS autonomy cap superset (ledger-replay) | **NOT STARTED** |
| 10. Delegation chain monotonicity (ledger-replay) | **NOT STARTED** |
| 11. Actor discriminator on emitted events (ledger-replay) | **NOT STARTED** |
| 12. `agent_identity` attribution matches (ledger-replay) | **NOT STARTED** |
| 13. Emitted types ⊆ `audit.event_types` (ledger-replay) | **NOT STARTED** |
| 14. Signature verifies (static) | **PARTIALLY IMPLEMENTED** — key structure validated, no crypto verify |
| 15. `supersedes` acyclic chain (static) | **NOT STARTED** |

### Drift

1. **Significant schema evolution.** ~30 fields → ~18 fields. Nine named fields were dropped, four audit event-type fields consolidated into one array, `max_invocations_per_case` renamed to `max_invocations_per_day`.
2. **Static lint covers rules 1–6 thoroughly but stops short on rules 14–15.**
3. **Ledger-replay rules 7–13 have zero implementation.** (Consistent with staged plan — "not required for Phase 1 O-4 close.")

### Verdict

**MOSTLY RESOLVED.** The core deliverable — a normative template, reference declaration, and static lint — is fully landed and the O-4 gate is closed. Gaps: static lint rules 14 (signature crypto verify) and 15 (supersedes chain) not implemented; design doc itself is stale relative to A.6.

---

## 9. `2026-04-18-trellis-o5-posture-transition-schemas.md` — FULLY RESOLVED (post-2026-04-23)

### Doc Summary

The design doc defines six decisions needed to close ratification gate O-5 (Posture-Transition Auditability): (1) a co-publish-required rule binding transitions to fresh declarations, (2) two event-type strings for Core §6.7, (3–4) CDDL schemas for custody-model and disclosure-profile transitions, (5) verifier sequencing and attestation rules, and (6) a six-vector fixture plan. It also proposes follow-on spec edits and lists four open items.

### Status per Item

| Decision | Status | Evidence |
|---|---|---|
| D1: Co-publish required rule | **IMPLEMENTED** | Companion A.5.3 step 3 + TR-OP-045. Rust verifier enforces digest match at `trellis-verify:1308-1318`. |
| D2: Event-type strings | **IMPLEMENTED** | Both registered in Core §6.7. |
| D3: CDDL: custody-model transition | **IMPLEMENTED** | Companion A.5.1. Rust decoder at `trellis-verify` `decode_custody_model_transition`. |
| D4: CDDL: disclosure-profile transition | **IMPLEMENTED** | Companion A.5.2 in spec; `decode_disclosure_profile_transition` in `trellis-verify`; `append/008` + `tamper/016-disclosure-profile-from-mismatch`. |
| D5: Verification semantics | **IMPLEMENTED** | Both axes verified in Rust + Python; see `ratification/ratification-checklist.md` O-5 re-close narrative. |
| D6: Fixture plan (6 vectors) | **IMPLEMENTED** | All six landed plus extras. |
| Appendix A.5 rewrite | **IMPLEMENTED** | A.5 structured as A.5/A.5.1/A.5.2/A.5.3. |
| Matrix rows TR-OP-042..045 | **IMPLEMENTED** | All four present. |

### Drift (historical — closed same session)

1. **CDDL type changes.** Design doc used integer enums; ratified spec uses string enums and plain uint.
2. ~~**Disclosure-profile transitions unverified in Rust.**~~ Fixed 2026-04-23 (`086d844`, `5a6c9d5`); see global post-audit banner.
3. **`PostureTransitionOutcome.kind` reporting** — disclosure-profile transitions now surface with `kind = disclosure-profile` in verifier outcomes (paired with custody-model outcomes).

### Verdict

**FULLY RESOLVED.** All six decisions implemented; disclosure-profile verifier gap closed 2026-04-23; dual-extension mutual-exclusion guard added 2026-04-24 in `decode_transition_details` (Rust + Python).

---

## 10. `2026-04-18-trellis-wave1-consolidation-plan.md` — FULLY RESOLVED

### Doc Summary

The Wave 1 Consolidation Plan defines how four design-brief outputs (G-2 invariant audit paths, O-3 projection conformance, O-4 declaration-doc template, O-5 posture-transition schemas) should be merged into the normative specs (Core, Companion, Matrix) and the `check-specs.py` lint harness. It identifies specific spec edits, new matrix rows, ~10 lint extensions, and an execution sequence to unblock O-3/O-4/O-5 fixture authoring and G-2 closure.

### Status per Item

All spec edits in Core, Companion, and Matrix: **IMPLEMENTED**.
All lint extensions: **IMPLEMENTED**.
All fixture authoring (O-3/O-4/O-5): **IMPLEMENTED**.
All WG item resolutions: **IMPLEMENTED** or correctly deferred.
Execution sequencing: **IMPLEMENTED** in the order prescribed.

### Drift

No significant drift detected. The spec edits, matrix rows, lint rules, and fixtures all landed in the order the plan prescribes.

### Verdict

**FULLY RESOLVED.** Every actionable item in the consolidation plan has been implemented. The two items explicitly marked "defer" or "n/a" remain deferred by design, not by neglect.

---

## 11. `2026-04-18-trellis-wave1-lint-extension-plan.md` — FULLY RESOLVED

### Doc Summary

The Wave 1 Lint Extension Plan specifies extending `scripts/check-specs.py` with shared plumbing helpers and 11 lint rules (R1–R11), sequenced across 6 commits. It covers new fixture surfaces and defines a two-allowlist topology plus a ~37-scenario test expansion.

### Status per Item

| Rule | Status | Evidence |
|---|---|---|
| R1 Fixture-naming guard | **IMPLEMENTED** | `check_vector_naming()` at `:448`; 7 test scenarios |
| R2 F6 deprecation enforcement | **IMPLEMENTED** | `check_vector_lifecycle_fields()` at `:715`; 5 scenarios |
| R3 `_pending-projection-drills.toml` loader | **IMPLEMENTED** | `load_pending_projection_drills()` at `:335` |
| R4 Projection/shred op recognition | **IMPLEMENTED** | `vector_manifests()` walks both ops |
| R5 `tr_op` + `companion_sections` round-trip | **IMPLEMENTED** | `:583–592` + `:827`; 6 scenarios |
| R6 Spec-cross-ref row resolution | **IMPLEMENTED** | `:928`; 3 scenarios |
| R7 Projection-rebuild-drill coverage | **IMPLEMENTED** | `:859`; 4 scenarios |
| R8 Model-check evidence assertion | **IMPLEMENTED** | `:1033`; 5 scenarios |
| R9 Event-type registry check | **IMPLEMENTED** | `:1358`; CBOR inspection |
| R10 CDDL cross-ref check | **IMPLEMENTED** | `:1443`; field-name extraction |
| R11 Declaration-doc validator | **IMPLEMENTED** | `:1550`; ~14 distinct validations |

### Drift

1. **Allowlist topology**: Plan specified exactly 2 files; implementation has 3 (`_pending-model-checks.toml` added).
2. **Line count**: Plan says "if it crosses ~800 lines, revisit." File is now **1818 lines**. No refactor has occurred.
3. **Declaration-doc scope expansion**: Plan scoped Phase 1 to 6 static checks; implementation has ~14.

### Verdict

**FULLY RESOLVED.** All 11 planned rules implemented, wired, and tested. The codebase additionally grew 3 extra rules, a third allowlist, and CBOR inspection infrastructure.

---

## 12. `2026-04-19-trellis-hpke-freshness-decision.md` — MOSTLY RESOLVED

### Doc Summary

This decision memo resolves two latent ambiguities in Core §9.4 HPKE-freshness semantics: (A) the scope of "wrap" (per-KeyBagEntry vs per-event vs per-ledger-lifetime) and (B) the conflict between "destroy the ephemeral key" and byte-reproducible fixture vectors. It recommends Option (a) — amending Core §9.4 normative prose.

### Status per Item

| Decision / Proposal | Status | Evidence |
|---|---|---|
| Adopt Option (a): amend Core §9.4 | **IMPLEMENTED** | `trellis-core.md:569-571` contains the amendment |
| Pin "wrap" to per-`KeyBagEntry` (reading A.1) | **IMPLEMENTED** | Line 569 |
| Test-vector carve-out with (a)/(b)/(c) procedural constraints | **IMPLEMENTED** | Line 571 |
| "Destroyed" scoped to private key only | **IMPLEMENTED** | Line 569 |
| Uniqueness scoped to ledger scope | **IMPLEMENTED** | Line 569 |
| Author `append/004-hpke-wrapped-inline` fixture | **IMPLEMENTED** | Full fixture with all 9 expected artifacts |
| `_keys/ephemeral-004-recipient-001.cose_key` | **IMPLEMENTED** | File exists at proposed path |
| `manifest.toml` `test_artifact_keys` declaration | **IMPLEMENTED** | Lines 14-16 |
| `derivation.md` production-vs-test statement | **IMPLEMENTED** | Lines 32-37 |
| §8.6 alignment sentence | **PARTIALLY IMPLEMENTED** | §8.6 "fresh" wording weaker than §9.4 |
| Duplicate-detection lint (G-2 deferred) | **NOT STARTED** | Deferred by design |
| Rust HPKE / KeyBag implementation | **NOT STARTED** | No Rust code implements §9.4 HPKE wrap logic |

### Drift

1. **Section renumbering.** Design doc references Core §4 and §29. Current spec cites §5.2 and §27.
2. **§8.6 wording asymmetry.** Still uses pre-amendment phrase "fresh X25519 ephemeral public key."
3. **No Rust implementation.** Amendment is spec-only without a reference implementation.

### Verdict

**MOSTLY RESOLVED.** All substantive decisions adopted. Core §9.4 amendment landed verbatim, `append/004` fixture complete. Gaps: §8.6 alignment weaker (deferred), no Rust HPKE implementation.

---

## 13. `2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md` — MOSTLY RESOLVED

### Doc Summary

This design doc captures seven principles and four ADRs (0001–0004) that locked in the Phase 1 wire-format posture for Trellis. It reversed an earlier "minimalism over reservation" stance to "maximalist envelope, restrictive Phase-1 runtime," resolved the Rust-vs-Python authority question, and specified the mechanism by which DAG topology, multi-anchor, and federation hooks would be reserved in the envelope without breaking Phase 1 scope.

### Status per Item

#### ADR 0001 — DAG-capable event topology, single-parent Phase-1 runtime

**INTENT IMPLEMENTED, MECHANISM CHANGED.** The ADR prescribed `priorEventHash: [Hash]` (list form). The ratified spec uses:

- **Scalar `prev_hash: digest / null`** in `EventPayload` — NOT the list form.
- **`causal_deps: [* digest] / null`** as a separate reserved field — this is the DAG-capable slot.
- Phase-1 lint: `causal_deps` MUST be `null` or `[]`.

#### ADR 0002 — List-form anchors, single-anchor deployment default

**INTENT IMPLEMENTED, MECHANISM CHANGED.** The ADR prescribed `anchor_refs: [AnchorRef]` (list form). The ratified spec uses:

- **Scalar `anchor_ref: bstr / null`** in `CheckpointPayload`.
- Multi-anchor capacity lives at manifest level (`external_anchors`).

#### ADR 0003 — Federation extension points

**INTENT IMPLEMENTED, MECHANISM CHANGED.** The ADR prescribed named optional fields. The ratified spec uses:

- **`extensions: { * tstr => any } / null`** containers with a registered-identifier namespace.
- This is a stronger, more general mechanism.

#### ADR 0004 — Rust is the byte authority

**FULLY IMPLEMENTED.** Rust crates are the reference implementation (G-4 closed). Python stranger closed G-5 (45/45).

### Drift

Three material mechanism changes between the design doc and the ratified spec:

1. **ADR 0001** — `prev_hash` is scalar, not list. DAG comes through `causal_deps` instead.
2. **ADR 0002** — `anchor_ref` is scalar in checkpoints, not list. Multi-anchor at manifest level only.
3. **ADR 0003** — Named fields replaced by `extensions` containers with registered-identifier namespace.

None weakens the architectural position. All three achieve the ADRs' goals through mechanisms that are equal or stronger. However, the design doc's CDDL-level prescriptions are now inaccurate as a description of the actual wire format.

### Verdict

**MOSTLY RESOLVED.** All four ADRs' architectural intents are achieved. ADR 0004 is an exact match. ADRs 0001, 0002, and 0003 achieved their goals through different wire-level mechanisms than prescribed.

---

## 14. `2026-04-21-trellis-evidence-integrity-attachment-binding.md` — FULLY RESOLVED

### Doc Summary

This design doc mirrors stack ADR 0072 and specifies how evidence attachment binding lands on the Trellis side: an `trellis.evidence-attachment-binding.v1` event extension carries binding metadata, `PayloadExternal` carries ciphertext bytes, an export-manifest extension `trellis.export.attachments.v1` binds an optional `061-attachments.cbor` manifest, and the verifier gains corresponding obligations.

### Status per Item

| Proposal | Status | Evidence |
|---|---|---|
| Export-manifest extension registration | **IMPLEMENTED** | Core §6.7 extension table line 321; Rust verifier lines 1701–1720 |
| Derived attachment manifest (`061-attachments.cbor`) | **IMPLEMENTED** | Core §18.2 line 1128; Rust verifier lines 1768–1856 |
| Verifier obligations (5 sub-steps) | **IMPLEMENTED** | Core §19 lines 1350–1356; Rust verifier implements all |
| Fixture corpus (4 fixtures) | **IMPLEMENTED** | `append/018`, `export/005`, `verify/013`, `tamper/013` |
| Event extension registration | **IMPLEMENTED** | Core §6.7 line 313; Rust verifier lines 1484–1582 |

### Drift

No conflicts detected between the design doc and the current normative state. Extension identifiers, `AttachmentManifestEntry` shape, and "extend, don't mutate" structural rule all align.

### Verdict

**FULLY RESOLVED.** Every proposal has landed in the normative Core spec, the Rust verifier, and the fixture corpus. No drift, no gaps, no open work items.

---

## 15. `2026-04-21-trellis-g5-stranger-commission-brief.md` — FULLY RESOLVED

### Doc Summary

The commission brief defines the rules, allowed/forbidden inputs, required deliverables, and acceptance bar for an independent second implementor to close ratification gate G-5. It specifies building `trellis-py` or `trellis-go` from spec prose alone, byte-matching the committed vector corpus, and producing six deliverables as proof of independence. The chosen language was Python (`trellis-py`).

### Status per Item

| Deliverable | Status | Evidence |
|---|---|---|
| 1: Standalone codebase | **IMPLEMENTED** | `trellis-py/` — 8 source files |
| 2: Local conformance runner | **IMPLEMENTED** | `conformance.py` walks all 6 op types |
| 3: Manifest of allowed inputs | **IMPLEMENTED** | `ALLOWED-READ-MANIFEST.txt` — 19 lines |
| 4: Independence attestation | **IMPLEMENTED** | `ATTESTATION.md` — 13 lines |
| 5: Byte-match report | **IMPLEMENTED** | `BYTE-MATCH-REPORT.json` — 45/45 pass |
| 6: Discrepancy log | **IMPLEMENTED** | `DISCREPANCY-LOG.txt` — zero discrepancies |
| Allowed inputs compliance | **IMPLEMENTED** | Path list matches brief exactly |
| Forbidden inputs not consulted | **IMPLEMENTED** | Attested in ATTESTATION.md |
| Acceptance bar (all 4 criteria) | **IMPLEMENTED** | All criteria met |

### Drift

No conflicts detected. The `ALLOWED-READ-MANIFEST.txt` path list matches the brief exactly. The 6 operation types are a strict superset of the brief's required behaviors. The brief allows Python or Go; Python was chosen.

### Verdict

**FULLY RESOLVED.** All six deliverables present, 45/45 byte-match, independence attestation on file, no conflicts. Gate G-5 is closed.

---

## 16. `2026-04-21-trellis-wos-custody-hook-wire-format.md` — MOSTLY RESOLVED

### Doc Summary

The design doc formalizes the wire-format split between WOS and Trellis at the `custodyHook` seam, pinning eight concrete decisions: one record per append, dCBOR-authored bytes, TypeID-structured identifiers, a two-field `(caseId, recordId)` idempotency tuple under domain tag `trellis-wos-idempotency-v1`, Trellis-owned anchors, dual-fact posture transitions, a `{canonical_event_hash}` return contract, and inter-record refs via `canonical_event_hash`. It also lists four Trellis-side follow-on actions.

### Status per Item

| Proposal | Status | Evidence |
|---|---|---|
| 1: One authored WOS record per append | **IMPLEMENTED** | Core §23.2; Companion §24.9 OC-112; fixture `append/010` |
| 2: Authored bytes are dCBOR | **IMPLEMENTED** | Companion §24.9 OC-112 field 4; `input-wos-record.dcbor` |
| 3: TypeID-structured `caseId` / `recordId` | **IMPLEMENTED** | Companion §24.9 OC-112 fields 1–2; derivation uses `linc_case_`/`linc_prov_` prefixed identifiers |
| 4: Idempotency tuple with domain tag | **IMPLEMENTED** | Companion §24.9; `input-wos-idempotency-tuple.cbor`; Core §23.5 |
| 5: Anchor target stays Trellis-owned | **IMPLEMENTED** | Core §23.2 items 5–6; no WOS-side anchor field |
| 6: Custody transitions produce two canonical facts | **IMPLEMENTED** | Companion §24.10 OC-113c |
| 7: Return is `{canonical_event_hash}` | **IMPLEMENTED** | Companion §24.9 |
| 8: Inter-record refs via `canonical_event_hash` | **IMPLEMENTED** | Core §23.2 item 4 |
| F1: Regenerate `append/010` | **IMPLEMENTED** | Fixture complete with dCBOR + TypeID + domain tag |
| F2: Verify/update Companion §24.9 | **IMPLEMENTED** | Four-field surface matches design doc |
| F3: Coordinate with WOS ADR 0061 | **NOT DETERMINABLE** | Cross-repo; requires WOS repo check |
| F4: No Core §23 prose changes needed | **CONFIRMED** | Core defers specifics to Companion §24.9 |

### Drift

No conflicts detected. Every concrete proposal is reflected in the current normative specs and the fixture corpus.

### Verdict

**MOSTLY RESOLVED.** All eight wire-format proposals implemented in spec prose and fixtures. Three of four follow-on items complete. The remaining item (F3 — cross-repo coordination with WOS ADR 0061) cannot be resolved from within the Trellis repo alone.
