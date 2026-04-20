# Derivation — `projection/005-watermark-staff-view-decision-binding`

## Header

**What this vector exercises.** The final O-3 breadth fixture (Stream B item
(c) per TODO.md) — TR-OP-006 (projection conformance: watermark presence +
stale-status) on its **staff-view** subtype, plus Companion §17.4 and the
Core §6.7-registered `trellis.staff-view-decision-binding.v1` Phase-1
extension. The vector is structurally parallel to
`projection/001-watermark-attestation`:

* a minimal 2-event canonical chain (`input-chain.cbor`),
* a signed `Checkpoint` at `tree_size = 2` (`input-checkpoint.cbor`),
* a derived view embedding a §15.2 `Watermark` (`input-view.cbor`),
* `expected-watermark.cbor` = `dCBOR(Watermark)`.

On top of that shape it pins the staff-view decision binding:

* the Watermark's `projection_schema_id` = `"trellis.staff-view.v1"` and its
  `rebuild_path` = `"trellis.staff-view.v1/default"` (Companion §17.4
  staff-view identifiers);
* `expected-staff-view-decision-binding.cbor` carries the §28 Appendix A
  `StaffViewDecisionBinding` whose `watermark` field is byte-identical to
  `expected-watermark.cbor`, with `staff_view_ref =
  "urn:trellis:staff-view:test-005/default"`, `stale_acknowledged = false`,
  and `extensions = null`.

**Scope of this vector.** Structural-only for the payload layer — no HPKE
wrap, empty `KeyBag`, opaque `PayloadInline` ciphertext. The staff-view
claim lands on the committed `Watermark` and the `StaffViewDecisionBinding`
artifacts; no event's `extensions` carries the binding key in this fixture
(Core §19 step 4.k's "rights-impacting decision event" path — event-level
binding presence — is deferred to a later `verify/` negative-/positive-split
vector that exercises the full §19 verifier algorithm). The fixture commits
the binding as a standalone artifact so the byte-construction is
self-contained.

**Ledger-scope choice.** `test-staff-view-ledger` is deliberately distinct
from `test-projection-ledger` (001..004) and every append-fixture scope, to
avoid sequence-0 collision per §10.1 / invariant #5.

**Pinned inputs.**

| Input | Value | Source |
|---|---|---|
| `signing_key` | Ed25519 COSE_Key for `issuer-001` | `../../_keys/issuer-001.cose_key` (reused) |
| `ledger_scope` | bstr `"test-staff-view-ledger"` (22 bytes) | §10.4 |
| Event 0 `sequence` / `authored_at` / `idempotency_key` | `0` / `1745020000` / `"idemp-staff-005a"` (16 B) | §6.1, §10.2, §12.1 |
| Event 0 `event_type` | bstr `"x-trellis-test/staff-view-seed"` | §14.6 |
| Event 1 `sequence` / `authored_at` / `idempotency_key` | `1` / `1745020060` / `"idemp-staff-005b"` (16 B) | §6.1 |
| Event 1 `event_type` | bstr `"x-trellis-test/staff-view-follow"` | §14.6 |
| `classification` (both) | bstr `"x-trellis-test/unclassified"` | §14.6 |
| Checkpoint `timestamp` | `1745020120` | §11.2 |
| Watermark `built_at` | `1745020180` | §15.2 |
| Watermark `rebuild_path` | `"trellis.staff-view.v1/default"` | §17.4 |
| Watermark `projection_schema_id` | `"trellis.staff-view.v1"` | §17.4 staff-view URI |
| Binding `staff_view_ref` | `"urn:trellis:staff-view:test-005/default"` | §28 Appendix A |
| Binding `stale_acknowledged` | `false` | §28 Appendix A; Companion §17.3 |
| Binding `extensions` | `null` | §28 Appendix A `{ * tstr => any } / null` |
| `suite_id` | `1` | §7.1 |
| `PayloadInline.nonce` | 12 bytes of `0x00` | §6.4 |

**Core § roadmap (in traversal order).**

