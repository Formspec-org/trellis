# Reducing Custom Risk in a Privacy-Preserving, Event-Sourced, Cryptographically Verifiable Ledger Workflow System

## Executive synthesis

### High-confidence takeaways from the proposal

The proposal (dated 2026-04-10) describes a browser-originated, append-only author-event ledger with: server-issued canonical receipts (total order), encrypted content-addressed blobs, immutable access grants/revocations, projection-based read models rebuildable from the ledger, offline-capable clients, passkeys/WebAuthn + OIDC/SAML identity, selective disclosure via BBS-style and SD‑JWT variants, key rotation/recovery workflows (including threshold custody), verifiable exports, and compliance workflows including retention/erasure/legal hold.

The same document also mixes **core protocol rules** (hashing, determinism, canonical ordering) with **operational workflows** (KMS deletion schedules, projection health checks, blob replication scavenging) in a single “ledger design,” which is a major driver of bespoke risk.

### The ten highest‑leverage adoption opportunities

These are the changes that most reduce bespoke machinery while *raising* correctness, operational safety, and trust clarity.

1.  **Replace the custom canonical receipt chain with a standard transparency‑log construction** (Certificate Transparency style): signed tree heads + inclusion/consistency proofs, using an existing log framework (e.g., Trillian‑style) rather than a homegrown receipt hash chain. Certificate Transparency’s basic model is well‑analyzed and widely deployed. [\[1\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com)
2.  **Align the “receipt → Merkle checkpoint” story with established transparency service patterns** (CT / modern tile-based logs) instead of inventing multiple parallel proof layers (receipt chain *and* Merkle checkpoints *and* optional anchoring). This reduces both implementation and verification surface area while preserving tamper-evidence. [\[2\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com)
3.  **Use standardized secure message containers for signing/encryption** (COSE + deterministic CBOR) instead of custom envelope/payload/keybag serialization rules. COSE is now standardized (RFC 9052) and CBOR deterministic encoding is standardized (RFC 8949). [\[3\]](https://datatracker.ietf.org/doc/rfc9052/?utm_source=chatgpt.com)
4.  **Make SD‑JWT the primary selective disclosure mechanism** (and treat BBS-style ZK selective disclosure as “optional/experimental”), because SD‑JWT is standardized as an RFC and has a clear ecosystem trajectory; BBS signatures remain draft- and cryptosuite-dependent. [\[4\]](https://datatracker.ietf.org/doc/rfc9901/)
5.  **Externalize authorization using a mature, analyzable authorization model** rather than bespoke “access events + projections with special casing.” For most systems like yours, you either want (a) relationship-based authorization (Zanzibar lineage) or (b) a policy language engine with tight auditing semantics (OPA/Cedar). [\[5\]](https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/?utm_source=chatgpt.com)
6.  **Reduce bespoke sync/merge logic by tightening the canonical model**: if you truly need offline multi-writer semantics, adopt a known sync model (CRDT-based or “server-reconciliation rebase” style) rather than inventing a novel DAG + merge frontier + conflict-sensitive “stop-the-world.” CRDTs and local-first sync have known tradeoffs; try not to create a third category. [\[6\]](https://replicache.dev/)
7.  **Treat custody/threshold signing as a narrow, well-specified subsystem** using standards like FROST (RFC 9591), and move “custody mode” out of the canonical ledger protocol where possible. [\[7\]](https://datatracker.ietf.org/doc/html/rfc9591?utm_source=chatgpt.com)
8.  **Use MLS for durable group membership keying when “permissioned sharing” resembles group collaboration** (rather than per-event bespoke rewrap graphs), and keep non-group sharing (one-off exports) separate. MLS is standardized and designed for asynchronous group keying with FS/PCS. [\[8\]](https://datatracker.ietf.org/doc/rfc9420/)
9.  **Adopt a durable execution engine (Temporal-class) as the workflow/orchestration layer** but enforce a hard “no second source of truth” boundary: the workflow engine drives commands; the ledger remains the canonical state. Durable execution is explicitly designed to survive crashes and resume reliably. [\[9\]](https://docs.temporal.io/workflow-execution?utm_source=chatgpt.com)
10. **Institutionalize formal-ish verification for the canonical path**: model-check the ordering/receipt invariants (TLA+), SAT-check the access-grant invariants (Alloy), and property-test the deterministic encoding + hashing + replay rules. This is one of the few credible ways to reduce long-term protocol risk. [\[10\]](https://lamport.azurewebsites.net/tla/tla.html)

### The five most dangerous things to keep custom

These are the areas where bespoke implementations routinely fail in production and are hard to audit.

1.  **A custom “canonical receipt” construction that is not literally a known transparency-log design** (inclusion/consistency proofs) or a mature ledger product. This invites subtle equivocation, fork handling, and verifier bugs. [\[11\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com)
2.  **A bespoke sync/merge algorithm with conflict-sensitive stop conditions in the canonical path**, especially if it couples application-level conflict rules to low-level ledger ordering. This is where determinism and liveness tend to fight. [\[12\]](https://aphyr.com/posts/286-jepsen-final-thoughts)
3.  **Key recovery and “re-grant” workflows that require rewriting access to large histories** without an adopted group-keying or delegated rewrap mechanism. This becomes operationally explosive and is a common source of confidentiality regressions. [\[13\]](https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final)
4.  **Selective disclosure based primarily on BBS-style ZK proofs in the canonical system**, because interoperability, implementations, and long-lived format stability remain materially riskier than SD‑JWT. [\[14\]](https://datatracker.ietf.org/doc/draft-irtf-cfrg-bbs-signatures/)
5.  **A privacy story that mostly encrypts payloads but leaves rich, stable metadata in the clear** while claiming strong privacy or “sovereignty.” A transparency log can be perfectly correct and still leak user behavior through metadata. [\[15\]](https://www.usenix.org/system/files/usenixsecurity23-reijsbergen.pdf)

### The five things that should remain custom

These aren’t “reinventions”; they encode the product’s core invariants or true differentiation.

1.  **Domain event semantics and invariants** (what events mean, which are conflict-sensitive, what governance events exist, what constitutes a valid workflow transition).
2.  **The canonical “what is signed” commitment structure** that binds encrypted payloads, schema versions, and key material references into a stable, replayable event identity (but implemented using standard containers like COSE/CBOR). [\[16\]](https://datatracker.ietf.org/doc/rfc9052/?utm_source=chatgpt.com)
3.  **The trust model explicitness**: who can decrypt what under each custody mode; what “offline-capable” means; and which parties can block progress. (This must be bespoke because it is your liability surface.) [\[17\]](https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final)
4.  **Compliance-driven operational policies** (retention vs erasure vs legal hold vs audit exports) mapped to concrete controls, but expressed using standard policy tooling where possible. [\[18\]](https://www.ecfr.gov/current/title-45/subtitle-A/subchapter-C/part-164/subpart-C/section-164.312)
5.  **Verifier UX and export packages** that match your regulated workflows and audience, but built on standard provenance/transparency packaging patterns rather than custom scripts as the trust root. [\[19\]](https://datatracker.ietf.org/doc/draft-ietf-scitt-architecture/)

### The three biggest architectural reframes I recommend

1.  **Reframe “canonical receipts” as a transparency log service, not a ledger-in-a-database.**  
    Your “server-issued canonical receipts” are functionally a transparency log. Make that explicit, adopt the standard proof model, and stop layering multiple bespoke proof mechanisms. [\[20\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com)
2.  **Reframe “access grants” as an authorization graph derived from the canonical log, with a mature evaluator.**  
    Keep the ledger as the system-of-record for grants/revocations, but move evaluation to a proven model (Zanzibar-style tuples or Cedar/OPA), and prove (by replay and audits) that the derived authorization store matches the ledger. [\[21\]](https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/?utm_source=chatgpt.com)
3.  **Reframe “selective disclosure” as a standards-first export/verifiable presentation problem (SD‑JWT VC), not an embedded ZK protocol project.**  
    Default to SD‑JWT (RFC) and hold BBS-derived proofs as a higher-risk optional path only if needed for unlinkability guarantees you can justify. [\[4\]](https://datatracker.ietf.org/doc/rfc9901/)

## Critical read of the proposal through the five lenses

### What is truly specified versus aspirational

The proposal is unusually explicit about determinism and replayability, including canonical serialization gates and normative merge-ordering requirements. That is a strength.

At the same time, it contains several **strong rhetorical claims** that are not automatically true given the stated trust dependencies. Two examples (quoted from the proposal):

- Principle claim: “Sovereignty is literal, not architectural.”
- Export claim: “No trust in the platform needed.”

Those claims can be true only under a carefully bounded adversary model (e.g., platform cannot equivocate undetectably, cannot decrypt without user, and cannot correlate identities), and the rest of the design must consistently enforce that model. As written, the proposal’s custody and key-handling modes appear to include platform-held decryption capabilities in at least some deployments, which weakens “literal sovereignty” unless made explicit as “conditional sovereignty.”

### Distributed systems and data systems lens

**Facts grounded in known designs:**  
Your “author events + server sequencing + Merkle checkpoints” maps closely to transparency log designs: append-only sequencing with tamper-evident commitments and verifiable inclusion. Certificate Transparency formalizes inclusion proofs and consistency proofs for the append-only property. [\[22\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com)

**Judgments about the proposal as written:**

- The proposal appears to implement *two parallel ordering/verification concepts*: (a) a per-receipt hash chain (`canonical_prev_receipt_hash`) and (b) Merkle checkpoints/anchors over receipt hashes. Maintaining both increases the size of the “canonical verifier surface” for limited additional security, unless you have a specific threat that one proof system covers and the other does not. In CT-style systems, the Merkle log itself plus signed tree heads + consistency proofs typically covers the append-only integrity goal. [\[23\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com)
- The proposal’s canonical order depends on (i) HLC timestamps and (ii) topological constraints, and it introduces a “stop advancing the frontier until an explicit merge event exists” rule. That can preserve determinism, but it risks liveness failure modes (a single unresolved conflict can block global progress) unless carefully scoped to per-ledger or per-object partitions. The more you rely on global canonical order across heterogeneous event types, the more you invite “coordination everywhere.” This is a classic distributed-systems trap. [\[12\]](https://aphyr.com/posts/286-jepsen-final-thoughts)
- The proposal is correct that projections should be rebuildable from canonical events; this is standard CQRS/event-sourcing reasoning. But the proposal’s own warning that replay-from-genesis becomes impractical for millions of events is real; therefore, snapshotting becomes a second integrity surface, and you need a strict “snapshot is derived, never authoritative” rule. Event sourcing literature treats projections/materialized views as derived state, often eventually consistent, and factored from write models. [\[24\]](https://eventsourcing.readthedocs.io/en/stable/topics/projection.html)

### Fault tolerance and reliability lens

**Facts:**  
Durable execution platforms like Temporal are explicitly designed so workflow executions can survive crashes and resume, reducing bespoke retry/recovery logic for long-running processes. [\[9\]](https://docs.temporal.io/workflow-execution?utm_source=chatgpt.com)

**Judgments about the proposal as written:**

- The proposal correctly treats database commit as the acceptance authority (“accepted iff blob written and DB tx commits”), and it uses an outbox concept for downstream retries. That’s broadly aligned with known reliability patterns, including avoiding “downstream failure revokes acceptance.”
- The key risk is **operational blast radius**: key rotation, re-grants, projection rebuilds, and blob replication/scan jobs are described as intrinsic parts of the ledger system. If those are tightly coupled to canonical ingest, the system’s liveness becomes a function of the least reliable subsystem. The safer move is: canonical ingest path must be minimal, and everything else is asynchronously verifiable/repairable.
- Threshold custody and recovery flows are correct to treat key ceremonies as first-class—this is not “just crypto.” But if custody mode changes require ledger-wide reprocessing or per-event rewrap storms, you will create the equivalent of “schema migration risk” except with keys. NIST guidance emphasizes that losing wrapping keys can render data unrecoverable—so the rotation and destruction workflows must be operationally drilled, not just specified. [\[25\]](https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final)

### Privacy and cryptography lens

**Facts:**

- HPKE is standardized in RFC 9180 as a hybrid public key encryption scheme. [\[26\]](https://datatracker.ietf.org/doc/html/rfc9180?utm_source=chatgpt.com)
- COSE is standardized (RFC 9052) for CBOR-based signing/encryption containers. [\[27\]](https://datatracker.ietf.org/doc/rfc9052/?utm_source=chatgpt.com)
- SD‑JWT is standardized (RFC 9901) for selective disclosure of JWT claims. [\[28\]](https://datatracker.ietf.org/doc/rfc9901/)
- MLS is standardized (RFC 9420) for asynchronous group keying with forward secrecy and post-compromise security. [\[29\]](https://datatracker.ietf.org/doc/rfc9420/)
- FROST is standardized (RFC 9591) for threshold Schnorr signatures (2-round). [\[7\]](https://datatracker.ietf.org/doc/html/rfc9591?utm_source=chatgpt.com)

**Judgments about the proposal as written:**

- The proposal includes significant privacy mitigation work (tag commitments, fixed-position Pedersen commitments), but the privacy threat is dominated by **metadata leakage** unless you aggressively minimize envelope fields and stabilize what observers can learn. Transparency systems are well known to leak query and timing patterns unless explicitly designed for privacy. [\[15\]](https://www.usenix.org/system/files/usenixsecurity23-reijsbergen.pdf)

- The proposal’s embrace of WebAuthn PRF is directionally good for “keys without passwords,” but browser/platform support and correctness nuances are still evolving. The WebAuthn PRF extension is defined in WebAuthn Level 3; it maps to CTAP2 “hmac-secret,” with domain separation handled by hashing salts with a context string. [\[30\]](https://www.w3.org/TR/webauthn-3/)  
  Operationally, you should assume heterogeneous support across browsers and platform authenticators; WebKit historically tracked PRF/hmac-secret as an explicit implementation item, reflecting non-uniform support and edge cases. [\[31\]](https://bugs.webkit.org/show_bug.cgi?id=259934)

- Selective disclosure: the proposal’s “BBS+ path” is ambitious but higher risk. The BBS signature scheme remains in IETF draft form, while SD‑JWT is an RFC with strong ecosystem momentum and explicit VC integration pathways in the W3C VC 2.0 standard family. [\[32\]](https://datatracker.ietf.org/doc/draft-irtf-cfrg-bbs-signatures/)

- The custody model needs sharper “who can decrypt” tables per mode. If the platform holds a ledger-wide decryption capability (directly or via rewrap services), you do not have end-to-end encryption in the strict sense; you have encrypted-at-rest with platform-mediated access, which is still useful but must be described honestly for trust clarity.

### Maintainability and engineering economics lens

**Facts:**

- Deterministic CBOR encoding is standardized; RFC 8949 defines deterministic encoding requirements, and ongoing work like CBOR CDE aims to reduce ambiguity across implementations. [\[33\]](https://www.rfc-editor.org/rfc/rfc8949.html?utm_source=chatgpt.com)
- Trillian generalizes CT-style append-only logs and provides signed tree heads; it requires application “personalities” to define semantics on top. [\[34\]](https://google.github.io/trillian/docs/TransparentLogging.html)

**Judgments about the proposal as written:**

- The proposal’s “one implementation, two targets” (native + WASM) is admirable, but it increases the chance your correctness depends on cross-compilation determinism, subtle serialization differences, and crypto library quirks. This is manageable only if you (a) strictly minimize the custom cryptographic surface, and (b) use standard formats and test vectors. RFC-defined encoding + COSE helps. [\[16\]](https://datatracker.ietf.org/doc/rfc9052/?utm_source=chatgpt.com)
- The proposal takes on a “platform infrastructure ownership” burden in blob replication semantics, periodic integrity scans, projection rebuild mechanisms, and custom verification modes. Much of this can be delegated to mature storage systems and standard transparency log infrastructures, reducing bespoke code and audit scope.

### Compliance and governance lens

**Facts:**

- HIPAA’s Security Rule requires audit controls: record and examine activity in systems containing ePHI (45 CFR 164.312(b)). [\[35\]](https://www.ecfr.gov/current/title-45/subtitle-A/subchapter-C/part-164/subpart-C/section-164.312)
- NIST SP 800-53 provides a catalog of security and privacy controls, including audit/accountability families relevant to regulated environments. [\[36\]](https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final)
- GDPR Article 17 defines a right to erasure, with exceptions; the legal reality is that “immutable logs” must be reconciled with erasure obligations using carefully scoped retention or cryptographic erasure patterns. [\[37\]](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX%3A02016R0679-20160504)

**Judgments about the proposal as written:**

- The proposal’s separation of “structural proof of existence” from “ability to decrypt” is a reasonable compliance strategy (“cryptographic erasure” by key destruction), but it should be treated as jurisdiction- and regulator-dependent, not universally sufficient. GDPR interpretations can treat metadata as personal data, which the proposal itself acknowledges.
- The safest governance model is: policies (retention, holds, disclosure rules) are versioned, auditable, and *bound* to ledger state transitions. Do not leave policy as an implicit operational knob; otherwise auditors will treat your system as ungoverned automation.

## Evidence-based substitutions and composable building blocks by layer

This section answers: **what should remain custom, what should be composed from existing systems, what should be replaced, and what should be cut.** Every recommendation below is categorized into the strict layers you requested.

### Canonical core

**Recommendation:** use a transparency log as the canonical receipt/append-only proof mechanism.

- **Adopt / compose:** CT-style verifiable log semantics (Merkle log with signed tree heads, inclusion proofs, consistency proofs). [\[22\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com)
- **Adopt:** a mature verifiable log framework rather than custom machinery. Trillian is explicitly designed to generalize CT-style logging and produce signed tree heads; it expects app-specific “personalities,” which is exactly your situation. [\[34\]](https://google.github.io/trillian/docs/TransparentLogging.html)
- **Prototype / consider:** transparency-log systems like Sigstore’s Rekor if your receipt model aligns with its API and operational ergonomics. Rekor is designed as an extendable transparency log and can be run stand-alone. [\[38\]](https://docs.sigstore.dev/logging/overview/?utm_source=chatgpt.com)
- **Compose:** if you need “external anchoring,” treat it as an *optional* time-anchor step rather than part of canonical correctness. OpenTimestamps provides a verifiable timestamping format with Bitcoin anchoring. [\[39\]](https://opentimestamps.org/)

**What remains custom in canonical core:** your domain-specific event validity rules, schema binding rules, and the minimal plaintext metadata model.

### Crypto and key-management layer

**Recommendation:** standardize containers and narrow custom crypto.

- **Adopt:** HPKE (RFC 9180) for key encapsulation where you currently define custom wrapping invariants; bind it inside COSE once the COSE-HPKE profile is stable enough for your timeline. [\[40\]](https://datatracker.ietf.org/doc/html/rfc9180?utm_source=chatgpt.com)
- **Adopt:** COSE (RFC 9052) for signed envelopes and encrypted payload containers to minimize “custom serialization + signature layout” risk. [\[27\]](https://datatracker.ietf.org/doc/rfc9052/?utm_source=chatgpt.com)
- **Adopt:** deterministic CBOR rules from RFC 8949 (and consider a common deterministic profile like CBOR CDE) to reduce cross-implementation pitfalls. [\[33\]](https://www.rfc-editor.org/rfc/rfc8949.html?utm_source=chatgpt.com)
- **Compose:** custody modes using standards:
- Threshold signing: FROST (RFC 9591). [\[7\]](https://datatracker.ietf.org/doc/html/rfc9591?utm_source=chatgpt.com)
- Group keying for permissioned sharing: MLS (RFC 9420) + a vetted implementation such as OpenMLS if you truly need group-style sharing semantics. [\[41\]](https://datatracker.ietf.org/doc/rfc9420/)
- **Operational practice:** treat key management guidance as compliance-grade engineering. NIST SP 800‑57 is a baseline for cryptoperiods and key lifecycle expectations. [\[42\]](https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final)

### Identity and authn/authz layer

**Recommendation:** de-hype “DID/VC identity” and harden what you actually need.

- **Adopt:** mainstream federation for authentication (OIDC/SAML), but keep it out of canonical event validity except for explicit governance events.
- **Compose/Adopt:** passkeys/WebAuthn for authentication; treat PRF-derived keys as an enhancement with explicit downgrade paths because support can vary by platform. WebAuthn Level 3 defines the API; PRF maps to CTAP2 “hmac-secret.” [\[30\]](https://www.w3.org/TR/webauthn-3/)
- **Adopt (authorization):** either:
- **Relationship-based authorization (Zanzibar lineage)** with an implementation such as OpenFGA or SpiceDB for evaluation, while keeping the ledger as the record of grants. [\[43\]](https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/?utm_source=chatgpt.com)
- **Policy-language evaluation** using OPA (Rego) or Cedar (open-source policy engine) where policies need auditable analysis and static tooling. [\[44\]](https://openpolicyagent.org/docs?utm_source=chatgpt.com)

### Workflow/orchestration layer

**Recommendation:** explicitly treat workflow as *derived execution*, not canonical truth.

- **Adopt/Compose:** a durable execution engine (Temporal) for retries, timeouts, long-lived workflows, and operational visibility. [\[45\]](https://docs.temporal.io/workflow-execution?utm_source=chatgpt.com)
- **Keep custom:** the mapping between workflow commands and ledger events, plus invariants like “only these events may be emitted at this step.”

### Storage/availability layer

**Recommendation:** keep encrypted content-addressed blobs, but stop reinventing storage primitives.

- **Compose:** use mature object storage + immutability/retention controls as a platform concern; keep content addressing and integrity verification in your app/verifier logic.
- **Compose:** periodic integrity scanning is reasonable, but it should be an operational job that does not affect canonical ingest correctness unless you explicitly enter a degraded mode.

### Developer assurance / verification layer

**Recommendation:** institutionalize correctness.

- **Adopt:** TLA+ for the canonical ordering/receipt invariants. [\[46\]](https://lamport.azurewebsites.net/tla/tla.html)
- **Adopt:** Alloy for access-grant invariants and “no second source of truth” properties, using small-scope model finding. [\[47\]](https://groups.csail.mit.edu/sdg/pubs/2019/alloy-cacm-18-feb-22-2019.pdf)
- **Adopt:** property-based testing at the protocol boundary (deterministic encoding, hashing, signature verification, replay determinism). Hypothesis and QuickCheck-style approaches are widely used for this style of assurance. [\[48\]](https://joss.theoj.org/papers/10.21105/joss.01891.pdf)
- **Adopt:** adversarial distributed-systems testing (Jepsen-style fault injection) for projection rebuild and sync correctness under partial failure. [\[49\]](https://jepsen.io/)

## Proposed safer target architecture summary

This is a concrete target that preserves your stated invariants while reducing bespoke machinery and clarifying trust.

### Canonical core

**Canonical core = a transparency log of signed author-events (or event commitments)**

- **Client produces AuthorEvent**
- Payload encrypted (AEAD); blob is content-addressed (hash of ciphertext).
- Event envelope is a COSE_Sign1 over a deterministic CBOR structure that includes: ciphertext hash, schema/version, minimal metadata, and references to required keys/policies. [\[16\]](https://datatracker.ietf.org/doc/rfc9052/?utm_source=chatgpt.com)
- **Server runs a Transparency Log Service (TLSv2 not required; “transparency log” in the CT sense)**
- Receipts are **inclusion proofs** + the current **signed tree head**, and **consistency proofs** between tree heads as needed. [\[50\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com)
- The “canonical order” becomes “log order” (append order), which is the standard trust contract in CT-style systems. [\[51\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com)
- **Monitoring / anti-equivocation**
- A transparency log is safest when there are independent verifiers/monitors. If you keep it permissioned/private, you must still define who monitors and how clients learn about log forks (this is a governance commitment, not just code). [\[52\]](https://www.cs.yale.edu/homes/cpap/published/logs-ccs19.pdf)

### Derived/index/query layer

All projections (task queues, dashboards, “current case state,” analytics) are **derived** and must carry:

- A **provenance watermark**: “computed from log tree size N / STH hash X.”
- A **rebuild path**: the ability to rebuild from canonical events and validate against a checkpointed tree head. [\[53\]](https://eventsourcing.readthedocs.io/en/stable/topics/projection.html)

### Workflow/orchestration layer

Use durable execution strictly as orchestration:

- Temporal (or equivalent) issues commands; commands produce author-events; the log is the authority. Temporal’s value is reliable retries and resumable workflows, not canonical state. [\[45\]](https://docs.temporal.io/workflow-execution?utm_source=chatgpt.com)

### Identity/authn/authz layer

- **Authentication:** OIDC/SAML + passkeys/WebAuthn.
- **Encryption keys:** use PRF-derived keys where available, but treat it as progressive enhancement; the WebAuthn PRF extension is real, standardized, and has non-trivial platform variance, so your design must treat it as “capability-based.” [\[54\]](https://www.w3.org/TR/webauthn-3/)
- **Authorization:** ledger remains the system-of-record for grants/revocations, but evaluation uses a mature model:
- OpenFGA/SpiceDB for relationship-based checks, inspired by Zanzibar. [\[43\]](https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/?utm_source=chatgpt.com)
- Or Cedar/OPA if you need a policy language that compliance teams can audit and analyze. [\[55\]](https://docs.cedarpolicy.com/?utm_source=chatgpt.com)

### Crypto/key-management layer

- Prefer standards and narrow custom:
- COSE + deterministic CBOR for signing/encryption containers. [\[16\]](https://datatracker.ietf.org/doc/rfc9052/?utm_source=chatgpt.com)
- HPKE for wrapping keys (where appropriate). [\[56\]](https://datatracker.ietf.org/doc/html/rfc9180?utm_source=chatgpt.com)
- SD‑JWT for selective disclosure; keep BBS as optional. [\[57\]](https://datatracker.ietf.org/doc/rfc9901/)
- MLS for group-style permissioned sharing when needed. [\[41\]](https://datatracker.ietf.org/doc/rfc9420/)

### Storage/availability layer

- Keep content-addressed encrypted blobs.
- Delegate replication, retention enforcement, and immutability controls to mature storage platforms; keep verification in your verifier.

### Operational control plane

- Separate governance duties: key ceremony, policy updates, disclosure issuer controls, log signer controls.
- Bind governance changes to log entries (so policy drift is itself auditable).

### Residual risks that remain even after simplification

- **Metadata leakage remains the dominant privacy risk** unless you drastically simplify envelope fields and accept UX/operational tradeoffs. [\[15\]](https://www.usenix.org/system/files/usenixsecurity23-reijsbergen.pdf)
- **Offline multi-writer semantics remain hard**: if you truly support conflicting offline edits, you must choose a known conflict model; don’t accidentally invent one. [\[58\]](https://wal.sh/research/local-first.html)
- **Key recovery is still a sociotechnical problem**: standards help, but recovery remains the largest “human + ops” risk in encrypted systems. [\[17\]](https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final)

## Keep / Adopt / Compose / Avoid matrix and ranked subsystem recommendations

### Keep / Adopt / Compose / Avoid matrix

| Subsystem | Current likely approach from the proposal | Candidate tool / framework / methodology | Category | Maturity | What it replaces | Why it helps | Hidden costs / risks | Trust impact | Operational impact | Fit score (1–10) | Recommendation |
|----|----|----|----|----|----|----|----|----|----|---:|----|
| Canonical receipts + append-only proof | Custom receipt hash chain + Merkle checkpoints + optional OpenTimestamps anchor | CT-style transparency log semantics + Trillian-style implementation [\[59\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com) | Canonical core | Mature (pattern); Trillian mature OSS | Replaces bespoke receipt chain & checkpoint code | Shrinks verifier surface; known proofs (inclusion/consistency) | Requires monitor/governance story; operations complexity moves to log management | Clarifies trust: log key + monitors | Fewer bespoke invariants; clearer SRE playbooks | 9 | Adopt directly |
| Event signing + serialization | Custom envelope/payload format with deterministic CBOR gates | COSE (RFC 9052) + deterministic CBOR (RFC 8949 / CDE profile) [\[3\]](https://datatracker.ietf.org/doc/rfc9052/?utm_source=chatgpt.com) | Canonical core | Mature | Replaces custom signature layout & canonicalization edge cases | Interop, tooling, test vectors; fewer footguns | COSE complexity; careful profile selection required | Improves trust clarity (standardized parsing) | Medium adoption cost; long-term savings | 8 | Adopt directly |
| Key wrapping / key bags | Per-event DEK wrapped to recipients; HPKE invariant defined in-house | HPKE (RFC 9180) + COSE-HPKE profile draft where feasible [\[56\]](https://datatracker.ietf.org/doc/html/rfc9180?utm_source=chatgpt.com) | Crypto/key-management | Mature HPKE; COSE-HPKE emerging | Replaces bespoke HPKE encoding & AEAD binding | Standardizes KEM/KDF/AEAD; reduces misuse risk | COSE-HPKE still draft; must lock a profile | Reduces protocol ambiguity | Moderate; requires migration plan | 7 | Compose around it |
| Offline sync + merge | Custom DAG + HLC ordering + explicit merge events | (Boring) server-reconciliation model (Replicache-style) [\[60\]](https://replicache.dev/) | Canonical core / sync | Mature pattern (not a standard) | Replaces bespoke merge frontier rules | Simpler mental model; app-defined conflict handling | Still app-specific; needs careful replay semantics | Improves determinism clarity | Cuts bespoke complexity but requires product decisions | 7 | Prototype only |
| Offline multi-writer data model | Event-sourcing plus conflict rules | CRDT library (Automerge) for specific doc-like fields [\[61\]](https://automerge.org/blog/automerge-repo/) | Supporting/derived | Mixed maturity | Replaces custom conflict resolution for doc-like state | Automatic merge for eligible data types | Encrypted sync not “free”; key mgmt hard; CRDTs not universal | Could reduce trust concentration (more local) | Can complicate storage & performance | 5 | Prototype only |
| Workflow orchestration | Workflow state partly implicit in projections and events | Temporal durable execution platform [\[45\]](https://docs.temporal.io/workflow-execution?utm_source=chatgpt.com) | Workflow/orchestration | Mature | Replaces bespoke retry/timer/workflow glue | Operational safety for long workflows | Must enforce “ledger is truth” boundary | Trust unchanged; less app glue | Major reduction in operational toil | 8 | Adopt directly (supporting role) |
| Authorization model | Immutable access events + projections | Relationship-based authorization (OpenFGA/SpiceDB, Zanzibar lineage) [\[43\]](https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/?utm_source=chatgpt.com) | Identity/authz | Mature OSS | Replaces bespoke grant-evaluator logic | Proven model; clarity; performance | Risk of creating alternate SoT if not derived strictly from ledger | Trust clearer (explicit auth graph) | Adds service; reduces app complexity | 8 | Compose around it |
| Policy evaluation (governance) | Custom DisclosurePolicy & rules | OPA (Rego) or Cedar engine [\[44\]](https://openpolicyagent.org/docs?utm_source=chatgpt.com) | Identity/authz | Mature | Replaces bespoke policy evaluation | Auditable policy-as-code; tooling | Requires strict binding/versioning to avoid drift | Improves auditability | Adds policy deployment discipline | 7 | Adopt directly (supporting role) |
| Selective disclosure | BBS+ attestations + SD‑JWT parallel | SD‑JWT (RFC 9901) + VC DM v2 [\[62\]](https://datatracker.ietf.org/doc/rfc9901/) | Identity/VC | Mature | Replaces “BBS-first” complexity | Standardized; broad ecosystem | Less unlinkability than ZK schemes in some cases | Clearer reliance on standard verifiers | Lower implementation risk | 9 | Adopt directly |
| Threshold custody | 2-of-3 threshold signing + shares across services | FROST (RFC 9591) for threshold signatures [\[7\]](https://datatracker.ietf.org/doc/html/rfc9591?utm_source=chatgpt.com) | Crypto/key-management | Mature standard | Replaces bespoke threshold protocol | Standard ciphersuites, known hazards | Ceremony + ops remain hard; need governance | Reduces single-party key power | High ops rigor required | 7 | Keep but narrow + standardize |
| Permissioned sharing | Rewrap graphs + access events | MLS (RFC 9420) via OpenMLS when group semantics apply [\[41\]](https://datatracker.ietf.org/doc/rfc9420/) | Crypto/key-management | Mature standard; implementations maturing | Replaces per-event membership rewrap storms | Efficient group membership changes; FS/PCS | More complexity; may not fit “audit all history” without care | Could reduce key exposure | Medium-high implementation effort | 6 | Prototype only (targeted use) |
| Verifiable exports | Deterministic ZIP + scripts + optional OTS headers | SCITT-style receipt/provenance packaging + COSE/DSSE patterns [\[63\]](https://datatracker.ietf.org/doc/draft-ietf-scitt-architecture/) | Derived/export | Emerging | Replaces custom verification scripts as trust root | Standard verifiers, clearer semantics | SCITT still evolving; DSSE/COSE choices | Improves third-party trust clarity | Medium migration effort | 7 | Compose around it |
| Managed ledger DB substitution | Consider QLDB-style managed ledger | Avoid: Amazon QLDB is discontinued (support ended 2025‑07‑31) [\[64\]](https://www.infoq.com/news/2024/07/aws-kill-qldb/) | Storage/canonical | N/A | Avoids dead product | Prevents lock-in to retired service | Migration pain if already used | Avoids long-term vendor risk | Avoids rewrite later | 1 | Avoid |
| Managed ledger alternative | N/A | Azure Confidential Ledger receipts (Merkle proofs) [\[65\]](https://learn.microsoft.com/en-us/azure/confidential-ledger/write-transaction-receipts) | Canonical core (optional) | Mature managed service | Replaces self-run log infra | Built-in receipts & verification | Vendor dependence; TEE trust assumptions | Moves trust to vendor + enclave | Lower ops; higher vendor coupling | 6 | Prototype only (regulated environments) |

### Ranked recommendations by subsystem

#### Canonical core (log, receipts, ordering)

1.  **CT-style transparency log + Trillian-style implementation** (canonical core)  
    Solves: append-only verifiable ordering, receipts with inclusion/consistency proofs. [\[59\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com)  
    Still custom: event validity rules, minimal metadata model, and governance/monitoring roles.
2.  **Rekor-style transparency log** (canonical core supporting component)  
    Solves: an operationally packaged transparency log with tooling; good if your “logged statement” abstraction fits. [\[38\]](https://docs.sigstore.dev/logging/overview/?utm_source=chatgpt.com)  
    Still custom: mapping your event schema to Rekor entry types and your privacy posture.
3.  **Managed ledger with verifiable receipts (Azure Confidential Ledger)** (canonical core alternative)  
    Solves: receipts and verification workflows with vendor-managed operation. [\[66\]](https://learn.microsoft.com/en-us/azure/confidential-ledger/write-transaction-receipts)  
    Risk: vendor lock-in and enclave trust model; use only if your regulatory buyers explicitly value it.

#### Deterministic serialization + hashing conventions

1.  **COSE (RFC 9052) + deterministic CBOR (RFC 8949)** (canonical core) [\[16\]](https://datatracker.ietf.org/doc/rfc9052/?utm_source=chatgpt.com)
2.  **CBOR CDE profile** to eliminate ambiguous deterministic variants across libraries (canonical core support) [\[67\]](https://cbor-wg.github.io/draft-ietf-cbor-cde/disentangle/draft-ietf-cbor-cde.html?utm_source=chatgpt.com)
3.  **DSSE envelope** for export/provenance wrapper where canonicalization is risky (derived/export) [\[68\]](https://github.com/secure-systems-lab/dsse)

#### Identity, authn, key derivation

1.  **OIDC/SAML for authentication + WebAuthn passkeys for phishing-resistant auth** (identity/authn) [\[69\]](https://developer.mozilla.org/en-US/docs/Web/API/Web_Authentication_API)
2.  **WebAuthn PRF as progressive enhancement** for local encryption keys, with explicit downgrade and recovery flows (crypto/key mgmt) [\[54\]](https://www.w3.org/TR/webauthn-3/)
3.  **Avoid “DID/VC identity as primary”** unless you have a hard interoperability requirement; otherwise VC is better treated as an export/presentation format (derived/export). [\[70\]](https://www.w3.org/TR/vc-data-model-2.0/)

#### Authorization and policy

1.  **Zanzibar lineage (OpenFGA/SpiceDB)** for relationship-based authorization, derived from ledger grants (authz layer) [\[43\]](https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/?utm_source=chatgpt.com)
2.  **OPA** for policy evaluation where you need policy-as-code across services (supporting role) [\[71\]](https://openpolicyagent.org/docs?utm_source=chatgpt.com)
3.  **Cedar engine** when you want a constrained, analyzable authorization language with strong auditing posture (supporting role) [\[72\]](https://docs.cedarpolicy.com/?utm_source=chatgpt.com)

#### Selective disclosure and verifiable sharing

1.  **SD‑JWT (RFC 9901)** as default selective disclosure (export/derived) [\[28\]](https://datatracker.ietf.org/doc/rfc9901/)
2.  **W3C VC DM v2** as the data model/container standard to ride ecosystem tooling (export/derived) [\[70\]](https://www.w3.org/TR/vc-data-model-2.0/)
3.  **BBS signatures** only if you have a justified need for unlinkable derived proofs and you can operationally support the higher crypto complexity (prototype-only) [\[73\]](https://datatracker.ietf.org/doc/draft-irtf-cfrg-bbs-signatures/)

#### Custody and recovery

1.  **FROST (RFC 9591)** for threshold signing (crypto/key-management) [\[7\]](https://datatracker.ietf.org/doc/html/rfc9591?utm_source=chatgpt.com)
2.  **MLS (RFC 9420)** for group membership key evolution if your permissioned sharing is truly group-based (prototype-only) [\[29\]](https://datatracker.ietf.org/doc/rfc9420/)
3.  **Operational key management discipline** aligned with NIST SP 800‑57, including cryptoperiods and destruction procedures (operational control plane) [\[42\]](https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final)

## Methodologies, assurance practices, reference stacks, red flags, and decision rubric

### Methodologies and assurance practices tied to concrete risks

**Model checking / formal-ish methods**

- **TLA+**: model the canonical receipt/log rules and ordering determinism (especially around forks/merge frontiers, pending dependencies, and replay). TLA+ is explicitly intended for concurrent/distributed systems modeling and catching design errors early. [\[46\]](https://lamport.azurewebsites.net/tla/tla.html)
- **Alloy**: model the access-grant invariants: monotonicity of grant graphs, revocation semantics, “no privilege escalation via re-grant,” and “no second source of truth” constraints. Alloy is designed to explore software designs via automated analysis/model finding. [\[47\]](https://groups.csail.mit.edu/sdg/pubs/2019/alloy-cacm-18-feb-22-2019.pdf)

**Property-based testing**

- Apply PBT to deterministic encoding, hashing, and verification routines, using cross-language test vectors and fuzzed event streams; Hypothesis literature reflects the QuickCheck lineage and the value of generated tests for broad input coverage. [\[48\]](https://joss.theoj.org/papers/10.21105/joss.01891.pdf)

**Protocol fuzzing**

- Fuzz the “canonical path” parser/verifier for COSE/CBOR artifacts and receipt proofs; treat parsers as attack surface.

**Threat modeling**

- Do a structured threat model specifically for:
- Metadata leakage and traffic analysis (what can an observer infer from envelopes + receipt timing?) [\[74\]](https://www.usenix.org/system/files/usenixsecurity23-reijsbergen.pdf)
- Trust concentration in KMS/admin/recovery flows (who can decrypt? who can block?) [\[17\]](https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final)
- Equivocation/rollback by log operator (how do monitors detect?) [\[75\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com)

**Red-team architecture review**

- Commission a protocol review that includes: log equivocation, canonicalization ambiguity, downgrade attacks (PRF not available), and key recovery social engineering.

**Operational drills**

- Key ceremony runbooks for threshold custody. [\[76\]](https://datatracker.ietf.org/doc/html/rfc9591?utm_source=chatgpt.com)
- Recovery drills that simulate: device loss, escrow failure, compromised admin, and partial blob loss.
- Chaos testing for projections and blob recovery; Jepsen-style fault injection culture is explicitly built to expose consistency failures under faults. [\[49\]](https://jepsen.io/)

### Two reference stacks

#### Boring stack

Optimizes for mature tech, productivity, and low operational risk.

- **Canonical core**
- Trillian-style transparency log (append-only log mode) + signed tree heads + inclusion/consistency proofs. [\[77\]](https://google.github.io/trillian/docs/TransparentLogging.html)
- COSE (RFC 9052) signed author events + deterministic CBOR (RFC 8949). [\[16\]](https://datatracker.ietf.org/doc/rfc9052/?utm_source=chatgpt.com)
- **Adopted components**
- OIDC/SAML for authentication; WebAuthn passkeys for auth; PRF only as enhancement. [\[54\]](https://www.w3.org/TR/webauthn-3/)
- Authorization: OpenFGA/SpiceDB derived from ledger grants (read-mostly service). [\[78\]](https://openfga.dev/docs/fga?utm_source=chatgpt.com)
- Policy evaluation: OPA for governance policies (versioned and hashed into the log whenever relevant). [\[79\]](https://openpolicyagent.org/docs?utm_source=chatgpt.com)
- Workflow: Temporal for orchestration only. [\[45\]](https://docs.temporal.io/workflow-execution?utm_source=chatgpt.com)
- Selective disclosure: SD‑JWT (RFC 9901), VC DM v2 for interoperability. [\[62\]](https://datatracker.ietf.org/doc/rfc9901/)
- **Custom components**
- Domain event types, conflict rules, minimal metadata model, compliance workflows.
- **Key trust assumptions**
- Trust in log signer key + explicit monitoring governance; trust in your key custody design clarity. [\[80\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com)
- **Residual risks**
- Metadata leakage; offline merge complexity where truly required. [\[81\]](https://www.usenix.org/system/files/usenixsecurity23-reijsbergen.pdf)
- **Implementation burden**
- Medium: integrating log + COSE/CBOR + derived auth store; low bespoke crypto.

#### Ambitious stack

Same invariants, higher upside, controlled risk.

- **Canonical core**
- Transparency log + optional external time anchoring (OpenTimestamps time anchors for selected milestones, not every checkpoint). [\[82\]](https://opentimestamps.org/)
- **Adopted components**
- For permissioned sharing that behaves like collaborative groups: MLS (RFC 9420) with OpenMLS for group membership and key update mechanics. [\[41\]](https://datatracker.ietf.org/doc/rfc9420/)
- Threshold custody institutionalized using FROST (RFC 9591) with formal key ceremony governance. [\[76\]](https://datatracker.ietf.org/doc/html/rfc9591?utm_source=chatgpt.com)
- Export/verifier alignment with transparency/provenance ecosystems (SCITT direction), acknowledging SCITT is still evolving. [\[83\]](https://datatracker.ietf.org/doc/draft-ietf-scitt-architecture/)
- **Custom components**
- Same as boring stack, plus carefully scoped support for group-keying and advanced export packaging.
- **Key trust assumptions**
- Stronger reliance on crypto protocol correctness (MLS/FROST) but these are standardized. [\[84\]](https://datatracker.ietf.org/doc/rfc9420/)
- **Residual risks**
- MLS integration complexity; interaction with auditability and long-term replay; metadata leakage still. [\[85\]](https://www.usenix.org/system/files/usenixsecurity23-reijsbergen.pdf)
- **Implementation burden**
- High: MLS lifecycle, recovery UX, and compatibility testing.

### Red flags to actively avoid

1.  **“Blockchain” framing when a transparency log suffices.** If you have a trusted operator (or a small governed consortium), CT-style logs usually beat bespoke blockchains on simplicity and auditability. [\[86\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com)
2.  **CRDTs as a universal answer.** CRDTs are excellent for specific concurrent-edit domains, but encryption + key management + auditability remains hard, and CRDT sync protocols commonly assume plaintext changes unless you redesign the system. [\[87\]](https://github.com/automerge/automerge/discussions/867)
3.  **Identity ceremony that increases trust surface.** DID/VC identity stacks can add complexity without reducing trust compared to well-run OIDC + strong keys; treat VC as an export/presentation layer unless you have a strict ecosystem requirement. [\[88\]](https://www.w3.org/TR/vc-data-model-2.0/)
4.  **Policy engines as second sources of truth.** If policy state is not versioned/bound to the canonical log, you will get “policy drift,” and auditors will treat the system as non-deterministic. [\[89\]](https://openpolicyagent.org/docs?utm_source=chatgpt.com)
5.  **Custody designs that centralize power while claiming sovereignty.** Threshold protocols reduce single-party key ownership, but governance and operational control can still centralize power; you must describe sovereignty in operational terms (who can block, who can recover, who can decrypt). [\[76\]](https://datatracker.ietf.org/doc/html/rfc9591?utm_source=chatgpt.com)
6.  **Privacy tech that hides payloads but leaks behavior.** Transparency logs and append-only ledgers are structurally prone to metadata leakage (timing, frequency, sizes, access patterns) unless you explicitly design against it. [\[15\]](https://www.usenix.org/system/files/usenixsecurity23-reijsbergen.pdf)
7.  **Depending on dead or unstable managed ledgers.** Amazon QLDB is discontinued (support ended 2025‑07‑31). Building new architecture around it is negative option value. [\[64\]](https://www.infoq.com/news/2024/07/aws-kill-qldb/)

### Decision rubric for future evaluations

Use this rubric to score any proposed component or architectural change (1–5 each; higher is better). A “go” typically needs: no 1s in correctness/trust, and an average ≥ 3.5.

- **Correctness risk**: Are semantics formally specified? Are there known proofs/test vectors? (CT/COSE/CBOR score higher.) [\[90\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com)
- **Trust concentration**: Does this concentrate decryption/signing power in one actor? Are there clear monitors/governance? [\[91\]](https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final)
- **Privacy leakage**: What metadata is revealed by default? Can we mitigate without breaking UX? [\[74\]](https://www.usenix.org/system/files/usenixsecurity23-reijsbergen.pdf)
- **Operational complexity**: How many subsystems must be healthy for canonical ingest to proceed?
- **Upgrade complexity**: Are formats standardized and versionable? Do you have migration stories? [\[92\]](https://www.rfc-editor.org/rfc/rfc8949.html?utm_source=chatgpt.com)
- **Ecosystem maturity**: Are there multiple implementations and production deployments? (Prefer RFCs and well-run OSS.) [\[93\]](https://datatracker.ietf.org/doc/rfc9901/)
- **Staffing burden**: Can a normal team operate it, or does it require scarce specialists?
- **Auditability**: Can you explain and independently verify behavior with standard tools? (Transparency proofs and standardized containers help.) [\[94\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com)
- **Portability**: Can you move clouds/regions without re-architecting?
- **Strategic flexibility**: Does adopting this reduce future choices (lock-in), or increase them?

------------------------------------------------------------------------

[\[1\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com) [\[2\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com) [\[11\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com) [\[20\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com) [\[22\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com) [\[23\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com) [\[50\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com) [\[51\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com) [\[59\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com) [\[75\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com) [\[80\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com) [\[86\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com) [\[90\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com) [\[94\]](https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com) RFC 6962: Certificate Transparency

<https://www.rfc-editor.org/rfc/rfc6962.html?utm_source=chatgpt.com>

[\[3\]](https://datatracker.ietf.org/doc/rfc9052/?utm_source=chatgpt.com) [\[16\]](https://datatracker.ietf.org/doc/rfc9052/?utm_source=chatgpt.com) [\[27\]](https://datatracker.ietf.org/doc/rfc9052/?utm_source=chatgpt.com) RFC 9052 - CBOR Object Signing and Encryption (COSE)

<https://datatracker.ietf.org/doc/rfc9052/?utm_source=chatgpt.com>

[\[4\]](https://datatracker.ietf.org/doc/rfc9901/) [\[28\]](https://datatracker.ietf.org/doc/rfc9901/) [\[57\]](https://datatracker.ietf.org/doc/rfc9901/) [\[62\]](https://datatracker.ietf.org/doc/rfc9901/) [\[93\]](https://datatracker.ietf.org/doc/rfc9901/) https://datatracker.ietf.org/doc/rfc9901/

<https://datatracker.ietf.org/doc/rfc9901/>

[\[5\]](https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/?utm_source=chatgpt.com) [\[21\]](https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/?utm_source=chatgpt.com) [\[43\]](https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/?utm_source=chatgpt.com) Zanzibar: Google's Consistent, Global Authorization System

<https://research.google/pubs/zanzibar-googles-consistent-global-authorization-system/?utm_source=chatgpt.com>

[\[6\]](https://replicache.dev/) [\[60\]](https://replicache.dev/) https://replicache.dev/

<https://replicache.dev/>

[\[7\]](https://datatracker.ietf.org/doc/html/rfc9591?utm_source=chatgpt.com) [\[76\]](https://datatracker.ietf.org/doc/html/rfc9591?utm_source=chatgpt.com) RFC 9591 - The Flexible Round-Optimized Schnorr ...

<https://datatracker.ietf.org/doc/html/rfc9591?utm_source=chatgpt.com>

[\[8\]](https://datatracker.ietf.org/doc/rfc9420/) [\[29\]](https://datatracker.ietf.org/doc/rfc9420/) [\[41\]](https://datatracker.ietf.org/doc/rfc9420/) [\[84\]](https://datatracker.ietf.org/doc/rfc9420/) https://datatracker.ietf.org/doc/rfc9420/

<https://datatracker.ietf.org/doc/rfc9420/>

[\[9\]](https://docs.temporal.io/workflow-execution?utm_source=chatgpt.com) [\[45\]](https://docs.temporal.io/workflow-execution?utm_source=chatgpt.com) Temporal Workflow Execution overview

<https://docs.temporal.io/workflow-execution?utm_source=chatgpt.com>

[\[10\]](https://lamport.azurewebsites.net/tla/tla.html) [\[46\]](https://lamport.azurewebsites.net/tla/tla.html) https://lamport.azurewebsites.net/tla/tla.html

<https://lamport.azurewebsites.net/tla/tla.html>

[\[12\]](https://aphyr.com/posts/286-jepsen-final-thoughts) https://aphyr.com/posts/286-jepsen-final-thoughts

<https://aphyr.com/posts/286-jepsen-final-thoughts>

[\[13\]](https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final) [\[17\]](https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final) [\[25\]](https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final) [\[42\]](https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final) [\[91\]](https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final) https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final

<https://csrc.nist.gov/pubs/sp/800/57/pt1/r5/final>

[\[14\]](https://datatracker.ietf.org/doc/draft-irtf-cfrg-bbs-signatures/) [\[32\]](https://datatracker.ietf.org/doc/draft-irtf-cfrg-bbs-signatures/) [\[73\]](https://datatracker.ietf.org/doc/draft-irtf-cfrg-bbs-signatures/) https://datatracker.ietf.org/doc/draft-irtf-cfrg-bbs-signatures/

<https://datatracker.ietf.org/doc/draft-irtf-cfrg-bbs-signatures/>

[\[15\]](https://www.usenix.org/system/files/usenixsecurity23-reijsbergen.pdf) [\[74\]](https://www.usenix.org/system/files/usenixsecurity23-reijsbergen.pdf) [\[81\]](https://www.usenix.org/system/files/usenixsecurity23-reijsbergen.pdf) [\[85\]](https://www.usenix.org/system/files/usenixsecurity23-reijsbergen.pdf) https://www.usenix.org/system/files/usenixsecurity23-reijsbergen.pdf

<https://www.usenix.org/system/files/usenixsecurity23-reijsbergen.pdf>

[\[18\]](https://www.ecfr.gov/current/title-45/subtitle-A/subchapter-C/part-164/subpart-C/section-164.312) [\[35\]](https://www.ecfr.gov/current/title-45/subtitle-A/subchapter-C/part-164/subpart-C/section-164.312) https://www.ecfr.gov/current/title-45/subtitle-A/subchapter-C/part-164/subpart-C/section-164.312

<https://www.ecfr.gov/current/title-45/subtitle-A/subchapter-C/part-164/subpart-C/section-164.312>

[\[19\]](https://datatracker.ietf.org/doc/draft-ietf-scitt-architecture/) [\[63\]](https://datatracker.ietf.org/doc/draft-ietf-scitt-architecture/) [\[83\]](https://datatracker.ietf.org/doc/draft-ietf-scitt-architecture/) https://datatracker.ietf.org/doc/draft-ietf-scitt-architecture/

<https://datatracker.ietf.org/doc/draft-ietf-scitt-architecture/>

[\[24\]](https://eventsourcing.readthedocs.io/en/stable/topics/projection.html) [\[53\]](https://eventsourcing.readthedocs.io/en/stable/topics/projection.html) https://eventsourcing.readthedocs.io/en/stable/topics/projection.html

<https://eventsourcing.readthedocs.io/en/stable/topics/projection.html>

[\[26\]](https://datatracker.ietf.org/doc/html/rfc9180?utm_source=chatgpt.com) [\[40\]](https://datatracker.ietf.org/doc/html/rfc9180?utm_source=chatgpt.com) [\[56\]](https://datatracker.ietf.org/doc/html/rfc9180?utm_source=chatgpt.com) RFC 9180 - Hybrid Public Key Encryption

<https://datatracker.ietf.org/doc/html/rfc9180?utm_source=chatgpt.com>

[\[30\]](https://www.w3.org/TR/webauthn-3/) [\[54\]](https://www.w3.org/TR/webauthn-3/) https://www.w3.org/TR/webauthn-3/

<https://www.w3.org/TR/webauthn-3/>

[\[31\]](https://bugs.webkit.org/show_bug.cgi?id=259934) https://bugs.webkit.org/show_bug.cgi?id=259934

<https://bugs.webkit.org/show_bug.cgi?id=259934>

[\[33\]](https://www.rfc-editor.org/rfc/rfc8949.html?utm_source=chatgpt.com) [\[92\]](https://www.rfc-editor.org/rfc/rfc8949.html?utm_source=chatgpt.com) RFC 8949: Concise Binary Object Representation (CBOR)

<https://www.rfc-editor.org/rfc/rfc8949.html?utm_source=chatgpt.com>

[\[34\]](https://google.github.io/trillian/docs/TransparentLogging.html) [\[77\]](https://google.github.io/trillian/docs/TransparentLogging.html) https://google.github.io/trillian/docs/TransparentLogging.html

<https://google.github.io/trillian/docs/TransparentLogging.html>

[\[36\]](https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final) https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final

<https://csrc.nist.gov/pubs/sp/800/53/r5/upd1/final>

[\[37\]](https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX%3A02016R0679-20160504) https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX%3A02016R0679-20160504

<https://eur-lex.europa.eu/legal-content/EN/TXT/PDF/?uri=CELEX%3A02016R0679-20160504>

[\[38\]](https://docs.sigstore.dev/logging/overview/?utm_source=chatgpt.com) Rekor

<https://docs.sigstore.dev/logging/overview/?utm_source=chatgpt.com>

[\[39\]](https://opentimestamps.org/) [\[82\]](https://opentimestamps.org/) https://opentimestamps.org/

<https://opentimestamps.org/>

[\[44\]](https://openpolicyagent.org/docs?utm_source=chatgpt.com) [\[71\]](https://openpolicyagent.org/docs?utm_source=chatgpt.com) [\[79\]](https://openpolicyagent.org/docs?utm_source=chatgpt.com) [\[89\]](https://openpolicyagent.org/docs?utm_source=chatgpt.com) Open Policy Agent (OPA)

<https://openpolicyagent.org/docs?utm_source=chatgpt.com>

[\[47\]](https://groups.csail.mit.edu/sdg/pubs/2019/alloy-cacm-18-feb-22-2019.pdf) https://groups.csail.mit.edu/sdg/pubs/2019/alloy-cacm-18-feb-22-2019.pdf

<https://groups.csail.mit.edu/sdg/pubs/2019/alloy-cacm-18-feb-22-2019.pdf>

[\[48\]](https://joss.theoj.org/papers/10.21105/joss.01891.pdf) https://joss.theoj.org/papers/10.21105/joss.01891.pdf

<https://joss.theoj.org/papers/10.21105/joss.01891.pdf>

[\[49\]](https://jepsen.io/) https://jepsen.io/

<https://jepsen.io/>

[\[52\]](https://www.cs.yale.edu/homes/cpap/published/logs-ccs19.pdf) https://www.cs.yale.edu/homes/cpap/published/logs-ccs19.pdf

<https://www.cs.yale.edu/homes/cpap/published/logs-ccs19.pdf>

[\[55\]](https://docs.cedarpolicy.com/?utm_source=chatgpt.com) [\[72\]](https://docs.cedarpolicy.com/?utm_source=chatgpt.com) What is Cedar? \| Cedar Policy Language Reference Guide

<https://docs.cedarpolicy.com/?utm_source=chatgpt.com>

[\[58\]](https://wal.sh/research/local-first.html) https://wal.sh/research/local-first.html

<https://wal.sh/research/local-first.html>

[\[61\]](https://automerge.org/blog/automerge-repo/) https://automerge.org/blog/automerge-repo/

<https://automerge.org/blog/automerge-repo/>

[\[64\]](https://www.infoq.com/news/2024/07/aws-kill-qldb/) https://www.infoq.com/news/2024/07/aws-kill-qldb/

<https://www.infoq.com/news/2024/07/aws-kill-qldb/>

[\[65\]](https://learn.microsoft.com/en-us/azure/confidential-ledger/write-transaction-receipts) [\[66\]](https://learn.microsoft.com/en-us/azure/confidential-ledger/write-transaction-receipts) https://learn.microsoft.com/en-us/azure/confidential-ledger/write-transaction-receipts

<https://learn.microsoft.com/en-us/azure/confidential-ledger/write-transaction-receipts>

[\[67\]](https://cbor-wg.github.io/draft-ietf-cbor-cde/disentangle/draft-ietf-cbor-cde.html?utm_source=chatgpt.com) CBOR Common Deterministic Encoding (CDE)

<https://cbor-wg.github.io/draft-ietf-cbor-cde/disentangle/draft-ietf-cbor-cde.html?utm_source=chatgpt.com>

[\[68\]](https://github.com/secure-systems-lab/dsse) https://github.com/secure-systems-lab/dsse

<https://github.com/secure-systems-lab/dsse>

[\[69\]](https://developer.mozilla.org/en-US/docs/Web/API/Web_Authentication_API) https://developer.mozilla.org/en-US/docs/Web/API/Web_Authentication_API

<https://developer.mozilla.org/en-US/docs/Web/API/Web_Authentication_API>

[\[70\]](https://www.w3.org/TR/vc-data-model-2.0/) [\[88\]](https://www.w3.org/TR/vc-data-model-2.0/) https://www.w3.org/TR/vc-data-model-2.0/

<https://www.w3.org/TR/vc-data-model-2.0/>

[\[78\]](https://openfga.dev/docs/fga?utm_source=chatgpt.com) Introduction to OpenFGA

<https://openfga.dev/docs/fga?utm_source=chatgpt.com>

[\[87\]](https://github.com/automerge/automerge/discussions/867) https://github.com/automerge/automerge/discussions/867

<https://github.com/automerge/automerge/discussions/867>
