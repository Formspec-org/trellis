// Rust guideline compliant 2026-02-21
//! Trellis verification for single events, tamper fixtures, and export ZIPs.

#![forbid(unsafe_code)]

use std::backtrace::Backtrace;
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};
use std::io::Cursor;

use ciborium::Value;
use ed25519_dalek::ed25519::signature::Verifier;
use ed25519_dalek::{Signature, VerifyingKey};
use sha2::{Digest, Sha256};
use trellis_cose::sig_structure_bytes;
use trellis_types::{
    AUTHOR_EVENT_DOMAIN, CONTENT_DOMAIN, COSE_LABEL_SUITE_ID, EVENT_DOMAIN, SUITE_ID_PHASE_1,
    domain_separated_sha256, encode_bstr, encode_tstr, encode_uint,
};
use zip::ZipArchive;

const SUITE_ID_PHASE_1_I128: i128 = SUITE_ID_PHASE_1 as i128;
const ALG_EDDSA: i128 = -8;
const COSE_LABEL_ALG: i128 = 1;
const COSE_LABEL_KID: i128 = 4;
const CHECKPOINT_DOMAIN: &str = "trellis-checkpoint-v1";
const MERKLE_LEAF_DOMAIN: &str = "trellis-merkle-leaf-v1";
const MERKLE_INTERIOR_DOMAIN: &str = "trellis-merkle-interior-v1";
const POSTURE_DECLARATION_DOMAIN: &str = "trellis-posture-declaration-v1";
const ATTACHMENT_EXPORT_EXTENSION: &str = "trellis.export.attachments.v1";
const ATTACHMENT_EVENT_EXTENSION: &str = "trellis.evidence-attachment-binding.v1";
const SIGNATURE_EXPORT_EXTENSION: &str = "trellis.export.signature-affirmations.v1";
const INTAKE_EXPORT_EXTENSION: &str = "trellis.export.intake-handoffs.v1";
const WOS_SIGNATURE_AFFIRMATION_EVENT_TYPE: &str = "wos.kernel.signatureAffirmation";
const WOS_INTAKE_ACCEPTED_EVENT_TYPE: &str = "wos.kernel.intakeAccepted";
const WOS_CASE_CREATED_EVENT_TYPE: &str = "wos.kernel.caseCreated";

/// Verification failure localized to one artifact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerificationFailure {
    pub kind: String,
    pub location: String,
}

impl VerificationFailure {
    fn new(kind: impl Into<String>, location: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            location: location.into(),
        }
    }
}

/// Outcome for one posture-transition verification.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PostureTransitionOutcome {
    pub transition_id: String,
    pub kind: String,
    pub event_index: u64,
    pub from_state: String,
    pub to_state: String,
    pub continuity_verified: bool,
    pub declaration_resolved: bool,
    pub attestations_verified: bool,
    pub failures: Vec<String>,
}

/// Verification report for the current Phase-1 runtime.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VerificationReport {
    pub structure_verified: bool,
    pub integrity_verified: bool,
    pub readability_verified: bool,
    pub event_failures: Vec<VerificationFailure>,
    pub checkpoint_failures: Vec<VerificationFailure>,
    pub proof_failures: Vec<VerificationFailure>,
    pub posture_transitions: Vec<PostureTransitionOutcome>,
    pub warnings: Vec<String>,
}

impl VerificationReport {
    fn fatal(kind: impl Into<String>, warning: impl Into<String>) -> Self {
        let warning = warning.into();
        let kind = kind.into();
        Self {
            structure_verified: false,
            integrity_verified: false,
            readability_verified: false,
            event_failures: vec![VerificationFailure::new(kind, "structure")],
            checkpoint_failures: Vec::new(),
            proof_failures: Vec::new(),
            posture_transitions: Vec::new(),
            warnings: vec![warning],
        }
    }

    fn from_integrity_state(
        event_failures: Vec<VerificationFailure>,
        checkpoint_failures: Vec<VerificationFailure>,
        proof_failures: Vec<VerificationFailure>,
        posture_transitions: Vec<PostureTransitionOutcome>,
        warnings: Vec<String>,
    ) -> Self {
        let posture_ok = posture_transitions.iter().all(|outcome| {
            outcome.continuity_verified
                && outcome.declaration_resolved
                && outcome.attestations_verified
        });

        Self {
            structure_verified: true,
            integrity_verified: event_failures.is_empty()
                && checkpoint_failures.is_empty()
                && proof_failures.is_empty()
                && posture_ok,
            readability_verified: true,
            event_failures,
            checkpoint_failures,
            proof_failures,
            posture_transitions,
            warnings,
        }
    }
}

/// Error returned when verifier inputs cannot be decoded at all.
#[derive(Debug)]
pub struct VerifyError {
    message: String,
    backtrace: Backtrace,
}

impl VerifyError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            backtrace: Backtrace::capture(),
        }
    }

    /// Returns the captured backtrace for this verify failure.
    pub fn backtrace(&self) -> &Backtrace {
        &self.backtrace
    }
}

impl Display for VerifyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for VerifyError {}

/// Verifies one COSE_Sign1 event against one Ed25519 public key.
///
/// # Errors
/// Returns an error when the signed bytes do not decode as a COSE_Sign1 item.
pub fn verify_single_event(
    public_key_bytes: [u8; 32],
    signed_event: &[u8],
) -> Result<VerificationReport, VerifyError> {
    let parsed = parse_sign1_bytes(signed_event)?;
    let mut registry = BTreeMap::new();
    registry.insert(
        parsed.kid.clone(),
        SigningKeyEntry {
            public_key: public_key_bytes,
            status: 0,
            valid_to: None,
        },
    );
    Ok(verify_event_set(
        &[parsed],
        &registry,
        None,
        None,
        false,
        None,
        None,
    ))
}

/// Verifies a tamper-fixture ledger plus its local key registry.
///
/// # Errors
/// Returns an error when the registry bytes cannot be decoded.
pub fn verify_tampered_ledger(
    signing_key_registry: &[u8],
    ledger: &[u8],
    initial_posture_declaration: Option<&[u8]>,
    posture_declaration: Option<&[u8]>,
) -> Result<VerificationReport, VerifyError> {
    let (registry, non_signing) = parse_key_registry(signing_key_registry)?;
    let events = parse_sign1_array(ledger).unwrap_or_else(|_| Vec::new());
    if events.is_empty() {
        return Ok(VerificationReport::fatal(
            "malformed_cose",
            "ledger is not a non-empty dCBOR array of COSE_Sign1 events",
        ));
    }

    Ok(verify_event_set_with_classes(
        &events,
        &registry,
        Some(&non_signing),
        initial_posture_declaration,
        posture_declaration,
        true,
        None,
        None,
    ))
}