1. §9.3 + §9.1 — `content_hash` over each event's 32-byte `PayloadInline.
   ciphertext`.
2. §6.8 (authored) + §9.5 + Appendix A — build each event's
   `AuthorEventHashPreimage`; dCBOR-encode.
3. §9.5 + §9.1 — `author_event_hash` for each event.
4. §6.1 — canonical-form `EventPayload` including the computed
   `author_event_hash`; dCBOR-encode.
5. §7.4 — protected-header map `{1: -8, 4: kid(issuer-001), -65537: 1}`;
   dCBOR-encode. §7.1 Ed25519 over `Sig_structure` built per RFC 9052 §4.4.
6. §6.1 + §7.4 — tag-18 `COSE_Sign1` envelope for each event; commit as a
   dCBOR array `input-chain.cbor`.
7. §9.2 + §11.3 — `canonical_event_hash` for each event (domain-sep tag
   `"trellis-event-v1"`); Merkle leaf hash (`"trellis-merkle-leaf-v1"`);
   Merkle interior hash (`"trellis-merkle-interior-v1"`) for `tree_size =
   2`; result is `tree_head_hash`.
8. §11.2 — build `CheckpointPayload` (`scope`, `tree_size = 2`,
   `tree_head_hash`, pinned `timestamp`, `anchor_ref = null`,
   `prev_checkpoint_hash = null`, `extensions = null`); sign as COSE_Sign1;
   commit as `input-checkpoint.cbor`.
9. §9.6 — `checkpoint_ref = ds_sha256("trellis-checkpoint-v1",
   dCBOR(CheckpointHashPreimage{version=1, scope, checkpoint_payload}))`.
10. §15.2 + §17.4 — build staff-view `Watermark` with
    `projection_schema_id = "trellis.staff-view.v1"` and `rebuild_path =
    "trellis.staff-view.v1/default"`; dCBOR-encode; commit as
    `expected-watermark.cbor`.
11. Build the derived view `{watermark, body: {row_count, schema_id}}`;
    dCBOR-encode; commit as `input-view.cbor`.
12. §28 Appendix A — build `StaffViewDecisionBinding{watermark, staff_view_ref,
    stale_acknowledged, extensions}` with the pinned sub-field values;
    dCBOR-encode; commit as `expected-staff-view-decision-binding.cbor`.
    **Byte claim:** `dCBOR(binding.watermark)` equals
    `expected-watermark.cbor` byte-for-byte.

## Construction-to-matrix-row mapping

* **TR-OP-006** — "Projection conformance tests MUST validate watermark
  presence (fields in Core §15.2 `Watermark`) and stale-status behavior."
  Covered by steps (10) and (12) — the committed Watermark carries every
  §15.2 required field, and the committed StaffViewDecisionBinding carries
  `stale_acknowledged` as a pinned boolean (`false`). A conformance runner
  can byte-compare both artifacts.
* **Companion §17.4** (Staff-View) — covered by the staff-view
  `projection_schema_id` and `rebuild_path`.

## Invariant reproduction checklist

* The `watermark` value decoded out of
  `expected-staff-view-decision-binding.cbor`, when re-encoded as dCBOR,
  equals `expected-watermark.cbor` byte-for-byte.
* `StaffViewDecisionBinding.stale_acknowledged` decodes as the boolean
  `false`.
* `StaffViewDecisionBinding.staff_view_ref` decodes as the RFC 3986 URI
  `"urn:trellis:staff-view:test-005/default"`.
* `Watermark.projection_schema_id` equals `"trellis.staff-view.v1"` —
  the §17.4 staff-view URI.
* `Watermark.tree_head_hash` equals the Merkle root over
  `canonical_event_hash` of every event in `input-chain.cbor` under the
  §11.3 domain-separated RFC 6962-compatible construction.
* `Watermark.checkpoint_ref` equals the §9.6 checkpoint digest over
  `input-checkpoint.cbor`'s decoded `CheckpointPayload`.

## What this vector does NOT cover

* §19 step 4.k event-level decode-and-validate path — carrying the
  `trellis.staff-view-decision-binding.v1` extension inside an
  `EventPayload.extensions` map for a registry-flagged rights-impacting
  decision event; that path lives with the `verify/` suite
  (TODO.md critical-path step 3) once its negative-/positive-split fixtures
  land.
* `stale_acknowledged = true` variant — the Companion §17.3 threshold path
  is a separate tamper / negative case (future
  `tamper/NNN-stale-view-acknowledged` or `verify/negative/…` candidate).
  This vector pins the positive-path boolean only.
* Multiple staff-view decisions in the same chain; one binding is enough
  to exercise the byte surface.

---

**Traceability.** TR-OP-006 coverage (staff-view subtype), Companion §14.1
(watermark obligation), §15.2 (required fields), §17.4 (staff-view schema),
§28 Appendix A (binding CDDL). This fixture is the final item from TODO.md §Stream B
(`projection/005-watermark-staff-view-decision-binding`).
