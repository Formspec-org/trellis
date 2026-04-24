# ADR 0005 — Cryptographic-Erasure Evidence Format

**Date:** 2026-04-23
**Status:** Accepted (pending implementation)
**Supersedes:** —
**Superseded by:** —
**Related:** Companion §20 (Lifecycle and Retention — OC-75..OC-78); Companion Appendix A.7 (Cascade-Scope Enumeration); Core §6.7 (Extension Registry); Core §9.3 (Hash over ciphertext); Core §18 (Export Package Layout); STACK.md end-state commitment #1 (independent verification) and #5 (custody-honest privacy); `specs/archive/cross-reference-map-coverage-analysis.md` §8 (erasure-evidence gap).

## Decision

Trellis adopts an **explicit, verifier-visible cryptographic-erasure evidence artifact**. Operators who perform cryptographic erasure MUST emit a canonical `trellis.erasure-evidence.v1` event declaring which key was destroyed, at what time, under which cascade scopes, and by which attesting authority. A conforming verifier processing an export bundle MUST decode and check this artifact and cross-reference it with the chain to detect accidental or tampered subsequent use of the destroyed key.

This is the stronger of the two options evaluated (option **B**). The rejected alternative (option **A** — absence of a key-bag entry combined with ciphertext-hash preservation IS implicitly the evidence) is declined because the destruction claim under option A degrades to operator-trust-in-prose, and fails in the DocuSign-replacement scenario where the pre-shutdown operator is no longer available to stand behind it.

## Context

### What Companion §20 already says

Today Companion §20.3–20.5 pin operational obligations:

- **OC-75** — Trellis Core's `content_hash` over ciphertext + HPKE key-bag are the cryptographic mechanics that make crypto-shredding work. Erasure is incomplete until the **purge cascade** completes across every derived artifact holding plaintext or plaintext-derived material.
- **OC-76** — Every derived artifact that holds plaintext or plaintext-derived material subject to a destruction-declaring lifecycle fact MUST be invalidated.
- **OC-77** — The purge cascade MUST reach every class in Appendix A.7 (CS-01 through CS-06); implementations MUST iterate the enumeration programmatically.
- **OC-78** — The Posture Declaration MUST document the operator's crypto-shredding scope.

None of those requires a verifier-visible *evidence* artifact that positively attests the destruction happened. OC-78's Posture Declaration documents *policy*; OC-77 constrains *cascade behavior*. A canonical event that says "I destroyed key K at time T under cascade scope S" does not yet exist in the byte protocol.

### Why absence doesn't carry the claim

For an offline verifier reading an export bundle from a shut-down vendor, option A degrades to: "the ciphertext is unreadable; the operator's retention policy in the Posture Declaration prose says it was destroyed." Three failure modes follow:

1. **Vendor death.** The Posture Declaration's "we will destroy under these conditions" is a future-tense policy statement. When the operator is gone, there is no signed artifact saying the policy was actually executed. The claim is unfalsifiable in either direction.
2. **Accidental re-use detection impossible.** If the operator claims destruction but a later event in the chain wraps data under the supposedly-destroyed kid, option A gives the verifier no artifact to contrast that against. The contradiction is undetectable without an explicit destruction event.
3. **Cascade-scope auditability empty.** OC-77 requires the cascade to reach all of Appendix A.7's classes. Option A carries no evidence of which classes were actually cascaded for any given destruction. An auditor can't distinguish "destroyed and cascaded everywhere" from "destroyed only the key, left the cache behind."

### Why option B fits the existing shape

Trellis already registers posture-change events via `EventPayload.extensions` (Core §6.7 Extension Registry). Custody-model transitions (`trellis.custody-model-transition.v1`) and disclosure-profile transitions (`trellis.disclosure-profile-transition.v1`) both ride this seam. A `trellis.erasure-evidence.v1` event fits the same mold: it is a canonical, signed, chain-visible claim about an operator-policy event, and it composes with the existing verifier loop (Core §19 step 6) alongside posture transitions.

