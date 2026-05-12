// Rust guideline compliant 2026-02-21
//! Trellis COSE compatibility helpers.
//!
//! The shared COSE implementation lives in `integrity-cose`. This crate keeps
//! the existing Trellis crate name while re-exporting the common implementation.

#![forbid(unsafe_code)]

pub use integrity_cose::{
    COSE_LABEL_ALG, COSE_LABEL_KID, COSE_LABEL_PROFILE_ID, COSE_LABEL_SUITE_ID,
    COSE_PROFILE_ID_LABEL_MAGNITUDE, COSE_SIGN1_TAG, COSE_SUITE_ID_LABEL_MAGNITUDE, CoseError,
    CoseSign1, SUITE_ID_PHASE_1, decode_cose_sign1, decode_cose_sign1_array,
    decode_cose_sign1_value, derive_kid, encode_cose_profile_id_label, encode_cose_sign1,
    encode_cose_suite_id_label, protected_header_bytes, protected_header_bytes_for_alg,
    protected_header_bytes_with_profile_id, sig_structure_bytes, sign_ed25519, sign1_bytes,
    sign1_detached_bytes, verify_ed25519_sign1, verify_ed25519_signature,
};