/// Verifies a complete export ZIP.
pub fn verify_export_zip(export_zip: &[u8]) -> VerificationReport {
    let archive = match parse_export_zip(export_zip) {
        Ok(archive) => archive,
        Err(error) => {
            return VerificationReport::fatal(
                "export_zip_invalid",
                format!("failed to open export ZIP: {error}"),
            );
        }
    };

    let signing_key_registry_bytes = match archive.members.get("030-signing-key-registry.cbor") {
        Some(bytes) => bytes,
        None => {
            return VerificationReport::fatal(
                "missing_signing_key_registry",
                "export is missing 030-signing-key-registry.cbor",
            );
        }
    };
    let (registry, non_signing_registry) = match parse_key_registry(signing_key_registry_bytes) {
        Ok(maps) => maps,
        Err(error) => {
            return VerificationReport::fatal(
                "signing_key_registry_invalid",
                format!("failed to decode signing-key registry: {error}"),
            );
        }
    };

    let manifest_bytes = match archive.members.get("000-manifest.cbor") {
        Some(bytes) => bytes,
        None => {
            return VerificationReport::fatal(
                "missing_manifest",
                "export is missing 000-manifest.cbor",
            );
        }
    };
    let manifest = match parse_sign1_bytes(manifest_bytes) {
        Ok(manifest) => manifest,
        Err(error) => {
            return VerificationReport::fatal(
                "manifest_structure_invalid",
                format!("manifest is not a valid COSE_Sign1 envelope: {error}"),
            );
        }
    };

    if manifest.alg != ALG_EDDSA || manifest.suite_id != SUITE_ID_PHASE_1_I128 {
        return VerificationReport::fatal(
            "unsupported_suite",
            "manifest protected header does not match the Trellis Phase-1 suite",
        );
    }

    let manifest_public_key = match registry.get(&manifest.kid) {
        Some(entry) => entry.public_key,
        None => {
            return VerificationReport::fatal(
                "unresolvable_manifest_kid",
                "manifest kid is not resolvable via the embedded signing-key registry",
            );
        }
    };
    if !verify_signature(&manifest, manifest_public_key) {
        return VerificationReport::fatal(
            "manifest_signature_invalid",
            "manifest COSE signature is invalid",
        );
    }

    let manifest_payload_bytes = match &manifest.payload {
        Some(bytes) => bytes,
        None => {
            return VerificationReport::fatal(
                "manifest_payload_missing",
                "manifest payload is detached, which is out of scope for Phase 1",
            );
        }
    };
    let manifest_payload = match decode_value(manifest_payload_bytes) {
        Ok(value) => value,
        Err(error) => {
            return VerificationReport::fatal(
                "manifest_payload_invalid",
                format!("failed to decode manifest payload: {error}"),
            );
        }
    };
    let manifest_map = match manifest_payload.as_map() {
        Some(map) => map,
        None => {
            return VerificationReport::fatal(
                "manifest_payload_invalid",
                "manifest payload root is not a map",
            );
        }
    };

    let required_digests = [
        ("010-events.cbor", "events_digest"),
        ("020-inclusion-proofs.cbor", "inclusion_proofs_digest"),
        ("025-consistency-proofs.cbor", "consistency_proofs_digest"),
        (
            "030-signing-key-registry.cbor",
            "signing_key_registry_digest",
        ),
        ("040-checkpoints.cbor", "checkpoints_digest"),
    ];
    for (member_name, field_name) in required_digests {
        let expected = match map_lookup_fixed_bytes(manifest_map, field_name, 32) {
            Ok(bytes) => bytes,
            Err(error) => {
                return VerificationReport::fatal(
                    "manifest_payload_invalid",
                    format!("manifest is missing {field_name}: {error}"),
                );
            }
        };
        let actual = match archive.members.get(member_name) {
            Some(bytes) => sha256_bytes(bytes),
            None => {
                return VerificationReport::fatal(
                    "archive_integrity_failure",
                    format!("export is missing required member {member_name}"),
                );
            }
        };
        if expected.as_slice() != actual {
            return VerificationReport::fatal(
                "archive_integrity_failure",
                format!("manifest digest mismatch for {member_name}"),
            );
        }
    }

    let registry_bindings = match map_lookup_array(manifest_map, "registry_bindings") {
        Ok(bindings) => bindings,
        Err(error) => {
            return VerificationReport::fatal(
                "manifest_payload_invalid",
                format!("manifest registry_bindings are invalid: {error}"),
            );
        }
    };
    let mut parsed_bindings = Vec::new();
    for binding in registry_bindings {
        let binding_map = match binding.as_map() {
            Some(map) => map,
            None => {
                return VerificationReport::fatal(
                    "manifest_payload_invalid",
                    "registry binding is not a map",
                );
            }
        };
        let digest = match map_lookup_fixed_bytes(binding_map, "registry_digest", 32) {
            Ok(bytes) => bytes,
            Err(error) => {
                return VerificationReport::fatal(
                    "manifest_payload_invalid",
                    format!("registry binding digest is invalid: {error}"),
                );
            }
        };
        let member_name = format!("050-registries/{}.cbor", hex_string(&digest));
        let actual = match archive.members.get(&member_name) {
            Some(bytes) => sha256_bytes(bytes),
            None => {
                return VerificationReport::fatal(
                    "archive_integrity_failure",
                    format!("export is missing bound registry member {member_name}"),
                );
            }
        };
        if actual != digest.as_slice() {
            return VerificationReport::fatal(
                "archive_integrity_failure",
                format!("bound registry digest mismatch for {member_name}"),
            );
        }
        let bound_at_sequence = match map_lookup_u64(binding_map, "bound_at_sequence") {
            Ok(value) => value,
            Err(error) => {
                return VerificationReport::fatal(
                    "manifest_payload_invalid",
                    format!("registry binding bound_at_sequence is invalid: {error}"),
                );
            }
        };
        parsed_bindings.push(RegistryBindingInfo {
            digest_hex: hex_string(&digest),
            bound_at_sequence,
        });
    }
    parsed_bindings.sort_by_key(|binding| binding.bound_at_sequence);

    let mut parsed_registries = BTreeMap::new();
    for binding in &parsed_bindings {
        let member_name = format!("050-registries/{}.cbor", binding.digest_hex);
        let registry_bytes = archive
            .members
            .get(&member_name)
            .expect("bound registry exists");
        match parse_bound_registry(registry_bytes) {
            Ok(registry) => {
                parsed_registries.insert(binding.digest_hex.clone(), registry);
            }
            Err(error) => {
                return VerificationReport::fatal(
                    "bound_registry_invalid",
                    format!("failed to decode {member_name}: {error}"),
                );
            }
        }
    }

    let scope = match map_lookup_bytes(manifest_map, "scope") {
        Ok(bytes) => bytes,
        Err(error) => {
            return VerificationReport::fatal(
                "manifest_payload_invalid",
                format!("manifest scope is invalid: {error}"),
            );
        }
    };

    let events = match archive.members.get("010-events.cbor") {
        Some(bytes) => match parse_sign1_array(bytes) {
            Ok(events) => events,
            Err(error) => {
                return VerificationReport::fatal(
                    "events_invalid",
                    format!("failed to decode 010-events.cbor: {error}"),
                );
            }
        },
        None => unreachable!("required member already checked"),
    };
    let payload_blobs = archive
        .members
        .iter()
        .filter_map(|(name, bytes)| {
            let digest_hex = name.strip_prefix("060-payloads/")?.strip_suffix(".bin")?;
            let digest_bytes = hex_decode(digest_hex).ok()?;
            let digest: [u8; 32] = digest_bytes.try_into().ok()?;
            Some((digest, bytes.clone()))
        })
        .collect::<BTreeMap<_, _>>();
    let mut report = verify_event_set_with_classes(
        &events,
        &registry,
        Some(&non_signing_registry),
        None,
        None,
        false,
        Some(scope.as_slice()),
        Some(&payload_blobs),
    );
    if let Some(extension) = match parse_attachment_export_extension(manifest_map) {
        Ok(extension) => extension,
        Err(error) => {
            return VerificationReport::fatal(
                "manifest_payload_invalid",
                format!("attachment export extension is invalid: {error}"),
            );
        }
    } {
        verify_attachment_manifest(&archive, &events, &extension, &mut report);
    }
    if let Some(extension) = match parse_signature_export_extension(manifest_map) {
        Ok(extension) => extension,
        Err(error) => {
            return VerificationReport::fatal(
                "manifest_payload_invalid",
                format!("signature export extension is invalid: {error}"),
            );
        }
    } {
        verify_signature_catalog(&archive, &events, &payload_blobs, &extension, &mut report);
    }
    if let Some(extension) = match parse_intake_export_extension(manifest_map) {
        Ok(extension) => extension,
        Err(error) => {
            return VerificationReport::fatal(
                "manifest_payload_invalid",
                format!("intake export extension is invalid: {error}"),
            );
        }
    } {
        verify_intake_catalog(&archive, &events, &payload_blobs, &extension, &mut report);
    }
    for failure in &mut report.event_failures {
        if failure.kind == "scope_mismatch" {
            failure.location = format!("manifest-scope/{}", failure.location);
        }
    }
    for event in &events {
        let details = match decode_event_details(event) {
            Ok(details) => details,
            Err(_) => continue,
        };
        let Some(binding) = parsed_bindings
            .iter()
            .filter(|binding| binding.bound_at_sequence <= details.sequence)
            .max_by_key(|binding| binding.bound_at_sequence)
        else {
            report.event_failures.push(VerificationFailure::new(
                "registry_digest_mismatch",
                hex_string(&details.canonical_event_hash),
            ));
            continue;
        };
        let Some(bound_registry) = parsed_registries.get(&binding.digest_hex) else {
            report.event_failures.push(VerificationFailure::new(
                "registry_digest_mismatch",
                hex_string(&details.canonical_event_hash),
            ));
            continue;
        };
        if !bound_registry
            .event_types
            .iter()
            .any(|value| value == &details.event_type)
            || !bound_registry
                .classifications
                .iter()
                .any(|value| value == &details.classification)
        {
            report.event_failures.push(VerificationFailure::new(
                "registry_digest_mismatch",
                hex_string(&details.canonical_event_hash),
            ));
        }
    }

    let canonical_hashes = events
        .iter()
        .filter_map(|event| event_identity(event).ok())
        .map(|(_, canonical_hash)| canonical_hash)
        .collect::<Vec<_>>();
    let leaf_hashes = canonical_hashes
        .iter()
        .copied()
        .map(merkle_leaf_hash)
        .collect::<Vec<_>>();

    let checkpoints = match archive.members.get("040-checkpoints.cbor") {
        Some(bytes) => match parse_sign1_array(bytes) {
            Ok(checkpoints) => checkpoints,
            Err(error) => {
                return VerificationReport::fatal(
                    "checkpoints_invalid",
                    format!("failed to decode 040-checkpoints.cbor: {error}"),
                );
            }
        },
        None => unreachable!("required member already checked"),
    };

    let mut prior_checkpoint_digest: Option<[u8; 32]> = None;
    let mut head_checkpoint_root: Option<[u8; 32]> = None;
    for checkpoint in &checkpoints {
        let public_key = match registry.get(&checkpoint.kid) {
            Some(entry) => entry.public_key,
            None => {
                return VerificationReport::fatal(
                    "unresolvable_manifest_kid",
                    "checkpoint kid is not resolvable via the embedded signing-key registry",
                );
            }
        };
        if !verify_signature(checkpoint, public_key) {
            return VerificationReport::fatal(
                "checkpoint_signature_invalid",
                "checkpoint COSE signature is invalid",
            );
        }

        let payload_bytes = checkpoint.payload.as_ref().expect("checkpoints are inline");
        let payload = match decode_value(payload_bytes) {
            Ok(value) => value,
            Err(error) => {
                return VerificationReport::fatal(
                    "checkpoint_payload_invalid",
                    format!("failed to decode checkpoint payload: {error}"),
                );
            }
        };
        let payload_map = match payload.as_map() {
            Some(map) => map,
            None => {
                return VerificationReport::fatal(
                    "checkpoint_payload_invalid",
                    "checkpoint payload root is not a map",
                );
            }
        };

        let checkpoint_scope = match map_lookup_bytes(payload_map, "scope") {
            Ok(bytes) => bytes,
            Err(error) => {
                return VerificationReport::fatal(
                    "checkpoint_payload_invalid",
                    format!("checkpoint scope is invalid: {error}"),
                );
            }
        };
        if checkpoint_scope != scope {
            report.checkpoint_failures.push(VerificationFailure::new(
                "scope_mismatch",
                "checkpoint/scope",
            ));
            continue;
        }

        let tree_size = match map_lookup_u64(payload_map, "tree_size") {
            Ok(value) => value as usize,
            Err(error) => {
                return VerificationReport::fatal(
                    "checkpoint_payload_invalid",
                    format!("checkpoint tree_size is invalid: {error}"),
                );
            }
        };
        if tree_size == 0 || tree_size > leaf_hashes.len() {
            report.checkpoint_failures.push(VerificationFailure::new(
                "tree_size_invalid",
                format!("checkpoint/tree_size/{tree_size}"),
            ));
            continue;
        }

        let expected_root = merkle_root(&leaf_hashes[..tree_size]);
        let actual_root = match map_lookup_fixed_bytes(payload_map, "tree_head_hash", 32) {
            Ok(bytes) => bytes_array(&bytes),
            Err(error) => {
                return VerificationReport::fatal(
                    "checkpoint_payload_invalid",
                    format!("checkpoint tree_head_hash is invalid: {error}"),
                );
            }
        };
        if expected_root != actual_root {
            report.checkpoint_failures.push(VerificationFailure::new(
                "checkpoint_root_mismatch",
                format!("checkpoint/tree_size/{tree_size}"),
            ));
        }

        let digest = checkpoint_digest(&scope, payload_bytes);
        if let Some(previous) = prior_checkpoint_digest {
            let actual_prev = match map_lookup_fixed_bytes(payload_map, "prev_checkpoint_hash", 32)
            {
                Ok(bytes) => bytes_array(&bytes),
                Err(error) => {
                    return VerificationReport::fatal(
                        "checkpoint_payload_invalid",
                        format!("checkpoint prev_checkpoint_hash is invalid: {error}"),
                    );
                }
            };
            if previous != actual_prev {
                report.checkpoint_failures.push(VerificationFailure::new(
                    "prev_checkpoint_hash_mismatch",
                    format!("checkpoint/tree_size/{tree_size}"),
                ));
            }
        }
        prior_checkpoint_digest = Some(digest);
        head_checkpoint_root = Some(actual_root);
    }

    let head_checkpoint_digest =
        match map_lookup_fixed_bytes(manifest_map, "head_checkpoint_digest", 32) {
            Ok(bytes) => bytes_array(&bytes),
            Err(error) => {
                return VerificationReport::fatal(
                    "manifest_payload_invalid",
                    format!("manifest head_checkpoint_digest is invalid: {error}"),
                );
            }
        };
    if prior_checkpoint_digest != Some(head_checkpoint_digest) {
        report.checkpoint_failures.push(VerificationFailure::new(
            "head_checkpoint_digest_mismatch",
            "manifest/head_checkpoint_digest",
        ));
    }

    let inclusion_map = match archive.members.get("020-inclusion-proofs.cbor") {
        Some(bytes) => match decode_value(bytes) {
            Ok(value) => value,
            Err(error) => {
                return VerificationReport::fatal(
                    "inclusion_proofs_invalid",
                    format!("failed to decode 020-inclusion-proofs.cbor: {error}"),
                );
            }
        },
        None => unreachable!("required member already checked"),
    };
    if let Some(proofs) = inclusion_map.as_map() {
        let expected_root = head_checkpoint_root.unwrap_or([0u8; 32]);
        for (_, proof_value) in proofs {
            let proof_map = match proof_value.as_map() {
                Some(map) => map,
                None => {
                    report.proof_failures.push(VerificationFailure::new(
                        "inclusion_proof_invalid",
                        "proof/map",
                    ));
                    continue;
                }
            };
            let tree_size = match map_lookup_u64(proof_map, "tree_size") {
                Ok(value) => value as usize,
                Err(_) => {
                    report.proof_failures.push(VerificationFailure::new(
                        "inclusion_proof_invalid",
                        "proof/tree_size",
                    ));
                    continue;
                }
            };
            if tree_size != leaf_hashes.len() {
                report.proof_failures.push(VerificationFailure::new(
                    "inclusion_proof_invalid",
                    format!("proof/tree_size/{tree_size}"),
                ));
                continue;
            }
            let leaf_index = match map_lookup_u64(proof_map, "leaf_index") {
                Ok(value) => value as usize,
                Err(_) => {
                    report.proof_failures.push(VerificationFailure::new(
                        "inclusion_proof_invalid",
                        "proof/leaf_index",
                    ));
                    continue;
                }
            };
            if leaf_index >= leaf_hashes.len() {
                report.proof_failures.push(VerificationFailure::new(
                    "inclusion_proof_invalid",
                    format!("proof/index/{leaf_index}"),
                ));
                continue;
            }
            let leaf_hash = match map_lookup_fixed_bytes(proof_map, "leaf_hash", 32) {
                Ok(bytes) => bytes_array(&bytes),
                Err(_) => {
                    report.proof_failures.push(VerificationFailure::new(
                        "inclusion_proof_invalid",
                        format!("proof/index/{leaf_index}"),
                    ));
                    continue;
                }
            };
            let audit_path_values = match map_lookup_array(proof_map, "audit_path") {
                Ok(path) => path,
                Err(_) => {
                    report.proof_failures.push(VerificationFailure::new(
                        "inclusion_proof_invalid",
                        format!("proof/index/{leaf_index}"),
                    ));
                    continue;
                }
            };
            let audit_path = match digest_path_from_values(audit_path_values) {
                Ok(nodes) => nodes,
                Err(_) => {
                    report.proof_failures.push(VerificationFailure::new(
                        "inclusion_proof_invalid",
                        format!("proof/index/{leaf_index}/audit_path"),
                    ));
                    continue;
                }
            };
            let matches_leaf = leaf_hash == leaf_hashes[leaf_index];
            let matches_root = root_from_inclusion_proof(
                leaf_index as u64,
                tree_size as u64,
                leaf_hash,
                &audit_path,
            )
            .is_ok_and(|root| root == expected_root);
            if !matches_leaf || !matches_root {
                report.proof_failures.push(VerificationFailure::new(
                    "inclusion_proof_mismatch",
                    format!("proof/index/{leaf_index}"),
                ));
            }
        }
    }

    let consistency_value = match archive.members.get("025-consistency-proofs.cbor") {
        Some(bytes) => match decode_value(bytes) {
            Ok(value) => value,
            Err(error) => {
                return VerificationReport::fatal(
                    "consistency_proofs_invalid",
                    format!("failed to decode 025-consistency-proofs.cbor: {error}"),
                );
            }
        },
        None => unreachable!("required member already checked"),
    };
    if let Some(records) = consistency_value.as_array() {
        for record in records {
            let record_map = match record.as_map() {
                Some(map) => map,
                None => {
                    report.proof_failures.push(VerificationFailure::new(
                        "consistency_proof_invalid",
                        "consistency/map",
                    ));
                    continue;
                }
            };
            let from_tree_size = map_lookup_u64(record_map, "from_tree_size").unwrap_or(0) as usize;
            let to_tree_size = map_lookup_u64(record_map, "to_tree_size").unwrap_or(0) as usize;
            let proof_path_values = match map_lookup_array(record_map, "proof_path") {
                Ok(path) => path,
                Err(_) => {
                    report.proof_failures.push(VerificationFailure::new(
                        "consistency_proof_invalid",
                        format!("consistency/{from_tree_size}-{to_tree_size}/proof_path"),
                    ));
                    continue;
                }
            };
            let location = format!("consistency/{from_tree_size}-{to_tree_size}");
            if from_tree_size == 0 {
                report.proof_failures.push(VerificationFailure::new(
                    "consistency_proof_invalid",
                    format!("{location}/from_zero"),
                ));
                continue;
            }
            if from_tree_size >= to_tree_size || to_tree_size > leaf_hashes.len() {
                report.proof_failures.push(VerificationFailure::new(
                    "consistency_proof_invalid",
                    location.clone(),
                ));
                continue;
            }
            let proof_path = match digest_path_from_values(proof_path_values) {
                Ok(nodes) => nodes,
                Err(_) => {
                    report.proof_failures.push(VerificationFailure::new(
                        "consistency_proof_invalid",
                        format!("{location}/proof_path"),
                    ));
                    continue;
                }
            };
            let root_old = merkle_root(&leaf_hashes[..from_tree_size]);
            let root_new = merkle_root(&leaf_hashes[..to_tree_size]);
            match root_from_consistency_proof(
                from_tree_size as u64,
                to_tree_size as u64,
                root_old,
                &proof_path,
            ) {
                Ok(computed) if computed == root_new => {}
                Ok(_) => report.proof_failures.push(VerificationFailure::new(
                    "consistency_proof_mismatch",
                    location,
                )),
                Err(_) => report.proof_failures.push(VerificationFailure::new(
                    "consistency_proof_invalid",
                    location,
                )),
            }
        }
    }

    report.structure_verified = true;
    report.integrity_verified = report.event_failures.is_empty()
        && report.checkpoint_failures.is_empty()
        && report.proof_failures.is_empty()
        && report.posture_transitions.iter().all(|outcome| {
            outcome.continuity_verified
                && outcome.declaration_resolved
                && outcome.attestations_verified
        });
    report.readability_verified = true;
    report
}

#[derive(Clone, Debug)]
struct ParsedSign1 {
    protected_bytes: Vec<u8>,
    kid: Vec<u8>,
    alg: i128,
    suite_id: i128,
    payload: Option<Vec<u8>>,
    signature: [u8; 64],
}

#[derive(Clone, Debug)]
struct EventDetails {
    scope: Vec<u8>,
    sequence: u64,
    authored_at: u64,
    event_type: String,
    classification: String,
    prev_hash: Option<[u8; 32]>,
    author_event_hash: [u8; 32],
    content_hash: [u8; 32],
    canonical_event_hash: [u8; 32],
    payload_ref: PayloadRef,
    transition: Option<TransitionDetails>,
    attachment_binding: Option<AttachmentBindingDetails>,
}

#[derive(Clone, Debug)]
struct SigningKeyEntry {
    public_key: [u8; 32],
    status: u64,
    valid_to: Option<u64>,
}

/// A reserved non-signing `KeyEntry` (Core §8.7 / ADR 0006).
///
/// Phase-1 verifiers track these so a signature attempt under a kid registered
/// as `tenant-root`, `scope`, `subject`, or `recovery` can be flagged with
/// `key_class_mismatch` (Core §8.7.3 step 4) rather than the generic
/// `unresolvable_manifest_kid` failure.
#[derive(Clone, Debug)]
struct NonSigningKeyEntry {
    /// Class string from the registry entry's `kind` field, normalized so the
    /// legacy synonym `"wrap"` is mapped to `"subject"` per Core §8.7.6.
    class: String,
}

#[derive(Clone, Debug)]
struct RegistryBindingInfo {
    digest_hex: String,
    bound_at_sequence: u64,
}

#[derive(Clone, Debug)]
struct BoundRegistry {
    event_types: Vec<String>,
    classifications: Vec<String>,
}

#[derive(Clone, Debug)]
enum PayloadRef {
    Inline(Vec<u8>),
    External,
}

#[derive(Clone, Debug)]
struct TransitionDetails {
    kind: TransitionKind,
    transition_id: String,
    from_state: String,
    to_state: String,
    declaration_digest: [u8; 32],
    attestation_classes: Vec<String>,
    /// Only populated for disclosure-profile transitions (Appendix A.5.2).
    /// Custody-model transitions derive their attestation rule from
    /// from_state→to_state custody-rank ordering instead (A.5.3 step 4).
    scope_change: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TransitionKind {
    CustodyModel,
    DisclosureProfile,
}

impl TransitionKind {
    fn as_report_str(&self) -> &'static str {
        match self {
            TransitionKind::CustodyModel => "custody-model",
            TransitionKind::DisclosureProfile => "disclosure-profile",
        }
    }
}

#[derive(Clone, Debug)]
struct AttachmentBindingDetails {
    attachment_id: String,
    slot_path: String,
    media_type: String,
    byte_length: u64,
    attachment_sha256: [u8; 32],
    payload_content_hash: [u8; 32],
    filename: Option<String>,
    prior_binding_hash: Option<[u8; 32]>,
}

#[derive(Clone, Debug)]
struct AttachmentExportExtension {
    manifest_digest: [u8; 32],
    inline_attachments: bool,
}

#[derive(Clone, Debug)]
struct AttachmentManifestEntry {
    binding_event_hash: [u8; 32],
    attachment_id: String,
    slot_path: String,
    media_type: String,
    byte_length: u64,
    attachment_sha256: [u8; 32],
    payload_content_hash: [u8; 32],
    filename: Option<String>,
    prior_binding_hash: Option<[u8; 32]>,
}

#[derive(Clone, Debug)]
struct SignatureExportExtension {
    catalog_digest: [u8; 32],
}

#[derive(Clone, Debug)]
struct IntakeExportExtension {
    catalog_digest: [u8; 32],
}

#[derive(Clone, Debug)]
struct SignatureManifestEntry {
    canonical_event_hash: [u8; 32],
    signer_id: String,
    role_id: String,
    role: String,
    document_id: String,
    document_hash: String,
    document_hash_algorithm: String,
    signed_at: String,
    identity_binding: Value,
    consent_reference: Value,
    signature_provider: String,
    ceremony_id: String,
    profile_ref: Option<String>,
    profile_key: Option<String>,
    formspec_response_ref: String,
}

#[derive(Clone, Debug)]
struct IntakeManifestEntry {
    intake_event_hash: [u8; 32],
    case_created_event_hash: Option<[u8; 32]>,
    handoff: IntakeHandoffDetails,
    response_bytes: Vec<u8>,
}

#[derive(Clone, Debug)]
struct SignatureAffirmationRecordDetails {
    signer_id: String,
    role_id: String,
    role: String,
    document_id: String,
    document_hash: String,
    document_hash_algorithm: String,
    signed_at: String,
    identity_binding: Value,
    consent_reference: Value,
    signature_provider: String,
    ceremony_id: String,
    profile_ref: Option<String>,
    profile_key: Option<String>,
    formspec_response_ref: String,
}

#[derive(Clone, Debug)]
struct IntakeHandoffDetails {
    handoff_id: String,
    initiation_mode: String,
    case_ref: Option<String>,
    definition_url: String,
    definition_version: String,
    response_ref: String,
    response_hash: String,
    validation_report_ref: String,
    ledger_head_ref: String,
}

#[derive(Clone, Debug)]
struct IntakeAcceptedRecordDetails {
    intake_id: String,
    case_intent: String,
    case_disposition: String,
    case_ref: String,
    definition_url: Option<String>,
    definition_version: Option<String>,
}

#[derive(Clone, Debug)]
struct CaseCreatedRecordDetails {
    case_ref: String,
    intake_handoff_ref: String,
    formspec_response_ref: String,
    validation_report_ref: String,
    ledger_head_ref: String,
    initiation_mode: String,
}

