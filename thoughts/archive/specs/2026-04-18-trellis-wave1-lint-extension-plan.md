# Trellis Wave 1 Lint Extension — Plan

**Date:** 2026-04-18
**Scope:** Extend `scripts/check-specs.py` with the shared plumbing and ~10 per-stream lint rules enumerated in the Wave 1 consolidation plan (§"check-specs.py backlog").
**Closes:** the design-level side of the Wave 1 lint bundle. Per-rule implementation, fixtures, and test expansion land in the commits this plan sequences.
**Does not cover:** authoring O-3 / O-4 / O-5 fixtures themselves, the Rust conformance crate (G-4) that owns ledger-replay declaration checks, or new Core/Companion spec prose. This plan is a lint-only refactor.

**Upstream inputs:**
- [`2026-04-18-trellis-wave1-consolidation-plan.md`](2026-04-18-trellis-wave1-consolidation-plan.md) §"check-specs.py backlog"
- [`2026-04-18-trellis-g2-invariant-audit-paths.md`](2026-04-18-trellis-g2-invariant-audit-paths.md) (spec-cross-ref, projection-rebuild-drill, model-check)
- [`2026-04-18-trellis-o3-projection-conformance.md`](2026-04-18-trellis-o3-projection-conformance.md) (`projection` / `shred` ops, `_pending-projection-drills.toml`)
- [`2026-04-18-trellis-o4-declaration-doc-template.md`](2026-04-18-trellis-o4-declaration-doc-template.md) (TOML-frontmatter-in-Markdown schema, 15 cross-checks)
- [`2026-04-18-trellis-o5-posture-transition-schemas.md`](2026-04-18-trellis-o5-posture-transition-schemas.md) (event-type registry, CDDL cross-ref)
- [`2026-04-18-trellis-g3-fixture-system-design.md`](2026-04-18-trellis-g3-fixture-system-design.md) F6 (status/deprecated_at, fixture-naming guard)

## Context

