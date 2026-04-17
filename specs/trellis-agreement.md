# Trellis Agreement Document

**Status:** Track A, step 1 — decision gate
**Date:** 2026-04-17
**Owner:** product strategy
**Derived from:** `thoughts/product-vision.md` (synthesized vision and roadmap)

This document is a ≤5-page gate, not a specification. It names scope, primitives, non-goals, seams, and invariants precisely enough that a sign-off either authorizes further Track A work or blocks it. Every claim here is derivable from the product vision; nothing new is introduced.

The key words **MUST**, **MUST NOT**, **SHALL**, **SHOULD**, and **MAY** in this document are to be interpreted as described in RFC 2119.

---

## 1. Purpose

Trellis is the **integrity substrate** of the Formspec + WOS + Trellis platform: a cryptographic envelope, hash chain, signed checkpoints, and offline-verifiable export package — the layer that survives the system, the vendor, and the years. It is its own concern because two already-written specs explicitly defer this concern downstream: the **Formspec Respondent Ledger add-on** (§13 `LedgerCheckpoint`, §6.2 `eventHash` / `priorEventHash`) defers signing and anchoring, and the **WOS Kernel** (§10.5 `custodyHook`) names the same seam. Trellis is the concrete answer to those two deferrals, not a fourth layer invented above them. Now, because Formspec and WOS are running the spec→schema→vectors→runtime loop and Trellis has not yet entered it; the eleven mutually arguing prior drafts are the symptom of codifying agreement that does not yet exist.

---

## 2. Scope

**Trellis IS** an integrity substrate composed of these and only these artifacts:

- Event envelope format (canonical CBOR, deterministic encoding).
- Hash chain over envelopes (response ledger, case ledger, agency log, federation log — same shape at different scopes).
- Signed checkpoint format over contiguous event ranges.
- Export bundle format (COSE-signed, self-contained, offline-verifiable).
- Verification algorithm and verification-independence contract.
- Composition rules with Formspec Respondent Ledger and WOS `custodyHook`.

**Trellis is NOT:**

- Not a general BPM engine (Temporal, Camunda, Flowable, Step Functions keep running orchestration).
- Not a generic identity platform (integrates ID.me, Login.gov, DIDs via provider-neutral adapters; does not issue identities).
- Not a document-management system (storage is pluggable; Trellis signs over opaque blob references).
- Not a BI / analytics tool (emits clean provenance to PROV-O, XES, OCEL 2.0 downstream).
- Not a cost play ("not Adobe but cheaper").
- Not a storage engine. Runtimes use whatever storage they like (Postgres default).

---

## 3. Primitives

One-line definitions. This vocabulary is normative for the Trellis specs.

- **Event** — an atomic append: one field edit, one governance action, one signature.
- **Envelope** — the canonical CBOR byte shape wrapping one event, carrying headers, commitments, and a content-addressed reference.
- **Chain** — a hash-linked sequence of envelopes; `prev_hash` links each to its predecessor within a scope.
- **Checkpoint** — a signed summary over a contiguous range of chain events, as defined by Respondent Ledger §13 `LedgerCheckpoint`.
- **Export bundle** — a COSE-signed, self-contained package: envelopes, inclusion proofs, signing-key registry snapshot, domain-registry digest, manifest. Verifiable offline via `trellis verify`.
- **Signature** — a COSE_Sign1 (or named successor) over canonical bytes, carrying a `suite_id` that identifies the cryptographic suite in force.
- **Response ledger** — a hash-chained sequence of events for one Formspec response. Scoped to a single respondent session; sealed at submission.
- **Case ledger** — a hash-chained sequence of governance events for one case, composing one or more sealed response-ledger heads with WOS governance events. Scoped to a single adjudicatory matter. The "portable case file" of Phase 3.
- **Agency log** — an append-only log of case-ledger heads (plus metadata and optional witness timestamps) maintained by an operator. Proves a case existed at time T and was not quietly deleted. CT-style log-of-cases.
- **Federation log** — a log of agency-log heads witnessed by an independent operator. Detects cross-operator equivocation.