#[derive(Debug)]
/// Parsed export ZIP: keys are **relative** paths under a single root directory
/// (for example `000-manifest.cbor`), not full ZIP entry names.
///
/// Every committed export uses exactly one top-level directory; see
/// [`parse_export_zip`] for the layout contract.
struct ExportArchive {
    members: BTreeMap<String, Vec<u8>>,
}

/// Parses a Trellis export ZIP into [`ExportArchive`] members.
///
/// **Layout contract:** each ZIP entry name must contain exactly one `/`
/// separating `{export_root}/` from the relative member path. Top-level
/// entries, extra leading segments, or nested roots are rejected so member
/// paths stay stable across toolchains.
fn parse_export_zip(bytes: &[u8]) -> Result<ExportArchive, VerifyError> {
    let mut archive = ZipArchive::new(Cursor::new(bytes))
        .map_err(|error| VerifyError::new(format!("failed to parse ZIP: {error}")))?;
    let mut members = BTreeMap::new();
    for index in 0..archive.len() {
        let mut file = archive
            .by_index(index)
            .map_err(|error| VerifyError::new(format!("failed to read ZIP member: {error}")))?;
        let name = file.name().to_string();
        let Some((_, relative_name)) = name.split_once('/') else {
            return Err(VerifyError::new(
                "ZIP member does not live under one export root",
            ));
        };
        let mut data = Vec::new();
        std::io::Read::read_to_end(&mut file, &mut data).map_err(|error| {
            VerifyError::new(format!("failed to read ZIP member bytes: {error}"))
        })?;
        members.insert(relative_name.to_string(), data);
    }
    Ok(ExportArchive { members })
}

fn verify_event_set(
    events: &[ParsedSign1],
    registry: &BTreeMap<Vec<u8>, SigningKeyEntry>,
    initial_posture_declaration: Option<&[u8]>,
    posture_declaration: Option<&[u8]>,
    classify_tamper: bool,
    expected_ledger_scope: Option<&[u8]>,
    payload_blobs: Option<&BTreeMap<[u8; 32], Vec<u8>>>,
) -> VerificationReport {
    verify_event_set_with_classes(
        events,
        registry,
        None,
        initial_posture_declaration,
        posture_declaration,
        classify_tamper,
        expected_ledger_scope,
        payload_blobs,
    )
}

#[allow(clippy::too_many_arguments)]
fn verify_event_set_with_classes(
    events: &[ParsedSign1],
    registry: &BTreeMap<Vec<u8>, SigningKeyEntry>,
    non_signing_registry: Option<&BTreeMap<Vec<u8>, NonSigningKeyEntry>>,
    initial_posture_declaration: Option<&[u8]>,
    posture_declaration: Option<&[u8]>,
    classify_tamper: bool,
    expected_ledger_scope: Option<&[u8]>,
    payload_blobs: Option<&BTreeMap<[u8; 32], Vec<u8>>>,
) -> VerificationReport {
    let mut event_failures = Vec::new();
    let mut posture_transitions = Vec::new();
    let mut previous_hash: Option<[u8; 32]> = None;
    let skip_prev_hash_check = initial_posture_declaration.is_some() && events.len() == 1;
    let mut shadow_custody_model =
        initial_posture_declaration.and_then(|bytes| parse_custody_model(bytes).ok());
    let mut shadow_disclosure_profile =
        initial_posture_declaration.and_then(|bytes| parse_disclosure_profile(bytes).ok());

    for (index, event) in events.iter().enumerate() {
        let key_entry = match registry.get(&event.kid) {
            Some(entry) => entry,
            None => {
                // Core §8.7.3 step 4: if the kid resolves to a reserved
                // non-signing class, this is `key_class_mismatch` rather
                // than `unresolvable_manifest_kid`. Recovery-only keys are
                // the canonical class-confusion attack surface; tenant-root
                // / scope / subject kids signing ordinary events are also
                // class violations under the unified taxonomy.
                if let Some(non_signing) = non_signing_registry
                    .and_then(|map| map.get(&event.kid))
                {
                    return VerificationReport::fatal(
                        "key_class_mismatch",
                        format!(
                            "event signed under a `{}`-class kid; only `signing` keys may sign canonical events (Core §8.7.3 step 4)",
                            non_signing.class
                        ),
                    );
                }
                return VerificationReport::fatal(
                    "unresolvable_manifest_kid",
                    "event kid is not resolvable via the provided signing-key registry",
                );
            }
        };
        if event.alg != ALG_EDDSA || event.suite_id != SUITE_ID_PHASE_1_I128 {
            return VerificationReport::fatal(
                "unsupported_suite",
                "event protected header does not match the Trellis Phase-1 suite",
            );
        }
        if !verify_signature(event, key_entry.public_key) {
            let location = event_identity(event)
                .map(|(_, hash)| hex_string(&hash))
                .unwrap_or_else(|_| format!("event[{index}]"));
            event_failures.push(VerificationFailure::new("signature_invalid", location));
            continue;
        }

        let details = match decode_event_details(event) {
            Ok(details) => details,
            Err(_) => {
                return VerificationReport::fatal(
                    "malformed_cose",
                    "event payload does not decode as a canonical Trellis event",
                );
            }
        };

        if key_entry.status == 3 {
            match key_entry.valid_to {
                Some(valid_to) if details.authored_at > valid_to => {
                    event_failures.push(VerificationFailure::new(
                        "revoked_authority",
                        hex_string(&details.canonical_event_hash),
                    ));
                }
                None => {
                    return VerificationReport::fatal(
                        "signing_key_registry_invalid",
                        "revoked signing-key registry entry is missing valid_to",
                    );
                }
                // Key is revoked, but this event was authored on or before
                // `valid_to` — accepted per Core §19 (historical signatures).
                _ => {}
            }
        }

        if let Some(expected) = expected_ledger_scope {
            if details.scope.as_slice() != expected {
                event_failures.push(VerificationFailure::new(
                    "scope_mismatch",
                    hex_string(&details.canonical_event_hash),
                ));
            }
        }

        match &details.payload_ref {
            PayloadRef::Inline(ciphertext) => {
                let expected_content_hash = domain_separated_sha256(CONTENT_DOMAIN, ciphertext);
                if expected_content_hash != details.content_hash {
                    event_failures.push(VerificationFailure::new(
                        "content_hash_mismatch",
                        hex_string(&details.canonical_event_hash),
                    ));
                }
            }
            PayloadRef::External => {
                if let Some(blobs) = payload_blobs
                    && let Some(payload_bytes) = blobs.get(&details.content_hash)
                {
                    let expected_content_hash =
                        domain_separated_sha256(CONTENT_DOMAIN, payload_bytes);
                    if expected_content_hash != details.content_hash {
                        event_failures.push(VerificationFailure::new(
                            "content_hash_mismatch",
                            hex_string(&details.canonical_event_hash),
                        ));
                    }
                }
            }
        }

        let payload_bytes = match event.payload.as_ref() {
            Some(bytes) => bytes.as_slice(),
            None => {
                event_failures.push(VerificationFailure::new(
                    "malformed_cose",
                    format!("event[{index}]"),
                ));
                continue;
            }
        };
        match recompute_author_event_hash(payload_bytes) {
            Some(expected_author_hash) if expected_author_hash == details.author_event_hash => {}
            Some(_) => {
                event_failures.push(VerificationFailure::new(
                    "hash_mismatch",
                    hex_string(&details.canonical_event_hash),
                ));
            }
            None => {
                event_failures.push(VerificationFailure::new(
                    "author_preimage_invalid",
                    hex_string(&details.canonical_event_hash),
                ));
            }
        }

        if skip_prev_hash_check {
        } else if details.sequence == 0 {
            if details.prev_hash.is_some() {
                let kind = if classify_tamper {
                    "event_reorder"
                } else {
                    "prev_hash_mismatch"
                };
                event_failures.push(VerificationFailure::new(
                    kind,
                    hex_string(&details.canonical_event_hash),
                ));
            }
        } else if previous_hash != details.prev_hash {
            let kind = if classify_tamper {
                if previous_hash.is_none() && events.len() == 1 {
                    "event_truncation"
                } else if previous_hash.is_none() {
                    "event_reorder"
                } else {
                    "prev_hash_break"
                }
            } else {
                "prev_hash_mismatch"
            };
            event_failures.push(VerificationFailure::new(
                kind,
                hex_string(&details.canonical_event_hash),
            ));
        }
        previous_hash = Some(details.canonical_event_hash);

        if let Some(transition) = details.transition {
            let mut outcome = PostureTransitionOutcome {
                transition_id: transition.transition_id.clone(),
                kind: transition.kind.as_report_str().to_string(),
                event_index: index as u64,
                from_state: transition.from_state.clone(),
                to_state: transition.to_state.clone(),
                continuity_verified: true,
                declaration_resolved: true,
                attestations_verified: true,
                failures: Vec::new(),
            };

            let shadow_state = match transition.kind {
                TransitionKind::CustodyModel => shadow_custody_model.clone(),
                TransitionKind::DisclosureProfile => shadow_disclosure_profile.clone(),
            };
            if let Some(initial_state) = shadow_state {
                if transition.from_state != initial_state {
                    outcome.continuity_verified = false;
                    outcome.failures.push("state_continuity_mismatch".into());
                }
            }

            if let Some(declaration_bytes) = posture_declaration {
                let expected_declaration_digest =
                    domain_separated_sha256(POSTURE_DECLARATION_DOMAIN, declaration_bytes);
                if expected_declaration_digest != transition.declaration_digest {
                    outcome.continuity_verified = false;
                    outcome.declaration_resolved = false;
                    outcome
                        .failures
                        .push("posture_declaration_digest_mismatch".into());
                }
            }

            let dual_required = match transition.kind {
                TransitionKind::CustodyModel => {
                    requires_dual_attestation(&transition.from_state, &transition.to_state)
                }
                TransitionKind::DisclosureProfile => {
                    // Appendix A.5.3 step 4: Narrowing MAY be attested by the
                    // new authority alone; Widening and Orthogonal MUST be
                    // dually attested. Unknown values fall through to dual
                    // as the conservative default.
                    match transition.scope_change.as_deref() {
                        Some("Narrowing") => false,
                        Some("Widening") | Some("Orthogonal") => true,
                        _ => true,
                    }
                }
            };
            if dual_required
                && !(transition
                    .attestation_classes
                    .iter()
                    .any(|value| value == "prior")
                    && transition
                        .attestation_classes
                        .iter()
                        .any(|value| value == "new"))
            {
                outcome.attestations_verified = false;
                outcome.failures.push("attestation_insufficient".into());
            }

            if let Some(first_failure) = outcome.failures.first() {
                event_failures.push(VerificationFailure::new(
                    first_failure.clone(),
                    hex_string(&details.canonical_event_hash),
                ));
            }
            match transition.kind {
                TransitionKind::CustodyModel => {
                    shadow_custody_model = Some(transition.to_state.clone());
                }
                TransitionKind::DisclosureProfile => {
                    shadow_disclosure_profile = Some(transition.to_state.clone());
                }
            }
            posture_transitions.push(outcome);
        }
    }

    VerificationReport::from_integrity_state(
        event_failures,
        Vec::new(),
        Vec::new(),
        posture_transitions,
        Vec::new(),
    )
}

fn parse_sign1_array(bytes: &[u8]) -> Result<Vec<ParsedSign1>, VerifyError> {
    let value = decode_value(bytes)?;
    let items = value
        .as_array()
        .ok_or_else(|| VerifyError::new("expected a dCBOR array"))?;
    items.iter().map(parse_sign1_value).collect()
}

fn parse_sign1_bytes(bytes: &[u8]) -> Result<ParsedSign1, VerifyError> {
    let value = decode_value(bytes)?;
    parse_sign1_value(&value)
}

fn parse_sign1_value(value: &Value) -> Result<ParsedSign1, VerifyError> {
    let tagged = match value {
        Value::Tag(18, inner) => inner,
        Value::Tag(tag, _) => {
            return Err(VerifyError::new(format!(
                "unexpected COSE tag {tag}; expected 18"
            )));
        }
        _ => return Err(VerifyError::new("value is not a tag-18 COSE_Sign1 item")),
    };
    let items = tagged
        .as_array()
        .ok_or_else(|| VerifyError::new("COSE_Sign1 body is not an array"))?;
    if items.len() != 4 {
        return Err(VerifyError::new(
            "COSE_Sign1 body does not contain four fields",
        ));
    }

    let protected_bytes = items[0]
        .as_bytes()
        .cloned()
        .ok_or_else(|| VerifyError::new("protected header is not a byte string"))?;
    let protected_value = decode_value(&protected_bytes)?;
    let protected_map = protected_value
        .as_map()
        .ok_or_else(|| VerifyError::new("protected header does not decode to a map"))?;
    let kid = map_lookup_integer_label_bytes(protected_map, COSE_LABEL_KID)?;
    let alg = map_lookup_integer_label(protected_map, COSE_LABEL_ALG)?;
    let suite_id = map_lookup_integer_label(protected_map, COSE_LABEL_SUITE_ID)?;

    match &items[1] {
        Value::Map(entries) if entries.is_empty() => {}
        Value::Map(_) => return Err(VerifyError::new("unprotected header map must be empty")),
        _ => return Err(VerifyError::new("unprotected header is not a map")),
    }

    let payload = match &items[2] {
        Value::Bytes(bytes) => Some(bytes.clone()),
        Value::Null => None,
        _ => return Err(VerifyError::new("payload is neither bytes nor null")),
    };
    let signature = items[3]
        .as_bytes()
        .cloned()
        .ok_or_else(|| VerifyError::new("signature is not a byte string"))?;
    let signature: [u8; 64] = signature
        .as_slice()
        .try_into()
        .map_err(|_| VerifyError::new("signature is not 64 bytes"))?;

    Ok(ParsedSign1 {
        protected_bytes,
        kid,
        alg,
        suite_id,
        payload,
        signature,
    })
}

fn verify_signature(item: &ParsedSign1, public_key_bytes: [u8; 32]) -> bool {
    let Some(payload) = &item.payload else {
        return false;
    };
    let signature = Signature::from_bytes(&item.signature);
    let verifying_key = match VerifyingKey::from_bytes(&public_key_bytes) {
        Ok(key) => key,
        Err(_) => return false,
    };
    let sig_structure = sig_structure_bytes(&item.protected_bytes, payload);
    verifying_key.verify(&sig_structure, &signature).is_ok()
}

fn decode_event_details(event: &ParsedSign1) -> Result<EventDetails, VerifyError> {
    let payload_bytes = event
        .payload
        .as_ref()
        .ok_or_else(|| VerifyError::new("detached event payloads are out of scope"))?;
    let payload_value = decode_value(payload_bytes)?;
    let payload_map = payload_value
        .as_map()
        .ok_or_else(|| VerifyError::new("event payload root is not a map"))?;
    let scope = map_lookup_bytes(payload_map, "ledger_scope")?;
    let sequence = map_lookup_u64(payload_map, "sequence")?;
    let prev_hash = match map_lookup_optional_bytes(payload_map, "prev_hash")? {
        Some(bytes) => Some(bytes_array(&bytes)),
        None => None,
    };
    let author_event_hash = bytes_array(&map_lookup_fixed_bytes(
        payload_map,
        "author_event_hash",
        32,
    )?);
    let content_hash = bytes_array(&map_lookup_fixed_bytes(payload_map, "content_hash", 32)?);
    let canonical_event_hash = recompute_canonical_event_hash(&scope, payload_bytes);

    let header = map_lookup_map(payload_map, "header")?;
    let authored_at = map_lookup_u64(header, "authored_at")?;
    let event_type_bytes = map_lookup_bytes(header, "event_type")?;
    let event_type = String::from_utf8(event_type_bytes)
        .map_err(|_| VerifyError::new("header.event_type is not valid UTF-8"))?;
    let classification_bytes = map_lookup_bytes(header, "classification")?;
    let classification = String::from_utf8(classification_bytes)
        .map_err(|_| VerifyError::new("header.classification is not valid UTF-8"))?;

    let payload_ref_map = map_lookup_map(payload_map, "payload_ref")?;
    let payload_ref = match map_lookup_text(payload_ref_map, "ref_type")?.as_str() {
        "inline" => PayloadRef::Inline(map_lookup_bytes(payload_ref_map, "ciphertext")?),
        "external" => PayloadRef::External,
        _ => {
            return Err(VerifyError::new(
                "payload_ref.ref_type is not a supported Phase-1 value",
            ));
        }
    };

    let (transition, attachment_binding) = match map_lookup_optional_map(payload_map, "extensions")?
    {
        Some(extensions) => (
            decode_transition_details(extensions)?,
            decode_attachment_binding_details(extensions)?,
        ),
        None => (None, None),
    };

    Ok(EventDetails {
        scope,
        sequence,
        authored_at,
        event_type,
        classification,
        prev_hash,
        author_event_hash,
        content_hash,
        canonical_event_hash,
        payload_ref,
        transition,
        attachment_binding,
    })
}

