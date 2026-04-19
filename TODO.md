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

## Current state (as of 2026-04-18, `c346313`)

- **Gates:** 14 closed (G-1/G-6, C-1..C-8, O-1/O-2, M-1..M-3); 7 open — see table below.
- **Lint:** green with `TRELLIS_SKIP_COVERAGE=1`; without bypass, 61 `TR-CORE-*` rows and 7 byte-testable invariants remain uncovered. Batched vector rollout and the allowlist replacement close these.

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

## Near-term

### Lint / fixture infrastructure

- [ ] **Replace `TRELLIS_SKIP_COVERAGE=1` with `_pending-invariants.toml` allowlist** — **S**.
      Small change to `scripts/check-specs.py`: remove the three bypass early-returns;
      load `fixtures/vectors/_pending-invariants.toml`; fail both on missing-and-not-listed
      and on listed-but-now-covered (forces list cleanup). Preserves ratification signal
      during batched rollout. Design rationale: amended fixture-system design F5.

- [ ] **Add lint rule: manifest input/expected paths must resolve** — **XS**.
      Review finding A/F7. Current lint accepts manifests referencing `.cbor` siblings
      that don't exist. Add `check_vector_manifest_paths` that verifies every path in
      `[inputs]` / `[expected]` resolves to a real file in the vector directory.

- [ ] **Vector-lifecycle policy** — **S**.
      Review finding A/F6. No story for deprecation, renumbering-forbidden rule, or
      overlap policy between vectors. Short amendment to the fixture-system design.

### Next vector batches

- [ ] **`append/002-rotation-signing-key`** — invariant #8 (key rotation). **S**.
- [ ] **`append/003-external-payload-ref`** — invariant #6 (external payload via `PayloadExternal`). **S**.
- [ ] **`append/004-hpke-wrapped-inline`** — real HPKE wrap with pinned X25519 ephemeral
      keypair committed under `_keys/`. Task 10 deferred this per S4. **S**.
- [ ] **`append/005-prior-head-chain`** — non-genesis append, explicit `prev_hash` linkage, invariant #7. **S**.
- [ ] **First tamper vector** — signature-invalid flip in COSE_Sign1 signature bytes →
      verifier reports `integrity_verified=false`. Establishes the tamper-op shape. **S**.

Each batch is its own plan under `thoughts/specs/…`. Brainstorm the set before
starting — canary invariants may get reshuffled once Task-10's surface area is
exercised by one or two more vectors.

### Rust reference implementation

- [ ] **Stand up `trellis-*` Cargo workspace** — **L**.
      Per Core-spec Track A step 7. Public API: `append`, `verify`, `export`.
      Consumes `fixtures/vectors/` as the test corpus. Byte-matching the first
      reference vector alone is a legitimate first milestone; full corpus match
      closes G-4. Independent of remaining vector batches.

### Residual Core clarifications surfaced by T10

- [ ] **COSE_Sign1 payload embedded vs detached** — **XS**.
      T10 picked embedded (payload inside tag-18 4-array at position 3). §7.4
      doesn't explicitly name this. One-paragraph Core clarification.
- [ ] **`idempotency_key` value-level discipline** — **XS**.
      §6.1 pins size `(1..64)` but not value semantics. T10 used ASCII
      `"idemp-append-001"` arbitrarily. Pin uniqueness / derivation rule.
- [ ] **§9.1 domain-separation length-prefix for single-component preimages** — **XS**.
      T10 inferred the generic `len(tag) || tag || len(component) || component` form
      applies unchanged to single-component cases. Core could state this explicitly.

---

## Parallel tracks (not blocked by Trellis ratification)

Tracks B (WOS runtime + Formspec coprocessor), C (FedRAMP / SOC 2 / GSA / WCAG certification clocks), and D (reviewer dashboard, document storage, webhooks, notifications) run independently of Track A. Detail in [`thoughts/product-vision.md`](thoughts/product-vision.md). Companion conformance fixtures (O-3 / O-4 / O-5) need their own design pass and run parallel to G-4.

---

## Recently closed

Prune aggressively — `git log` is the real record.

- Matrix drift for Core §6.8 / §10.6 / §14.6 closed; `append/001` coverage updated (`475b064`, `a1eb41f`).
- Working norms encoded in the handoff prompt (`c346313`).
- Ratification-evidence removed; normalization handoff archived (`617f9ae`, `28f551c`).
- G-3 scaffold plan (12 tasks, `880ebdd..18c72c8`), Core amendments B1..S5 (`6ad24ab..e1895ae`), first reference vector (`e1ab065`).
