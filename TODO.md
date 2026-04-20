# Trellis — TODO

Tactical work list. Concrete, near-term, actionable.

**This file is for:** current tactical state + "next thing we could pick up."
One-liners, each pointing to where the real context lives.

**This file is not for:** strategy (→ [`thoughts/product-vision.md`](thoughts/product-vision.md)),
ratification scope (→ [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md)),
implementation plans (→ [`thoughts/specs/`](thoughts/specs/)),
or wave history / closed work (→ [`COMPLETED.md`](COMPLETED.md)).

When a TODO grows into a spec-sized effort, move its substance to
`thoughts/specs/…` and replace the entry here with a pointer. When an item
lands, move it to `COMPLETED.md` — this file stays forward-looking.

Size tags: **XS** (≤1h) · **S** (≤1 session) · **M** (≤3 sessions) · **L** (multi-session).

---

## Current state (as of 2026-04-20; snapshot — see `git log`)

- **Gates:** 15 closed (G-1/G-6, C-1..C-8, O-1/O-2, M-1..M-3); 7 open — see table below. G-2 / O-3 / O-4 / O-5 all have normative spec anchors + fixture coverage; O-3 is fully covered on Phase-1 breadth. Closure blocked on G-3 `verify/` + `export/` + tamper-residue batches, G-2 audit sign-off, and G-4/G-5 implementation evidence.
- **Lint:** green; 99/99 pytest. All six Wave-1 lint rules (R1-R11) live; all `Verification=test-vector` matrix rows are now claimed by ≥1 vector manifest (pending-coverage allowlists removed); all `projection-rebuild-drill` rows are now claimed by ≥1 `projection/` or `shred/` manifest (pending-drill allowlist removed); `_pending-model-checks.toml` 8 rows awaiting G-4 evidence. Pre-merge vector-renumbering guard green.
- **Fixture corpus:** 32 vectors across `append/{001..009}`, `export/{001}`, `verify/{001..007}`, `projection/{001..005}`, `shred/{001,002}`, `tamper/{001..008}`. Reference O-4 declaration at `fixtures/declarations/ssdi-intake-triage/` with R11-resolvable event-registry stub.
- **End-state = Trellis Phase 1 stranger test passes** ([`thoughts/product-vision.md`](thoughts/product-vision.md) §"Phase 1 success criterion"): a stranger writes a second impl from Core + Companion + Agreement alone and byte-matches every vector. Closes when all 7 open gates close + Track A steps 6–9 done. Phase 2–4 explicitly out of scope.

---

## Format lock-in decisions (pre-G-4, pre-corpus-freeze)

Three envelope-structure decisions whose cost profile makes them resolve-now-or-pay-forever: ordinary to settle before G-4 byte-freezes them into the Rust reference implementation and G-5 commits the shape by having a stranger byte-match every vector; format-breaking to change once any Trellis record has been issued under the pinned shape. Each is spec-sized; substance belongs in `thoughts/specs/…` once drafted. Listed above the ratification gates because they gate not just G-4 but the *meaning* of G-5 — after a second implementation has byte-matched a wrong-shaped format, the wrong shape is public.

- [ ] **Event topology — single-parent chain vs multi-parent DAG** — **M**.
      Current Core specifies a single-parent hash chain (`priorEventHash: Hash`); amendments ride on an `amendmentRef` sidecar. DAG form (`priorEventHash: [Hash]`) expresses consolidated adjudications, cross-case merges, and federation composition as structure rather than convention; single-parent cases degenerate to list-of-one and remain operationally identical to the current chain. **Cost shape:** chain retained forever = every consolidation, every merge, every visualization tool pays convention tax that git-model familiarity would cover; chain changed after G-5 = every fixture regenerates, every second-impl rewrites, every issued record strands. Decide-now cost is bounded work; defer-cost is unbounded across retention horizons.

- [ ] **Anchor slot cardinality — `anchor_ref: AnchorRef` vs `List[AnchorRef]`** — **S**.
      Core §11.5 currently reserves one anchor reference per checkpoint. Expanding to a list enables threshold-of-N multi-witness verification (OTS + Rekor-style log + agency Trillian) without a format change when Phase 4 federation lands; list-of-length-1 is operationally identical to the current single-slot for Phase 1 deployments. **Cost shape:** single-slot = every checkpoint permanently tied to its one anchor substrate's survival (Bitcoin cost dynamics, Sigstore continuity, Trillian maintenance lifecycle); "records survive the vendor" silently degrades to "records survive one specific infrastructure provider." List-form is approximately free to carry; single-to-list is format-breaking to retrofit.

