# ADR 0006 — Key-Class Taxonomy and Wire Format

**Date:** 2026-04-24
**Status:** Accepted (pending implementation)
**Supersedes:** —
**Superseded by:** —
**Related:** Core §8 (SigningKeyEntry); ADR 0005 (`key_class` field of `ErasureEvidencePayload` back-plugs into this taxonomy); ADR 0001-0004 (Phase-1 envelope / runtime discipline); STACK.md end-state commitment #5 (custody-honest privacy — identity separation); `specs/archive/cross-reference-map-coverage-analysis.md` §8 (five-key-class gap that motivates this ADR).

## Decision

Trellis generalizes Core §8's `SigningKeyEntry` into a tagged union `KeyEntry` discriminated by a `kind` field, with five concrete variants covering the full key-material taxonomy surfaced by the archived eight-spec family:

- `signing` — signing authority keys (the current `SigningKeyEntry`; verbatim shape preservation)
- `tenant-root` — per-tenant top-level trust anchors
- `scope` — per-`ledger_scope` domain-separation keys
- `subject` — per-subject/per-data-principal keys protecting subject-scoped payload material
- `recovery` — recovery authorities (no signing authority; activation-only)

Phase-1 CDDL lands all five variants as envelope-reserved shapes in Core §8. Phase-1 lint requires any registry entry to carry `kind = "signing"` (the only class whose runtime semantics are live in Phase 1). Phase-2+ custody models (CM-D threshold, CM-F client-origin sovereign) and Phase-3+ case-ledger scoping activate the remaining variants. **Wire evolution:** introducing the `KeyEntry` discriminator (and any non-flat encoding) is **not** byte-identical to today's flat `SigningKeyEntry` CBOR rows — acceptable only while no production records exist; see *Wire preservation* below.

The rejected alternative — parallel registries (one per class) in Core §8 — is declined because it multiplies registry surface, complicates cross-class queries (e.g., "every key that signed anything attributable to subject X"), and creates migration hazards when a Phase-2+ custody model needs to reference keys of more than one class. Single tagged-union with a stable discriminator is the standard Trellis extension pattern (compare `EventPayload.extensions` with registered identifiers, `trellis.custody-model-transition.v1` vs `trellis.disclosure-profile-transition.v1`, etc.).

## Context

### What Core §8 already has

Core §8 defines exactly one key-material type: `SigningKeyEntry`. Its shape (§8.3 `kid` derivation, §8.4 `SigningKeyStatus` enum: `Active` / `Rotating` / `Retired` / `Revoked`, §8.5 registry snapshot in export, §8.6 `LedgerServiceWrapEntry` for LAK rotation) covers signing authorities comprehensively for Phase-1 scope. No other key class has a CDDL type.

The HPKE infrastructure (Core §9.4) uses recipient pubkeys for payload-key wrap, but those pubkeys are opaque bytes in `KeyBagEntry` today; they are not registered in a typed registry. That works for Phase-1 structural tests but breaks down as soon as a deployment needs to:

- Track a subject-scoped recipient key through rotation and destruction.
- Attest that a destruction event (ADR 0005) destroyed a specific wrap key by its registered `kid`.
- Express threshold recovery under CM-D where a recovery-only key authorizes key reconstruction without itself having signing authority.
- Declare client-origin sovereign keys under CM-F where the adopter (not the operator) authors the key material.

### Why archive preserved five classes

The 2026-04-23 provenance analysis surfaced the gap: the archived eight-spec family separated key classes with distinct lifecycles. The consolidation into Core + Companion preserved the signing-class lifecycle but dropped the others. Rebuilding the taxonomy inside the current envelope is cheap now (wire reservation only); retrofitting it after a Phase-2+ adopter needs the expressiveness is wire-breaking.

### Why one tagged union vs five sibling types

The discriminator pattern carries three Trellis-native advantages:

1. **Single registry surface.** One `KeyEntry` array in the export manifest covers all classes. Verifiers iterate once, dispatch on `kind`. Parallel registries force "look in all N places" behavior and create inter-registry-consistency obligations.
2. **Cross-class references work.** An `ErasureEvidencePayload` (ADR 0005) referencing `kid_destroyed` resolves that kid via one lookup; it does not need to know in advance which class the kid belongs to. A per-class registry forces the lookup to try every class.
3. **Matches the `EventPayload.extensions` pattern.** Trellis already routes custody-model-transition vs disclosure-profile-transition events through a single `extensions` map keyed by a registered identifier (Core §6.7). Key classes applying the same discriminator pattern are idiomatic; siblings would be a different pattern for no gain.

## Wire shape

### Wire preservation (correction 2026-04-24)

An earlier draft claimed the signing class could nest legacy fields inside an `attributes` map and remain **byte-for-byte** identical to current `SigningKeyEntry` registry bytes. That is **false**: any extra map nesting or a new top-level `kind` key changes dCBOR map length and key ordering versus existing vectors.

**Normative position for execution:**

1. Treat migration from flat `SigningKeyEntry` to discriminated `KeyEntry` as an **explicit registry-snapshot wire evolution** (pre-release acceptable; retag vectors after migration).
2. Prefer a **flat signing arm** if the CDDL group can express it: top-level keys for `kind`, `kid`, `suite_id`, and all current `SigningKeyEntry` fields, with **no** `attributes` wrapper for `kind = "signing"`. Non-signing classes use `attributes: KeyAttributes` as class-specific payload.
3. `append/031-key-entry-signing-lifecycle` (and friends) prove **semantic** parity with today's signing lifecycle, not bitwise reuse of old registry CBOR without re-issuance.

### CDDL

Core §8 gains a generic `KeyEntry` type replacing the current hardcoded `SigningKeyEntry` as the registry element.

#### Normative `kind`-to-shape binding

A bare CDDL alternation of attribute maps (e.g. `KeyAttributes = A / B / …`) does **not** force a single CBOR map to satisfy exactly one arm: validators need a dispatch rule tied to `kind`. **Normative rule:** after decoding the registry entry map, the verifier reads `kind` and **then** applies exactly one of: (1) the flat signing field set (*Wire preservation*), or (2) the `attributes` map whose inner shape matches that `kind`, or (3) for a registered extension `kind` (a `tstr` outside the five reserved literals), the CDDL and prose registered for that identifier.

**CDDL execution shape:** Core SHOULD express this as two top-level alternatives so tooling and the stranger test stay aligned, for example:

```cddl
; Registry admits exactly one of these encodings (discriminated by kind + layout).
KeyEntry = KeyEntrySigning / KeyEntryNonSigning

KeyEntrySigning = {
  kind:         "signing",
  kid:          bstr .size 16,
  suite_id:     uint,
  pubkey:       bstr .size 32,
  status:       "active" / "rotating" / "retired" / "revoked",
  valid_from:   uint,
  valid_to:     uint / null,
  supersedes:   bstr .size 16 / null,                ; prior kid in this registry, if any
  attestation:  bstr / null,
  extensions:   { * tstr => any } / null,
}

KeyEntryNonSigning = {
  kind:         "tenant-root" / "scope" / "subject" / "recovery" / tstr,
  kid:          bstr .size 16,
  suite_id:     uint,
  attributes:   TenantRootKeyAttributes / ScopeKeyAttributes /
                SubjectKeyAttributes / RecoveryKeyAttributes / { * tstr => any },
  extensions:   { * tstr => any } / null,
}
```

For reserved literals (`tenant-root` … `recovery`), `attributes` MUST decode as the corresponding structure below (pairing a literal `kind` with the wrong inner map is a structure failure). For extension `kind` strings, `attributes` MUST satisfy the CDDL pinned in that kind's registry row; the `{ * tstr => any }` arm is the CDDL escape hatch—normative tightness lives in the registry + verifier dispatch table.

The fragment below documents the **inner** `attributes` shapes for the four reserved non-signing kinds (and remains the reference for their fields). Extension kinds reuse the same outer `KeyEntryNonSigning` map unless a future registration defines a different top-level layout.

