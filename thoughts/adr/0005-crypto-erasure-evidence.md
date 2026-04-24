# ADR 0005 — Cryptographic-Erasure Evidence Format

**Date:** 2026-04-23
**Status:** Accepted (pending implementation)
**Supersedes:** —
**Superseded by:** —
**Related:** ADR 0006 (key-class taxonomy — single export key registry, `subject` vs legacy `wrap` synonym); Companion §20 (Lifecycle and Retention — OC-75..OC-78); Companion Appendix A.7 (Cascade-Scope Enumeration); Core §6.7 (Extension Registry); Core §9.3 (Hash over ciphertext); Core §18 (Export Package Layout); STACK.md end-state commitment #1 (independent verification) and #5 (custody-honest privacy); `specs/archive/cross-reference-map-coverage-analysis.md` §8 (erasure-evidence gap).

**Normative landing:** Once Companion §20, Core §6.7, and Core §19 incorporate this ADR’s tables and CDDL, those spec sections are the byte-exact source of truth. This ADR stays the design archive; any intentional wire or obligation change MUST update both the ratified spec and this document so traceability rows do not drift.

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

Trellis already registers posture-change events via `EventPayload.extensions` (Core §6.7 Extension Registry). Custody-model transitions (`trellis.custody-model-transition.v1`) and disclosure-profile transitions (`trellis.disclosure-profile-transition.v1`) both ride this seam. A `trellis.erasure-evidence.v1` event fits the same mold: it is a canonical, signed, chain-visible claim about an operator-policy event, and it composes with the existing verifier loop (Core §19 extension processing alongside posture transitions; normative checklist in this ADR §Verifier obligations).

