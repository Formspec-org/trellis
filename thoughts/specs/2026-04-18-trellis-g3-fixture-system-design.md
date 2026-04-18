# Trellis G-3 Fixture System — Design

**Date:** 2026-04-18
**Scope:** `trellis/fixtures/vectors/` layout, manifest schema, derivation-evidence convention, coverage lint, authoring discipline.
**Closes:** G-3 (design); folds G-2 (invariant coverage audit) into the G-3 lint.
**Unblocks:** G-4 (Rust reference impl — vectors become its test corpus), G-5 (stranger-test second implementation — vectors are the ratification artifact).
**Does not cover:** authoring the ~50 vectors themselves, the Rust reference impl, or the second-implementation runner. Each of those is a separate plan consuming this design.

## Context

Ratification bar per `ratification/ratification-checklist.md` G-3 and the stranger test in `specs/trellis-agreement.md` §10: an independent implementor who reads only Core + Companion + Agreement must be able to implement `append` / `verify` / `export` and byte-match against a shared set of fixtures. "Reproducible from Core prose alone" is load-bearing — if fixtures come from a reference impl rather than from the prose, the stranger test collapses into "stranger matches impl" rather than "stranger matches spec."

This design resolves seven decisions needed before any bytes are committed: operation contract, on-disk shape, manifest schema, derivation-evidence format, coverage enforcement, runner contract, and authoring discipline.

## Directory layout

```
fixtures/vectors/
├── append/NNN-slug/         # one directory per vector
├── verify/NNN-slug/
├── export/NNN-slug/
├── tamper/NNN-slug/
├── _keys/                   # pinned COSE_Key bytes + README
├── _inputs/                 # pinned payloads / prior heads + README
└── _generator/              # Python authoring aid (non-normative, §7)
```

Ordering within each op-dir is lexicographic via prefix (`001-`, `002-`, …). Underscored directories signal non-vector scaffolding and are excluded from conformance-runner walks.

## Vector contract

Operation-first tagged union. Each vector declares its `op` in its manifest and carries op-specific inputs and expected outputs. The runner dispatches on `op` rather than on directory placement — a vector is self-describing and relocatable.

- **append** — `(prior_head?, signing_key, authored_event) → (canonical_event, signed_event, next_head)`. Runner byte-compares outputs against committed sibling files.
- **verify** — `(ledger_artifact) → VerificationReport`. Runner compares report fields (`structure_verified`, `integrity_verified`, `readability_verified`) against inline expected.
- **export** — `(ledger_state) → zip_bytes`. Runner byte-compares ZIP bytes against committed expected; `zip_sha256` in the manifest is a convenience mirror, not the acceptance check.
- **tamper** — `(tampered_artifact) → VerificationReport` where at least one `*_verified` flag is false. Runner compares failure kind + failing event id.

Manifest input/expected fields mirror whatever Core says each API's signature is. This spec does not re-normatize the API — it reflects Core.

## Manifest schema (TOML per vector)

Format: TOML. Chosen over CBOR (circular dependency), JSON (no comments, awkward for humans), YAML (indentation traps). First-class parsers in Rust / Python / Go.

Common fields (all vectors):

```toml
id          = "append/001-minimal-inline-payload"
op          = "append"
description = "Minimal append to fresh ledger; exercises canonical_event_hash preimage, COSE_Sign1, head chaining."

[coverage]
tr_core       = ["TR-CORE-014", "TR-CORE-021", "TR-CORE-032"]   # canonical
core_sections = ["§6.2", "§7.3", "§8.1", "§11.2"]               # optional; if declared, lint-verified equal to matrix-derived set
invariants    = [1, 3, 7, 11]                                    # optional; if declared, lint-verified equal to matrix-derived set

[derivation]
document = "derivation.md"
```

Op-specific fields:

```toml
# append
[inputs]
prior_head     = "input-prior-head.cbor"        # omit for genesis
signing_key    = "../../_keys/issuer-001.cose_key"  # sibling paths resolve within the vector dir; _keys/_inputs relative paths also permitted
authored_event = "input-authored-event.cbor"

[expected]
canonical_event = "expected-canonical-event.cbor"
signed_event    = "expected-signed-event.cbor"  # COSE_Sign1
next_head       = "expected-next-head.cbor"
```

