# Core gaps surfaced by G-3 fixture authoring

**Date:** 2026-04-18
**Status:** open — blocks Task 10 of `thoughts/specs/2026-04-18-trellis-g3-fixture-scaffold-plan.md`
**Source:** T10 implementer subagent escalation (NEEDS_CONTEXT) after prerequisite read of `specs/trellis-core.md` §§5, 6, 7, 9, 10, 11, 12, 26, 27, 29, 30 and Appendix A (§28).
**Reviewer independently-flagged-risk:** yes — semi-formal review A Finding 3 ("Task 10 is severely underspecified for its claimed role") predicted this.

## Why this exists

The G-3 ratification bar is the stranger test: an independent implementor reads only Core + Companion + Agreement and produces byte-identical output to the committed vectors. If that claim holds, G-5 can land. If it doesn't, G-5 reduces to "my impl matches yours because we both used the same reference."

Task 10 of the fixture scaffold plan asks an engineer to produce the **first** reference vector (`append/001-minimal-inline-payload`) from Core prose alone. The T10 subagent read Core end-to-end and escalated before writing any bytes: Core is under-specified for byte-exact reproduction in three places, plus ancillary gaps. This is the ratification bar doing its job — surfacing Core bugs now, before G-5 would have surfaced them as an interop failure.

This document catalogues what Core needs to pin before Task 10 can complete. Each gap is framed with what Core currently says, why it blocks byte-exact reproduction, and a recommended amendment. The user (spec author) decides whether to amend Core now, defer T10 until a future pass, or resolve the gap in a fixture-system-level normative annex.

---

## Blocking gaps (cannot produce bytes without resolving)

### B1 — No COSE protected-header label for `suite_id`

**Where:** `specs/trellis-core.md` §7.4 (Signature Profile → "Protected headers").

**What Core says:** §7.4 mandates `alg`, `kid`, `suite_id` in the protected header (and optionally `artifact_type`). RFC 9052 §3.1 registers integer labels for `alg = 1` and `kid = 4`. `suite_id` and `artifact_type` are Trellis-specific and have no registered labels in either RFC 9052's COSE header registry or in Core.

**Why it blocks byte reproduction:** The protected-header CBOR bstr is a hash of the header map. The header map's keys must be pinned integers (or text strings), deterministically serialized. Without a pinned label for `suite_id`, the protected-header bytes are indeterminate → `Sig_structure` is indeterminate → Ed25519 signature is indeterminate → `expected-signed-event.cbor` is indeterminate. A second implementor reading Core would assign a label by guessing and byte-mismatch the first.

**Recommended amendment:** Add a row to §7.4's header table (or a new §7.5 "COSE header label registry") pinning:
- `suite_id` → integer label (suggest a Trellis-reserved negative integer per RFC 9052 §1.4, e.g. `-65537` or similar; exact value at author discretion).
- `artifact_type` → integer label (same style).
- Any other Trellis-introduced headers that may appear.

Pin serialization order of the map keys if dCBOR canonical ordering isn't sufficient — dCBOR orders integer keys by numeric value, which handles this, but state it explicitly.

### B2 — Vocabulary drift vs. current CDDL (blocks the three-file output split)

**Where:** Task 10 text + plan prose cite "AuthoredEvent" / "CanonicalEvent" as distinct CDDL types. Current Core Appendix A (§28) defines `EventPayload`, `AuthorEventHashPreimage`, and `Event = COSESign1Bytes`. There is no `AuthoredEvent` or `CanonicalEvent` CDDL type in the current spec.

**Why it blocks byte reproduction:** The fixture specifies three output CBOR files (`input-authored-event.cbor`, `expected-canonical-event.cbor`, `expected-signed-event.cbor`). Their shapes follow from their names. Current Core does not carry those names. A reasonable mapping is:

- `input-authored-event.cbor` = dCBOR(`AuthorEventHashPreimage`) — the CDDL struct that omits `author_event_hash`.
- `expected-canonical-event.cbor` = dCBOR(`EventPayload`) — the CDDL struct including `author_event_hash`.
- `expected-signed-event.cbor` = `COSE_Sign1` tag-18 wire bytes per `Event = COSESign1Bytes`.

But this mapping is inference, not derivation. A stranger reading only current Core would not reliably arrive at the same file split.

**Recommended amendment (choose one):**

(a) **Spec-side:** Add a paragraph to §6 naming three event surfaces as distinct CDDL-level artifacts:
- The unhashed preimage (`AuthorEventHashPreimage`).
- The canonical pre-sign payload (`EventPayload`).
- The wire form (`Event = COSESign1Bytes`).

