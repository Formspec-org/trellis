# Trellis — TODO

Tactical work list. Concrete, near-term, actionable.

**This file is for:** current tactical state + "next thing we could pick up."
One-liners, each pointing to where the real context lives.

**This file is not for:** strategy (→ [`thoughts/product-vision.md`](thoughts/product-vision.md)),
ratification scope (→ [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md)),
or implementation plans (→ [`thoughts/specs/`](thoughts/specs/)).

When a TODO grows into a spec-sized effort, move its substance to
`thoughts/specs/…` and replace the entry here with a pointer.

Size tags: **XS** (≤1h) · **S** (≤1 session) · **M** (≤3 sessions) · **L** (multi-session).

---

## Current state (as of 2026-04-18, post-`c346313` unstaged)

- **Gates:** 14 closed (G-1/G-6, C-1..C-8, O-1/O-2, M-1..M-3); 7 open — see table below.
- **Lint:** green; `fixtures/vectors/_pending-invariants.toml` allowlist tracks 61 `TR-CORE-*`/`TR-OP-*` rows + 6 byte-testable invariants pending vector coverage. `TRELLIS_SKIP_COVERAGE=1` retired. Batched vector rollout drives the allowlist to zero.
- **End-state = Trellis Phase 1 stranger test passes** ([`thoughts/product-vision.md`](thoughts/product-vision.md) §"Phase 1 success criterion"): a stranger writes a second impl from Core + Companion + Agreement alone and byte-matches every vector. Closes when all 7 open gates close + Track A steps 6–9 done + Track E bindings landed. Phase 2–4 explicitly out of scope.

---

## Open ratification gates

Tracked in [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md).

| Gate | State | What closes it |
|------|-------|----------------|
| **G-2** Invariant coverage | partial | Byte-testable invariants audited via G-3 lint (`check_invariant_coverage`); non-byte-testable (model-check / declaration-doc-check / spec-cross-ref) still need a dedicated audit pass. |
| **G-3** Byte-exact vectors | partial | ~49 more vectors across `{append, verify, export, tamper}/`. First vector `append/001-minimal-inline-payload` committed. |
| **G-4** Rust reference impl | open | Cargo workspace + `append`/`verify`/`export` API + byte-match on all fixtures. |
| **G-5** Second implementation | open | Independent stranger-test impl (Python or Go) byte-matching every vector, written by someone who read only the specs. |
| **O-3** Projection discipline | open | Conformance fixtures for watermark, rebuild equivalence, snapshot cadence, purge-cascade verification. |
| **O-4** Delegated-compute honesty | open | Declaration documents per Companion §19 for every agent-in-the-loop deployment. |
| **O-5** Posture-transition audit | open | Canonical events recorded for custody-model / disclosure-profile changes per Companion §10. |

---

## Near-term — Sprint queue

### Lint / fixture infrastructure

- [ ] **Lint-enforce F6 deprecation fields** — **XS**.
      `check-specs.py` should validate `status` / `deprecated_at` manifest fields per
      the F6 amendment and exclude `status = "deprecated"` vectors from byte-testable
      coverage. Also: pre-merge guard on the renumbering-forbidden rule. Both flagged
      as Follow-ons in the fixture-system design.
- [ ] **Close review nits from 2026-04-18 patch set** — **XS**.
      Background review findings: F1 (lint silently skips lists in manifest values —
      fail-loud on unknown shape), F2 (absolute/empty-path bypass in
      `check_vector_manifest_paths`), F4 ("ratification branch" wording in F6 amendment —
      Trellis is single-branch on `main`), F5 (rename `pending_tr_core` →
      `pending_matrix_rows`; holds 20 `TR-OP-*` IDs), F3 (§7.4 tag-18 nit),
      F8 (missing tests: unknown-TR-row in allowlist, bad-type in `pending_invariants`).

### First vector batch (G-3)

Each batch is its own plan under `thoughts/specs/…`; brainstorm the set before starting.

- [ ] **`append/002-rotation-signing-key`** — invariant #8 (key rotation). **S**.
- [ ] **`append/003-external-payload-ref`** — invariant #6 (external payload via `PayloadExternal`). **S**.
- [ ] **`append/004-hpke-wrapped-inline`** — real HPKE wrap with pinned X25519 ephemeral
      keypair committed under `_keys/`. Task 10 deferred this per S4. **S**.