```toml
# tamper (and verify)
[inputs]
ledger = "input-ledger.cbor"

[expected.report]
structure_verified   = true
integrity_verified   = false
readability_verified = true
tamper_kind          = "signature_invalid"
failing_event_id     = "evt-0001"
```

```toml
# export
[inputs]
ledger_state = "input-ledger-state.cbor"

[expected]
zip         = "expected-export.zip"
zip_sha256  = "..."                              # convenience mirror; canonical source is the zip
```

**Coverage rule**: `tr_core` is the canonical list. `core_sections` and `invariants`, when declared, are lint-verified equal to the set derived from the matrix via `tr_core` lookup. This preserves author intent as commentary while preventing drift.

**Inline `[expected.report]`**: structured small-data outputs stay in the manifest; byte outputs go to sibling files. Uniformity loses to ergonomics — a reviewer should not have to open a second file to see a four-field boolean table.

## Derivation evidence

Each vector ships `derivation.md` following a fixed template:

1. **Header** — one paragraph naming what this vector exercises, plus a Core § roadmap (which sections the derivation traverses, in order).
2. **Body** — step-by-step prose. For every intermediate artifact (preimage, hash, signature, canonical encoding), the narrative:
   - Cites the Core § that defines the construction.
   - Quotes the load-bearing normative sentence.
   - Shows the input bytes in hex.
   - Computes the operation (hash / sign / encode) and shows the result in hex.
   - Names the sibling `.bin` / `.cbor` file holding those bytes.
3. **Footer** — full hex dump of every intermediate, cross-referenced by filename to the sibling files.

**Intermediates as sibling files**: cryptographic intermediates (`author-event-preimage.bin`, `canonical-event-hash.bin`, `sig-structure.bin`, `tree-head-preimage.bin`, etc.) are committed as binaries alongside the narrative's hex. Rationale: ~90% of stranger-impl reproduction failures happen at an intermediate step, not at the final output. A debugger wants to diff bytes at each stage, not parse hex out of markdown.

Format rationale for (a) narrative over (b) structured step list or (c) hybrid: the whole point of G-3 is that a human implementor can follow the derivation *from Core alone*. A machine-readable step list becomes a translation layer — it shifts the verification problem from "prove you read Core" to "prove the step list is equivalent to Core," which the stranger has no way to check. Prose forces the author to quote Core, which is the actual evidence wanted.

## Coverage enforcement

`specs/trellis-requirements-matrix.md` grows two columns per TR-CORE row:

- `testable_bytes: bool` — human-set flag. True means "this row's MUST is byte-level testable via fixtures."
- `invariants: [int]` — optional list of Phase 1 envelope invariants (#1–#15) this row covers.

`scripts/check-specs.py` extends with:

1. Every matrix row where `testable_bytes = true` MUST have ≥1 vector whose `coverage.tr_core` contains that row's ID.
2. Every vector's declared `core_sections` and `invariants` MUST equal the set *derived* from its canonical `tr_core` list via matrix lookup.
3. Every invariant #1–#15 MUST be covered by ≥1 matrix row where `testable_bytes = true` AND ≥1 vector referencing such a row.

Rule (3) folds G-2 (invariant coverage audit) into the G-3 lint: once the matrix is annotated and vectors exist for the byte-level invariants, the lint's exit code is the audit.

Some TR-CORE rows are prose-level obligations (e.g., "MUST document the custody model") and are not byte-level testable. Those carry `testable_bytes = false` and are not gated by the lint. Human judgment sets the flag.

## Conformance runner contract

Vectors are pure data. There is no shared runner protocol. Each implementation writes its own runner in its own language; implementations couple only through the committed vector bytes.

Runner responsibilities:

- Walk `fixtures/vectors/{append,verify,export,tamper}/*/`, ignoring `_`-prefixed siblings.
- Parse `manifest.toml`, dispatch on `op`.
- Load inputs; invoke the local `append` / `verify` / `export` API.
- For `append` / `export`: byte-compare output against expected sibling files.
- For `verify` / `tamper`: compare report fields against inline `[expected.report]`.

A shared stdin/stdout protocol was considered and rejected. It would dilute the stranger test by introducing a second normative artifact beyond Core — "did I implement the protocol right?" competing with "did I implement Core right?" Data-only preserves the ratification bar.

## Authoring discipline

A Python generator lives at `fixtures/vectors/_generator/` as an authoring aid. Hand-typing ~50 multi-kilobyte CBOR / COSE structures does not scale, and forbidding tooling would just push authors to ad-hoc scripts outside the repo. The generator is permitted under hard constraints:

- **Allowed imports:** `hashlib`, `cryptography.hazmat.*`, `cbor2`, `pathlib`, `tomllib`, `json`, stdlib only.
- **Forbidden imports:** any `trellis-*` crate or package, any Trellis-derived abstraction, any high-level spec-interpretive library.
- **Spec-interpretive code** — preimage construction, domain-separation tags, canonical encoding rules, `Sig_structure` assembly, `author_event_hash` / `canonical_event_hash` / `tree_head_hash` preimages — is hand-written in the generator with inline Core § citations.
- **Derivation authority**: `derivation.md` cites Core prose, not generator source. The generator is an authoring aid; it is not normative, not an oracle.
- **G-5 isolation**: the stranger never sees the generator. `_generator/` is excluded from the set of documents read for the stranger test.
- `scripts/check-specs.py` enforces the allowed-import list via AST scan of `fixtures/vectors/_generator/**/*.py`.

The generator doubles as a second hand-written reading of Core (Python), parallel to the Rust reference impl. Two independent hand-written readings of Core are a stronger evidentiary base than one — disagreements between generator output and Rust output during G-4 land as ratification signal, not as bugs in one impl.

## Key & input provenance

`fixtures/vectors/_keys/` and `fixtures/vectors/_inputs/` hold committed binary artifacts (COSE_Keys, initial payloads, prior ledger heads) referenced by manifest paths. Each underscored directory ships a `README.md` cataloguing entries with role notes:

```
# fixtures/vectors/_keys/README.md

| File                  | Role                                                |
|-----------------------|-----------------------------------------------------|
| issuer-001.cose_key   | Primary issuer — append happy-path vectors          |
| issuer-002.cose_key   | Secondary issuer — multi-signer append vectors      |
| witness-001.cose_key  | Witness signer — checkpoint signature vectors       |
```

No derivation procedure — bytes are authoritative. Seeded derivation was considered and rejected: it trades committed binaries (a few dozen KB) for a procedure the stranger also has to implement, adding surface area for disagreement. This matches RFC 8032 / COSE / HPKE test-vector conventions.

## Non-goals

- Authoring the ~50 vectors themselves. Each op-dir batch is a separate plan.
- Defining the Rust reference impl (G-4). The fixture format is language-agnostic.
- Defining the stranger-impl runner (G-5). External to this repo.
- Re-normatizing the Core API surface. Manifest fields mirror Core; they do not define it.

## Open items (resolved during implementation)

- **First-batch priority.** Candidate: the 15 invariants first, then canonical_event_hash preimage, COSE_Sign1 signing, deterministic export ZIP layout. Final call at plan time.
- **Matrix column additions.** `testable_bytes` and `invariants` columns need exact placement; TR-CORE row IDs may need renumbering if column insertion shifts layout.
- **verify/ subdirectory split.** Whether to split into `verify/success/` and `verify/negative-nontamper/` — defer until first batch reveals need.
- **Vector slug convention.** `append/001-minimal-inline-payload` style is proposed; formalize once first batch is authored.

## Consumers

Once ratified, this design is consumed by:

1. An implementation plan to scaffold `fixtures/vectors/` and author the first-slice batch (likely 5–8 vectors covering invariants #1, #3, #7, #11 plus one tamper case). That plan lives at `thoughts/plans/…` and is written via the `superpowers:writing-plans` skill.
2. Follow-on plans, one per subsequent op-dir batch.
3. G-4 plan: the Rust reference impl consumes `fixtures/vectors/` as its test corpus.
4. G-5 plan: the stranger implementor reads Core + Companion + Agreement, writes their own runner, consumes the same fixtures.
