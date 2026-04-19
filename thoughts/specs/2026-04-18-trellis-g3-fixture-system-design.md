# Trellis G-3 Fixture System — Design

**Date:** 2026-04-18
**Scope:** `trellis/fixtures/vectors/` layout, manifest schema, derivation-evidence convention, coverage lint, authoring discipline.
**Closes:** G-3 (design); folds G-2 (invariant coverage audit) into the G-3 lint.
**Unblocks:** G-4 (Rust reference impl — vectors become its test corpus), G-5 (stranger-test second implementation — vectors are the ratification artifact).
**Does not cover:** authoring the ~50 vectors themselves, the Rust reference impl, or the second-implementation runner. Each of those is a separate plan consuming this design.

**Amended 2026-04-18:** closes review findings F1 (invariant derivation unsound), F2 (G-2 fold-in over-claim), F4 (matrix column reconciliation), F5 (bypass → allowlist), F6 (vector lifecycle: identity, deprecation, overlap).

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
# status        = "deprecated"   # optional; omitted = "active". See "Vector lifecycle."
# deprecated_at = "2026-07-01"   # required iff status = "deprecated" (ISO-8601 date).

[coverage]
tr_core       = ["TR-CORE-014", "TR-CORE-021", "TR-CORE-032"]   # canonical
core_sections = ["§6.2", "§7.3", "§8.1", "§11.2"]               # optional; if declared, lint-verified equal to matrix-derived set
# invariants = [1, 3, 7, 11]                                    # optional commentary; lint issues a warning (not an error) on mismatch

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

**Coverage rule**: `tr_core` is the canonical coverage anchor. `core_sections`, when declared, is lint-verified equal to the set derived from its `tr_core` list via matrix lookup — a mismatch is an error. `invariants`, when declared, is commentary only: the lint uses it for an informational cross-check and emits a warning (not an error) if the declared set disagrees with the union of `Invariant` cells for the vector's `tr_core` rows. No bidirectional equality is enforced for invariants.

Rationale: matrix rows with `Invariant=—` are intentionally lossy under the derivation. Enforcing bidirectional equality would pressure authors to cherry-pick `tr_core` entries to match a declared `invariants` set rather than letting `tr_core` represent the maximum set of rows the bytes actually exercise. That degrades coverage signal. Making `invariants` commentary-only removes the perverse incentive while preserving author context for human readers.

**Inline `[expected.report]`**: structured small-data outputs stay in the manifest; byte outputs go to sibling files. Uniformity loses to ergonomics — a reviewer should not have to open a second file to see a four-field boolean table.

**Deprecation fields (F6)**: `status` and `deprecated_at` are optional top-level fields introduced by the "Vector lifecycle" amendment. `status` defaults to `"active"`; the only other accepted value is `"deprecated"`. When `status = "deprecated"`, `deprecated_at` is required and MUST be an ISO-8601 date. Runners and the G-3 lint treat deprecated vectors as tombstones — present on disk, skipped for execution, and excluded from byte-testable coverage accounting.

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

The matrix's existing `Verification` and `Invariant` columns carry the data needed for coverage enforcement. `Verification` values containing the literal substring `test-vector` flag byte-level testability; the `Invariant` column (values like `#5` or `#1, #4`) carries the invariant link. No column additions to the matrix are required.

`scripts/check-specs.py` extends with:

1. Every matrix row where `Verification` contains `test-vector` MUST have ≥1 vector whose `coverage.tr_core` contains that row's ID.
2. Every vector's declared `core_sections` MUST equal the set *derived* from its canonical `tr_core` list via matrix lookup (error on mismatch). A vector's declared `invariants`, if present, is checked informally — a mismatch emits a warning, not an error.
3. (Narrowed — see "Invariant audit paths" below.) Every invariant for which the matrix contains ≥1 row with `Verification` containing `test-vector` MUST be covered by ≥1 vector whose `coverage.tr_core` includes such a row.

### Invariant audit paths

Not all 15 Phase 1 invariants are byte-testable. The matrix's `Verification` column already distinguishes two paths:

