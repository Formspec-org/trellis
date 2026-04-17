# Trellis Spec Normalization Handoff

**Date:** 2026-04-17
**Scope:** `trellis/specs/trellis-*.md`
**Source context:** `trellis/thoughts/product-vision.md`

## Context

The four top-level Trellis documents are directionally aligned with the product vision:

- `trellis-agreement.md` is the product decision gate.
- `trellis-core.md` should be the Phase 1 byte-level protocol.
- `trellis-operational-companion.md` should be the Phase 2+ operator-obligations spec.
- `trellis-requirements-matrix.md` should trace mined legacy requirements into Core and Companion.

The product vision explains why several decisions that look aggressive in isolation are intentional: Phase 1 must reserve future wire seams, pin byte-exact encoding, bind semantic registries, include idempotency, and carry watermarks because each is cheap now and wire-breaking later.

The remaining problem is not the architecture. The problem is drift between the vision and the generated/spec prose: stale section references, contradictory authority claims, inconsistent versions, and underspecified byte-level constructions.

## Verified findings

The following factual claims were spot-checked against the current files and confirmed:

- **Signature-zeroing scheme is in Core.** `trellis-core.md` lines 252, 297–303, 403, 542, 654, 893, 1401 specify a custom "zero the `signature` field to a fixed-length placeholder during canonicalization" procedure rather than COSE_Sign1's standardized `Sig_structure` preimage. This works but requires every implementation to coordinate on a non-standard convention when RFC 9052 already solves the self-reference problem.
- **Matrix contradicts Core on canonical encoding.** `trellis-requirements-matrix.md` row TR-CORE-032 specifies "JSON Canonicalization Scheme (JCS, RFC 8785) with SHA-256." `trellis-core.md` §5 pins dCBOR, which is what product-vision invariant #1 requires. The matrix is wrong; Core is right.
- **Stale section references in Companion are real.** Core §13 is Commitment Slots (Companion refs §15 — wrong); Core §18 is Export Package Layout (Companion refs §12 — wrong); Core §19 is Verification Algorithm (Companion refs §9 — wrong); Canonical Append Service is a §2 conformance class concept, not Core §7. All confirmed.

## Reasoning

The product vision says Trellis should ship as two W3C-style specs: Core and Operational Companion. The requirements matrix is an input to mine, update, and feed into those specs, not a separate normative source. Two options exist for how the matrix relates to the prose:

1. **Prose-first:** Core and Companion prose is normative; the matrix is pure traceability (every MUST in prose is linked to a `TR-*` row for provenance; prose wins on conflict).
2. **Matrix-first:** The `TR-*` rows are the canonical normative anchors; prose expands them.

Implementors generally find prose-first easier to read. The matrix currently claims every normative obligation is enumerated there, but Core and Companion do not reference `TR-*` rows, and the matrix contradicts Core on canonical encoding. This handoff adopts **prose-first**: matrix is demoted to traceability.

The Phase 1 success criterion is cross-implementation byte identity. That raises the bar for any byte-level text. COSE signing, hash preimages, payload references, manifest signatures, checkpoint signatures, deterministic ZIP layout, and verifier reports must be exact enough that a stranger can produce matching fixtures from the prose alone.

The Operational Companion can remain broad, but every Core reference must point to the intended section or stable requirement anchor. Current raw `Core §...` references are stale and sometimes point to unrelated sections.

## Tasks

Ordered such that byte-protocol decisions land before cosmetic alignment, because the cosmetic fixes cascade once the bytes change.

### Group A — Byte protocol repair (do first)

#### 1. Repair the signature model

- Adopt RFC 9052 COSE_Sign1 directly for events, checkpoints, and manifests.
- Use COSE's `Sig_structure` preimage (`["Signature1", protected, external_aad, payload]`) rather than a zero-fill placeholder.
- Remove the "signature field zeroed to 64 bytes" mechanism and all its supporting text (Core §7.4, §7 examples, §11 checkpoint signing, §18 manifest signing).
- Put `kid`, `suite_id`, and `alg` in COSE protected headers; remove redundant copies from the payload unless they are independently required.
- Ensure verifier pseudocode (§19) reads only fields actually defined by the CDDL.