fn decode_transition_details(
    extensions: &[(Value, Value)],
) -> Result<Option<TransitionDetails>, VerifyError> {
    let custody = map_lookup_optional_value(extensions, "trellis.custody-model-transition.v1");
    let disclosure = map_lookup_optional_value(extensions, "trellis.disclosure-profile-transition.v1");
    if custody.is_some() && disclosure.is_some() {
        return Err(VerifyError::new(
            "extensions MUST NOT contain both trellis.custody-model-transition.v1 and trellis.disclosure-profile-transition.v1 on the same event",
        ));
    }
    if let Some(extension_value) = custody {
        return Ok(Some(decode_custody_model_transition(extension_value)?));
    }
    if let Some(extension_value) = disclosure {
        return Ok(Some(decode_disclosure_profile_transition(extension_value)?));
    }
    Ok(None)
}

fn decode_custody_model_transition(
    extension_value: &Value,
) -> Result<TransitionDetails, VerifyError> {
    let extension_map = extension_value
        .as_map()
        .ok_or_else(|| VerifyError::new("custody-model transition extension is not a map"))?;
    let transition_id = map_lookup_text(extension_map, "transition_id")?;
    let from_state = map_lookup_text(extension_map, "from_custody_model")?;
    let to_state = map_lookup_text(extension_map, "to_custody_model")?;
    let _effective_at = map_lookup_u64(extension_map, "effective_at")?;
    let declaration_digest = bytes_array(&map_lookup_fixed_bytes(
        extension_map,
        "declaration_doc_digest",
        32,
    )?);
    let attestation_classes = decode_attestation_classes(extension_map)?;

    Ok(TransitionDetails {
        kind: TransitionKind::CustodyModel,
        transition_id,
        from_state,
        to_state,
        declaration_digest,
        attestation_classes,
        scope_change: None,
    })
}

fn decode_disclosure_profile_transition(
    extension_value: &Value,
) -> Result<TransitionDetails, VerifyError> {
    let extension_map = extension_value
        .as_map()
        .ok_or_else(|| VerifyError::new("disclosure-profile transition extension is not a map"))?;
    let transition_id = map_lookup_text(extension_map, "transition_id")?;
    let from_state = map_lookup_text(extension_map, "from_disclosure_profile")?;
    let to_state = map_lookup_text(extension_map, "to_disclosure_profile")?;
    let _effective_at = map_lookup_u64(extension_map, "effective_at")?;
    let declaration_digest = bytes_array(&map_lookup_fixed_bytes(
        extension_map,
        "declaration_doc_digest",
        32,
    )?);
    let scope_change = map_lookup_text(extension_map, "scope_change")?;
    let attestation_classes = decode_attestation_classes(extension_map)?;

    Ok(TransitionDetails {
        kind: TransitionKind::DisclosureProfile,
        transition_id,
        from_state,
        to_state,
        declaration_digest,
        attestation_classes,
        scope_change: Some(scope_change),
    })
}

fn decode_attestation_classes(
    extension_map: &[(Value, Value)],
) -> Result<Vec<String>, VerifyError> {
    let attestations = map_lookup_array(extension_map, "attestations")?;
    Ok(attestations
        .iter()
        .filter_map(|item| item.as_map())
        .filter_map(|map| map_lookup_text(map, "authority_class").ok())
        .collect())
}

fn decode_attachment_binding_details(
    extensions: &[(Value, Value)],
) -> Result<Option<AttachmentBindingDetails>, VerifyError> {
    let Some(extension_value) = map_lookup_optional_value(extensions, ATTACHMENT_EVENT_EXTENSION)
    else {
        return Ok(None);
    };
    let extension_map = extension_value
        .as_map()
        .ok_or_else(|| VerifyError::new("attachment binding extension is not a map"))?;
    let attachment_id = map_lookup_text(extension_map, "attachment_id")?;
    let slot_path = map_lookup_text(extension_map, "slot_path")?;
    let media_type = map_lookup_text(extension_map, "media_type")?;
    let byte_length = map_lookup_u64(extension_map, "byte_length")?;
    let attachment_sha256 =
        parse_sha256_text(&map_lookup_text(extension_map, "attachment_sha256")?)?;
    let payload_content_hash =
        parse_sha256_text(&map_lookup_text(extension_map, "payload_content_hash")?)?;
    let filename = map_lookup_optional_text(extension_map, "filename")?;
    let prior_binding_hash = match map_lookup_optional_value(extension_map, "prior_binding_hash") {
        Some(Value::Text(value)) => Some(parse_sha256_text(value)?),
        Some(Value::Null) | None => None,
        Some(_) => {
            return Err(VerifyError::new(
                "attachment binding prior_binding_hash is neither sha256 text nor null",
            ));
        }
    };

    Ok(Some(AttachmentBindingDetails {
        attachment_id,
        slot_path,
        media_type,
        byte_length,
        attachment_sha256,
        payload_content_hash,
        filename,
        prior_binding_hash,
    }))
}

fn binding_lineage_graph_has_cycle(adj: &BTreeMap<[u8; 32], Vec<[u8; 32]>>) -> bool {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Color {
        White,
        Gray,
        Black,
    }

    let mut nodes: BTreeSet<[u8; 32]> = BTreeSet::new();
    for (from, tos) in adj {
        nodes.insert(*from);
        for t in tos {
            nodes.insert(*t);
        }
    }

    let mut color: BTreeMap<[u8; 32], Color> = BTreeMap::new();
    for node in &nodes {
        color.insert(*node, Color::White);
    }

    fn dfs(
        node: [u8; 32],
        adj: &BTreeMap<[u8; 32], Vec<[u8; 32]>>,
        color: &mut BTreeMap<[u8; 32], Color>,
    ) -> bool {
        use Color::{Black, Gray, White};
        match color.get(&node).copied().unwrap_or(White) {
            Gray => return true,
            Black => return false,
            White => {}
        }
        color.insert(node, Gray);
        if let Some(neighbors) = adj.get(&node) {
            for &next in neighbors {
                if dfs(next, adj, color) {
                    return true;
                }
            }
        }
        color.insert(node, Black);
        false
    }

    for node in nodes {
        if matches!(color.get(&node).copied(), Some(Color::White)) && dfs(node, adj, &mut color) {
            return true;
        }
    }
    false
}

/// ADR 0072 topology: duplicate manifest rows, prior resolution, strict prior-before-binding
/// order in the exported event array, and cycles in the prior-pointer graph.
fn attachment_manifest_topology_failures(
    entries: &[AttachmentManifestEntry],
    hash_to_index: &BTreeMap<[u8; 32], usize>,
) -> Vec<VerificationFailure> {
    let mut failures = Vec::new();

    let mut seen_bindings = BTreeSet::new();
    for entry in entries {
        if !seen_bindings.insert(entry.binding_event_hash) {
            failures.push(VerificationFailure::new(
                "attachment_manifest_duplicate_binding",
                hex_string(&entry.binding_event_hash),
            ));
        }
    }

    let mut adj: BTreeMap<[u8; 32], Vec<[u8; 32]>> = BTreeMap::new();
    for entry in entries {
        let Some(prior_hash) = entry.prior_binding_hash else {
            continue;
        };
        if hash_to_index.contains_key(&entry.binding_event_hash)
            && hash_to_index.contains_key(&prior_hash)
        {
            adj.entry(entry.binding_event_hash)
                .or_default()
                .push(prior_hash);
        }
    }
    if binding_lineage_graph_has_cycle(&adj) {
        failures.push(VerificationFailure::new(
            "attachment_binding_lineage_cycle",
            "061-attachments.cbor",
        ));
    }

    for entry in entries {
        let Some(&current_idx) = hash_to_index.get(&entry.binding_event_hash) else {
            continue;
        };
        let Some(prior_hash) = entry.prior_binding_hash else {
            continue;
        };
        let Some(&prior_idx) = hash_to_index.get(&prior_hash) else {
            failures.push(VerificationFailure::new(
                "attachment_prior_binding_unresolved",
                hex_string(&entry.binding_event_hash),
            ));
            continue;
        };
        if prior_idx >= current_idx {
            failures.push(VerificationFailure::new(
                "attachment_prior_binding_forward_reference",
                hex_string(&entry.binding_event_hash),
            ));
        }
    }

    failures
}

fn parse_attachment_export_extension(
    manifest_map: &[(Value, Value)],
) -> Result<Option<AttachmentExportExtension>, VerifyError> {
    let Some(extensions) = map_lookup_optional_map(manifest_map, "extensions")? else {
        return Ok(None);
    };
    let Some(extension_value) = map_lookup_optional_value(extensions, ATTACHMENT_EXPORT_EXTENSION)
    else {
        return Ok(None);
    };
    let extension_map = extension_value
        .as_map()
        .ok_or_else(|| VerifyError::new("attachment export extension is not a map"))?;
    Ok(Some(AttachmentExportExtension {
        manifest_digest: bytes_array(&map_lookup_fixed_bytes(
            extension_map,
            "attachment_manifest_digest",
            32,
        )?),
        inline_attachments: map_lookup_bool(extension_map, "inline_attachments")?,
    }))
}

fn parse_signature_export_extension(
    manifest_map: &[(Value, Value)],
) -> Result<Option<SignatureExportExtension>, VerifyError> {
    let Some(extensions) = map_lookup_optional_map(manifest_map, "extensions")? else {
        return Ok(None);
    };
    let Some(extension_value) = map_lookup_optional_value(extensions, SIGNATURE_EXPORT_EXTENSION)
    else {
        return Ok(None);
    };
    let extension_map = extension_value
        .as_map()
        .ok_or_else(|| VerifyError::new("signature export extension is not a map"))?;
    Ok(Some(SignatureExportExtension {
        catalog_digest: bytes_array(&map_lookup_fixed_bytes(
            extension_map,
            "signature_catalog_digest",
            32,
        )?),
    }))
}

fn parse_intake_export_extension(
    manifest_map: &[(Value, Value)],
) -> Result<Option<IntakeExportExtension>, VerifyError> {
    let Some(extensions) = map_lookup_optional_map(manifest_map, "extensions")? else {
        return Ok(None);
    };
    let Some(extension_value) = map_lookup_optional_value(extensions, INTAKE_EXPORT_EXTENSION)
    else {
        return Ok(None);
    };
    let extension_map = extension_value
        .as_map()
        .ok_or_else(|| VerifyError::new("intake export extension is not a map"))?;
    Ok(Some(IntakeExportExtension {
        catalog_digest: bytes_array(&map_lookup_fixed_bytes(
            extension_map,
            "intake_catalog_digest",
            32,
        )?),
    }))
}

fn verify_attachment_manifest(
    archive: &ExportArchive,
    events: &[ParsedSign1],
    extension: &AttachmentExportExtension,
    report: &mut VerificationReport,
) {
    let Some(manifest_bytes) = archive.members.get("061-attachments.cbor") else {
        report.event_failures.push(VerificationFailure::new(
            "missing_attachment_manifest",
            "061-attachments.cbor",
        ));
        return;
    };
    let actual_digest = sha256_bytes(manifest_bytes);
    if actual_digest.as_slice() != extension.manifest_digest {
        report.event_failures.push(VerificationFailure::new(
            "attachment_manifest_digest_mismatch",
            "061-attachments.cbor",
        ));
    }

    let entries = match parse_attachment_manifest_entries(manifest_bytes) {
        Ok(entries) => entries,
        Err(error) => {
            report.event_failures.push(VerificationFailure::new(
                "attachment_manifest_invalid",
                format!("061-attachments.cbor/{error}"),
            ));
            return;
        }
    };
    let event_details = events
        .iter()
        .filter_map(|event| decode_event_details(event).ok())
        .collect::<Vec<_>>();

    let mut hash_to_index: BTreeMap<[u8; 32], usize> = BTreeMap::new();
    for (index, event) in events.iter().enumerate() {
        if let Ok(details) = decode_event_details(event) {
            hash_to_index.insert(details.canonical_event_hash, index);
        }
    }
    for failure in attachment_manifest_topology_failures(&entries, &hash_to_index) {
        report.event_failures.push(failure);
    }

    for entry in &entries {
        let matching_events = event_details
            .iter()
            .filter(|details| details.canonical_event_hash == entry.binding_event_hash)
            .collect::<Vec<_>>();
        if matching_events.len() != 1 {
            report.event_failures.push(VerificationFailure::new(
                "attachment_binding_event_unresolved",
                hex_string(&entry.binding_event_hash),
            ));
            continue;
        }
        let details = matching_events[0];
        let Some(binding) = &details.attachment_binding else {
            report.event_failures.push(VerificationFailure::new(
                "attachment_binding_missing",
                hex_string(&entry.binding_event_hash),
            ));
            continue;
        };
        if !attachment_entry_matches_binding(entry, binding) {
            report.event_failures.push(VerificationFailure::new(
                "attachment_binding_mismatch",
                hex_string(&entry.binding_event_hash),
            ));
        }
        if entry.payload_content_hash != details.content_hash
            || binding.payload_content_hash != details.content_hash
        {
            report.event_failures.push(VerificationFailure::new(
                "attachment_payload_hash_mismatch",
                hex_string(&entry.binding_event_hash),
            ));
        }
        if extension.inline_attachments {
            let member = format!(
                "060-payloads/{}.bin",
                hex_string(&entry.payload_content_hash)
            );
            if !archive.members.contains_key(&member) {
                report
                    .event_failures
                    .push(VerificationFailure::new("missing_attachment_body", member));
            }
        }
    }
}

fn verify_signature_catalog(
    archive: &ExportArchive,
    events: &[ParsedSign1],
    payload_blobs: &BTreeMap<[u8; 32], Vec<u8>>,
    extension: &SignatureExportExtension,
    report: &mut VerificationReport,
) {
    let Some(catalog_bytes) = archive.members.get("062-signature-affirmations.cbor") else {
        report.event_failures.push(VerificationFailure::new(
            "missing_signature_catalog",
            "062-signature-affirmations.cbor",
        ));
        return;
    };
    let actual_digest = sha256_bytes(catalog_bytes);
    if actual_digest.as_slice() != extension.catalog_digest {
        report.event_failures.push(VerificationFailure::new(
            "signature_catalog_digest_mismatch",
            "062-signature-affirmations.cbor",
        ));
    }

    let entries = match parse_signature_manifest_entries(catalog_bytes) {
        Ok(entries) => entries,
        Err(error) => {
            report.event_failures.push(VerificationFailure::new(
                "signature_catalog_invalid",
                format!("062-signature-affirmations.cbor/{error}"),
            ));
            return;
        }
    };

    let mut event_by_hash: BTreeMap<[u8; 32], EventDetails> = BTreeMap::new();
    for event in events {
        if let Ok(details) = decode_event_details(event) {
            match event_by_hash.entry(details.canonical_event_hash) {
                Entry::Vacant(slot) => {
                    slot.insert(details);
                }
                Entry::Occupied(_) => {
                    report.event_failures.push(VerificationFailure::new(
                        "export_events_duplicate_canonical_hash",
                        hex_string(&details.canonical_event_hash),
                    ));
                }
            }
        }
    }

    let mut seen_hashes = BTreeSet::new();
    for entry in &entries {
        if !seen_hashes.insert(entry.canonical_event_hash) {
            report.event_failures.push(VerificationFailure::new(
                "signature_catalog_duplicate_event",
                hex_string(&entry.canonical_event_hash),
            ));
        }
    }

    for entry in &entries {
        let Some(details) = event_by_hash.get(&entry.canonical_event_hash) else {
            report.event_failures.push(VerificationFailure::new(
                "signature_catalog_event_unresolved",
                hex_string(&entry.canonical_event_hash),
            ));
            continue;
        };
        if details.event_type != WOS_SIGNATURE_AFFIRMATION_EVENT_TYPE {
            report.event_failures.push(VerificationFailure::new(
                "signature_catalog_event_type_mismatch",
                hex_string(&entry.canonical_event_hash),
            ));
            continue;
        }
        let Some(payload_bytes) = readable_payload_bytes(details, payload_blobs) else {
            report.event_failures.push(VerificationFailure::new(
                "signature_affirmation_payload_unreadable",
                hex_string(&entry.canonical_event_hash),
            ));
            continue;
        };
        let record = match parse_signature_affirmation_record(&payload_bytes) {
            Ok(record) => record,
            Err(error) => {
                report.event_failures.push(VerificationFailure::new(
                    "signature_affirmation_payload_invalid",
                    format!("{}/{}", hex_string(&entry.canonical_event_hash), error),
                ));
                continue;
            }
        };
        if !signature_entry_matches_record(entry, &record) {
            report.event_failures.push(VerificationFailure::new(
                "signature_catalog_mismatch",
                hex_string(&entry.canonical_event_hash),
            ));
        }
    }
}