- [ ] **Federation extension points — Core §22 / §24 hook reservation** — **M**.
      Stream E names `case ledger` (§22) and `agency log` (§24 extension points). Before G-4 locks the envelope shape, confirm the Phase-1 envelope reserves the hooks cross-case cryptographic references will need: stable content-addressable case-ledger IDs, the reference-to-other-case-ledger field shape, and version pinning for composed heads. **Reserve, do not implement** — Phase 4 federation semantics stay out of Phase-1 scope, but the hooks must exist in the Phase-1 envelope so Phase 4 composition is not a format break. **Cost shape:** reserved-unused = a few optional envelope fields, no runtime effect; unreserved = every applicant moving jurisdictions breaks cryptographic continuity at the boundary, every consolidated adjudication invents a new pattern, and the Phase-4 federation arc becomes a format break.

Each of these is an ADR target. Pairs cleanly with Stream E authoring — the Respondent Ledger ↔ Trellis binding work depends on the same envelope-shape decisions.

---

## Open ratification gates

Tracked in [`ratification/ratification-checklist.md`](ratification/ratification-checklist.md).

| Gate | State | What closes it |
|------|-------|----------------|
| **G-2** Invariant coverage | partial | `pending_invariants = []` since Wave 7. All six Wave-1 lint rules landed. Remaining: G-2 audit sign-off + G-4 evidence artifacts to flush `_pending-model-checks.toml`. |
| **G-3** Byte-exact vectors | partial | 32 committed; requirements-matrix coverage now complete (no pending-coverage allowlists). Remaining surfaces: `verify/` negative-non-tamper tail (revocation/valid_to enforcement needs an explicit §19 pin) (S), `export/` suite expansion (M), tamper residue (four enum rows; two bundle with verify/export manifests). |
| **G-4** Rust reference impl | open | Cargo workspace + `append`/`verify`/`export` API + byte-match on all fixtures. Plan: [`thoughts/specs/2026-04-18-trellis-g4-rust-workspace-plan.md`](thoughts/specs/2026-04-18-trellis-g4-rust-workspace-plan.md). |
| **G-5** Second implementation | open | Independent stranger-test impl (Python or Go) byte-matching every vector, written by someone who read only the specs. |
| **O-3** Projection discipline | open | Conformance fixtures for watermark, rebuild equivalence, snapshot cadence, purge-cascade verification. Phase-1 breadth closed; awaiting G-3 audit sign-off. |
| **O-4** Delegated-compute honesty | open | Declaration documents per Companion §19 for every agent-in-the-loop deployment. R11 static validator live; ledger-replay checks deferred to G-4 Rust. |
| **O-5** Posture-transition audit | open | Canonical events recorded for custody-model / disclosure-profile changes per Companion §10. Phase-1 fixtures landed (`append/006..008` + `tamper/002..004`); awaiting G-4 evidence. |

---

## Critical path (serial, gates the stranger test)

1. ~~First vector batch (G-3 start)~~ — **done.** See [`COMPLETED.md`](COMPLETED.md) §"First vector batch".
2. ~~`append/` residue batch~~ — **done.** `append/009-signing-key-revocation` (Wave 7).
3. **`verify/` suite (G-3)** — **M**.
      Landed: happy-path `fixtures/vectors/verify/001-export-001-two-event-chain/` (§19 steps 1–5, 7 pass) + negative-non-tamper `fixtures/vectors/verify/002-export-001-manifest-sigflip/` (fatal step 2.c) + `fixtures/vectors/verify/003-export-001-missing-registry-snapshot/` (fatal step 3.f) + `fixtures/vectors/verify/004-export-001-unsupported-suite/` (fatal step 2.b) + `fixtures/vectors/verify/005-export-001-unresolvable-manifest-kid/` (fatal step 2.a) + integrity-fail localizable `fixtures/vectors/verify/006-export-001-checkpoint-root-mismatch/` (step 5.c + 7.b) + `fixtures/vectors/verify/007-export-001-inclusion-proof-mismatch/` (step 7.b). Remaining negative-non-tamper coverage holes, annotated by §19 step:
      - step 4.* — per-event verification (4.a signature, 4.b canonical-hash recompute, 4.d author_event_hash, 4.f wrong_scope, 4.h prev_hash break). 4.b/d/h/wrong-scope bundle with the tamper residue in step 5 below; 4.a still needs its own vector.
      - step 5.d — checkpoint chain break (prev_checkpoint_hash mismatch between adjacent checkpoints). No vector yet.
      - step 5.e — checkpoint signature invalid (distinct from 5.c root mismatch). No vector yet.
      - step 6 — revoked/valid_to enforcement (Core §8.2 says Revoked is a hard-reject after valid_to, but Core §19 step 6 does not yet pin the check). Needs both a core-spec pin and a vector.
      - step 8 — consistency proof mismatch between checkpoint pair (1→2 fixture has one consistency record; no negative case).
      `verify/success/` vs `verify/negative/` split deferred per fixture-system design. Bundles two outstanding tamper cases: `wrong_scope` (step 4.f), `registry_snapshot_swap` (step 3.f, fatal).
