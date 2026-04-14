# Trellis Companion — Shared Ledger Binding (Draft)

## Status

Draft extracted from `DRAFTS/unified_ledger_companion.md` for normalization.

## Purpose

Bind Formspec-family, WOS-family, trust/access-family, and release-family facts to one Trellis canonical substrate without redefining source-repo semantics.

## Normative Focus

1. Family IDs and minimum required fields by family.
2. Schema/version references and upgrade rules.
3. Canonization eligibility and rejection reasons.
4. Cross-family reference rules (e.g., authored fact to governance fact links).
5. Binding conformance matrix for producers, appenders, verifiers.

## Family binding matrix (draft scaffold)

| Family | Authority | Required minimum fields (draft) | Notes |
|---|---|---|---|
| Formspec-authored facts | Formspec | `family_id`, `schema_ref`, `authored_at`, `author_ref`, `payload_ref` | Trellis binds, does not redefine authored semantics. |
| WOS governance/workflow facts | WOS | `family_id`, `schema_ref`, `governance_scope`, `actor_ref`, `payload_ref` | Runtime meaning remains in WOS. |
| Trust/access facts | Trellis companions | `family_id`, `profile_ref`, `policy_ref`, `effective_at`, `subject_ref` | Must align with trust-profile declarations. |
| Release/export/disclosure facts | Trellis companions | `family_id`, `package_ref`, `audience_ref`, `readability_ref`, `version_ref` | Used for verifier/export semantics. |

## Canonization rules (draft)

1. Binding MUST reject records missing required minimum fields for the declared family.
2. Binding MUST carry stable schema/version references for verification portability.
3. Binding MUST preserve canonical hash construction from Trellis core without family-specific overrides.
4. Binding MUST NOT permit two families to create competing canonical order claims for the same governed scope.

## Schema/version compatibility policy (draft)

1. Backward-compatible additive fields MAY be accepted within a declared major version.
2. Breaking semantic changes MUST require a new major schema reference.
3. Verifiers MUST reject unknown major versions unless an explicit compatibility adapter is declared.

## Canonization rejection codes (draft)

- `missing_required_field`
- `invalid_schema_ref`
- `unsupported_major_version`
- `hash_construction_mismatch`
- `scope_order_conflict`

## Deferral Rules

- Formspec remains authoritative for authored/spec/runtime semantics.
- WOS remains authoritative for workflow/governance meaning and runtime envelope.
- Trellis binding must not reinterpret Formspec or WOS meaning.

## Core handoff note

Core admission semantics are defined in `trellis-core.md`.

This companion owns schema/version compatibility policy and machine-testable rejection codes used during canonical admission.

## Migrated requirements from `unified_ledger_core.md` (Section 15)

1. Domain vocabularies MUST be bound through family IDs and schema references, not by redefining core ontology.
2. Family bindings MUST remain subordinate to core canonical truth/order/hash semantics.
3. Sidecar and binding content MUST NOT create alternate canonical truth or competing canonical order for the same governed scope.
