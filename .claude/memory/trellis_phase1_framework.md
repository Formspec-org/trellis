---
name: Trellis Phase-1 MVP decision framework
description: Load-bearing principles for Trellis architecture, format, and scope decisions. Check before making any non-trivial call in the trellis repo.
type: project
originSessionId: 41ed97d7-a8d5-4ebc-aa79-cd1b32d3ebdd
---
Authoritative doc (read before acting on any non-trivial trellis decision):
`/Users/mikewolfd/Work/formspec/trellis/thoughts/specs/2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md`

Stack-level context lives in `/Users/mikewolfd/Work/formspec/.claude/vision-model.md` (§ Trellis) and `/Users/mikewolfd/Work/formspec/trellis/CLAUDE.md`. Those win on conflict with this memory.

**The seven principles** (paraphrased — doc is canonical):

1. **MVP-deployable first.** Ship the smallest envelope that makes the stranger test meaningful.
2. **Maximalist envelope, restrictive Phase-1 runtime.** Reserve wire-shape capacity for Phases 2–4 in the envelope now; enforce Phase-1 scope with lint + runtime constraints rather than by omitting capacity. Retrofitting wire shape is expensive; reserving fields is cheap.
3. **Nothing is released; tags are snapshots, not freezes.** `v1.0.0` is tagged and G-4 / G-5 are closed, but no production records exist and no users depend on the surface. If an architectural change prevents future debt, make it and retag. The revision window only closes when real adopters show up — and they haven't.
4. **Rust is the byte authority.** For decisions prose can't pin (CBOR ordering, COSE headers, ZIP metadata, Merkle steps) Rust is canonical; Python (`trellis-py/`) is a cross-check that updates to match. Disagreement is a spec-clarification trigger.
5. **Named-seam extension only.** New extensibility lives at declared seams (`suite_id` registry, `SigningKeyEntry`, `anchor_refs`, Respondent Ledger §13, WOS `custodyHook` §10.5, Track E §21). New seams require ADRs.
6. **G-5 stranger is the integrity anchor.** Internal Rust/Python agreement catches typos and intra-team ambiguity; G-5 catches spec ambiguity for an outside implementor. G-5 is closed (45/45 via `trellis-py/`), so the anchor currently holds — any new wire work must preserve it.
7. **DRY/KISS as tie-breakers** — once principles 1–6 don't pick a winner.

**Decided ADRs (accepted and ratified into `v1.0.0`):**
- **ADR 0001** — DAG-capable event topology, Phase-1 single-parent runtime. `priorEventHash: [Hash]` in the envelope; Phase-1 lint requires array length = 1.
- **ADR 0002** — List-form anchors, single-anchor deployment default. `anchor_refs: [AnchorRef]`; operators typically populate one. Substrate choice is adapter-tier (OpenTimestamps default; Trillian / Rekor in scope).
- **ADR 0003** — Core §22/§24 reservations kept in the envelope, locked off in Phase 1. Reserve the optional fields; Phase-1 lint requires they remain absent.
- **ADR 0004** — Rust is byte authority, Python retained as cross-check. Python generators in `fixtures/vectors/_generator/` stay live as the parallel implementation.

**Why:** Owner posture, captured 2026-04-20 and refined after ratification — economic model is `Imp × Debt` under minutes-not-days. Coding, time, and compute are cheap; architectural debt is the only expensive cost. That pushes toward **maximalist-envelope reservation** (cheap now, wire-breaking to retrofit) rather than minimalism-first. Pair with the stack-wide rule that "ratified / v1.0 / pinned" labels never justify refusing architectural change — nothing is released across Formspec, WOS, or Trellis.

**How to apply:**
- On architecture/format/scope questions in the trellis repo: cite the doc, mechanically derive the decision from the principles.
- If a proposal would create architectural debt (weak compromise carried forward because the current tag says "v1.0.0"), push back — tag state is not a reason to preserve a known-weak choice.
- If a proposal would strip reserved envelope capacity in the name of minimalism, flag principle #2 — reservations that preserve Phase-1 lint restriction are cheap and prevent wire-format breaks later.
- If posture changes (real adopters, issued records, deployment commitments), revisit principle #3 before derived decisions.
- Rust/Python byte disagreement → Rust wins, Python updates, and the spec gets a clarification if the ambiguity was load-bearing.