#### 2. Define non-circular hash preimages

- Define an explicit `HashPreimage` structure for each hashed artifact (`author_event_hash`, `canonical_event_hash`, `tree_head_hash`, manifest digest).
- Ensure `author_event_hash` excludes itself and all signature material by construction (not by zero-fill).
- Ensure `canonical_event_hash` has one authoritative preimage defined in CDDL.
- Include **ledger scope** (response-ledger id, case-ledger id) in signed event material so genesis events and copied ledgers cannot replay across scopes.
- Add domain separation tags between event, checkpoint, and export-manifest hashes.

#### 3. Fix payload reference semantics

- Replace ambiguous `ciphertext_ref: bstr` with a tagged payload reference:
  - `PayloadInline { ciphertext: bstr, nonce: bstr }`, or
  - `PayloadExternal { content_hash: bstr, availability: AvailabilityHint, retrieval_hint: URI? }`.
- Split verifier output into at least:
  - `structure_verified: bool` — CDDL and signature-structure intact,
  - `integrity_verified: bool` — content_hash matches retrieved ciphertext,
  - `readability_verified: bool` — payload decrypted and schema-validates.
- Require exports that omit ciphertext bytes to report which payload checks could not run offline.
- Update §19 verifier pseudocode accordingly.

#### 4. Make ZIP determinism executable

- Resolve the conflict between lexicographic archive order and `manifest.cbor` appearing first.
- Prefer prefixed filenames: `000-manifest.cbor`, `010-events.cbor`, `020-inclusion-proofs.cbor`, `030-signing-key-registry.cbor`, `040-checkpoints.cbor`, `090-verify.sh`, `099-trellis-verify-*`. Single lexicographic order yields the required entry sequence.
- Use `STORED` (compression method 0) only. DEFLATE is not deterministic across libraries unless its parameters are fully pinned; the spec should not open that door.
- Pin local file header fields: extra-field length zero, file-time fixed to a well-known constant, external attributes zero.

#### 5. Clarify Phase 1 to Phase 3 superset semantics

- Define "strict superset" normatively as **semantic and structural preservation via reserved extension fields**, not automatic acceptance of unknown top-level fields by Phase 1 verifiers.
- Keep Phase 1 top-level event fields fixed. Phase 1 verifiers MUST reject unknown top-level fields.
- Reserve explicit extension containers now: `extensions: { * tstr => any }` (CBOR map) with registration discipline so Phase 2+ additions go into reserved slots.
- Make Phase 3 case-ledger heads embed or preserve the Phase 1 checkpoint payload unchanged (byte-for-byte), carrying additional fields only in the reserved extension container.

#### 6. Tighten idempotency

- Make Core idempotency identity **permanent within a ledger scope**.
- Same key + same payload → same canonical reference (deterministic).
- Same key + different payload → deterministic rejection with `IdempotencyKeyPayloadMismatch`.
- Do not allow key reuse within the same ledger scope after any TTL expiry — the identity is scope-permanent.
- Move retry budgets, API TTLs, and dedup-store lifecycle to the Operational Companion. Core defines the identity; Companion defines operator policy.

#### 7. Verify head-format extension points (invariant #12)

- Confirm that Core §24 (Agency Log / Phase 3 Superset Preview) actually reserves head-format extension points that Phase 3 populates. If it does not, add the reservation as CDDL in §11 (Checkpoint Format).
- Spot-check that every "extension added in Phase 2/3" sits in a reserved container, not at the top level.

### Group B — Document hygiene (do once bytes are stable)

#### 8. Normalize document authority

- Declare `trellis-agreement.md` **non-normative** (it imposes obligations on the organization to sign off, not on implementors to conform). Add a one-line Status notice.
- Declare `trellis-core.md` and `trellis-operational-companion.md` the only normative prose specs.
- Demote `trellis-requirements-matrix.md` to **traceability**. Prose wins on any conflict. Link every Core/Companion MUST to a `TR-*` row in the matrix, not the other way around.
- Update `trellis/specs/README.md` so agents read the top-level normalized documents first and treat older split specs (`specs/core/*`, `specs/trust/*`, `specs/export/*`, `specs/projection/*`, `specs/operations/*`, `specs/forms/*`, `specs/workflow/*`, `specs/assurance/*`) as superseded inputs.
- Archive the superseded per-family spec directories per product-vision Track A step 3.

