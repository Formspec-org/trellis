# Trellis Ratification Checklist (Draft)

## Purpose

Define a concrete stopping condition for moving [`../specs/trellis-core.md`](../specs/trellis-core.md) and [`../specs/trellis-operational-companion.md`](../specs/trellis-operational-companion.md) from Phase 1 drafts to ratified normative specs.

The acceptance bar is the **stranger test** from [`../specs/trellis-agreement.md`](../specs/trellis-agreement.md) §10: a second implementor reads Agreement + Core + Operational Companion, then implements `append`, `verify`, and `export` against fixtures without asking which document wins or how to encode a signed byte.

## Global gates

- [ ] **G-1 — Normalization handoff complete.** Every task in [`../thoughts/specs/2026-04-17-trellis-normalization-handoff.md`](../thoughts/specs/2026-04-17-trellis-normalization-handoff.md) Groups A–D is closed. *(evidence: `G-1`)*
- [ ] **G-2 — Invariant coverage.** Every Phase 1 envelope invariant #1–#15 appears as normative MUST text in Core and is cross-referenced from at least one `TR-CORE-*` row. *(evidence: `G-2`)*
- [ ] **G-3 — Byte-exact vectors.** ~50 test vectors under `fixtures/vectors/{append,verify,export,tamper}/` cover every byte-level claim. Every vector reproducible from Core prose alone. *(evidence: `G-3`)*
- [ ] **G-4 — Reference implementation passes.** `trellis-core`, `trellis-cose`, `trellis-store-postgres`, `trellis-store-memory`, `trellis-verify`, `trellis-cli`, `trellis-conformance` build; public API is `append` / `verify` / `export`; every vector passes. *(evidence: `G-4`)*
- [ ] **G-5 — Second implementation byte-matches.** An independent implementation (Python or Go) written by someone who read only the specs produces byte-identical output on every vector. *(evidence: `G-5`)*
- [ ] **G-6 — Lint clean.** `python3 scripts/check-specs.py` reports zero violations across all normative documents. *(evidence: `G-6`)*

## Per-document readiness gates

### [`../specs/trellis-core.md`](../specs/trellis-core.md)

- [ ] **C-1 — Signature model via COSE_Sign1.** Signatures use RFC 9052 `Sig_structure` preimage. No custom signature-zeroing procedure. *(evidence: `C-1`)*
- [ ] **C-2 — Explicit hash preimages.** Every hashed artifact (`author_event_hash`, `canonical_event_hash`, `tree_head_hash`, manifest digest) has a single CDDL-defined preimage structure; domain separation tags defined; ledger scope included in signed material. *(evidence: `C-2`)*
- [ ] **C-3 — Tagged payload references.** `PayloadInline` and `PayloadExternal` variants defined; verifier output reports `structure_verified`, `integrity_verified`, `readability_verified` independently. *(evidence: `C-3`)*
- [ ] **C-4 — Deterministic export.** ZIP layout reproducible via a single `zip -0` invocation over prefix-ordered filenames (`000-`, `010-`, …); local-file-header fields pinned. *(evidence: `C-4`)*
- [ ] **C-5 — Strict-superset semantics normative.** "Strict superset" defined as reserved-extension preservation; Phase 1 verifiers MUST reject unknown top-level fields; `extensions` container reserved in CDDL. *(evidence: `C-5`)*
- [ ] **C-6 — Idempotency identity scope-permanent.** Same key + same payload → same canonical reference; same key + different payload → deterministic rejection; no reuse within ledger scope after TTL expiry. Retry budgets and dedup-store lifecycle are deferred to the Operational Companion. *(evidence: `C-6`)*
- [ ] **C-7 — Agency-log extension points reserved.** §24 extension points reflected in §11 checkpoint CDDL as reserved fields. *(evidence: `C-7`)*
- [ ] **C-8 — Profile/Custody/Conformance-Class vocabulary unambiguous.** No bare "Profile" without scope qualifier; Respondent Ledger owns `Profile A/B/C`; legacy core profiles named "Conformance Classes"; legacy companion profiles named "Custody Models." *(evidence: `C-8`)*

### [`../specs/trellis-operational-companion.md`](../specs/trellis-operational-companion.md)

- [ ] **O-1 — Core section references resolve.** Every `Core §N` reference points to the correct heading in the current Core. *(evidence: `O-1`)*
- [ ] **O-2 — Custody-model identifier set unified.** Companion §9 custody-model identifiers match Core §21 vocabulary and Matrix `TR-OP-010..014` rows. *(evidence: `O-2`)*
- [ ] **O-3 — Projection discipline testable.** Watermark contract, rebuild equivalence, snapshot cadence, and purge-cascade verification have conformance fixtures. *(evidence: `O-3`)*
- [ ] **O-4 — Delegated-compute honesty declarations present.** Every agent-in-the-loop deployment has a declaration document covering scope, authority attestation, audit trail, attribution per Companion §19. *(evidence: `O-4`)*
- [ ] **O-5 — Posture-transition auditability enforced.** Custody-model and disclosure-profile changes are recorded as canonical events per Companion §10. *(evidence: `O-5`)*

### [`../specs/trellis-requirements-matrix.md`](../specs/trellis-requirements-matrix.md)

- [ ] **M-1 — Factual consistency with Core.** TR-CORE-032 specifies dCBOR (not JCS); every MUST in Core has at least one matching `TR-CORE-*` row; every MUST in Companion has at least one matching `TR-OP-*` row. *(evidence: `M-1`)*
- [ ] **M-2 — Gap-log soundness.** Every dropped legacy row is justified against an invariant, an upstream spec, or a replacement `TR-*` row. *(evidence: `M-2`)*
- [ ] **M-3 — Invariant coverage.** All 15 invariants have at least one `TR-CORE-*` row. *(evidence: `M-3`)*

## Natural stopping point

Ratification is complete when all gates above are checked, all handoff tasks are closed, G-5 has landed an independently-written second implementation that byte-matches every vector, and the lint reports zero violations.
