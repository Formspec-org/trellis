// Rust guideline compliant 2026-02-21
//! Shared Trellis types and byte helpers.
//!
//! This crate keeps the Phase-1 append scaffold on `std` types and fixed
//! byte constructions. It intentionally exposes only Trellis-owned types so
//! sibling crates do not leak third-party APIs through their public surface.

#![forbid(unsafe_code)]

use sha2::{Digest, Sha256};

/// HKDF info string for payload-inline nonce derivation (Core §9.4).
pub const PAYLOAD_NONCE_INFO: &[u8] = b"trellis-payload-nonce-v1";

/// Phase-1 ChaCha20-Poly1305 nonce length in bytes.
pub const PAYLOAD_NONCE_LEN: usize = 12;

/// Domain tag for `author_event_hash`.
pub const AUTHOR_EVENT_DOMAIN: &str = "trellis-author-event-v1";

/// Domain tag for `content_hash`.
pub const CONTENT_DOMAIN: &str = "trellis-content-v1";

/// Domain tag for `canonical_event_hash`.
pub const EVENT_DOMAIN: &str = "trellis-event-v1";

/// Phase-1 Trellis signature suite identifier (Core §7 suite registry).
pub const SUITE_ID_PHASE_1: u64 = 1;

/// COSE protected-header map label for Trellis `suite_id` (Core §7.4, RFC 9052 §3.1).
///
/// This value must stay aligned with Python `COSE_LABEL_SUITE_ID` in
/// `fixtures/vectors/_generator/_lib/byte_utils.py` and with every runtime
/// that builds or parses Phase-1 protected headers.
pub const COSE_LABEL_SUITE_ID: i128 = -65_537;

/// Unsigned magnitude `n` such that the CBOR negative integer `-1 - n` equals
/// [`COSE_LABEL_SUITE_ID`] (here `n = 65536` gives `-65537`).
pub const COSE_SUITE_ID_LABEL_MAGNITUDE: u64 = 65_536;

/// Signed and canonical event bytes stored after a successful append.
///
/// `idempotency_key` is the optional Core §6.1 / §17 wire-contract
/// identity. Phase-1 callers that have already extracted the key from the
/// authored event (the §17.3 retry-conflict resolution path) pass it
/// through [`StoredEvent::with_idempotency_key`]; legacy callers that
/// have not yet been threaded use [`StoredEvent::new`] which defaults to
/// `None`. The stores read the key via [`StoredEvent::idempotency_key`]
/// to enforce the §17.3 unique-`(scope, key)` invariant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StoredEvent {
    scope: Vec<u8>,
    sequence: u64,
    canonical_event: Vec<u8>,
    signed_event: Vec<u8>,
    idempotency_key: Option<Vec<u8>>,
}

impl StoredEvent {
    /// Creates a stored event snapshot without an `idempotency_key`.
    ///
    /// Phase-1 callers prefer [`StoredEvent::with_idempotency_key`] when the
    /// authored event has been parsed; this constructor stays available for
    /// legacy / structural-only callers.
    ///
    /// # Examples
    /// ```rust
    /// use trellis_types::StoredEvent;
    ///
    /// let event = StoredEvent::new(b"scope".to_vec(), 0, vec![0x01], vec![0x02]);
    /// assert_eq!(event.sequence(), 0);
    /// assert!(event.idempotency_key().is_none());
    /// ```
    pub fn new(
        scope: Vec<u8>,
        sequence: u64,
        canonical_event: Vec<u8>,
        signed_event: Vec<u8>,
    ) -> Self {
        Self {
            scope,
            sequence,
            canonical_event,
            signed_event,
            idempotency_key: None,
        }
    }

    /// Creates a stored event snapshot carrying its Core §6.1 `idempotency_key`.
    ///
    /// The caller MUST have already validated that `idempotency_key.len()` is
    /// in the closed interval `[IDEMPOTENCY_KEY_MIN_LEN, IDEMPOTENCY_KEY_MAX_LEN]`
    /// (see [`IDEMPOTENCY_KEY_MIN_LEN`] / [`IDEMPOTENCY_KEY_MAX_LEN`]). This
    /// constructor does not re-validate; the store-side `append_event_in_tx`
    /// path is the load-bearing length check.
    pub fn with_idempotency_key(
        scope: Vec<u8>,
        sequence: u64,
        canonical_event: Vec<u8>,
        signed_event: Vec<u8>,
        idempotency_key: Vec<u8>,
    ) -> Self {
        Self {
            scope,
            sequence,
            canonical_event,
            signed_event,
            idempotency_key: Some(idempotency_key),
        }
    }