4. **`export/` suite (G-3)** — **M**.
      First export landed: `fixtures/vectors/export/001-two-event-chain/` (2-event chain; 2 checkpoints; 1→2 consistency proof; full manifest digest bindings). Remaining: additional ZIP determinism edge cases + manifest variants + key-material handling + larger inclusion-proof sets. Per Core §18. Byte-exact ZIP is the acceptance gate.
5. **Expanded `tamper/` suite — residual cases (G-3 close)** — **S** per case.
      Four landed (`tamper/005..008` covering step 4.b/d/h + structural-ID). Remaining: `prev_hash_break` via mutated-bytes + re-sign; `missing_head` (needs checkpoint); `wrong_scope` / `registry_snapshot_swap` (bundle with verify/export manifests).
6. ~~Rust workspace plan (G-4)~~ — **done.** [`thoughts/specs/2026-04-18-trellis-g4-rust-workspace-plan.md`](thoughts/specs/2026-04-18-trellis-g4-rust-workspace-plan.md).
7. **Rust workspace: first-vector byte-match (G-4)** — **L**.
      Build `append`/`verify`/`export` APIs; byte-match `append/001`. Does not need the full corpus. Blocked only by the plan (landed).
8. **Rust workspace: full corpus match (G-4 closed)** — **L**.
      Byte-match every vector in `fixtures/vectors/`. Blocked by step 5 (corpus freeze).
9. **Commission stranger second impl (G-5)** — **L**.
      `trellis-py` or `trellis-go`. Implementor reads only Core + Companion + Agreement (never `_generator/`, never the Rust impl). Byte-matches every vector. Can begin reading specs mid-way through the corpus authoring (steps 3–5) but cannot finish until the corpus is frozen.
10. **Ratification close-out** — **XS** (mechanical).
      Once all 7 gates flip to `[x]` and `python3 scripts/check-specs.py` reports zero violations, update `ratification/ratification-checklist.md` with final evidence SHAs, strike "(Draft)" from `specs/trellis-core.md` and `specs/trellis-operational-companion.md` titles, cut a version tag. Per `ratification/ratification-checklist.md` §"Natural stopping point."

**The frame:** the byte stack (G-3 → G-4 → G-5) is the critical path. Everything off it (G-2 audit sign-off, Track E) runs in parallel.

---

## Parallel streams — open items only

Closed items moved to [`COMPLETED.md`](COMPLETED.md) §"Closed stream items".

### Stream E — Track E cross-cutting bindings

Named in vision §"Next steps → Track E" as closing conditions for the three-tier claim. Not a Phase-1 gate but load-bearing for the full vision claim.

- [ ] **Respondent Ledger ↔ Trellis binding** (vision item 21) — **M**.
      Three parts: (a) promote Formspec Respondent Ledger §6.2 `eventHash`/`priorEventHash` SHOULD → MUST when wrapped by a Trellis envelope; (b) define the **case ledger** as a top-level object composing sealed response-ledger heads with WOS governance events (Core §22); (c) define the **agency log** as the operator-maintained log of case-ledger heads (Core §24 reserves the extension points). Spec extension across Core §22 + new Core §24 content, not a nesting note. Requires Formspec spec edits — coordinate with the Formspec side before authoring.

---

## Next waves

- **Wave 9 (next):** G-4 Rust workspace execution per the plan at [`thoughts/specs/2026-04-18-trellis-g4-rust-workspace-plan.md`](thoughts/specs/2026-04-18-trellis-g4-rust-workspace-plan.md). Not agent-friendly at L scale. Vector corpus (`verify/`, `export/`, expanded `tamper/`) continues in parallel sessions as Rust progresses.
- **Wave 10:** commission G-5 stranger test once corpus is frozen. Parallel streams should all have closed by this point or be in fixture-authoring tail.
- **Merge:** ratification close-out (step 10) is trivial once all streams merge back.

**Velocity estimate:** serial execution ≈ 7–9 months wall-clock. Parallelized per above ≈ 4–6 months, bounded by the critical path (G-3 corpus → G-4 full match → G-5 stranger). Parallel streams finish inside the G-3/G-4 window with weeks to spare.

---

## Parallel tracks (not blocked by Trellis ratification)

Tracks B (WOS runtime + Formspec coprocessor), C (FedRAMP / SOC 2 / GSA / WCAG certification clocks), and D (reviewer dashboard, document storage, webhooks, notifications) run independently of Track A. Detail in [`thoughts/product-vision.md`](thoughts/product-vision.md).

---

**Closed work:** [`COMPLETED.md`](COMPLETED.md) has the wave-by-wave dispatch history (Waves 1–8), closed sprint-queue items, and closed stream items.