"Ledger" is always qualified by scope. "Log" is reserved for structures whose entries are other ledgers' heads.

---

## 4. Trust posture

Trellis claims **"difficult and obvious," not "system-owner-proof."** Per-phase bar:

- **Phase 1.** Tampering requires the operator to replace signed export bundles already in third-party hands, and any reissue is detectable by checkpoint divergence against prior recipients.
- **Phase 4.** Transparency witnessing raises the bar to equivocation-proof: no single operator, not even the operating agency, can rewrite history without cross-operator detection.

Implementations **MUST NOT** describe their trust posture more strongly than their behavior supports (see invariant #15).

---

## 5. Phase 1 non-negotiable invariants

These fifteen decisions **MUST** land in the first envelope format and its manifest. Each is cheap to include now and requires a wire-format break to retrofit. If any cannot be committed, this document is not ready to sign. The normative prose lives in the Core spec; the one-line essence is the gate.

1. **Canonical CBOR profile pinned** — the spec **MUST** name one deterministic encoding (dCBOR or a named equivalent) so byte-exact vectors are meaningful.
2. **Signature suite identified, not assumed** — every signed artifact **MUST** carry a `suite_id`; Phase 1 names the suite (e.g. Ed25519/COSE_Sign1) and reserves space for hybrid and PQ suites.
3. **Signing-key registry part of the export** — exports **MUST** include a `SigningKeyEntry` registry snapshot (Active/Revoked) so verification is self-contained across rotations.
4. **Hashes over ciphertext, not plaintext** — payloads **MUST** be encrypted before hashing so per-subject key destruction ("crypto-shredding") does not invalidate the chain.
5. **Ordering model named** — the spec **MUST** state whether `prev_hash` denotes strict linear sequence or a causal DAG; if linear-only, the header **MUST** reserve the causal-dependency field.
6. **Registry-snapshot binding in the manifest** — the manifest **MUST** include a content-addressed digest of the domain registry (event taxonomy, role vocabulary, governance rules) in force at signing.
7. **`key_bag` / author-event-hash immutable under rotation** — any re-wrap that would change `author_event_hash` is forbidden; re-wrapping **MUST** produce an append-only `LedgerServiceWrapEntry`.
8. **Redaction-aware commitment slots reserved** — the header **MUST** provide field positions for per-field commitments (Pedersen, Merkle leaves, or equivalent); BBS+ implementation is deferred, the slot is not.
9. **Plaintext-vs-committed header policy explicit** — the spec **MUST** list which header fields are plaintext (routing, audit classification) and which are commitments to encrypted or private values.
10. **Phase 1 envelope IS the Phase 3 case-ledger event format** — the byte shape produced by Phase 1 export **MUST** be the byte shape of a Phase 3 case-ledger event. Phase 2 and 3 are strict supersets; they do not redefine the event.
11. **"Profile" not overloaded across three namespaces** — Respondent Ledger owns "Profile A/B/C" (posture axes); legacy core-draft profiles are renamed "Conformance Classes"; legacy companion-draft profiles are renamed "Custody Models"; capability tiers use phase names.
12. **Head formats compose forward; agency log is a Phase 3 superset** — the Phase 3 case-ledger head **MUST** be a strict superset of Phase 1's checkpoint; agency-log entries are case-ledger heads plus arrival metadata and optional witness signatures.
13. **Append idempotency is part of the wire contract** — every `append` **MUST** carry a stable idempotency key; same-key-same-payload retries return the same canonical reference; same-key-different-payload **MUST** be rejected with a defined error.
14. **Snapshots and watermarks are day-one, not retrofitted** — every derived artifact and every agency-log entry **MUST** carry a watermark `(tree_size, tree_head_hash)` plus a rebuild path; full-replay-only is not a valid Phase 1 implementation.
15. **Implementations MUST NOT describe trust posture more strongly than behavior supports** — provider-readable payloads, externally-anchored tamper-evidence, and cryptographic controls **MUST** be declared honestly; cryptographic controls alone **MUST NOT** be described as legal admissibility.

---

## 6. Seams

Trellis **MUST** cleanly compose with the following already-written spec surfaces without redefining them:

- **Formspec Respondent Ledger §13 `LedgerCheckpoint`** — Trellis supplies the signed checkpoint format wrapping a contiguous range of ledger events.
- **Formspec Respondent Ledger §6.2 `eventHash` / `priorEventHash`** — when a Trellis envelope wraps the event, these fields are promoted from SHOULD to MUST; the binding covers both per-event (§6.2) and per-range (§13) scopes as different hashes over different scopes.
- **WOS Kernel §10.5 `custodyHook`** — a WOS runtime uses Trellis as its custody backend through this hook without redefining either spec.
- **Case ledger and agency log (Track E §21 spec extension)** — the case ledger is defined normatively as a composition of sealed response-ledger heads plus WOS governance events; the agency log is defined normatively as the operator-maintained log of case-ledger heads. These are a spec extension, not a nesting note.

---

## 7. Phase sequencing commitment

Each phase is a **strict superset** of the prior, as required by invariants #10 and #12.

- **Phase 1 — Attested exports.** Trellis is a signed export bundle format; runtimes serialize COSE-signed bundles at submission, sealing, case close, or FOIA boundary, and third parties verify offline.
- **Phase 2 — Runtime-time integrity.** Trellis becomes a Rust crate both runtimes link against; every write is attested as it happens, via the Respondent Ledger extension point and `custodyHook`.
- **Phase 3 — Portable case files.** One case ledger per case composes sealed response-ledger heads with WOS governance events; the operating agency maintains an agency log of case-ledger heads.
- **Phase 4 — Federation + Sovereign.** Independent transparency-witness operators detect equivocation via gossip; the Sovereign variant gives the respondent's device a cryptographic record signed with a key only they control.

---

## 8. Delivery shape

Two W3C-style specs, not one, not eight.

| Artifact | Phase | Size | Contents |
|---|---|---|---|
| **Trellis Core** | Phase 1 | ~30–40 pp | envelope format, canonical CBOR, hash construction, signature profile, chain construction, checkpoint format, export package layout, verification algorithm, verification-independence contract, append idempotency contract, security and privacy considerations, composition with Respondent Ledger and `custodyHook` |
| **Trellis Operational Companion** | Phase 2 | ~30–50 pp | projections and derived-artifact discipline, metadata-budget declarations, delegated-compute honesty, trust-profile transition auditability, snapshot watermarks, rebuild semantics |

Plus:

- **~50 test vectors** — language-neutral JSON under `fixtures/vectors/{append,verify,export,tamper}/`; every byte-level claim corresponds to at least one vector.
- **Rust reference implementation** — crates: `trellis-core`, `trellis-cose`, `trellis-store-postgres`, `trellis-store-memory`, `trellis-verify`, `trellis-cli`, `trellis-conformance`. Public API is three functions: `append`, `verify`, `export`.
- **CLI + WASM bindings** — `trellis verify | append | export`; WASM for browser-side respondent-facing verification.
- **One independent second implementation** — `trellis-py` or `trellis-go`, written by someone who only reads the spec, passing every vector byte-for-byte.

---

## 9. Out of scope for Phase 1

Explicitly deferred, with named seams but no implementation:

- External witnessing / transparency anchoring infrastructure (Phase 4).
- BBS+ / selective-disclosure implementation (commitment slots reserved under invariant #8).
- Threshold custody / FROST (Phase 4 custody models).
- Respondent-held keys / Sovereign variant (Phase 4).
- Consortium federation (Phase 4 federation log).
- Post-quantum signature implementation (`suite_id` reserved under invariant #2; no PQ suite ships in Phase 1).

---

## 10. Success gate

**A stranger reads the three spec documents, writes conformant implementations of each in their preferred language, and passes every conformance vector.**

No other test matters more. Not ratification checklists, not constitutional hierarchies, not requirements matrices. Can a stranger build it, and does it interop.

---

## 11. Sign-off

Signed off by ________________________ on ____________. Further Track A work authorized.
