# Trellis — COMPLETED

Historical rollup of landed Trellis work. Organized wave-by-wave.

**This file is for:** wave dispatch history, closed sprint-queue items, closed
stream items, completed critical-path steps. Anyone asking "what's been
done?" reads this.

**This file is not for:** active tactical work (→ [`TODO.md`](TODO.md)),
strategy (→ [`thoughts/product-vision.md`](thoughts/product-vision.md)),
ratification scope (→ [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md)),
or implementation plans (→ [`thoughts/specs/`](thoughts/specs/)).

Prune aggressively — `git log` is the real record. Entries here capture
cross-commit wave context that a raw log cannot reconstruct.

---

## Wave-by-wave dispatch history

### Wave 10 (working tree) — G-5 close and 1.0.0 ratification

Closed ratification after the clean-room stranger pass landed:

- Bound the `trellis-py/` evidence bundle into
  `ratification/ratification-checklist.md` and flipped G-5 to closed.
- Reissued `specs/trellis-core.md` and
  `specs/trellis-operational-companion.md` as `1.0.0`.
- Updated repo-facing status text so active docs no longer describe the
  ratified surface as draft-only.
- Cut the first Trellis release tag for the ratified 1.0.0 surface.

### Wave 9 (working tree) — G-2 traceability cleanup

Closed the remaining local ratification bookkeeping before G-5:

- Added explicit `Core §N` / `Companion §N` anchors to every
  `spec-cross-ref` matrix row that lacked one.
- Flipped G-2 in `ratification/ratification-checklist.md` after
  byte-vector, model-check, declaration-doc, projection-drill, and
  spec-cross-ref evidence all had live checks or fixtures.
- Reconciled `TODO.md` and the executable dispatch doc: O-3/O-4/O-5 are
  closed, the G-5 commission brief exists, and ratification close-out is
  blocked on the independent G-5 implementation only.
- Packaged the tracked G-5 allowed read set under `ratification/g5-package/`
  with per-file and archive SHA-256 checksums. The package excludes forbidden
  paths and untracked workspace files.
- Accepted the Trellis-side custodyHook wire-format note and regenerated
  `append/010-wos-custody-hook-state-transition` on the accepted ADR-0061
  shape: dCBOR authored WOS bytes, TypeID-shaped `caseId` / `recordId`, and
  the two-field `trellis-wos-idempotency-v1` idempotency construction.

### Wave 8 (4 commits `ee57780..b0f114d`) — Wave 6 tail closure

Closed out the Wave 6 tail that had been sitting in the working tree. Four
slices in dependency order:

- `ee57780` — Core amendments: §9.4 HPKE freshness (per-`KeyBagEntry`
  ephemeral uniqueness + fixture test-vector carve-out), §6.4
  ChaCha20-Poly1305 AEAD pin, §6.7 `trellis.staff-view-decision-binding.v1`
  extension-registry row, §19 step 4.k verifier obligation, §28 Appendix A
  CDDL additions (`StaffViewDecisionBinding` + optional
  `projection_schema_id` on `Watermark`). Retroactively satisfies
  `projection/005`'s §6.7 + §28 Appendix A citations.
- `fd54232` — `tamper/004` `tamper_kind` reconciled to canonical
  `posture_declaration_digest_mismatch` per `tamper/001`'s enum.
- `4cc9fe8` — `append/004-hpke-wrapped-inline` vector + 2 pinned X25519 keys
  (`recipient-004-ledger-service`, `ephemeral-004-recipient-001`) under
  `_keys/`. Depends on `ee57780`. Claims TR-CORE-031 + TR-CORE-038.
- `b0f114d` — Lint R9/R10/R11 + 122 new pytest lines + SSDI event-registry
  stub (`fixtures/declarations/ssdi-intake-triage/event-registry.stub.md`).
  Closes Wave 1 lint-refactor plan commits 5-6.

Net state: 25 vectors (up from 24); all six Wave-1 lint rules live; Core
§§6.4/6.7/9.4/19/28 amendments landed. check-specs.py green; pytest 92/92;
renumbering guard green.

### Wave 7 (6 commits `964716c..dd0d1da`) — residue batch + review-fix

5 new fixtures + semi-formal-review-fix pass + TODO refresh.

