// Rust guideline compliant 2026-02-21
//! DID interop adapter (ADR 0008).
//!
//! This crate emits the `did-key-view` sidecar: a deterministic JSON
//! labeling view over Trellis signing-class key registry entries. It does
//! not sign, resolve DIDs over the network, or participate in core event
//! verification. The view maps each signing-class `kid` to the equivalent
//! `did:key` value for the same Ed25519 public key bytes.

#![forbid(unsafe_code)]

use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};

use trellis_types::{
    SUITE_ID_PHASE_1, Value, decode_cbor_value, map_lookup_fixed_bytes, map_lookup_optional_value,
    map_lookup_u64,
};

/// `did-key-view` sidecar kind.
pub const KIND: &str = "did-key-view";

/// `did-key-view` `derivation_version`.
pub const DERIVATION_VERSION: u8 = 1;

/// DID-key-view document version.
pub const DOCUMENT_VERSION: u8 = 1;

/// Ed25519 public-key multicodec prefix.
const ED25519_PUB_MULTICODEC_PREFIX: [u8; 2] = [0xed, 0x01];

/// Base58btc alphabet.
const BASE58BTC_ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// One signing-class key row for DID projection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SigningKeyViewEntry {
    pub kid: [u8; 16],
    pub pubkey: [u8; 32],
    pub suite_id: u64,
}

/// One emitted DID-key-view mapping.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DidKeyViewEntry {
    pub kid: String,
    pub did_key: String,
}

/// DID-key-view document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DidKeyView {
    pub version: u8,
    pub derivation_version: u8,
    pub suite_id: u64,
    pub entries: Vec<DidKeyViewEntry>,
}

/// DID adapter error category.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DidKeyViewErrorKind {
    CborDecode,
    RegistryRootType,
    RegistryEntryType,
    RegistryEntryField,
    UnsupportedSuite,
    DuplicateKid,
}

/// DID adapter error.
#[derive(Debug, PartialEq, Eq)]
pub struct DidKeyViewError {
    kind: DidKeyViewErrorKind,
    message: String,
}

