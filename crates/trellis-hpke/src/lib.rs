// Rust guideline compliant 2026-02-21

//! Re-exports shared HPKE primitives from `integrity-hpke`.
//!
//! HPKE ownership is promoted to `integrity-stack` per ADR-0074. This crate
//! remains as the Trellis-facing package name so existing Trellis workspace
//! tests and consumers do not gain a second HPKE implementation.

pub use integrity_hpke::{
    HPKE_SUITE1_AAD, HPKE_SUITE1_INFO, HpkeError, WrapResult, unwrap_dek, wrap_dek,
};

#[cfg(feature = "test-vectors")]
pub use integrity_hpke::wrap_dek_with_pinned_ephemeral;
