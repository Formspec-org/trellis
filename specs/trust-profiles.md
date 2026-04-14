# Trellis Companion — Trust Profiles (Draft)

## Status

Draft extracted from `DRAFTS/unified_ledger_companion.md` for normalization.

## Purpose

Define explicit custody/readability postures and trust-honesty declarations.

## Baseline Profiles

1. Reader-held by default.
2. Provider-readable (explicit, not implied).
3. Tenant-operated key plane.

## Mandatory Profile Declarations

- who can decrypt what,
- visible metadata classes,
- stable-linkage behavior,
- delegated compute behavior,
- recovery paths,
- destruction/disclosure authority.

## Metadata Budget Requirement

Each profile must include a metadata budget table by canonical fact family:

- visible fields,
- observer classes,
- timing/access-pattern leakage,
- linkage stability,
- delegated-compute effects.

## Trust honesty rule (draft)

A deployment MUST NOT claim a stronger trust posture than it operationally provides.

At minimum, each profile declaration MUST make explicit:

1. whether provider operators can decrypt protected payload classes,
2. whether delegated compute introduces readable exposure beyond nominal posture,
3. whether recovery paths can reintroduce provider readability,
4. what destruction guarantees exist and what residual metadata remains.

## Profile declaration schema (draft)

Required declaration object fields:

- `profile_id`
- `decryptor_classes`
- `metadata_budget_ref`
- `delegated_compute_mode`
- `recovery_mode`
- `destruction_semantics`
- `disclosure_authority`

## Conformance audit hooks (draft)

- Deployments MUST publish machine-readable profile declarations.
- Auditors MUST be able to compare declared posture versus observed control-plane behavior.
- Any mismatch between declaration and operation MUST be reported as trust-honesty nonconformance.

## Operational trust disclosure requirements (draft)

- Trust claims MUST be consistent with declared decryptability, delegated compute behavior, recovery behavior, and destruction semantics.
- Metadata-leakage characteristics MUST be declared at profile level and MUST NOT be omitted from trust posture statements.

## Migrated requirements from `unified_ledger_core.md` (Sections 10, 11, 14)

1. Trust profile objects MUST declare disclosure posture and assurance posture explicitly.
2. Trust profile transitions MUST be append-attributable and auditable.
3. Reader-held and delegated-compute semantics MUST remain distinguishable in declarations and audits.
4. Profile-specific export claims MUST NOT overstate payload readability.