    /// Returns the ledger scope bytes.
    pub fn scope(&self) -> &[u8] {
        &self.scope
    }

    /// Returns the sequence number within the ledger scope.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the canonical event bytes.
    pub fn canonical_event(&self) -> &[u8] {
        &self.canonical_event
    }

    /// Returns the signed COSE event bytes.
    pub fn signed_event(&self) -> &[u8] {
        &self.signed_event
    }

    /// Returns the Core §6.1 `idempotency_key` if it was threaded through the
    /// authored-event parse, otherwise `None`. Used by `LedgerStore` impls
    /// to enforce the §17.3 unique-`(ledger_scope, idempotency_key)` invariant.
    pub fn idempotency_key(&self) -> Option<&[u8]> {
        self.idempotency_key.as_deref()
    }
}

/// Minimum byte length of `idempotency_key` per Core §6.1 / §17.2 (`bstr .size (1..64)`).
pub const IDEMPOTENCY_KEY_MIN_LEN: usize = 1;

/// Maximum byte length of `idempotency_key` per Core §6.1 / §17.2 (`bstr .size (1..64)`).
pub const IDEMPOTENCY_KEY_MAX_LEN: usize = 64;

/// Returns `true` iff `key` satisfies the Core §6.1 `bstr .size (1..64)` bound.
#[must_use]
pub fn idempotency_key_length_in_bound(key: &[u8]) -> bool {
    (IDEMPOTENCY_KEY_MIN_LEN..=IDEMPOTENCY_KEY_MAX_LEN).contains(&key.len())
}

/// The append head returned after a successful append.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppendHead {
    scope: Vec<u8>,
    sequence: u64,
    canonical_event_hash: [u8; 32],
}

impl AppendHead {
    /// Creates a new append head value.
    pub fn new(scope: Vec<u8>, sequence: u64, canonical_event_hash: [u8; 32]) -> Self {
        Self {
            scope,
            sequence,
            canonical_event_hash,
        }
    }

    /// Returns the ledger scope bytes.
    pub fn scope(&self) -> &[u8] {
        &self.scope
    }

    /// Returns the sequence number.
    pub fn sequence(&self) -> u64 {
        self.sequence
    }

    /// Returns the canonical event hash.
    pub fn canonical_event_hash(&self) -> [u8; 32] {
        self.canonical_event_hash
    }
}

/// Byte artifacts produced by the current append scaffold.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppendArtifacts {
    pub author_event_hash: [u8; 32],
    pub canonical_event_hash: [u8; 32],
    pub protected_header: Vec<u8>,
    pub sig_structure: Vec<u8>,
    pub canonical_event: Vec<u8>,
    pub signed_event: Vec<u8>,
    pub append_head: Vec<u8>,
}

/// Encodes a CBOR byte string.
pub fn encode_bstr(bytes: &[u8]) -> Vec<u8> {
    let mut encoded = encode_major_len(2, bytes.len() as u64);
    encoded.extend_from_slice(bytes);
    encoded
}

/// Encodes a CBOR text string.
pub fn encode_tstr(text: &str) -> Vec<u8> {
    let mut encoded = encode_major_len(3, text.len() as u64);
    encoded.extend_from_slice(text.as_bytes());
    encoded
}

/// Encodes a CBOR unsigned integer.
pub fn encode_uint(value: u64) -> Vec<u8> {
    encode_major_len(0, value)
}

/// Encodes a CBOR negative integer `-1 - n` (RFC 8949 major type 1).
///
/// For example, `n == 7` yields `-8` (EdDSA `alg` value in COSE headers).
#[must_use]
pub fn encode_cbor_negative_int(n: u64) -> Vec<u8> {
    encode_major_len(1, n)
}

