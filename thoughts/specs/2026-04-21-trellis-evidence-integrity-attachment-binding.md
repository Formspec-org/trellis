# Trellis-side read of ADR 0072 — evidence integrity and attachment binding

**Status:** Accepted, 2026-04-22. Mirrors accepted stack ADR 0072.
**Scope:** Mirror [ADR 0072](../../../thoughts/adr/0072-stack-evidence-integrity-and-attachment-binding.md)
so Trellis execution work is concrete: what lands on the Trellis side, what
does not require a `v1.0.0` core rewrite, and which fixture / verifier /
export surfaces move first.

---

## Trellis-side read of the contract

The split is:

- **Origin layer owns** what the attachment means:
  - attachment lifecycle act (`added`, `replaced`, `removed`)
  - attachment slot / field path
  - media type
  - raw-byte digest (`attachment_sha256`)
  - replacement linkage (`prior_binding_hash`)
- **Trellis owns** how the attachment's encrypted bytes are carried,
  hash-checked, and exported:
  - `PayloadExternal`
  - ciphertext hash (`payload_content_hash`)
  - inclusion of ciphertext bytes under `060-payloads/`
  - offline attachment manifest and verifier obligations

Concretely:

1. An attachment-binding event is an ordinary Trellis canonical event whose
   binding metadata is carried in
   `EventPayload.extensions["trellis.evidence-attachment-binding.v1"]` and
   whose attachment ciphertext body is carried as `PayloadExternal`.
2. The originating layer's authored attachment-binding record carries both:
   - `attachment_sha256` — digest of the exact attachment bytes before
     Trellis encryption
   - `payload_content_hash` — digest of the exact ciphertext bytes named by
     `PayloadExternal`
3. Trellis does **not** infer attachment semantics from storage layout or
   from `060-payloads/` membership. The chain-authored binding record remains
   the source of truth.
4. Export bundles MAY include a derived `061-attachments.cbor` manifest that
   re-expresses those binding records in a verifier-friendly catalog.
5. The export manifest binds `061-attachments.cbor` through
   `ExportManifestPayload.extensions`, not through a new top-level
   `ExportManifestPayload` field.

---

## Why this matters on the Trellis side

Trellis already has half the primitive:

- `PayloadExternal` exists in Core §6.4.
- `060-payloads/<content_hash>.bin` already exists in Core §18.
- verifier behavior for absent vs present external payload bytes already
  exists in Core §19 step 4.g.

What is missing is the contract that says **which payloads are evidence
attachments** and what portable metadata must travel with them. Without that,
`PayloadExternal` proves ciphertext continuity but not document identity.

ADR 0072 fills exactly that gap without requiring an immediate rewrite of the
ratified `v1.0.0` event envelope:

- the new semantics live first in a stack contract, not in silent drift,
- the export-level addition can be expressed through the already-reserved
  manifest `extensions` container,
- the verifier additions are incremental and localizable.

The accepted stack contract uses `prior_binding_hash` for within-chain
attachment replacement lineage. That is deliberately narrower than ADR 0066
supersession: `prior_binding_hash` links one attachment binding to the prior
binding it replaces on the same chain; case-level correction, amendment, and
supersession authority remains owned by the originating governance act.

The binding record itself rides under the registered event extension
`trellis.evidence-attachment-binding.v1`. `PayloadExternal` names the
attachment ciphertext bytes, not the metadata record. This keeps Trellis'
ciphertext-hash invariant intact while making the binding metadata part of the
canonical event payload.

---

## Structural rule — extend, don't mutate

ADR 0072 lands through extension surfaces rather than top-level field
additions, for a reason that is independent of the `v1.0.0` tag state:
mutating Core's top-level `EventPayload` or `ExportManifestPayload` field
set breaks the ciphertext-hash invariant and muddies the extension-vs-core
seam. Therefore:

- Implementation enters through:
  - a registered manifest extension (`trellis.export.attachments.v1`),
  - a new derived archive member (`061-attachments.cbor`),
  - additional verifier logic,
  - new fixture coverage.

If a later design insight shows Core prose should name this contract
directly, that is a real architectural decision — take it deliberately,
retag, and migrate vectors. Nothing is released; the constraint is clean
design, not version hygiene.

---

## Proposed Trellis execution order

