// Rust guideline compliant 2026-02-21
//! Shared Trellis types and byte helpers.
//!
//! This crate keeps the Phase-1 append scaffold on `std` types and fixed
//! byte constructions. It intentionally exposes only Trellis-owned types so
//! sibling crates do not leak third-party APIs through their public surface.

#![forbid(unsafe_code)]

use sha2::{Digest, Sha256};

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

/// Computes the SHA-256 digest of `bytes`.
pub fn sha256_bytes(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

/// Domain tag for `checkpoint_digest`.
pub const CHECKPOINT_DOMAIN: &str = "trellis-checkpoint-v1";

/// Computes a standard Trellis checkpoint digest per Core §18.2.
pub fn checkpoint_digest(scope: &[u8], payload_bytes: &[u8]) -> [u8; 32] {
    let mut preimage = Vec::new();
    preimage.push(0xa3);
    preimage.extend_from_slice(&encode_tstr("scope"));
    preimage.extend_from_slice(&encode_bstr(scope));
    preimage.extend_from_slice(&encode_tstr("version"));
    preimage.extend_from_slice(&encode_uint(1));
    preimage.extend_from_slice(&encode_tstr("checkpoint_payload"));
    preimage.extend_from_slice(payload_bytes);
    domain_separated_sha256(CHECKPOINT_DOMAIN, &preimage)
}

/// Error returned by shared CBOR map lookup helpers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CborHelperError(pub String);

impl std::fmt::Display for CborHelperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for CborHelperError {}

/// Re-export `ciborium::Value` for use in shared helpers.
pub use ciborium::Value;

/// Decodes `bytes` as a single CBOR [`Value`].
pub fn decode_cbor_value(bytes: &[u8]) -> Result<Value, CborHelperError> {
    ciborium::from_reader(bytes).map_err(|error| {
        CborHelperError(format!("failed to decode CBOR: {error}"))
    })
}

/// Performs a case-sensitive map lookup for a text key.
pub fn map_lookup_optional_value<'a>(
    map: &'a [(Value, Value)],
    key_name: &str,
) -> Option<&'a Value> {
    map.iter()
        .find(|(key, _)| key.as_text().is_some_and(|text| text == key_name))
        .map(|(_, value)| value)
}

/// Performs a case-sensitive map lookup for a text key, returning an error if missing.
pub fn map_lookup_value<'a>(map: &'a [(Value, Value)], key_name: &str) -> Result<&'a Value, CborHelperError> {
    map_lookup_optional_value(map, key_name)
        .ok_or_else(|| CborHelperError(format!("missing `{key_name}` value")))
}

/// Looks up a byte string field in a map.
pub fn map_lookup_bytes(map: &[(Value, Value)], key_name: &str) -> Result<Vec<u8>, CborHelperError> {
    map_lookup_value(map, key_name).and_then(|value| {
        value
            .as_bytes()
            .cloned()
            .ok_or_else(|| CborHelperError(format!("`{key_name}` is not a byte string")))
    })
}

/// Looks up a fixed-length byte string field in a map.
pub fn map_lookup_fixed_bytes(
    map: &[(Value, Value)],
    key_name: &str,
    expected_len: usize,
) -> Result<Vec<u8>, CborHelperError> {
    let bytes = map_lookup_bytes(map, key_name)?;
    if bytes.len() != expected_len {
        return Err(CborHelperError(format!(
            "`{key_name}` must be {expected_len} bytes"
        )));
    }
    Ok(bytes)
}

/// Looks up an optional byte string field in a map.
pub fn map_lookup_optional_bytes(
    map: &[(Value, Value)],
    key_name: &str,
) -> Result<Option<Vec<u8>>, CborHelperError> {
    match map_lookup_optional_value(map, key_name) {
        Some(Value::Bytes(bytes)) => Ok(Some(bytes.clone())),
        Some(Value::Null) | None => Ok(None),
        Some(_) => Err(CborHelperError(format!(
            "`{key_name}` is neither bytes nor null"
        ))),
    }
}