Phrase each as "the authored form", "the canonical form", "the signed form" so the G-3 vector filenames become unambiguous. This is probably cleaner.

(b) **Fixture-side:** Update `thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md` and the plan's Task 10 to drop "AuthoredEvent" / "CanonicalEvent" and adopt current Core's CDDL names (`AuthorEventHashPreimage`, `EventPayload`, `Event`). Vector filenames become `input-author-event-hash-preimage.cbor`, `expected-event-payload.cbor`, `expected-event.cbor`. Less intuitive but avoids touching Core.

Recommend (a). The three-surface framing is a real concept that authors will want to refer to across vectors.

### B3 — `expected-next-head.cbor` shape undefined

**Where:** Task 10 cites "Core §11 (canonical_event_hash and next_head chaining)". Current Core §11 is Checkpoint Format (Merkle), not head-chaining. §10.2 defines `prev_hash` of event N+1 as `canonical_event_hash(N)` but does not define a CBOR artifact representing the "head" state between events (post-append, pre-checkpoint).

**Why it blocks byte reproduction:** The fixture expects an `expected-next-head.cbor` file — a byte-exact artifact that an `append` operation returns. If no such artifact is defined in Core, there is no byte shape to reproduce. Candidates:

- Raw 32 bytes: the `canonical_event_hash` itself.
- dCBOR-wrapped bstr (34 bytes): `dCBOR(h'<32-byte hash>')`.
- A small struct: `{scope, sequence, canonical_event_hash}` or similar.

Each yields different bytes. Core picks none explicitly.

**Recommended amendment (choose one):**

(a) **Spec-side:** Define in Core a minimal `LedgerHead` / `AppendHead` CBOR struct that `append` returns. Even if it contains only `{scope: tstr, sequence: uint, canonical_event_hash: bstr}` — this is enough to pin the bytes and give `next_head` a normative home.

(b) **Fixture-side:** The G-3 design declares that when Core does not define a return structure for an operation, the fixture's `expected-*.cbor` file holds the raw bytes of the most specific Core artifact. For `next_head`, that's the 32-byte `canonical_event_hash`. Document this in the fixture system design + plan Task 10.

Recommend (a). Having `append` return a structured head is cleaner for the Rust reference impl (G-4) and for stranger-impl authors who will want a typed return.

---

## Secondary gaps (do not block Task 10 for structural vectors, but narrow the freedom for full-fidelity vectors)

### S1 — `event_type` identifier registration

**Where:** §12.1 refers to `event_type` as a "registered event-type identifier (§14)". §14 is the Registry Snapshot Binding — it binds registry digests but does not define event-type strings.

**Why it matters:** A minimal vector needs a pinned string that does not depend on resolving an external registry. Without a known event-type identifier, the vector must either fabricate one (unreliable) or exercise a code path that does not require an event-type.

**Recommended amendment:** Either add a "reserved for testing" event-type string (e.g., `"x-trellis-test/append-minimal"`) in §12.1 or §14, or document in the fixture system design that test vectors use a pinned string that is not resolvable against any deployed registry and that this is acceptable for structural conformance.

### S2 — `classification` identifier registration

Same issue as S1 but for `classification`. §12.1 refers to "registered classification identifier". Same two resolution paths.

### S3 — `PayloadInline.nonce` length

**Where:** §6.4 defines `PayloadInline.nonce: bstr` with no length constraint. §9.4 pins AEAD = ChaCha20-Poly1305 but does not tie the inline nonce length to the AEAD nonce (12 bytes for ChaCha20-Poly1305).

**Why it matters:** Byte-exact reproduction requires a pinned length. 12 bytes is the sensible default per ChaCha20-Poly1305, but a stranger reading Core could choose any length.

**Recommended amendment:** In §6.4 state `nonce: bstr .size 12` (or whichever size Core actually intends), aligning with §9.4's AEAD choice.

### S4 — `key_bag` entry byte shape for structural-only vectors

**Where:** §9.4 requires a fresh X25519 ephemeral per wrap and an HPKE-sealed DEK for inline payloads.

**Why it matters:** Most early G-3 vectors will test structural conformance (CBOR layout, signing, chaining) rather than HPKE roundtrip. Running real HPKE introduces reproducibility concerns (the HPKE output depends on ephemeral keys, which must themselves be pinned) without adding signal for the invariants being tested.

