// Rust guideline compliant 2026-02-21
//! Trellis verification for single events, tamper fixtures, and export ZIPs.

#![forbid(unsafe_code)]

use std::backtrace::Backtrace;
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::io::Cursor;

use ciborium::Value;
use ed25519_dalek::ed25519::signature::Verifier;
use ed25519_dalek::{Signature, VerifyingKey};
use sha2::{Digest, Sha256};
use trellis_cose::sig_structure_bytes;
use trellis_types::{
    AUTHOR_EVENT_DOMAIN, CONTENT_DOMAIN, EVENT_DOMAIN, domain_separated_sha256, encode_bstr,
    encode_tstr, encode_uint,
};
use zip::ZipArchive;

const SUITE_ID_PHASE_1: i128 = 1;
const ALG_EDDSA: i128 = -8;
const COSE_LABEL_ALG: i128 = 1;
const COSE_LABEL_KID: i128 = 4;
const COSE_LABEL_SUITE_ID: i128 = -65_537;
const CHECKPOINT_DOMAIN: &str = "trellis-checkpoint-v1";
const MERKLE_LEAF_DOMAIN: &str = "trellis-merkle-leaf-v1";
const MERKLE_INTERIOR_DOMAIN: &str = "trellis-merkle-interior-v1";
const POSTURE_DECLARATION_DOMAIN: &str = "trellis-posture-declaration-v1";

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
    registry.insert(parsed.kid.clone(), public_key_bytes);
    Ok(verify_event_set(
        &[parsed],
        &registry,
        None,
        None,
        false,
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
    let registry = parse_signing_key_registry(signing_key_registry)?;
    let events = parse_sign1_array(ledger).unwrap_or_else(|_| Vec::new());
    if events.is_empty() {
        return Ok(VerificationReport::fatal(
            "malformed_cose",
            "ledger is not a non-empty dCBOR array of COSE_Sign1 events",
        ));
    }

    Ok(verify_event_set(
        &events,
        &registry,
        initial_posture_declaration,
        posture_declaration,
        true,
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
    let registry = match parse_signing_key_registry(signing_key_registry_bytes) {
        Ok(registry) => registry,
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

    if manifest.alg != ALG_EDDSA || manifest.suite_id != SUITE_ID_PHASE_1 {
        return VerificationReport::fatal(
            "unsupported_suite",
            "manifest protected header does not match the Trellis Phase-1 suite",
        );
    }

    let manifest_public_key = match registry.get(&manifest.kid) {
        Some(public_key) => *public_key,
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
        ("030-signing-key-registry.cbor", "signing_key_registry_digest"),
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
    let mut report = verify_event_set(&events, &registry, None, None, false, Some(scope.as_slice()));
    for failure in &mut report.event_failures {
        if failure.kind == "scope_mismatch" {
            failure.location = format!("manifest-scope/{}", failure.location);
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
            Some(public_key) => *public_key,
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
            let actual_prev = match map_lookup_fixed_bytes(payload_map, "prev_checkpoint_hash", 32) {
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

    let head_checkpoint_digest = match map_lookup_fixed_bytes(manifest_map, "head_checkpoint_digest", 32) {
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
        && report
            .posture_transitions
            .iter()
            .all(|outcome| outcome.continuity_verified
                && outcome.declaration_resolved
                && outcome.attestations_verified);
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
    prev_hash: Option<[u8; 32]>,
    author_event_hash: [u8; 32],
    content_hash: [u8; 32],
    canonical_event_hash: [u8; 32],
    ciphertext: Vec<u8>,
    transition: Option<TransitionDetails>,
}

#[derive(Clone, Debug)]
struct TransitionDetails {
    transition_id: String,
    from_state: String,
    to_state: String,
    declaration_digest: [u8; 32],
    attestation_classes: Vec<String>,
}

#[derive(Debug)]
struct ExportArchive {
    members: BTreeMap<String, Vec<u8>>,
}

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
            return Err(VerifyError::new("ZIP member does not live under one export root"));
        };
        let mut data = Vec::new();
        std::io::Read::read_to_end(&mut file, &mut data)
            .map_err(|error| VerifyError::new(format!("failed to read ZIP member bytes: {error}")))?;
        members.insert(relative_name.to_string(), data);
    }
    Ok(ExportArchive { members })
}

fn verify_event_set(
    events: &[ParsedSign1],
    registry: &BTreeMap<Vec<u8>, [u8; 32]>,
    initial_posture_declaration: Option<&[u8]>,
    posture_declaration: Option<&[u8]>,
    classify_tamper: bool,
    expected_ledger_scope: Option<&[u8]>,
) -> VerificationReport {
    let mut event_failures = Vec::new();
    let mut posture_transitions = Vec::new();
    let mut previous_hash: Option<[u8; 32]> = None;
    let skip_prev_hash_check = initial_posture_declaration.is_some() && events.len() == 1;
    let mut shadow_custody_model = initial_posture_declaration
        .and_then(|bytes| parse_custody_model(bytes).ok());

    for (index, event) in events.iter().enumerate() {
        let public_key = match registry.get(&event.kid) {
            Some(public_key) => *public_key,
            None => {
                return VerificationReport::fatal(
                    "unresolvable_manifest_kid",
                    "event kid is not resolvable via the provided signing-key registry",
                );
            }
        };
        if event.alg != ALG_EDDSA || event.suite_id != SUITE_ID_PHASE_1 {
            return VerificationReport::fatal(
                "unsupported_suite",
                "event protected header does not match the Trellis Phase-1 suite",
            );
        }
        if !verify_signature(event, public_key) {
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

        if let Some(expected) = expected_ledger_scope {
            if details.scope.as_slice() != expected {
                event_failures.push(VerificationFailure::new(
                    "scope_mismatch",
                    hex_string(&details.canonical_event_hash),
                ));
            }
        }

        let expected_content_hash = domain_separated_sha256(CONTENT_DOMAIN, &details.ciphertext);
        if expected_content_hash != details.content_hash {
            event_failures.push(VerificationFailure::new(
                "content_hash_mismatch",
                hex_string(&details.canonical_event_hash),
            ));
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
                event_failures.push(VerificationFailure::new(
                    "prev_hash_mismatch",
                    hex_string(&details.canonical_event_hash),
                ));
            }
        } else if previous_hash != details.prev_hash {
            let kind = if classify_tamper {
                if events.len() == 1 {
                    "event_truncation"
                } else {
                    "event_reorder"
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
                kind: "custody-model".into(),
                event_index: index as u64,
                from_state: transition.from_state.clone(),
                to_state: transition.to_state.clone(),
                continuity_verified: true,
                declaration_resolved: true,
                attestations_verified: true,
                failures: Vec::new(),
            };

            if let Some(initial_state) = shadow_custody_model.clone() {
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

            if requires_dual_attestation(&transition.from_state, &transition.to_state)
                && !(transition.attestation_classes.iter().any(|value| value == "prior")
                    && transition.attestation_classes.iter().any(|value| value == "new"))
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
            shadow_custody_model = Some(transition.to_state.clone());
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
        return Err(VerifyError::new("COSE_Sign1 body does not contain four fields"));
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
    let author_event_hash = bytes_array(&map_lookup_fixed_bytes(payload_map, "author_event_hash", 32)?);
    let content_hash = bytes_array(&map_lookup_fixed_bytes(payload_map, "content_hash", 32)?);
    let canonical_event_hash = recompute_canonical_event_hash(&scope, payload_bytes);

    let header = map_lookup_map(payload_map, "header")?;
    let event_type_bytes = map_lookup_bytes(header, "event_type")?;
    let _event_type = String::from_utf8(event_type_bytes)
        .map_err(|_| VerifyError::new("header.event_type is not valid UTF-8"))?;

    let payload_ref = map_lookup_map(payload_map, "payload_ref")?;
    let ciphertext = map_lookup_bytes(payload_ref, "ciphertext")?;

    let transition = match map_lookup_optional_map(payload_map, "extensions")? {
        Some(extensions) => decode_transition_details(extensions)?,
        None => None,
    };

    Ok(EventDetails {
        scope,
        sequence,
        prev_hash,
        author_event_hash,
        content_hash,
        canonical_event_hash,
        ciphertext,
        transition,
    })
}

fn decode_transition_details(
    extensions: &[(Value, Value)],
) -> Result<Option<TransitionDetails>, VerifyError> {
    let Some(extension_value) = map_lookup_optional_value(
        extensions,
        "trellis.custody-model-transition.v1",
    ) else {
        return Ok(None);
    };
    let extension_map = extension_value
        .as_map()
        .ok_or_else(|| VerifyError::new("transition extension is not a map"))?;
    let transition_id = map_lookup_text(extension_map, "transition_id")?;
    let from_state = map_lookup_text(extension_map, "from_custody_model")?;
    let to_state = map_lookup_text(extension_map, "to_custody_model")?;
    let _effective_at = map_lookup_u64(extension_map, "effective_at")?;
    let declaration_digest =
        bytes_array(&map_lookup_fixed_bytes(extension_map, "declaration_doc_digest", 32)?);
    let attestations = map_lookup_array(extension_map, "attestations")?;
    let attestation_classes = attestations
        .iter()
        .filter_map(|item| item.as_map())
        .filter_map(|map| map_lookup_text(map, "authority_class").ok())
        .collect::<Vec<_>>();

    Ok(Some(TransitionDetails {
        transition_id,
        from_state,
        to_state,
        declaration_digest,
        attestation_classes,
    }))
}

fn parse_signing_key_registry(bytes: &[u8]) -> Result<BTreeMap<Vec<u8>, [u8; 32]>, VerifyError> {
    let value = decode_value(bytes)?;
    let entries = value
        .as_array()
        .ok_or_else(|| VerifyError::new("signing-key registry root is not an array"))?;
    let mut registry = BTreeMap::new();
    for entry in entries {
        let map = entry
            .as_map()
            .ok_or_else(|| VerifyError::new("signing-key registry entry is not a map"))?;
        let kid = map_lookup_bytes(map, "kid")?;
        let pubkey = bytes_array(&map_lookup_fixed_bytes(map, "pubkey", 32)?);
        registry.insert(kid, pubkey);
    }
    Ok(registry)
}

fn parse_custody_model(bytes: &[u8]) -> Result<String, VerifyError> {
    let value = decode_value(bytes)?;
    let map = value
        .as_map()
        .ok_or_else(|| VerifyError::new("posture declaration root is not a map"))?;
    let custody_model = map_lookup_map(map, "custody_model")?;
    map_lookup_text(custody_model, "custody_model_id")
}

fn event_identity(event: &ParsedSign1) -> Result<(Vec<u8>, [u8; 32]), VerifyError> {
    let details = decode_event_details(event)?;
    Ok((details.scope, details.canonical_event_hash))
}

fn recompute_author_event_hash(canonical_event_bytes: &[u8]) -> Option<[u8; 32]> {
    let authored = authored_preimage_from_canonical(canonical_event_bytes)?;
    Some(domain_separated_sha256(AUTHOR_EVENT_DOMAIN, &authored))
}

fn authored_preimage_from_canonical(canonical_event_bytes: &[u8]) -> Option<Vec<u8>> {
    let key = encode_tstr("author_event_hash");
    let key_position = canonical_event_bytes
        .windows(key.len())
        .rposition(|window| window == key.as_slice())?;
    let value_position = key_position + key.len();
    if canonical_event_bytes.len() != value_position + 34 {
        return None;
    }
    if canonical_event_bytes[value_position] != 0x58 || canonical_event_bytes[value_position + 1] != 0x20 {
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
        0 => [0u8; 32],
        1 => leaves[0],
        _ => {
            let mut level = leaves.to_vec();
            while level.len() > 1 {
                let mut next = Vec::new();
                let mut index = 0;
                while index < level.len() {
                    if index + 1 == level.len() {
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

fn map_lookup_u64(map: &[(Value, Value)], key_name: &str) -> Result<u64, VerifyError> {
    let value = map_lookup_optional_value(map, key_name)
        .ok_or_else(|| VerifyError::new(format!("missing `{key_name}`")))?;
    value
        .as_integer()
        .and_then(|integer| integer.try_into().ok())
        .ok_or_else(|| VerifyError::new(format!("`{key_name}` is not an unsigned integer")))
}

fn map_lookup_text(map: &[(Value, Value)], key_name: &str) -> Result<String, VerifyError> {
    map_lookup_optional_value(map, key_name)
        .and_then(|value| value.as_text().map(ToOwned::to_owned))
        .ok_or_else(|| VerifyError::new(format!("missing or invalid `{key_name}` text")))
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

fn map_lookup_integer_label_bytes(
    map: &[(Value, Value)],
    label: i128,
) -> Result<Vec<u8>, VerifyError> {
    map.iter()
        .find(|(key, _)| key.as_integer().is_some_and(|value| i128::from(value) == label))
        .and_then(|(_, value)| value.as_bytes().cloned())
        .ok_or_else(|| VerifyError::new(format!("missing COSE label {label} bytes")))
}

fn map_lookup_integer_label(
    map: &[(Value, Value)],
    label: i128,
) -> Result<i128, VerifyError> {
    map.iter()
        .find(|(key, _)| key.as_integer().is_some_and(|value| i128::from(value) == label))
        .and_then(|(_, value)| value.as_integer())
        .map(i128::from)
        .ok_or_else(|| VerifyError::new(format!("missing COSE label {label} integer")))
}

fn map_lookup_optional_value<'a>(map: &'a [(Value, Value)], key_name: &str) -> Option<&'a Value> {
    map.iter()
        .find(|(key, _)| key.as_text().is_some_and(|text| text == key_name))
        .map(|(_, value)| value)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use trellis_cddl::parse_ed25519_cose_key;

    use super::{verify_export_zip, verify_single_event, verify_tampered_ledger};

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
    fn rfc6962_inclusion_paths_reconstruct_three_leaf_root() {
        use super::{merkle_interior_hash, merkle_leaf_hash, merkle_root, root_from_inclusion_proof};

        let c0 = [1u8; 32];
        let c1 = [2u8; 32];
        let c2 = [3u8; 32];
        let l0 = merkle_leaf_hash(c0);
        let l1 = merkle_leaf_hash(c1);
        let l2 = merkle_leaf_hash(c2);
        let root = merkle_root(&[l0, l1, l2]);
        let h01 = merkle_interior_hash(l0, l1);

        assert_eq!(
            root_from_inclusion_proof(2, 3, l2, &[h01]).unwrap(),
            root
        );
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
        assert_eq!(
            root_from_consistency_proof(1, 2, r1, &[l1]).unwrap(),
            r2
        );
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
}