fn verify_intake_catalog(
    archive: &ExportArchive,
    events: &[ParsedSign1],
    payload_blobs: &BTreeMap<[u8; 32], Vec<u8>>,
    extension: &IntakeExportExtension,
    report: &mut VerificationReport,
) {
    let Some(catalog_bytes) = archive.members.get("063-intake-handoffs.cbor") else {
        report.event_failures.push(VerificationFailure::new(
            "missing_intake_handoff_catalog",
            "063-intake-handoffs.cbor",
        ));
        return;
    };
    let actual_digest = sha256_bytes(catalog_bytes);
    if actual_digest.as_slice() != extension.catalog_digest {
        report.event_failures.push(VerificationFailure::new(
            "intake_handoff_catalog_digest_mismatch",
            "063-intake-handoffs.cbor",
        ));
    }

    let entries = match parse_intake_manifest_entries(catalog_bytes) {
        Ok(entries) => entries,
        Err(error) => {
            report.event_failures.push(VerificationFailure::new(
                "intake_handoff_catalog_invalid",
                format!("063-intake-handoffs.cbor/{error}"),
            ));
            return;
        }
    };

    let mut event_by_hash: BTreeMap<[u8; 32], EventDetails> = BTreeMap::new();
    for event in events {
        if let Ok(details) = decode_event_details(event) {
            match event_by_hash.entry(details.canonical_event_hash) {
                Entry::Vacant(slot) => {
                    slot.insert(details);
                }
                Entry::Occupied(_) => {
                    report.event_failures.push(VerificationFailure::new(
                        "export_events_duplicate_canonical_hash",
                        hex_string(&details.canonical_event_hash),
                    ));
                }
            }
        }
    }

    let mut seen_hashes = BTreeSet::new();
    for entry in &entries {
        if !seen_hashes.insert(entry.intake_event_hash) {
            report.event_failures.push(VerificationFailure::new(
                "intake_handoff_catalog_duplicate_event",
                hex_string(&entry.intake_event_hash),
            ));
        }
    }

    for entry in &entries {
        let Some(details) = event_by_hash.get(&entry.intake_event_hash) else {
            report.event_failures.push(VerificationFailure::new(
                "intake_event_unresolved",
                hex_string(&entry.intake_event_hash),
            ));
            continue;
        };
        if details.event_type != WOS_INTAKE_ACCEPTED_EVENT_TYPE {
            report.event_failures.push(VerificationFailure::new(
                "intake_event_type_mismatch",
                hex_string(&entry.intake_event_hash),
            ));
            continue;
        }
        let Some(payload_bytes) = readable_payload_bytes(details, payload_blobs) else {
            report.event_failures.push(VerificationFailure::new(
                "intake_payload_unreadable",
                hex_string(&entry.intake_event_hash),
            ));
            continue;
        };
        let intake_record = match parse_intake_accepted_record(&payload_bytes) {
            Ok(record) => record,
            Err(error) => {
                report.event_failures.push(VerificationFailure::new(
                    "intake_payload_invalid",
                    format!("{}/{}", hex_string(&entry.intake_event_hash), error),
                ));
                continue;
            }
        };
        if !intake_entry_matches_record(entry, &intake_record) {
            report.event_failures.push(VerificationFailure::new(
                "intake_handoff_mismatch",
                hex_string(&entry.intake_event_hash),
            ));
        }
        match response_hash_matches(&entry.handoff.response_hash, &entry.response_bytes) {
            Ok(true) => {}
            Ok(false) => {
                report.event_failures.push(VerificationFailure::new(
                    "intake_response_hash_mismatch",
                    hex_string(&entry.intake_event_hash),
                ));
            }
            Err(error) => {
                report.event_failures.push(VerificationFailure::new(
                    "intake_handoff_catalog_invalid",
                    format!("{}/{}", hex_string(&entry.intake_event_hash), error),
                ));
            }
        }

        match (
            entry.handoff.initiation_mode.as_str(),
            entry.case_created_event_hash,
        ) {
            ("workflowInitiated", Some(_)) => {
                report.event_failures.push(VerificationFailure::new(
                    "case_created_handoff_mismatch",
                    hex_string(&entry.intake_event_hash),
                ));
                continue;
            }
            ("workflowInitiated", None) => continue,
            ("publicIntake", None) => {
                report.event_failures.push(VerificationFailure::new(
                    "case_created_handoff_mismatch",
                    hex_string(&entry.intake_event_hash),
                ));
                continue;
            }
            ("publicIntake", Some(case_created_hash)) => {
                let Some(case_details) = event_by_hash.get(&case_created_hash) else {
                    report.event_failures.push(VerificationFailure::new(
                        "case_created_event_unresolved",
                        hex_string(&case_created_hash),
                    ));
                    continue;
                };
                if case_details.event_type != WOS_CASE_CREATED_EVENT_TYPE {
                    report.event_failures.push(VerificationFailure::new(
                        "case_created_event_type_mismatch",
                        hex_string(&case_created_hash),
                    ));
                    continue;
                }
                let Some(case_payload_bytes) = readable_payload_bytes(case_details, payload_blobs)
                else {
                    report.event_failures.push(VerificationFailure::new(
                        "case_created_payload_unreadable",
                        hex_string(&case_created_hash),
                    ));
                    continue;
                };
                let case_record = match parse_case_created_record(&case_payload_bytes) {
                    Ok(record) => record,
                    Err(error) => {
                        report.event_failures.push(VerificationFailure::new(
                            "case_created_payload_invalid",
                            format!("{}/{}", hex_string(&case_created_hash), error),
                        ));
                        continue;
                    }
                };
                if !case_created_record_matches_handoff(entry, &intake_record, &case_record) {
                    report.event_failures.push(VerificationFailure::new(
                        "case_created_handoff_mismatch",
                        hex_string(&case_created_hash),
                    ));
                }
            }
            _ => {
                report.event_failures.push(VerificationFailure::new(
                    "intake_handoff_catalog_invalid",
                    format!(
                        "{}/unknown-initiation-mode",
                        hex_string(&entry.intake_event_hash)
                    ),
                ));
            }
        }
    }
}

fn parse_attachment_manifest_entries(
    manifest_bytes: &[u8],
) -> Result<Vec<AttachmentManifestEntry>, VerifyError> {
    let value = decode_value(manifest_bytes)?;
    let entries = value
        .as_array()
        .ok_or_else(|| VerifyError::new("attachment manifest root is not an array"))?;
    entries
        .iter()
        .map(|entry| {
            let map = entry
                .as_map()
                .ok_or_else(|| VerifyError::new("attachment manifest entry is not a map"))?;
            Ok(AttachmentManifestEntry {
                binding_event_hash: bytes_array(&map_lookup_fixed_bytes(
                    map,
                    "binding_event_hash",
                    32,
                )?),
                attachment_id: map_lookup_text(map, "attachment_id")?,
                slot_path: map_lookup_text(map, "slot_path")?,
                media_type: map_lookup_text(map, "media_type")?,
                byte_length: map_lookup_u64(map, "byte_length")?,
                attachment_sha256: bytes_array(&map_lookup_fixed_bytes(
                    map,
                    "attachment_sha256",
                    32,
                )?),
                payload_content_hash: bytes_array(&map_lookup_fixed_bytes(
                    map,
                    "payload_content_hash",
                    32,
                )?),
                filename: map_lookup_optional_text(map, "filename")?,
                prior_binding_hash: map_lookup_optional_fixed_bytes(map, "prior_binding_hash", 32)?
                    .map(|bytes| bytes_array(&bytes)),
            })
        })
        .collect()
}

fn parse_signature_manifest_entries(
    manifest_bytes: &[u8],
) -> Result<Vec<SignatureManifestEntry>, VerifyError> {
    let value = decode_value(manifest_bytes)?;
    let entries = value
        .as_array()
        .ok_or_else(|| VerifyError::new("signature catalog root is not an array"))?;
    entries
        .iter()
        .map(|entry| {
            let map = entry
                .as_map()
                .ok_or_else(|| VerifyError::new("signature catalog entry is not a map"))?;
            Ok(SignatureManifestEntry {
                canonical_event_hash: bytes_array(&map_lookup_fixed_bytes(
                    map,
                    "canonical_event_hash",
                    32,
                )?),
                signer_id: map_lookup_text(map, "signer_id")?,
                role_id: map_lookup_text(map, "role_id")?,
                role: map_lookup_text(map, "role")?,
                document_id: map_lookup_text(map, "document_id")?,
                document_hash: map_lookup_text(map, "document_hash")?,
                document_hash_algorithm: map_lookup_text(map, "document_hash_algorithm")?,
                signed_at: map_lookup_text(map, "signed_at")?,
                identity_binding: map_lookup_value_clone(map, "identity_binding")?,
                consent_reference: map_lookup_value_clone(map, "consent_reference")?,
                signature_provider: map_lookup_text(map, "signature_provider")?,
                ceremony_id: map_lookup_text(map, "ceremony_id")?,
                profile_ref: map_lookup_optional_text(map, "profile_ref")?,
                profile_key: map_lookup_optional_text(map, "profile_key")?,
                formspec_response_ref: map_lookup_text(map, "formspec_response_ref")?,
            })
        })
        .collect()
}

fn parse_intake_manifest_entries(
    manifest_bytes: &[u8],
) -> Result<Vec<IntakeManifestEntry>, VerifyError> {
    let value = decode_value(manifest_bytes)?;
    let entries = value
        .as_array()
        .ok_or_else(|| VerifyError::new("intake handoff catalog root is not an array"))?;
    entries
        .iter()
        .map(|entry| {
            let map = entry
                .as_map()
                .ok_or_else(|| VerifyError::new("intake handoff catalog entry is not a map"))?;
            let handoff = parse_intake_handoff_details(
                map_lookup_optional_value(map, "handoff")
                    .ok_or_else(|| VerifyError::new("missing `handoff`"))?,
            )?;
            Ok(IntakeManifestEntry {
                intake_event_hash: bytes_array(&map_lookup_fixed_bytes(
                    map,
                    "intake_event_hash",
                    32,
                )?),
                case_created_event_hash: map_lookup_optional_fixed_bytes(
                    map,
                    "case_created_event_hash",
                    32,
                )?
                .map(|bytes| bytes_array(&bytes)),
                handoff,
                response_bytes: map_lookup_bytes(map, "response_bytes")?,
            })
        })
        .collect()
}

fn readable_payload_bytes(
    details: &EventDetails,
    payload_blobs: &BTreeMap<[u8; 32], Vec<u8>>,
) -> Option<Vec<u8>> {
    match &details.payload_ref {
        PayloadRef::Inline(bytes) => Some(bytes.clone()),
        PayloadRef::External => payload_blobs.get(&details.content_hash).cloned(),
    }
}

fn parse_signature_affirmation_record(
    payload_bytes: &[u8],
) -> Result<SignatureAffirmationRecordDetails, VerifyError> {
    let value = decode_value(payload_bytes)?;
    let map = value
        .as_map()
        .ok_or_else(|| VerifyError::new("signature affirmation payload root is not a map"))?;
    let record_kind = map_lookup_text(map, "recordKind")?;
    if record_kind != "signatureAffirmation" {
        return Err(VerifyError::new(
            "signature affirmation payload recordKind is not signatureAffirmation",
        ));
    }
    let data = map_lookup_map(map, "data")?;
    Ok(SignatureAffirmationRecordDetails {
        signer_id: map_lookup_text(data, "signerId")?,
        role_id: map_lookup_text(data, "roleId")?,
        role: map_lookup_text(data, "role")?,
        document_id: map_lookup_text(data, "documentId")?,
        document_hash: map_lookup_text(data, "documentHash")?,
        document_hash_algorithm: map_lookup_text(data, "documentHashAlgorithm")?,
        signed_at: map_lookup_text(data, "signedAt")?,
        identity_binding: map_lookup_value_clone(data, "identityBinding")?,
        consent_reference: map_lookup_value_clone(data, "consentReference")?,
        signature_provider: map_lookup_text(data, "signatureProvider")?,
        ceremony_id: map_lookup_text(data, "ceremonyId")?,
        profile_ref: map_lookup_optional_text(data, "profileRef")?,
        profile_key: map_lookup_optional_text(data, "profileKey")?,
        formspec_response_ref: map_lookup_text(data, "formspecResponseRef")?,
    })
}

fn parse_intake_accepted_record(
    payload_bytes: &[u8],
) -> Result<IntakeAcceptedRecordDetails, VerifyError> {
    let value = decode_value(payload_bytes)?;
    let map = value
        .as_map()
        .ok_or_else(|| VerifyError::new("intake accepted payload root is not a map"))?;
    let record_kind = map_lookup_text(map, "recordKind")?;
    if record_kind != "intakeAccepted" {
        return Err(VerifyError::new(
            "intake accepted payload recordKind is not intakeAccepted",
        ));
    }
    let data = map_lookup_map(map, "data")?;
    let case_ref = map_lookup_text(data, "caseRef")?;
    let outputs = map_lookup_array(map, "outputs")?;
    let Some(output_case_ref) = first_array_text(outputs) else {
        return Err(VerifyError::new(
            "intake accepted outputs array is missing or empty",
        ));
    };
    if output_case_ref != case_ref {
        return Err(VerifyError::new(
            "intake accepted outputs[0] does not match data.caseRef",
        ));
    }
    Ok(IntakeAcceptedRecordDetails {
        intake_id: map_lookup_text(data, "intakeId")?,
        case_intent: map_lookup_text(data, "caseIntent")?,
        case_disposition: map_lookup_text(data, "caseDisposition")?,
        case_ref,
        definition_url: map_lookup_optional_text(data, "definitionUrl")?,
        definition_version: map_lookup_optional_text(data, "definitionVersion")?,
    })
}

fn parse_case_created_record(
    payload_bytes: &[u8],
) -> Result<CaseCreatedRecordDetails, VerifyError> {
    let value = decode_value(payload_bytes)?;
    let map = value
        .as_map()
        .ok_or_else(|| VerifyError::new("case created payload root is not a map"))?;
    let record_kind = map_lookup_text(map, "recordKind")?;
    if record_kind != "caseCreated" {
        return Err(VerifyError::new(
            "case created payload recordKind is not caseCreated",
        ));
    }
    let data = map_lookup_map(map, "data")?;
    let case_ref = map_lookup_text(data, "caseRef")?;
    let outputs = map_lookup_array(map, "outputs")?;
    let Some(output_case_ref) = first_array_text(outputs) else {
        return Err(VerifyError::new(
            "case created outputs array is missing or empty",
        ));
    };
    if output_case_ref != case_ref {
        return Err(VerifyError::new(
            "case created outputs[0] does not match data.caseRef",
        ));
    }
    Ok(CaseCreatedRecordDetails {
        case_ref,
        intake_handoff_ref: map_lookup_text(data, "intakeHandoffRef")?,
        formspec_response_ref: map_lookup_text(data, "formspecResponseRef")?,
        validation_report_ref: map_lookup_text(data, "validationReportRef")?,
        ledger_head_ref: map_lookup_text(data, "ledgerHeadRef")?,
        initiation_mode: map_lookup_text(data, "initiationMode")?,
    })
}

fn parse_intake_handoff_details(value: &Value) -> Result<IntakeHandoffDetails, VerifyError> {
    let map = value
        .as_map()
        .ok_or_else(|| VerifyError::new("handoff is not a map"))?;
    let initiation_mode = map_lookup_text(map, "initiationMode")?;
    let case_ref = map_lookup_optional_text(map, "caseRef")?;
    match initiation_mode.as_str() {
        "workflowInitiated" if case_ref.is_none() => {
            return Err(VerifyError::new(
                "workflowInitiated handoff is missing caseRef",
            ));
        }
        "publicIntake" if case_ref.is_some() => {
            return Err(VerifyError::new(
                "publicIntake handoff caseRef must be null or absent",
            ));
        }
        "workflowInitiated" | "publicIntake" => {}
        _ => return Err(VerifyError::new("handoff initiationMode is unsupported")),
    }
    let definition_ref = map_lookup_map(map, "definitionRef")?;
    let response_hash = map_lookup_text(map, "responseHash")?;
    parse_sha256_text(&response_hash)?;
    Ok(IntakeHandoffDetails {
        handoff_id: map_lookup_text(map, "handoffId")?,
        initiation_mode,
        case_ref,
        definition_url: map_lookup_text(definition_ref, "url")?,
        definition_version: map_lookup_text(definition_ref, "version")?,
        response_ref: map_lookup_text(map, "responseRef")?,
        response_hash,
        validation_report_ref: map_lookup_text(map, "validationReportRef")?,
        ledger_head_ref: map_lookup_text(map, "ledgerHeadRef")?,
    })
}

fn attachment_entry_matches_binding(
    entry: &AttachmentManifestEntry,
    binding: &AttachmentBindingDetails,
) -> bool {
    entry.attachment_id == binding.attachment_id
        && entry.slot_path == binding.slot_path
        && entry.media_type == binding.media_type
        && entry.byte_length == binding.byte_length
        && entry.attachment_sha256 == binding.attachment_sha256
        && entry.payload_content_hash == binding.payload_content_hash
        && entry.filename == binding.filename
        && entry.prior_binding_hash == binding.prior_binding_hash
}

