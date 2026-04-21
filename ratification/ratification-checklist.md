# Trellis Ratification Checklist (Draft)

## Purpose

Define a concrete stopping condition for moving [`../specs/trellis-core.md`](../specs/trellis-core.md) and [`../specs/trellis-operational-companion.md`](../specs/trellis-operational-companion.md) from Phase 1 drafts to ratified normative specs.

The acceptance bar is the **stranger test** from [`../specs/trellis-agreement.md`](../specs/trellis-agreement.md) §10: a second implementor reads Agreement + Core + Operational Companion, then implements `append`, `verify`, and `export` against fixtures without asking which document wins or how to encode a signed byte.

**This file is the evidence-of-record.** Each gate carries inline commit SHAs and artifact pointers. Tactical work needed to close open gates is tracked in [`../TODO.md`](../TODO.md). A separate `ratification-evidence.md` registry existed briefly as a parallel view; it was removed because the inline evidence pointers here are sufficient and the duplication drifted.

## Global gates

- [x] **G-1 — Normalization handoff complete.** Every task in [`../thoughts/archive/specs/2026-04-17-trellis-normalization-handoff.md`](../thoughts/archive/specs/2026-04-17-trellis-normalization-handoff.md) Groups A–D is closed. *(evidence: 3a143a1)*
- [x] **G-2 — Invariant coverage.** Every Phase 1 envelope invariant #1–#15 appears as normative MUST text in Core and is cross-referenced from at least one `TR-CORE-*` row. Byte-testable invariants are audited via the G-3 lint (`check_invariant_coverage`); non-byte-testable invariants are covered by the model-check registry, declaration-document validator, projection/shred drill coverage, and matrix cross-reference lint. *(evidence: matrix §4 invariant summary; `thoughts/model-checks/evidence.toml`; `crates/trellis-conformance/src/model_checks.rs`; `fixtures/declarations/ssdi-intake-triage/`; `fixtures/vectors/{projection,shred}/`; `scripts/check-specs.py` R7/R8/R11; `python3 scripts/check-specs.py` passed cleanly on 2026-04-21 after the remaining `spec-cross-ref` warning rows gained explicit `Core §N` / `Companion §N` anchors.)*
- [x] **G-3 — Byte-exact vectors.** ~50 test vectors under `fixtures/vectors/{append,verify,export,tamper,projection,shred}/` cover every byte-level claim. Every vector reproducible from Core prose alone. *(evidence: fixture system design `thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md`; 12-task scaffold plan `thoughts/specs/2026-04-18-trellis-g3-fixture-scaffold-plan.md`. 44 vectors now landed across six op-dirs — append/1-9, verify/1-12, export/1-4, tamper/1-12, projection/1-5, shred/1-2. The residual V3 breadth batch on 2026-04-21 landed `verify/008-012`, `export/002-004`, and `tamper/009-012`, including the §19 step-4 revoked/`valid_to` branch, step-6 posture-transition happy path, and step-8 optional-anchor happy path. All G-3 coverage allowlists are closed (`_pending-projection-drills.toml` removed, `_pending-invariants.toml` removed, `_pending-matrix-rows.toml` removed, `_pending-model-checks.toml` emptied). Core gaps surfaced by G-3 authoring are documented at `thoughts/specs/2026-04-18-trellis-core-gaps-surfaced-by-g3.md`, and the revocation-language pin landed in Core §19 step 4.a. Validation passed on 2026-04-21 via `python3 scripts/check-specs.py`, `cargo test -p trellis-verify`, and `cargo test -p trellis-conformance committed_vectors_match_the_rust_runtime`.)*
- [x] **G-4 — Reference implementation passes.** `trellis-core`, `trellis-cose`, `trellis-store-postgres`, `trellis-store-memory`, `trellis-verify`, `trellis-cli`, `trellis-conformance` build; public API is `append` / `verify` / `export`; every vector passes. *(evidence: Rust workspace under `crates/`; `cargo test -p trellis-types -p trellis-cddl -p trellis-cose -p trellis-core -p trellis-store-memory -p trellis-store-postgres -p trellis-export -p trellis-verify -p trellis-conformance -p trellis-cli`; committed-corpus replay in `crates/trellis-conformance/src/lib.rs`; model-check suite in `crates/trellis-conformance/src/model_checks.rs`.)*
- [ ] **G-5 — Second implementation byte-matches.** An independent implementation (Python or Go) written by someone who read only the specs produces byte-identical output on every vector. *(evidence: `G-5`)*
- [x] **G-6 — Lint clean.** `python3 scripts/check-specs.py` reports zero violations across all normative documents. *(evidence: 3a143a1)*

## Per-document readiness gates

### [`../specs/trellis-core.md`](../specs/trellis-core.md)

