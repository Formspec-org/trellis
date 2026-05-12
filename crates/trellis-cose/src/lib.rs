// Rust guideline compliant 2026-02-21
//! COSE helpers for the Phase-1 append scaffold.

#![forbid(unsafe_code)]

use std::{
    collections::HashSet,
    fmt::{Display, Formatter},
};

use ed25519_dalek::ed25519::signature::Verifier;
use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use sha2::{Digest, Sha256};
use trellis_types::{
    COSE_LABEL_SUITE_ID, SUITE_ID_PHASE_1, Value, decode_cbor_value, encode_bstr,
    encode_cbor_negative_int, encode_cose_profile_id_label, encode_cose_suite_id_label,
    encode_tstr, encode_uint,
};

pub use trellis_types::COSE_LABEL_PROFILE_ID;

/// COSE algorithm protected-header label.
pub const COSE_LABEL_ALG: i128 = 1;
/// COSE key identifier protected-header label.
pub const COSE_LABEL_KID: i128 = 4;
/// COSE_Sign1 CBOR tag.
pub const COSE_SIGN1_TAG: u64 = 18;

/// Canonical one-byte CBOR header for a three-pair map.
const CBOR_MAP_3: u8 = 0xa3;
/// Canonical one-byte CBOR header for a four-pair map.
const CBOR_MAP_4: u8 = 0xa4;
/// Canonical one-byte CBOR header for a four-element array.
const CBOR_ARRAY_4: u8 = 0x84;
/// Canonical one-byte CBOR tag header for COSE_Sign1 tag 18.
const CBOR_TAG_18_COSE_SIGN1: u8 = 0xd2;
/// Canonical one-byte CBOR header for an empty byte string.
const CBOR_EMPTY_BSTR: u8 = 0x40;
/// Canonical one-byte CBOR header for an empty map.
const CBOR_EMPTY_MAP: u8 = 0xa0;
/// Canonical one-byte CBOR simple value for null.
const CBOR_NULL: u8 = 0xf6;

/// Decoded COSE_Sign1 envelope.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoseSign1 {
    protected_header: Vec<u8>,
    alg: Option<i128>,
    kid: Option<Vec<u8>>,
    suite_id: Option<u64>,
    profile_id: Option<u64>,
    payload: Option<Vec<u8>>,
    signature: Vec<u8>,
}

impl CoseSign1 {
    /// Returns the protected-header byte string.
    #[must_use]
    pub fn protected_header(&self) -> &[u8] {
        &self.protected_header
    }

    /// Returns the COSE algorithm value, if present.
    #[must_use]
    pub fn alg(&self) -> Option<i128> {
        self.alg
    }

    /// Returns the key identifier, if present.
    #[must_use]
    pub fn kid(&self) -> Option<&[u8]> {
        self.kid.as_deref()
    }

    /// Returns the Trellis signature suite identifier, if present.
    #[must_use]
    pub fn suite_id(&self) -> Option<u64> {
        self.suite_id
    }

    /// Returns the plugin-dispatch identifier, if present.
    #[must_use]
    pub fn profile_id(&self) -> Option<u64> {
        self.profile_id
    }

    /// Returns the embedded payload, if the envelope is not detached.
    #[must_use]
    pub fn payload(&self) -> Option<&[u8]> {
        self.payload.as_deref()
    }

    /// Returns the primitive signature bytes.
    #[must_use]
    pub fn signature(&self) -> &[u8] {
        &self.signature
    }

    /// Resolves the payload used by RFC 9052 `Sig_structure` construction.
    ///
    /// Detached envelopes require `detached_payload`. Embedded envelopes can be
    /// used directly; if the caller also supplies bytes, they must match the
    /// embedded bytes exactly.
    ///
    /// # Errors
    /// Returns an error when a detached envelope has no supplied payload, or
    /// when an embedded payload differs from the caller-supplied bytes.
    pub fn resolve_payload<'a>(
        &'a self,
        detached_payload: Option<&'a [u8]>,
    ) -> Result<&'a [u8], CoseError> {
        match (self.payload(), detached_payload) {
            (Some(payload), Some(supplied)) if payload == supplied => Ok(payload),
            (Some(_), Some(_)) => Err(CoseError::new(
                "embedded COSE payload does not match supplied signed bytes",
            )),
            (Some(payload), None) => Ok(payload),
            (None, Some(supplied)) => Ok(supplied),
            (None, None) => Err(CoseError::new("detached COSE payload was not supplied")),
        }
    }
}

/// COSE decode or verification error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoseError {
    message: String,
}

impl CoseError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for CoseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for CoseError {}