impl DidKeyViewError {
    fn new(kind: DidKeyViewErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    /// Returns the structured error category.
    pub fn kind(&self) -> DidKeyViewErrorKind {
        self.kind
    }
}

impl Display for DidKeyViewError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DidKeyViewError {}

/// Parses signing-class entries from registry CBOR.
///
/// Accepts both Core §8.2 legacy `SigningKeyEntry` rows and Core §8.7
/// `KeyEntrySigning` rows. Non-signing `KeyEntry` rows are skipped because
/// ADR 0008 scopes `did-key-view` to signing keys only.
///
/// # Errors
///
/// Returns [`DidKeyViewError`] when the registry is not a CBOR array, a
/// signing-class row is malformed, a signing row uses an unsupported
/// suite, or two signing rows reuse the same `kid`.
pub fn signing_keys_from_registry_cbor(
    registry_cbor: &[u8],
) -> Result<Vec<SigningKeyViewEntry>, DidKeyViewError> {
    let value = decode_cbor_value(registry_cbor).map_err(|error| {
        DidKeyViewError::new(
            DidKeyViewErrorKind::CborDecode,
            format!("failed to decode key registry CBOR: {error}"),
        )
    })?;
    let entries = match value {
        Value::Array(entries) => entries,
        _ => {
            return Err(DidKeyViewError::new(
                DidKeyViewErrorKind::RegistryRootType,
                "key registry root is not a CBOR array",
            ));
        }
    };

    let mut signing_entries = Vec::new();
    let mut seen_kids = BTreeSet::new();

    for (index, entry) in entries.iter().enumerate() {
        let entry_map = entry.as_map().ok_or_else(|| {
            DidKeyViewError::new(
                DidKeyViewErrorKind::RegistryEntryType,
                format!("key registry entry {index} is not a map"),
            )
        })?;
        if !is_signing_entry(entry_map, index)? {
            continue;
        }

        let kid = fixed_array::<16>(lookup_fixed(entry_map, "kid", 16, index)?, "kid", index)?;
        let pubkey = fixed_array::<32>(
            lookup_fixed(entry_map, "pubkey", 32, index)?,
            "pubkey",
            index,
        )?;
        let suite_id = map_lookup_u64(entry_map, "suite_id").map_err(|error| {
            DidKeyViewError::new(
                DidKeyViewErrorKind::RegistryEntryField,
                format!("key registry entry {index} suite_id invalid: {error}"),
            )
        })?;
        if suite_id != SUITE_ID_PHASE_1 {
            return Err(DidKeyViewError::new(
                DidKeyViewErrorKind::UnsupportedSuite,
                format!(
                    "did-key-view derivation_version 1 supports Ed25519 suite_id {SUITE_ID_PHASE_1}, got {suite_id}"
                ),
            ));
        }
        if !seen_kids.insert(kid) {
            return Err(DidKeyViewError::new(
                DidKeyViewErrorKind::DuplicateKid,
                format!("key registry repeats signing kid {}", hex_lower(&kid)),
            ));
        }

        signing_entries.push(SigningKeyViewEntry {
            kid,
            pubkey,
            suite_id,
        });
    }

    signing_entries.sort_by(|left, right| left.kid.cmp(&right.kid));
    Ok(signing_entries)
}

/// Emits a deterministic DID-key-view JSON document.
///
/// The output is compact UTF-8 JSON with entries sorted by `kid`. Identical
/// inputs produce byte-identical output (ADR 0008 ISC-02).
///
/// # Errors
///
/// Returns [`DidKeyViewErrorKind::UnsupportedSuite`] if any entry is not the
/// Phase-1 Ed25519 suite, or [`DidKeyViewErrorKind::DuplicateKid`] if the
/// input repeats a signing `kid`.
pub fn emit_did_key_view_json(entries: &[SigningKeyViewEntry]) -> Result<Vec<u8>, DidKeyViewError> {
    let view = did_key_view(entries)?;
    Ok(render_json(&view).into_bytes())
}

/// Emits a DID-key-view JSON document from registry CBOR.
///
/// # Errors
///
/// Returns [`DidKeyViewError`] when registry parsing or JSON view emission
/// fails.
pub fn emit_did_key_view_json_from_registry_cbor(
    registry_cbor: &[u8],
) -> Result<Vec<u8>, DidKeyViewError> {
    let entries = signing_keys_from_registry_cbor(registry_cbor)?;
    emit_did_key_view_json(&entries)
}

/// Builds a typed DID-key-view document.
///
/// # Errors
///
/// Returns [`DidKeyViewErrorKind::UnsupportedSuite`] if any entry is not the
/// Phase-1 Ed25519 suite, or [`DidKeyViewErrorKind::DuplicateKid`] if the
/// input repeats a signing `kid`.
pub fn did_key_view(entries: &[SigningKeyViewEntry]) -> Result<DidKeyView, DidKeyViewError> {
    let mut sorted = entries.to_vec();
    sorted.sort_by(|left, right| left.kid.cmp(&right.kid));

    let mut seen_kids = BTreeSet::new();
    let mut view_entries = Vec::with_capacity(sorted.len());
    for entry in sorted {
        if entry.suite_id != SUITE_ID_PHASE_1 {
            return Err(DidKeyViewError::new(
                DidKeyViewErrorKind::UnsupportedSuite,
                format!(
                    "did-key-view derivation_version 1 supports Ed25519 suite_id {SUITE_ID_PHASE_1}, got {}",
                    entry.suite_id
                ),
            ));
        }
        if !seen_kids.insert(entry.kid) {
            return Err(DidKeyViewError::new(
                DidKeyViewErrorKind::DuplicateKid,
                format!("input repeats signing kid {}", hex_lower(&entry.kid)),
            ));
        }
        view_entries.push(DidKeyViewEntry {
            kid: hex_lower(&entry.kid),
            did_key: did_key_for_ed25519_public_key(&entry.pubkey),
        });
    }

    Ok(DidKeyView {
        version: DOCUMENT_VERSION,
        derivation_version: DERIVATION_VERSION,
        suite_id: SUITE_ID_PHASE_1,
        entries: view_entries,
    })
}

/// Renders an Ed25519 public key as `did:key`.
#[must_use]
pub fn did_key_for_ed25519_public_key(public_key: &[u8; 32]) -> String {
    let mut multicodec = Vec::with_capacity(ED25519_PUB_MULTICODEC_PREFIX.len() + public_key.len());
    multicodec.extend_from_slice(&ED25519_PUB_MULTICODEC_PREFIX);
    multicodec.extend_from_slice(public_key);
    format!("did:key:{}", base58btc_encode(&multicodec))
}

fn is_signing_entry(map: &[(Value, Value)], index: usize) -> Result<bool, DidKeyViewError> {
    match map_lookup_optional_value(map, "kind") {
        None => Ok(true),
        Some(Value::Text(kind)) if kind == "signing" => Ok(true),
        Some(Value::Text(_)) => Ok(false),
        Some(_) => Err(DidKeyViewError::new(
            DidKeyViewErrorKind::RegistryEntryField,
            format!("key registry entry {index} kind is not a text string"),
        )),
    }
}

fn lookup_fixed(
    map: &[(Value, Value)],
    key: &'static str,
    expected_len: usize,
    index: usize,
) -> Result<Vec<u8>, DidKeyViewError> {
    map_lookup_fixed_bytes(map, key, expected_len).map_err(|error| {
        DidKeyViewError::new(
            DidKeyViewErrorKind::RegistryEntryField,
            format!("key registry entry {index} {key} invalid: {error}"),
        )
    })
}

fn fixed_array<const N: usize>(
    bytes: Vec<u8>,
    field: &'static str,
    index: usize,
) -> Result<[u8; N], DidKeyViewError> {
    if bytes.len() != N {
        return Err(DidKeyViewError::new(
            DidKeyViewErrorKind::RegistryEntryField,
            format!("key registry entry {index} {field} must be {N} bytes"),
        ));
    }
    let mut array = [0u8; N];
    array.copy_from_slice(&bytes);
    Ok(array)
}

fn render_json(view: &DidKeyView) -> String {
    let entries = view
        .entries
        .iter()
        .map(|entry| {
            format!(
                "{{\"kid\":\"{}\",\"did:key\":\"{}\"}}",
                entry.kid, entry.did_key
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!(
        "{{\"version\":{},\"derivation_version\":{},\"suite_id\":{},\"entries\":[{}]}}",
        view.version, view.derivation_version, view.suite_id, entries
    )
}

fn base58btc_encode(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "z".to_string();
    }

    let leading_zeroes = bytes.iter().take_while(|byte| **byte == 0).count();
    let mut digits = Vec::<u8>::new();

    for byte in bytes {
        let mut carry = u32::from(*byte);
        for digit in &mut digits {
            let value = u32::from(*digit) * 256 + carry;
            *digit = (value % 58) as u8;
            carry = value / 58;
        }
        while carry > 0 {
            digits.push((carry % 58) as u8);
            carry /= 58;
        }
    }

    let mut encoded = String::with_capacity(1 + leading_zeroes + digits.len());
    encoded.push('z');
    for _ in 0..leading_zeroes {
        encoded.push('1');
    }
    for digit in digits.iter().rev() {
        encoded.push(BASE58BTC_ALPHABET[*digit as usize] as char);
    }
    encoded
}

fn hex_lower(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use trellis_types::{encode_bstr, encode_tstr, encode_uint};

    fn entry(kid_byte: u8, pubkey_byte: u8) -> SigningKeyViewEntry {
        SigningKeyViewEntry {
            kid: [kid_byte; 16],
            pubkey: [pubkey_byte; 32],
            suite_id: SUITE_ID_PHASE_1,
        }
    }

    #[test]
    fn base58btc_matches_known_vectors() {
        assert_eq!(base58btc_encode(b"hello world"), "zStV1DL6CwTryKyV");
        assert_eq!(base58btc_encode(&[0]), "z1");
        assert_eq!(base58btc_encode(&[0, 0]), "z11");
        assert_eq!(base58btc_encode(&[0x61]), "z2g");
    }

    #[test]
    fn did_key_for_ed25519_public_key_matches_multicodec_vector() {
        assert_eq!(
            did_key_for_ed25519_public_key(&[0u8; 32]),
            "did:key:z6MkeTG3bFFSLYVU7VqhgZxqr6YzpaGrQtFMh1uvqGy1vDnP"
        );
    }

    #[test]
    fn emit_did_key_view_sorts_entries_and_is_byte_stable() {
        let bytes = emit_did_key_view_json(&[entry(0x22, 0x02), entry(0x11, 0x01)]).unwrap();
        let text = String::from_utf8(bytes.clone()).unwrap();
        assert!(text.contains("\"version\":1"));
        assert!(text.contains("\"derivation_version\":1"));
        assert!(text.contains("\"suite_id\":1"));
        let first = text
            .find("\"kid\":\"11111111111111111111111111111111\"")
            .expect("first kid");
        let second = text
            .find("\"kid\":\"22222222222222222222222222222222\"")
            .expect("second kid");
        assert!(first < second, "entries must sort by kid");
        let bytes_again = emit_did_key_view_json(&[entry(0x22, 0x02), entry(0x11, 0x01)]).unwrap();
        assert_eq!(bytes, bytes_again);
    }

    #[test]
    fn registry_parser_accepts_legacy_and_keyentry_signing_rows() {
        let legacy = signing_entry(false, 0x11, 0x01);
        let keyentry = signing_entry(true, 0x22, 0x02);
        let subject = non_signing_entry("subject", 0x33);
        let registry = cbor_array(&[legacy, keyentry, subject]);

        let entries = signing_keys_from_registry_cbor(&registry).unwrap();
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].kid, [0x11; 16]);
        assert_eq!(entries[1].kid, [0x22; 16]);

        let json = emit_did_key_view_json_from_registry_cbor(&registry).unwrap();
        let text = String::from_utf8(json).unwrap();
        assert!(text.contains("11111111111111111111111111111111"));
        assert!(text.contains("22222222222222222222222222222222"));
        assert!(!text.contains("33333333333333333333333333333333"));
    }

    #[test]
    fn registry_parser_rejects_duplicate_signing_kids() {
        let first = signing_entry(false, 0x11, 0x01);
        let second = signing_entry(true, 0x11, 0x02);
        let registry = cbor_array(&[first, second]);
        let err = signing_keys_from_registry_cbor(&registry).unwrap_err();
        assert_eq!(err.kind(), DidKeyViewErrorKind::DuplicateKid);
    }

    #[test]
    fn emit_rejects_non_ed25519_suite() {
        let mut input = entry(0x11, 0x01);
        input.suite_id = 99;
        let err = emit_did_key_view_json(&[input]).unwrap_err();
        assert_eq!(err.kind(), DidKeyViewErrorKind::UnsupportedSuite);
    }

    fn signing_entry(kind_field: bool, kid_byte: u8, pubkey_byte: u8) -> Vec<u8> {
        let mut fields = Vec::new();
        if kind_field {
            fields.push(cbor_pair("kind", encode_tstr("signing")));
        }
        fields.push(cbor_pair("kid", encode_bstr(&[kid_byte; 16])));
        fields.push(cbor_pair("pubkey", encode_bstr(&[pubkey_byte; 32])));
        fields.push(cbor_pair("suite_id", encode_uint(SUITE_ID_PHASE_1)));
        cbor_map(&fields)
    }

    fn non_signing_entry(kind: &str, kid_byte: u8) -> Vec<u8> {
        cbor_map(&[
            cbor_pair("kind", encode_tstr(kind)),
            cbor_pair("kid", encode_bstr(&[kid_byte; 16])),
        ])
    }

    fn cbor_pair(key: &str, value: Vec<u8>) -> (String, Vec<u8>) {
        (key.to_string(), value)
    }

    fn cbor_map(fields: &[(String, Vec<u8>)]) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(0xa0 | fields.len() as u8);
        for (key, value) in fields {
            bytes.extend_from_slice(&encode_tstr(key));
            bytes.extend_from_slice(value);
        }
        bytes
    }

    fn cbor_array(items: &[Vec<u8>]) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.push(0x80 | items.len() as u8);
        for item in items {
            bytes.extend_from_slice(item);
        }
        bytes
    }
}