- **Byte-testable invariants** — those for which the matrix contains ≥1 row with `Verification` containing `test-vector`. These are audited by the G-3 lint (rule 3 above). A vector must reference such a row in its `coverage.tr_core` for the invariant to be considered covered.
- **Non-byte-testable invariants** — those whose matrix rows carry `Verification` values in {`model-check`, `declaration-doc-check`, `spec-cross-ref`, `projection-rebuild-drill`, `manual-review`}. These are not gated by the G-3 lint. They are audited outside this system — through model checking, manual doc review, or spec cross-reference passes.

A follow-on audit pass (tracked as the remaining G-2 work, not part of G-3) will assign each of the 15 invariants to a path and confirm coverage in the appropriate channel. That pass must complete before G-2 closes. The G-3 lint's rule 3 covers only the byte-testable subset — it does not claim to close G-2 on its own.

Some TR-CORE rows are prose-level obligations (e.g., "MUST document the custody model") and are not byte-level testable. Their `Verification` column will not contain `test-vector` and they are not gated by rule 1.

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

## Vector lifecycle

**Amended 2026-04-18 (F6):** the original design pinned a slug convention in "Open items" but said nothing about what happens to a vector after it lands. The stranger test (`specs/trellis-agreement.md` §10) requires that external implementors key off `<op>/<NNN>-<slug>` as a stable ID. Three rules govern vector identity, retirement, and overlap. All three exist to keep the corpus walkable by an implementor reading only Core + Companion + Agreement, with no access to git history.

**Slug convention.** Vector directories are `fixtures/vectors/<op>/<NNN>-<kebab-slug>/`, where `NNN` is a zero-padded three-digit decimal and `<kebab-slug>` is a short lowercase kebab-case description of what the vector exercises. The first authored vector `append/001-minimal-inline-payload` sets the precedent. This supersedes the "Vector slug convention" entry under "Open items."

**Renumbering forbidden.** Once a vector is committed with a given `NNN-` prefix, the numeric prefix is immutable for the life of that vector identity. Stranger-impl runners and downstream consumers treat `<op>/<NNN>-<slug>` as the vector's stable ID. The slug portion *after* `NNN-` may be renamed if the rename clarifies without changing what the vector exercises (e.g., `append/001-minimal-inline-payload` → `append/001-inline-payload-genesis` would be permitted; `append/001-...` → `append/017-...` would not). Any change that alters what the bytes exercise demands a new vector at a new `NNN-`, not a rename in place.

Narrow exception: a vector is not considered committed for the purposes of this rule until the PR that introduces it has merged to the ratification branch. Within the lifetime of an open authoring PR, `NNN-` may be renumbered freely to resolve collisions between parallel in-flight branches. Once the PR merges, the prefix freezes.

Rationale: `coverage.tr_core` lists, cross-vector references in `derivation.md`, and stranger-impl expected-output files all cite vectors by path. Renumbering in place would silently invalidate every such citation without producing a lint failure, because the string `append/003-...` would still resolve to *some* vector — just not the one the citation meant. Treating `NNN-` as load-bearing prevents this class of drift.

**Deprecation.** Hard-delete of a committed vector directory is forbidden. To retire a vector, the author sets `status = "deprecated"` in the manifest and adds `deprecated_at = "YYYY-MM-DD"`. Conformance runners skip deprecated vectors; the G-3 lint ignores their `coverage.tr_core` entries when computing byte-testable coverage (a deprecated vector does not count toward closing a TR-CORE row). The directory and its bytes remain in the tree as a tombstone.

Rationale: the stranger test says an independent implementor walks Core + Companion + Agreement, then `fixtures/vectors/`. If a vector's directory disappears, the stranger has no way to know whether the corpus intentionally shrank or whether they are looking at a corrupted checkout — git history is not part of the inputs they read. A manifest-level deprecation flag makes retirement legible without consulting history: the directory is still there, the bytes still validate, and the manifest explicitly says "this is a tombstone." A lexicographic gap in `NNN-` prefixes (e.g., missing `003-` between `002-` and `004-`) is never ambiguous — it cannot happen, because deprecated vectors stay in place.

This adds two fields to the manifest schema; see "Manifest schema" below.