The extension-slot approach carries Phase 1 ⊂ Phase 3 superset preservation (invariant #10) for free: no envelope change, no wire break, no ADR 0003 violation.

## Wire shape

Under `EventPayload.extensions["trellis.erasure-evidence.v1"]`:

```cddl
ErasureEvidencePayload = {
  evidence_id:           tstr,                                ; stable within ledger_scope
  kid_destroyed:         bstr .size 16,                       ; the wrap-key kid from the signing-key registry
                                                              ; or, for non-signing keys, an operator-scoped opaque id
                                                              ; (see §Field semantics)
  key_class:             "signing" / "wrap" / "recovery" /
                         "scope" / "tenant-root" / tstr,     ; tstr permits registered extensions per ADR
                                                              ; (key-class taxonomy ADR, forthcoming)
  destroyed_at:          uint,                                ; Unix seconds UTC
  cascade_scopes:        [+ CascadeScope],                    ; which Appendix A.7 classes the operator declares
                                                              ; cascaded for this destruction; MUST be non-empty
  completion_mode:       "complete" / "in-progress" /
                         "best-effort",                       ; completion state at evidence-emission time
  destruction_actor:     tstr,                                ; principal URI of the agent that performed destruction
  policy_authority:      tstr,                                ; governance authority URI
  reason_code:           uint,                                ; registered reason; 255 = Other
  subject_scope:         SubjectScope,                        ; what the destroyed key protected
  hsm_receipt:           bstr / null,                         ; optional KMS/HSM destruction receipt (opaque)
  hsm_receipt_kind:      tstr / null,                         ; the `ReceiptKind` identifier from §8 registry;
                                                              ; null iff hsm_receipt is null
  attestations:          [+ Attestation],                     ; shared with A.5.x; 1+ required per reason_code
  extensions:            { * tstr => any } / null,
}

CascadeScope = "CS-01" / "CS-02" / "CS-03" /
               "CS-04" / "CS-05" / "CS-06" / tstr           ; tstr permits registry-appended future classes

SubjectScope = {
  kind:            "per-subject" / "per-scope" /
                   "per-tenant" / "deployment-wide",
  subject_refs:    [* tstr] / null,                          ; REQUIRED non-null iff kind = "per-subject";
                                                              ; each a principal URI
  ledger_scopes:   [* bstr] / null,                          ; REQUIRED non-null iff kind = "per-scope";
                                                              ; each a ledger_scope byte-string
  tenant_refs:     [* tstr] / null,                          ; REQUIRED non-null iff kind = "per-tenant"
}
```

`Attestation` reuses the shape defined in Companion Appendix A.5 (shared with custody-model and disclosure-profile transitions). No new attestation shape.

### Field semantics

- **`evidence_id`** — operator-minted stable identifier. Enables idempotent re-emission across retries and cross-reference from audit reports.
- **`kid_destroyed`** — for Phase 1, scoped to signing-key entries in the registry (§8.3 derivation). For wrap keys and non-signing-key classes (once the key-class taxonomy ADR lands), operators use an opaque 16-byte id under their own scheme. The verifier treats the bytes as opaque and only checks consistency, not derivation.
- **`key_class`** — anchored to the (forthcoming) key-class taxonomy ADR. Phase-1 implementations emit `"signing"` or `"wrap"`; the taxonomy ADR will open the other values. Enum carries a `tstr` escape for registry extension.
- **`cascade_scopes`** — a non-empty subset of CS-01..CS-06. An operator who performs crypto-erasure without any cascade is non-conformant under OC-77; this field makes the scope explicit per destruction event rather than only in Posture Declaration policy prose.
- **`completion_mode`** — records the cascade state at emission time. Phase-1 operators should emit `"complete"` or `"in-progress"`. `"best-effort"` is reserved for environments where the operator cannot prove cascade completion (e.g., third-party cache-invalidation API without a signed receipt) and opts to attest partial execution rather than silently over-claim.
- **`subject_scope`** — describes what the destroyed key protected. Disjoint unions on `kind` drive which `*_refs` field is populated. Verifier SHALL validate the cross-field requirement.
- **`hsm_receipt`** — opaque bytes from the KMS/HSM confirming key destruction. Verifier does NOT parse these in Phase 1 — they are operator-supplied evidence for post-hoc human review. `hsm_receipt_kind` tags the format (e.g., `"aws-kms-audit-v1"`, `"pkcs11-destruction-receipt-v1"`); the Phase-1 verifier only checks the null-consistency rule. Values of `hsm_receipt_kind` are **append-only registry strings** (Companion §20 or Core §6.7 follow-on table — same discipline as `CascadeScope`'s `tstr` escape); Phase-1 MAY use a single catch-all such as `"opaque-vendor-receipt-v1"` until the vendor registry in Open questions §2 lands.
- **`attestations`** — at least one attestation required. Operators SHOULD require dual attestation for destruction events that affect data shared across governance boundaries (analogous to the A.5.3 step 4 dual-attestation rule for Widening / Orthogonal posture changes). Specific attestation-count rules per `reason_code` are registered per deployment in the Posture Declaration.

### Reason codes (registered, extensible)

| code | meaning |
|---|---|
| 1 | `retention-expired` (age-based policy fired) |
| 2 | `subject-requested-erasure` (data-subject exercised an erasure right) |
| 3 | `legal-order-compelling-erasure` |
| 4 | `operator-initiated-policy-change` (e.g., dropping support for a data class) |
| 5 | `key-compromise-mitigation` (destruction as part of incident response) |
| 255 | `Other` — rationale documented in Posture Declaration narrative |

## Event-type registration (Core §6.7)

Add to the Core §6.7 Extension Registry:

| event_type | admitted payload | authority boundary |
|---|---|---|
| `trellis.erasure-evidence.v1` | `ErasureEvidencePayload` (this ADR) | Operator-authored; subject to Companion §20 discipline; verifier obligations in Core §19 step 6 extension. |

The existing §6.7 registration table convention (see `trellis.custody-model-transition.v1`, `trellis.signature-affirmations`, `trellis.intake-handoffs.v1`) applies: one row, pinned field reference.

## Verifier obligations (Core §19 step 6 extension)

A conforming verifier processing an export bundle containing `trellis.erasure-evidence.v1` events MUST:

1. **Decode** the payload against the CDDL above. Mismatch is a structure failure (Core §19 step 1).
2. **Validate** `subject_scope` cross-field: exactly one of `subject_refs` / `ledger_scopes` / `tenant_refs` is non-null, and matches the declared `kind`. Mismatch is a structure failure.
3. **Validate** `hsm_receipt` / `hsm_receipt_kind` null-consistency (both null or both non-null). Mismatch is a structure failure.
4. **Verify** every `attestations[*].signature` under `trellis-transition-attestation-v1` domain separation (shared with A.5.3). Invalid signature flips `integrity_verified = false` per Core §19 step 9.
5. **Cross-check chain consistency for the destroyed kid:**
   - Let `destroyed_at` be the `destroyed_at` value.
   - For every canonical event in the chain authored-at > `destroyed_at` whose COSE_Sign1 protected header `kid` equals `kid_destroyed`: mark it as **`post_erasure_use`**. This is a localizable failure per Core §19 step 6.
   - For every canonical event in the chain authored-at > `destroyed_at` whose `key_bag.entries` contains an entry wrapped under `kid_destroyed`: mark it as **`post_erasure_wrap`**. Also localizable.

   **Phase-1 scope (explicit bound; pairs with ADR 0006):** The two checks above are defined for the **signing / HPKE-wrap** surfaces that exist in Phase-1 exports today. They do **not**, until a follow-on verifier milestone lands with ADR 0006, forbid post-`destroyed_at` use of **descendant** material that was encrypted under a destroyed **scope**, **tenant-root**, **subject**, or **recovery** class kid (those registry classes are envelope-reserved but not yet load-bearing in the reference verifier). Erasure evidence for non-`signing` `key_class` values remains wire-valid; normative "forbid the whole subtree" semantics **co-land with ADR 0006** (extend step 5 or add a dedicated subtree walk).
6. **Cross-check cascade scope against export contents:**
   - For each `CascadeScope` entry declared, if the export bundle contains derived artifacts corresponding to that scope (e.g., CS-01 projections in `070-projections/`, CS-03 snapshots in `080-snapshots/`), check that those artifacts do NOT decode the destroyed key's material. Detection is best-effort in Phase 1 (full lint is deferred to the O-3 projection-discipline infrastructure); Phase-1 verifier MAY emit a warning if it cannot perform the check for a given scope.
7. **Accumulate outcomes** into a new `VerificationReport.erasure_evidence` array, parallel to `posture_transitions`. Each entry carries: `evidence_id`, `kid_destroyed`, `destroyed_at`, `cascade_scopes`, `completion_mode`, `signature_verified`, `post_erasure_uses` (count), `post_erasure_wraps` (count), `cascade_violations` (array of scope + artifact refs), `failures` (array of localizable failure codes).

`integrity_verified = false` if any erasure-evidence entry has `signature_verified = false`, `post_erasure_uses > 0`, or `post_erasure_wraps > 0`. Cascade violations surface as warnings in Phase 1 (subject to O-3 evolution); `integrity_verified` folding for cascade violations is deferred to a Phase-2 follow-on.

## Export manifest catalog (optional, mirrors signature-affirmations)

When an export contains one or more `trellis.erasure-evidence.v1` events in its chain, the export manifest MAY include the following extension catalog (Core §18.2 pattern, mirrors `trellis.export.signature-affirmations.v1`):

```cddl
ErasureEvidenceManifestExtension = {
  catalog_ref:    tstr,                 ; filename inside the ZIP (e.g. "064-erasure-evidence.cbor")
  catalog_digest: bstr .size 32,        ; SHA-256 over the catalog bytes under `trellis-content-v1`
  entry_count:    uint,                 ; number of erasure-evidence events cataloged
}
```

Catalog entries (one per erasure-evidence event in chain order):

```cddl
ErasureEvidenceCatalogEntry = {
  canonical_event_hash:  digest,
  evidence_id:           tstr,
  kid_destroyed:         bstr .size 16,
  destroyed_at:          uint,
  completion_mode:       "complete" / "in-progress" / "best-effort",
  cascade_scopes:        [+ CascadeScope],
  subject_scope_kind:    "per-subject" / "per-scope" / "per-tenant" / "deployment-wide",
}
```

Verifier obligation when the extension is present: verify the catalog digest equals the recomputed digest over `064-erasure-evidence.cbor`, and every catalog entry cross-references an in-chain erasure-evidence event byte-for-byte. Mismatch flips `integrity_verified = false`.

Exporters who do NOT include this catalog are conformant; the catalog is a performance convenience for auditor UX, not a normative requirement. Verifiers MUST NOT require its presence.

## Operator workflow

`trellis-cli erase-key` is the reference UX:

```
trellis-cli erase-key \
  --kid <kid-hex> \
  --key-class signing|wrap|recovery|scope|tenant-root \
  --subject-scope per-subject|per-scope|per-tenant|deployment-wide \
  --subject-refs <uri-list>             # conditional per scope kind
  --cascade-scopes CS-01,CS-03          # non-empty subset of A.7
  --reason-code 1..5|255
  --policy-authority <uri>
  --destruction-actor <uri>
  --attestation-key <cose-key-file>     # repeatable; ≥1 required
  --hsm-receipt <file>                  # optional
  --hsm-receipt-kind <identifier>       # required iff --hsm-receipt
  --completion-mode complete|in-progress|best-effort
```

The command MUST perform the KMS/HSM destruction operation (or dispatch it to the configured KMS adapter) AND emit the canonical event in a single atomic unit. If the KMS operation fails, no event is emitted. If the event emission fails, the operator is responsible for re-attempting emission against the `evidence_id` idempotency key (re-running the command with the same `--evidence-id` is a no-op on the KMS side and a retry on the ledger side).

Separation-of-concerns note: destroying the key without emitting the evidence is conformant under OC-75 but weakens the verifier claim; emitting evidence without actually destroying the key is non-conformant and constitutes an attestation fraud (caught by chain-consistency only if the operator later re-uses the kid; detection may require independent audit).

## Companion §20 rewrite deltas

- **OC-75** unchanged.
- **OC-76** unchanged.
- **OC-77** unchanged.
- **OC-78** promoted from SHOULD-adjacent guidance to normative: every cryptographic erasure performed by the Operator MUST be accompanied by a canonical `trellis.erasure-evidence.v1` event per this ADR. The Posture Declaration continues to document policy scope; the event records execution.
- **New OC-141 (MUST)** — every cascade-scope entry declared in an `ErasureEvidencePayload` MUST be one of (a) a value registered in Appendix A.7 or (b) a registry-appended future identifier per the append-only convention. Emitting free-text scope identifiers is non-conformant. *(Numbering uses OC-141, not OC-79: Companion already assigns OC-79..OC-81 in §20.6 to rejection / admissibility taxonomy — reusing those ids would silently rebind traceability rows.)*
- **New OC-142 (MUST)** — for every canonical event in chain order after a destroyed kid's `destroyed_at`, the operator MUST NOT sign under that kid and MUST NOT emit a key-bag entry wrapped under that kid, **within the Phase-1 verifier surfaces named in step 5** (signing `kid` + `key_bag` wraps). This is the sibling of the verifier cross-check in step 5 above; the obligation is on the emit side, the check is on the verify side. *(Subtree obligations for non-signing registry classes follow ADR 0006.)*
- **New OC-143 (SHOULD)** — operators SHOULD require dual attestation (prior + new) for erasure events with `reason_code ∈ {3, 5}` (legal order, compromise mitigation) and for `subject_scope.kind ∈ {per-tenant, deployment-wide}`. The specific policy is declared per deployment in the Posture Declaration.

## Fixture plan

Minimum Phase-1 fixture set (landed alongside the Rust implementation):

| Vector | Purpose | Cascade scopes | Subject scope |
|---|---|---|---|
| `append/023-erasure-evidence-per-subject-cs-03` | Canonical positive shape. Single subject, snapshots-only cascade. | CS-03 | per-subject |
| `append/024-erasure-evidence-per-subject-full-cascade` | Per-subject erasure declaring all six CS scopes. | CS-01..CS-06 | per-subject |
| `append/025-erasure-evidence-per-tenant` | Tenant-wide erasure (dual attestation exercise). | CS-01..CS-06 | per-tenant |
| `append/026-erasure-evidence-in-progress` | `completion_mode = "in-progress"` path. | CS-03 | per-subject |
| `append/027-erasure-evidence-hsm-receipt` | Positive shape with HSM receipt bytes attached. | CS-03 | per-subject |
| `export/009-erasure-evidence-inline` | Export catalog + event chain integration (mirrors `export/006-signature-affirmations-inline`). | — | — |
| `tamper/017-erasure-post-use` | Tampered chain: a later event signs under the destroyed kid. Expected `post_erasure_use > 0` → `integrity_verified = false`. | CS-03 | per-subject |
| `tamper/018-erasure-post-wrap` | Tampered chain: a later event's key_bag wraps under the destroyed kid. Expected `post_erasure_wrap > 0` → `integrity_verified = false`. | CS-03 | per-subject |
| `tamper/019-erasure-catalog-digest-mismatch` | Tampered export catalog; verifier rejects per §18 pattern. | — | — |

Six positive + three tamper cover the **minimum** happy path + the two post-erasure checks + catalog integrity. Follow-on tamper vectors (execution backlog, not optional forever): invalid `cascade_scopes` free-text (OC-141), `attestations` signature failure, `subject_scope` shape violations, `completion_mode` / `reason_code` illegal combinations, and dual-key posture-extension collision (custody + disclosure keys on one event) should mirror Trellis's general **tamper-first** discipline once the erasure decoder lands.

Follow-on fixtures (per-scope erasure, third-party HSM-receipt-kind variants) deferred to the trigger-gated list.

## Adversary model

What this design catches:

- **Accidental post-erasure use.** An operator destroys a key, then mistakenly signs a new event under it (or wraps a new payload under it). The verifier's chain-consistency check flags `post_erasure_use` / `post_erasure_wrap` and fails integrity.
- **Partial-cascade claims.** An operator who cascades only CS-03 (snapshots) but declares CS-01..CS-06 is making a signed false claim. An auditor with cascade-check infrastructure can detect the discrepancy. Phase-1 check is best-effort; deep check rides O-3 evolution.
- **Unsigned destruction claims.** A destruction declared only in the Posture Declaration narrative is not an `ErasureEvidencePayload`; a verifier treats it as absent. This is the failure mode option A had; option B promotes it to a structured claim.

What this design does NOT catch:

- **Hidden backups.** An operator who backs up the wrap key before destroying the on-record copy can still decrypt ciphertext. No cryptographic artifact proves that no copy exists. This requires external controls (HSM-receipt cross-audits, multi-operator quorum, regulatory audit). The `hsm_receipt` field carries opaque evidence for post-hoc human review; Phase-1 does not automate this.
- **Collusion between destruction-actor and policy-authority.** The attestation structure assumes the signing authorities have independent incentives. If both are compromised, the signed claim is false but appears valid. Mitigation rides the (forthcoming) key-class taxonomy ADR and multi-operator quorum Phase-4 work.
- **Off-chain exports of plaintext prior to destruction.** If the operator decrypted plaintext into an out-of-scope system (e.g., a vendor analytics tool) before destroying the key, the plaintext persists in that system and Trellis has no record. This is a WOS governance concern (authorization for out-of-scope decryption) and a deployment policy concern; Trellis records the destruction of the protecting key, not the absence of plaintext elsewhere.

## Alternatives considered

### Option A — absence-is-evidence (rejected)

Treat the combination of (1) `content_hash` over ciphertext in the canonical event and (2) the absence of a key-bag entry permitting decryption as the implicit erasure evidence. Companion §20 prose would clarify this as the recognized pattern.

Rejected because:

- **Operator survivability gap.** When the pre-shutdown operator is unavailable, the destruction claim is policy-prose only and dies with the operator's ability to stand behind it. The DocuSign-replacement positioning in STACK.md lead-wedge depends on post-shutdown verification; this option degrades exactly when it's most needed.
- **No cascade-scope auditability.** Option A carries no record of which CS-01..CS-06 scopes were actually cascaded for any specific destruction. The cascade obligation (OC-77) is documented but its execution is unverifiable event-by-event.
- **No accidental re-use detection.** Option A has no signed artifact to contradict a later chain event that re-uses a "destroyed" kid. The tamper vector is undetectable.

### Option C — destruction as a sidecar document (not considered load-bearing)

Emit erasure evidence as a sidecar manifest outside the canonical event chain (similar to Posture Declarations in Companion §11). Rejected in Phase-1 scoping because:

- Sidecar documents do not participate in chain ordering; the `destroyed_at` timestamp is not chain-relatively anchored. Accidental re-use detection (step 5 of verifier obligations) cannot run.
- Idempotency / ordering / signature-affirmation semantics are already established for the extension-slot pattern (`trellis.custody-model-transition.v1`, `trellis.signature-affirmations`). A sidecar model introduces a parallel wire grammar without earning its keep.

## Phase alignment

- **Phase 1 envelope compatibility.** The entire wire shape rides `EventPayload.extensions["trellis.erasure-evidence.v1"]`. No envelope change required (invariant #10 preserved; ADR 0003 honored).
- **Phase 1 runtime.** The ADR is runtime-eligible in Phase 1. Phase-1 adopters (SBA PoC) who perform retention-expired erasure (reason_code 1) or subject-requested erasure (reason_code 2) MUST emit per this ADR.
- **Phase-2 evolution.** The `key_class` field accepts registry-appended values, so the forthcoming key-class taxonomy ADR extends it non-breakingly. Multi-operator quorum (Phase-4) extends `attestations` without wire change.
- **Phase-3 case-ledger composition.** Erasure evidence events compose into case ledgers identically to other `trellis.*` extension events. No special-casing.

## Open questions / follow-ups

1. **Interaction with `LedgerServiceWrapEntry` rotation (Core §8.6).** When the LAK rotates, existing wrap entries re-wrap under the new key. An erasure-evidence event destroying an LAK-rotation predecessor must specify whether cascade includes rotating-out the affected entries. Follow-on: add a `"re-wrap-required"` cascade mode or a companion "LAK rotation coupled to erasure" recipe. Not Phase-1.
2. **HSM-receipt format registry.** The `hsm_receipt_kind` identifier needs a small registry (AWS KMS, PKCS#11, GCP KMS, Azure Key Vault, HSM-vendor-specific). Registry-append-only; empty in Phase-1 beyond an `"opaque-vendor-receipt-v1"` catch-all.
3. **Per-scope erasure of legal-hold-protected events.** Legal hold (Companion §20.6) prevents retention-expired destruction while the hold is in force. The ADR does not prohibit erasure-evidence emission under legal hold, but operators should treat this as a governance violation; a Phase-2 lint rule SHOULD detect it.
4. **Multi-operator quorum.** Companion §26 Phase-4 work covers witness / federation. Erasure evidence for cross-operator shared keys (e.g., a federated trust anchor) needs a quorum-of-N attestation shape; the current 1+ attestation requirement is a Phase-1 lower bound.

## Cross-references

- **STACK.md end-state commitments:** #1 (independent verification), #5 (custody-honest privacy). This ADR's signed claim shape + chain cross-check is the mechanism by which #5's "what verifiers can still prove" obligation becomes provable offline.
- **Invariant #10** (Phase 1 envelope IS Phase 3 case-ledger event): the extension-slot approach preserves this; no wire break.
- **Companion §20 rewrite** deltas above.
- **Companion Appendix A.5** `Attestation` reused; no new attestation shape.
- **Companion Appendix A.7** cascade-scope enumeration referenced normatively; extended non-breakingly via `tstr` escape.
- **Core §6.7** extension registry entry.
- **Core §9** domain separation for `trellis-transition-attestation-v1` reused for attestations.
- **Core §18** export-manifest extension pattern mirrored (optional `064-erasure-evidence.cbor` catalog).
- **Core §19** step 6 verifier obligations extended with step 6.d erasure-evidence cross-check.
- **Upstream ADR gap:** `specs/archive/cross-reference-map-coverage-analysis.md` §8 surfaced the "Key destruction evidence format" as dropped-without-replacement during the 8→2 Trellis consolidation. This ADR lands the replacement.
- **Downstream dependency:** the forthcoming key-class taxonomy ADR (TODO Stream 6) will extend `key_class` values without wire change.

## Implementation sequencing

1. **Spec** — Companion §20 rewrite (OC-78 promotion, new OC-141, OC-142, OC-143). Core §6.7 registration row. Core §19 verifier-obligation step. Appendix A.5 attestation reuse confirmation. This ADR pinned as the design anchor.
2. **Rust verifier** — extend `trellis-verify` with erasure-evidence decode, chain cross-check, report accumulation. `VerificationReport.erasure_evidence` field added.
3. **First positive vector** — `append/023-erasure-evidence-per-subject-cs-03` byte-matched end-to-end.
4. **Python stranger mirror** — `trellis-py` fix.
5. **Remaining positive vectors** — `append/024..027`.
6. **Tamper vectors** — `tamper/017..019`.
7. **Export catalog** — `export/009-erasure-evidence-inline` + `064-erasure-evidence.cbor`.
8. **`trellis-cli erase-key`** command.
9. **Companion §27 conformance tests** — extend to cover erasure-evidence scenarios.

Steps 1–3 are the minimum set for the ADR's core claim to hold; later steps are fixture breadth and operational ergonomics.

---

*End of ADR 0005.*