/// Decodes one tagged COSE_Sign1 envelope.
///
/// # Errors
/// Returns an error when bytes are not CBOR tag 18 with the COSE_Sign1
/// four-field body.
pub fn decode_cose_sign1(bytes: &[u8]) -> Result<CoseSign1, CoseError> {
    let value = decode_cbor_value(bytes).map_err(|error| CoseError::new(error.to_string()))?;
    decode_cose_sign1_value(&value)
}

fn decode_cose_sign1_value(value: &Value) -> Result<CoseSign1, CoseError> {
    let body = match value {
        Value::Tag(COSE_SIGN1_TAG, inner) => inner,
        Value::Tag(tag, _) => {
            return Err(CoseError::new(format!(
                "unexpected COSE tag {tag}; expected {COSE_SIGN1_TAG}"
            )));
        }
        _ => return Err(CoseError::new("value is not tagged COSE_Sign1")),
    };
    let items = body
        .as_array()
        .ok_or_else(|| CoseError::new("COSE_Sign1 body is not an array"))?;
    if items.len() != 4 {
        return Err(CoseError::new("COSE_Sign1 body must have four fields"));
    }

    let protected_header = items[0]
        .as_bytes()
        .cloned()
        .ok_or_else(|| CoseError::new("protected header is not a byte string"))?;
    let protected_value = decode_cbor_value(&protected_header)
        .map_err(|error| CoseError::new(format!("failed to decode protected header: {error}")))?;
    let protected_map = protected_value
        .as_map()
        .ok_or_else(|| CoseError::new("protected header does not decode to a map"))?;
    reject_duplicate_integer_labels(protected_map)?;

    match &items[1] {
        Value::Map(entries) if entries.is_empty() => {}
        Value::Map(_) => return Err(CoseError::new("unprotected header map must be empty")),
        _ => return Err(CoseError::new("unprotected header is not a map")),
    }

    let payload = match &items[2] {
        Value::Bytes(bytes) => Some(bytes.clone()),
        Value::Null => None,
        _ => return Err(CoseError::new("payload is neither bytes nor null")),
    };
    let signature = items[3]
        .as_bytes()
        .cloned()
        .ok_or_else(|| CoseError::new("signature is not a byte string"))?;

    Ok(CoseSign1 {
        protected_header,
        alg: integer_label_i128(protected_map, COSE_LABEL_ALG)?,
        kid: integer_label_bytes(protected_map, COSE_LABEL_KID)?,
        suite_id: integer_label_u64(protected_map, COSE_LABEL_SUITE_ID)?,
        profile_id: integer_label_u64(protected_map, COSE_LABEL_PROFILE_ID)?,
        payload,
        signature,
    })
}

fn reject_duplicate_integer_labels(map: &[(Value, Value)]) -> Result<(), CoseError> {
    let mut seen = HashSet::new();
    for (key, _) in map {
        let Some(integer) = key.as_integer().map(i128::from) else {
            continue;
        };
        if !seen.insert(integer) {
            return Err(CoseError::new(format!(
                "duplicate protected-header label {integer}"
            )));
        }
    }
    Ok(())
}

fn integer_label_i128(map: &[(Value, Value)], label: i128) -> Result<Option<i128>, CoseError> {
    integer_label_value(map, label)
        .map(|value| {
            value
                .as_integer()
                .map(i128::from)
                .ok_or_else(|| CoseError::new(format!("COSE label {label} is not an integer")))
        })
        .transpose()
}

fn integer_label_u64(map: &[(Value, Value)], label: i128) -> Result<Option<u64>, CoseError> {
    integer_label_i128(map, label)?
        .map(|integer| {
            u64::try_from(integer).map_err(|_| {
                CoseError::new(format!("COSE label {label} is not an unsigned integer"))
            })
        })
        .transpose()
}

fn integer_label_bytes(map: &[(Value, Value)], label: i128) -> Result<Option<Vec<u8>>, CoseError> {
    integer_label_value(map, label)
        .map(|value| {
            value
                .as_bytes()
                .cloned()
                .ok_or_else(|| CoseError::new(format!("COSE label {label} is not bytes")))
        })
        .transpose()
}

fn integer_label_value(map: &[(Value, Value)], label: i128) -> Option<&Value> {
    map.iter()
        .find(|(key, _)| {
            key.as_integer()
                .is_some_and(|integer| i128::from(integer) == label)
        })
        .map(|(_, value)| value)
}