/// RFC 8949 §4.2.2 map key ordering: sort keys by the bytewise lexicographic order
/// of their encoded CBOR form. Used only for semantic equality of nested maps.
fn cbor_map_key_sort_bytes(key: &Value) -> Vec<u8> {
    let mut buf = Vec::new();
    ciborium::into_writer(key, &mut buf).expect("cbor map key encode");
    buf
}

/// Recursively re-encode CBOR maps with canonically sorted keys so two values
/// that differ only in map entry order compare equal.
fn normalize_cbor_value_for_compare(value: &Value) -> Value {
    match value {
        Value::Map(pairs) => {
            let mut normalized: Vec<(Value, Value)> = pairs
                .iter()
                .map(|(k, v)| {
                    (
                        normalize_cbor_value_for_compare(k),
                        normalize_cbor_value_for_compare(v),
                    )
                })
                .collect();
            normalized
                .sort_by(|a, b| cbor_map_key_sort_bytes(&a.0).cmp(&cbor_map_key_sort_bytes(&b.0)));
            Value::Map(normalized)
        }
        Value::Array(items) => {
            Value::Array(items.iter().map(normalize_cbor_value_for_compare).collect())
        }
        Value::Tag(tag, inner) => {
            Value::Tag(*tag, Box::new(normalize_cbor_value_for_compare(inner)))
        }
        _ => value.clone(),
    }
}

fn cbor_nested_map_semantic_eq(a: &Value, b: &Value) -> bool {
    normalize_cbor_value_for_compare(a) == normalize_cbor_value_for_compare(b)
}

fn signature_entry_matches_record(
    entry: &SignatureManifestEntry,
    record: &SignatureAffirmationRecordDetails,
) -> bool {
    entry.signer_id == record.signer_id
        && entry.role_id == record.role_id
        && entry.role == record.role
        && entry.document_id == record.document_id
        && entry.document_hash == record.document_hash
        && entry.document_hash_algorithm == record.document_hash_algorithm
        && entry.signed_at == record.signed_at
        && cbor_nested_map_semantic_eq(&entry.identity_binding, &record.identity_binding)
        && cbor_nested_map_semantic_eq(&entry.consent_reference, &record.consent_reference)
        && entry.signature_provider == record.signature_provider
        && entry.ceremony_id == record.ceremony_id
        && entry.profile_ref == record.profile_ref
        && entry.profile_key == record.profile_key
        && entry.formspec_response_ref == record.formspec_response_ref
}

fn intake_entry_matches_record(
    entry: &IntakeManifestEntry,
    record: &IntakeAcceptedRecordDetails,
) -> bool {
    if entry.handoff.handoff_id != record.intake_id {
        return false;
    }

    match entry.handoff.initiation_mode.as_str() {
        "workflowInitiated" => {
            entry.handoff.case_ref.as_deref() == Some(record.case_ref.as_str())
                && record.case_intent == "attachToExistingCase"
                && record.case_disposition == "attachToExistingCase"
        }
        "publicIntake" => {
            record.case_intent == "requestGovernedCaseCreation"
                && record.case_disposition == "createGovernedCase"
                && record.definition_url.as_deref() == Some(entry.handoff.definition_url.as_str())
                && record.definition_version.as_deref()
                    == Some(entry.handoff.definition_version.as_str())
        }
        _ => false,
    }
}

fn case_created_record_matches_handoff(
    entry: &IntakeManifestEntry,
    intake_record: &IntakeAcceptedRecordDetails,
    case_record: &CaseCreatedRecordDetails,
) -> bool {
    case_record.case_ref == intake_record.case_ref
        && case_record.intake_handoff_ref == entry.handoff.handoff_id
        && case_record.formspec_response_ref == entry.handoff.response_ref
        && case_record.validation_report_ref == entry.handoff.validation_report_ref
        && case_record.ledger_head_ref == entry.handoff.ledger_head_ref
        && case_record.initiation_mode == entry.handoff.initiation_mode
}

/// Reserved non-signing class literals from Core §8.7 (ADR 0006).
const RESERVED_NON_SIGNING_KIND: &[&str] = &["tenant-root", "scope", "subject", "recovery"];

/// Parses the unified key registry per Core §8 (ADR 0006).
///
/// Verifier dispatch follows Core §8.7.3 step 1: an entry whose top-level map
/// carries a `kind` field is `KeyEntry` (§8.7.1); an entry without `kind` is
/// the legacy `SigningKeyEntry` flat shape (§8.2). Both paths populate the
/// signing-key map identically for `kind = "signing"` and the legacy shape.
///
/// Reserved non-signing classes (`tenant-root`, `scope`, `subject`,
/// `recovery`) and unknown extension `tstr` kinds are NOT inserted into the
/// signing-key map — they cannot resolve a COSE_Sign1 protected-header `kid`.
/// They are returned in `non_signing` so the caller can emit
/// `key_class_mismatch` (Core §8.7.3 step 4) when an event tries to sign under
/// such a kid, distinct from the generic `unresolvable_manifest_kid` failure.
///
/// Per Core §8.7.6 the wire string `"wrap"` is a deprecated synonym for
/// `"subject"`; this parser normalizes the stored class label so callers see
/// only the canonical taxonomy.
#[cfg(test)]
fn parse_signing_key_registry(
    bytes: &[u8],
) -> Result<BTreeMap<Vec<u8>, SigningKeyEntry>, VerifyError> {
    let (signing, _non_signing) = parse_key_registry(bytes)?;
    Ok(signing)
}

fn parse_key_registry(
    bytes: &[u8],
) -> Result<
    (
        BTreeMap<Vec<u8>, SigningKeyEntry>,
        BTreeMap<Vec<u8>, NonSigningKeyEntry>,
    ),
    VerifyError,
> {
    let value = decode_value(bytes)?;
    let entries = value
        .as_array()
        .ok_or_else(|| VerifyError::new("signing-key registry root is not an array"))?;
    let mut registry = BTreeMap::new();
    let mut non_signing = BTreeMap::new();
    for entry in entries {
        let map = entry
            .as_map()
            .ok_or_else(|| VerifyError::new("signing-key registry entry is not a map"))?;

        // Core §8.7.3 step 1: dispatch on presence of the top-level `kind`
        // field. Absent → legacy `SigningKeyEntry` (§8.2); present →
        // `KeyEntry` (§8.7.1) with `kind` discriminating the arm.
        let kind = map_lookup_optional_text(map, "kind")?;
        let kind_norm = kind.as_deref().map(|s| match s {
            // Core §8.7.6: `"wrap"` is a deprecated synonym for `"subject"`.
            "wrap" => "subject",
            other => other,
        });

        let kid = map_lookup_bytes(map, "kid")?;

        match kind_norm {
            // Legacy `SigningKeyEntry` (no `kind` field) OR new `KeyEntrySigning`.
            None | Some("signing") => {
                let pubkey = bytes_array(&map_lookup_fixed_bytes(map, "pubkey", 32)?);
                let status = map_lookup_u64(map, "status")?;
                let valid_to = match map_lookup_optional_value(map, "valid_to") {
                    Some(Value::Integer(value)) => {
                        Some(u64::try_from(*value).map_err(|_| {
                            VerifyError::new("signing-key registry valid_to is out of range")
                        })?)
                    }
                    Some(Value::Null) | None => None,
                    Some(_) => {
                        return Err(VerifyError::new(
                            "signing-key registry valid_to is neither uint nor null",
                        ));
                    }
                };
                registry.insert(
                    kid,
                    SigningKeyEntry {
                        public_key: pubkey,
                        status,
                        valid_to,
                    },
                );
            }
            // Core §8.7.3 step 3: reserved non-signing class. Phase-1
            // verifier does not validate class-specific inner fields (those
            // slots are envelope reservations and the deep validation rides
            // Phase-2+ activation per ADR 0006), but it DOES enforce the
            // structural-shape gate of §8.7.1: the entry MUST carry an
            // `attributes` map. Absent or wrong-typed `attributes` → fail
            // with `key_entry_attributes_shape_mismatch` (TR-CORE-048).
            Some(class) if RESERVED_NON_SIGNING_KIND.contains(&class) => {
                let attributes = map_lookup_optional_value(map, "attributes");
                match attributes {
                    Some(Value::Map(_)) => {}
                    _ => {
                        return Err(VerifyError::new(format!(
                            "key_entry_attributes_shape_mismatch: KeyEntry of                              kind=\"{class}\" missing required `attributes` map (Core §8.7.1)"
                        )));
                    }
                }
                non_signing.insert(
                    kid,
                    NonSigningKeyEntry {
                        class: class.to_string(),
                    },
                );
            }
            // Core §8.7.3 step 4 *Unknown `kind`*: forward-compatibility
            // floor. The entry is admitted at the wire layer; downstream
            // resolution failures (signature attempt under this kid) surface
            // as a capability gap rather than a structure failure here.
            Some(other) => {
                non_signing.insert(
                    kid,
                    NonSigningKeyEntry {
                        class: other.to_string(),
                    },
                );
            }
        }
    }
    Ok((registry, non_signing))
}

fn parse_bound_registry(bytes: &[u8]) -> Result<BoundRegistry, VerifyError> {
    let value = decode_value(bytes)?;
    let map = value
        .as_map()
        .ok_or_else(|| VerifyError::new("bound registry root is not a map"))?;
    let event_types_map = map_lookup_map(map, "event_types")?;
    let mut event_types = Vec::new();
    for (key, _) in event_types_map {
        let name = key
            .as_text()
            .ok_or_else(|| VerifyError::new("event_types key is not text"))?;
        event_types.push(name.to_string());
    }
    let classifications_values = map_lookup_array(map, "classifications")?;
    let classifications = classifications_values
        .iter()
        .map(|value| {
            value
                .as_text()
                .map(|text| text.to_string())
                .ok_or_else(|| VerifyError::new("classification entry is not text"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(BoundRegistry {
        event_types,
        classifications,
    })
}

fn parse_custody_model(bytes: &[u8]) -> Result<String, VerifyError> {
    let value = decode_value(bytes)?;
    let map = value
        .as_map()
        .ok_or_else(|| VerifyError::new("posture declaration root is not a map"))?;
    let custody_model = map_lookup_map(map, "custody_model")?;
    map_lookup_text(custody_model, "custody_model_id")
}

fn parse_disclosure_profile(bytes: &[u8]) -> Result<String, VerifyError> {
    let value = decode_value(bytes)?;
    let map = value
        .as_map()
        .ok_or_else(|| VerifyError::new("posture declaration root is not a map"))?;
    map_lookup_text(map, "disclosure_profile")
}

fn event_identity(event: &ParsedSign1) -> Result<(Vec<u8>, [u8; 32]), VerifyError> {
    let details = decode_event_details(event)?;
    Ok((details.scope, details.canonical_event_hash))
}

fn recompute_author_event_hash(canonical_event_bytes: &[u8]) -> Option<[u8; 32]> {
    let authored = authored_preimage_from_canonical(canonical_event_bytes)?;
    Some(domain_separated_sha256(AUTHOR_EVENT_DOMAIN, &authored))
}

/// Recovers authored-event CBOR by stripping the `author_event_hash` entry
/// from the canonical map.
///
/// **Coupling:** The `canonical_event_from_authored` helper in `trellis-cddl`
/// always appends `author_event_hash` as the **last** map field with canonical
/// key encoding. If the CDDL map gains trailing fields or reorders keys, this
/// locator must be updated alongside that helper.
fn authored_preimage_from_canonical(canonical_event_bytes: &[u8]) -> Option<Vec<u8>> {
    let key = encode_tstr("author_event_hash");
    let key_position = canonical_event_bytes
        .windows(key.len())
        .rposition(|window| window == key.as_slice())?;
    let value_position = key_position + key.len();
    if canonical_event_bytes.len() != value_position + 34 {
        return None;
    }
    if canonical_event_bytes[value_position] != 0x58
        || canonical_event_bytes[value_position + 1] != 0x20
    {
        return None;
    }
    let mut authored = Vec::with_capacity(canonical_event_bytes.len() - 35);
    let new_map_prefix = canonical_event_bytes.first()?.checked_sub(1)?;
    authored.push(new_map_prefix);
    authored.extend_from_slice(&canonical_event_bytes[1..key_position]);
    Some(authored)
}

fn recompute_canonical_event_hash(scope: &[u8], canonical_event_bytes: &[u8]) -> [u8; 32] {
    let mut preimage = Vec::new();
    preimage.push(0xa3);
    preimage.extend_from_slice(&encode_tstr("version"));
    preimage.extend_from_slice(&encode_uint(1));
    preimage.extend_from_slice(&encode_tstr("ledger_scope"));
    preimage.extend_from_slice(&encode_bstr(scope));
    preimage.extend_from_slice(&encode_tstr("event_payload"));
    preimage.extend_from_slice(canonical_event_bytes);
    domain_separated_sha256(EVENT_DOMAIN, &preimage)
}

fn checkpoint_digest(scope: &[u8], payload_bytes: &[u8]) -> [u8; 32] {
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

fn merkle_leaf_hash(canonical_hash: [u8; 32]) -> [u8; 32] {
    domain_separated_sha256(MERKLE_LEAF_DOMAIN, &canonical_hash)
}

fn merkle_interior_hash(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
    let mut joined = Vec::with_capacity(64);
    joined.extend_from_slice(&left);
    joined.extend_from_slice(&right);
    domain_separated_sha256(MERKLE_INTERIOR_DOMAIN, &joined)
}

fn merkle_root(leaves: &[[u8; 32]]) -> [u8; 32] {
    match leaves.len() {
        // Unreachable for valid checkpoints (`tree_size == 0` is rejected
        // earlier); kept as a defensive sentinel.
        0 => [0u8; 32],
        1 => leaves[0],
        _ => {
            let mut level = leaves.to_vec();
            while level.len() > 1 {
                let mut next = Vec::new();
                let mut index = 0;
                while index < level.len() {
                    if index + 1 == level.len() {
                        // RFC 6962 §2.1: unpaired end leaf is promoted without hashing
                        // with a duplicate of itself.
                        next.push(level[index]);
                    } else {
                        next.push(merkle_interior_hash(level[index], level[index + 1]));
                    }
                    index += 2;
                }
                level = next;
            }
            level[0]
        }
    }
}

fn digest_path_from_values(nodes: &[Value]) -> Result<Vec<[u8; 32]>, ()> {
    let mut out = Vec::with_capacity(nodes.len());
    for node in nodes {
        let bytes = node.as_bytes().ok_or(())?;
        let array: [u8; 32] = bytes.as_slice().try_into().map_err(|_| ())?;
        out.push(array);
    }
    Ok(out)
}

fn inner_proof_size(index: u64, size: u64) -> usize {
    let xor = index ^ (size - 1);
    if xor == 0 {
        0
    } else {
        (u64::BITS - xor.leading_zeros()) as usize
    }
}

fn decomp_inclusion_proof(index: u64, size: u64) -> (usize, usize) {
    let inner = inner_proof_size(index, size);
    let border = (index >> inner).count_ones() as usize;
    (inner, border)
}

fn chain_inner_merkle(mut seed: [u8; 32], proof: &[[u8; 32]], index: u64) -> [u8; 32] {
    for (i, sibling) in proof.iter().enumerate() {
        if (index >> i) & 1 == 0 {
            seed = merkle_interior_hash(seed, *sibling);
        } else {
            seed = merkle_interior_hash(*sibling, seed);
        }
    }
    seed
}

fn chain_inner_right_merkle(mut seed: [u8; 32], proof: &[[u8; 32]], index: u64) -> [u8; 32] {
    for (i, sibling) in proof.iter().enumerate() {
        if (index >> i) & 1 == 1 {
            seed = merkle_interior_hash(*sibling, seed);
        }
    }
    seed
}

fn chain_border_right_merkle(mut seed: [u8; 32], proof: &[[u8; 32]]) -> [u8; 32] {
    for sibling in proof {
        seed = merkle_interior_hash(*sibling, seed);
    }
    seed
}

fn root_from_inclusion_proof(
    leaf_index: u64,
    tree_size: u64,
    leaf_hash: [u8; 32],
    proof: &[[u8; 32]],
) -> Result<[u8; 32], ()> {
    if tree_size == 0 || leaf_index >= tree_size {
        return Err(());
    }
    let (inner, border) = decomp_inclusion_proof(leaf_index, tree_size);
    if proof.len() != inner + border {
        return Err(());
    }
    let mut node = chain_inner_merkle(leaf_hash, &proof[..inner], leaf_index);
    node = chain_border_right_merkle(node, &proof[inner..]);
    Ok(node)
}

fn root_from_consistency_proof(
    size1: u64,
    size2: u64,
    root1: [u8; 32],
    proof: &[[u8; 32]],
) -> Result<[u8; 32], ()> {
    if size2 < size1 {
        return Err(());
    }
    if size1 == size2 {
        if !proof.is_empty() {
            return Err(());
        }
        return Ok(root1);
    }
    if size1 == 0 {
        return Err(());
    }
    if proof.is_empty() {
        return Err(());
    }
    let (mut inner, border) = decomp_inclusion_proof(size1 - 1, size2);
    let shift = size1.trailing_zeros() as usize;
    if inner < shift {
        return Err(());
    }
    inner -= shift;
    let mut seed = proof[0];
    let mut start = 1usize;
    if size1 == 1u64 << shift {
        seed = root1;
        start = 0;
    }
    if proof.len() != start + inner + border {
        return Err(());
    }
    let suffix = &proof[start..];
    let mask = (size1 - 1) >> shift;
    let hash1 = chain_inner_right_merkle(seed, &suffix[..inner], mask);
    let hash1 = chain_border_right_merkle(hash1, &suffix[inner..]);
    if hash1 != root1 {
        return Err(());
    }
    let hash2 = chain_inner_merkle(seed, &suffix[..inner], mask);
    Ok(chain_border_right_merkle(hash2, &suffix[inner..]))
}

fn requires_dual_attestation(from_state: &str, to_state: &str) -> bool {
    custody_rank(to_state) > custody_rank(from_state)
}

fn custody_rank(value: &str) -> i32 {
    match value {
        "CM-A" => 3,
        "CM-B" => 2,
        "CM-C" => 1,
        _ => 0,
    }
}

fn sha256_bytes(bytes: &[u8]) -> Vec<u8> {
    Sha256::digest(bytes).to_vec()
}

fn hex_string(bytes: &[u8]) -> String {
    let mut text = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        use std::fmt::Write as _;
        let _ = write!(text, "{byte:02x}");
    }
    text
}

fn hex_decode(value: &str) -> Result<Vec<u8>, VerifyError> {
    if value.len() % 2 != 0 {
        return Err(VerifyError::new("hex string must have even length"));
    }
    let mut out = Vec::with_capacity(value.len() / 2);
    for chunk in value.as_bytes().chunks_exact(2) {
        let high = hex_nibble(chunk[0])?;
        let low = hex_nibble(chunk[1])?;
        out.push((high << 4) | low);
    }
    Ok(out)
}

fn hex_nibble(value: u8) -> Result<u8, VerifyError> {
    match value {
        b'0'..=b'9' => Ok(value - b'0'),
        b'a'..=b'f' => Ok(value - b'a' + 10),
        b'A'..=b'F' => Ok(value - b'A' + 10),
        _ => Err(VerifyError::new("hex string contains a non-hex digit")),
    }
}

fn parse_sha256_text(value: &str) -> Result<[u8; 32], VerifyError> {
    let Some(hex) = value.strip_prefix("sha256:") else {
        return Err(VerifyError::new("hash text must use sha256: prefix"));
    };
    let bytes = hex_decode(hex)?;
    bytes
        .as_slice()
        .try_into()
        .map_err(|_| VerifyError::new("sha256 hash text must be 32 bytes"))
}

fn response_hash_matches(value: &str, response_bytes: &[u8]) -> Result<bool, VerifyError> {
    Ok(parse_sha256_text(value)? == bytes_array(&sha256_bytes(response_bytes)))
}

fn bytes_array(bytes: &[u8]) -> [u8; 32] {
    bytes.try_into().expect("caller validates fixed size")
}

fn decode_value(bytes: &[u8]) -> Result<Value, VerifyError> {
    ciborium::from_reader(bytes)
        .map_err(|error| VerifyError::new(format!("failed to decode CBOR: {error}")))
}

fn map_lookup_bytes(map: &[(Value, Value)], key_name: &str) -> Result<Vec<u8>, VerifyError> {
    map_lookup_optional_value(map, key_name)
        .and_then(|value| value.as_bytes().cloned())
        .ok_or_else(|| VerifyError::new(format!("missing or invalid `{key_name}` byte string")))
}

fn map_lookup_fixed_bytes(
    map: &[(Value, Value)],
    key_name: &str,
    expected_len: usize,
) -> Result<Vec<u8>, VerifyError> {
    let bytes = map_lookup_bytes(map, key_name)?;
    if bytes.len() != expected_len {
        return Err(VerifyError::new(format!(
            "`{key_name}` must be {expected_len} bytes"
        )));
    }
    Ok(bytes)
}

fn map_lookup_optional_bytes(
    map: &[(Value, Value)],
    key_name: &str,
) -> Result<Option<Vec<u8>>, VerifyError> {
    match map_lookup_optional_value(map, key_name) {
        Some(Value::Bytes(bytes)) => Ok(Some(bytes.clone())),
        Some(Value::Null) => Ok(None),
        None => Ok(None),
        Some(_) => Err(VerifyError::new(format!(
            "`{key_name}` is neither bytes nor null"
        ))),
    }
}

fn map_lookup_optional_fixed_bytes(
    map: &[(Value, Value)],
    key_name: &str,
    expected_len: usize,
) -> Result<Option<Vec<u8>>, VerifyError> {
    match map_lookup_optional_bytes(map, key_name)? {
        Some(bytes) if bytes.len() == expected_len => Ok(Some(bytes)),
        Some(_) => Err(VerifyError::new(format!(
            "`{key_name}` must be {expected_len} bytes"
        ))),
        None => Ok(None),
    }
}

fn map_lookup_u64(map: &[(Value, Value)], key_name: &str) -> Result<u64, VerifyError> {
    let value = map_lookup_optional_value(map, key_name)
        .ok_or_else(|| VerifyError::new(format!("missing `{key_name}`")))?;
    value
        .as_integer()
        .and_then(|integer| integer.try_into().ok())
        .ok_or_else(|| VerifyError::new(format!("`{key_name}` is not an unsigned integer")))
}

fn map_lookup_bool(map: &[(Value, Value)], key_name: &str) -> Result<bool, VerifyError> {
    map_lookup_optional_value(map, key_name)
        .and_then(Value::as_bool)
        .ok_or_else(|| VerifyError::new(format!("missing or invalid `{key_name}` bool")))
}

fn map_lookup_text(map: &[(Value, Value)], key_name: &str) -> Result<String, VerifyError> {
    map_lookup_optional_value(map, key_name)
        .and_then(|value| value.as_text().map(ToOwned::to_owned))
        .ok_or_else(|| VerifyError::new(format!("missing or invalid `{key_name}` text")))
}

fn map_lookup_optional_text(
    map: &[(Value, Value)],
    key_name: &str,
) -> Result<Option<String>, VerifyError> {
    match map_lookup_optional_value(map, key_name) {
        Some(Value::Text(value)) => Ok(Some(value.clone())),
        Some(Value::Null) | None => Ok(None),
        Some(_) => Err(VerifyError::new(format!(
            "`{key_name}` is neither text nor null"
        ))),
    }
}

fn map_lookup_map<'a>(
    map: &'a [(Value, Value)],
    key_name: &str,
) -> Result<&'a [(Value, Value)], VerifyError> {
    map_lookup_optional_value(map, key_name)
        .and_then(Value::as_map)
        .map(Vec::as_slice)
        .ok_or_else(|| VerifyError::new(format!("missing or invalid `{key_name}` map")))
}

fn map_lookup_optional_map<'a>(
    map: &'a [(Value, Value)],
    key_name: &str,
) -> Result<Option<&'a [(Value, Value)]>, VerifyError> {
    match map_lookup_optional_value(map, key_name) {
        Some(Value::Null) | None => Ok(None),
        Some(value) => value
            .as_map()
            .map(Vec::as_slice)
            .map(Some)
            .ok_or_else(|| VerifyError::new(format!("`{key_name}` is not a map"))),
    }
}

