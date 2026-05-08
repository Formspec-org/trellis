//! Trellis-COSE Formspec signature adapter.
//!
//! Implements `formspec_signature_port::Verifier` using `trellis-cose` primitives.
//! This is a consumer-owned adapter (NOT a Trellis center crate) per ADR-0086 D-7.
//! Same registry coverage as webcrypto/ring adapters; PQC suites composable as Trellis adds them.
//! Receipt signing uses Trellis-managed signing keys per ADR-0006 key-class taxonomy.

use formspec_signature_port::{
    AdapterInfo, KeyInfo, SignatureMethodRegistry, VerificationReceipt, VerificationResult,
    Verifier, VerifierError, VerifyRequest,
};

const ADAPTER_ID: &str = "urn:formspec:adapter:trellis-cose@1";
const ADAPTER_VERSION: &str = "0.1.0";

pub struct TrellisCoseVerifier {
    adapter_info: AdapterInfo,
}

impl TrellisCoseVerifier {
    pub fn new() -> Self {
        Self {
            adapter_info: AdapterInfo {
                id: ADAPTER_ID.into(),
                version: ADAPTER_VERSION.into(),
            },
        }
    }

    fn unsupported_receipt(
        &self,
        request: &VerifyRequest,
        registry: &SignatureMethodRegistry,
    ) -> VerificationReceipt {
        VerificationReceipt {
            result: VerificationResult::Unsupported,
            method: request.signature_method.clone(),
            method_registry_version: registry.version.clone(),
            adapter: self.adapter_info.clone(),
            key: KeyInfo {
                r#ref: request.key_ref.clone(),
                version: None,
                snapshot: None,
            },
            verified_at: chrono_now(),
            context: None,
            receipt_bytes: None,
        }
    }
}

impl Default for TrellisCoseVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl Verifier for TrellisCoseVerifier {
    fn verify(
        &self,
        request: &VerifyRequest,
        registry: &SignatureMethodRegistry,
    ) -> Result<VerificationReceipt, VerifierError> {
        let entry = registry.resolve(&request.signature_method);
        let entry = match entry {
            Some(e) => e,
            None => {
                return Ok(self.unsupported_receipt(request, registry));
            }
        };

        if entry.status == "deprecated" {
            return Ok(self.unsupported_receipt(request, registry));
        }

        match entry.alg {
            None => Ok(self.unsupported_receipt(request, registry)),
            Some(_alg) => {
                // TODO(Phase 4.1): integrate trellis-cose COSE_Sign1::verify() with key resolution
                // from trellis-core's key registry.
                //
                // Planned implementation:
                // let cose = trellis_cose::CoseSign1::from_bytes(&request.signature_bytes)?;
                // let key = trellis_core::key_registry::resolve(&request.key_ref)?;
                // let valid = trellis_cose::verify_detached(&cose, &request.signed_bytes, &key)?;
                //
                // Until COSE integration lands, this adapter MUST NOT be used in production
                // builds. The `allow_placeholder_verify` feature gate makes accidental
                // use explicit.

                #[cfg(not(feature = "allow_placeholder_verify"))]
                {
                    let _ = _alg;
                    return Err(VerifierError::Internal {
                        reason: "trellis-cose COSE_Sign1 verification not yet integrated; \
                                 enable 'allow_placeholder_verify' feature for testing only"
                            .to_string(),
                    });
                }

                #[cfg(feature = "allow_placeholder_verify")]
                {
                    Ok(VerificationReceipt {
                        result: VerificationResult::Verified,
                        method: request.signature_method.clone(),
                        method_registry_version: registry.version.clone(),
                        adapter: self.adapter_info.clone(),
                        key: KeyInfo {
                            r#ref: request.key_ref.clone(),
                            version: None,
                            snapshot: None,
                        },
                        verified_at: chrono_now(),
                        context: None,
                        receipt_bytes: None,
                    })
                }
            }
        }
    }
}

/// RFC 3339 UTC timestamp from system clock using Hinnant civil_from_days algorithm.
fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = dur.as_secs() as i64;

    let days = secs / 86400;
    let time_secs = secs % 86400;
    let hours = time_secs / 3600;
    let minutes = (time_secs % 3600) / 60;
    let seconds = time_secs % 60;

    let z = days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let mut y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    if m <= 2 {
        y += 1;
    }

    format!("{y:04}-{m:02}-{d:02}T{hours:02}:{minutes:02}:{seconds:02}Z")
}

#[cfg(test)]
mod tests {
    use super::*;
    use formspec_signature_port::RegistryEntry;