The extension-slot approach carries Phase 1 ⊂ Phase 3 superset preservation (invariant #10) for free: no envelope change, no wire break, no ADR 0003 violation.

## Wire shape

Under `EventPayload.extensions["trellis.erasure-evidence.v1"]`:

```cddl
ErasureEvidencePayload = {
  evidence_id:           tstr,                                ; stable within ledger_scope
  kid_destroyed:         bstr .size 16,                       ; MUST be 16 bytes (Core §8.3 width). When the key is
                                                              ; registered (ADR 0006 `KeyEntry`), bytes MUST match
                                                              ; that row's kid; Phase-1 opaque HPKE recipients MAY
                                                              ; use operator-scoped ids not in the registry (§Field semantics)
  key_class:             "signing" / "subject" / "wrap" /     ; ADR 0006 kinds; "subject" = normative wrap/subject class
                         "recovery" / "scope" / "tenant-root" / tstr,   ; "wrap" = deprecated synonym for "subject"
                                                              ; (verifiers SHOULD normalize); tstr = registry extensions
  destroyed_at:          uint,                                ; Unix seconds UTC
  cascade_scopes:        [+ CascadeScope],                    ; non-empty: classes cascaded *for this destruction
                                                              ; event (subset of A.7); see §Field semantics vs OC-77
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
                                                              ; iff kind = "deployment-wide": all three ref fields null
}
```

`Attestation` reuses the shape defined in Companion Appendix A.5 (shared with custody-model and disclosure-profile transitions). No new attestation shape.

### Field semantics

- **`evidence_id`** — operator-minted stable identifier. Enables idempotent re-emission across retries and cross-reference from audit reports.
- **`kid_destroyed`** — the 16-byte identifier for the destroyed key material. **ADR 0006 registry:** when the material is listed in the export's unified `KeyEntry` array, `kid_destroyed` MUST equal that row's `kid`, and `key_class` MUST match that row's `kind` after normalizing `"wrap"` → `"subject"` (see **this ADR step 2**). **Phase-1 opaque HPKE path:** when the recipient is not yet registered as a `subject` kid (Core §9.4), operators MAY use an operator-scoped opaque 16-byte value not present in the registry; the verifier skips registry-bind in step 2 but still runs **this ADR step 8** chain consistency for `norm_key_class ∈ {"signing", "subject"}` (see step 8 Phase-1 scope).
- **`key_class`** — aligned with ADR 0006 `KeyEntry.kind` for reserved classes (`signing`, `subject`, `recovery`, `scope`, `tenant-root`; extension `tstr` per registry). Operators MUST emit `"subject"` (not `"wrap"`) for subject-class wrap keys when authoring new material; wire `"wrap"` remains **deprecated** interop only — verifiers MUST normalize `"wrap"` → `"subject"` before registry comparison and before dispatching verifier logic. Enum carries a `tstr` escape for registry extension.
- **`cascade_scopes`** — a non-empty list of cascade classes the operator **attests were addressed for this destruction emission** (subset of Appendix A.7, plus registry-appended `tstr` values per OC-141). This does **not** relax OC-77: the purge cascade for a conforming erasure still MUST eventually reach every A.7 class where plaintext or plaintext-derived material exists for the destroyed key’s scope. The field is **per-event execution accounting** (what was cascaded by the time this evidence was signed), not a claim that one row in the chain equals the entire OC-77 universe. Use `completion_mode` (`in-progress`, `best-effort`) when work is intentionally incomplete; declaring scopes not yet cascaded is a signed over-claim detectable by auditors and (when O-3 lint lands) by machinery.
- **`completion_mode`** — records the cascade state at emission time. Phase-1 operators should emit `"complete"` or `"in-progress"`. `"best-effort"` is reserved for environments where the operator cannot prove cascade completion (e.g., third-party cache-invalidation API without a signed receipt) and opts to attest partial execution rather than silently over-claim.
- **`subject_scope`** — describes what the destroyed key protected. **`per-subject` / `per-scope` / `per-tenant`:** exactly one of `subject_refs`, `ledger_scopes`, or `tenant_refs` MUST be non-null and MUST match `kind`. **`deployment-wide`:** all three MUST be null (no ref list on a whole-deployment destruction). Verifier SHALL validate per **this ADR step 3**.
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
| `trellis.erasure-evidence.v1` | `ErasureEvidencePayload` (this ADR) | Operator-authored; subject to Companion §20 discipline; verifier obligations in Core §19 (hook: extension processing; normative checklist this ADR §Verifier obligations). |

The existing §6.7 registration table convention (see `trellis.custody-model-transition.v1`, `trellis.signature-affirmations`, `trellis.intake-handoffs.v1`) applies: one row, pinned field reference.

## Verifier obligations (Core §19 extension)

**Step index (this ADR):** The numbered steps **1–10** below are the *erasure-evidence checklist*. When this ADR says “**step *n***” without a “Core §19” prefix, it means that checklist. Core §19 has its own step numbering (e.g. §19 step 1 = global decode); do not conflate the two.

A conforming verifier processing an export bundle containing `trellis.erasure-evidence.v1` events MUST:

1. **Decode** the payload against the CDDL above. Mismatch is a structure failure (Core §19 step 1).
2. **Normalize `key_class` and bind to registry (when resolvable):** Let `norm_key_class` be `"subject"` if the decoded `key_class` is `"wrap"`, else the decoded `key_class`. *(Rationale: ADR 0006’s `KeyEntry.kind` uses `"subject"`; wire `"wrap"` is deprecated synonym — verifiers normalize before any `kind` comparison.)* If `kid_destroyed` resolves to exactly one row in the export’s unified key material per **ADR 0006** `KeyEntry` / Core §8, then `norm_key_class` MUST equal that row’s `kind` (text-string equality). Mismatch is structure failure **`erasure_key_class_registry_mismatch`**. **Pre-`KeyEntry` interop:** if the export still uses a flat signing-key registry only, and `kid_destroyed` resolves there, then `norm_key_class` MUST be `"signing"`. If `kid_destroyed` does not resolve in any registry snapshot the verifier loads for that export, skip registry-bind (opaque-kid path per field semantics); step 8 still applies when `norm_key_class ∈ {"signing", "subject"}`.
3. **Validate** `subject_scope` cross-field by `kind`:
   - **`per-subject`:** `subject_refs` MUST be non-null; `ledger_scopes` and `tenant_refs` MUST be null.
   - **`per-scope`:** `ledger_scopes` MUST be non-null; `subject_refs` and `tenant_refs` MUST be null.
   - **`per-tenant`:** `tenant_refs` MUST be non-null; `subject_refs` and `ledger_scopes` MUST be null.
   - **`deployment-wide`:** `subject_refs`, `ledger_scopes`, and `tenant_refs` MUST all be null.
   Any other pattern is a structure failure.
4. **`destroyed_at` vs hosting event time:** For each erasure-evidence payload, let `host_authored_at` be the canonical `authored-at` of the **event that carries** that extension. The payload’s `destroyed_at` MUST satisfy `destroyed_at` ≤ `host_authored_at`. This pins the destruction claim to not-after the signed emission (reduces clock-skew games where a future-dated `destroyed_at` would vacuously forbid nothing). Violation is a structure failure.
5. **Multiple erasure payloads per `kid_destroyed`:** After collecting every decoded `trellis.erasure-evidence.v1` in chain order, group by `kid_destroyed` (byte-identical `bstr`). For each group, all payloads MUST carry the **same** `destroyed_at` value (integer equality). Rationale: one physical key has one destruction instant; legitimate retries and completion-mode updates re-use the same `destroyed_at` and typically the same `evidence_id`. If two payloads name the same `kid_destroyed` but disagree on `destroyed_at`, the chain is **non-conformant** — verifier MUST treat as structure failure with localizable code **`erasure_destroyed_at_conflict`**. (Distinct `evidence_id` values with identical kid + identical `destroyed_at` are allowed, e.g. duplicate emission attempts; implementers SHOULD dedupe reporting by `(kid_destroyed, destroyed_at)` for UX.)
6. **Validate** `hsm_receipt` / `hsm_receipt_kind` null-consistency (both null or both non-null). Mismatch is a structure failure.
7. **Verify** every `attestations[*].signature` under `trellis-transition-attestation-v1` domain separation (shared with A.5.3). Invalid signature flips `integrity_verified = false` per Core §19 step 9.
8. **Cross-check chain consistency for the destroyed kid:**
   - For each distinct `kid_destroyed`, let `destroyed_at` be the **single** agreed value from step 5 (after step 5 succeeds, the value is unique per kid). Let `norm_key_class` be as computed in step 2 for that payload group (all payloads in a `kid_destroyed` group MUST agree on `key_class` after normalization — if wire `key_class` differs across the group, treat as structure failure **`erasure_key_class_payload_conflict`**).
   - **When to run the two checks below:** If `norm_key_class` is **`signing`** or **`subject`**, perform both bullets. If `norm_key_class` is `recovery`, `scope`, `tenant-root`, or an extension `tstr`, the Phase-1 reference verifier **does not** apply these two checks (wire-valid; subtree / class dispatch **co-lands with ADR 0006** milestone — extend **this ADR step 8** or add a dedicated walk).
   - **Comparison rule:** “After destruction” means canonical event **`authored-at` > `destroyed_at`** (strict inequality so an event emitted in the same second as `destroyed_at` is still allowed if `authored_at` equals `destroyed_at`; the erasure event itself may carry that kid until a future spec tightens this). Verifiers MAY additionally warn when `authored-at` equals `destroyed_at` on a non-erasure event that still uses the kid, for operator hygiene.
   - For every canonical event in the chain with `authored-at` > `destroyed_at` whose COSE_Sign1 protected header `kid` equals `kid_destroyed`: mark **`post_erasure_use`**. Localizable failure per Core §19 step 6.
   - For every canonical event in the chain with `authored-at` > `destroyed_at` whose `key_bag.entries` contains an entry wrapped under `kid_destroyed`: mark **`post_erasure_wrap`**. Also localizable.

   **Phase-1 scope note:** `norm_key_class = "subject"` covers subject-class HPKE wrap keys (including wire label `"wrap"`). Chain **position** does not override `authored-at` for these inequalities; if monotonic chain order and timestamps disagree, normative behavior follows Core’s existing timestamp semantics for verification (this ADR does not introduce a parallel ordering axis).
9. **Cross-check cascade scope against export contents:**
   - For each `CascadeScope` entry declared, if the export bundle contains derived artifacts corresponding to that scope (e.g., CS-01 projections in `070-projections/`, CS-03 snapshots in `080-snapshots/`), check that those artifacts do NOT decode the destroyed key's material. Detection is best-effort in Phase 1 (full lint is deferred to the O-3 projection-discipline infrastructure); Phase-1 verifier MAY emit a warning if it cannot perform the check for a given scope.
10. **Accumulate outcomes** into a new `VerificationReport.erasure_evidence` array, parallel to `posture_transitions`. Each entry carries: `evidence_id`, `kid_destroyed`, `destroyed_at`, `cascade_scopes`, `completion_mode`, `signature_verified`, `post_erasure_uses` (count), `post_erasure_wraps` (count), `cascade_violations` (array of scope + artifact refs), `failures` (array of localizable failure codes).

`integrity_verified = false` if any erasure-evidence entry has `signature_verified = false`, `post_erasure_uses > 0`, or `post_erasure_wraps > 0`, or if **this ADR steps 1–6** produced a structure failure for any erasure payload (includes CDDL decode, **`erasure_key_class_registry_mismatch`** / **`erasure_key_class_payload_conflict`**, `subject_scope`, `destroyed_at` vs host time, `erasure_destroyed_at_conflict`, and HSM receipt null-consistency). Cascade violations surface as warnings in Phase 1 (subject to O-3 evolution); `integrity_verified` folding for cascade violations is deferred to a Phase-2 follow-on.

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
  --evidence-id <stable-id>             # idempotency key; same value on KMS retry + ledger retry
  --kid <kid-hex> \
  --key-class signing|tenant-root|scope|subject|wrap|recovery \
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

**`--key-class` → wire `key_class`:** CLI tokens match the CDDL text strings **verbatim** (`signing`, `tenant-root`, `scope`, `subject`, `recovery`). **`wrap`** is **deprecated** wire interop for subject-class destruction (synonym of `subject` after normalization — **this ADR step 2**); new operators SHOULD pass `subject` for that case. Order above matches **ADR 0006** §Decision key-material list, with `wrap` grouped after `subject` as the legacy alias.

The command MUST perform the KMS/HSM destruction operation (or dispatch it to the configured KMS adapter) AND emit the canonical event in a single atomic unit. If the KMS operation fails, no event is emitted. If the event emission fails, the operator is responsible for re-attempting emission against the `evidence_id` idempotency key (re-running the command with the same `--evidence-id` is a no-op on the KMS side and a retry on the ledger side).

Separation-of-concerns note: destroying the key without emitting the evidence is conformant under OC-75 but weakens the verifier claim; emitting evidence without actually destroying the key is non-conformant and constitutes an attestation fraud (caught by chain-consistency only if the operator later re-uses the kid; detection may require independent audit).

## Companion §20 rewrite deltas

- **OC-75** unchanged.
- **OC-76** unchanged.
- **OC-77** unchanged.
- **OC-78** promoted from SHOULD-adjacent guidance to normative: every cryptographic erasure performed by the Operator MUST be accompanied by a canonical `trellis.erasure-evidence.v1` event per this ADR. The Posture Declaration continues to document policy scope; the event records execution.
- **New OC-141 (MUST)** — every cascade-scope entry declared in an `ErasureEvidencePayload` MUST be one of (a) a value registered in Appendix A.7 or (b) a registry-appended future identifier per the append-only convention. Emitting free-text scope identifiers is non-conformant. *(Numbering uses OC-141, not OC-79: Companion already assigns OC-79..OC-81 in §20.6 to rejection / admissibility taxonomy — reusing those ids would silently rebind traceability rows.)*
- **New OC-142 (MUST)** — for every canonical event with `authored-at` > `destroyed_at` (where `destroyed_at` is the single agreed value per `kid_destroyed` after **this ADR step 5**), the operator MUST NOT sign under that kid and MUST NOT emit a key-bag entry wrapped under that kid, **within the Phase-1 verifier surfaces enforced by this ADR step 8** when `norm_key_class ∈ {"signing", "subject"}` (signing `kid` + `key_bag` wraps). Emit-side obligation pairs with verify-side **this ADR step 8**. *(Subtree obligations for other classes follow ADR 0006.)*
- **New OC-143 (SHOULD)** — operators SHOULD require dual attestation (prior + new) for erasure events with `reason_code ∈ {3, 5}` (legal order, compromise mitigation) and for `subject_scope.kind ∈ {per-tenant, deployment-wide}`. The specific policy is declared per deployment in the Posture Declaration.
- **New OC-144 (MUST)** — each `ErasureEvidencePayload`’s `destroyed_at` MUST be ≤ the `authored-at` of the canonical event that carries the extension (same rule as **this ADR step 4**).
- **New OC-145 (MUST)** — for any fixed `kid_destroyed`, every `trellis.erasure-evidence.v1` payload in the ledger scope MUST carry the same `destroyed_at` (no contradictory destruction instants). Pairs with **this ADR step 5**.
- **New OC-146 (MUST)** — when `kid_destroyed` resolves to a registry row, payload `key_class` after `wrap`→`subject` normalization MUST equal that row’s `kind` (same rule as **this ADR step 2**).

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

Six positive + three tamper cover the **minimum** happy path + the two post-erasure checks + catalog integrity. Follow-on tamper vectors (execution backlog, not optional forever): invalid `cascade_scopes` free-text (OC-141), `attestations` signature failure, `subject_scope` shape violations (including `deployment-wide` with a non-null ref field), **`erasure_destroyed_at_conflict`** (two payloads, same `kid_destroyed`, different `destroyed_at`), `destroyed_at` > hosting event `authored-at` (OC-144 / **this ADR step 4**), **`erasure_key_class_registry_mismatch`** / **`erasure_key_class_payload_conflict`** (OC-146 / **this ADR step 2**), `completion_mode` / `reason_code` illegal combinations, and dual-key posture-extension collision (custody + disclosure keys on one event) should mirror Trellis's general **tamper-first** discipline once the erasure decoder lands.

Follow-on fixtures (per-scope erasure, third-party HSM-receipt-kind variants) deferred to the trigger-gated list.

## Adversary model

What this design catches:

- **Accidental post-erasure use.** An operator destroys a key, then mistakenly signs a new event under it (or wraps a new payload under it). The verifier's chain-consistency check flags `post_erasure_use` / `post_erasure_wrap` and fails integrity.
- **Partial-cascade claims.** An operator who cascades only CS-03 (snapshots) but declares CS-01..CS-06 is making a signed false claim. An auditor with cascade-check infrastructure can detect the discrepancy. Phase-1 check is best-effort; deep check rides O-3 evolution.
- **Unsigned destruction claims.** A destruction declared only in the Posture Declaration narrative is not an `ErasureEvidencePayload`; a verifier treats it as absent. This is the failure mode option A had; option B promotes it to a structured claim.

What this design does NOT catch:

- **Hidden backups.** An operator who backs up the wrap key before destroying the on-record copy can still decrypt ciphertext. No cryptographic artifact proves that no copy exists. This requires external controls (HSM-receipt cross-audits, multi-operator quorum, regulatory audit). The `hsm_receipt` field carries opaque evidence for post-hoc human review; Phase-1 does not automate this.
- **Collusion between destruction-actor and policy-authority.** The attestation structure assumes the signing authorities have independent incentives. If both are compromised, the signed claim is false but appears valid. Mitigation rides ADR 0006 (typed registry + class dispatch) and multi-operator quorum Phase-4 work.
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

- Sidecar documents do not participate in chain ordering; the `destroyed_at` timestamp is not chain-relatively anchored. Accidental re-use detection (**this ADR step 8**) cannot run.
- Idempotency / ordering / signature-affirmation semantics are already established for the extension-slot pattern (`trellis.custody-model-transition.v1`, `trellis.signature-affirmations`). A sidecar model introduces a parallel wire grammar without earning its keep.

## Phase alignment

- **Phase 1 envelope compatibility.** The entire wire shape rides `EventPayload.extensions["trellis.erasure-evidence.v1"]`. No envelope change required (invariant #10 preserved; ADR 0003 honored).
- **Phase 1 runtime.** The ADR is runtime-eligible in Phase 1. Phase-1 adopters (SBA PoC) who perform retention-expired erasure (reason_code 1) or subject-requested erasure (reason_code 2) MUST emit per this ADR.
- **Phase-2 evolution.** The `key_class` field accepts registry-appended values; ADR 0006 is the design anchor for reserved literals and `kid` lookup. Multi-operator quorum (Phase-4) extends `attestations` without wire change.
- **Phase-3 case-ledger composition.** Erasure evidence events compose into case ledgers identically to other `trellis.*` extension events. No special-casing.

## Open questions / follow-ups

1. **Interaction with `LedgerServiceWrapEntry` rotation (Core §8.6).** When the LAK rotates, existing wrap entries re-wrap under the new key. An erasure-evidence event destroying an LAK-rotation predecessor must specify whether cascade includes rotating-out the affected entries. Follow-on: add a `"re-wrap-required"` cascade mode or a companion "LAK rotation coupled to erasure" recipe. Not Phase-1.
2. **HSM-receipt format registry.** The `hsm_receipt_kind` identifier needs a small registry (AWS KMS, PKCS#11, GCP KMS, Azure Key Vault, HSM-vendor-specific). Registry-append-only; empty in Phase-1 beyond an `"opaque-vendor-receipt-v1"` catch-all.
3. **Per-scope erasure of legal-hold-protected events.** Legal hold (Companion §20.6) prevents retention-expired destruction while the hold is in force. The ADR does not prohibit erasure-evidence emission under legal hold, but operators should treat this as a governance violation; a Phase-2 lint rule SHOULD detect it.
4. **Multi-operator quorum.** Companion §26 Phase-4 work covers witness / federation. Erasure evidence for cross-operator shared keys (e.g., a federated trust anchor) needs a quorum-of-N attestation shape; the current 1+ attestation requirement is a Phase-1 lower bound.

## Cross-references

- **ADR 0006** — `KeyEntry` / `kind` taxonomy; unified export key registry; `unknown_key_class` vs `integrity_verified` (capability gap) when extension kinds are referenced.
- **STACK.md end-state commitments:** #1 (independent verification), #5 (custody-honest privacy). This ADR's signed claim shape + chain cross-check is the mechanism by which #5's "what verifiers can still prove" obligation becomes provable offline.
- **Invariant #10** (Phase 1 envelope IS Phase 3 case-ledger event): the extension-slot approach preserves this; no wire break.
- **Companion §20 rewrite** deltas above.
- **Companion Appendix A.5** `Attestation` reused; no new attestation shape.
- **Companion Appendix A.7** cascade-scope enumeration referenced normatively; extended non-breakingly via `tstr` escape.
- **Core §6.7** extension registry entry.
- **Core §9** domain separation for `trellis-transition-attestation-v1` reused for attestations.
- **Core §18** export-manifest extension pattern mirrored (optional `064-erasure-evidence.cbor` catalog).
- **Core §19** verifier obligations extended with erasure-evidence processing (this ADR §Verifier obligations, steps 1–10).
- **Upstream ADR gap:** `specs/archive/cross-reference-map-coverage-analysis.md` §8 surfaced the "Key destruction evidence format" as dropped-without-replacement during the 8→2 Trellis consolidation. This ADR lands the replacement.
- **Downstream dependency:** ADR 0006 (`KeyEntry` / `kind`) is the normative key-class taxonomy; `key_class` in this payload stays wire-stable while semantics tighten via ADR cross-reference and verifier dispatch.

## Implementation sequencing

1. **Spec** — Companion §20 rewrite (OC-78 promotion; new OC-141..OC-146). Core §6.7 registration row. Core §19 extension hook + **this ADR §Verifier obligations steps 1–10**. Appendix A.5 attestation reuse confirmation. This ADR pinned as the design anchor.
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
