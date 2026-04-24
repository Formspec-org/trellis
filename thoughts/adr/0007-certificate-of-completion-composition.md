# ADR 0007 — Certificate-of-Completion Composition

**Date:** 2026-04-24
**Status:** Accepted (pending implementation)
**Supersedes:** —
**Superseded by:** —
**Related:** ADR 0005 (crypto-erasure evidence — mirrors this doc's wire pattern); WOS-T4 `SignatureAffirmation` work (Core §6.7 `trellis.export.signature-affirmations.v1`); ADR 0072 (evidence integrity + attachment binding) at parent; WOS [ADR 0062](../../../wos-spec/thoughts/adr/0062-signature-profile-workflow-semantics.md) (Signature Profile workflow semantics); STACK.md end-state commitment #3 (one meaning of signing) + DocuSign-replacement positioning.

## Decision

Trellis defines a canonical event extension `trellis.certificate-of-completion.v1` that binds a human-readable signed artifact (typically PDF) to a signing-workflow completion event. The artifact travels in the export bundle as an attachment under ADR 0072's binding discipline. The canonical event carries a structured summary of chain-derived facts — signer count, signer display labels, response hash, workflow status — which a verifier cross-checks against the chain and against the artifact's content hash.

**Trellis does not normatively pin the PDF rendering.** Operators choose the template or use a reference implementation. Trellis binds the artifact's content hash and exposes a chain-derived summary the verifier can use to detect gross mismatch. The "portable case record" claim is: counsel receives the PDF + the export bundle; a verifier confirms the PDF was bound at a specific completion event, the signer count matches the chain, and the referenced `SignatureAffirmation` events are all present and valid.

The rejected alternative — normatively pinning a deterministic rendering pipeline (LaTeX + pinned pdftex, or structured HTML with fixed CSS) — would reproduce every PDF byte from the chain. It is declined because (a) PDF determinism across platforms and font installations is a hard unsolved problem that does not advance the core claim and (b) counsel cares about what the PDF says, not whether it re-renders bit-exact.

## Context

### The commitment

STACK.md end-state commitment #3 ("one meaning of signing") and the DocuSign-replacement positioning both depend on the stack producing a human-readable, signed, portable artifact. The machine-verifiable path shipped 2026-04-22 via WOS-T4: `SignatureAffirmation` canonical events bind signer evidence through `custodyHook`; `export/006-signature-affirmations-inline` carries the chain-derived catalog. That's the *engineer-facing* proof. What remains is the *counsel/bank/court*-facing artifact — the thing an applicant physically hands to a lawyer.

Without it, the stack is engineering-facing only. The DocuSign pitch doesn't close.

### What DocuSign does

DocuSign's "Certificate of Completion" is a PDF that lists signer names, timestamps, IP addresses, and a chronological audit event log. It is a rendering of the audit state inside DocuSign's system. If DocuSign shuts down, the PDF survives but its audit-trail claims become unverifiable — there is no public infrastructure to check against.

Trellis's counterpart should improve on exactly that failure mode: the PDF + the export bundle together verify offline without the vendor runtime. The PDF renders the facts; the chain proves them.

### What the PDF is, and what it is not

The PDF is a **presentation** of a canonical event. It is not the canonical event itself. The canonical event carries the facts in structured form; the PDF renders those facts in human-readable form; the chain is the authoritative record.

A verifier looking at the PDF alone cannot fully trust its claims. Looking at the PDF **plus** the export bundle plus the referenced canonical event, the verifier can confirm: signer count matches, signing events are all present and validly signed, the PDF's content hash was bound to the completion event at time T, and the workflow reached the claimed status.

Gross PDF-vs-chain divergence (e.g., PDF lists 3 signers, chain binds 2) is detectable. Fine-grained divergence (e.g., PDF mis-renders a signer's display name) is not normatively blocked in Phase 1 — the `signer_display` field is operator-sourced and the verifier checks it for gross consistency only. This is a trust boundary the ADR names explicitly.

## Wire shape

Under `EventPayload.extensions["trellis.certificate-of-completion.v1"]`:

```cddl
CertificateOfCompletionPayload = {
  certificate_id:        tstr,                         ; stable within ledger_scope
  case_ref:              tstr / null,                  ; principal URI of the governed case, null for
                                                       ; intake-record certificates without a case
  completed_at:          uint,                         ; Unix seconds UTC
  presentation_artifact: PresentationArtifact,
  chain_summary:         ChainSummary,
  signing_events:        [+ digest],                   ; canonical_event_hash of each SignatureAffirmation
                                                       ; bound by this certificate; order is workflow order
  workflow_ref:          tstr / null,                  ; optional reference to a WOS workflow execution;
                                                       ; null for signing-only deployments without WOS
  attestations:          [+ Attestation],              ; operator + any counter-signers per OC-11
  extensions:            { * tstr => any } / null,
}

PresentationArtifact = {
  content_hash:     digest,                            ; SHA-256 over the rendered artifact bytes under
                                                       ; domain tag trellis-presentation-artifact-v1
  media_type:       tstr,                              ; "application/pdf" default; "text/html" also valid
  byte_length:      uint,
  attachment_id:    tstr,                              ; ADR 0072 attachment-binding id that carries the
                                                       ; artifact bytes in the export bundle
  template_id:      tstr / null,                       ; optional reference to a pinned operator-chosen
                                                       ; template; non-normative
  template_hash:    digest / null,                     ; optional content-hash of the template used;
                                                       ; REQUIRED non-null iff template_id is non-null
}

ChainSummary = {
  signer_count:       uint,                            ; MUST equal len(signing_events)
  signer_display:     [+ SignerDisplayEntry],          ; one per signing_events entry, same order
  response_ref:       digest / null,                   ; canonical-response-hash the signing workflow covered;
                                                       ; null for non-Formspec-sourced signing ceremonies
  workflow_status:    "completed" / "countersigned" /
                      "notarized" / "partially-completed" /
                      tstr,                            ; tstr permits registered status extensions
  impact_level:       "low" / "moderate" / "high" / tstr,  ; echoes WOS Signature Profile impactLevel
                                                           ; when present; null for signing-only flows
}

SignerDisplayEntry = {
  principal_ref:   tstr,                               ; same principal URI as on the underlying
                                                       ; SignatureAffirmation event
  display_name:    tstr,                               ; what the PDF shows for this signer
  display_role:    tstr / null,                        ; "applicant" / "notary" / "witness" / custom
  signed_at:       uint,                               ; must equal or be within the SignatureAffirmation
                                                       ; event's authored_at window
}
```

`Attestation` reuses the shape defined in Companion Appendix A.5 (shared with custody-model transitions, disclosure-profile transitions, and ADR 0005 erasure-evidence events). No new attestation shape.

### Field semantics

- **`certificate_id`** — operator-minted stable identifier. Enables idempotent re-emission and cross-reference from export manifests.
- **`case_ref`** — null for intake-record-scoped signing where a governed case doesn't yet exist (per STACK.md intake vs. governed-case distinction). Matches ADR 0073's handoff distinction.
- **`presentation_artifact.content_hash`** — under a new domain tag `trellis-presentation-artifact-v1`, added to Core §9 alongside existing tags. Verifier recomputes against attachment bytes on verification.
- **`template_id` / `template_hash`** — null for one-off/operator-bespoke renderings. When both are non-null, a verifier MAY re-render from the template + chain data to detect rendering divergence; this is an optional stronger check, not a Phase-1 requirement.
- **`signing_events`** — each digest MUST reference a `SignatureAffirmation` event in the chain (either inline via Core §6.7 registration or catalogued in `062-signature-affirmations.cbor` per the WOS-T4 export pattern).
- **`workflow_ref`** — opaque to Trellis; set by the operator to a WOS workflow-execution URI when WOS drives the signing ceremony. Null for signing-only deployments that use Trellis without WOS (rare but contemplated).
- **`chain_summary.signer_count == len(signing_events)`** — invariant. Verifier MUST flag mismatch.
- **`chain_summary.signer_display[i].principal_ref`** — MUST equal the principal on `signing_events[i]`. Verifier MUST flag mismatch. The `display_name` field is operator-rendered and NOT strict-compared — it exists so a verifier can surface "PDF shows X, chain says principal Y" for human review, not to normatively gate acceptance.
- **`chain_summary.response_ref`** — when non-null, MUST match the canonical-response-hash the signing workflow covered. For Formspec-sourced signing this comes from the `authoredSignatures` field. Null for workflows that don't sign a Formspec canonical response.
- **`attestations`** — at least one attestation required (operator closing the workflow). Specific counter-signature requirements per `workflow_status` value are declared per deployment in the Posture Declaration (reuses OC-11 pattern).

## Event-type registration (Core §6.7)

Add to the Core §6.7 Extension Registry:

| event_type | admitted payload | authority boundary |
|---|---|---|
| `trellis.certificate-of-completion.v1` | `CertificateOfCompletionPayload` (this ADR) | Operator-authored; subject to Companion §6.4 Operator + §10 Posture-transition discipline; verifier obligations in Core §19 step 6 extension. |

Also add the new domain tag to Core §9 domain-separation discipline:

| tag | scope |
|---|---|
| `trellis-presentation-artifact-v1` | SHA-256 preimage for `PresentationArtifact.content_hash`. |

## Verifier obligations (Core §19 step 6 extension)

A conforming verifier processing an export bundle containing `trellis.certificate-of-completion.v1` events MUST:

1. **Decode** the payload against the CDDL above. Mismatch is a structure failure (Core §19 step 1).
2. **Validate** the invariants: `signer_count == len(signing_events)`, `len(signer_display) == len(signing_events)`, and each `signer_display[i].principal_ref` equals the principal on `signing_events[i]`. Any mismatch flips `integrity_verified = false` with failure `certificate_chain_summary_mismatch`.
3. **Verify** every `attestations[*].signature` under `trellis-transition-attestation-v1` domain separation (shared with A.5.3 and ADR 0005).
4. **Resolve** `presentation_artifact.attachment_id` via the ADR 0072 attachment-binding lineage. If the attached artifact is present, recompute its content hash under `trellis-presentation-artifact-v1` and confirm it equals `presentation_artifact.content_hash`. Mismatch flips `integrity_verified = false` with failure `presentation_artifact_content_mismatch`.
5. **Resolve** every `signing_events[i]` digest against the chain. Each MUST be a chain-present `SignatureAffirmation` event (or WOS equivalent registered in Core §6.7). Missing or wrong-type events flag `signing_event_unresolved`.
6. **Validate temporal consistency**: every `signer_display[i].signed_at` MUST equal or fall within the `authored_at` window of `signing_events[i]`. Mismatch flags `signing_event_timestamp_mismatch`.
7. **Validate** `chain_summary.response_ref` when non-null: MUST equal the canonical-response-hash on the referenced Formspec authoring events. Mismatch flags `response_ref_mismatch`.
8. **Accumulate** outcomes into a new `VerificationReport.certificates_of_completion` array, parallel to `posture_transitions` and `erasure_evidence`. Each entry carries: `certificate_id`, `completed_at`, `signer_count`, `attachment_resolved`, `all_signing_events_resolved`, `chain_summary_consistent`, `failures`.

`integrity_verified = false` if any certificate entry has `chain_summary_consistent = false`, `attachment_resolved = false` with a present attachment, or any unresolved/mismatched signing event.

Rendering-drift checks (re-rendering from `template_id` + chain data) are NOT required in Phase 1. Adopters that want stronger binding publish `template_id` + `template_hash` and rebuild at verification time as a stretch check.

## Export manifest catalog (optional)

When an export contains one or more `trellis.certificate-of-completion.v1` events, the export manifest MAY include a catalog extension (mirrors `trellis.export.signature-affirmations.v1` and ADR 0005's planned erasure catalog):

```cddl
CertificateOfCompletionManifestExtension = {
  catalog_ref:    tstr,                 ; filename inside the ZIP (e.g. "065-certificates-of-completion.cbor")
  catalog_digest: bstr .size 32,        ; SHA-256 over the catalog bytes under trellis-content-v1
  entry_count:    uint,
}
```

Catalog entries (one per certificate event, chain order):

```cddl
CertificateOfCompletionCatalogEntry = {
  canonical_event_hash:  digest,
  certificate_id:        tstr,
  completed_at:          uint,
  signer_count:          uint,
  media_type:            tstr,
  attachment_id:         tstr,
  workflow_status:       tstr,
}
```

Verifier obligation when the extension is present: recompute the catalog digest and confirm every entry byte-matches an in-chain certificate event. Mismatch flips `integrity_verified = false`.

Exporters who do NOT include this catalog are conformant; the catalog is a performance convenience for auditor UX.

## Operator workflow

`trellis-cli seal-completion` is the reference UX (precedence: mirrors `erase-key` from ADR 0005):

```
trellis-cli seal-completion \
  --workflow-ref <uri>                    # optional
  --case-ref <uri>                        # optional
  --response-ref <digest>                 # optional
  --signing-events <digest>,<digest>,...  # canonical_event_hash per SignatureAffirmation
  --signer-display <json-array-file>      # structured signer display entries
  --workflow-status completed|countersigned|notarized|partially-completed|<custom>
  --impact-level low|moderate|high        # optional
  --template-id <id>                      # optional
  --presentation-artifact <path>          # path to PDF/HTML file
  --media-type application/pdf|text/html  # default application/pdf
  --attestation-key <cose-key-file>       # repeatable; ≥1 required
```

The command performs a single atomic unit: (a) hash the presentation artifact; (b) construct the canonical event; (c) bind the attachment via ADR 0072's mechanism; (d) sign; (e) append.

## Reference template (non-normative)

A reference PDF template lives at `reference/certificate-of-completion/template-v1/` (path convention; not committed as part of this ADR). It is:

- Non-normative. Operators MAY use, extend, or ignore it.
- Single-file HTML + print stylesheet, renderable to PDF via any standard HTML→PDF pipeline (Chromium headless, WeasyPrint, etc.). Chosen because HTML-to-PDF is more deterministic across platforms than native PDF authoring.
- Published with `template_id = "trellis.reference.certificate-of-completion.v1"` and `template_hash = <content-hash>`. Deployments using it set both fields; third-party auditors can recompute.

The reference template is a Phase-1 deliverable but is NOT load-bearing for this ADR — the ADR accepts any operator-chosen template as long as the wire shape is honored.

## Adversary model

What this design catches:

- **Signer-count divergence.** PDF claims 3 signers, chain binds 2 (or vice versa). `signer_count` invariant + `signing_events` cross-resolution catches it.
- **Signer-identity divergence.** PDF attributes a signature to principal X, chain binds principal Y at the same position. `signer_display[i].principal_ref` comparison catches it.
- **Missing signing events.** PDF claims signatures exist that never landed in the chain. `signing_events` resolution catches it.
- **PDF tampering after issuance.** Artifact content hash mismatch catches post-issuance edits.
- **Artifact swap.** An operator substitutes a different PDF for the one originally bound; ADR 0072 lineage resolution catches it.

What this design does NOT catch:

- **Fine-grained rendering divergence.** PDF mis-renders a signer's display name (typo, localization, date format). The `display_name` is operator-sourced and not strictly compared to the chain. Mitigation: adopters publish `template_id` + `template_hash` and re-render for strict binding as a stretch check.
- **PDF layout claims outside the structured summary.** If the PDF contains prose claims not covered by `chain_summary` (e.g., a paragraph asserting facts), the chain does not refute those claims. Mitigation: operators should restrict certificate content to facts derivable from the chain.
- **Template substitution.** Operator uses `template_id = X` to claim a specific template but renders with a different one. Mitigation: `template_hash` field. When deployments publish the hash, verifiers can refuse rendering-drift. When they don't, the claim is trust-in-operator.

## Alternatives considered

### Option A — normatively pinned deterministic rendering (rejected)

Pin a LaTeX / pdftex / font-embedding pipeline so every PDF byte reproduces from the chain + template. Declined: cross-platform PDF byte-determinism is an unsolved problem in the general case (different pdftex versions, different font versions, OS-level anti-aliasing). The engineering cost is high; the adopter value is low because counsel doesn't verify PDFs byte-for-byte — they read them.

### Option B — sign HTML as the authoritative form, ship PDF as presentation only (rejected)

Declined: makes the presentation format dual (HTML for proof, PDF for delivery). Counsel still wants a PDF; the chain now has to describe two artifacts; UX worsens.

### Option D — opaque-attachment only, no chain summary (rejected)

Attach the PDF via ADR 0072 binding with no `chain_summary`. Verifier confirms "some PDF was bound at time T" but cannot detect PDF-vs-chain divergence. Declined: gives up the key improvement over DocuSign (detectable signer-count / signer-identity mismatch).

## Phase alignment

- **Phase 1 envelope compatible.** Rides `EventPayload.extensions` (Core §6.7). No envelope change; ADR 0003 preserved; invariant #10 preserved.
- **Phase 1 runtime eligible.** Ships alongside WOS-T4 execution; depends on `SignatureAffirmation` (live) and ADR 0072 attachment binding (live).
- **Phase 2+ evolution.** `workflow_status` and `impact_level` fields accept registry-appended values. Additional rendering-drift strictness can layer on via `template_hash` without wire change.
- **Phase 3 case-ledger composition.** Certificates compose into case ledgers identically to other Trellis events. `case_ref` field is the composition point.

## Fixture plan

Minimum Phase-1 fixture set:

| Vector | Purpose |
|---|---|
| `append/028-certificate-of-completion-minimal` | Canonical positive shape; single signer, no template. |
| `append/029-certificate-of-completion-multi-signer` | Three signers; workflow_status = `"countersigned"`; dual attestation. |
| `append/030-certificate-of-completion-with-template` | Operator-chosen template with `template_id` + `template_hash`. |
| `export/010-certificate-of-completion-inline` | Export-catalog integration (`065-certificates-of-completion.cbor`). |
| `tamper/020-cert-content-hash-mismatch` | PDF content doesn't match bound hash. |
| `tamper/021-cert-signer-count-mismatch` | Chain_summary.signer_count != len(signing_events). |
| `tamper/022-cert-signing-event-unresolved` | Referenced signing_event hash not in chain. |

Three positive + three tamper + one export catalog. Exercises the full verifier state space.

## Open questions / follow-ons

1. **Reference template authorship.** Which HTML-to-PDF pipeline does the reference template assume? (Chromium headless is the most common default; WeasyPrint is the pure-Python alternative.) Not a blocking question for this ADR; resolved when the reference template lands.
2. **Accessibility claims.** PDFs for counsel/court contexts should be WCAG-accessible (tagged PDF). Reference template should produce tagged PDFs by default but non-normative. Phase-2 concern if accessibility becomes a compliance requirement.
3. **Cross-jurisdictional localization.** Date formats, name ordering, RTL scripts. Template-level concern; `signer_display.display_name` allows arbitrary strings.
4. **PAdES / eIDAS / ESIGN integration.** Some jurisdictions require a PAdES-conformant signature inside the PDF itself (not just a Trellis-chain reference). If an operator needs this, they produce a PAdES-signed PDF themselves and bind it via this ADR; Trellis does not embed the PAdES signing. Separate ADR if this becomes a required capability.

## Cross-references

- **STACK.md** end-state commitment #3 ("one meaning of signing"); DocuSign-replacement positioning.
- **WOS ADR 0062** Signature Profile workflow semantics (source of the SignatureAffirmation events this certificate binds).
- **ADR 0072** evidence integrity + attachment binding (the mechanism `presentation_artifact.attachment_id` rides).
- **ADR 0005** crypto-erasure evidence (wire pattern mirror; same extension-slot + optional export-catalog approach).
- **Companion §6.4** Operator role (who authors certificates).
- **Companion §10** Posture-transition discipline (attestation pattern reused).
- **Core §6.7** Extension Registry (where the new event type registers).
- **Core §9** Domain separation (new `trellis-presentation-artifact-v1` tag).
- **Core §19** Verification Algorithm (new step 6 extension for certificate cross-checks).

## Implementation sequencing

1. **Spec** — Core §6.7 registration row; Core §9 domain tag; Core §19 verifier-obligation step; Companion §6.4 / §11 references; this ADR as design anchor.
2. **Rust verifier** — extend `trellis-verify` with certificate decode, attachment-resolution, chain-summary cross-check, report accumulation. `VerificationReport.certificates_of_completion` field added.
3. **First positive vector** — `append/028-certificate-of-completion-minimal` byte-matched end-to-end.
4. **Python stranger mirror** — `trellis-py` fix.
5. **Remaining positive vectors** — `append/029..030`.
6. **Tamper vectors** — `tamper/020..022`.
7. **Export catalog** — `export/010` + `065-certificates-of-completion.cbor`.
8. **`trellis-cli seal-completion`** command.
9. **Reference template** — non-normative HTML/CSS at `reference/certificate-of-completion/template-v1/`.

Steps 1–3 are the minimum for the ADR's claim to hold; steps 4–7 close the corpus; steps 8–9 are adopter ergonomics.

---

*End of ADR 0007.*
