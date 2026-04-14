# Trellis Companion — Key Lifecycle Operating Model (Draft)

## Status

Draft extracted from `DRAFTS/unified_ledger_companion.md` for normalization.

## Purpose

Define key lifecycle as first-class platform behavior, not implementation detail.

## Normative Focus

1. Key classes and scope boundaries.
2. Lifecycle states and allowed transitions.
3. Rotation and versioning rules.
4. Grace periods for offline/intermittent clients.
5. Recovery and re-establishment procedures.
6. Destruction / crypto-shredding semantics.
7. Historical verification across key evolution.

## Key classes (draft)

1. Tenant root / policy keys
2. Scope or ledger keys
3. Subject/record encryption keys
4. Signing/attestation keys
5. Recovery-only keys (if supported)

## Lifecycle states (draft)

| State | Meaning | Allowed transitions (draft) |
|---|---|---|
| `provisioning` | Key material is being established | `active`, `destroyed` |
| `active` | Current signing/decryption use | `rotating`, `suspended`, `destroyed` |
| `rotating` | Dual-validity/grace handling in progress | `active`, `retired`, `destroyed` |
| `retired` | No new encrypt/sign operations; verify/decrypt history as permitted | `destroyed` |
| `suspended` | Temporarily disabled by policy/incident response | `active`, `destroyed` |
| `destroyed` | Cryptographic use permanently disallowed | _(terminal)_ |

## Grace-period rule (draft)

- Rotations affecting intermittently connected clients MUST define a grace window.
- During grace windows, verification of historical signatures and controlled decryptability MUST remain predictable and declared.
- After grace expiry, stale-key writes MUST be rejected with auditable errors.

## Required Completeness Rule

Crypto-shredding is not complete unless plaintext-derived projections and caches are purged according to declared cascade policy.

## Recovery and destruction evidence requirements (draft)

1. Recovery operations MUST emit auditable events with actor, scope, and policy authority.
2. Destruction operations MUST emit auditable events with destroyed key references and effective time.
3. Purge-cascade completion MUST produce verifiable evidence artifacts tied to canonical checkpoint state.

## Migrated requirements from `unified_ledger_core.md` (Section 16.5)

1. Cryptographic inaccessibility claims MUST include scope, authority, and effective-time semantics.
2. Key-destruction claims MUST be distinguishable from payload-redaction or disclosure filtering events.
3. Historical verification across key evolution MUST remain possible where declared by policy.
