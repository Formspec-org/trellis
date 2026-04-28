// Rust guideline compliant 2026-04-27
//! Reserved interop adapter crate (ADR 0008).
//!
//! Phase-1 scope: reservation only. The crate exists to lock the name
//! and enforce cargo-deny hygiene; adapter logic lands when the kind
//! activates per its stated trigger.

#![forbid(unsafe_code)]