/// Derives the 16-byte `kid` from `suite_id` and the Ed25519 public key.
///
/// The preimage uses canonical CBOR unsigned encoding for `suite_id`, matching
/// Python `dcbor(suite_id)` in `fixtures/vectors/_generator/gen_v3_remaining.py`
/// (and therefore differs from a raw single byte when `suite_id >= 24`).
pub fn derive_kid(suite_id: u8, public_key: [u8; 32]) -> [u8; 16] {
    let mut hasher = Sha256::new();
    hasher.update(encode_uint(suite_id as u64));
    hasher.update(public_key);
    let digest: [u8; 32] = hasher.finalize().into();
    let mut kid = [0u8; 16];
    kid.copy_from_slice(&digest[..16]);
    kid
}

/// Builds the protected-header map bytes.
pub fn protected_header_bytes(kid: [u8; 16]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(32);
    bytes.push(CBOR_MAP_3);
    bytes.extend_from_slice(&encode_uint(1));
    bytes.extend_from_slice(&encode_cbor_negative_int(7));
    bytes.extend_from_slice(&encode_uint(4));
    bytes.extend_from_slice(&encode_bstr(&kid));
    bytes.extend_from_slice(&encode_cose_suite_id_label());
    bytes.extend_from_slice(&encode_uint(SUITE_ID_PHASE_1));
    bytes
}

/// Builds protected-header map bytes with a Trellis `profile_id`.
///
/// Existing Phase-1 fixtures use [`protected_header_bytes`]. This helper pins
/// the profile-aware wire shape for consumers that dispatch semantic plugins
/// from the COSE protected header.
pub fn protected_header_bytes_with_profile_id(kid: [u8; 16], profile_id: u64) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(40);
    bytes.push(CBOR_MAP_4);
    bytes.extend_from_slice(&encode_uint(1));
    bytes.extend_from_slice(&encode_cbor_negative_int(7));
    bytes.extend_from_slice(&encode_uint(4));
    bytes.extend_from_slice(&encode_bstr(&kid));
    bytes.extend_from_slice(&encode_cose_suite_id_label());
    bytes.extend_from_slice(&encode_uint(SUITE_ID_PHASE_1));
    bytes.extend_from_slice(&encode_cose_profile_id_label());
    bytes.extend_from_slice(&encode_uint(profile_id));
    bytes
}

/// Builds the RFC 9052 `Sig_structure`.
pub fn sig_structure_bytes(protected_header: &[u8], payload: &[u8]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push(CBOR_ARRAY_4);
    bytes.extend_from_slice(&encode_tstr("Signature1"));
    bytes.extend_from_slice(&encode_bstr(protected_header));
    bytes.push(CBOR_EMPTY_BSTR);
    bytes.extend_from_slice(&encode_bstr(payload));
    bytes
}

/// Verifies the `Sig_structure` with an Ed25519 public key.
pub fn verify_ed25519_signature(
    public_key: [u8; 32],
    sig_structure: &[u8],
    signature: [u8; 64],
) -> bool {
    let Ok(verifying_key) = VerifyingKey::from_bytes(&public_key) else {
        return false;
    };
    let signature = Signature::from_bytes(&signature);
    verifying_key.verify(sig_structure, &signature).is_ok()
}

/// Verifies one COSE_Sign1 envelope with an Ed25519 public key.
///
/// `detached_payload` is required when the COSE payload field is `null`. When
/// the envelope embeds a payload and the caller also supplies bytes, they must
/// match before signature verification runs.
///
/// # Errors
/// Returns an error when the envelope cannot be decoded, a detached payload is
/// missing, an embedded payload mismatches the supplied bytes, or the signature
/// is not 64 bytes.
pub fn verify_ed25519_sign1(
    public_key: [u8; 32],
    sign1_bytes: &[u8],
    detached_payload: Option<&[u8]>,
) -> Result<bool, CoseError> {
    let sign1 = decode_cose_sign1(sign1_bytes)?;
    let payload = sign1.resolve_payload(detached_payload)?;
    let signature: [u8; 64] = sign1
        .signature()
        .try_into()
        .map_err(|_| CoseError::new("signature is not 64 bytes"))?;
    let sig_structure = sig_structure_bytes(sign1.protected_header(), payload);
    Ok(verify_ed25519_signature(
        public_key,
        &sig_structure,
        signature,
    ))
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
    bytes.push(CBOR_TAG_18_COSE_SIGN1);
    bytes.push(CBOR_ARRAY_4);
    bytes.extend_from_slice(&encode_bstr(protected_header));
    bytes.push(CBOR_EMPTY_MAP);
    bytes.extend_from_slice(&encode_bstr(payload));
    bytes.extend_from_slice(&encode_bstr(&signature));
    bytes
}

