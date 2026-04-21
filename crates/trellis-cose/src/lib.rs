// Rust guideline compliant 2026-02-21
//! COSE helpers for the Phase-1 append scaffold.

#![forbid(unsafe_code)]

use ed25519_dalek::{Signature, Signer, SigningKey};
use sha2::{Digest, Sha256};
use trellis_types::{
    encode_bstr, encode_cbor_negative_int, encode_cose_suite_id_label, encode_tstr, encode_uint,
    SUITE_ID_PHASE_1,
};

/// Derives the 16-byte `kid` from `suite_id` and the Ed25519 public key.
///
/// The preimage uses canonical CBOR unsigned encoding for `suite_id`, matching
/// Python `dcbor(suite_id)` in `fixtures/vectors/_generator/gen_v3_remaining.py`
/// (and therefore differs from a raw single byte when `suite_id >= 24`).
pub fn derive_kid(suite_id: u8, public_key: [u8; 32]) -> [u8; 16] {
    let mut hasher = Sha256::new();
    hasher.update(&encode_uint(suite_id as u64));
    hasher.update(public_key);
    let digest: [u8; 32] = hasher.finalize().into();
    let mut kid = [0u8; 16];
    kid.copy_from_slice(&digest[..16]);
    kid
}

/// Builds the protected-header map bytes.
pub fn protected_header_bytes(kid: [u8; 16]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(32);
    bytes.push(0xa3);
    bytes.extend_from_slice(&encode_uint(1));
    bytes.extend_from_slice(&encode_cbor_negative_int(7));
    bytes.extend_from_slice(&encode_uint(4));
    bytes.extend_from_slice(&encode_bstr(&kid));
    bytes.extend_from_slice(&encode_cose_suite_id_label());
    bytes.extend_from_slice(&encode_uint(SUITE_ID_PHASE_1));
    bytes
}

/// Builds the RFC 9052 `Sig_structure`.
pub fn sig_structure_bytes(protected_header: &[u8], payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push(0x84);
    bytes.extend_from_slice(&encode_tstr("Signature1"));
    bytes.extend_from_slice(&encode_bstr(protected_header));
    bytes.push(0x40);
    bytes.extend_from_slice(&encode_bstr(payload));
    bytes
}

/// Signs the `Sig_structure` with the pinned Ed25519 seed.
pub fn sign_ed25519(private_seed: [u8; 32], sig_structure: &[u8]) -> [u8; 64] {
    let signing_key = SigningKey::from_bytes(&private_seed);
    let signature: Signature = signing_key.sign(sig_structure);
    signature.to_bytes()
}

/// Builds the tagged COSE_Sign1 envelope bytes.
pub fn sign1_bytes(protected_header: &[u8], payload: &[u8], signature: [u8; 64]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push(0xd2);
    bytes.push(0x84);
    bytes.extend_from_slice(&encode_bstr(protected_header));
    bytes.push(0xa0);
    bytes.extend_from_slice(&encode_bstr(payload));
    bytes.extend_from_slice(&encode_bstr(&signature));
    bytes
}
