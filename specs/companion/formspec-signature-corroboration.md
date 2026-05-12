# Formspec Signature Corroboration via UCA

**Status:** Accepted (implementation deferred to adapter integration follow-up)
**Date:** 2026-05-08
**Scope:** Trellis companion spec — defines how `trellis.user-content-attestation.v1` carries Formspec COSE_Sign1 signature bytes for integrity corroboration.

## Purpose

`trellis.user-content-attestation.v1` (UCA) can carry the COSE_Sign1 `signatureValue` bytes from a Formspec authored signature. UCA provides byte-level integrity attestation — it proves that the same signature bytes were sealed in the Trellis ledger. UCA does NOT replace Formspec cryptographic verification; verification remains via the Formspec verifier port.

## Binding

- UCA carries `signatureValue` bytes in its payload extension.
- The `signingIntent` URI from the Formspec Response flows through to UCA's `signing_intent` field.
- A `VerificationReceipt` (ADR-0088) MAY be embedded in UCA as `uca.payload.formspecSignatureReceipt` (COSE_Sign1 bytes) or referenced separately.
- The chain position is attested via `attested_event_hash` + `attested_event_position`.
- Identity binding is attested via `identity_attestation_ref` (references an identity-attestation event whose subject equals the attestor).

## Verification Boundary

Formspec cryptographic verification remains outside the Trellis center. Trellis
uses shared COSE_Sign1 primitives to decode and corroborate byte identity, while
Formspec verifier adapters own `VerificationReceipt` production and registry
policy for Formspec signature methods.

Implementation follow-up:
- Keep Trellis UCA/certificate verification receipt-aware without making Trellis
  a Formspec verifier implementation.
- Use the shared COSE primitive for any overlapping Sig_structure or COSE_Sign1
  byte behavior.
- PQC suites remain composable as Trellis adds them.
- Receipt signing uses the owner verifier service's signing keys.