    fn test_registry() -> SignatureMethodRegistry {
        SignatureMethodRegistry {
            version: "1.0.0".into(),
            entries: vec![
                RegistryEntry {
                    id: "urn:formspec:sig-method:ed25519-cose-sign1@1".into(),
                    suite: "Ed25519".to_string(),
                    wire: "COSE_Sign1 with alg = -8 (EdDSA)".to_string(),
                    alg: Some(-8),
                    status: "registered".to_string(),
                    deprecation_notice: None,
                },
                RegistryEntry {
                    id: "urn:formspec:sig-method:ml-dsa-65-cose-sign1@1".into(),
                    suite: "ML-DSA-65 (FIPS 204)".to_string(),
                    wire: "COSE_Sign1 with alg = TBD".to_string(),
                    alg: None,
                    status: "registered".to_string(),
                    deprecation_notice: None,
                },
            ],
        }
    }

    #[test]
    fn test_unsupported_for_unknown_method() {
        let verifier = TrellisCoseVerifier::new();
        let registry = test_registry();
        let receipt = verifier
            .verify(
                &VerifyRequest {
                    signed_bytes: vec![1, 2, 3],
                    signature_bytes: vec![4, 5, 6],
                    signature_method: "urn:formspec:sig-method:unknown@1".into(),
                    key_ref: "deadbeef".into(),
                },
                &registry,
            )
            .unwrap();
        assert_eq!(receipt.result.to_string(), "unsupported");
    }

    #[test]
    fn test_adapter_info() {
        let verifier = TrellisCoseVerifier::new();
        assert_eq!(
            verifier.adapter_info.id,
            "urn:formspec:adapter:trellis-cose@1"
        );
    }

    #[test]
    fn test_unsupported_for_null_alg() {
        let verifier = TrellisCoseVerifier::new();
        let registry = test_registry();
        let receipt = verifier
            .verify(
                &VerifyRequest {
                    signed_bytes: vec![1, 2, 3],
                    signature_bytes: vec![4, 5, 6],
                    signature_method: "urn:formspec:sig-method:ml-dsa-65-cose-sign1@1".into(),
                    key_ref: "deadbeef".into(),
                },
                &registry,
            )
            .unwrap();
        assert_eq!(receipt.result.to_string(), "unsupported");
    }

    #[test]
    #[cfg(not(feature = "allow_placeholder_verify"))]
    fn test_known_method_returns_error_without_feature_gate() {
        let verifier = TrellisCoseVerifier::new();
        let registry = test_registry();
        let result = verifier.verify(
            &VerifyRequest {
                signed_bytes: vec![1, 2, 3],
                signature_bytes: vec![4, 5, 6],
                signature_method: "urn:formspec:sig-method:ed25519-cose-sign1@1".into(),
                key_ref: "deadbeef".into(),
            },
            &registry,
        );
        assert!(result.is_err());
        let err = result.unwrap_err();
        match err {
            VerifierError::Internal { reason } => {
                assert!(
                    reason.contains("not yet integrated"),
                    "expected COSE-not-integrated message, got: {reason}"
                );
            }
            other => panic!("expected Internal error, got: {other}"),
        }
    }

    #[test]
    #[cfg(feature = "allow_placeholder_verify")]
    fn test_known_method_returns_verified_with_feature_gate() {
        let verifier = TrellisCoseVerifier::new();
        let registry = test_registry();
        let receipt = verifier
            .verify(
                &VerifyRequest {
                    signed_bytes: vec![1, 2, 3],
                    signature_bytes: vec![4, 5, 6],
                    signature_method: "urn:formspec:sig-method:ed25519-cose-sign1@1".into(),
                    key_ref: "deadbeef".into(),
                },
                &registry,
            )
            .expect("feature-gated placeholder returns a receipt");
        assert_eq!(receipt.result.to_string(), "verified");
    }

    #[test]
    fn test_chrono_now_produces_valid_rfc3339() {
        let ts = chrono_now();
        assert!(ts.ends_with('Z'), "must end with Z: {ts}");
        let parts: Vec<&str> = ts.split('T').collect();
        assert_eq!(parts.len(), 2, "must have date T time: {ts}");
        let date_parts: Vec<&str> = parts[0].split('-').collect();
        assert_eq!(date_parts.len(), 3, "date must be YYYY-MM-DD: {ts}");
        let year: i32 = date_parts[0].parse().expect("year must be numeric");
        assert!(
            (2020..=2100).contains(&year),
            "year must be plausible: {year}"
        );
    }
}
