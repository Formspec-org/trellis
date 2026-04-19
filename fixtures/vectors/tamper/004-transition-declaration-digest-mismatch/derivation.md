# Derivation — `tamper/004-transition-declaration-digest-mismatch`

## Header

**What this vector exercises.** Core §19 step 6.c's declaration-digest-mismatch branch. The tampered export ships a Posture Declaration whose recomputed SHA-256 under the `trellis-posture-declaration-v1` domain tag does NOT match the transition event's `declaration_doc_digest` field. §19 step 6.c names this explicitly as tamper evidence — it sets **both** `declaration_resolved = false` AND `continuity_verified = false` for the affected transition, and appends `declaration_digest_mismatch` to that outcome's failures list. The latter drops `integrity_verified` via §19 step 9.

This is the vector that realizes Decision 1 of the O-5 design brief (`thoughts/specs/2026-04-18-trellis-o5-posture-transition-schemas.md`): co-publish is not a free assertion — the co-published declaration must actually hash to the digest the transition pinned. An operator who edits the declaration text after publishing the transition event cannot hide the edit: any single-byte change in the declaration preimage propagates through SHA-256 and surfaces at step 6.c.

**Scope.** Tamper of a single custody-model transition event's declaration binding. `from_custody_model = "CM-B"` matches the initial declaration so step 6.b passes on its own. Attestations are complete (both `prior` and `new`, valid signatures) so step 6.d passes. The failure is isolated to step 6.c — and then propagates back into `continuity_verified` per §19 step 6.c's explicit instruction.

**Core / Companion § roadmap.**

1. Core §6.7 / Companion Appendix A.5.1 — event shape.
2. Companion §11.3 (OC-15) — the co-publish rule.
3. Core §9.8 — `trellis-posture-declaration-v1` domain tag definition.
4. Core §19 step 6.c — the verifier's digest-recompute-and-compare check, and the explicit tamper-evidence branch.
5. §19 step 9 `integrity_verified` conjunction.

---

## Body

### Step 1 — Commit the initial posture declaration

Pins `custody_model = "CM-B"`. Committed as `input-initial-posture-declaration.bin` (301 B). The event's `from_custody_model = "CM-B"` matches; step 6.b passes on its own merit.

### Step 2 — Build the *intended* declaration (the one the digest was computed over)

This is declaration A — the one the event's `declaration_doc_digest` pins. The operator presumably published it at the moment the transition was admitted. It is committed as `intended-posture-declaration.bin` (358 B) for human review only; a conforming verifier does not see this file, it sees only the shipped declaration in `input-posture-declaration.bin`. The intended declaration's SHA-256 under `trellis-posture-declaration-v1` is:

`declaration_doc_digest = SHA-256(domain_preimage("trellis-posture-declaration-v1", <intended_declaration_bytes>)) = 97612494014c2c574541db8a951b1c4e2425e307b55c12d4571a30f95065974b`.

This value is embedded in the transition event.

### Step 3 — Build the *shipped* declaration (the tamper site)

Declaration B — the one the export actually ships. It differs from declaration A in exactly one field: `posture_honesty_statement` text is mutated (`"test fixture post-transition posture declaration"` → `"test fixture SUBSTITUTED posture declaration"`). Every other field is byte-identical. Committed as `input-posture-declaration.bin` (354 B).

The shipped declaration's recomputed digest under the same domain tag is:

`SHA-256(domain_preimage("trellis-posture-declaration-v1", <shipped_declaration_bytes>)) = 5bf9d9dcc5ab10fd4c9b67d71c6edc0969a2dd22c590d811cde22c4e02e84dce`.

This does not equal `97612494…65974b`. Step 6.c observes the mismatch.

### Step 4 — Full attestations (both classes)

The transition advertises a widening (CM-B → CM-A). Both attestations are present and valid so step 6.d passes.

### Step 5 — Assemble the transition payload and re-sign

The event's `declaration_doc_digest` is the digest of the *intended* declaration (declaration A) from Step 2. Field-by-field:

| Field | Value |
|---|---|
| `transition_id` | `"urn:trellis:transition:test:tamper-004"` |
| `from_custody_model` | `"CM-B"` |
| `to_custody_model` | `"CM-A"` |
| `effective_at` | `1745000600` |
| `reason_code` | `3` |
| `declaration_doc_digest` | `97612494…65974b` (intended A's digest; does NOT match shipped B) |
| `transition_actor` | `"urn:trellis:principal:test-operator"` |
| `policy_authority` | `"urn:trellis:authority:test-governance"` |
| `temporal_scope` | `"prospective"` |
| `attestations` | 2-element array (prior + new). |
| `extensions` | `null` |

Re-signed end-to-end with issuer-001. `canonical_event_hash` of the tampered event = **`952347b52bec17cbba1a85763c28c37124262f2ac97278b64c0b168e5c1a7c27`**.

### Step 6 — Ledger + signing-key registry

`input-tampered-ledger.cbor` = `dCBOR([<tag-18 event>])` (1380 B). `input-signing-key-registry.cbor` (133 B, byte-identical to the other tampers).

### Step 7 — Pin `[expected.report]`

Core §19 step 6.c, verbatim:

> "If the referenced declaration is present but its recomputed digest does not equal `declaration_doc_digest`: record `declaration_resolved = false`, also set `continuity_verified = false`, and append `declaration_digest_mismatch` to the outcome's failures list. Digest mismatch is tamper evidence; the fatality path is the `continuity_verified = false` conjunct in step 9."

| Field | Value | Reasoning |
|---|---|---|
| `structure_verified` | `true` | Event decodes cleanly. |
| `integrity_verified` | `false` | Step 9 sees `continuity_verified = false` (set by step 6.c). |
| `readability_verified` | `true` | No payload decryption attempted. |
| `tamper_kind` | `"declaration_digest_mismatch"` | tamper/001's enum. |
| `failing_event_id` | `952347b5…5c1a7c27` | Tampered event's `canonical_event_hash`. |

The `PostureTransitionOutcome` carries: `continuity_verified = false` (forced by step 6.c), `declaration_resolved = false`, `attestations_verified = true`, `failures = ["declaration_digest_mismatch"]`.

---

## Invariant → byte mapping

| Invariant / capability | Where in this vector's bytes |
|---|---|
| Core §9.8 `trellis-posture-declaration-v1` domain tag | Used to compute both the intended digest (Step 2) and the verifier's recomputation (Step 3). |
| Core §19 step 6.c digest-mismatch tamper-evidence branch | Recompute over `input-posture-declaration.bin` yields `5bf9d9dc…02e84dce`; event's `declaration_doc_digest` is `97612494…65974b`. Mismatch. |
| Decision 1 co-publish rule | The operator's claim that they co-published the declaration is falsified by the shipped bytes hashing to a different digest. |
| §19 step 9 `integrity_verified` conjunction | Drops via the `continuity_verified = false` conjunct that step 6.c forces. |

## Core-gap notes

None. Step 6.c is unambiguous and names the tamper-evidence branch explicitly.