- `964716c` — residue-batch: `append/009-signing-key-revocation`
  (TR-CORE-037 + TR-CORE-070; §8 Active→Revoked + §14.3 RegistryBinding
  digest; discharges the last `pending_invariants` entries),
  `projection/005-watermark-staff-view-decision-binding` (TR-OP-006;
  staff-view §15.2 Watermark + §28 Appendix A StaffViewDecisionBinding;
  closes final O-3 breadth item), `tamper/006-event-reorder` (TR-CORE-020;
  step 4.h swap).
- `8ba1f59` — TODO refresh for the residue batch.
- `4ae9d3c` — semi-formal-review fix pass (verdict REQUEST CHANGES on
  `964716c`; 1 blocker + 2 warnings fixed in-patch): `§29.3 → §28 Appendix
  A` citation drift (8 places across projection/005
  manifest/derivation/generator), stale `tamper/006` forward-reference
  removed, TR-CORE-023 added to `tamper/006`.
- `f69f9e4` — symmetric TR-CORE-023 claim on `tamper/005-chain-truncation`.
- `914f032` — `tamper/007-hash-mismatch` (§19 step 4.d; re-signed tampered
  `author_event_hash`; TR-CORE-020+023+061).
- `dd0d1da` — `tamper/008-malformed-cose` (§19 structural-identification;
  CBOR tag-flip 18→17; TR-CORE-035+060+061; closes TR-CORE-060 from the
  allowlist).

Net allowlist state: `pending_invariants = []`; `pending_matrix_rows` drops
TR-CORE-037 / TR-CORE-070 / TR-CORE-060 / TR-OP-006 (and TR-OP-006 also
drops from `_pending-projection-drills.toml`). 92/92 pytest, check-specs
green (warnings only), renumbering guard green (23 base prefixes). Corpus:
24 vectors.

### Wave 6 (2 commits `992fbc1..6b20ef3`)

4 parallel in-tree agents (no worktrees). Landed `tamper/005-chain-truncation`
(TR-CORE-020, Core §19 step 4.h pin; first expanded-tamper case, `992fbc1`)
and §9.4 HPKE-freshness decision memo recommending Option (a) amendment
(`6b20ef3`, `thoughts/specs/2026-04-19-trellis-hpke-freshness-decision.md`).
Stream D O-5 semi-formal review completed (verdict REQUEST CHANGES, 1
blocking-style finding + 1 nit + 6 notes; Finding 1 is the `tamper_kind`
enum-name drift between `tamper/001` and `tamper/004`). `projection/005`
authoring agent halted cleanly on a Phase-1 extension-registry spec blocker.
3 new spec-amendment tasks surfaced: `tamper_kind` reconcile, register
`trellis.staff-view-decision-binding.v1`, land Core §9.4 HPKE amendment.
89/89 pytest; check-specs + renumbering guard green (18 base prefixes).

### Wave 5 (9 commits `334bb75..552c142`)

4 parallel code-scout agents + working-tree commit-split. Landed:
`shred/002-backup-refusal` (`334bb75`), lint commit 4 R6+R8 including new
`_pending-model-checks.toml` scaffolding (`d9f228a`), Stream D O-5 bundle
`append/006..008` + `tamper/002..004` (`dbdfe0a` + `814b2fe`),
`append/002-rotation-signing-key` rebased from worktree (`4585646`), lint
commit 3 R4-R5/R7 fixture corpora + pre-merge renumbering guard
(`0fcee6f`), Companion A.6 ambiguity amendment (`65090f8`), Wave 5 fixture
batch `append/003` + `projection/002-004` + `tamper/001` (`905668b`),
TODO refresh (`552c142`). Allowlist 2 invariants + 54 rows + 5
projection-drill + 8 model-check; 89/89 pytest; `check-specs.py` +
`check-vector-renumbering.py --base-ref HEAD` green.

### Wave 4 (6 commits `248781f..00042c4`)