fn map_lookup_array<'a>(
    map: &'a [(Value, Value)],
    key_name: &str,
) -> Result<&'a [Value], VerifyError> {
    map_lookup_optional_value(map, key_name)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| VerifyError::new(format!("missing or invalid `{key_name}` array")))
}

fn first_array_text(values: &[Value]) -> Option<String> {
    values
        .first()
        .and_then(Value::as_text)
        .map(ToOwned::to_owned)
}

fn map_lookup_integer_label_bytes(
    map: &[(Value, Value)],
    label: i128,
) -> Result<Vec<u8>, VerifyError> {
    map.iter()
        .find(|(key, _)| {
            key.as_integer()
                .is_some_and(|value| i128::from(value) == label)
        })
        .and_then(|(_, value)| value.as_bytes().cloned())
        .ok_or_else(|| VerifyError::new(format!("missing COSE label {label} bytes")))
}

fn map_lookup_integer_label(map: &[(Value, Value)], label: i128) -> Result<i128, VerifyError> {
    map.iter()
        .find(|(key, _)| {
            key.as_integer()
                .is_some_and(|value| i128::from(value) == label)
        })
        .and_then(|(_, value)| value.as_integer())
        .map(i128::from)
        .ok_or_else(|| VerifyError::new(format!("missing COSE label {label} integer")))
}

fn map_lookup_optional_value<'a>(map: &'a [(Value, Value)], key_name: &str) -> Option<&'a Value> {
    map.iter()
        .find(|(key, _)| key.as_text().is_some_and(|text| text == key_name))
        .map(|(_, value)| value)
}