#### 9. Align versions and document-count language

- Use one Core version everywhere, likely `1.0.0-draft.1`.
- Replace Companion references to `Trellis Core v0.1` with the chosen version string.
- Remove "forthcoming" language for the Operational Companion now that it exists.
- Replace "three spec documents" with "an agreement plus two normative specs" (or another single consistent phrase) everywhere the stranger-test is invoked.

#### 10. Replace stale section references

- Replace raw `Core §N` references in the Companion with stable named anchors or requirement IDs.
- Fix the confirmed bad references:
  - Canonical Append Service is a §2 conformance class concept, not Core §7.
  - Commitment Slots are Core §13, not Core §15.
  - Export Package Layout is Core §18, not Core §12.
  - Verification Algorithm is Core §19, not Core §9.
- Add a docs check that fails when a `Core §...` reference does not resolve to the expected heading.

### Group C — Vocabulary and traceability reconciliation

#### 11. Normalize custody and posture vocabulary

- Let the Operational Companion own custody-model semantics.
- Pick one custody-model identifier set, preferably `CM-A` through `CM-F`, and update Core and Matrix to match. Companion §9 owns the canonical list; Core §21 cites it.
- Use **"Posture Declaration"** for the Core export artifact (per Core §20 invariant #15).
- Use "Trust Profile" only if a separate Trust Profiles spec remains normative; otherwise mark it legacy or replace it.
- Verify invariant #11 is consistently applied across all four files:
  - Respondent Ledger owns the letters `Profile A/B/C` (posture axes).
  - Legacy core-draft profiles are renamed **Conformance Classes** (Core §5).
  - Legacy companion-draft Profiles A–E are renamed **Custody Models** (Companion §9).
- Any file still using "Profile" without scope qualification is a bug.

#### 12. Audit matrix gap log for soundness

- Every dropped legacy row in the matrix gap log must be justified against an invariant (`#N`), an upstream spec (Formspec / WOS), or a replacement `TR-*` row — not merely against another legacy ID.
- Spot-check the 23 gap-log entries against invariants #1–#15. If a drop reads "superseded by invariant X" and invariant X does not actually cover the dropped requirement, the drop is unsound and the requirement must be re-instated.
- Confirm the matrix is free of legacy overloaded "Profile" letters anywhere except in the explicit disambiguation table (§4).

#### 13. Correct matrix–Core contradictions

- Fix matrix TR-CORE-032 to specify dCBOR (RFC 8949 §4.2.2 deterministic profile), not JCS. This is a direct factual error.
- Audit every matrix row whose `Legacy` column references a dropped spec family for lingering assumptions from that family.

### Group D — Automation

#### 14. Docs checks

- Anchor-resolution check: every `§N` reference in any Trellis spec resolves to an existing heading with a plausible title.
- `TR-*` coverage check (if retained as traceability): every `TR-*` ID cited in prose exists in the matrix; every matrix MUST has at least one prose anchor.
- Forbidden-term check: flag bare "Profile" without "Posture" / "Custody" / "Conformance Class" prefix; flag any remaining "signature field zeroed" prose; flag any reference to archived per-family spec paths.

## Acceptance Bar

The handoff is complete when a second implementor can read the Agreement, Core, and Operational Companion, then implement `append`, `verify`, and `export` against fixtures without asking which document wins or how to encode a signed byte.

Specifically:

- COSE_Sign1 signing and verification use RFC 9052's `Sig_structure` only — no custom zero-fill. A COSE library is sufficient.
- Every hashed artifact has a single explicit preimage structure defined in CDDL.
- Every payload reference is tagged (inline or external) and the three verifier outputs are reported independently.
- Export ZIP determinism is reproducible with a single command-line `zip -0` invocation over prefixed filenames.
- "Strict superset" means reserved-extension-container preservation, not unknown-field acceptance.
- Profile / Posture / Custody / Conformance Class letters and names are unambiguous at every occurrence.
- Matrix is traceability; prose is normative; contradictions between them are bugs in the matrix.
