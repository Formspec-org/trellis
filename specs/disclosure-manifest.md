# Trellis Companion — Disclosure Manifest (Draft)

## Status

Draft started from normalization plan and companion split.

## Purpose

Define audience-scoped disclosure semantics as first-class release artifacts, separate from canonical append records.

## Normative Focus

1. Disclosure scope and audience declaration.
2. Claim-class declaration (authorship, append, payload integrity, authorization history, etc.).
3. Provenance preservation from canonical facts to disclosed claims.
4. Selective disclosure semantics without canonical rewrite.
5. Payload readability and redaction declarations.

## Claim-class taxonomy (draft)

1. authorship claim
2. append/inclusion claim
3. payload-integrity claim
4. authorization-history claim
5. disclosure-policy claim

Disclosure artifacts MUST preserve references back to canonical records and MUST NOT be represented as canonical rewrites.

## Interop Direction

- SD-JWT / VC profile path is preferred for early disclosure interoperability.
- Advanced privacy mechanisms (e.g., BBS selective disclosure variants) remain later-phase seams.