fn map_lookup_value_clone(map: &[(Value, Value)], key_name: &str) -> Result<Value, VerifyError> {
    map_lookup_optional_value(map, key_name)
        .cloned()
        .ok_or_else(|| VerifyError::new(format!("missing `{key_name}` value")))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs;
    use std::io::{Cursor, Write};
    use std::path::Path;

    use ciborium::Value;
    use trellis_cddl::parse_ed25519_cose_key;
    use zip::write::SimpleFileOptions;
    use zip::{CompressionMethod, ZipArchive, ZipWriter};

    use super::{
        parse_sign1_bytes, parse_signing_key_registry, verify_event_set, verify_export_zip,
        verify_single_event, verify_tampered_ledger,
    };

    fn rebuild_export_zip(template: &[u8], overrides: &[(&str, &[u8])], omit: &[&str]) -> Vec<u8> {
        let prefix = {
            let mut archive = ZipArchive::new(Cursor::new(template)).unwrap();
            let name = archive.by_index(0).unwrap().name().to_string();
            let (root, _) = name.split_once('/').unwrap();
            format!("{root}/")
        };

        let mut members: BTreeMap<String, Vec<u8>> = BTreeMap::new();
        {
            let mut archive = ZipArchive::new(Cursor::new(template)).unwrap();
            for index in 0..archive.len() {
                let mut file = archive.by_index(index).unwrap();
                let name = file.name().to_string();
                let (_, relative) = name.split_once('/').unwrap();
                let mut data = Vec::new();
                std::io::Read::read_to_end(&mut file, &mut data).unwrap();
                members.insert(relative.to_string(), data);
            }
        }
        for key in omit {
            members.remove(*key);
        }
        for (rel, data) in overrides {
            members.insert(rel.to_string(), data.to_vec());
        }

        let mut cursor = Cursor::new(Vec::new());
        {
            let mut zip = ZipWriter::new(&mut cursor);
            let opts = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
            for (relative, data) in members {
                zip.start_file(format!("{prefix}{relative}"), opts).unwrap();
                zip.write_all(&data).unwrap();
            }
            zip.finish().unwrap();
        }
        cursor.into_inner()
    }

    fn intake_accepted_payload(outputs: Option<Vec<Value>>) -> Vec<u8> {
        let mut map = vec![
            (
                Value::Text("recordKind".into()),
                Value::Text("intakeAccepted".into()),
            ),
            (
                Value::Text("data".into()),
                Value::Map(vec![
                    (
                        Value::Text("intakeId".into()),
                        Value::Text("handoff-1".into()),
                    ),
                    (
                        Value::Text("caseIntent".into()),
                        Value::Text("attachToExistingCase".into()),
                    ),
                    (
                        Value::Text("caseDisposition".into()),
                        Value::Text("attachToExistingCase".into()),
                    ),
                    (Value::Text("caseRef".into()), Value::Text("case-1".into())),
                ]),
            ),
        ];
        if let Some(outputs) = outputs {
            map.push((Value::Text("outputs".into()), Value::Array(outputs)));
        }
        let mut bytes = Vec::new();
        ciborium::into_writer(&Value::Map(map), &mut bytes).unwrap();
        bytes
    }

    fn case_created_payload(outputs: Option<Vec<Value>>) -> Vec<u8> {
        let mut map = vec![
            (
                Value::Text("recordKind".into()),
                Value::Text("caseCreated".into()),
            ),
            (
                Value::Text("data".into()),
                Value::Map(vec![
                    (Value::Text("caseRef".into()), Value::Text("case-1".into())),
                    (
                        Value::Text("intakeHandoffRef".into()),
                        Value::Text("handoff-1".into()),
                    ),
                    (
                        Value::Text("formspecResponseRef".into()),
                        Value::Text("response-1".into()),
                    ),
                    (
                        Value::Text("validationReportRef".into()),
                        Value::Text("validation-1".into()),
                    ),
                    (
                        Value::Text("ledgerHeadRef".into()),
                        Value::Text("ledger-1".into()),
                    ),
                    (
                        Value::Text("initiationMode".into()),
                        Value::Text("publicIntake".into()),
                    ),
                ]),
            ),
        ];
        if let Some(outputs) = outputs {
            map.push((Value::Text("outputs".into()), Value::Array(outputs)));
        }
        let mut bytes = Vec::new();
        ciborium::into_writer(&Value::Map(map), &mut bytes).unwrap();
        bytes
    }

    fn intake_handoff_value(initiation_mode: &str, case_ref: Value) -> Value {
        Value::Map(vec![
            (
                Value::Text("handoffId".into()),
                Value::Text("handoff-1".into()),
            ),
            (
                Value::Text("initiationMode".into()),
                Value::Text(initiation_mode.into()),
            ),
            (Value::Text("caseRef".into()), case_ref),
            (
                Value::Text("definitionRef".into()),
                Value::Map(vec![
                    (
                        Value::Text("url".into()),
                        Value::Text("https://example.test/definitions/intake".into()),
                    ),
                    (Value::Text("version".into()), Value::Text("1.0.0".into())),
                ]),
            ),
            (
                Value::Text("responseRef".into()),
                Value::Text("response-1".into()),
            ),
            (
                Value::Text("responseHash".into()),
                Value::Text(
                    "sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                        .into(),
                ),
            ),
            (
                Value::Text("validationReportRef".into()),
                Value::Text("validation-1".into()),
            ),
            (
                Value::Text("ledgerHeadRef".into()),
                Value::Text("ledger-1".into()),
            ),
        ])
    }

    #[test]
    fn verify_single_event_accepts_append_001_fixture() {
        let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/vectors/append/001-minimal-inline-payload");
        let key_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/vectors/_keys/issuer-001.cose_key");

        let signed_event = fs::read(fixture_root.join("expected-event.cbor")).unwrap();
        let parsed_key = parse_ed25519_cose_key(&fs::read(key_path).unwrap()).unwrap();

        let report = verify_single_event(parsed_key.public_key, &signed_event).unwrap();
        assert!(report.structure_verified);
        assert!(report.integrity_verified);
        assert!(report.readability_verified);
    }

    #[test]
    fn verify_export_zip_accepts_export_001_fixture() {
        let zip_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/vectors/verify/001-export-001-two-event-chain/input-export.zip");
        let report = verify_export_zip(&fs::read(zip_path).unwrap());
        assert!(report.structure_verified);
        assert!(report.integrity_verified);
        assert!(report.readability_verified);
    }

    #[test]
    fn verify_export_zip_rejects_invalid_zip_bytes() {
        let report = verify_export_zip(&[1, 2, 3, 4]);
        assert_eq!(report.event_failures[0].kind, "export_zip_invalid");
    }

    #[test]
    fn verify_export_zip_rejects_zip_without_export_root_directory() {
        let mut buf = Vec::new();
        {
            let mut cursor = Cursor::new(&mut buf);
            let mut zip = ZipWriter::new(&mut cursor);
            let opts = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
            zip.start_file("readme.txt", opts).unwrap();
            zip.write_all(b"x").unwrap();
            zip.finish().unwrap();
        }
        let report = verify_export_zip(&buf);
        assert_eq!(report.event_failures[0].kind, "export_zip_invalid");
        assert!(
            report.warnings[0].contains("export root")
                || report.warnings[0].contains("failed to parse ZIP"),
            "{}",
            report.warnings[0]
        );
    }

    #[test]
    fn verify_export_zip_missing_manifest_is_fatal() {
        let template =
            fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(
                "../../fixtures/vectors/verify/001-export-001-two-event-chain/input-export.zip",
            ))
            .unwrap();
        let zip = rebuild_export_zip(&template, &[], &["000-manifest.cbor"]);
        let report = verify_export_zip(&zip);
        assert_eq!(report.event_failures[0].kind, "missing_manifest");
    }

    #[test]
    fn verify_export_zip_tampered_events_triggers_archive_integrity_failure() {
        let template =
            fs::read(Path::new(env!("CARGO_MANIFEST_DIR")).join(
                "../../fixtures/vectors/verify/001-export-001-two-event-chain/input-export.zip",
            ))
            .unwrap();
        let zip = rebuild_export_zip(&template, &[("010-events.cbor", &[0xff])], &[]);
        let report = verify_export_zip(&zip);
        assert_eq!(
            report.event_failures[0].kind, "archive_integrity_failure",
            "manifest member digests are checked before 010-events.cbor is parsed"
        );
    }

    #[test]
    fn parse_sign1_array_rejects_invalid_cbor() {
        assert!(super::parse_sign1_array(&[0xff]).is_err());
    }

    #[test]
    fn parse_sign1_array_rejects_array_of_non_sign1_items() {
        let mut bytes = Vec::new();
        ciborium::into_writer(
            &ciborium::Value::Array(vec![ciborium::Value::Integer(0.into())]),
            &mut bytes,
        )
        .unwrap();
        assert!(super::parse_sign1_array(&bytes).is_err());
    }

    #[test]
    fn parse_intake_accepted_record_rejects_missing_or_empty_outputs() {
        let missing = super::parse_intake_accepted_record(&intake_accepted_payload(None))
            .expect_err("missing outputs must fail");
        assert!(missing.to_string().contains("outputs"), "{missing}");

        let empty = super::parse_intake_accepted_record(&intake_accepted_payload(Some(vec![])))
            .expect_err("empty outputs must fail");
        assert!(empty.to_string().contains("outputs"), "{empty}");
    }

    #[test]
    fn parse_case_created_record_rejects_missing_or_empty_outputs() {
        let missing = super::parse_case_created_record(&case_created_payload(None))
            .expect_err("missing outputs must fail");
        assert!(missing.to_string().contains("outputs"), "{missing}");

        let empty = super::parse_case_created_record(&case_created_payload(Some(vec![])))
            .expect_err("empty outputs must fail");
        assert!(empty.to_string().contains("outputs"), "{empty}");
    }

    #[test]
    fn parse_intake_handoff_details_rejects_public_intake_with_case_ref() {
        let error = super::parse_intake_handoff_details(&intake_handoff_value(
            "publicIntake",
            Value::Text("urn:wos:case:case-1".into()),
        ))
        .expect_err("public intake caseRef must fail");
        assert!(error.to_string().contains("caseRef"), "{error}");
    }

    #[test]
    fn parse_intake_handoff_details_accepts_public_intake_with_null_case_ref() {
        let details =
            super::parse_intake_handoff_details(&intake_handoff_value("publicIntake", Value::Null))
                .expect("null public intake caseRef must pass");
        assert_eq!(details.initiation_mode, "publicIntake");
        assert_eq!(details.case_ref, None);
    }

    #[test]
    fn verify_tampered_ledger_rejects_signature_flip() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/vectors/tamper/001-signature-flip");
        let report = verify_tampered_ledger(
            &fs::read(root.join("input-signing-key-registry.cbor")).unwrap(),
            &fs::read(root.join("input-tampered-ledger.cbor")).unwrap(),
            None,
            None,
        )
        .unwrap();
        assert!(report.structure_verified);
        assert!(!report.integrity_verified);
        assert!(report.readability_verified);
        assert_eq!(report.event_failures[0].kind, "signature_invalid");
    }

    #[test]
    fn verify_event_rejects_signature_after_revocation_valid_to() {
        let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/vectors/append/009-signing-key-revocation");
        let signed_event = fs::read(fixture_root.join("expected-event.cbor")).unwrap();
        let mut registry_value = ciborium::from_reader::<ciborium::Value, _>(
            &fs::read(fixture_root.join("input-signing-key-registry-after.cbor")).unwrap()[..],
        )
        .unwrap();
        let registry_entries = registry_value.as_array_mut().unwrap();
        let entry_map = registry_entries[0].as_map_mut().unwrap();
        for (key, value) in entry_map.iter_mut() {
            if key.as_text() == Some("valid_to") {
                *value = ciborium::Value::Integer(1745109999u64.into());
            }
        }
        let mut registry_bytes = Vec::new();
        ciborium::into_writer(&registry_value, &mut registry_bytes).unwrap();

        let parsed = parse_sign1_bytes(&signed_event).unwrap();
        let registry = parse_signing_key_registry(&registry_bytes).unwrap();
        let report = verify_event_set(&[parsed], &registry, None, None, false, None, None);

        assert!(report.structure_verified);
        assert!(!report.integrity_verified);
        assert!(report.readability_verified);
        assert_eq!(report.event_failures[0].kind, "revoked_authority");
    }

    #[test]
    fn verify_event_allows_historical_signature_before_revocation_valid_to() {
        let fixture_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/vectors/append/009-signing-key-revocation");
        let signed_event = fs::read(fixture_root.join("expected-event.cbor")).unwrap();
        let registry_bytes =
            fs::read(fixture_root.join("input-signing-key-registry-after.cbor")).unwrap();

        let parsed = parse_sign1_bytes(&signed_event).unwrap();
        let registry = parse_signing_key_registry(&registry_bytes).unwrap();
        let report = verify_event_set(&[parsed], &registry, None, None, false, None, None);

        assert!(report.structure_verified);
        assert!(report.integrity_verified);
        assert!(report.readability_verified);
    }

    #[test]
    fn rfc6962_inclusion_paths_reconstruct_three_leaf_root() {
        use super::{
            merkle_interior_hash, merkle_leaf_hash, merkle_root, root_from_inclusion_proof,
        };

        let c0 = [1u8; 32];
        let c1 = [2u8; 32];
        let c2 = [3u8; 32];
        let l0 = merkle_leaf_hash(c0);
        let l1 = merkle_leaf_hash(c1);
        let l2 = merkle_leaf_hash(c2);
        let root = merkle_root(&[l0, l1, l2]);
        let h01 = merkle_interior_hash(l0, l1);

        assert_eq!(root_from_inclusion_proof(2, 3, l2, &[h01]).unwrap(), root);
        assert_eq!(
            root_from_inclusion_proof(0, 3, l0, &[l1, l2]).unwrap(),
            root
        );
        assert_eq!(
            root_from_inclusion_proof(1, 3, l1, &[l0, l2]).unwrap(),
            root
        );
    }

    #[test]
    fn rfc6962_consistency_proof_one_to_two() {
        use super::{merkle_leaf_hash, merkle_root, root_from_consistency_proof};

        let l0 = merkle_leaf_hash([9u8; 32]);
        let l1 = merkle_leaf_hash([8u8; 32]);
        let r1 = merkle_root(&[l0]);
        let r2 = merkle_root(&[l0, l1]);
        assert_eq!(root_from_consistency_proof(1, 2, r1, &[l1]).unwrap(), r2);
        let wrong_head = root_from_consistency_proof(1, 2, r1, &[[0u8; 32]]).unwrap();
        assert_ne!(wrong_head, r2);
    }

    #[test]
    fn inclusion_proof_rejects_short_audit_sibling() {
        use super::{merkle_leaf_hash, root_from_inclusion_proof};

        let leaf = merkle_leaf_hash([4u8; 32]);
        let bad = [0u8; 31];
        let v = ciborium::Value::Bytes(bad.to_vec());
        let path = [v];
        assert!(super::digest_path_from_values(&path).is_err());
        assert!(root_from_inclusion_proof(0, 1, leaf, &[]).unwrap() == leaf);
        assert!(root_from_inclusion_proof(0, 2, leaf, &[]).is_err());
    }

    fn test_attachment_hash(suffix: u8) -> [u8; 32] {
        let mut b = [0u8; 32];
        b[31] = suffix;
        b
    }

    fn attachment_manifest_cbor(rows: &[([u8; 32], Option<[u8; 32]>)]) -> Vec<u8> {
        use ciborium::Value as V;
        let entries = rows
            .iter()
            .map(|(binding, prior)| {
                let mut pairs: Vec<(V, V)> = vec![
                    (
                        V::Text("binding_event_hash".into()),
                        V::Bytes(binding.to_vec()),
                    ),
                    (V::Text("attachment_id".into()), V::Text("id".into())),
                    (V::Text("slot_path".into()), V::Text("slot".into())),
                    (
                        V::Text("media_type".into()),
                        V::Text("application/octet-stream".into()),
                    ),
                    (V::Text("byte_length".into()), V::Integer(1u64.into())),
                    (
                        V::Text("attachment_sha256".into()),
                        V::Bytes([7u8; 32].to_vec()),
                    ),
                    (
                        V::Text("payload_content_hash".into()),
                        V::Bytes([8u8; 32].to_vec()),
                    ),
                ];
                if let Some(p) = prior {
                    pairs.push((V::Text("prior_binding_hash".into()), V::Bytes(p.to_vec())));
                }
                V::Map(pairs)
            })
            .collect::<Vec<_>>();
        let root = V::Array(entries);
        let mut out = Vec::new();
        ciborium::into_writer(&root, &mut out).unwrap();
        out
    }

    #[test]
    fn attachment_topology_duplicate_binding_event_hash() {
        let h = test_attachment_hash(1);
        let bytes = attachment_manifest_cbor(&[(h, None), (h, None)]);
        let entries = super::parse_attachment_manifest_entries(&bytes).unwrap();
        let mut m = std::collections::BTreeMap::new();
        m.insert(h, 0usize);
        let f = super::attachment_manifest_topology_failures(&entries, &m);
        assert!(
            f.iter()
                .any(|e| e.kind == "attachment_manifest_duplicate_binding")
        );
    }

    #[test]
    fn attachment_topology_unresolved_prior() {
        let h0 = test_attachment_hash(2);
        let h_unknown = test_attachment_hash(99);
        let bytes = attachment_manifest_cbor(&[(h0, Some(h_unknown))]);
        let entries = super::parse_attachment_manifest_entries(&bytes).unwrap();
        let mut m = std::collections::BTreeMap::new();
        m.insert(h0, 0usize);
        let f = super::attachment_manifest_topology_failures(&entries, &m);
        assert!(
            f.iter()
                .any(|e| e.kind == "attachment_prior_binding_unresolved")
        );
    }

    #[test]
    fn attachment_topology_forward_reference() {
        let h0 = test_attachment_hash(3);
        let h1 = test_attachment_hash(4);
        let bytes = attachment_manifest_cbor(&[(h0, Some(h1))]);
        let entries = super::parse_attachment_manifest_entries(&bytes).unwrap();
        let mut m = std::collections::BTreeMap::new();
        m.insert(h0, 0usize);
        m.insert(h1, 1);
        let f = super::attachment_manifest_topology_failures(&entries, &m);
        assert!(
            f.iter()
                .any(|e| e.kind == "attachment_prior_binding_forward_reference")
        );
    }

    #[test]
    fn attachment_topology_lineage_two_cycle() {
        let h0 = test_attachment_hash(10);
        let h1 = test_attachment_hash(11);
        let bytes = attachment_manifest_cbor(&[(h1, Some(h0)), (h0, Some(h1))]);
        let entries = super::parse_attachment_manifest_entries(&bytes).unwrap();
        let mut m = std::collections::BTreeMap::new();
        m.insert(h0, 0usize);
        m.insert(h1, 1);
        let f = super::attachment_manifest_topology_failures(&entries, &m);
        assert!(
            f.iter()
                .any(|e| e.kind == "attachment_binding_lineage_cycle")
        );
    }

    #[test]
    fn attachment_topology_lineage_three_cycle() {
        let h0 = test_attachment_hash(20);
        let h1 = test_attachment_hash(21);
        let h2 = test_attachment_hash(22);
        let bytes = attachment_manifest_cbor(&[(h0, Some(h2)), (h1, Some(h0)), (h2, Some(h1))]);
        let entries = super::parse_attachment_manifest_entries(&bytes).unwrap();
        let mut m = std::collections::BTreeMap::new();
        m.insert(h0, 0usize);
        m.insert(h1, 1);
        m.insert(h2, 2);
        let f = super::attachment_manifest_topology_failures(&entries, &m);
        assert!(
            f.iter()
                .any(|e| e.kind == "attachment_binding_lineage_cycle")
        );
    }

    #[test]
    fn attachment_topology_multirevision_ok() {
        let h0 = test_attachment_hash(30);
        let h1 = test_attachment_hash(31);
        let h2 = test_attachment_hash(32);
        let bytes = attachment_manifest_cbor(&[(h0, None), (h1, Some(h0)), (h2, Some(h1))]);
        let entries = super::parse_attachment_manifest_entries(&bytes).unwrap();
        let mut m = std::collections::BTreeMap::new();
        m.insert(h0, 0usize);
        m.insert(h1, 1);
        m.insert(h2, 2);
        let f = super::attachment_manifest_topology_failures(&entries, &m);
        assert!(f.is_empty());
    }

    fn signature_manifest_entry(event_hash: [u8; 32]) -> super::SignatureManifestEntry {
        super::SignatureManifestEntry {
            canonical_event_hash: event_hash,
            signer_id: "applicant".to_string(),
            role_id: "applicantSigner".to_string(),
            role: "signer".to_string(),
            document_id: "benefitsApplication".to_string(),
            document_hash: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                .to_string(),
            document_hash_algorithm: "sha-256".to_string(),
            signed_at: "2026-04-22T14:30:00Z".to_string(),
            identity_binding: Value::Map(vec![
                (
                    Value::Text("method".into()),
                    Value::Text("email-otp".into()),
                ),
                (
                    Value::Text("assuranceLevel".into()),
                    Value::Text("standard".into()),
                ),
            ]),
            consent_reference: Value::Map(vec![
                (
                    Value::Text("consentTextRef".into()),
                    Value::Text("urn:agency.gov:consent:esign-benefits:v1".into()),
                ),
                (
                    Value::Text("consentVersion".into()),
                    Value::Text("1.0.0".into()),
                ),
                (
                    Value::Text("acceptedAtPath".into()),
                    Value::Text("response.signature.acceptedAt".into()),
                ),
                (
                    Value::Text("affirmationPath".into()),
                    Value::Text("response.signature.affirmed".into()),
                ),
            ]),
            signature_provider: "urn:agency.gov:signature:providers:formspec".to_string(),
            ceremony_id: "ceremony-2026-0001".to_string(),
            profile_ref: Some("urn:agency.gov:wos:signature-profile:benefits:v1".to_string()),
            profile_key: None,
            formspec_response_ref: "urn:agency.gov:formspec:responses:benefits:case-2026-0001"
                .to_string(),
        }
    }

    fn signature_record_details() -> super::SignatureAffirmationRecordDetails {
        let entry = signature_manifest_entry(test_attachment_hash(40));
        super::SignatureAffirmationRecordDetails {
            signer_id: entry.signer_id,
            role_id: entry.role_id,
            role: entry.role,
            document_id: entry.document_id,
            document_hash: entry.document_hash,
            document_hash_algorithm: entry.document_hash_algorithm,
            signed_at: entry.signed_at,
            identity_binding: entry.identity_binding,
            consent_reference: entry.consent_reference,
            signature_provider: entry.signature_provider,
            ceremony_id: entry.ceremony_id,
            profile_ref: entry.profile_ref,
            profile_key: entry.profile_key,
            formspec_response_ref: entry.formspec_response_ref,
        }
    }

    #[test]
    fn signature_catalog_entry_matches_record_when_fields_align() {
        let entry = signature_manifest_entry(test_attachment_hash(41));
        let record = signature_record_details();
        assert!(super::signature_entry_matches_record(&entry, &record));
    }

    #[test]
    fn signature_catalog_entry_detects_field_mismatch() {
        let entry = signature_manifest_entry(test_attachment_hash(42));
        let mut record = signature_record_details();
        record.document_hash_algorithm = "sha-512".to_string();
        assert!(!super::signature_entry_matches_record(&entry, &record));
    }

    #[test]
    fn cbor_nested_map_semantic_eq_ignores_map_entry_order() {
        let a = Value::Map(vec![
            (Value::Text("z".into()), Value::Integer(1.into())),
            (Value::Text("a".into()), Value::Integer(2.into())),
        ]);
        let b = Value::Map(vec![
            (Value::Text("a".into()), Value::Integer(2.into())),
            (Value::Text("z".into()), Value::Integer(1.into())),
        ]);
        assert!(super::cbor_nested_map_semantic_eq(&a, &b));
    }

    #[test]
    fn cbor_nested_map_semantic_eq_nested_maps_ignore_order() {
        let inner_a = Value::Map(vec![
            (Value::Text("second".into()), Value::Bool(false)),
            (Value::Text("first".into()), Value::Bool(true)),
        ]);
        let inner_b = Value::Map(vec![
            (Value::Text("first".into()), Value::Bool(true)),
            (Value::Text("second".into()), Value::Bool(false)),
        ]);
        let outer_a = Value::Map(vec![(Value::Text("k".into()), inner_a)]);
        let outer_b = Value::Map(vec![(Value::Text("k".into()), inner_b)]);
        assert!(super::cbor_nested_map_semantic_eq(&outer_a, &outer_b));
    }
}