`scripts/check-specs.py` is ~480 lines with ~15 discrete check functions today. It already owns: forbidden-term scan, Core-section cross-reference, requirement-ID uniqueness + anchor resolution, bare-"Profile" guard, archived-input guard, vector-coverage audit (`TR-CORE-*` / `TR-OP-*` rows with `Verification=test-vector`), declared-coverage audit (`tr_core`/`core_sections`/`invariants` round-trip), invariant-coverage audit (invariant #N → any testable row → any vector), manifest-path resolution (A/F7), generator-import discipline, and the `_pending-invariants.toml` allowlist.

Wave 1 spec edits add four new surfaces that lint must learn: (a) two new manifest ops (`projection`, `shred`) and a parallel allowlist; (b) a new governance artifact (delegated-compute declaration doc) with its own schema and cross-refs; (c) Core §6.7's reject-if-unknown-at-version event-type registry plus the two CDDL transition types Core §19 / Companion A.5 pin; (d) `TR-OP-*` rows and §-anchors in the Companion, which the existing `core_sections` / `tr_core` plumbing does not cover.

The consolidation plan correctly calls this out as a single refactor rather than four PRs. The question this plan answers is *how* to sequence it so Stream B / C / D fixture authoring is unblocked as fast as possible while keeping the existing green bar intact.

## Design

### Guiding decisions

1. **Shared plumbing first, rules second.** The reusable primitives (op dispatch, allowlist loader, companion section/row resolution, markdown-table extraction) are ≥60% of the diff. Landing them as their own commit — with no new rules firing — lets Stream B / C / D begin fixture authoring against stable helpers and surfaces plumbing bugs in isolation.
2. **Phase the rules. Do not ship 10+ rules at once.** The minimum viable lint for unblocking fixture authoring is the op dispatch + allowlist + fixture-naming guard + F6 deprecation enforcement. Every other rule (declaration-doc validator, event-type registry, CDDL cross-ref, spec-cross-ref resolution, projection-rebuild-drill coverage, model-check evidence) lands after fixtures start arriving, when we know what the real false-positive surface is.
3. **One allowlist file, not four.** `_pending-invariants.toml` already carries two namespaces (`pending_invariants`, `pending_tr_core`). O-3's `_pending-projection-drills.toml` gets its own file because the coverage mechanism differs (drill rows, not test-vector rows). Declaration-doc and transition-event rules piggyback on `pending_tr_core` — they are per-row, not per-invariant. Two allowlist files total.
4. **Minimum viable markdown extraction.** Regex is sufficient for Core §6.7 registry and Companion A.5 CDDL blocks. Both live under stable fenced headings in specs we already lint, and both have <30 entries. A dedicated markdown+table parser is overbuilt until a rule actually trips on regex brittleness.
5. **Phase 1 declaration-doc validator = 6 static cross-checks only.** The 7 ledger-replay checks (agreement signatures, event-emission coverage, grantee-attestation chain, scope-drift detection, etc.) require Core event bytes in hand, which means G-4 Rust. Leave them to the Rust conformance crate; the Python lint does the frontmatter schema + the 6 checks that resolve by file-system + spec-text alone.

### Shared plumbing — the interface pin

Land in commit 1. Nothing new fires; existing rules keep working; new helpers wait for callers.

```python
# --- Op dispatch (extended) ---
VECTOR_OPS = ("append", "verify", "export", "tamper", "projection", "shred")
def vector_manifests() -> list[tuple[Path, dict]]: ...   # already exists; extend op list

# --- Companion section/row resolution (new, mirrors core_headings / matrix_rows) ---
def companion_headings() -> dict[str, str]:              # "§16.2" -> "Snapshot Cadence"
    """Scan Companion headings with the same regex shape as core_headings."""

def derived_companion_sections_for_tr_op(row_ids: list[str]) -> set[str]:
    """Mirror of derived_sections_for_tr_core but over trellis-operational-companion.md."""

def tr_op_ids() -> list[str]:                            # already implicit in matrix_ids();
                                                         # split into tr_core_ids + tr_op_ids helpers

# --- Allowlist loader (parameterized) ---
def load_allowlist(path: Path, schema: dict[str, type], errors: list[str]) -> dict:
    """Generic TOML allowlist loader. Schema maps field name -> element type.
    Malformed TOML -> error appended, returns empty dict. Unknown fields warned."""
    # _pending-invariants.toml is the first caller; _pending-projection-drills.toml
    # is the second. Keeps one code path for "load allowlist, tolerate absent,
    # reject malformed, return typed sets."

# --- Markdown extraction layer (new; minimal) ---
def core_event_type_registry() -> set[str]:
    """Extract the event-type-string table under Core §6.7. Regex over the
    fenced markdown table only; no general table parser."""

def companion_cddl_blocks() -> dict[str, str]:
    """Extract CDDL fenced code blocks under Companion §Appendix A, keyed by
    the type name declared in the block's opening line."""
```

Non-goals for commit 1: no new `check_*` functions are wired into `main()`. The helpers are dead code until commit 2+. This keeps the diff reviewable and the existing test suite green unchanged.

### Per-rule order (dependency-driven)

Rules sorted by what they depend on, not by stream origin. Rules with no new data source run first as quick wins.

| # | Rule | Depends on | Commit |
|---|---|---|---|
| R1 | **Fixture-naming guard** — enforce `<op>/NNN-slug` under every vector tree. Reject drift, uppercase, missing NNN prefix, gap-in-numbering is NOT enforced (renumbering-forbidden is the G-3 F6 discipline). | op dispatch only | commit 2 |
| R2 | **F6 deprecation enforcement** — if manifest declares `status = "deprecated"`, require `deprecated_at` (RFC 3339 UTC); reject other `status` values; deprecated vectors are skipped from coverage counting. | op dispatch, `vector_manifests()` | commit 2 |
| R3 | **`_pending-projection-drills.toml` loader + drift guard** — parallel to `_pending-invariants.toml`; listed-but-now-covered is an error; unknown row IDs are errors. | allowlist loader | commit 2 |
| R4 | **Projection/shred op recognition in existing coverage** — `tr_op` coverage entries from `projection/*` and `shred/*` manifests feed the existing `check_vector_coverage` path, gated by `Verification=projection-rebuild-drill` rather than `test-vector`. | op dispatch, R3, `tr_op` helper | commit 3 |
| R5 | **`tr_op` + `companion_sections` round-trip in declared-coverage audit** — extend `check_vector_declared_coverage` so manifests declaring `tr_op` must satisfy the same derived-equals-declared rule `tr_core` already does. | companion_headings, tr_op helper | commit 3 |
| R6 | **Spec-cross-ref row resolution (G-2)** — for matrix rows with `Verification=spec-cross-ref`, extract the referenced §N from the row's Rationale/Notes cell and verify it exists in the named spec. | companion_headings, core_headings | commit 4 |
| R7 | **Projection-rebuild-drill coverage (G-2 / O-3)** — for matrix rows with `Verification=projection-rebuild-drill`, verify ≥1 fixture under `fixtures/vectors/projection/` or `fixtures/vectors/shred/` references the row in `coverage.tr_op`. `_pending-projection-drills.toml` is the escape hatch. | R3, R4 | commit 4 |
| R8 | **Model-check evidence assertion (G-2)** — for rows with `Verification=model-check`, require an evidence artifact at a declared path (e.g., `thoughts/model-checks/<row-id>/*.tla`). Path convention pinned in this plan; evidence file existence is the only check. | — | commit 4 |
| R9 | **Event-type registry check (O-5)** — extract Core §6.7 registry; scan manifests' expected events for `event_type` strings; every string MUST appear in the registry. | `core_event_type_registry()` | commit 5 |
| R10 | **CDDL cross-ref check (O-5)** — for event-type strings emitted in vectors, verify the referenced CDDL block in Companion Appendix A declares fields consistent with what the vector emits (name-level, not byte-level; byte-level is the G-4 runner's job). | R9, `companion_cddl_blocks()` | commit 5 |
| R11 | **Declaration-doc validator — Phase 1 (6 static checks)** — frontmatter TOML parses; required fields present; `declaration_uri` round-trips to file path; `posture_declaration_ref` resolves to a declared posture doc; `authorized_actions` values are in the enum (minus `decide`, per WG); `effective_from` is RFC 3339 UTC. | allowlist loader (for any deferred-doc allowlist if needed; likely none) | commit 6 |

Rules R1–R3 are the **minimum viable unblocker**. They enforce the authoring conventions Stream B / C / D need before any fixture lands. R4–R10 can run asynchronously behind fixture authoring. R11 lands last because O-4 fixtures (the reference declaration) are themselves a follow-on in the consolidation plan.

### Allowlist topology — final

| File | Owns | Fields | Callers |
|---|---|---|---|
| `fixtures/vectors/_pending-invariants.toml` | Byte-testable coverage backlog | `pending_invariants` (int list), `pending_tr_core` (row-id list; field name historical — holds both `TR-CORE-*` and `TR-OP-*` row IDs with `Verification=test-vector`) | `check_invariant_coverage`, `check_vector_coverage` |
| `fixtures/vectors/_pending-projection-drills.toml` | Projection-rebuild-drill coverage backlog | `pending_tr_op` (row-id list; `TR-OP-*` rows with `Verification=projection-rebuild-drill`) | R7 |

**Declaration-doc and transition-event lint rules do not get their own allowlists.** Rationale:
- O-5 transition events: coverage is per-row (`TR-OP-042..045`) with `Verification=test-vector`; `pending_tr_core` already accepts `TR-OP-*` row IDs.
- O-4 declaration-doc: the reference declaration is a single artifact in Phase 1, not a coverage-matrix obligation. If it fails lint, fix the artifact — don't allowlist it.

Two files keeps the topology legible: **pending_invariants** is "invariants we haven't covered yet," **pending_projection_drills** is "drill rows we haven't exercised yet." Each allowlist answers one question.

### Testing strategy

Existing harness (`scripts/test_check_specs.py`) uses per-scenario fixture directories under `scripts/check-specs-fixtures/`. Each scenario is a synthetic `specs/` + `fixtures/vectors/` snapshot; the test runs `check-specs.py` as a subprocess with `TRELLIS_LINT_ROOT` repointed.

**The harness scales.** No refactor needed. Per-scenario directories are cheap and self-documenting. Test count grows linearly with rule count — that is acceptable.

Expected new scenarios (≈happy + 1 negative per error branch + edge cases):

| Rule | Scenarios | Count |
|---|---|---|
| R1 fixture-naming | valid, uppercase-slug, missing-NNN, symlink-under-op | 4 |
| R2 F6 deprecation | valid-active, valid-deprecated, missing-deprecated_at, unknown-status | 4 |
| R3 allowlist loader | pending-ok, listed-but-covered, malformed-toml, unknown-row-id | 4 |
| R4 op dispatch | projection-vector-valid, shred-vector-valid, bad-op-name | 3 |
| R5 declared-coverage tr_op | valid, mismatch | 2 |
| R6 spec-cross-ref | resolves, §-does-not-exist, missing-cite | 3 |
| R7 projection-rebuild-drill coverage | covered, gap, pending-ok, listed-but-covered | 4 |
| R8 model-check evidence | evidence-present, evidence-missing | 2 |
| R9 event-type registry | in-registry, missing-from-registry | 2 |
| R10 CDDL cross-ref | fields-match, field-name-drift | 2 |
| R11 declaration-doc (6 checks) | happy, each-of-6-negatives | 7 |
| **Total new scenarios** | | **≈37** |

Each scenario is a directory of maybe 4–8 small files. The scenario directory count roughly doubles from today's 18 to ≈55. That is manageable.

### Spec-text parsing — minimum viable

- **`core_event_type_registry()`**: regex over the single fenced table under Core §6.7 with a `^\| `event-type-string` \|` pattern. <30 entries expected. No parser.
- **`companion_cddl_blocks()`**: regex for fenced \`\`\`cddl blocks inside Appendix A. Key by the first `TypeName =` line in the block. Two callers (R10 + potentially a future sanity check).
- **Matrix row Notes/Verification cells**: already parsed by `matrix_rows()`. The spec-cross-ref target extraction (R6) is a regex `§[0-9]+(?:\.[0-9]+)*` over the row's Verification/Notes column; if ambiguous, require the author to pin the target in `Verification` cell as `spec-cross-ref(§6.7)`.

A dedicated markdown+table parser is a trap at this scale — it takes days to write and introduces a dependency. Revisit only if a rule needs structural knowledge regex can't encode (nested blockquotes, cross-table join).

### Declaration-doc validator — Phase 1 / Phase 2 split

**Phase 1 — Python lint (this plan, R11):**
1. Frontmatter is valid TOML; required fields present per O-4 schema.
2. `declaration_uri` matches the document's on-disk path.
3. `posture_declaration_ref` resolves to a URN declared in a Companion §11 Appendix A posture doc (or the reference posture doc under `deployments/`).
4. `authorized_actions` values are subset of `{read, propose, commit_on_behalf_of}` (WG dropped `decide`).
5. `effective_from` is RFC 3339 UTC; `supersedes` (if non-empty) references an existing `declaration_id`.
6. `scope.time_bound` non-empty unless `open_ended_permitted = true`.

**Phase 2 — Rust conformance crate (G-4; out of scope here, listed for provenance):**
7–13. Grant-signature verification, event-emission coverage (every action in scope emits a ledger event), grantee-attestation chain, scope-drift detection against observed events, decision-class-vs-observed-action consistency, attribution-ledger-id resolution, supersedes-chain acyclicity.

The 2 "mixed" checks from O-4 (the ones that could plausibly land in either phase) both require reading CBOR event bytes to verify claims; they go to Phase 2 with the other replay checks.

### Fixture-naming guard + F6 deprecation — integration

R1 + R2 together enforce the pre-merge fixture-authoring discipline G-3 F6 designed.

- **R1** walks every op-dir, asserts each child directory matches `^\d{3}-[a-z0-9-]+$`. Rejects uppercase, rejects missing NNN, rejects symlinks (symlinks would let a vector participate under two IDs, breaking the renumbering-forbidden rule).
- **R2** reads each manifest's top-level `status` field. Absent/`"active"` means the vector counts toward coverage. `"deprecated"` requires `deprecated_at` and removes the vector from coverage counting (it still lints — a deprecated vector that became invalid is still a bug).
- Together: a vector is born with a permanent `NNN-slug` identity, can be deprecated but not deleted or renumbered, and the lint enforces both halves. This is the authoring-window discipline G-3 F6 pins.

## Execution — commit-sized chunks

Target: no commit > **M**. Each commit is tested in isolation; each commit leaves lint green.

| # | Commit | Size | Contents |
|---|---|---|---|
| 1 | **Shared plumbing** | S | Op dispatch list widened; `companion_headings()`, `derived_companion_sections_for_tr_op()`, `load_allowlist()` generic, `core_event_type_registry()`, `companion_cddl_blocks()` helpers. Zero new rules wired. Existing tests unchanged. |
| 2 | **Authoring-discipline rules (R1–R3)** | S | Fixture-naming guard, F6 deprecation enforcement, `_pending-projection-drills.toml` loader. ≈12 new scenarios. Unblocks Stream B / C / D. |
| 3 | **Coverage expansion (R4–R5)** | S | `projection`/`shred` op recognition, `tr_op` + `companion_sections` round-trip. ≈5 new scenarios. Pairs with spec-edit commit that adds `TR-OP-*` rows. |
| 4 | **G-2 verification-column rules (R6–R8)** | S | Spec-cross-ref, projection-rebuild-drill coverage, model-check evidence. ≈9 new scenarios. |
| 5 | **O-5 registry + CDDL rules (R9–R10)** | S | Event-type registry scan, CDDL cross-ref. ≈4 new scenarios. Depends on Core §6.7 + Companion A.5 spec edits. |
| 6 | **O-4 declaration-doc Phase 1 (R11)** | S | 6 static cross-checks. ≈7 new scenarios. Lands once reference declaration exists under `deployments/`. |

**Total: 6 commits, all S.** The bundle fits in **M** overall; breaking into 6 commits keeps any single review digestible and lets Stream B start authoring after commit 2.

## Non-goals

- Rewriting the existing lint architecture. `check-specs.py` stays a single file. If it crosses ~800 lines, revisit — not sooner.
- Ledger-replay declaration-doc checks (7 of 15 surfaces). Owned by G-4 Rust conformance crate.
- Authoring the reference declaration document itself (O-4 follow-on).
- Authoring O-3 / O-5 fixtures (separate plans).
- Adding a general markdown/table parser. Regex until a rule demands otherwise.
- Retrofitting existing scenarios. Existing 18 directories stay; new rules add new scenarios.

## Follow-ons

- **F-LINT-01** — Once G-4 Rust lands, port the 7 ledger-replay declaration-doc rules (O-4 Phase 2) into the conformance crate. Python lint stays at Phase 1.
- **F-LINT-02** — After Stream B / C / D fixtures land, review false-positive / false-negative rate on R6 (spec-cross-ref) and R10 (CDDL cross-ref). If regex proves too brittle, replace with a thin markdown AST helper at that point.
- **F-LINT-03** — If `TR-OP-*` or Companion §-count grows past ≈200, the linear scans in `matrix_rows()` / `companion_headings()` become visible in CI time. Measure before optimizing; currently a non-issue.

## Unknowns flagged for orchestrator

1. **Spec-cross-ref target syntax.** R6 assumes matrix Verification cells for these rows are pinned as `spec-cross-ref(§N)` so the regex has a deterministic anchor. If the existing rows write the cite elsewhere (Rationale, Notes), R6 needs a wider regex and the false-positive surface grows. Orchestrator call: pin syntax in matrix format guide, or widen regex.
2. **Model-check evidence path convention.** R8 requires a path. Candidates: `thoughts/model-checks/<row-id>/`, `fixtures/model-checks/<row-id>/`, or leave path in the matrix cell itself (`model-check(path=...)`). Cheapest is matrix-cell encoding; most discoverable is a dedicated tree. Orchestrator call.
3. **Declaration-doc home directory.** R11 assumes `deployments/<deployment-id>/declaration.md`. Consolidation-plan WG decision housed the declaration *schema* in Companion Appendix A.6 but did not pin a repository path for the reference instance. Orchestrator call: confirm `deployments/` or choose alternative before R11 lands.
