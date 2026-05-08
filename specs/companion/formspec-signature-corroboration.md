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

## Trellis Formspec Signature Adapter

A new consumer-owned adapter crate `trellis/crates/trellis-formspec-signature/` implements the Formspec `Verifier` trait (from `formspec-signature-port`) using `trellis-cose` primitives. This is a consumer-owned adapter per ADR-0086 D-7 dependency inversion — it is NOT a Trellis center crate.

Implementation deferred to Phase 2.5 follow-up (requires COSE library integration). The adapter:
- Implements `verify(signed_bytes, signature_bytes, signature_method, key_ref) -> VerificationReceipt`
- Same registry coverage as webcrypto/ring (ed25519, ecdsa-p256, rsa-pss-sha256)
- PQC suites composable as Trellis adds them
- Receipt signing uses Trellis-managed signing keys