```cddl
TenantRootKeyAttributes = {
  pubkey:           bstr .size 32,                    ; algorithm pinned by suite_id + kind
  tenant_ref:       tstr,
  effective_from:   uint,
  supersedes:       bstr .size 16 / null,
  ; NO status field: tenant-root keys are activation-scoped, not rotating. Supersession replaces them.
}

ScopeKeyAttributes = {
  pubkey:              bstr .size 32,
  scope_ref:           bstr,                          ; ledger_scope byte-string (same as EventPayload.ledger_scope)
  parent_tenant_ref:   tstr,
  effective_from:      uint,
  supersedes:          bstr .size 16 / null,
}

SubjectKeyAttributes = {
  pubkey:            bstr .size 32,
  subject_ref:       tstr,
  authorized_for:    [+ bstr],
  effective_from:    uint,
  valid_to:          uint / null,
  supersedes:        bstr .size 16 / null,
}

RecoveryKeyAttributes = {
  pubkey:                  bstr .size 32,
  authorizes_recovery_for: [+ bstr .size 16],         ; kids of signing keys only; see Field semantics
  activation_quorum:       uint,
  activation_quorum_set:   [+ bstr .size 16] / null,
  effective_from:          uint,
  supersedes:              bstr .size 16 / null,
}
```

**Execution tightening:** the flat `KeyEntrySigning` arm is the executed shape for `kind = "signing"` (no nested `attributes` map). Non-`signing` kinds use `KeyEntryNonSigning` with class-specific material under `attributes`.

### Field semantics and rationale

- **`kind` discriminator.** Closed taxonomy at the envelope layer; `tstr` escape for future registry extensions (Phase-4+ might introduce e.g. `"witness"` or `"federation-delegate"` classes). Registry extension uses the same append-only pattern as Core §6.7 event-type registration.
- **`kid` construction.** Unchanged from Core §8.3 — `SHA-256(dCBOR_encode_uint(suite_id) || pubkey_raw)[0..16]`. The derivation is class-agnostic; any key regardless of class has a kid derivable from its suite + pubkey.
- **`suite_id` and algorithms.** Core §7.2 suite registry entries pin how `pubkey` bytes are interpreted for each `kind` (e.g., Ed25519 for signing and recovery versus X25519 for tenant-root / scope / subject material in the default registry). Verifier MUST reject entries whose `pubkey` length or algorithm does not match the resolved suite for that `kind`.
- **`attributes` sub-map (non-signing only).** After reading `kind`, the verifier selects the matching inner CDDL group and validates `attributes`. Mismatch is a structure failure.
- **`supersedes` chain.** When non-null, `supersedes` MUST equal the 16-byte `kid` of another registry entry this key replaces (same registry array). Acyclicity across supersession edges is a verifier obligation (per the ADR 0005 sibling concern surfaced there).
- **`authorizes_recovery_for` / `activation_quorum_set`.** Each `bstr .size 16` MUST be a `kid` that resolves to a registry row with `kind = "signing"` (recovery re-enables signing authorities, not tenant-root/scope/subject rows). A future ADR MAY widen this set if CM-D requires recovery over other classes.
- **Signing-class variant** — field set isomorphic to current `SigningKeyEntry` (same semantics), encoded per *Wire preservation* above. The legacy `SigningKeyEntry` CDDL name may remain as an alias for the flat `kind = "signing"` arm once Core prose is updated.

### Cross-class references

ADR 0005 `ErasureEvidencePayload.key_class` uses the same literals as `KeyEntry.kind` for the five reserved classes. **Normative name for HPKE / subject-wrap material is `subject`** (ADR 0005 documents the legacy synonym `wrap` for pre-reconciliation artifacts). `kid_destroyed` resolves against the **single** export key registry (ADR 0005 prose updated accordingly).

### Extension injection point

Adopters or future phases can register new key classes via the `tstr` escape on `kind`. The registration process mirrors Core §6.7 event-type registration: append-only, unique identifier per class, declared CDDL shape for the `attributes` variant. Extension classes MUST NOT collide with reserved names (`signing`, `tenant-root`, `scope`, `subject`, `recovery`).

## Verifier obligations (Core §19 step 4 extension)

A conforming verifier resolving a `kid` in the export bundle MUST:

1. **Locate** the registry entry by `kid` in the export-manifest key-entry registry.
2. **Validate** the `kind` field against the closed taxonomy (plus registered extensions).
3. **Dispatch** on `kind` and validate the entry shape: for `"signing"`, validate the flat signing field set; for reserved non-signing kinds, validate `attributes` against the matching CDDL group; for extension `kind` strings, apply the CDDL registered for that identifier—if the verifier implements no row for that `kind`, accept outer-map decodability only and record `unknown_key_class` per *Unknown `kind` and `integrity_verified`* (do not invent inner-field requirements). Mismatch against the applicable rule set is a structure failure.
4. **Enforce class-specific invariants:**
   - `signing`: `valid_from ≤ authored_at ≤ valid_to` (when `valid_to` is non-null) for events signed under this kid; existing §8.4 lifecycle rules.
   - `tenant-root` / `scope`: `effective_from ≤ authored_at` for events claiming this scope.
   - `subject`: `effective_from ≤ authored_at ≤ valid_to` (when non-null) for `KeyBagEntry` wraps referencing this subject-kid; NO wraps MAY reference this kid after `valid_to`.
   - `recovery`: NOT acceptable as a signing kid for any non-recovery event. Recovery-only kids authorize re-enable events (Phase-2+) and fail validation if used to sign ordinary events.
5. **Validate supersession acyclicity** across the per-class supersedes chains.

