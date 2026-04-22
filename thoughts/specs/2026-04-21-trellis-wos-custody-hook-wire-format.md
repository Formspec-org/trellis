# Trellis/WOS `custodyHook` wire format

**Status:** Accepted, 2026-04-21 (revised 2026-04-21 after byte-format + identifier-scheme resolution).
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
   `custodyHook` (Phase-1 simplification; the tuple extends cleanly if a later
   phase opens batch appends).
2. The authored bytes are **dCBOR** per Trellis Core §5 encoding rules —
   mechanically derived from JSON-authored, JSON-Schema-validated WOS records
   at the binding seam. The WOS JSON surface is authoring-only; chain bytes
   are dCBOR-native. The encoding table is closed (ADR §2.2); vendor
   extensions register their own encoding rule alongside their WOS Extension
   Registry entry.
3. The stable WOS-side identifiers are **TypeID-structured**:
   `{tenant}_{type}_{uuidv7_base32}`. `caseId` carries `type = case`;
   `recordId` carries a type-prefix registered per record family (`prov`,
   `override`, `aigov`, `assurance`, plus vendor `x-*` extensions). UUIDv7
   provides structural uniqueness; no per-family uniqueness fixture required.
4. The idempotency tuple is **`(caseId, recordId)`** — two fields. The
   domain-separated hash construction is
   `idempotency_key = SHA-256( len_prefix("trellis-wos-idempotency-v1") || dCBOR(input_map) )`
   where `input_map` is the CBOR map `{"caseId": caseId, "recordId": recordId}`
   encoded with dCBOR canonical rules (lexicographic key order; both values
   as CBOR text strings, major type 3, untagged). Matches Trellis §9.1
   length-prefixed domain-separation discipline.
5. `anchor_refs` / anchor target stay **Trellis-owned**; there is no
   per-record WOS `anchorTarget` field.
6. A WOS-governance decision that changes custody posture yields **two**
   canonical facts: the `wos.*` governance record first, then the Trellis
   posture-transition event. The second references the first via
   `canonical_event_hash`; the pair is not transactional, and the deployment
   MUST surface a detectable condition when step 1 commits and step 2 does
   not.
7. The `custodyHook` return is **`{ canonical_event_hash }`** at minimum —
   step 1's hash only. Step 2's hash resolves through a separate
   Trellis posture-transition lookup API, outside the custodyHook contract.
8. WOS records cite one another (intra- and cross-case) via
   `canonical_event_hash`, not via a replicated WOS-side id chain. This
   matches Trellis §23.2 item 4's hash-of-record contract.

---

## Why this matters on the Trellis side

Trellis Core §23 already says:

- WOS provides authored-fact bytes.
- Trellis wraps them unchanged.
- Trellis owns `ledger_scope`, idempotency behavior, chain order, and posture
  transitions.

What was missing was the concrete answer to "which WOS identifiers are stable
enough to feed Trellis idempotency and sidecar catalogs, what byte form does
the authored payload take, and what do deployments receive back?" The joint
ADR answers all three without expanding Trellis envelope scope:

- `recordId` and `caseId` are TypeID-structured — time-ordered, tenant-routable,
  structurally unique. Trellis §14 bound-registry entries reference WOS
  Extension Registry entries at a declared WOS spec version; `eventType`
  registration stays WOS-owned.
- **Authored bytes are dCBOR** (Trellis Core §5 encoding rules). One byte
  oracle across the chain layer — Rust is byte authority per Trellis ADR
  0004, and the same discipline extends through the `custodyHook` seam.
- **Return contract is narrow**: `canonical_event_hash` only. Additional
  return fields are refused until a concrete WOS consumer forces them.

This keeps ADR 0003 intact: no new Trellis envelope fields, no reservation
creep, no Phase-1 runtime widening.

---

## Phase-1 corpus impact

The existing `append/010-wos-custody-hook-state-transition` fixture is
**outside the G-5 allowed-readset** (the Phase-1 stranger-test corpus
terminates at `append/009`). Regenerating it with dCBOR authored bytes and
TypeID-shaped identifiers does not disturb Phase-1 ratification. The fixture
is a demonstration of the WOS-Trellis seam, not a G-5 artifact; it tracks
the ADR and updates when the ADR is accepted.

---

## Follow-on on the Trellis side

1. **Regenerate `append/010`** with:
   - dCBOR authored bytes (replace `input-wos-record.jcs.json` with
     `input-wos-record.dcbor`);
   - TypeID-shaped `caseId` and `recordId` inputs;
   - Two-field idempotency tuple `{ caseId, recordId }`;
   - Domain tag `trellis-wos-idempotency-v1` pinned in the derivation;
   - Updated manifest `description` and `derivation.md`.
2. **Verify Trellis Operational Companion §24.9** against the four-field WOS
   append-input shape and the one-field receipt. If §24.9 currently
   references the earlier 12-field draft or 3-tuple idempotency construction,
   update.
3. **Coordinate acceptance** with the WOS-side acceptance gate in
   [`0061-custody-hook-trellis-wire-format.md`](../../../wos-spec/thoughts/adr/0061-custody-hook-trellis-wire-format.md)
   §5. Trellis-side fixture regeneration does not block on WOS's
   emission-site TypeID wiring (that is WOS-internal) but does block on the
   conversion algorithm being pinned in WOS prose and the round-trip fixture
   corpus landing.
4. **No Core §23 prose changes required.** Core §23 already defers byte form
   and idempotency-tuple shape to WOS; the format + TypeID + domain-tag
   decisions live entirely on the WOS side and in fixture derivations.

---

## Cross-reference map

| Concern | Trellis anchor | WOS ADR anchor |
|---|---|---|
| Authored bytes | §23.2 item 3; §5 encoding | §2.2 |
| `wos.*` namespace | §23.4, §14 | §2.3 `eventType` row |
| Idempotency tuple → key | §23.5, §17.2, §9.1 | §2.4, §2.4.1 |
| Hash-of-record | §23.2 item 4, §9.2 | §2.5 (inter-record refs), §2.8 (return) |
| Posture transitions | §6.7, Operational Companion §10 / §24.10 | §2.6 |
| Payload bounds | §6.4 `PayloadInline` | §2.7 size bound paragraph |