/// Builds a tagged COSE_Sign1 envelope whose payload field is `null`.
pub fn sign1_detached_bytes(protected_header: &[u8], signature: [u8; 64]) -> Vec<u8> {
    let mut bytes = Vec::new();
    bytes.push(CBOR_TAG_18_COSE_SIGN1);
    bytes.push(CBOR_ARRAY_4);
    bytes.extend_from_slice(&encode_bstr(protected_header));
    bytes.push(CBOR_EMPTY_MAP);
    bytes.push(CBOR_NULL);
    bytes.extend_from_slice(&encode_bstr(&signature));
    bytes
}

#[cfg(test)]
mod tests {
    use ed25519_dalek::SigningKey;

    use super::{
        decode_cose_sign1, protected_header_bytes, protected_header_bytes_with_profile_id,
        sig_structure_bytes, sign_ed25519, sign1_bytes, sign1_detached_bytes, verify_ed25519_sign1,
    };

    #[test]
    fn profile_id_protected_header_matches_allocated_wire_bytes() {
        let protected = protected_header_bytes_with_profile_id([0x11; 16], 1);

        assert_eq!(
            protected,
            vec![
                0xa4, 0x01, 0x27, 0x04, 0x50, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
                0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x3a, 0x00, 0x01, 0x00, 0x00, 0x01, 0x3a,
                0x00, 0x01, 0x00, 0x02, 0x01,
            ]
        );
    }

    #[test]
    fn profile_id_sig_structure_golden_vector() {
        let protected = protected_header_bytes_with_profile_id([0x11; 16], 1);
        let sig_structure = sig_structure_bytes(&protected, b"payload");

        assert_eq!(
            sig_structure,
            vec![
                0x84, 0x6a, 0x53, 0x69, 0x67, 0x6e, 0x61, 0x74, 0x75, 0x72, 0x65, 0x31, 0x58, 0x21,
                0xa4, 0x01, 0x27, 0x04, 0x50, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
                0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x3a, 0x00, 0x01, 0x00, 0x00, 0x01, 0x3a,
                0x00, 0x01, 0x00, 0x02, 0x01, 0x40, 0x47, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64,
            ]
        );
    }

    #[test]
    fn detached_sign1_verifies_against_supplied_payload() {
        let seed = [0x42; 32];
        let public_key = SigningKey::from_bytes(&seed).verifying_key().to_bytes();
        let protected = protected_header_bytes_with_profile_id([0x22; 16], 1);
        let payload = b"detached-payload";
        let signature = sign_ed25519(seed, &sig_structure_bytes(&protected, payload));
        let sign1 = sign1_detached_bytes(&protected, signature);
        let decoded = decode_cose_sign1(&sign1).expect("decode detached sign1");

        assert_eq!(decoded.payload(), None);
        assert_eq!(decoded.suite_id(), Some(1));
        assert_eq!(decoded.profile_id(), Some(1));
        assert_eq!(
            decoded
                .resolve_payload(Some(payload))
                .expect("resolve detached payload"),
            payload
        );
        assert!(
            verify_ed25519_sign1(public_key, &sign1, Some(payload)).expect("verify detached sign1")
        );
    }

    #[test]
    fn detached_sign1_rejects_mismatched_supplied_payload() {
        let seed = [0x24; 32];
        let public_key = SigningKey::from_bytes(&seed).verifying_key().to_bytes();
        let protected = protected_header_bytes([0x33; 16]);
        let signature = sign_ed25519(seed, &sig_structure_bytes(&protected, b"payload-a"));
        let sign1 = sign1_detached_bytes(&protected, signature);

        assert!(
            !verify_ed25519_sign1(public_key, &sign1, Some(b"payload-b"))
                .expect("verify detached sign1")
        );
    }

    #[test]
    fn embedded_sign1_rejects_supplied_payload_mismatch() {
        let protected = protected_header_bytes([0x44; 16]);
        let sign1 = sign1_bytes(&protected, b"inside", [0x55; 64]);
        let decoded = decode_cose_sign1(&sign1).expect("decode embedded sign1");
        let error = decoded.resolve_payload(Some(b"outside")).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("embedded COSE payload does not match")
        );
    }

    #[test]
    fn detached_sign1_requires_supplied_payload() {
        let protected = protected_header_bytes([0x66; 16]);
        let sign1 = sign1_detached_bytes(&protected, [0x77; 64]);
        let error = verify_ed25519_sign1([0x88; 32], &sign1, None).unwrap_err();

        assert!(error.to_string().contains("detached COSE payload"));
    }
}