**Recommended amendment:** Either (a) grant the fixture system design explicit latitude to use opaque pinned bytes for `key_bag` entries in vectors that do not exercise HPKE roundtrip (with a declared `coverage.skips_hpke: true` flag or similar), or (b) require all vectors to pin full HPKE material (committed pinned X25519 ephemerals, real sealed-DEK bytes). (a) is vastly cheaper; (b) is more honest. The G-3 design should commit to one.

### S5 — `kid` construction

**Where:** §8.3 permits either derivation from `SHA-256(suite_id || pubkey)` or administrative assignment.

**Why it matters:** If derivation is used, the concatenation order and the byte encoding of `suite_id` (single byte? CBOR uint? ASCII?) determine the `kid`. If administrative, the vector needs to pin a specific `kid` value.

**Recommended amendment:** Pin the derivation byte-encoding (probably: `SHA-256(cbor_encode(suite_id_integer) || pubkey_raw_32_bytes)`), or declare that the reference vector uses administrative assignment with `kid = h'issuer-001-…'` pinned explicitly. The latter is simpler for a first vector.

---

## Section-numbering drift in the scaffold plan

The plan cites Core §§6, 7, 8, 11 for the constructions Task 10 exercises. The T10 subagent verified these do not match current Core:

| Plan cite | Plan says | Current Core §N actually is |
|-----------|-----------|-----------------------------|
| §6 | "AuthoredEvent and CanonicalEvent CDDL" | Event Format (`EventPayload`, structure) |
| §7 | "author_event_hash preimage + domain separation" | Signature Profile (`suite_id`, Ed25519, `Sig_structure`) |
| §8 | "COSE_Sign1 signing via RFC 9052 Sig_structure" | Signing-Key Registry — NOT signing procedure. Signing is §7.4. |
| §9 | (not cited in plan) | Hash Construction (`author_event_hash` at §9.5; `canonical_event_hash` at §9.2) |
| §11 | "canonical_event_hash / next_head chaining" | Checkpoint Format (Merkle tree) — NOT canonical hash or head chaining. Those are §9 + §10. |

Plan must be updated once B1-B3 are resolved so future readers of Task 10 aren't pointed at the wrong sections. This is a documentation fix, not a Core spec change.

---

## Recommended action (decision for user)

Two paths, not mutually exclusive:

**Path 1 — Amend Core now, then retry Task 10.** Apply B1 (pin `suite_id` label), B2 (name the three event surfaces), B3 (define a `LedgerHead` struct or equivalent). This is the cleanest ratification outcome: Core is tightened; the first vector proves Core is byte-reproducible; subsequent batches inherit the same discipline. Effort: ~half a session of careful spec authoring. Risk: other implicit ambiguities may surface during subsequent vectors, each requiring another round.

**Path 2 — Defer Task 10, push forward on parallel work.** Land the lint hardening (in flight) and the design+plan amendments (in flight), then let the remaining Trellis tracks proceed without a reference vector. G-3 stays open. This unblocks the wider project but leaves the "reproducible from prose alone" claim untested in practice.

If Path 1 is chosen, **Path 2's parallel work is a no-op blocker** — the lint hardening and design amendments are good hygiene regardless. They do not depend on Core amendments.

Recommend Path 1. The entire purpose of the Trellis ratification gates is to force exactly this kind of surfacing at the cheapest possible time, and the cheapest time is now, with zero deployed implementations.

---

## Impact on the ratification checklist

None of these gaps flip any currently-green gate red. They clarify that:

- **G-3** is open (as before), and its specific stopping condition is now: Core amended per B1-B3, first reference vector authored, byte-identical on rerun, all Phase 1 invariants with `test-vector` rows covered.
- **G-2** (invariant coverage audit) is partially folded into G-3 per the amended design, but non-byte-testable invariants remain to be audited separately. No material change.
- **G-4** (Rust reference impl) remains blocked on G-3 vector corpus, which is blocked on B1-B3.
- **G-5** (stranger test) inherits the same block chain.

---

## Cross-references

- Fixture system design: `thoughts/specs/2026-04-18-trellis-g3-fixture-system-design.md`
- Scaffold plan: `thoughts/specs/2026-04-18-trellis-g3-fixture-scaffold-plan.md` (Task 10)
- Ratification checklist: `ratification/ratification-checklist.md` (G-3)
- Phase 1 envelope invariants: `thoughts/product-vision.md` "Phase 1 envelope invariants (non-negotiable)"
- Core spec: `specs/trellis-core.md` §§6, 7, 9, 10, 11, 12, 28