### 1. Export-manifest extension registration

Register:

- `trellis.export.attachments.v1`

Purpose:

- binds the digest of `061-attachments.cbor`
- declares whether attachment ciphertext bodies are bundled inline in the
  export (`inline_attachments`)

This is the smallest Trellis-owned change that makes the attachment manifest a
first-class export artifact without widening the manifest top level.

The extension payload is:

```text
{
  attachment_manifest_digest: digest,
  inline_attachments: bool,
}
```

### 2. Derived attachment manifest

Add optional archive member:

- `061-attachments.cbor`

This file is derived from chain-authored attachment-binding events. It is not
an independent source of authority; it is an index for offline verification.

Each entry carries:

```text
AttachmentManifestEntry = {
  binding_event_hash: Hash,
  attachment_id: string,
  slot_path: string,
  media_type: string,
  byte_length: uint,
  attachment_sha256: Hash,
  payload_content_hash: Hash,
  filename: string | null,
  prior_binding_hash: Hash | null,
}
```

### 3. Verifier obligations

Extend the verifier to:

- check the manifest-extension digest for `061-attachments.cbor`
- resolve every `binding_event_hash` to exactly one canonical event whose
  `EventPayload.extensions` carries `trellis.evidence-attachment-binding.v1`
- confirm each manifest entry's `payload_content_hash` equals the referenced
  event's `EventPayload.content_hash`
- when `inline_attachments = true`, require corresponding ciphertext bytes in
  `060-payloads/`
- when `prior_binding_hash` is non-null, resolve it to a prior canonical event
  on the same chain and reject unresolved references or cycles in the
  binding-lineage graph

### 4. Fixture corpus

Land a narrow first batch:

- `append/018-attachment-bound`
- `export/005-attachments-inline`
- `verify/013-export-005-missing-attachment-body`
- `tamper/013-attachment-manifest-digest-mismatch`

These fixtures should prove:

- attachment-binding events ride `PayloadExternal`
- the export manifest extension binds the attachment manifest
- missing inline attachment bodies fail the claimed inline path
- manifest digest tampering localizes correctly

---

## Honest readiness / blocker split

Two things are true at once:

- Trellis can define its **export-side** contract now.
- Trellis cannot honestly mint the first attachment-binding event bytes until an
  originating spec family publishes the authored record shape those bytes wrap.

That means the work splits cleanly:

### Ready now on the Trellis side

- pin the `trellis.export.attachments.v1` extension payload — **closed by
  this note**
- pin `061-attachments.cbor` archive semantics — **closed by this note**
- state verifier obligations for manifest digest checks, lineage checks, and
  inline ciphertext
  presence
- prepare Rust/export fixture plumbing that does not assume invented
  attachment semantics

### Blocked until origin-layer acceptance

- WOS-originated evidence bindings, if WOS later introduces post-intake
  evidence files

The line is deliberate: Trellis owns ciphertext carriage and export proof
shape, but the originating layer owns what the attachment-binding record
means. Crossing that line in Trellis first would recreate the same category
error ADR 0072 is trying to prevent.

Formspec now supplies the first origin-layer binding shape in Respondent Ledger
§6.9 and the concrete fixture
`../fixtures/respondent-ledger/attachment-added-binding.json`; Trellis
`append/018-attachment-bound` should derive from that fixture.

---

## What this does not decide on the Trellis side

- whether Formspec or WOS is the first origin layer to land the authored
  `EvidenceAttachmentBinding` shape
- the exact origin-layer `event_type` identifiers
- whether attachment readability checks are possible under a given declared
  posture
- whether attachments are legally admissible evidence
- whether an origin-layer removal event is named `attachment.removed`,
  `evidence.withdrawn`, or another lifecycle-specific event type

Those stay with the originating layer or the deployment's governance policy.

---

## Cross-reference map

| Concern | Trellis anchor | ADR 0072 anchor |
|---|---|---|
| Ciphertext carriage | Core §6.4 `PayloadExternal` | D-3 |
| Export payload directory | Core §18.2 `060-payloads/` | D-5 |
| Export-manifest extension path | Core §18.3 `extensions` | D-5 |
| Offline verifier checks | Core §19 step 4.g + export verification | D-6 |
| Origin-layer ownership split | Core §22 / §23 composition posture | D-4 |