Phase-1 lint addition: any registry entry whose `kind != "signing"` causes a lint warning (NOT a verifier failure — reservation is valid even when the class isn't yet used). Phase-2+ lifts the lint warning when the relevant custody model activates.

## Phase-1 runtime discipline

Phase-1 runtime restriction (complementary to lint):

- Only `kind = "signing"` entries are populated in practice. The other variants' CDDL shapes exist as envelope reservations; no current adopter emits them.
- `KeyBagEntry.ephemeral_pubkey` + recipient-pubkey path (Core §9.4) continues to use opaque bytes for the recipient; Phase-2+ lifts the Wrap-entry recipient reference to a registered `subject` kid.
- Signing-path wire shape is unchanged (invariant #10 preserved).

## Companion §20 and §27 integration

Companion §20 (lifecycle / retention) gains a paragraph noting that the key-class taxonomy applies to all operator-held key material, not just signing keys. OC-75 through OC-78 and ADR 0005’s **OC-141..OC-145** erasure obligations apply class-agnostic at the policy layer; the cascade-scope enumeration in Appendix A.7 is class-agnostic. Chain-consistency checks in **ADR 0005 §Verifier obligations step 7** remain **Phase-1 signing-bounded** until ADR 0006 extends them (see the Phase-1 scope note indented under ADR 0005 step 7).

Companion §27 (conformance tests) extends:

- **Phase-1 test:** registry may contain only `signing` entries; other variants present cause a lint warning.
- **Phase-2+ test:** per-class lifecycle invariants hold across a committed fixture corpus.

## Adversary model

What this design catches:

- **Class-confusion attacks.** A forged event claiming signature under a recovery-only kid fails class-dispatch validation (step 4 bullet 4).
- **Cross-class kid collisions.** Unique-kid-within-registry is an invariant; the derivation (suite + pubkey) makes collisions cryptographically improbable, and class-discriminated lookup prevents a valid `signing` kid from being silently reinterpreted as a `subject` kid.
- **Unauthorized subject-key reuse.** `subject.valid_to` + "no wraps after `valid_to`" obligation detects a compromised operator continuing to wrap under a subject-kid past its window.
- **Recovery-key misuse.** Recovery-only keys acting as signing keys on ordinary events fails step 4 bullet 4.

What this design does NOT catch:

- **Deliberately-mislabeled class at registration time.** An operator who registers a recovery-authority key under `kind = "signing"` and then signs ordinary events with it passes validation because the verifier trusts the declared class. Mitigation: class declarations are operator attestations; a later audit of the tenant-root / scope / subject / recovery relationships could surface incoherence.
- **Semantic coherence of cross-class references.** The CDDL pins shapes, not semantic consistency (e.g., a `scope` key's `parent_tenant_ref` pointing at a nonexistent tenant-root kid). Optional Phase-2+ lint addition.

## Alternatives considered

### Option A — parallel registries per class (rejected)

Five separate arrays in the export manifest (`signing_keys`, `tenant_root_keys`, `scope_keys`, `subject_keys`, `recovery_keys`). Declined because cross-class kid lookup becomes O(N classes) instead of O(1), verifier code duplicates per class, and the registry-snapshot binding (invariant #3) becomes five bindings instead of one.

### Option B — single `KeyEntry` with `class` as a tstr only (no closed variants) (rejected)

Let operators freely declare arbitrary class names. Declined: violates the closed-taxonomy design principle from the user profile. Opens the door to vendor-extension sprawl at the core registry layer — exactly the anti-pattern this project rejects.

### Option C — defer the non-signing classes until Phase 2+ actually needs them (rejected)

Do not reserve CDDL shapes now; land when adopter presents a need. Declined: wire reservation is cheap; retrofit requires an envelope change that breaks invariant #10 (Phase 1 envelope IS Phase 3 case-ledger event). The whole "maximalist envelope, restrictive Phase-1 runtime" principle from ADR 0001-0004 says this class of change belongs in the envelope now.

## Phase alignment

- **Phase 1 envelope compatible (structural sense).** The manifest still carries one registry array; entries become `KeyEntry`. Signing-only consumers must accept the new discriminant / layout per *Wire preservation* — **not** a claim of bitwise preservation of pre-migration registry bytes.
- **Phase 1 runtime discipline.** Only `signing` entries emitted; lint warns on other classes; reservation is valid but unused.
- **Phase 2 evolution.** CM-D (threshold custody) activates `recovery` + custom quorum-discipline entries. CM-F (client-origin sovereign) activates `subject` entries authored by the adopter.
- **Phase 3 case-ledger composition.** `scope` keys become load-bearing for per-case scope-material separation.
- **Phase 4 federation.** `tenant-root` keys become load-bearing for cross-tenant trust anchors.

Strict-superset preservation (invariant #10): yes. A Phase-1 verifier encountering a Phase-3 export containing reserved non-signing-class entries (`tenant-root`, `scope`, `subject`, `recovery`) dispatches on `kind`, validates structure (CDDL), and applies any class-specific checks that implementation supports.

### Unknown `kind` and `integrity_verified`

When a verifier encounters a registry entry whose `kind` is an **unregistered extension** `tstr` (not one of the five reserved literals and not in the verifier's extension registry):

1. It MUST record `unknown_key_class` (or equivalent) on the verification report.
2. It MUST NOT coerce the entry into a reserved class.
3. It MUST NOT set `integrity_verified = false` **solely** because the class is unknown—forward-compatible structural acceptance mirrors the "unknown extension event type" pattern: the bundle may still be structurally valid.
4. If verification of a **downstream artifact** requires interpreting that `kid` under the unknown class (e.g., validating a signature, HPKE wrap, or erasure-evidence subtree that references it) and the verifier lacks normative rules or crypto for that class, it MUST fail closed for that obligation: set `integrity_verified = false` with an explicit **capability gap** / **unsupported key class** code (same family as "referenced suite not implemented"). Until such a reference appears, unknown `kind` remains informational.

## Fixture plan

Minimum Phase-1 fixture set:

| Vector | Purpose |
|---|---|
| `append/031-key-entry-signing-lifecycle` | Baseline: registry-entry lifecycle for the signing class, **re-generated** under the executed `KeyEntry` CDDL (semantic parity with `append/002-rotation-signing-key`; canonical CBOR bytes are re-pinned, not assumed equal to the legacy flat map). |
| `append/032-key-entry-subject-reservation` | Phase-1 reservation demonstration: a `kind = "subject"` entry exists in the registry; lint warns but verifier accepts. |
| `append/033-key-entry-tenant-root-reservation` | Same for `tenant-root`. |
| `append/034-key-entry-scope-reservation` | Same for `scope`. |
| `append/035-key-entry-recovery-reservation` | Same for `recovery`. |
| `tamper/023-key-class-mismatch-signing-as-recovery` | A `recovery`-class kid signs an ordinary event. Verifier rejects per class-dispatch obligation. |
| `tamper/024-key-entry-attributes-shape-mismatch` | Declared `kind` does not match the encoded shape (e.g., signing arm carries a nested `attributes` map, or non-signing arm omits required inner fields). Structure-failure. |
| `tamper/025-subject-key-wrap-after-valid-to` | A `KeyBagEntry` wrap references a `subject` kid whose `valid_to` has passed. Detectable when the subject-kind classes activate. |

Five positive + three tamper. `append/031` is load-bearing (signing-class semantic parity + new byte pin). Follow-on tamper vectors SHOULD cover unknown `kind` escape, `tstr` class injection, and supersession-cycle violations mirroring Trellis tamper discipline.

## Open questions / follow-ons

1. **Migration plan for the existing signing-key registry.** Deployments that already emit `SigningKeyEntry` need a migration: re-author current registry as `KeyEntry` entries with `kind = "signing"`. Phase-1 is greenfield so no migration pressure today; write the migration notes alongside the first real adopter.
2. **Quorum-discipline CDDL for `recovery`.** The `activation_quorum_set` field is a simple N-of-M reference list. Phase-2+ threshold custody may need richer quorum discipline (e.g., weighted votes, time-staggered authorities). Separate follow-on ADR when CM-D activates.
3. **Cross-class attestation signatures.** A `scope` key's existence might need an attestation from the `tenant-root` key that authorized its creation. Not in the Phase-1 CDDL above. Phase-2+ SHOULD use **top-level `KeyEntry.extensions`** (or `KeyEntrySigning.extensions`) for cross-class metadata and vendor-specific attestation bundles; reserve per-variant detached **signing-style `attestation` bytes** only when reusing the Core §8.7 signing-key attestation pattern literally—avoid a second open-ended extension map nested inside `attributes`.
4. **Key-class query tooling in `trellis-cli`.** A `trellis-cli list-keys --kind subject` or similar would make day-to-day operator work readable. Non-blocking, adopter-driven.

## Cross-references

- **Core §8** — wire-format rewrite. `SigningKeyEntry` becomes a named variant of `KeyEntry`; §8.3 derivation stays class-agnostic.
- **Core §9.4** — HPKE wrap recipient-pubkey path will reference `subject` kids in Phase-2+; Phase-1 remains opaque bytes.
- **ADR 0005 `ErasureEvidencePayload`** — `key_class` literals + `kid_destroyed` registry lookup reconciled with this ADR (`subject` normative, `wrap` legacy synonym; **ADR 0005 §Verifier obligations step 2** normalizes `wrap`→`subject` and binds to `KeyEntry.kind` when the kid resolves).
- **Companion §6.4 Operator role** — operator authors key entries across all classes.
- **Companion §20 lifecycle obligations** — apply class-agnostic; Appendix A.7 cascade-scope enumeration is class-agnostic.
- **STACK.md** commitment #5 custody-honest privacy — identity separation depends on having subject / scope / tenant-root classes expressible in the wire.
- **Archive source** — `specs/archive/cross-reference-map-coverage-analysis.md` §8 surfaced the dropped-without-replacement five-key-class gap this ADR closes.

## Implementation sequencing

1. **Spec** — Core §8 CDDL: `KeyEntry = KeyEntrySigning / KeyEntryNonSigning` (per *Normative kind-to-shape binding*) + inner attribute structs. §8.1 registry-binding prose updated. §8.4 signing lifecycle prose lives on the flat signing arm. §8.3 derivation class-agnostic. Companion §20 class-agnostic note. Companion §27 lint extension.
2. **Rust** — `trellis-types::KeyEntry` + `KeyAttributes` enum. Registry lookup in `trellis-verify` dispatches on `kind`. Phase-1 lint in `check-specs.py`: warn on non-signing entries.
3. **First positive vector** — `append/031-key-entry-signing-lifecycle` pins the signing-class behavior under the executed `KeyEntry` encoding (new golden bytes after migration).
4. **Python stranger mirror** — `trellis-py` verifier extended matchingly.
5. **Reservation vectors** — `append/032..035` (one per non-signing class).
6. **Tamper vectors** — `tamper/023..025`.
7. **ADR 0005 reconciliation** — done in ADR prose: `key_class` includes `subject` and legacy `wrap` (**ADR 0005 step 2** normalizes to `subject` before registry match); `kid_destroyed` + field semantics reference the unified registry.

Steps 1–3 are the minimum for the ADR's claim to hold; steps 4–7 close the corpus.

---

*End of ADR 0006.*