- [ ] **`append/005-prior-head-chain`** — non-genesis append, explicit `prev_hash` linkage, invariant #7. **S**.
- [ ] **First tamper vector** — signature-invalid flip in COSE_Sign1 signature bytes →
      verifier reports `integrity_verified=false`. Establishes the tamper-op shape. **S**.

---

## Remaining work to close Phase 1 (end-state map)

One-line pointers; each grows into its own design brief under `thoughts/specs/…` before execution.

### G-2 — non-byte-testable invariant audit

- [ ] **Design: invariant audit-path assignment** — **S**.
      For each invariant #1–#15, assign its audit channel: byte-testable → G-3 vector,
      or non-byte-testable → model-check / declaration-doc-check / spec-cross-ref /
      projection-rebuild-drill / manual-review. Record the channel and a pointer to
      the evidence per invariant. Closes G-2 alongside the G-3 lint's byte-testable half.
      Companion §§10 / 19 / 22 are the likely anchors for non-byte paths.

### G-3 — vector corpus completion

Beyond the first-batch 5 above, the corpus targets ~50 total vectors. Plan one batch at a time.

- [ ] **Remaining `append/` batches for uncovered byte-testable invariants** — **M**.
      The 6 uncovered byte-testable invariants (`_pending-invariants.toml` today):
      #3 signing-key registry Active/Revoked lifecycle;
      #6 registry-snapshot binding (manifest digest of domain registry);
      #7 `key_bag` immutable under rotation (`LedgerServiceWrapEntry` append-only);
      #8 redaction-aware commitment slots reserved;
      #10 Phase 1 envelope = Phase 3 case-ledger event format (structural superset);
      #13 append idempotency retry semantics (same key + different payload → rejection).
      Target: `pending_invariants = []`. Note the first-batch 5 (above) cover #6/#7/#8/#13
      partially; this batch closes the residue.
- [ ] **`verify/` suite** — **M**.
      Happy-path verification: structure / integrity / readability all pass. Plus
      negative-non-tamper (expired key, suite-unsupported, missing registry snapshot).
      Split `verify/success/` vs `verify/negative/` deferred per fixture-system design
      "Open items."
- [ ] **`export/` suite** — **M**.
      Deterministic ZIP layout, manifest shape, key-material handling, inclusion-proof
      shape. Per Core §"Export package layout." Byte-exact ZIP is the acceptance gate.
- [ ] **Expanded `tamper/` suite** — **S** per case.
      Beyond first sig-flip: truncation, event reorder, missing head, malformed COSE,
      wrong-scope, stale `prev_hash`, registry-snapshot swap, checkpoint divergence.

### G-4 — Rust reference implementation

- [ ] **Plan: `trellis-*` Cargo workspace** — **S** (plan only).
      Brainstorm + write the implementation plan before coding. Crate split:
      `trellis-core`, `trellis-cose`, `trellis-store-memory`, `trellis-store-postgres`,
      `trellis-verify`, `trellis-conformance`, `trellis-cli`. Per Track A step 7.
- [ ] **Build workspace + first-vector byte-match** — **L**.
      Milestone 1: `append` / `verify` / `export` APIs; byte-match `append/001`.
- [ ] **Full corpus match → G-4 closed** — **L**.
      Milestone 2: byte-match every vector in `fixtures/vectors/`. Closes G-4.
- [ ] **CLI + WASM bindings** — **M**.
      Track A step 8. `trellis verify | append | export` + WASM for browser-side
      respondent-facing verification. Same workspace as G-4.

### G-5 — stranger-test second implementation

- [ ] **Commission second impl (`trellis-py` or `trellis-go`)** — **L**.
      Track A step 9. Implementor reads only Core + Companion + Agreement (never
      `_generator/`, never the Rust impl). Passes every vector byte-for-byte.
      Closes G-5. Ratification (all gates + close-out) follows once G-2 / O-3 /
      O-4 / O-5 have also closed.

### O-3 — projection discipline

- [ ] **Design: projection conformance fixtures** — **S** (design brief).
      Per Companion §"Projections." Watermark `(tree_size, tree_head_hash)` attestation;
      rebuild-equivalence drill (replay from canonical chain must reproduce derived
      view); snapshot cadence; purge-cascade verification. Pick format (TOML manifest
      like G-3 vs something richer) and coverage enforcement.