/// Looks up an optional fixed-length byte string field in a map.
pub fn map_lookup_optional_fixed_bytes(
    map: &[(Value, Value)],
    key_name: &str,
    expected_len: usize,
) -> Result<Option<Vec<u8>>, CborHelperError> {
    match map_lookup_optional_bytes(map, key_name)? {
        Some(bytes) if bytes.len() == expected_len => Ok(Some(bytes)),
        Some(_) => Err(CborHelperError(format!(
            "`{key_name}` must be {expected_len} bytes"
        ))),
        None => Ok(None),
    }
}

/// Looks up an unsigned integer field in a map.
pub fn map_lookup_u64(map: &[(Value, Value)], key_name: &str) -> Result<u64, CborHelperError> {
    let value = map_lookup_value(map, key_name)?;
    value
        .as_integer()
        .and_then(|integer| integer.try_into().ok())
        .ok_or_else(|| CborHelperError(format!("`{key_name}` is not an unsigned integer")))
}

/// Looks up a boolean field in a map.
pub fn map_lookup_bool(map: &[(Value, Value)], key_name: &str) -> Result<bool, CborHelperError> {
    map_lookup_value(map, key_name).and_then(|value| {
        value
            .as_bool()
            .ok_or_else(|| CborHelperError(format!("`{key_name}` is not a boolean")))
    })
}

/// Looks up a text string field in a map.
pub fn map_lookup_text(map: &[(Value, Value)], key_name: &str) -> Result<String, CborHelperError> {
    map_lookup_value(map, key_name).and_then(|value| {
        value
            .as_text()
            .map(ToOwned::to_owned)
            .ok_or_else(|| CborHelperError(format!("`{key_name}` is not a text string")))
    })
}

/// Looks up an optional text string field in a map.
pub fn map_lookup_optional_text(
    map: &[(Value, Value)],
    key_name: &str,
) -> Result<Option<String>, CborHelperError> {
    match map_lookup_optional_value(map, key_name) {
        Some(Value::Text(value)) => Ok(Some(value.clone())),
        Some(Value::Null) | None => Ok(None),
        Some(_) => Err(CborHelperError(format!(
            "`{key_name}` is neither text nor null"
        ))),
    }
}

/// Looks up a map field in a map.
pub fn map_lookup_map<'a>(
    map: &'a [(Value, Value)],
    key_name: &str,
) -> Result<&'a [(Value, Value)], CborHelperError> {
    map_lookup_value(map, key_name).and_then(|value| {
        value
            .as_map()
            .map(Vec::as_slice)
            .ok_or_else(|| CborHelperError(format!("`{key_name}` is not a map")))
    })
}

/// Looks up an optional map field in a map.
pub fn map_lookup_optional_map<'a>(
    map: &'a [(Value, Value)],
    key_name: &str,
) -> Result<Option<&'a [(Value, Value)]>, CborHelperError> {
    match map_lookup_optional_value(map, key_name) {
        Some(Value::Null) | None => Ok(None),
        Some(value) => value
            .as_map()
            .map(Vec::as_slice)
            .map(Some)
            .ok_or_else(|| CborHelperError(format!("`{key_name}` is not a map"))),
    }
}

/// Looks up an array field in a map.
pub fn map_lookup_array<'a>(map: &'a [(Value, Value)], key_name: &str) -> Result<&'a [Value], CborHelperError> {
    map_lookup_value(map, key_name).and_then(|value| {
        value
            .as_array()
            .map(Vec::as_slice)
            .ok_or_else(|| CborHelperError(format!("`{key_name}` is not an array")))
    })
}

/// Performs a map lookup for an integer label (as used in COSE).
pub fn map_lookup_integer_label_value<'a>(
    map: &'a [(Value, Value)],
    label: i128,
) -> Option<&'a Value> {
    map.iter()
        .find(|(key, _)| {
            key.as_integer()
                .is_some_and(|value| i128::from(value) == label)
        })
        .map(|(_, value)| value)
}

/// Looks up an integer-labeled byte string field in a map (as used in COSE).
pub fn map_lookup_integer_label_bytes(map: &[(Value, Value)], label: i128) -> Result<Vec<u8>, CborHelperError> {
    map_lookup_integer_label_value(map, label)
        .and_then(|value| value.as_bytes().cloned())
        .ok_or_else(|| CborHelperError(format!("missing COSE label {label} bytes")))
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
}