- [x] **C-1 — Signature model via COSE_Sign1.** Signatures use RFC 9052 `Sig_structure` preimage. No custom signature-zeroing procedure. *(evidence: 3a143a1)*
- [x] **C-2 — Explicit hash preimages.** Every hashed artifact (`author_event_hash`, `canonical_event_hash`, `tree_head_hash`, manifest digest) has a single CDDL-defined preimage structure; domain separation tags defined; ledger scope included in signed material. *(evidence: 3a143a1)*
- [x] **C-3 — Tagged payload references.** `PayloadInline` and `PayloadExternal` variants defined; verifier output reports `structure_verified`, `integrity_verified`, `readability_verified` independently. *(evidence: 3a143a1)*
- [x] **C-4 — Deterministic export.** ZIP layout reproducible via a single `zip -0` invocation over prefix-ordered filenames (`000-`, `010-`, …); local-file-header fields pinned. *(evidence: 3a143a1)*
- [x] **C-5 — Strict-superset semantics normative.** "Strict superset" defined as reserved-extension preservation; Phase 1 verifiers MUST reject unknown top-level fields; `extensions` container reserved in CDDL. *(evidence: 3a143a1)*
- [x] **C-6 — Idempotency identity scope-permanent.** Same key + same payload → same canonical reference; same key + different payload → deterministic rejection; no reuse within ledger scope after TTL expiry. Retry budgets and dedup-store lifecycle are deferred to the Operational Companion. *(evidence: 3a143a1)*
- [x] **C-7 — Agency-log extension points reserved.** §24 extension points reflected in §11 checkpoint CDDL as reserved fields. *(evidence: 3a143a1)*
- [x] **C-8 — Profile/Custody/Conformance-Class vocabulary unambiguous.** No bare "Profile" without scope qualifier; Respondent Ledger owns `Profile A/B/C`; legacy core profiles named "Conformance Classes"; legacy companion profiles named "Custody Models." *(evidence: 3a143a1)*

### [`../specs/trellis-operational-companion.md`](../specs/trellis-operational-companion.md)

- [x] **O-1 — Core section references resolve.** Every `Core §N` reference points to the correct heading in the current Core. *(evidence: 3a143a1)*
- [x] **O-2 — Custody-model identifier set unified.** Companion §9 custody-model identifiers match Core §21 vocabulary and Matrix `TR-OP-010..014` rows. *(evidence: 3a143a1)*
- [x] **O-3 — Projection discipline testable.** Watermark contract, rebuild equivalence, snapshot cadence, and purge-cascade verification have conformance fixtures. *(evidence: design brief `e895920`; projection + shred fixture batches `00042c4`, `334bb75`, `905668b`, `964716c`; fixtures under `fixtures/vectors/{projection,shred}/`; committed corpus replayed by `cargo test --workspace` via `trellis-conformance` `tests::committed_vectors_match_the_rust_runtime` on 2026-04-21.)*
- [x] **O-4 — Delegated-compute honesty declarations present.** Every agent-in-the-loop deployment has a declaration document covering scope, authority attestation, audit trail, attribution per Companion §19. *(evidence: template/design brief `b40e8a4`; Companion A.6 normative text `8069062` + `65090f8`; reference declaration corpus under `fixtures/declarations/ssdi-intake-triage/` landed in `7d47c3e`; static validator `R11` landed in `b0f114d`; `python3 scripts/check-specs.py` and `python3 -m pytest scripts/test_check_specs.py` passed on 2026-04-21, including `TestDeclarationDocs`.)*
- [x] **O-5 — Posture-transition auditability enforced.** Custody-model and disclosure-profile changes are recorded as canonical events per Companion §10. *(evidence: design brief `f94342b`; normative Companion posture-transition text + Appendix A.5 schemas in `8069062`; append posture-transition vectors `dbdfe0a`; tamper posture-transition vectors `814b2fe`; tamper-kind reconciliation `fd54232`; full committed corpus replayed by `cargo test --workspace` on 2026-04-21, including the append/006-008 and tamper/002-004 fixtures.)*

### [`../specs/trellis-requirements-matrix.md`](../specs/trellis-requirements-matrix.md)

- [x] **M-1 — Factual consistency with Core.** TR-CORE-032 specifies dCBOR (not JCS); every MUST in Core has at least one matching `TR-CORE-*` row; every MUST in Companion has at least one matching `TR-OP-*` row. *(evidence: 3a143a1)*
- [x] **M-2 — Gap-log soundness.** Every dropped legacy row is justified against an invariant, an upstream spec, or a replacement `TR-*` row. *(evidence: 3a143a1)*
- [x] **M-3 — Invariant coverage.** All 15 invariants are covered by at least one `TR-CORE-*` row, except invariant #11 (Profile-namespace disambiguation) which is covered by Matrix §4 prose. *(evidence: 3a143a1; wording refined in a later commit to reflect #11's §4 routing accurately.)*

## Natural stopping point

Ratification is complete when all gates above are checked, all handoff tasks are closed, G-5 has landed an independently-written second implementation that byte-matches every vector, and the lint reports zero violations.
