# Trellis/WOS `custodyHook` wire format

**Status:** Draft, 2026-04-21.  
**Scope:** Mirror the WOS-side ADR for the `custodyHook` seam so Trellis Core
§23 and Operational Companion §24.9 stop leaving the WOS-owned append surface
implicit.

**Primary ADR:** [`../../../wos-spec/thoughts/adr/0061-custody-hook-trellis-wire-format.md`](../../../wos-spec/thoughts/adr/0061-custody-hook-trellis-wire-format.md)

---

## Trellis-side read of the joint decision

The wire-format split is:

- **WOS owns** the authored record bytes and the identifiers that make those
  bytes stable as a governance fact.
- **Trellis owns** canonical append, chain order, checkpoints, posture
  declarations, and anchor policy.

Concretely:

1. A WOS runtime routes **one authored WOS record per append** through
   `custodyHook`.
2. The authored bytes are **JCS-canonical UTF-8 JSON**, not a WOS-internal
   Trellis-shaped object.
3. The stable WOS-side idempotency tuple is
   **`(caseRef, eventType, recordId)`**.
4. `anchor_refs` / anchor target stay **Trellis-owned**; there is no per-record
   WOS `anchorTarget` field.
5. A WOS-governance decision that changes custody posture yields **two**
   canonical facts: the `wos.*` governance record first, then the Trellis
   posture-transition event.

---

## Why this matters on the Trellis side

Trellis Core §23 already says:

- WOS provides authored-fact bytes.
- Trellis wraps them unchanged.
- Trellis owns `ledger_scope`, idempotency behavior, chain order, and posture
  transitions.

What was missing was the concrete answer to "which WOS identifiers are stable
enough to feed Trellis idempotency and sidecar catalogs?" The joint ADR answers
that without expanding Trellis envelope scope:

- `recordId` is the WOS stable authored-fact identity.
- `wosRecordKind` is the WOS-native discriminator surfaced to the sidecar.
- `recordDigestSha256` is a WOS content digest only; it is **not**
  `canonical_event_hash`.

This keeps ADR 0003 intact: no new Trellis envelope fields, no reservation
creep, no Phase-1 runtime widening.
