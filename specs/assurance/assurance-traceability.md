# Trellis Companion — Assurance Traceability (Draft)

## Status

Draft started to make assurance methodology first-class in the spec family.

## Purpose

Map each core invariant to concrete assurance methods so assurance remains architectural, not appendix-only.

## Traceability matrix (draft)

| Core invariant | Primary methods | Secondary methods | Evidence artifacts |
|---|---|---|---|
| Append-only canonical history | TLA+ model of append transitions | Property-based append tests | Model files, passing model checks, test logs |
| One canonical order per governed scope | TLA+ scope/order invariants | Adversarial replay tests | Scope-partition proofs, replay test reports |
| One canonical event hash construction | Shared test vectors (native + WASM) | Parser/verifier fuzzing | Vector fixtures, fuzz corpus/crash reports |
| No second canonical truth | Alloy constraints on canonical/derived separation | Rebuild-from-canonical drills | Alloy model checks, rebuild drill logs |
| Append idempotency | Property-based idempotency tests | Failure/retry chaos tests | Deterministic idempotency test suite outputs |
| Trust profile honesty | Profile conformance tests | Metadata-budget disclosure audits | Profile declarations + audit records |
| Key lifecycle correctness | State-transition property tests | Rotation/recovery drills | Transition test reports, recovery drill artifacts |
| Crypto-shredding completeness | Purge-cascade integration tests | Snapshot/cache residue scans | Purge verification reports |
| Export offline verifiability | Offline verifier cross-implementation vectors | Package corruption fuzz tests | Verifier outputs, corruption-detection logs |

## Minimum CI expectations (draft)

1. Every normative invariant MUST map to at least one automated check.
2. Hash/serialization vectors MUST run in native and WASM paths.
3. Recovery and destruction drills MUST run on a recurring schedule and produce retained evidence.
4. Fuzzing outcomes MUST feed parser/verifier hardening backlogs.

## Evidence retention policy (draft)

- Assurance artifacts SHOULD be retained for at least one full major-version support window.
- Artifacts MUST include build/version identifiers and execution timestamps.
- Failed assurance runs MUST be retained with remediation linkage.
