---
name: Trellis Phase-1 MVP decision framework
description: Load-bearing principles for Trellis architecture, format, and scope decisions. Check before making any non-trivial call in the trellis repo.
type: project
originSessionId: 41ed97d7-a8d5-4ebc-aa79-cd1b32d3ebdd
---
Authoritative doc (read before acting on any non-trivial trellis decision):
`/Users/mikewolfd/Work/formspec/trellis/thoughts/specs/2026-04-20-trellis-phase-1-mvp-principles-and-format-adrs.md`

**The seven principles** (paraphrased — doc is canonical):

1. **MVP-deployable first.** Ship the smallest envelope that makes G-5 meaningful.
2. **Zero Phase-1-shape records issued before G-5.** Creates a cheap-revision window.
3. **"Format-breaking" is lifecycle-dependent.** Free pre-G-5; moderate post-G-5 pre-issuance; expensive post-issuance. Minimalism dominates while in the free tier.
4. **Rust is the byte authority.** Python generators retire as Rust reaches op-level parity; fixtures become Rust-computed output from declarative inputs.
5. **Minimalism over reservation.** No Phase-2/3/4 envelope hooks until the use case is real.
6. **G-5 stranger is the integrity anchor.** Internal impl agreement catches typos, not spec ambiguity.
7. **DRY/KISS as tie-breakers.**

**Decided ADRs (per the doc, pending user validation as of 2026-04-20):**
- ADR 0001 — single-parent event chain (not DAG).
- ADR 0002 — single-slot anchor (not list); OTS recommended substrate.
- ADR 0003 — no Core §22/§24 federation reservations.
- ADR 0004 — retire Python generators; Rust CLI + declarative inputs replace them.

**Why:** User posture confirmed 2026-04-20 — "zero records before G-5, PROD-MVP fast, G-5 ASAP, DRY/KISS while maintaining integrity, Python generators were AI-introduced and not endorsed." This moves Trellis out of the usual "format is forever" regime and into a cheap-revision window where minimalism + revise-if-needed dominates reserve-because-cheap.

**How to apply:**
- On architecture/format/scope questions in the trellis repo: cite the doc, mechanically derive the decision from the principles.
- If the user proposes a maximalist / reservation-heavy change, flag principle #5 (minimalism over reservation) before executing and ask whether the posture has changed.
- If posture changes (records being issued, deployment accelerating past G-5), revisit the principles before derived decisions.
- When the doc is promoted from Draft → Accepted, strike "Draft" language in references to it.
- TODO.md's Blocker section collapses to a pointer into this doc once the ADRs are accepted.