**Overlap policy.** Two vectors may legitimately claim the same `TR-CORE-*` row in their `coverage.tr_core`. Duplicate coverage is **encouraged, not discouraged** — the byte-testable subset of Core is small enough that the same invariant will be exercised incidentally across many vectors, and independent byte-matches at different call sites strengthen the ratification signal. There is no "primary coverage" notion: coverage is a pure `covered / not covered` boolean per TR-CORE row. The G-3 lint accepts overlap silently (neither error nor warning). A row is covered once ≥1 non-deprecated vector claims it.

Rationale: this matches the F2 philosophy. F2 narrowed rule 3 to the byte-testable subset specifically because forcing bidirectional equality between declared coverage and matrix-derived coverage pressured authors to cherry-pick claims rather than reflect what the bytes actually exercised. A "primary vector per row" rule, or a lint warning on overlap, would recreate the same bad incentive — authors would omit TR-CORE rows from `coverage.tr_core` to avoid collisions with another vector's claim, degrading signal. Treating coverage as a boolean and welcoming duplicates keeps `tr_core` an honest inventory of what each vector demonstrably exercises.

## Non-goals

- Authoring the ~50 vectors themselves. Each op-dir batch is a separate plan.
- Defining the Rust reference impl (G-4). The fixture format is language-agnostic.
- Defining the stranger-impl runner (G-5). External to this repo.
- Re-normatizing the Core API surface. Manifest fields mirror Core; they do not define it.

## Open items (resolved during implementation)

- **First-batch priority.** Candidate: the 15 invariants first, then canonical_event_hash preimage, COSE_Sign1 signing, deterministic export ZIP layout. Final call at plan time.
- **verify/ subdirectory split.** Whether to split into `verify/success/` and `verify/negative-nontamper/` — defer until first batch reveals need.
- **Vector slug convention.** ~~`append/001-minimal-inline-payload` style is proposed; formalize once first batch is authored.~~ **Resolved by F6** — see "Vector lifecycle."

## Follow-ons

- **Replace `TRELLIS_SKIP_COVERAGE=1` blanket bypass with per-invariant allowlist.** The implementation plan uses `TRELLIS_SKIP_COVERAGE=1` as a transitional mechanism while vectors are authored in batches. This blanket toggle is too blunt: it silently neutralizes the audit for an extended period. The follow-on design: the lint reads `fixtures/vectors/_pending-invariants.toml` listing invariant numbers that are currently uncovered (by author declaration). The lint fails if an invariant is on the pending list but IS covered (forces list cleanup) or if an invariant is NOT on the list and is NOT covered. This preserves full audit signal throughout the rollout and drives the pending list to zero. Tracked separately; the allowlist design takes effect as a follow-on plan after initial vector batches land.

- **Lint enforcement for F6 deprecation fields.** The amendment introduces `status = "deprecated"` + `deprecated_at` but does not yet wire lint checks. A follow-on pass to `scripts/check-specs.py` should: (a) require `deprecated_at` iff `status = "deprecated"`; (b) validate `deprecated_at` parses as an ISO-8601 date; (c) exclude deprecated vectors from the set that satisfies byte-testable coverage in rule 3; (d) reject `status` values other than `"active"` or `"deprecated"`. Tracked alongside the other G-3 lint work. Another agent is extending `check-specs.py` concurrently — do not modify it as part of this amendment.

- **Lint enforcement for F6 renumbering-forbidden rule.** No purely local lint can catch renumbering after the fact, but a pre-merge check is possible: compare the set of `<op>/NNN-` prefixes on the current branch against the set on the ratification branch. A prefix that existed upstream but is missing locally (and is not a rename preserving `NNN-`) fails the check. This is a CI-level guard, not a `check-specs.py` rule. Scoped to the lint follow-on plan.

## Consumers

Once ratified, this design is consumed by:

1. An implementation plan to scaffold `fixtures/vectors/` and author the first-slice batch (likely 5–8 vectors covering invariants #1, #3, #7, #11 plus one tamper case). That plan lives at `thoughts/plans/…` and is written via the `superpowers:writing-plans` skill.
2. Follow-on plans, one per subsequent op-dir batch.
3. G-4 plan: the Rust reference impl consumes `fixtures/vectors/` as its test corpus.
4. G-5 plan: the stranger implementor reads Core + Companion + Agreement, writes their own runner, consumes the same fixtures.
