# Trellis G-5 Stranger Implementation — Commission Brief

**Date:** 2026-04-21
**Purpose:** Commission an independent `trellis-py` or `trellis-go`
implementation for ratification gate **G-5**.
**Constraint:** The implementation must be honestly independent. The
implementor reads only the allowed materials below and must not inspect the
existing Trellis implementation, generators, or planning docs.

## What this is

Trellis Phase 1 ratifies only if a second implementor can read the specs,
write their own implementation, and byte-match the committed vector corpus.
This brief defines the allowed read set, forbidden read set, required
deliverables, and acceptance bar for that implementor.

This is not a request to "port the Rust code." It is a request to
independently derive the implementation from the normative docs and the
committed vectors.

## Handoff package

The reviewer or project owner SHOULD hand the implementor the packaged allowed
read set archive at:

- `ratification/g5-package/trellis-g5-allowed-readset-2026-04-21.tar.gz`

That archive is the concrete delivery vehicle for the allowed read set below.
Do not hand the implementor the surrounding `ratification/` directory, its
README, or its checksum files; those are reviewer-side materials only.

## Allowed inputs

The implementor MAY read only these repo paths:

- `specs/trellis-core.md`
- `specs/trellis-operational-companion.md`
- `specs/trellis-agreement.md`
- `fixtures/vectors/README.md`
- `fixtures/vectors/append/`
- `fixtures/vectors/verify/`
- `fixtures/vectors/export/`
- `fixtures/vectors/tamper/`
- `fixtures/vectors/projection/`
- `fixtures/vectors/shred/`
- `fixtures/vectors/_keys/`
- `fixtures/vectors/_inputs/`

Normative references cited from the three spec documents are allowed because
the specs themselves depend on them.

## Forbidden inputs

The implementor MUST NOT read, grep, index, summarize, or receive excerpts
from any of these:

- `crates/`
- `fixtures/vectors/_generator/`
- `thoughts/`
- `ratification/`
- `scripts/`
- `TODO.md`
- `COMPLETED.md`
- `README.md` other than `fixtures/vectors/README.md`
- any prior PR, commit diff, review note, or chat transcript describing how
  the current implementation works

If an AI assistant is used, it MUST be started in a clean thread with only the
allowed inputs above. Do not paste forbidden material into that thread.

## Work to perform

Build an independent implementation in either Python or Go that can consume
the committed vector corpus and reproduce the expected outputs byte-for-byte.

Minimum required public behavior:

- `append`
- `verify`
- `export`

Companion-fixture support is also required where needed to consume the current
corpus:

- projection fixtures
- shred fixtures

The implementor writes their own local conformance runner. There is no shared
runner protocol.

## Required deliverables

Deliver all of the following:

1. A standalone `trellis-py` or `trellis-go` codebase.
2. A local conformance runner that walks the committed vector corpus and
   reports pass/fail per vector.
3. A manifest of the exact allowed inputs used during implementation.
4. A short attestation that the forbidden paths were not consulted.
5. A byte-match report against the committed corpus.
6. A discrepancy log for any vector that does not match, citing only spec
   sections and vector artifacts, never the Rust implementation.

## Independence rules

- Do not ask which implementation is "right." The specs are the authority.
- Do not ask for algorithmic hints derived from the Rust code or generators.
- If the specs seem ambiguous, stop and file a spec issue citing the relevant
  section and vector. Ambiguity surfacing is a valid G-5 outcome.
- Do not normalize bytes by ad hoc post-processing to force a match. The local
  implementation must emit the expected bytes directly.
- Do not delete or skip vectors because they are inconvenient. If a vector
  appears wrong, log the discrepancy and the spec basis for the objection.

## Acceptance bar

G-5 closes only when all of the following are true:

1. The implementor used only the allowed inputs.
2. The implementation is independently written.
3. The committed vector corpus byte-matches end-to-end.
4. Any non-match is framed as a spec/vector discrepancy, not patched by
   consulting the existing implementation.

## Handoff note

The project owner or reviewer may inspect the resulting implementation and its
byte-match report. That review is for evidence capture, not for feeding the
implementor hints from the existing reference implementation.
