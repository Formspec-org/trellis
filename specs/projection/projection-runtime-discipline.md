# Trellis Companion — Projection and Runtime Discipline (Draft)

## Status

Draft started from normalization plan; intended to separate Trellis projection invariants from WOS runtime semantics.

## Purpose

Define strict rules for derived systems so canonical truth never drifts into hidden second truth.

## Normative Focus

1. Projection provenance watermarking
   - staff-facing derived views MUST carry a watermark indicating canonical append/checkpoint state.

2. Rebuild contract
   - derived systems MUST be discardable/rebuildable from canonical facts and append attestations.

3. Snapshot discipline
   - snapshots are operational artifacts; they MUST reference canonical checkpoint state and remain non-canonical.

4. Purge-cascade requirement
   - crypto-shredding is incomplete unless plaintext-derived projections/caches are purged according to policy.

5. Runtime boundary
   - workflow/orchestration engines are derived processors, not canonical ledgers.

## Projection watermark contract (draft)

Every staff-facing projection MUST expose:

1. canonical checkpoint identifier,
2. canonical append height/sequence at build time,
3. projection build timestamp,
4. projection schema/version identifier.

If the projection is stale relative to a newer canonical checkpoint, the view MUST indicate stale status.

## Rebuild verification (draft)

- Rebuilding a projection from canonical records for the same checkpoint MUST yield semantically equivalent output for declared projection fields.
- Systems SHOULD retain deterministic rebuild fixtures for critical projection types.
- Projection conformance tests MUST validate watermark presence and stale-status behavior.

## Deferral to WOS

- execution semantics,
- runtime envelope and governance-time behavior,
- orchestration policy specifics.

## Migrated requirements from `unified_ledger_core.md` (Sections 16.1 and 16.4)

1. Derived artifacts MUST be discardable and rebuildable from canonical records + append/checkpoint material.
2. Snapshots MUST remain operational artifacts and MUST carry canonical checkpoint provenance.
3. Rebuild and snapshot procedures MUST NOT mutate canonical truth.