- [ ] **Author O-3 fixtures** — **M**.
      Once design lands.

### O-4 — delegated-compute honesty

- [ ] **Design: declaration-doc template** — **S**.
      Per Companion §19. Template for "what an agent-in-the-loop deployment declares
      about its autonomy, authority, and audit surface." One worked example (e.g., an
      LLM-assisted triage tier). Declaration-doc-check is one of the non-byte-testable
      audit channels.
- [ ] **Author one reference declaration doc** — **S**.

### O-5 — posture-transition audit

- [ ] **Design: posture-transition event schemas** — **S**.
      Per Companion §10. Canonical CBOR/JSON shape for custody-model transitions
      (e.g., provider-readable → reader-held) and disclosure-profile transitions.
      Emitted as ordinary ledger events; verification semantics pinned.
- [ ] **Author O-5 fixtures** — **S**.
      Coverage via `tamper/` + `append/` test vectors that exercise the transitions.

### Track E — cross-cutting bindings (three-tier coherence)

Not Phase 1 gates, but named in vision §"Next steps → Track E" as closing conditions for the three-tier claim. Core already reserves §22 (Composition with Respondent Ledger) and §23 (Composition with WOS `custodyHook`) as anchor sections.

- [ ] **Respondent Ledger ↔ Trellis binding** (vision item 21) — **M**.
      Three parts: (a) Formspec Respondent Ledger §6.2 `eventHash`/`priorEventHash`
      SHOULD → MUST when wrapped by a Trellis envelope; (b) define the **case ledger**
      as a top-level object composing sealed response-ledger heads with WOS governance
      events; (c) define the **agency log** as the operator-maintained log of case-ledger
      heads (Core §24 reserves the extension points). This is a spec extension across
      Core §22 + new Core §24 content, not a nesting note.
- [ ] **WOS `custodyHook` ↔ Trellis binding** (vision item 22) — **S**.
      Flesh out Core §23 + Companion §24 (Workflow Governance Sidecar). Document how
      a WOS runtime uses Trellis as its custody backend without redefining either spec.

### Ratification close-out

- [ ] **Ratify Core + Companion** — **XS** (mechanical).
      Once all 7 gates flip to `[x]` and `python3 scripts/check-specs.py` reports zero
      violations, update `ratification/ratification-checklist.md` with the final evidence
      SHAs, strike "(Draft)" from the titles of `specs/trellis-core.md` and
      `specs/trellis-operational-companion.md`, and cut a version tag. This is the
      natural stopping point per `ratification/ratification-checklist.md` §"Natural
      stopping point."

---

## Parallel tracks (not blocked by Trellis ratification)

Tracks B (WOS runtime + Formspec coprocessor), C (FedRAMP / SOC 2 / GSA / WCAG certification clocks), and D (reviewer dashboard, document storage, webhooks, notifications) run independently of Track A. Detail in [`thoughts/product-vision.md`](thoughts/product-vision.md).

---

## Recently closed

Prune aggressively — `git log` is the real record.

- Core clarifications from T10 — §6.1 (`idempotency_key` uniqueness scope + UUIDv7 construction), §7.4 (COSE_Sign1 embedded payload, verifier MUST reject `payload == nil`), §9.1 (length-prefix form applies uniformly including single-component).
- Allowlist rollout — `TRELLIS_SKIP_COVERAGE=1` bypass removed; `_pending-invariants.toml` allowlist drives batched vector coverage (F5). `check_vector_manifest_paths` lint rule added (F7). 20/20 pytest green.
- Vector-lifecycle policy (F6) — renumbering-forbidden, `status = "deprecated"` tombstones, overlap-encouraged-as-boolean. Landed as F6 amendment in `thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md` under "Vector lifecycle" + "Manifest schema"; lint enforcement deferred to the separate `check-specs.py` follow-on plan.
- Matrix drift for Core §6.8 / §10.6 / §14.6 closed; `append/001` coverage updated (`475b064`, `a1eb41f`).
- Working norms encoded in the handoff prompt (`c346313`).
- Ratification-evidence removed; normalization handoff archived (`617f9ae`, `28f551c`).
- G-3 scaffold plan (12 tasks, `880ebdd..18c72c8`), Core amendments B1..S5 (`6ad24ab..e1895ae`), first reference vector (`e1ab065`).
