# Trellis Companion — Export Verification Package (Draft)

## Status

Draft started from normalization plan and companion split.

## Purpose

Define offline-verifiable export package requirements for canonical integrity claims.

## Normative Focus

1. Required package members
   - canonical records in scope,
   - append attestations/checkpoint material,
   - verification manifest and schema/version references.

2. Payload readability declaration
   - package MUST declare what payload content is readable, encrypted, or intentionally omitted.

3. Trust-profile carriage
   - package SHOULD include the active trust-profile declaration and metadata-budget reference.

4. Verification mode
   - verifier MUST be able to validate integrity and append claims offline.

5. Optional external anchoring seam
   - package MAY include anchoring references (e.g., OpenTimestamps) without making anchoring mandatory.

## Verification manifest minimum fields (draft)

1. package format/version identifier,
2. canonical checkpoint reference,
3. included claim classes,
4. hash/canonicalization algorithm identifiers,
5. trust-profile reference (if declared),
6. disclosure/readability declarations.

## Cross-implementation verification requirement (draft)

At least two independent verifier implementations SHOULD validate the same package fixture set and produce equivalent claim outcomes.

## Provenance distinction requirement (draft)

Export packages MUST preserve distinction among:

1. canonical records,
2. canonical append attestations,
3. derived release/disclosure artifacts.

## Migrated requirements from `unified_ledger_core.md` (Section 12)

1. Export packages MUST include sufficient material to verify declared claim classes.
2. Export verification MUST remain independent of runtime-only derived artifacts.
3. Exports MUST preserve provenance distinction among authored facts, canonical records, append attestations, and disclosure artifacts.