/// Encodes the CBOR map key bytes for [`COSE_LABEL_SUITE_ID`].
///
/// Equivalent to canonical CBOR for integer `-65537` (`-1 - 65536`).
#[must_use]
pub fn encode_cose_suite_id_label() -> Vec<u8> {
    encode_major_len(1, COSE_SUITE_ID_LABEL_MAGNITUDE)
}

/// Computes a Trellis domain-separated SHA-256 digest.
pub fn domain_separated_sha256(tag: &str, component: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update((tag.len() as u32).to_be_bytes());
    hasher.update(tag.as_bytes());
    hasher.update((component.len() as u32).to_be_bytes());
    hasher.update(component);
    hasher.finalize().into()
}

fn encode_major_len(major: u8, value: u64) -> Vec<u8> {
    let header = major << 5;
    match value {
        0..=23 => vec![header | value as u8],
        24..=0xff => vec![header | 24, value as u8],
        0x100..=0xffff => {
            let mut encoded = vec![header | 25];
            encoded.extend_from_slice(&(value as u16).to_be_bytes());
            encoded
        }
        0x1_0000..=0xffff_ffff => {
            let mut encoded = vec![header | 26];
            encoded.extend_from_slice(&(value as u32).to_be_bytes());
            encoded
        }
        _ => {
            let mut encoded = vec![header | 27];
            encoded.extend_from_slice(&value.to_be_bytes());
            encoded
        }
    }
}

/// Derives a deterministic 12-byte ChaCha20-Poly1305 nonce for `PayloadInline`.
///
/// Construction per Core §9.4:
/// ```text
/// nonce = HKDF-SHA256(
///     salt = dCBOR(idempotency_key),
///     ikm  = SHA-256(plaintext_payload),
///     info = "trellis-payload-nonce-v1",
///     length = 12
/// )
/// ```
///
/// # Errors
/// Returns `None` if the HKDF expansion fails (should not happen for length 12).
#[must_use]
pub fn derive_payload_nonce(idempotency_key: &[u8], plaintext_payload: &[u8]) -> Option<[u8; PAYLOAD_NONCE_LEN]> {
    use hkdf::Hkdf;
    let salt = encode_bstr(idempotency_key);
    let ikm = Sha256::digest(plaintext_payload);
    let (prk, _) = Hkdf::<Sha256>::extract(Some(&salt), &ikm);
    let hk = Hkdf::<Sha256>::from_prk(&prk).ok()?;
    let mut okm = [0u8; PAYLOAD_NONCE_LEN];
    hk.expand(PAYLOAD_NONCE_INFO, &mut okm).ok()?;
    Some(okm)
}

#[cfg(test)]
mod tests {
    use super::{encode_cose_suite_id_label, encode_uint};

    #[test]
    fn encode_uint_matches_single_byte_for_small_suite_ids() {
        assert_eq!(encode_uint(1), vec![0x01]);
    }

    #[test]
    fn encode_cose_suite_id_label_matches_historical_bytes() {
        assert_eq!(
            encode_cose_suite_id_label(),
            vec![0x3a, 0x00, 0x01, 0x00, 0x00]
        );
    }

    #[test]
    fn derive_payload_nonce_is_deterministic_and_length_correct() {
        use crate::{PAYLOAD_NONCE_LEN, derive_payload_nonce};
        let key = b"idemp-append-041";
        let plaintext = b"some-plaintext-bytes";
        let n1 = derive_payload_nonce(key, plaintext).unwrap();
        let n2 = derive_payload_nonce(key, plaintext).unwrap();
        assert_eq!(n1.len(), PAYLOAD_NONCE_LEN);
        assert_eq!(n1, n2);
    }

    #[test]
    fn derive_payload_nonce_changes_with_key_or_plaintext() {
        use crate::derive_payload_nonce;
        let key_a = b"key-a";
        let key_b = b"key-b";
        let pt_x = b"plaintext-x";
        let pt_y = b"plaintext-y";
        let n_ax = derive_payload_nonce(key_a, pt_x).unwrap();
        let n_ay = derive_payload_nonce(key_a, pt_y).unwrap();
        let n_bx = derive_payload_nonce(key_b, pt_x).unwrap();
        assert_ne!(n_ax, n_ay, "different plaintext should yield different nonce");
        assert_ne!(n_ax, n_bx, "different key should yield different nonce");
    }
}