`append/005-prior-head-chain` vector (closes #10/#13, TR-CORE-020/023/050/080);
SSDI intake-triage reference O-4 declaration at `fixtures/declarations/`;
first O-3 projection + shred fixtures (Test 1 watermark + Test 4
purge-cascade); Wave 1 lint refactor commits 1-2 (shared plumbing + R1
fixture-naming guard + R3 projection-drills loader, 30→52 pytest); 10
WOS-binding review findings applied.

### Wave 3 (5 commits + 1 review)

`append/005` brainstorm (corrected TODO invariant mislabels; pinned serial
order 005→003→004→002→tamper); Wave 1 lint-refactor plan (6 S-commits
phased); WOS custodyHook ↔ Trellis binding (Core §23 4→8 subsections +
Companion §24 OC-113a/b/c/d/e + Appendix B.2 extensions); G-4 Rust workspace
plan (10 crates, 5 layers, M1 six sub-milestones); F6 deprecation-field
lint + F1/F2/F5/F8 review-nits cleanup.

### Wave 2 spec edits (`cfd587b..1233e02`)

Core §§6.5/6.7/9.8/15.2/15.3/19 (Posture-transition registry,
`trellis-posture-declaration-v1` + `trellis-transition-attestation-v1`
domain tags, `projection_schema_id` reconciliation, dCBOR rebuild encoding,
verification algorithm step 5.5 + `PostureTransitionOutcome`); Companion
§§10.3/16.2/19.9/20.5 + Appendix A.5 (shared `Attestation` rule + A.5.1 /
A.5.2 / A.5.3) + A.6 (Delegated-Compute Declaration + OC-70a mandates) +
A.7 (Cascade-scope enum); Matrix TR-OP-008/042..045 + TR-OP-005/006 flipped;
allowlist promotes #11/#14/#15 to hybrid. Validated through 3 opus-model
`/semi-formal-code-review` cycles; 15 blockers+warnings closed in-patch.

### Wave 1 (done)

4 parallel design-brief agents landed Streams A/B/C/D briefs; consolidated
in `thoughts/specs/2026-04-18-trellis-wave1-consolidation-plan.md`.

---

## Closed sprint-queue items

### Lint / fixture infrastructure — Wave 1 refactor plan (all 6 commits + renumbering guard)

Wave 1 lint-refactor plan: [`thoughts/specs/2026-04-18-trellis-wave1-lint-extension-plan.md`](thoughts/specs/2026-04-18-trellis-wave1-lint-extension-plan.md).

- **Lint-refactor commits 1-2** — shared plumbing + R1 fixture-naming guard
  + R3 projection-drills loader. Landed Wave 4 (30→52 pytest).
- **Lint-refactor commit 3** — R4-R5: projection/shred op dispatch +
  `tr_op` / `companion_sections` coverage lint. Landed Wave 5 working tree
  (`0fcee6f`).
- **Lint-refactor commit 4** — R6-R8: G-2 non-byte verification channels.
  R7 landed Wave 5; R6 spec-cross-ref (warning-not-error on uncited rows;
  non-resolving cites remain hard errors) + R8 model-check evidence (new
  `_pending-model-checks.toml` + `thoughts/model-checks/evidence.toml` path
  convention) landed in `d9f228a`.
- **Lint-refactor commit 5** — R9-R10: O-5 extension registry check + CDDL
  cross-ref. Landed Wave 8 (`b0f114d`).
- **Lint-refactor commit 6** — R11: O-4 declaration-doc Phase 1 static
  validator (frontmatter/schema, posture ref, authorized actions,
  event-registry stub, actor discriminator, runtime enclave, UTC bounds,
  signature table; ledger-replay checks deferred to G-4 Rust). Landed
  Wave 8 (`b0f114d`) with SSDI event-registry stub.
- **Pre-merge renumbering guard** — F6 amendment's complementary rule at
  merge time: `scripts/check-specs.py` enforces lifecycle fields, and
  `scripts/check-vector-renumbering.py` compares the current tree to a
  ratification/base ref to reject deleted or renumbered `<op>/NNN-*` vector
  prefixes. Landed Wave 5 working tree with CLI/git-path tests.
- **R12 verify-report consistency check** — Cross-checks failure-kind
  tokens in verify/* manifests against `[expected.report]` booleans per
  Core §19. Landed `d3af6a8`.
- **Generator `_lib/` extraction** — Shared byte-level plumbing
  centralized in
  [`fixtures/vectors/_generator/_lib/byte_utils.py`](fixtures/vectors/_generator/_lib/byte_utils.py).
  Renamed `gen_verify_002_003.py` → `gen_verify_negative_export_001.py`.
  Landed `b3cb833`.
- **Verify vectors 008 + 009** — Closed Core §19 step 5.d
  (`prev_checkpoint_hash` mismatch) and step 5.e (consistency-proof
  mismatch) with
  [`verify/008-export-001-prev-checkpoint-hash-mismatch`](fixtures/vectors/verify/008-export-001-prev-checkpoint-hash-mismatch)
  and
  [`verify/009-export-001-consistency-proof-mismatch`](fixtures/vectors/verify/009-export-001-consistency-proof-mismatch).
  Extended `gen_verify_negative_export_001.py`; lint + Trellis
  conformance replay pass.
- **Residual V3 breadth closure** — Landed the remaining G-3 fixture batch
  on 2026-04-21: `export/002-revoked-key-history`,
  `export/003-three-event-transition-chain`,
  `export/004-external-payload-optional-anchor`,
  `verify/010-export-002-revoked-key-after-valid-to`,
  `verify/011-export-003-transition-chain`,
  `verify/012-export-004-optional-anchor`,
  `tamper/009-prev-hash-break`, `tamper/010-missing-head`,
  `tamper/011-wrong-scope`, and `tamper/012-registry-snapshot-swap`.
  Also landed the Core §19 revocation-language pin, `trellis-verify`
  support for bundled `PayloadExternal` export members, and distinct
  `prev_hash_break` tamper classification. `python3 scripts/check-specs.py`,
  `cargo test -p trellis-verify`, and
  `cargo test -p trellis-conformance committed_vectors_match_the_rust_runtime`
  all pass, and ratification gate G-3 is now checked.

### First vector batch (G-3) — all 5 landed

Per [`thoughts/specs/2026-04-18-trellis-g3-first-batch-brainstorm.md`](thoughts/specs/2026-04-18-trellis-g3-first-batch-brainstorm.md).
Brainstorm corrected TODO's prior invariant mislabels. Serial order:
005 → 003 → 004 → 002 → tamper/001 (from 005, not 001).

- **`append/005-prior-head-chain`** — invariants #5, #10, #13;
  TR-CORE-020/023/050/080. Landed (`060a547`).
- **`append/003-external-payload-ref`** — invariants #4 + #8 partial + #13.
  `PayloadExternal` variant. Claims TR-CORE-031, -071. Landed (`905668b`).
- **`append/004-hpke-wrapped-inline`** — invariants #4 real + #8 populated
  + #11 latent. Real ChaCha20-Poly1305 payload encryption + HPKE suite-1
  DEK wrap with pinned X25519 recipient/ephemeral keys under `_keys/`.
  Landed Wave 8 (`4cc9fe8`) depending on Core amendments in `ee57780` (§9.4
  HPKE freshness + §6.4 AEAD pin + §9.4 test-vector carve-out). Claims
  TR-CORE-031, -038.
- **`append/002-rotation-signing-key`** — invariant #7 (key-bag immutable
  under rotation; not "key rotation" writ large). Claims TR-CORE-036, -038.
  Landed (`4585646`, rebased onto main from worktree).
- **`tamper/001-signature-flip`** (derived from `append/005`, not 001) —
  verification side; claims TR-CORE-061. Landed (`905668b`).

### `append/` residue batch (critical-path step 2)

Closed by Wave 7 `append/009-signing-key-revocation` (TR-CORE-037 +
TR-CORE-070). `pending_invariants = []` achieved.

### G-2 model-check flush

Closed 2026-04-21. `trellis-conformance` now carries a real model-check
suite at [`crates/trellis-conformance/src/model_checks.rs`](crates/trellis-conformance/src/model_checks.rs)
covering TR-CORE-001 / 020 / 023 / 025 / 046 / 050 and TR-OP-061 / 111.
`thoughts/model-checks/evidence.toml` and the row-specific evidence briefs
under `thoughts/model-checks/` are live, and
[`fixtures/vectors/_pending-model-checks.toml`](fixtures/vectors/_pending-model-checks.toml)
is empty.

### G-4 reference implementation

Closed 2026-04-21. The Rust workspace under `crates/` now provides real
`append`, `verify`, and `export` behavior, a real Postgres backend, and a
conformance harness that replays the committed `append`, `export`, `verify`,
`tamper`, `projection`, and `shred` corpora byte-for-byte. Ratification gate
G-4 is now checked in
[`ratification/ratification-checklist.md`](ratification/ratification-checklist.md).

---

## Closed stream items

### Stream B — O-3 projection discipline

- **`projection/002-rebuild-equivalence-minimal`** — Test 2 / TR-OP-005;
  first fixture exercising Core §15.3's new dCBOR rebuild pin. Landed
  (`905668b`).
- **`projection/003-cadence-positive-height` + `004-cadence-gap`** —
  Test 3 / TR-OP-008. Landed (`905668b`).
- **`shred/002-backup-refusal`** — Test 4 backup variant. Landed
  (`334bb75`).
- **`projection/005-watermark-staff-view-decision-binding`** — TR-OP-006 +
  §17.4 Staff-View. Final O-3 breadth item landed Wave 7.

### Stream C — O-4 delegated-compute honesty

- **Companion A.6 amendment to pin ambiguities** — Pinned
  key-absence-as-null for TOML nullable fields,
  `[signature] = {cose_sign1_b64, signer_kid, alg}` shape, and optional
  `audit.registry_ref`. Landed Wave 5 (`65090f8`).

### Stream D — O-5 posture-transition audit

- **Author O-5 fixtures** — All 6 cases landed: `append/006` CM-B→CM-A
  (TR-OP-042/045), `append/007` CM-C narrowing, `append/008`
  disclosure-profile A→B (TR-OP-043/045 + invariant #11), `tamper/002`
  from-state mismatch (TR-OP-044), `tamper/003` missing dual-attestation,
  `tamper/004` declaration-digest mismatch. Commits `dbdfe0a` + `814b2fe`.
  Semi-formal review (Wave 6) returned REQUEST CHANGES — minor, single
  blocking finding.
- **Reconcile `tamper_kind` enum naming** — Landed Wave 8 (`fd54232`):
  `tamper/004` uses canonical `posture_declaration_digest_mismatch`;
  Core §19 `failures[]` remains `declaration_digest_mismatch` (different
  layer).

### Stream E — Track E cross-cutting bindings

- **WOS `custodyHook` ↔ Trellis binding** (vision item 22). Core §23
  (4→8 subsections) + Companion §24 (OC-113a/b/c/d/e) + Appendix B.2
  extensions landed Wave 3C + Wave 4E (10 opus-review findings applied).
  Committed as `248781f`.

### Stream A — G-2 non-byte-testable invariant audit

Design at [`thoughts/specs/2026-04-18-trellis-g2-invariant-audit-paths.md`](thoughts/specs/2026-04-18-trellis-g2-invariant-audit-paths.md).
Hybrid classification; 11 byte-testable, 4 non-byte-only, 5 hybrid
invariants. No authoring tasks — Stream A closes when G-2 audit sign-off
lands.

---

## Wave 6 tail checklist (all closed in Wave 8 unless noted)

- (a) [x] Core §9.4 HPKE freshness + §6.4 AEAD pin → unblocks `append/004`.
  Landed `ee57780`.
- (b) [x] Core §6.7 `trellis.staff-view-decision-binding.v1` + §19
  step 4.k + §28 Appendix A CDDL → retroactively satisfies
  `projection/005`. Landed `ee57780`.
- (c) [x] Reconcile `tamper_kind` enum. Landed `fd54232`.
- (d) [x] Lint-refactor commits 5 (R9-R10) + 6 (R11). Landed `b0f114d`.
- (e) [x] G-3 `append/` residue batch. Landed Wave 7 `964716c`
  (`append/009-signing-key-revocation`).
- (g) [x] Review checkpoints — Wave 7 semi-formal-review cycle run on the
  residue batch (background opus agent); REQUEST CHANGES verdict with
  1 blocker + 2 warnings fixed in-patch (`4ae9d3c`, `f69f9e4`).

**(f) Stream E Respondent Ledger ↔ Trellis binding — still open**; see
[`TODO.md`](TODO.md).

---

## Earlier closed items (pre-Wave 1)

- Core clarifications from T10 — §6.1 (`idempotency_key` uniqueness scope +
  UUIDv7 construction), §7.4 (COSE_Sign1 embedded payload, verifier MUST
  reject `payload == nil`), §9.1 (length-prefix form applies uniformly
  including single-component).
- Allowlist rollout — `TRELLIS_SKIP_COVERAGE=1` bypass removed;
  `_pending-invariants.toml` allowlist drives batched vector coverage
  (F5). `check_vector_manifest_paths` lint rule added (F7). 20/20 pytest
  green.
- Vector-lifecycle policy (F6) — renumbering-forbidden, `status =
  "deprecated"` tombstones, overlap-encouraged-as-boolean. Landed as F6
  amendment in
  `thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md` under
  "Vector lifecycle" + "Manifest schema"; lint enforcement deferred to the
  separate `check-specs.py` follow-on plan.
- Matrix drift for Core §6.8 / §10.6 / §14.6 closed; `append/001` coverage
  updated (`475b064`, `a1eb41f`).
- Working norms encoded in the handoff prompt (`c346313`).
- Ratification-evidence removed; normalization handoff archived (`617f9ae`,
  `28f551c`).
- G-3 scaffold plan (12 tasks, `880ebdd..18c72c8`), Core amendments B1..S5
  (`6ad24ab..e1895ae`), first reference vector (`e1ab065`).
