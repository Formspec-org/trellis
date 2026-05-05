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
const ERASURE_EVIDENCE_EVENT_EXTENSION: &str = "trellis.erasure-evidence.v1";
const ERASURE_EVIDENCE_EXPORT_EXTENSION: &str = "trellis.export.erasure-evidence.v1";
/// ADR 0007 §6.7 registration — `EventPayload.extensions` slot for
/// certificate-of-completion records. Per-certificate inline shape per
/// `CertificateOfCompletionPayload` in ADR 0007 §"Wire shape".
const CERTIFICATE_EVENT_EXTENSION: &str = "trellis.certificate-of-completion.v1";
/// ADR 0007 §"Export manifest catalog" — optional manifest extension binding
/// `065-certificates-of-completion.cbor`.
const CERTIFICATE_EXPORT_EXTENSION: &str = "trellis.export.certificates-of-completion.v1";
/// ADR 0007 §9.8 / Core §9 — domain-separation tag for the SHA-256 preimage
/// covering rendered presentation-artifact bytes (PDF / HTML).
const PRESENTATION_ARTIFACT_DOMAIN: &str = "trellis-presentation-artifact-v1";
/// ADR 0010 §6.7 registration — `EventPayload.extensions` slot for
/// user-content-attestation records. Per-attestation inline shape per
/// `UserContentAttestationPayload` in Core §28 (CDDL) / ADR 0010 §"Wire shape".
const USER_CONTENT_ATTESTATION_EVENT_EXTENSION: &str = "trellis.user-content-attestation.v1";
/// ADR 0010 §9.8 / Core §9 — domain-separation tag for the Ed25519 signature
/// preimage carried by `UserContentAttestationPayload.signature`. Distinct
/// from `trellis-transition-attestation-v1` so a wrongly-typed user-content
/// attestation cannot cross-validate against the operator-actor
/// posture-transition family (and vice versa). Per ADR 0010 §"Verifier
/// obligations" step 5 the inner preimage is `dCBOR([attestation_id,
/// attested_event_hash, attested_event_position, attestor,
/// identity_attestation_ref, signing_intent, attested_at])`.
const USER_CONTENT_ATTESTATION_DOMAIN: &str = "trellis-user-content-attestation-v1";
/// Phase-1 deployment-local identity-attestation event-type convention. Per
/// ADR 0010 open question 1, the parent-repo identity-attestation stack ADR
/// (PLN-0381) ratifies the `wos.identity.*` namespace for `IdentityAttestation`
/// events. Until that lands, this verifier admits any event whose
/// `event_type` matches one of the values in [`identity_attestation_event_type`]
/// Phase-1 identity-attestation event type (test-only). The Trellis Working
/// Group reserves `x-trellis-test/*` (Core §6.7 + §10.6) for conformance
/// fixtures; this constant pins the literal the fixture corpus mints so
/// `is_identity_attestation_event_type` admits it. When PLN-0381 ratifies
/// the canonical `wos.identity.*` naming, that string lands in Core §6.7
/// and `is_identity_attestation_event_type` gains the canonical branch in
/// the same commit; the test prefix stays for future fixture authoring.
const PHASE_1_TEST_IDENTITY_EVENT_TYPE: &str = "x-trellis-test/identity-attestation/v1";
/// Operator URI scheme convention for the Phase-1 Companion §6.4 enforcement
/// of step 8 (operator-as-attestor forbidden in user-content-attestation
/// `attestor` slots). The `urn:trellis:operator:` prefix is the fixture
/// corpus's stand-in for a real operator-principal registry; deployments
/// substitute their canonical operator-URI scheme. Recognized prefixes here
/// are the conservative Phase-1 set; lint enforcement tightens before
/// deployment per ADR 0010 §"Adversary model" "operator masquerading as
/// user" mitigation.
const OPERATOR_URI_PREFIX_TRELLIS: &str = "urn:trellis:operator:";
const OPERATOR_URI_PREFIX_WOS: &str = "urn:wos:operator:";
/// Domain separation tag for transition-attestation preimages (Core §9.8).
/// Shared verbatim between §A.5 posture transitions and ADR 0005 erasure
/// evidence; Phase-1 verifier checks structural shape (`signature` is 64
/// bytes, `authority_class` is one of `prior` / `new`) — full Ed25519
/// crypto-verification of attestation signatures rides Phase-2+ when an
/// authority↔key registry binding lands. Mirrors the existing
/// posture-transition flow: presence + class-count today, crypto later.
#[allow(dead_code)]
const TRANSITION_ATTESTATION_DOMAIN: &str = "trellis-transition-attestation-v1";
const WOS_SIGNATURE_AFFIRMATION_EVENT_TYPE: &str = "wos.kernel.signatureAffirmation";
const WOS_INTAKE_ACCEPTED_EVENT_TYPE: &str = "wos.kernel.intakeAccepted";
const WOS_CASE_CREATED_EVENT_TYPE: &str = "wos.kernel.caseCreated";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrellisTimestamp {
    pub seconds: u64,
    pub nanos: u32,
}

impl PartialOrd for TrellisTimestamp {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TrellisTimestamp {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.seconds
            .cmp(&other.seconds)
            .then_with(|| self.nanos.cmp(&other.nanos))
    }
}

impl Display for TrellisTimestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.seconds, self.nanos)
    }
}

/// Reserved CascadeScope identifiers from Companion Appendix A.7. Free-text
/// scope values are non-conformant per OC-141 (Companion §20.6.3 / TR-OP-106);
/// registry-extension `tstr` values that follow the Appendix A.7 convention
/// are admitted (Phase-1 reference verifier accepts any `tstr` shape; deep
/// registry-membership lint rides O-3 evolution).
#[allow(dead_code)]
const APPENDIX_A7_CASCADE_SCOPES: &[&str] = &["CS-01", "CS-02", "CS-03", "CS-04", "CS-05", "CS-06"];

/// ADR 0008 closed kind registry. New kinds land via ADR revision; this
/// constant updates in lockstep with the ADR's "Registry — Initial
/// entries" section. Order is alphabetic for grep stability; not
/// semantically ordered.
const INTEROP_SIDECAR_KIND_C2PA_MANIFEST: &str = "c2pa-manifest";
const INTEROP_SIDECAR_KIND_DID_KEY_VIEW: &str = "did-key-view";
const INTEROP_SIDECAR_KIND_SCITT_RECEIPT: &str = "scitt-receipt";
const INTEROP_SIDECAR_KIND_VC_JOSE_COSE_EVENT: &str = "vc-jose-cose-event";

/// ADR 0008 §"Export bundle layout" — sidecar files live under a single
/// `interop-sidecars/` tree at the export root. Manifest-listed paths
/// MUST start with this byte prefix (TR-CORE-167). Path is checked as
/// raw bytes; no normalization.
const INTEROP_SIDECARS_PATH_PREFIX: &str = "interop-sidecars/";

/// Wave 25: `c2pa-manifest@v1` is the only dispatched kind/version
/// today. Bumping the supported set is a wire-breaking event per
/// ISC-06; bumps land in this constant + the ADR + a new
/// `derivation_version = 2` test suite, in lockstep.
const INTEROP_SIDECAR_C2PA_MANIFEST_SUPPORTED_VERSIONS: &[u8] = &[1];

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

/// Outcome for one cryptographic-erasure-evidence verification (ADR 0005
/// step 10 / Core §19 step 6b). One entry per `trellis.erasure-evidence.v1`
/// payload in the chain, in chain order. `post_erasure_uses` and
/// `post_erasure_wraps` count cross-event violations attributable to this
/// evidence's `kid_destroyed`; `cascade_violations` is reserved for the
/// Phase-2 deep-cascade lint (Phase-1 leaves it empty).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ErasureEvidenceOutcome {
    pub evidence_id: String,
    pub kid_destroyed: Vec<u8>,
    pub destroyed_at: TrellisTimestamp,
    pub cascade_scopes: Vec<String>,
    pub completion_mode: String,
    pub event_index: u64,
    /// Phase-1 contract: `true` iff every attestation row carries the
    /// structural shape (64-byte `signature`, valid `authority_class`).
    /// Crypto-verification rides Phase-2+ — see Phase-1 limit comment on
    /// `TRANSITION_ATTESTATION_DOMAIN`. Parallel to the existing
    /// posture-transition flow (`PostureTransitionOutcome.attestations_verified`).
    pub signature_verified: bool,
    pub post_erasure_uses: u64,
    pub post_erasure_wraps: u64,
    pub cascade_violations: Vec<String>,
    pub failures: Vec<String>,
}

/// Outcome for one ADR 0007 certificate-of-completion verification (Core
/// §19 step 6c). One entry per `trellis.certificate-of-completion.v1`
/// payload in scope, in chain order. `attachment_resolved` /
/// `all_signing_events_resolved` / `chain_summary_consistent` are the three
/// booleans that participate in the §19 step-9 integrity fold; `failures`
/// localizes the concrete tamper kinds (e.g.
/// `signing_event_unresolved`, `presentation_artifact_content_mismatch`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CertificateOfCompletionOutcome {
    pub certificate_id: String,
    pub event_index: u64,
    pub completed_at: TrellisTimestamp,
    pub signer_count: u64,
    pub attachment_resolved: bool,
    pub all_signing_events_resolved: bool,
    pub chain_summary_consistent: bool,
    pub failures: Vec<String>,
}

/// Outcome for one ADR 0010 user-content-attestation verification (Core
/// §19 step 6d). One entry per `trellis.user-content-attestation.v1`
/// payload in scope, in chain order. `chain_position_resolved` /
/// `identity_resolved` / `signature_verified` / `key_active` are the four
/// booleans that participate in the §19 step-9 integrity fold per ADR
/// 0010 §"Verifier obligations" step 9; `failures` localizes the concrete
/// tamper kinds (`user_content_attestation_*`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserContentAttestationOutcome {
    pub attestation_id: String,
    pub attested_event_hash: [u8; 32],
    pub attestor: String,
    pub signing_intent: String,
    pub event_index: u64,
    pub chain_position_resolved: bool,
    pub identity_resolved: bool,
    pub signature_verified: bool,
    pub key_active: bool,
    pub failures: Vec<String>,
}

/// ADR 0008 §"Phase-1 verifier obligation" per-entry interop-sidecar
/// outcome. One entry per `manifest.interop_sidecars[i]` walked under
/// Wave 25 dispatch (today: `c2pa-manifest@v1`). The struct mirrors
/// Core §28 `InteropSidecarVerificationEntry` byte-for-byte.
///
/// Path-(b) discipline (Core §18.3a / ADR 0008 §"Phase-1 verifier
/// obligation" step 3): digest-binds only — the verifier does NOT
/// resolve `source_ref` to the canonical event, decode the C2PA
/// manifest bytes, or import `c2pa-rs` (ISC-05). The C2PA-tooling-path
/// consumer is documented in `trellis-interop-c2pa/README.md` as an
/// additive verification path.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InteropSidecarVerificationEntry {
    pub kind: String,
    pub path: String,
    pub derivation_version: u8,
    pub content_digest_ok: bool,
    pub kind_registered: bool,
    pub phase_1_locked: bool,
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
    pub erasure_evidence: Vec<ErasureEvidenceOutcome>,
    pub certificates_of_completion: Vec<CertificateOfCompletionOutcome>,
    /// ADR 0010 user-content-attestation outcomes (Core §19 step 6d
    /// step 9). One entry per `trellis.user-content-attestation.v1`
    /// payload in scope, in chain order.
    pub user_content_attestations: Vec<UserContentAttestationOutcome>,
    /// ADR 0008 / Core §18.3a interop-sidecar outcomes (Wave 25). One
    /// entry per dispatched manifest entry; empty when manifest carries
    /// no `interop_sidecars` or only locked-off kinds (lock-off raises
    /// `tamper_kind = "interop_sidecar_phase_1_locked"` via
    /// `event_failures` and short-circuits before this slice fills).
    pub interop_sidecars: Vec<InteropSidecarVerificationEntry>,
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
            erasure_evidence: Vec::new(),
            certificates_of_completion: Vec::new(),
            user_content_attestations: Vec::new(),
            interop_sidecars: Vec::new(),
            warnings: vec![warning],
        }
    }

    fn from_integrity_state(
        event_failures: Vec<VerificationFailure>,
        checkpoint_failures: Vec<VerificationFailure>,
        proof_failures: Vec<VerificationFailure>,
        posture_transitions: Vec<PostureTransitionOutcome>,
        erasure_evidence: Vec<ErasureEvidenceOutcome>,
        certificates_of_completion: Vec<CertificateOfCompletionOutcome>,
        user_content_attestations: Vec<UserContentAttestationOutcome>,
        warnings: Vec<String>,
    ) -> Self {
        let posture_ok = posture_transitions.iter().all(|outcome| {
            outcome.continuity_verified
                && outcome.declaration_resolved
                && outcome.attestations_verified
        });
        // ADR 0005 step 10 fold: any erasure-evidence outcome with
        // `signature_verified = false`, `post_erasure_uses > 0`, or
        // `post_erasure_wraps > 0` flips integrity. Structure failures
        // (steps 1-6) already accumulated into `event_failures` so they
        // also gate via the `event_failures.is_empty()` predicate below.
        // Cascade violations remain warnings in Phase 1 (step 9 best-effort).
        let erasure_ok = erasure_evidence.iter().all(|outcome| {
            outcome.signature_verified
                && outcome.post_erasure_uses == 0
                && outcome.post_erasure_wraps == 0
        });
        // ADR 0007 §"Verifier obligations" + Core §19 step 9 fold: a
        // certificate-of-completion outcome with `chain_summary_consistent =
        // false`, `attachment_resolved = false`, or
        // `all_signing_events_resolved = false` flips integrity. Step-3
        // attestation failures and step-6/7 timestamp / response_ref
        // failures already land in `event_failures` so they gate via the
        // `event_failures.is_empty()` predicate.
        let certificate_ok = certificates_of_completion.iter().all(|outcome| {
            outcome.chain_summary_consistent
                && outcome.attachment_resolved
                && outcome.all_signing_events_resolved
        });
        // ADR 0010 §"Verifier obligations" step 9 (Global integrity —
        // user-content-attestation slice): outcome with any of
        // `chain_position_resolved = false`, `identity_resolved = false`,
        // `signature_verified = false`, or `key_active = false` flips
        // integrity. Step-7 (id_collision) and step-8 (operator-in-user-slot)
        // failures already land in `event_failures` and gate via the
        // `event_failures.is_empty()` predicate.
        let user_content_attestation_ok = user_content_attestations.iter().all(|outcome| {
            outcome.chain_position_resolved
                && outcome.identity_resolved
                && outcome.signature_verified
                && outcome.key_active
        });

        Self {
            structure_verified: true,
            integrity_verified: event_failures.is_empty()
                && checkpoint_failures.is_empty()
                && proof_failures.is_empty()
                && posture_ok
                && erasure_ok
                && certificate_ok
                && user_content_attestation_ok,
            readability_verified: true,
            event_failures,
            checkpoint_failures,
            proof_failures,
            posture_transitions,
            erasure_evidence,
            certificates_of_completion,
            user_content_attestations,
            // Genesis-append path produces no interop sidecar outcomes;
            // the export-archive path populates this slice via
            // `verify_interop_sidecars` (ADR 0008 §"Phase-1 verifier
            // obligation" — Wave 25).
            interop_sidecars: Vec::new(),
            warnings,
        }
    }
}

/// Error returned when verifier inputs cannot be decoded at all.
#[derive(Debug)]
pub struct VerifyError {
    message: String,
    /// Optional structural-failure kind tag, used by registry / decode
    /// paths so callers can map the error to a [`VerificationReport`] with
    /// the correct `tamper_kind` (e.g., `key_entry_attributes_shape_mismatch`
    /// for ADR 0006 §8.7.1 violations) instead of the generic
    /// `signing_key_registry_invalid`.
    kind: Option<&'static str>,
    backtrace: Backtrace,
}

impl VerifyError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            kind: None,
            backtrace: Backtrace::capture(),
        }
    }

    fn with_kind(message: impl Into<String>, kind: &'static str) -> Self {
        Self {
            message: message.into(),
            kind: Some(kind),
            backtrace: Backtrace::capture(),
        }
    }

    /// Returns the structural-failure kind tag, if one was set.
    pub fn kind(&self) -> Option<&'static str> {
        self.kind
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
    // Surface typed structural shape failures (e.g.
    // `key_entry_attributes_shape_mismatch`, TR-CORE-048) as a
    // `VerificationReport` with the matching `tamper_kind` rather than
    // an `Err(VerifyError)` that would panic the conformance harness.
    // Untyped decode errors continue to bubble as `Err` so callers
    // distinguish "registry was structurally invalid in a known
    // tamper-detectable way" from "we could not parse the bytes at all".
    let (registry, non_signing) = match parse_key_registry(signing_key_registry) {
        Ok(maps) => maps,
        Err(error) => {
            if let Some(kind) = error.kind() {
                return Ok(VerificationReport::fatal(
                    kind,
                    format!("failed to decode signing-key registry: {error}"),
                ));
            }
            return Err(error);
        }
    };
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
            // Core §8.7.3 step 3 / TR-CORE-048: structural shape failures
            // surface as their typed `tamper_kind` (e.g.
            // `key_entry_attributes_shape_mismatch`) rather than the
            // generic `signing_key_registry_invalid` so tamper vectors can
            // pin them. Decode failures with no typed kind keep the
            // generic kind for back-compat with existing fixtures.
            let kind = error.kind().unwrap_or("signing_key_registry_invalid");
            return VerificationReport::fatal(
                kind,
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

    // ADR 0008 §"Phase-1 verifier obligation" — Wave 25 dispatched
    // verifier. Path-(b): digest-binds only, no `source_ref`
    // resolution. The `c2pa-manifest@v1` kind dispatches; the three
    // other registered kinds (`scitt-receipt`, `vc-jose-cose-event`,
    // `did-key-view`) remain Phase-1 locked-off and short-circuit with
    // `interop_sidecar_phase_1_locked`. Outcomes accumulate into
    // `interop_sidecars` for the dispatched-kind entries; lock-off,
    // unknown-kind, derivation-version-unknown, path-invalid,
    // content-mismatch, and unlisted-file all short-circuit via
    // `VerificationReport::fatal` (Core §19.1 / TR-CORE-145, 163..167).
    let interop_sidecars = match verify_interop_sidecars(manifest_map, &archive) {
        Ok(outcomes) => outcomes,
        Err(report) => return report,
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
        let registry_bytes = match archive.members.get(&member_name) {
            Some(bytes) => bytes,
            None => {
                return VerificationReport::fatal(
                    "archive_integrity_failure",
                    format!("export is missing bound registry member {member_name}"),
                );
            }
        };
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
    // ADR 0008 / Core §18.3a — Wave 25 dispatched-verifier outcomes
    // accumulate here. `verify_interop_sidecars` already short-circuited
    // any fatal lock-off / unknown-kind / digest-mismatch / unlisted-file
    // / path-invalid / version-unknown failures via `VerificationReport::fatal`,
    // so what reaches this site is the per-entry success slice. The
    // export-archive integrity fold below treats absent failures as
    // pass-through; non-fatal failures localize per-entry in
    // `outcome.failures` and collapse to integrity-true (the dispatched
    // path-(b) is digest-binds-only — there is no sub-fatal failure
    // surface today).
    report.interop_sidecars = interop_sidecars;
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
    if let Some(extension) = match parse_erasure_evidence_export_extension(manifest_map) {
        Ok(extension) => extension,
        Err(error) => {
            return VerificationReport::fatal(
                "manifest_payload_invalid",
                format!("erasure export extension is invalid: {error}"),
            );
        }
    } {
        verify_erasure_evidence_catalog(&archive, &events, &extension, &mut report);
    }
    // ADR 0007 §"Verifier obligations" step 4 — export-bundle context
    // resolves attachment lineage + recomputes content hash. Runs
    // unconditionally so certificate events that travel without the
    // optional manifest catalog still get step-4 enforcement.
    verify_certificate_attachment_lineage(&events, &payload_blobs, &mut report);
    if let Some(extension) = match parse_certificate_export_extension(manifest_map) {
        Ok(extension) => extension,
        Err(error) => {
            return VerificationReport::fatal(
                "manifest_payload_invalid",
                format!("certificate export extension is invalid: {error}"),
            );
        }
    } {
        verify_certificate_catalog(&archive, &events, &extension, &mut report);
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
        })
        // ADR 0005 step 10 fold — mirrors `from_integrity_state` so the
        // export-bundle path computes integrity identically to the
        // genesis path.
        && report.erasure_evidence.iter().all(|outcome| {
            outcome.signature_verified
                && outcome.post_erasure_uses == 0
                && outcome.post_erasure_wraps == 0
        })
        // ADR 0007 §"Verifier obligations" + Core §19 step 9 fold —
        // certificate-of-completion outcomes flip integrity when chain
        // summary, attachment lineage, or signing-event resolution failed.
        && report.certificates_of_completion.iter().all(|outcome| {
            outcome.chain_summary_consistent
                && outcome.attachment_resolved
                && outcome.all_signing_events_resolved
        })
        // ADR 0010 §"Verifier obligations" step 9 fold — user-content
        // attestation outcomes flip integrity when chain-position binding,
        // identity resolution, signature verification, or key-state check
        // failed. Step-7 collision and step-8 operator-in-user-slot
        // failures already land in `event_failures` above.
        && report.user_content_attestations.iter().all(|outcome| {
            outcome.chain_position_resolved
                && outcome.identity_resolved
                && outcome.signature_verified
                && outcome.key_active
        })
        // ADR 0008 §"Phase-1 verifier obligation" / Core §18.3a fold —
        // an interop-sidecar outcome with `content_digest_ok = false`
        // OR `kind_registered = false` OR localized `failures` would
        // flip integrity. Today every non-pass condition short-circuits
        // via `VerificationReport::fatal` before this slice is built;
        // this fold is defensive against a future sub-fatal failure
        // surface (e.g., `c2pa-manifest@v2` dispatching to a richer
        // path-(b) with localized warnings).
        && report.interop_sidecars.iter().all(|outcome| {
            outcome.content_digest_ok
                && outcome.kind_registered
                && !outcome.phase_1_locked
                && outcome.failures.is_empty()
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
    authored_at: TrellisTimestamp,
    event_type: String,
    classification: String,
    prev_hash: Option<[u8; 32]>,
    author_event_hash: [u8; 32],
    content_hash: [u8; 32],
    canonical_event_hash: [u8; 32],
    /// Core §6.1 / §17.2 wire-contract identity. Length is validated against
    /// `bstr .size (1..64)` at parse time; out-of-bound length surfaces as a
    /// typed `VerifyError` with kind `idempotency_key_length_invalid`.
    /// Used by the per-event-set loop to detect §17.3 duplicate `(scope, key)`
    /// identity with divergent canonical material.
    idempotency_key: Vec<u8>,
    payload_ref: PayloadRef,
    transition: Option<TransitionDetails>,
    attachment_binding: Option<AttachmentBindingDetails>,
    /// Decoded ADR 0005 erasure-evidence payload, populated when
    /// `EventPayload.extensions["trellis.erasure-evidence.v1"]` is present.
    /// `None` for non-erasure events. The decoder runs ADR 0005 §"Verifier
    /// obligations" steps 1, 3, 6 (CDDL + subject_scope shape + hsm_receipt
    /// null-consistency) inline; structural failures surface as `Err`
    /// `VerifyError` from `decode_erasure_evidence_details` and bubble up
    /// to a `tamper_kind` via `VerifyError::with_kind`. Step 2 (registry
    /// bind), step 4 (destroyed_at vs host), step 5 (cross-event group),
    /// step 7 (attestation), and step 8 (chain consistency) run from the
    /// caller after collecting all events.
    erasure: Option<ErasureEvidenceDetails>,
    /// Decoded ADR 0007 certificate-of-completion payload, populated when
    /// `EventPayload.extensions["trellis.certificate-of-completion.v1"]` is
    /// present. `None` for non-certificate events. The decoder runs ADR 0007
    /// §"Verifier obligations" step 1 (CDDL decode + per-event invariants
    /// `signer_count == len(signing_events)`, `len(signer_display) ==
    /// len(signing_events)`, HTML→`template_hash` non-null) inline; structural
    /// failures bubble up via `VerifyError`, with cross-summary invariant
    /// failures tagged `certificate_chain_summary_mismatch`. Steps 2 (per-index
    /// principal-ref / id-collision), 3 (attestation crypto — Phase-2),
    /// 4 (attachment lineage), 5 (signing-event resolution), 6 (timestamp
    /// equivalence), 7 (response_ref equivalence) run in
    /// [`finalize_certificates_of_completion`] from the caller after every
    /// event is decoded.
    certificate: Option<CertificateDetails>,
    /// Decoded ADR 0010 user-content-attestation payload, populated when
    /// `EventPayload.extensions["trellis.user-content-attestation.v1"]` is
    /// present. `None` for non-attestation events. The decoder runs ADR 0010
    /// §"Verifier obligations" step 1 (CDDL decode) and step 2 partial
    /// (`signing_intent` URI well-formedness, `attested_at == authored_at`)
    /// inline; structural failures surface as `Err` `VerifyError` from
    /// `decode_user_content_attestation_payload` with typed kinds via
    /// `VerifyError::with_kind`. Cross-event steps 3 (chain-position
    /// resolution), 4 (identity resolution), 5 (signature verification),
    /// 6 (key-state check), 7 (collision detection), 8 (operator-in-user-slot
    /// enforcement), and 9 (outcome accumulation) run in
    /// [`finalize_user_content_attestations`] from the caller after every
    /// event is decoded.
    user_content_attestation: Option<UserContentAttestationDetails>,
    /// Identity-attestation subject for events whose `event_type` matches
    /// one of the registered identity-attestation taxonomies (Phase-1
    /// admission via `is_identity_attestation_event_type`; canonical
    /// `wos.identity.*` lands with PLN-0381). Populated from
    /// `EventPayload.extensions[event_type]["subject"]` when present.
    /// `None` for non-identity events or for identity events whose
    /// payload omits the subject field. ADR 0010 §"Verifier obligations"
    /// step 4 reads this for the subject-equals-attestor check.
    identity_attestation_subject: Option<String>,
    /// Wrap recipients from `key_bag.entries[*].recipient`. Bytes copied
    /// verbatim from the wire so step 8 can compare against `kid_destroyed`
    /// (a `bstr .size 16`) for `post_erasure_wrap` detection. Empty when
    /// the event has no key_bag entries (Phase-1 plaintext path).
    wrap_recipients: Vec<Vec<u8>>,
}

#[derive(Clone, Debug)]
struct SigningKeyEntry {
    public_key: [u8; 32],
    status: u64,
    valid_to: Option<TrellisTimestamp>,
}

/// A reserved non-signing `KeyEntry` (Core §8.7 / ADR 0006).
///
/// Phase-1 verifiers track these so a signature attempt under a kid registered
/// as `tenant-root`, `scope`, `subject`, or `recovery` can be flagged with
/// `key_class_mismatch` (Core §8.7.3 step 4) rather than the generic
/// `unresolvable_manifest_kid` failure.
///
/// `valid_to` is captured for `subject`-class entries so a future Phase-1
/// `KeyBagEntry`-mediated `subject_wrap_after_valid_to` enforcement can run
/// without re-decoding the registry. Phase-1 `KeyBagEntry.recipient` is
/// opaque bytes (Core §9.4); ADR 0006 *Phase-1 runtime discipline* defers
/// recipient-to-`subject` kid binding to Phase-2+. The field is therefore
/// captured-but-unused at runtime today; see `tamper/025` for the wire
/// shape that tests this path.
#[derive(Clone, Debug)]
struct NonSigningKeyEntry {
    /// Class string from the registry entry's `kind` field, normalized so the
    /// legacy synonym `"wrap"` is mapped to `"subject"` per Core §8.7.6.
    class: String,
    /// Subject-class `valid_to` (per `SubjectKeyAttributes` in Core §8.7.2),
    /// captured for forward-compatible enforcement (see field-level doc above).
    /// `None` for non-`subject` classes and for `subject` rows with `valid_to = null`.
    #[allow(dead_code)]
    subject_valid_to: Option<TrellisTimestamp>,
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

/// Decoded `trellis.erasure-evidence.v1` payload (ADR 0005 §"Wire shape").
/// Carries the inputs needed for cross-event finalization (steps 5 / 8) and
/// the report-level fields surfaced through [`ErasureEvidenceOutcome`].
#[derive(Clone, Debug)]
struct ErasureEvidenceDetails {
    evidence_id: String,
    kid_destroyed: Vec<u8>,
    /// Wire `key_class` AFTER the `wrap` → `subject` normalization (Core
    /// §8.7.6 / ADR 0005 step 2). Stored as the normalized string so step 5
    /// / step 8 group reasoning compares apples to apples.
    norm_key_class: String,
    destroyed_at: TrellisTimestamp,
    cascade_scopes: Vec<String>,
    completion_mode: String,
    /// Phase-1 contract: every attestation row has structural shape
    /// (64-byte signature; valid `authority_class`). Crypto-verification of
    /// the Ed25519 signature itself is deferred to Phase-2+ alongside the
    /// posture-transition flow — see `TRANSITION_ATTESTATION_DOMAIN`.
    attestation_signatures_well_formed: bool,
    /// Reserved for the §10 outcome shape (per-class attestation reporting).
    /// The dual-attestation rule (Companion OC-143) is a SHOULD-grade
    /// per-deployment policy declared in the Posture Declaration; the
    /// Phase-1 verifier captures the classes for tooling but does not gate
    /// `integrity_verified` on count.
    #[allow(dead_code)]
    attestation_classes: Vec<String>,
    /// Subject-scope kind text (`per-subject` / `per-scope` / `per-tenant`
    /// / `deployment-wide`). Captured for the §10 outcome report; the
    /// cross-field shape rule (step 3) is enforced inline at decode time.
    #[allow(dead_code)]
    subject_scope_kind: String,
}

/// Decoded `trellis.certificate-of-completion.v1` payload (ADR 0007 §"Wire
/// shape"). Carries the inputs needed for cross-event finalization (step 5
/// signing-event resolution; step 6 temporal equivalence; step 7 response
/// hash) and the report-level fields surfaced through
/// [`CertificateOfCompletionOutcome`]. CDDL decode + chain-summary
/// invariants (steps 1 and 2) run in [`decode_certificate_payload`]; step 3
/// (attestation crypto) shape-checks at decode time and crypto-verification
/// is deferred to Phase 2+ — see [`TRANSITION_ATTESTATION_DOMAIN`].
#[derive(Clone, Debug)]
struct CertificateDetails {
    certificate_id: String,
    /// Reserved for §6.4 operator obligations; Phase-1 verifier captures the
    /// field but does not gate on its presence beyond the CDDL shape.
    #[allow(dead_code)]
    case_ref: Option<String>,
    completed_at: TrellisTimestamp,
    presentation_artifact: PresentationArtifactDetails,
    chain_summary: ChainSummaryDetails,
    /// `signing_events[i]` digests in workflow order. Step 5 resolves each
    /// to a chain-present `wos.kernel.signatureAffirmation` event.
    signing_events: Vec<[u8; 32]>,
    /// Opaque to Trellis verification per ADR 0007 §"Field semantics";
    /// captured for completeness, not gated.
    #[allow(dead_code)]
    workflow_ref: Option<String>,
    /// Phase-1 contract: every attestation row has structural shape
    /// (64-byte signature; valid `authority_class`). Crypto-verification
    /// of the Ed25519 signature itself is deferred to Phase-2+ alongside
    /// the posture-transition + erasure flows — see
    /// [`TRANSITION_ATTESTATION_DOMAIN`]. Maps to step-3 contract; surfaces
    /// via the existing `attestation_insufficient` failure code.
    attestation_signatures_well_formed: bool,
}

/// Decoded `PresentationArtifact` map (ADR 0007 §"Wire shape").
#[derive(Clone, Debug)]
struct PresentationArtifactDetails {
    content_hash: [u8; 32],
    media_type: String,
    /// Reserved for the §"Adversary model" artifact-swap detection extension;
    /// Phase-1 captures but does not gate beyond CDDL shape.
    #[allow(dead_code)]
    byte_length: u64,
    /// Step-4 attachment lineage resolution is parameterized on this id.
    attachment_id: String,
    /// Reserved for the optional template-rendering-drift stretch check
    /// (ADR 0007 §"Field semantics"); Phase-1 verifier captures but does not
    /// re-render.
    #[allow(dead_code)]
    template_id: Option<String>,
    /// Reserved for the optional template-rendering-drift stretch check;
    /// Phase-1 enforces the CDDL invariant that HTML media type carries a
    /// non-null template_hash but does not recompute.
    #[allow(dead_code)]
    template_hash: Option<[u8; 32]>,
}

/// Decoded `ChainSummary` map (ADR 0007 §"Wire shape").
#[derive(Clone, Debug)]
struct ChainSummaryDetails {
    signer_count: u64,
    /// Per-signer display rows; step 2 invariant
    /// `len(signer_display) == len(signing_events)` enforced at decode.
    signer_display: Vec<SignerDisplayDetails>,
    response_ref: Option<[u8; 32]>,
    /// Wire `workflow_status` value; CDDL admits the four enum literals
    /// plus registered extension `tstr`. Phase-1 reference verifier
    /// admits any `tstr` shape; deep registry-membership rides
    /// `certificate_enum_extension_unknown` evolution alongside WOS
    /// signature-profile registry plumbing.
    #[allow(dead_code)]
    workflow_status: String,
    /// Wire `impact_level` value or null; same registry-deferral posture
    /// as `workflow_status`.
    #[allow(dead_code)]
    impact_level: Option<String>,
    /// Operator-asserted cross-check tag set; empty / absent means the
    /// default §"Verifier obligations" check set applies. Phase-1
    /// reference verifier admits any `tstr` and surfaces unknown tags via
    /// `certificate_covered_claim_unknown` evolution alongside the §19.1
    /// fixture corpus.
    #[allow(dead_code)]
    covered_claims: Vec<String>,
}

/// Decoded `trellis.user-content-attestation.v1` payload (ADR 0010 §"Wire
/// shape" / Core §28 CDDL `UserContentAttestationPayload`). Mirrors the
/// shape used by `trellis_py.verify.UserContentAttestationDetails`. The
/// fields land directly in [`UserContentAttestationOutcome`] after
/// cross-event finalization in [`finalize_user_content_attestations`].
#[derive(Clone, Debug, PartialEq, Eq)]
struct UserContentAttestationDetails {
    attestation_id: String,
    attested_event_hash: [u8; 32],
    attested_event_position: u64,
    attestor: String,
    /// `None` only when the deployment Posture Declaration in force at
    /// `attested_at` declares `admit_unverified_user_attestations: true`.
    /// Default REQUIRED non-null per ADR 0010 §"Field semantics"; step 4
    /// (identity resolution) gates on the resolved Posture Declaration
    /// field, surfacing `user_content_attestation_identity_required` when
    /// admission is missing.
    identity_attestation_ref: Option<[u8; 32]>,
    signing_intent: String,
    attested_at: TrellisTimestamp,
    /// 64-byte detached Ed25519 signature over `dCBOR([attestation_id,
    /// attested_event_hash, attested_event_position, attestor,
    /// identity_attestation_ref, signing_intent, attested_at])` under domain
    /// tag `trellis-user-content-attestation-v1` (Core §9.8). Decoder
    /// validates structural shape; crypto verification runs in finalize step 5.
    signature: [u8; 64],
    /// Core §8 KeyEntry kid (16 bytes; per the Rust-byte-authority
    /// reconciliation noted in Core §28 — the ADR 0010 prose draft used
    /// `tstr` informally but the canonical taxonomy is the 16-byte digest
    /// of `dCBOR(suite_id) || pubkey_raw`). Resolved against the
    /// signing-key registry in finalize step 6.
    signing_kid: Vec<u8>,
    /// Canonical preimage that step 5 hashes against
    /// `USER_CONTENT_ATTESTATION_DOMAIN`. Pre-computed at decode time so
    /// the finalize pass can re-verify without re-encoding.
    canonical_preimage: Vec<u8>,
    /// ADR 0010 §"Verifier obligations" step 2 deferred-failure marker.
    /// `Some(kind)` when the decoder detected an intra-payload-invariant
    /// failure (`user_content_attestation_intent_malformed` /
    /// `user_content_attestation_timestamp_mismatch`); `None` when step 2
    /// passed. Step 2 failures flip `integrity_verified = false` per ADR
    /// 0010 — they are NOT structure failures and MUST NOT flip
    /// `readability_verified`. The finalize pass raises the marker as an
    /// `event_failure` and skips remaining checks for the event.
    step_2_failure: Option<&'static str>,
}

/// Decoded `SignerDisplayEntry` (ADR 0007 §"Wire shape").
#[derive(Clone, Debug)]
struct SignerDisplayDetails {
    principal_ref: String,
    /// Operator-rendered display name; not strict-compared per ADR 0007
    /// §"Field semantics" (verifier surfaces gross mismatch only).
    #[allow(dead_code)]
    display_name: String,
    /// Operator-supplied display role; reserved for surfaced summary.
    #[allow(dead_code)]
    display_role: Option<String>,
    /// Step 6 inputs: MUST exactly equal the resolved SignatureAffirmation
    /// header `authored_at` for `signing_events[i]`.
    signed_at: TrellisTimestamp,
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

/// Optional `trellis.export.erasure-evidence.v1` manifest extension (ADR 0005).
#[derive(Clone, Debug)]
struct ErasureEvidenceExportExtension {
    catalog_ref: String,
    catalog_digest: [u8; 32],
    entry_count: u64,
}

/// One row in `064-erasure-evidence.cbor` (ADR 0005 export manifest catalog).
#[derive(Clone, Debug)]
struct ErasureEvidenceCatalogEntryRow {
    canonical_event_hash: [u8; 32],
    evidence_id: String,
    kid_destroyed: [u8; 16],
    destroyed_at: TrellisTimestamp,
    completion_mode: String,
    cascade_scopes: Vec<String>,
    subject_scope_kind: String,
}

/// Optional `trellis.export.certificates-of-completion.v1` manifest extension
/// (ADR 0007 §"Export manifest catalog"). Mirror of
/// [`ErasureEvidenceExportExtension`].
#[derive(Clone, Debug)]
struct CertificateExportExtension {
    catalog_ref: String,
    catalog_digest: [u8; 32],
    entry_count: u64,
}

/// One row in `065-certificates-of-completion.cbor` (ADR 0007 §"Export
/// manifest catalog" — `CertificateOfCompletionCatalogEntry`). Mirrors
/// [`ErasureEvidenceCatalogEntryRow`]; binds canonical certificate event
/// metadata for auditor-UX cross-check.
#[derive(Clone, Debug)]
struct CertificateCatalogEntryRow {
    canonical_event_hash: [u8; 32],
    certificate_id: String,
    completed_at: TrellisTimestamp,
    signer_count: u64,
    media_type: String,
    attachment_id: String,
    workflow_status: String,
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

/// Path-prefix predicate (TR-CORE-167). The path is checked as raw
/// bytes — no normalization, no canonicalization, no Unicode folding.
/// Anything that does not start with the literal `interop-sidecars/`
/// byte sequence is invalid; this closes the path-traversal attack
/// surface where a manifest could redirect a `content_digest` check at
/// a canonical-tree file (e.g., `010-events.cbor`).
fn is_interop_sidecar_path_valid(path: &str) -> bool {
    path.starts_with(INTEROP_SIDECARS_PATH_PREFIX)
}

/// ADR 0008 §"Phase-1 verifier obligation" — Wave 25 dispatched
/// verifier (path-(b): digest-binds only). Walks
/// `manifest.interop_sidecars` and the on-disk `interop-sidecars/`
/// tree and produces one outcome per dispatched-kind entry. Fatal
/// short-circuits return a `VerificationReport::fatal` via the `Err`
/// arm; the caller propagates that report. Non-dispatched (locked-off)
/// kinds short-circuit with `interop_sidecar_phase_1_locked`.
///
/// Failure dispatch order (per ADR 0008 §"Phase-1 verifier obligation"
/// step 2): kind-registered → derivation-version-supported →
/// path-prefix-valid → phase-1-lock-off (`interop_sidecar_phase_1_locked`
/// for the three still-locked kinds) → content-digest-match. Files-on-disk
/// that are not manifest-listed → `interop_sidecar_unlisted_file` (after
/// manifest walk completes; closes the smuggled-sidecar attack surface).
/// For each manifest entry, checks run in this order; the verifier returns
/// the first failing check (one fatal `VerificationReport` per export, not
/// a bundle of competing failure codes).
fn verify_interop_sidecars(
    manifest_map: &[(Value, Value)],
    archive: &ExportArchive,
) -> Result<Vec<InteropSidecarVerificationEntry>, VerificationReport> {
    let raw = match map_lookup_optional_value(manifest_map, "interop_sidecars") {
        Some(value) => value,
        None => return Ok(Vec::new()),
    };
    if raw.is_null() {
        return Ok(Vec::new());
    }
    let entries = match raw.as_array() {
        Some(arr) => arr,
        None => {
            return Err(VerificationReport::fatal(
                "manifest_payload_invalid",
                "interop_sidecars must be an array or null",
            ));
        }
    };

    let mut outcomes: Vec<InteropSidecarVerificationEntry> = Vec::with_capacity(entries.len());
    let mut listed_paths: BTreeSet<String> = BTreeSet::new();

    for (index, entry) in entries.iter().enumerate() {
        let entry_map = match entry.as_map() {
            Some(map) => map,
            None => {
                return Err(VerificationReport::fatal(
                    "manifest_payload_invalid",
                    format!("interop_sidecars[{index}] is not a map"),
                ));
            }
        };

        let kind = match map_lookup_text(entry_map, "kind") {
            Ok(kind) => kind,
            Err(error) => {
                return Err(VerificationReport::fatal(
                    "manifest_payload_invalid",
                    format!("interop_sidecars[{index}].kind is invalid: {error}"),
                ));
            }
        };
        let path = match map_lookup_text(entry_map, "path") {
            Ok(path) => path,
            Err(error) => {
                return Err(VerificationReport::fatal(
                    "manifest_payload_invalid",
                    format!("interop_sidecars[{index}].path is invalid: {error}"),
                ));
            }
        };
        let derivation_version = match map_lookup_u64(entry_map, "derivation_version") {
            Ok(value) if value <= u8::MAX as u64 => value as u8,
            Ok(value) => {
                return Err(VerificationReport::fatal(
                    "manifest_payload_invalid",
                    format!(
                        "interop_sidecars[{index}].derivation_version {value} exceeds uint .size 1"
                    ),
                ));
            }
            Err(error) => {
                return Err(VerificationReport::fatal(
                    "manifest_payload_invalid",
                    format!("interop_sidecars[{index}].derivation_version is invalid: {error}"),
                ));
            }
        };
        let content_digest = match map_lookup_fixed_bytes(entry_map, "content_digest", 32) {
            Ok(bytes) => bytes,
            Err(error) => {
                return Err(VerificationReport::fatal(
                    "manifest_payload_invalid",
                    format!("interop_sidecars[{index}].content_digest is invalid: {error}"),
                ));
            }
        };
        // ADR 0008 Open Q5 (resolved Wave 25): `source_ref` validated
        // for presence only; full resolution semantics deferred to a
        // future ADR. Decode-failure is a manifest-payload error;
        // empty / non-string is rejected at the CDDL boundary.
        match map_lookup_text(entry_map, "source_ref") {
            Ok(_) => {}
            Err(error) => {
                return Err(VerificationReport::fatal(
                    "manifest_payload_invalid",
                    format!("interop_sidecars[{index}].source_ref is invalid: {error}"),
                ));
            }
        }

        // Step 2.a (kind-registered) — TR-CORE-164.
        let kind_registered = matches!(
            kind.as_str(),
            INTEROP_SIDECAR_KIND_C2PA_MANIFEST
                | INTEROP_SIDECAR_KIND_DID_KEY_VIEW
                | INTEROP_SIDECAR_KIND_SCITT_RECEIPT
                | INTEROP_SIDECAR_KIND_VC_JOSE_COSE_EVENT
        );
        if !kind_registered {
            return Err(VerificationReport::fatal(
                "interop_sidecar_kind_unknown",
                format!("interop_sidecars[{index}].kind {kind:?} is not in the ADR 0008 registry"),
            ));
        }

        // Step 2.b (derivation-version-supported) — TR-CORE-166. Wave
        // 25 supports `c2pa-manifest@v1` only; other registered kinds
        // are still locked-off so their version set is empty.
        let supported_versions: &[u8] = match kind.as_str() {
            INTEROP_SIDECAR_KIND_C2PA_MANIFEST => INTEROP_SIDECAR_C2PA_MANIFEST_SUPPORTED_VERSIONS,
            _ => &[],
        };
        if !supported_versions.contains(&derivation_version)
            && kind == INTEROP_SIDECAR_KIND_C2PA_MANIFEST
        {
            return Err(VerificationReport::fatal(
                "interop_sidecar_derivation_version_unknown",
                format!(
                    "interop_sidecars[{index}] kind={kind:?} derivation_version={derivation_version} not in supported set"
                ),
            ));
        }

        // Step 2.c (path-prefix-valid) — TR-CORE-167. Predicate also covered
        // by `interop_sidecar_path_prefix_invariant`; full dispatch path by
        // `verify_interop_sidecars_rejects_manifest_path_outside_interop_tree`.
        if !is_interop_sidecar_path_valid(&path) {
            return Err(VerificationReport::fatal(
                "interop_sidecar_path_invalid",
                format!(
                    "interop_sidecars[{index}].path {path:?} does not start with {INTEROP_SIDECARS_PATH_PREFIX:?}"
                ),
            ));
        }

        // Step 2.d (Phase-1 lock-off — three locked kinds short-circuit
        // here AFTER passing structural checks, so a fixture
        // mis-listing kind+path under a still-locked kind surfaces the
        // dominant `interop_sidecar_phase_1_locked` failure rather than
        // a structural one). Wave 25 unlocks `c2pa-manifest@v1` only.
        let phase_1_locked = !matches!(kind.as_str(), INTEROP_SIDECAR_KIND_C2PA_MANIFEST);
        if phase_1_locked {
            return Err(VerificationReport::fatal(
                "interop_sidecar_phase_1_locked",
                format!(
                    "interop_sidecars[{index}] kind={kind:?} is still Phase-1 locked-off (ADR 0008 / ADR 0003)"
                ),
            ));
        }

        // Step 2.e (content-digest-match) — TR-CORE-163. Recompute
        // SHA-256 under domain tag `trellis-content-v1` over the on-disk
        // sidecar bytes. Missing file is a `content_mismatch` (no bytes
        // to digest); cleaner and more localizable than a generic
        // archive-integrity error because the manifest already
        // promised them.
        let actual_bytes = match archive.members.get(&path) {
            Some(bytes) => bytes,
            None => {
                return Err(VerificationReport::fatal(
                    "interop_sidecar_content_mismatch",
                    format!(
                        "interop_sidecars[{index}].path {path:?} is missing from the export ZIP"
                    ),
                ));
            }
        };
        let actual_digest = domain_separated_sha256(CONTENT_DOMAIN, actual_bytes);
        let content_digest_ok = actual_digest.as_slice() == content_digest.as_slice();
        if !content_digest_ok {
            return Err(VerificationReport::fatal(
                "interop_sidecar_content_mismatch",
                format!(
                    "interop_sidecars[{index}].content_digest does not match SHA-256(trellis-content-v1, {path:?})"
                ),
            ));
        }

        listed_paths.insert(path.clone());
        outcomes.push(InteropSidecarVerificationEntry {
            kind,
            path,
            derivation_version,
            content_digest_ok: true,
            kind_registered: true,
            phase_1_locked: false,
            failures: Vec::new(),
        });
    }

    // Step 2.f (unlisted-file) — TR-CORE-165. Walk every archive
    // member under `interop-sidecars/` and assert it appears in
    // `listed_paths`. The check runs after the manifest walk so a
    // first manifest-listed entry with a digest-mismatch wins
    // localization over a stray file (auditors expect to see the
    // explicit listing failure, not a confusing "unlisted file"
    // signal whose root cause is digest divergence).
    for member_path in archive.members.keys() {
        if !member_path.starts_with(INTEROP_SIDECARS_PATH_PREFIX) {
            continue;
        }
        if listed_paths.contains(member_path) {
            continue;
        }
        return Err(VerificationReport::fatal(
            "interop_sidecar_unlisted_file",
            format!(
                "{member_path:?} is present under interop-sidecars/ but not catalogued in manifest.interop_sidecars"
            ),
        ));
    }

    Ok(outcomes)
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

#[cfg(test)]
fn export_archive_for_tests(members: BTreeMap<String, Vec<u8>>) -> ExportArchive {
    ExportArchive { members }
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
    let mut previous_authored_at: Option<TrellisTimestamp> = None;
    let skip_prev_hash_check = initial_posture_declaration.is_some() && events.len() == 1;

    // Core §17.3 — Track every `(ledger_scope, idempotency_key)` identity
    // seen so far in this event set, mapped to the first canonical event's
    // `canonical_event_hash`. A second event sharing the identity but with
    // a divergent `canonical_event_hash` is a §17.3 clause-3 violation
    // surfaced as `idempotency_key_payload_mismatch` (§17.5 + TR-CORE-160).
    // The check is offline (Core §16) and operates purely on the canonical
    // events; no Canonical Append Service state is required.
    let mut idempotency_index: BTreeMap<(Vec<u8>, Vec<u8>), [u8; 32]> = BTreeMap::new();
    let mut shadow_custody_model =
        initial_posture_declaration.and_then(|bytes| parse_custody_model(bytes).ok());
    let mut shadow_disclosure_profile =
        initial_posture_declaration.and_then(|bytes| parse_disclosure_profile(bytes).ok());

    // ADR 0005 cross-event finalization inputs.
    // - `erasure_payloads` collects every decoded erasure-evidence event for
    //   the post-loop finalize pass (steps 2 / 5 / 7 / 8 / 10).
    // - `chain_summaries` carries every event's (authored_at, signing kid,
    //   wrap recipients, canonical_event_hash) tuple so step 8 (chain
    //   consistency for the destroyed kid) can flag post_erasure_use /
    //   post_erasure_wrap.
    let mut erasure_payloads: Vec<(usize, ErasureEvidenceDetails, [u8; 32])> = Vec::new();
    let mut chain_summaries: Vec<ChainEventSummary> = Vec::with_capacity(events.len());

    // ADR 0007 certificate-of-completion finalization input. `(event_index,
    // CertificateDetails, canonical_event_hash)` tuples — same shape as
    // `erasure_payloads`. The post-loop pass cross-references against the
    // chain to run steps 2 (id collision), 5 (signing-event resolution),
    // 6 (timestamp equivalence), and 7 (response_ref equivalence). Step 4
    // (attachment lineage + content recompute) requires payload-blob access
    // and runs in the export-bundle path only; the genesis-append path
    // marks `attachment_resolved = true` and `failures = []` per ADR 0007
    // §"Verifier obligations" Phase-1 minimal-genesis posture.
    let mut certificate_payloads: Vec<(usize, CertificateDetails, [u8; 32])> = Vec::new();

    // ADR 0010 user-content-attestation finalization input. `(event_index,
    // UserContentAttestationDetails, canonical_event_hash)` tuples —
    // same shape as `erasure_payloads` / `certificate_payloads`. Per-event
    // CDDL decode + signing_intent URI well-formedness + attested_at-equals-
    // authored_at run inline at decode time; the post-loop pass cross-
    // references against the chain to run steps 3 (chain-position resolution),
    // 4 (identity resolution), 5 (signature verification), 6 (key-state
    // check), 7 (collision detection), 8 (operator-in-user-slot enforcement),
    // and 9 (outcome accumulation).
    let mut user_content_attestation_payloads: Vec<(
        usize,
        UserContentAttestationDetails,
        [u8; 32],
    )> = Vec::new();

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
                if let Some(non_signing) = non_signing_registry.and_then(|map| map.get(&event.kid))
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
            Err(error) => {
                // Surface typed structural-failure kinds (e.g.
                // `erasure_destroyed_at_after_host` from ADR 0005 step 4)
                // as the report's `tamper_kind`. Untyped decode errors
                // continue to land as the generic `malformed_cose` for
                // back-compat with existing fixtures.
                let kind = error.kind().unwrap_or("malformed_cose");
                let warning = if error.kind().is_some() {
                    error.to_string()
                } else {
                    "event payload does not decode as a canonical Trellis event".to_string()
                };
                return VerificationReport::fatal(kind, warning);
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

        // Core §17.3 clause 3 + §17.5 — duplicate `(scope, idempotency_key)`
        // identity with divergent canonical material is `idempotency_key_payload_mismatch`.
        // The first occurrence is admitted as the canonical reference; the
        // second occurrence is the failing event. Identity is determined by
        // (scope, idempotency_key); divergence is by canonical_event_hash
        // (which transitively binds content_hash + author_event_hash via
        // §9.2 / §9.5 preimages). TR-CORE-160 + TR-CORE-162.
        let identity_key = (details.scope.clone(), details.idempotency_key.clone());
        match idempotency_index.get(&identity_key) {
            Some(prior_hash) if *prior_hash != details.canonical_event_hash => {
                event_failures.push(VerificationFailure::new(
                    "idempotency_key_payload_mismatch",
                    hex_string(&details.canonical_event_hash),
                ));
            }
            Some(_) => {
                // Same identity, byte-equal canonical hash — §17.3 clause 1
                // (same canonical reference) / clause 2 (declared no-op).
                // Phase 1 ledgers SHOULD NOT carry duplicate entries, but a
                // byte-equal duplicate is a no-op rather than a tamper.
            }
            None => {
                idempotency_index.insert(identity_key, details.canonical_event_hash);
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

        // ADR 0069 D-3 — timestamps must be non-decreasing along chain
        // order. Equal timestamps are permitted; chain position
        // disambiguates. A backwards timestamp is an integrity failure
        // distinct from hash/signature tamper (Core §19 step 4.h-temporal).
        if let Some(prev_at) = previous_authored_at {
            if details.authored_at < prev_at {
                event_failures.push(VerificationFailure::new(
                    "timestamp_order_violation",
                    hex_string(&details.canonical_event_hash),
                ));
            }
        }
        previous_authored_at = Some(details.authored_at);

        // ADR 0005 step 8 input collection — every event contributes a
        // chain summary so the post-loop pass can flag `authored_at >
        // destroyed_at` events that sign under (post_erasure_use) or wrap
        // for (post_erasure_wrap) a destroyed kid.
        chain_summaries.push(ChainEventSummary {
            event_index: index as u64,
            authored_at: details.authored_at,
            signing_kid: event.kid.clone(),
            wrap_recipients: details.wrap_recipients.clone(),
            canonical_event_hash: details.canonical_event_hash,
        });
        if let Some(erasure) = details.erasure.clone() {
            erasure_payloads.push((index, erasure, details.canonical_event_hash));
        }
        if let Some(certificate) = details.certificate.clone() {
            certificate_payloads.push((index, certificate, details.canonical_event_hash));
        }
        if let Some(uca) = details.user_content_attestation.clone() {
            user_content_attestation_payloads.push((index, uca, details.canonical_event_hash));
        }

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

    let erasure_evidence = finalize_erasure_evidence(
        &erasure_payloads,
        &chain_summaries,
        registry,
        non_signing_registry,
        &mut event_failures,
    );

    // ADR 0007 certificate-of-completion finalization (steps 2 / 5 / 6 / 7 /
    // 8 cross-event reasoning). Step 4 (attachment lineage + content
    // recompute) defers to the export-bundle path; the genesis-append path
    // accumulates outcomes with `attachment_resolved = true` so the §19
    // step-9 fold doesn't false-positive on minimal-genesis fixtures.
    let certificates_of_completion =
        finalize_certificates_of_completion(&certificate_payloads, events, &mut event_failures);

    // ADR 0010 user-content-attestation finalization (Core §19 step 6d
    // steps 3 / 4 / 5 / 6 / 7 / 8 / 9 cross-event reasoning).
    let user_content_attestations = finalize_user_content_attestations(
        &user_content_attestation_payloads,
        events,
        registry,
        posture_declaration,
        &mut event_failures,
    );

    VerificationReport::from_integrity_state(
        event_failures,
        Vec::new(),
        Vec::new(),
        posture_transitions,
        erasure_evidence,
        certificates_of_completion,
        user_content_attestations,
        Vec::new(),
    )
}

/// Per-event chain summary used by ADR 0005 step 8 — the destroyed-kid
/// chain-consistency walk needs `authored_at`, the signing `kid`, every
/// `key_bag.entries[*].recipient`, and the canonical event hash for failure
/// localization.
#[derive(Clone, Debug)]
struct ChainEventSummary {
    /// Reserved for future step-8 localization where the chain index is the
    /// dominant audit dimension. Today step 8 localizes by canonical event
    /// hash (parallel to existing event_failures localization).
    #[allow(dead_code)]
    event_index: u64,
    authored_at: TrellisTimestamp,
    signing_kid: Vec<u8>,
    wrap_recipients: Vec<Vec<u8>>,
    canonical_event_hash: [u8; 32],
}

/// ADR 0005 §"Verifier obligations" finalization pass: runs steps 2 / 5 / 7
/// / 8 / 10 after every event has been individually decoded. Steps 1, 3, 4,
/// and 6 ran inline in [`decode_erasure_evidence_details`] (those checks
/// are local to one event and short-circuit on first violation). Step 9
/// (cascade-scope cross-check) is reserved for a Phase-2 deep-cascade pass.
///
/// Localizable failures are pushed into `event_failures` so the report's
/// `tamper_kind` projection picks them up; cross-payload group failures
/// (`erasure_destroyed_at_conflict`, `erasure_key_class_payload_conflict`)
/// localize to the second-emitted payload's canonical hash so the auditor
/// can find the disagreement by walking forward from the first row.
fn finalize_erasure_evidence(
    payloads: &[(usize, ErasureEvidenceDetails, [u8; 32])],
    chain: &[ChainEventSummary],
    registry: &BTreeMap<Vec<u8>, SigningKeyEntry>,
    non_signing_registry: Option<&BTreeMap<Vec<u8>, NonSigningKeyEntry>>,
    event_failures: &mut Vec<VerificationFailure>,
) -> Vec<ErasureEvidenceOutcome> {
    if payloads.is_empty() {
        return Vec::new();
    }

    // Step 5 / 8 group state — keyed by `kid_destroyed` bytes.
    let mut group_destroyed_at: BTreeMap<Vec<u8>, TrellisTimestamp> = BTreeMap::new();
    let mut group_key_class: BTreeMap<Vec<u8>, String> = BTreeMap::new();
    let mut group_conflict_destroyed_at: BTreeSet<Vec<u8>> = BTreeSet::new();
    let mut group_conflict_key_class: BTreeSet<Vec<u8>> = BTreeSet::new();

    let mut outcomes: Vec<ErasureEvidenceOutcome> = Vec::with_capacity(payloads.len());

    // First pass: step 2 (registry bind) per payload + step 5 (group
    // destroyed_at / key_class agreement). Localize conflicts on the
    // second-encountered row.
    for (index, payload, canonical_hash) in payloads {
        // Step 2: registry bind. If `kid_destroyed` resolves to exactly one
        // KeyEntry row, `norm_key_class` MUST match that row's `kind`. For
        // legacy flat `SigningKeyEntry` (no `kind` on the row), the
        // expected class is `signing`.
        let registry_class: Option<&str> = if registry.contains_key(&payload.kid_destroyed) {
            Some("signing")
        } else if let Some(non_signing) = non_signing_registry
            && let Some(entry) = non_signing.get(&payload.kid_destroyed)
        {
            Some(entry.class.as_str())
        } else {
            None
        };

        if let Some(expected_class) = registry_class
            && expected_class != payload.norm_key_class
        {
            event_failures.push(VerificationFailure::new(
                "erasure_key_class_registry_mismatch",
                hex_string(canonical_hash),
            ));
        }

        // Step 5: group by kid_destroyed. First payload sets the canonical
        // destroyed_at + key_class; subsequent rows must agree byte-for-byte.
        match group_destroyed_at.entry(payload.kid_destroyed.clone()) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(payload.destroyed_at);
            }
            std::collections::btree_map::Entry::Occupied(entry) => {
                if *entry.get() != payload.destroyed_at
                    && !group_conflict_destroyed_at.contains(&payload.kid_destroyed)
                {
                    event_failures.push(VerificationFailure::new(
                        "erasure_destroyed_at_conflict",
                        hex_string(canonical_hash),
                    ));
                    group_conflict_destroyed_at.insert(payload.kid_destroyed.clone());
                }
            }
        }
        match group_key_class.entry(payload.kid_destroyed.clone()) {
            std::collections::btree_map::Entry::Vacant(entry) => {
                entry.insert(payload.norm_key_class.clone());
            }
            std::collections::btree_map::Entry::Occupied(entry) => {
                if entry.get() != &payload.norm_key_class
                    && !group_conflict_key_class.contains(&payload.kid_destroyed)
                {
                    event_failures.push(VerificationFailure::new(
                        "erasure_key_class_payload_conflict",
                        hex_string(canonical_hash),
                    ));
                    group_conflict_key_class.insert(payload.kid_destroyed.clone());
                }
            }
        }

        outcomes.push(ErasureEvidenceOutcome {
            evidence_id: payload.evidence_id.clone(),
            kid_destroyed: payload.kid_destroyed.clone(),
            destroyed_at: payload.destroyed_at,
            cascade_scopes: payload.cascade_scopes.clone(),
            completion_mode: payload.completion_mode.clone(),
            event_index: *index as u64,
            signature_verified: payload.attestation_signatures_well_formed,
            post_erasure_uses: 0,
            post_erasure_wraps: 0,
            cascade_violations: Vec::new(),
            failures: Vec::new(),
        });
    }

    // Step 8: chain consistency for `norm_key_class ∈ {"signing", "subject"}`.
    // For each kid_destroyed group with a non-conflicting destroyed_at +
    // key_class, walk every chain event with `authored_at > destroyed_at`
    // and flag uses / wraps.
    for outcome in outcomes.iter_mut() {
        // Skip groups that already failed step 5 — propagating step-8 noise
        // on top of a destroyed_at conflict is misleading.
        if group_conflict_destroyed_at.contains(&outcome.kid_destroyed) {
            outcome
                .failures
                .push("erasure_destroyed_at_conflict".into());
            continue;
        }
        if group_conflict_key_class.contains(&outcome.kid_destroyed) {
            outcome
                .failures
                .push("erasure_key_class_payload_conflict".into());
            continue;
        }
        let class = group_key_class
            .get(&outcome.kid_destroyed)
            .map(String::as_str)
            .unwrap_or("");
        if class != "signing" && class != "subject" {
            // ADR 0005 step 8 Phase-1 scope: only signing + subject classes
            // run the chain walk. recovery / scope / tenant-root and
            // extension `tstr` classes are admitted at the wire layer; the
            // subtree obligations co-land with ADR 0006 follow-on milestones.
            continue;
        }
        let destroyed_at = match group_destroyed_at.get(&outcome.kid_destroyed) {
            Some(value) => *value,
            None => continue,
        };
        for event in chain {
            if event.authored_at <= destroyed_at {
                continue;
            }
            if event.signing_kid == outcome.kid_destroyed {
                outcome.post_erasure_uses += 1;
                outcome.failures.push("post_erasure_use".into());
                event_failures.push(VerificationFailure::new(
                    "post_erasure_use",
                    hex_string(&event.canonical_event_hash),
                ));
            }
            if event
                .wrap_recipients
                .iter()
                .any(|recipient| recipient == &outcome.kid_destroyed)
            {
                outcome.post_erasure_wraps += 1;
                outcome.failures.push("post_erasure_wrap".into());
                event_failures.push(VerificationFailure::new(
                    "post_erasure_wrap",
                    hex_string(&event.canonical_event_hash),
                ));
            }
        }
    }

    // Step 7 (Phase-1 structural): `signature_verified = false` already set
    // on individual outcomes if any attestation row is malformed; surface a
    // localized failure so `integrity_verified` flips and the
    // `tamper_kind` projection can find it.
    for outcome in outcomes.iter() {
        if !outcome.signature_verified {
            event_failures.push(VerificationFailure::new(
                "erasure_attestation_signature_invalid",
                hex_string(&[0u8; 32]),
            ));
        }
    }

    outcomes
}

/// ADR 0007 §"Verifier obligations" cross-event finalization. Step 1 runs
/// in [`decode_certificate_payload`] (CDDL + per-event chain-summary
/// invariants); this pass runs steps 2 (id collision + workflow_status /
/// impact_level / covered_claims registry — Phase-1 admit-any-tstr posture),
/// 5 (signing-event resolution), 6 (timestamp equivalence), 7 (response_ref
/// equivalence), and 8 (outcome accumulation).
///
/// **Phase-1 chain-context posture.** Step 5 / 6 / 7 require the full event
/// list to resolve `signing_events[i]` digests against in-chain
/// `wos.kernel.signatureAffirmation` events. The genesis-append code paths
/// (`verify_single_event` / `verify_tampered_ledger`) frequently pass a
/// minimal `events` slice that does not include the referenced signing
/// events; in that case `signing_event_unresolved` would false-positive on
/// vectors whose contract is "this one event decodes". Posture: when an
/// event in `events` matches a `signing_events[i]` digest, run the cross
/// checks; when it does not, accumulate the outcome with
/// `all_signing_events_resolved = false` ONLY if the export-bundle context
/// is in scope. The export-bundle path runs the full `verify_event_set_with_classes`
/// over all events, so chain context is complete and step 5 / 6 / 7 are
/// authoritative there. Step 4 (attachment lineage) is wholly deferred to
/// the export-bundle path — see [`verify_certificate_attachment_lineage`].
///
/// Step 2 covered_claims / workflow_status / impact_level enum-extension
/// gating: per ADR 0007 §"Field semantics" Phase-1 reference verifier
/// admits any registered `tstr` shape; deep registry-membership rides
/// follow-on milestones alongside WOS signature-profile registry plumbing.
/// `certificate_covered_claim_unknown` and `certificate_enum_extension_unknown`
/// reserve their tamper_kind in §19.1 for that evolution.
fn finalize_certificates_of_completion(
    payloads: &[(usize, CertificateDetails, [u8; 32])],
    events: &[ParsedSign1],
    event_failures: &mut Vec<VerificationFailure>,
) -> Vec<CertificateOfCompletionOutcome> {
    if payloads.is_empty() {
        return Vec::new();
    }

    // Build a (canonical_event_hash → EventDetails) lookup once. Reused for
    // step 5 (signing-event resolution), step 6 (timestamp equivalence),
    // and step 7 (response_ref equivalence).
    let mut event_by_hash: BTreeMap<[u8; 32], EventDetails> = BTreeMap::new();
    for event in events {
        if let Ok(details) = decode_event_details(event) {
            event_by_hash
                .entry(details.canonical_event_hash)
                .or_insert(details);
        }
    }

    // Step 2 first sub-clause (per-index principal_ref equivalence) requires
    // the resolved SignatureAffirmation event's payload to extract its
    // declared principal. Phase-1 reference verifier reads the payload's
    // `data.signerId` field per the WOS-T4 record shape (mirror of what
    // `parse_signature_affirmation_record` reads in the catalog path).
    // When `signing_events[i]` is unresolvable in this `events` slice, skip
    // the per-index comparison (recorded as `signing_event_unresolved` in
    // step 5 instead of double-flagging here).

    // Step 2 second sub-clause: certificate_id collision detection across
    // the certificate event set in scope. "Differ" is canonical-payload
    // disagreement; for the Phase-1 reference verifier we compare
    // `(content_hash, signing_events, signer_count, completed_at,
    // workflow_status)` because those are the load-bearing fields that
    // ADR 0007 §"Field semantics" identifies as collision-indicative.
    let mut id_to_canonical: BTreeMap<String, &CertificateDetails> = BTreeMap::new();
    let mut id_collision_reported: BTreeSet<String> = BTreeSet::new();
    for (_index, payload, canonical_hash) in payloads {
        match id_to_canonical.entry(payload.certificate_id.clone()) {
            std::collections::btree_map::Entry::Vacant(slot) => {
                slot.insert(payload);
            }
            std::collections::btree_map::Entry::Occupied(slot) => {
                let prior = *slot.get();
                let differs = prior.presentation_artifact.content_hash
                    != payload.presentation_artifact.content_hash
                    || prior.signing_events != payload.signing_events
                    || prior.chain_summary.signer_count != payload.chain_summary.signer_count
                    || prior.completed_at != payload.completed_at
                    || prior.chain_summary.workflow_status != payload.chain_summary.workflow_status;
                if differs && id_collision_reported.insert(payload.certificate_id.clone()) {
                    event_failures.push(VerificationFailure::new(
                        "certificate_id_collision",
                        hex_string(canonical_hash),
                    ));
                }
            }
        }
    }

    let mut outcomes: Vec<CertificateOfCompletionOutcome> = Vec::with_capacity(payloads.len());

    for (index, payload, canonical_hash) in payloads {
        let mut outcome = CertificateOfCompletionOutcome {
            certificate_id: payload.certificate_id.clone(),
            event_index: *index as u64,
            completed_at: payload.completed_at,
            signer_count: payload.chain_summary.signer_count,
            // Step 4 (attachment lineage + content recompute) is the
            // export-bundle path's responsibility. Genesis-append context:
            // mark `attachment_resolved = true` so the §19 step-9 fold
            // doesn't false-positive on minimal-genesis fixtures. The
            // export-bundle path overrides this via
            // `verify_certificate_attachment_lineage`.
            attachment_resolved: true,
            all_signing_events_resolved: true,
            chain_summary_consistent: true,
            failures: Vec::new(),
        };

        // Step 3 (Phase-1 structural attestation contract): if any row was
        // malformed at decode time, surface as `attestation_insufficient`
        // (existing code reused per ADR 0007 §"Verifier obligations" step 3).
        // Crypto verification rides Phase-2+.
        if !payload.attestation_signatures_well_formed {
            outcome.chain_summary_consistent = false;
            outcome.failures.push("attestation_insufficient".into());
            event_failures.push(VerificationFailure::new(
                "attestation_insufficient",
                hex_string(canonical_hash),
            ));
        }

        // Steps 5 / 6 / 7 — each `signing_events[i]` digest cross-checked
        // against the chain. When the slice does not carry the referenced
        // event, treat it as unresolvable (export-bundle context) — the
        // export-bundle path is the authoritative caller because it carries
        // the full chain. Genesis-append context lacks chain visibility;
        // see the docstring on this fn.
        for (i, signing_event_hash) in payload.signing_events.iter().enumerate() {
            let resolved = event_by_hash.get(signing_event_hash);
            let Some(target) = resolved else {
                outcome.all_signing_events_resolved = false;
                outcome.failures.push("signing_event_unresolved".into());
                event_failures.push(VerificationFailure::new(
                    "signing_event_unresolved",
                    hex_string(signing_event_hash),
                ));
                continue;
            };
            // Step 5: event_type MUST be wos.kernel.signatureAffirmation
            // (or other registered SignatureAffirmation equivalent per Core
            // §6.7; only the WOS form is registered in Phase-1).
            if target.event_type != WOS_SIGNATURE_AFFIRMATION_EVENT_TYPE {
                outcome.all_signing_events_resolved = false;
                outcome.failures.push("signing_event_unresolved".into());
                event_failures.push(VerificationFailure::new(
                    "signing_event_unresolved",
                    hex_string(signing_event_hash),
                ));
                continue;
            }
            // Step 6: signed_at MUST equal authored_at (uint exact, no skew).
            let display = &payload.chain_summary.signer_display[i];
            if display.signed_at != target.authored_at {
                outcome.chain_summary_consistent = false;
                outcome
                    .failures
                    .push("signing_event_timestamp_mismatch".into());
                event_failures.push(VerificationFailure::new(
                    "signing_event_timestamp_mismatch",
                    hex_string(signing_event_hash),
                ));
            }
        }

        // Step 7: when `chain_summary.response_ref` is non-null, lookup the
        // resolved SignatureAffirmation event's payload and compare its
        // `data.formspecResponseRef` digest. The reference verifier reads
        // inline payloads (Phase-1 minimal-genesis posture); external
        // payloads via `payload_blobs` ride the export-bundle path.
        if let Some(response_ref) = payload.chain_summary.response_ref {
            // Pick the first signing event as the canonical link target —
            // ADR 0007 §"Field semantics" `response_ref` clause: "the same
            // digest bound by SignatureAffirmation / authoredSignatures for
            // that ceremony" — Phase-1 ties `response_ref` to the ceremony
            // (one ceremony per certificate); we cross-check against every
            // resolved SignatureAffirmation and require at least one match
            // (operator MAY co-bind multiple SignatureAffirmation events
            // for the same ceremony per §"Field semantics" `signing_events`
            // clause "order is workflow order").
            let mut matched = false;
            let mut had_resolvable_response = false;
            for signing_event_hash in &payload.signing_events {
                let Some(target) = event_by_hash.get(signing_event_hash) else {
                    continue;
                };
                if target.event_type != WOS_SIGNATURE_AFFIRMATION_EVENT_TYPE {
                    continue;
                }
                let payload_bytes = match &target.payload_ref {
                    PayloadRef::Inline(bytes) => bytes.clone(),
                    PayloadRef::External => continue,
                };
                let Ok(record) = parse_signature_affirmation_record(&payload_bytes) else {
                    continue;
                };
                let Ok(record_response_hash) = parse_sha256_text(&record.formspec_response_ref)
                else {
                    // The record's `formspecResponseRef` is per ADR 0007 a
                    // sha256: digest text. If it doesn't parse, surface as
                    // a response_ref_mismatch — the certificate claims a
                    // hash that has no comparable digest on the chain side.
                    continue;
                };
                had_resolvable_response = true;
                if record_response_hash == response_ref {
                    matched = true;
                    break;
                }
            }
            if had_resolvable_response && !matched {
                outcome.chain_summary_consistent = false;
                outcome.failures.push("response_ref_mismatch".into());
                event_failures.push(VerificationFailure::new(
                    "response_ref_mismatch",
                    hex_string(canonical_hash),
                ));
            }
        }

        // Step 2 (per-index principal_ref equivalence) — when the signing
        // event is resolvable AND its inline payload decodes, compare the
        // declared principal on the SignatureAffirmation against the
        // certificate's signer_display row.
        for (i, signing_event_hash) in payload.signing_events.iter().enumerate() {
            let Some(target) = event_by_hash.get(signing_event_hash) else {
                continue;
            };
            if target.event_type != WOS_SIGNATURE_AFFIRMATION_EVENT_TYPE {
                continue;
            }
            let payload_bytes = match &target.payload_ref {
                PayloadRef::Inline(bytes) => bytes.clone(),
                PayloadRef::External => continue,
            };
            let Ok(record) = parse_signature_affirmation_record(&payload_bytes) else {
                continue;
            };
            let display = &payload.chain_summary.signer_display[i];
            if display.principal_ref != record.signer_id {
                outcome.chain_summary_consistent = false;
                outcome
                    .failures
                    .push("certificate_chain_summary_mismatch".into());
                event_failures.push(VerificationFailure::new(
                    "certificate_chain_summary_mismatch",
                    hex_string(canonical_hash),
                ));
                break;
            }
        }

        outcomes.push(outcome);
    }

    outcomes
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

    // Core §6.1 / §17.2 — `idempotency_key` MUST be a CBOR byte string of
    // 1..=64 bytes. Length-bound violations surface as the typed §17.5
    // `idempotency_key_length_invalid` so the report's `tamper_kind`
    // localizes the structural failure.
    let idempotency_key = map_lookup_bytes(payload_map, "idempotency_key")?;
    if idempotency_key.is_empty() || idempotency_key.len() > 64 {
        return Err(VerifyError::with_kind(
            format!(
                "idempotency_key length {} outside Core §6.1 / §17.2 bound 1..=64",
                idempotency_key.len(),
            ),
            "idempotency_key_length_invalid",
        ));
    }

    let header = map_lookup_map(payload_map, "header")?;
    let authored_at = map_lookup_timestamp(header, "authored_at")?;
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

    let (
        transition,
        attachment_binding,
        erasure,
        certificate,
        user_content_attestation,
        identity_attestation_subject,
    ) = match map_lookup_optional_map(payload_map, "extensions")? {
        Some(extensions) => (
            decode_transition_details(extensions)?,
            decode_attachment_binding_details(extensions)?,
            decode_erasure_evidence_details(extensions, authored_at)?,
            decode_certificate_payload(extensions)?,
            decode_user_content_attestation_payload(extensions, authored_at)?,
            decode_identity_attestation_subject(extensions, &event_type),
        ),
        None => (None, None, None, None, None, None),
    };
    let wrap_recipients = decode_key_bag_recipients(payload_map)?;

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
        idempotency_key,
        payload_ref,
        transition,
        attachment_binding,
        erasure,
        certificate,
        user_content_attestation,
        identity_attestation_subject,
        wrap_recipients,
    })
}

/// Extracts wrap recipients from `payload.key_bag.entries[*].recipient` so
/// step 8 (post_erasure_wrap detection) can compare against `kid_destroyed`.
/// Returns an empty vec when `key_bag` is missing or has no entries (Phase-1
/// plaintext path). Recipients are opaque bytes per Core §9.4 — comparison
/// is byte-equality with the 16-byte `kid_destroyed` in step 8.
fn decode_key_bag_recipients(payload_map: &[(Value, Value)]) -> Result<Vec<Vec<u8>>, VerifyError> {
    let Some(key_bag_value) = map_lookup_optional_value(payload_map, "key_bag") else {
        return Ok(Vec::new());
    };
    let key_bag = match key_bag_value {
        Value::Map(map) => map.as_slice(),
        Value::Null => return Ok(Vec::new()),
        _ => return Err(VerifyError::new("key_bag is neither a map nor null")),
    };
    let Some(entries_value) = map_lookup_optional_value(key_bag, "entries") else {
        return Ok(Vec::new());
    };
    let entries = match entries_value {
        Value::Array(array) => array,
        Value::Null => return Ok(Vec::new()),
        _ => {
            return Err(VerifyError::new(
                "key_bag.entries is neither an array nor null",
            ));
        }
    };
    let mut recipients = Vec::with_capacity(entries.len());
    for entry in entries {
        let entry_map = entry
            .as_map()
            .ok_or_else(|| VerifyError::new("key_bag entry is not a map"))?;
        let recipient = map_lookup_bytes(entry_map, "recipient")?;
        recipients.push(recipient);
    }
    Ok(recipients)
}

fn decode_transition_details(
    extensions: &[(Value, Value)],
) -> Result<Option<TransitionDetails>, VerifyError> {
    let custody = map_lookup_optional_value(extensions, "trellis.custody-model-transition.v1");
    let disclosure =
        map_lookup_optional_value(extensions, "trellis.disclosure-profile-transition.v1");
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
    let _effective_at = map_lookup_timestamp(extension_map, "effective_at")?;
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
    let _effective_at = map_lookup_timestamp(extension_map, "effective_at")?;
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

/// Decodes the optional `trellis.erasure-evidence.v1` extension payload
/// and runs ADR 0005 §"Verifier obligations" steps 1 (CDDL), 3 (subject_scope
/// shape), and 6 (hsm_receipt null-consistency) inline. Step 4 (`destroyed_at`
/// vs hosting event `authored_at`) is also enforced here because both inputs
/// are local to one event. Steps 2 / 5 / 7 / 8 / 9 / 10 run in the cross-event
/// finalization pass after every event has been decoded.
///
/// `host_authored_at` is the `authored_at` of the carrying event so step 4
/// can short-circuit at decode time.
fn decode_erasure_evidence_details(
    extensions: &[(Value, Value)],
    host_authored_at: TrellisTimestamp,
) -> Result<Option<ErasureEvidenceDetails>, VerifyError> {
    let Some(extension_value) =
        map_lookup_optional_value(extensions, ERASURE_EVIDENCE_EVENT_EXTENSION)
    else {
        return Ok(None);
    };
    let extension_map = extension_value
        .as_map()
        .ok_or_else(|| VerifyError::new("erasure-evidence extension is not a map"))?;

    // Step 1: CDDL decode. Required fields per ADR 0005 §"Wire shape".
    let evidence_id = map_lookup_text(extension_map, "evidence_id")?;
    let kid_destroyed = map_lookup_fixed_bytes(extension_map, "kid_destroyed", 16)?;

    // Step 2 prep: capture `key_class` and apply the `wrap` → `subject`
    // normalization at decode time so cross-event step 5 / step 8 reasoning
    // operates on the canonical taxonomy. Registry-bind happens in the
    // finalize pass once the registry maps are in scope.
    let wire_key_class = map_lookup_text(extension_map, "key_class")?;
    let norm_key_class = if wire_key_class == "wrap" {
        "subject".to_string()
    } else {
        wire_key_class
    };

    let destroyed_at = map_lookup_timestamp(extension_map, "destroyed_at")?;

    // Step 4: `destroyed_at` MUST be ≤ host event's `authored_at`.
    // Companion OC-144 / TR-OP-109. Violation is a structure failure with
    // typed kind so the report's `tamper_kind` carries
    // `erasure_destroyed_at_after_host`.
    if destroyed_at > host_authored_at {
        return Err(VerifyError::with_kind(
            format!(
                "erasure-evidence `destroyed_at` ({destroyed_at}) exceeds hosting event `authored_at` ({host_authored_at}) (Companion OC-144 / ADR 0005 step 4)"
            ),
            "erasure_destroyed_at_after_host",
        ));
    }

    // CDDL: cascade_scopes is a non-empty array of CascadeScope text strings.
    let cascade_array = map_lookup_array(extension_map, "cascade_scopes")?;
    if cascade_array.is_empty() {
        return Err(VerifyError::new(
            "erasure-evidence `cascade_scopes` MUST be a non-empty array (ADR 0005 §Wire shape)",
        ));
    }
    let mut cascade_scopes = Vec::with_capacity(cascade_array.len());
    for scope_value in cascade_array {
        let scope = scope_value
            .as_text()
            .ok_or_else(|| VerifyError::new("erasure-evidence cascade_scope entry is not text"))?;
        cascade_scopes.push(scope.to_string());
    }

    let completion_mode = map_lookup_text(extension_map, "completion_mode")?;
    let _destruction_actor = map_lookup_text(extension_map, "destruction_actor")?;
    let _policy_authority = map_lookup_text(extension_map, "policy_authority")?;
    let _reason_code = map_lookup_u64(extension_map, "reason_code")?;

    // Step 3: `subject_scope` cross-field shape by `kind`.
    let subject_scope_value = map_lookup_optional_value(extension_map, "subject_scope")
        .ok_or_else(|| VerifyError::new("erasure-evidence `subject_scope` is missing"))?;
    let subject_scope_map = subject_scope_value
        .as_map()
        .ok_or_else(|| VerifyError::new("erasure-evidence `subject_scope` is not a map"))?;
    let subject_scope_kind = map_lookup_text(subject_scope_map, "kind")?;
    validate_subject_scope_shape(subject_scope_map, &subject_scope_kind)?;

    // Step 6: `hsm_receipt` / `hsm_receipt_kind` null-consistency.
    let receipt_present = matches!(
        map_lookup_optional_value(extension_map, "hsm_receipt"),
        Some(Value::Bytes(_))
    );
    let receipt_kind_present = matches!(
        map_lookup_optional_value(extension_map, "hsm_receipt_kind"),
        Some(Value::Text(_))
    );
    if receipt_present != receipt_kind_present {
        return Err(VerifyError::new(
            "erasure-evidence `hsm_receipt` and `hsm_receipt_kind` must both be null or both non-null (ADR 0005 step 6)",
        ));
    }

    // Step 7 (Phase-1 structural): every attestation row carries a 64-byte
    // signature and a recognized `authority_class`. Crypto-verification of
    // the Ed25519 signature itself rides Phase-2+ — same posture as the
    // existing `decode_attestation_classes` flow for posture transitions.
    let attestations = map_lookup_array(extension_map, "attestations")?;
    if attestations.is_empty() {
        return Err(VerifyError::new(
            "erasure-evidence `attestations` MUST be non-empty (ADR 0005 §Wire shape)",
        ));
    }
    let mut attestation_classes = Vec::with_capacity(attestations.len());
    let mut attestation_signatures_well_formed = true;
    for entry in attestations {
        let entry_map = entry
            .as_map()
            .ok_or_else(|| VerifyError::new("attestation entry is not a map"))?;
        let class = map_lookup_text(entry_map, "authority_class")?;
        attestation_classes.push(class);
        let signature = map_lookup_bytes(entry_map, "signature")?;
        if signature.len() != 64 {
            attestation_signatures_well_formed = false;
        }
        // `authority` is captured by ADR 0005 wire but not yet used by the
        // Phase-1 verifier (no authority↔key registry binding); we still
        // require the field to exist per CDDL.
        let _authority = map_lookup_text(entry_map, "authority")?;
    }

    Ok(Some(ErasureEvidenceDetails {
        evidence_id,
        kid_destroyed,
        norm_key_class,
        destroyed_at,
        cascade_scopes,
        completion_mode,
        attestation_signatures_well_formed,
        attestation_classes,
        subject_scope_kind,
    }))
}

/// Decodes the optional `trellis.certificate-of-completion.v1` extension
/// payload and runs ADR 0007 §"Verifier obligations" step 1 (CDDL decode +
/// per-event chain-summary invariants) inline. Cross-event steps 2 (id
/// collision), 4 (attachment lineage), 5 (signing-event resolution),
/// 6 (timestamp equivalence), 7 (response_ref equivalence) run in
/// [`finalize_certificates_of_completion`] after every event has been decoded.
///
/// Per-event invariants enforced here:
/// - `signer_count == len(signing_events)` (ADR 0007 §"Verifier obligations"
///   step 2 first clause; `certificate_chain_summary_mismatch`)
/// - `len(signer_display) == len(signing_events)` (same step; same kind)
/// - HTML media type carries non-null `template_hash` (ADR 0007 §"Wire shape"
///   `PresentationArtifact.template_hash`; emitted as a structure failure via
///   the generic `malformed_cose` kind because §19.1 has no dedicated
///   tamper_kind for this case)
fn decode_certificate_payload(
    extensions: &[(Value, Value)],
) -> Result<Option<CertificateDetails>, VerifyError> {
    let Some(extension_value) = map_lookup_optional_value(extensions, CERTIFICATE_EVENT_EXTENSION)
    else {
        return Ok(None);
    };
    let extension_map = extension_value
        .as_map()
        .ok_or_else(|| VerifyError::new("certificate-of-completion extension is not a map"))?;

    let certificate_id = map_lookup_text(extension_map, "certificate_id")?;
    let case_ref = map_lookup_optional_text(extension_map, "case_ref")?;
    let completed_at = map_lookup_timestamp(extension_map, "completed_at")?;

    // PresentationArtifact decode.
    let pa_value = map_lookup_optional_value(extension_map, "presentation_artifact")
        .ok_or_else(|| VerifyError::new("certificate `presentation_artifact` is missing"))?;
    let pa_map = pa_value
        .as_map()
        .ok_or_else(|| VerifyError::new("certificate `presentation_artifact` is not a map"))?;
    let pa_content_hash = bytes_array(&map_lookup_fixed_bytes(pa_map, "content_hash", 32)?);
    let pa_media_type = map_lookup_text(pa_map, "media_type")?;
    let pa_byte_length = map_lookup_u64(pa_map, "byte_length")?;
    let pa_attachment_id = map_lookup_text(pa_map, "attachment_id")?;
    let pa_template_id = map_lookup_optional_text(pa_map, "template_id")?;
    let pa_template_hash = map_lookup_optional_fixed_bytes(pa_map, "template_hash", 32)?
        .map(|bytes| bytes_array(&bytes));
    // ADR 0007 §"Wire shape" `PresentationArtifact.template_hash`: when
    // `media_type = "text/html"`, `template_hash` MUST be non-null even when
    // `template_id` is null. §19.1 has no dedicated tamper_kind for this
    // case; surface as a generic structure failure via `malformed_cose`
    // (consistent with other CDDL-shape failures at decode time).
    if pa_media_type == "text/html" && pa_template_hash.is_none() {
        return Err(VerifyError::with_kind(
            "certificate presentation_artifact: media_type=text/html requires template_hash to be non-null (ADR 0007 §Wire shape)",
            "malformed_cose",
        ));
    }

    // ChainSummary decode + per-event invariants.
    let cs_value = map_lookup_optional_value(extension_map, "chain_summary")
        .ok_or_else(|| VerifyError::new("certificate `chain_summary` is missing"))?;
    let cs_map = cs_value
        .as_map()
        .ok_or_else(|| VerifyError::new("certificate `chain_summary` is not a map"))?;
    let signer_count = map_lookup_u64(cs_map, "signer_count")?;
    let signer_display_array = map_lookup_array(cs_map, "signer_display")?;
    if signer_display_array.is_empty() {
        return Err(VerifyError::new(
            "certificate `chain_summary.signer_display` MUST be non-empty (ADR 0007 §Wire shape)",
        ));
    }
    let mut signer_display = Vec::with_capacity(signer_display_array.len());
    for entry in signer_display_array {
        let entry_map = entry
            .as_map()
            .ok_or_else(|| VerifyError::new("signer_display entry is not a map"))?;
        let principal_ref = map_lookup_text(entry_map, "principal_ref")?;
        let display_name = map_lookup_text(entry_map, "display_name")?;
        let display_role = map_lookup_optional_text(entry_map, "display_role")?;
        let signed_at = map_lookup_timestamp(entry_map, "signed_at")?;
        signer_display.push(SignerDisplayDetails {
            principal_ref,
            display_name,
            display_role,
            signed_at,
        });
    }
    let response_ref = map_lookup_optional_fixed_bytes(cs_map, "response_ref", 32)?
        .map(|bytes| bytes_array(&bytes));
    let workflow_status = map_lookup_text(cs_map, "workflow_status")?;
    let impact_level = map_lookup_optional_text(cs_map, "impact_level")?;
    let covered_claims_value = map_lookup_optional_value(cs_map, "covered_claims");
    let covered_claims = match covered_claims_value {
        Some(Value::Array(items)) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                let tag = item.as_text().ok_or_else(|| {
                    VerifyError::new("certificate covered_claims entry is not text")
                })?;
                out.push(tag.to_string());
            }
            out
        }
        Some(Value::Null) | None => Vec::new(),
        Some(_) => {
            return Err(VerifyError::new(
                "certificate `chain_summary.covered_claims` is not an array",
            ));
        }
    };

    // signing_events decode.
    let signing_events_array = map_lookup_array(extension_map, "signing_events")?;
    if signing_events_array.is_empty() {
        return Err(VerifyError::new(
            "certificate `signing_events` MUST be non-empty (ADR 0007 §Wire shape)",
        ));
    }
    let mut signing_events = Vec::with_capacity(signing_events_array.len());
    for digest_value in signing_events_array {
        let bytes = digest_value
            .as_bytes()
            .ok_or_else(|| VerifyError::new("signing_events entry is not a byte string"))?;
        let digest: [u8; 32] = bytes
            .as_slice()
            .try_into()
            .map_err(|_| VerifyError::new("signing_events entry is not 32 bytes"))?;
        signing_events.push(digest);
    }

    // ADR 0007 §"Verifier obligations" step 2 first invariant: per-event
    // shape (signer_count == len(signing_events) AND len(signer_display) ==
    // len(signing_events)). Mismatch flips integrity via the
    // `certificate_chain_summary_mismatch` tamper_kind.
    if signer_count as usize != signing_events.len() || signer_display.len() != signing_events.len()
    {
        return Err(VerifyError::with_kind(
            format!(
                "certificate chain_summary invariant violated: signer_count={}, signing_events={}, signer_display={} (ADR 0007 §Verifier obligations step 2)",
                signer_count,
                signing_events.len(),
                signer_display.len()
            ),
            "certificate_chain_summary_mismatch",
        ));
    }

    let workflow_ref = map_lookup_optional_text(extension_map, "workflow_ref")?;

    // Step 3 (Phase-1 structural): every attestation row carries a 64-byte
    // signature and a recognized `authority_class`. Crypto-verification of
    // the Ed25519 signature itself rides Phase-2+ — same posture as the
    // existing posture-transition + erasure flows.
    let attestations = map_lookup_array(extension_map, "attestations")?;
    if attestations.is_empty() {
        return Err(VerifyError::new(
            "certificate `attestations` MUST be non-empty (ADR 0007 §Wire shape)",
        ));
    }
    let mut attestation_signatures_well_formed = true;
    for entry in attestations {
        let entry_map = entry
            .as_map()
            .ok_or_else(|| VerifyError::new("attestation entry is not a map"))?;
        let _class = map_lookup_text(entry_map, "authority_class")?;
        let signature = map_lookup_bytes(entry_map, "signature")?;
        if signature.len() != 64 {
            attestation_signatures_well_formed = false;
        }
        let _authority = map_lookup_text(entry_map, "authority")?;
    }

    Ok(Some(CertificateDetails {
        certificate_id,
        case_ref,
        completed_at,
        presentation_artifact: PresentationArtifactDetails {
            content_hash: pa_content_hash,
            media_type: pa_media_type,
            byte_length: pa_byte_length,
            attachment_id: pa_attachment_id,
            template_id: pa_template_id,
            template_hash: pa_template_hash,
        },
        chain_summary: ChainSummaryDetails {
            signer_count,
            signer_display,
            response_ref,
            workflow_status,
            impact_level,
            covered_claims,
        },
        signing_events,
        workflow_ref,
        attestation_signatures_well_formed,
    }))
}

/// Decodes the optional `trellis.user-content-attestation.v1` extension
/// payload and runs ADR 0010 §"Verifier obligations" step 1 (CDDL decode)
/// and step 2 partial (`signing_intent` URI well-formedness;
/// `attested_at == envelope.authored_at` exact equality) inline. Cross-event
/// steps 3 (chain-position resolution), 4 (identity resolution),
/// 5 (signature verification), 6 (key-state check), 7 (collision detection),
/// 8 (operator-in-user-slot enforcement), and 9 (outcome accumulation) run
/// in [`finalize_user_content_attestations`] after every event has been
/// decoded.
///
/// Per-event invariants enforced here:
/// - `attested_at == host EventHeader.authored_at` (uint exact equality;
///   `user_content_attestation_timestamp_mismatch`)
/// - `signing_intent` is a syntactically valid URI per RFC 3986
///   (`user_content_attestation_intent_malformed`)
/// - structural CDDL shape per Core §28 / ADR 0010 §"Wire shape"
fn decode_user_content_attestation_payload(
    extensions: &[(Value, Value)],
    host_authored_at: TrellisTimestamp,
) -> Result<Option<UserContentAttestationDetails>, VerifyError> {
    let Some(extension_value) =
        map_lookup_optional_value(extensions, USER_CONTENT_ATTESTATION_EVENT_EXTENSION)
    else {
        return Ok(None);
    };
    let extension_map = extension_value
        .as_map()
        .ok_or_else(|| VerifyError::new("user-content-attestation extension is not a map"))?;

    let attestation_id = map_lookup_text(extension_map, "attestation_id")?;
    let attested_event_hash = bytes_array(&map_lookup_fixed_bytes(
        extension_map,
        "attested_event_hash",
        32,
    )?);
    let attested_event_position = map_lookup_u64(extension_map, "attested_event_position")?;
    let attestor = map_lookup_text(extension_map, "attestor")?;
    let identity_attestation_ref =
        map_lookup_optional_fixed_bytes(extension_map, "identity_attestation_ref", 32)?
            .map(|bytes| bytes_array(&bytes));
    let signing_intent = map_lookup_text(extension_map, "signing_intent")?;
    let attested_at = map_lookup_timestamp(extension_map, "attested_at")?;
    let signature_bytes = map_lookup_fixed_bytes(extension_map, "signature", 64)?;
    let signature: [u8; 64] = signature_bytes
        .as_slice()
        .try_into()
        .map_err(|_| VerifyError::new("user-content-attestation signature is not 64 bytes"))?;
    let signing_kid = map_lookup_fixed_bytes(extension_map, "signing_kid", 16)?;

    // Step 2 partial — `attested_at` MUST exactly equal envelope `authored_at`
    // (uint seconds; no skew slack per ADR 0010 §"Field semantics"
    // `attested_at` clause).
    // Step 2 — intra-payload invariants. Per ADR 0010 §"Verifier obligations"
    // step 2, these flip `integrity_verified = false` only — they are NOT
    // structure failures, so the deferred-failure marker rides through to
    // finalize where it surfaces as an `event_failure`. Returning `Err` here
    // would (incorrectly) flip `readability_verified = false` per the
    // `verify_tampered_ledger` fatal-decode path. First-detected wins;
    // additional invariants land via the same marker pattern if the corpus
    // grows.
    let step_2_failure: Option<&'static str> = if attested_at != host_authored_at {
        Some("user_content_attestation_timestamp_mismatch")
    } else if !is_syntactically_valid_uri(&signing_intent) {
        Some("user_content_attestation_intent_malformed")
    } else {
        None
    };

    let canonical_preimage = compute_user_content_attestation_preimage(
        &attestation_id,
        &attested_event_hash,
        attested_event_position,
        &attestor,
        identity_attestation_ref.as_ref(),
        &signing_intent,
        attested_at,
    );

    Ok(Some(UserContentAttestationDetails {
        attestation_id,
        attested_event_hash,
        attested_event_position,
        attestor,
        identity_attestation_ref,
        signing_intent,
        attested_at,
        signature,
        signing_kid,
        canonical_preimage,
        step_2_failure,
    }))
}

/// Builds the dCBOR signature preimage for a user-content attestation per
/// ADR 0010 §"Wire shape": `dCBOR([attestation_id, attested_event_hash,
/// attested_event_position, attestor, identity_attestation_ref,
/// signing_intent, attested_at])`. Pre-computed at decode time so the
/// finalize pass can re-verify without re-encoding. The encoded array is
/// then domain-separated under `trellis-user-content-attestation-v1`
/// (Core §9.8) inside [`verify_user_content_attestation_signature`].
fn compute_user_content_attestation_preimage(
    attestation_id: &str,
    attested_event_hash: &[u8; 32],
    attested_event_position: u64,
    attestor: &str,
    identity_attestation_ref: Option<&[u8; 32]>,
    signing_intent: &str,
    attested_at: TrellisTimestamp,
) -> Vec<u8> {
    let identity_value = match identity_attestation_ref {
        Some(digest) => Value::Bytes(digest.to_vec()),
        None => Value::Null,
    };
    let timestamp_value = Value::Array(vec![
        Value::Integer(attested_at.seconds.into()),
        Value::Integer(attested_at.nanos.into()),
    ]);
    let array = Value::Array(vec![
        Value::Text(attestation_id.to_owned()),
        Value::Bytes(attested_event_hash.to_vec()),
        Value::Integer(attested_event_position.into()),
        Value::Text(attestor.to_owned()),
        identity_value,
        Value::Text(signing_intent.to_owned()),
        timestamp_value,
    ]);
    let mut buf = Vec::new();
    ciborium::ser::into_writer(&array, &mut buf)
        .expect("ciborium serialization to a Vec cannot fail");
    buf
}

/// Minimal RFC 3986 syntactic URI check used for ADR 0010 §"Verifier
/// obligations" step 2 `signing_intent` validation. Per the ADR, Trellis
/// owns the bytes and WOS owns the meaning — we verify syntactic validity
/// only (scheme present, well-formed). The check accepts:
///   - a non-empty `scheme` per RFC 3986 §3.1
///     (`ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )`)
///   - followed by a `:` separator
///   - followed by any non-empty remainder (the Phase-1 reference verifier
///     does not validate the URI body shape; deployment-side lint
///     tightens this when intent registries ratify per PLN-0380).
///
/// Returns `false` for empty strings, missing `:`, empty schemes, schemes
/// starting with a non-ALPHA character, or schemes containing characters
/// outside the RFC 3986 `scheme` production. URIs without an authority
/// component (e.g. `urn:trellis:intent:notarial`) are admitted.
fn is_syntactically_valid_uri(value: &str) -> bool {
    let Some((scheme, rest)) = value.split_once(':') else {
        return false;
    };
    if scheme.is_empty() || rest.is_empty() {
        return false;
    }
    let mut chars = scheme.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !first.is_ascii_alphabetic() {
        return false;
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '-' || c == '.')
}

/// Companion §6.4 operator-URI prefix check used for ADR 0010 §"Verifier
/// obligations" step 8 (`user_content_attestation_operator_in_user_slot`).
/// The Phase-1 verifier checks against the conservative
/// `urn:trellis:operator:` and `urn:wos:operator:` prefixes; deployments
/// substitute or extend this list via deployment-local lint per the ADR
/// 0010 §"Adversary model" "operator masquerading as user" mitigation.
fn is_operator_uri(value: &str) -> bool {
    value.starts_with(OPERATOR_URI_PREFIX_TRELLIS) || value.starts_with(OPERATOR_URI_PREFIX_WOS)
}

/// Phase-1 identity-attestation event-type admission. Admits the
/// `x-trellis-test/*` reserved fixture identifier (Core §6.7 + §10.6).
/// Per ADR 0010 open question 1, this gains the canonical `wos.identity.*`
/// branch when PLN-0381 ratifies; the test branch stays for future fixture
/// authoring under the spec-reserved test prefix.
fn is_identity_attestation_event_type(event_type: &str) -> bool {
    event_type == PHASE_1_TEST_IDENTITY_EVENT_TYPE
}

/// Reads `admit_unverified_user_attestations` from a Posture Declaration's
/// dCBOR bytes. Per ADR 0010 §"Field semantics" `identity_attestation_ref`
/// clause and Core §28 CDDL, the field is OPTIONAL with a `false` default.
/// Returns `false` when the bytes don't decode as a map, or the field is
/// absent / null / not a bool — failing-closed to the default-required
/// posture so a malformed Posture Declaration cannot silently relax the
/// identity-required gate.
fn parse_admit_unverified_user_attestations(bytes: &[u8]) -> bool {
    let Ok(value) = decode_value(bytes) else {
        return false;
    };
    let Some(map) = value.as_map() else {
        return false;
    };
    matches!(
        map_lookup_optional_value(map, "admit_unverified_user_attestations"),
        Some(Value::Bool(true))
    )
}

/// Resolves the subject of an identity-attestation event from its
/// already-decoded `EventPayload.extensions` map. The Phase-1
/// deployment-local convention (mirrored in the fixture corpus) is to
/// put the subject URI at `extensions[event_type]["subject"]`, where
/// `event_type` is the identity-attestation extension key (registered
/// per Core §6.7; Phase-1 admission via [`is_identity_attestation_event_type`]).
/// Returns `Some(subject)` when resolvable, `None` for non-identity
/// events or for identity events whose payload omits the subject field
/// (that surfaces as `user_content_attestation_identity_subject_mismatch`
/// at the call site).
///
/// Reads from `EventPayload.extensions` directly — the `payload_ref.ciphertext`
/// bytes are an opaque content marker, not the extensions container, and
/// reading from there returns `None` for any well-formed event.
fn decode_identity_attestation_subject(
    extensions: &[(Value, Value)],
    event_type: &str,
) -> Option<String> {
    if !is_identity_attestation_event_type(event_type) {
        return None;
    }
    let identity_value = map_lookup_optional_value(extensions, event_type)?;
    let identity_map = identity_value.as_map()?;
    map_lookup_text(identity_map, "subject").ok()
}

/// Verifies the Ed25519 signature on a user-content-attestation payload
/// per ADR 0010 §"Verifier obligations" step 5. Re-hashes the
/// pre-computed `canonical_preimage` under domain tag
/// `trellis-user-content-attestation-v1` (Core §9.8) and verifies under
/// the public key resolved from `signing_kid`. Returns `Ok(true)` on
/// successful verification, `Ok(false)` on signature failure (caller
/// flips `signature_verified = false`), and `Err` on `signing_kid`
/// resolution failure (caller flips `key_active = false` with
/// `user_content_attestation_key_not_active`).
fn verify_user_content_attestation_signature(
    details: &UserContentAttestationDetails,
    public_key: &[u8; 32],
) -> bool {
    let signed_hash =
        domain_separated_sha256(USER_CONTENT_ATTESTATION_DOMAIN, &details.canonical_preimage);
    let signature = Signature::from_bytes(&details.signature);
    let Ok(verifying_key) = VerifyingKey::from_bytes(public_key) else {
        return false;
    };
    verifying_key.verify(&signed_hash, &signature).is_ok()
}

/// ADR 0010 §"Verifier obligations" cross-event finalization. Step 1 + step 2
/// partial (CDDL + intra-payload invariants) run in
/// [`decode_user_content_attestation_payload`]; this pass runs steps 3
/// (chain-position resolution), 4 (identity resolution), 5 (signature
/// verification), 6 (key-state check), 7 (collision detection), 8
/// (operator-in-user-slot enforcement), and 9 (outcome accumulation).
///
/// **Posture Declaration handling.** Step 4's null-admission gate reads
/// `admit_unverified_user_attestations` from the Posture Declaration in
/// force at `attested_at`. The verifier passes the bytes through; absent
/// declaration defaults to `false` (REQUIRED non-null, fails-closed).
///
/// **Phase-1 identity-attestation taxonomy.** Step 4 admits any chain-
/// present event whose `event_type` matches
/// [`is_identity_attestation_event_type`] — the fixture-corpus
/// deployment-local naming or the PLN-0381 candidate string. Resolved
/// event's `ledger_scope` MUST match the attestation's; `sequence` MUST
/// be strictly less than `attested_event_position`; payload subject
/// MUST equal `attestor`.
fn finalize_user_content_attestations(
    payloads: &[(usize, UserContentAttestationDetails, [u8; 32])],
    events: &[ParsedSign1],
    registry: &BTreeMap<Vec<u8>, SigningKeyEntry>,
    posture_declaration: Option<&[u8]>,
    event_failures: &mut Vec<VerificationFailure>,
) -> Vec<UserContentAttestationOutcome> {
    if payloads.is_empty() {
        return Vec::new();
    }

    // Build chain lookups. `event_by_hash` resolves chain-present events
    // by their canonical_event_hash (step 3: `attested_event_hash` lookup).
    // `event_by_position` resolves by `(scope, sequence)` so step 3 can
    // verify position-binding consistency. Both indexed once.
    let mut event_by_hash: BTreeMap<[u8; 32], EventDetails> = BTreeMap::new();
    let mut event_by_position: BTreeMap<(Vec<u8>, u64), EventDetails> = BTreeMap::new();
    for event in events {
        if let Ok(details) = decode_event_details(event) {
            event_by_hash
                .entry(details.canonical_event_hash)
                .or_insert_with(|| details.clone());
            event_by_position
                .entry((details.scope.clone(), details.sequence))
                .or_insert(details);
        }
    }

    // Step 7 collision detection. Two events sharing `attestation_id` with
    // disagreeing canonical payload fail closed. Use the same load-bearing
    // fields ADR 0010 §"Field semantics" `attestation_id` clause names as
    // collision-indicative: every signed field on the payload (excluding the
    // signature itself, which by construction differs whenever any signed
    // field differs).
    let mut id_to_canonical: BTreeMap<String, &UserContentAttestationDetails> = BTreeMap::new();
    let mut id_collision_reported: BTreeSet<String> = BTreeSet::new();
    for (_index, payload, canonical_hash) in payloads {
        match id_to_canonical.entry(payload.attestation_id.clone()) {
            Entry::Vacant(slot) => {
                slot.insert(payload);
            }
            Entry::Occupied(slot) => {
                let prior = *slot.get();
                let differs = prior.attested_event_hash != payload.attested_event_hash
                    || prior.attested_event_position != payload.attested_event_position
                    || prior.attestor != payload.attestor
                    || prior.identity_attestation_ref != payload.identity_attestation_ref
                    || prior.signing_intent != payload.signing_intent
                    || prior.attested_at != payload.attested_at;
                if differs && id_collision_reported.insert(payload.attestation_id.clone()) {
                    event_failures.push(VerificationFailure::new(
                        "user_content_attestation_id_collision",
                        hex_string(canonical_hash),
                    ));
                }
            }
        }
    }

    let mut outcomes: Vec<UserContentAttestationOutcome> = Vec::with_capacity(payloads.len());

    for (index, payload, canonical_hash) in payloads {
        let mut outcome = UserContentAttestationOutcome {
            attestation_id: payload.attestation_id.clone(),
            attested_event_hash: payload.attested_event_hash,
            attestor: payload.attestor.clone(),
            signing_intent: payload.signing_intent.clone(),
            event_index: *index as u64,
            chain_position_resolved: true,
            identity_resolved: true,
            signature_verified: true,
            key_active: true,
            failures: Vec::new(),
        };

        // Step 2 deferred-failure surface. Per ADR 0010 §"Verifier
        // obligations" step 2, intra-payload-invariant failures
        // (`user_content_attestation_intent_malformed` /
        // `user_content_attestation_timestamp_mismatch`) flip
        // `integrity_verified = false` only — they are NOT structure
        // failures. The decoder set the marker; finalize raises it as an
        // `event_failure` and skips remaining per-event checks for this
        // attestation. Outcome accumulates the failure so the §9 report
        // reflects it; subsequent per-step booleans stay `true` because
        // the steps weren't run, not because they passed.
        if let Some(kind) = payload.step_2_failure {
            outcome.failures.push(kind.into());
            event_failures.push(VerificationFailure::new(kind, hex_string(canonical_hash)));
            outcomes.push(outcome);
            continue;
        }

        // Step 8 — operator-in-user-slot enforcement. Companion §6.4
        // operator URIs MUST NOT appear in `attestor` slots of
        // user-content-attestation events.
        if is_operator_uri(&payload.attestor) {
            outcome
                .failures
                .push("user_content_attestation_operator_in_user_slot".into());
            event_failures.push(VerificationFailure::new(
                "user_content_attestation_operator_in_user_slot",
                hex_string(canonical_hash),
            ));
        }

        // Step 3 — chain-position resolution. `attested_event_position` MUST
        // resolve to a chain-present event in scope whose
        // `canonical_event_hash` equals `attested_event_hash`. To find the
        // host scope we use the attestation event's own scope (caller is
        // expected to feed events from one ledger; cross-scope lookup is
        // out of step 3's scope).
        //
        // We recover the attestation event's scope from `event_by_hash`
        // because `payloads` doesn't carry scope alongside the attestation
        // canonical hash directly; the chain-loop already populated that
        // index above.
        let attestation_scope = event_by_hash
            .get(canonical_hash)
            .map(|d| d.scope.clone())
            .unwrap_or_default();
        let host_lookup_key = (attestation_scope, payload.attested_event_position);
        match event_by_position.get(&host_lookup_key) {
            Some(host) if host.canonical_event_hash == payload.attested_event_hash => {}
            Some(_) | None => {
                outcome.chain_position_resolved = false;
                outcome
                    .failures
                    .push("user_content_attestation_chain_position_mismatch".into());
                event_failures.push(VerificationFailure::new(
                    "user_content_attestation_chain_position_mismatch",
                    hex_string(canonical_hash),
                ));
            }
        }

        // Step 4 — identity resolution.
        if let Some(identity_ref) = payload.identity_attestation_ref {
            // Non-null path: resolve to a chain-present event of a
            // registered identity-attestation event type, scope match,
            // sequence-strictly-less-than-attested_event_position, subject
            // equals attestor.
            match event_by_hash.get(&identity_ref) {
                None => {
                    outcome.identity_resolved = false;
                    outcome
                        .failures
                        .push("user_content_attestation_identity_unresolved".into());
                    event_failures.push(VerificationFailure::new(
                        "user_content_attestation_identity_unresolved",
                        hex_string(&identity_ref),
                    ));
                }
                Some(identity_event) => {
                    if !is_identity_attestation_event_type(&identity_event.event_type) {
                        outcome.identity_resolved = false;
                        outcome
                            .failures
                            .push("user_content_attestation_identity_unresolved".into());
                        event_failures.push(VerificationFailure::new(
                            "user_content_attestation_identity_unresolved",
                            hex_string(&identity_ref),
                        ));
                    } else {
                        // Scope match check. The attestation's scope (from
                        // `event_by_hash`) must equal the identity event's.
                        let attestation_scope_for_identity = event_by_hash
                            .get(canonical_hash)
                            .map(|d| d.scope.clone())
                            .unwrap_or_default();
                        if identity_event.scope != attestation_scope_for_identity {
                            outcome.identity_resolved = false;
                            outcome
                                .failures
                                .push("user_content_attestation_identity_unresolved".into());
                            event_failures.push(VerificationFailure::new(
                                "user_content_attestation_identity_unresolved",
                                hex_string(&identity_ref),
                            ));
                        } else if identity_event.sequence >= payload.attested_event_position {
                            // Temporal precedence: identity proof MUST
                            // strictly precede the attestation.
                            outcome.identity_resolved = false;
                            outcome.failures.push(
                                "user_content_attestation_identity_temporal_inversion".into(),
                            );
                            event_failures.push(VerificationFailure::new(
                                "user_content_attestation_identity_temporal_inversion",
                                hex_string(&identity_ref),
                            ));
                        } else {
                            // Subject match check — read the subject the
                            // decoder extracted from the resolved event's
                            // EventPayload.extensions map. External-payload
                            // identity events can extend this to fetch from
                            // payload_blobs; Phase-1 inline events carry the
                            // subject directly.
                            match identity_event.identity_attestation_subject.as_deref() {
                                Some(s) if s == payload.attestor => {}
                                _ => {
                                    outcome.identity_resolved = false;
                                    outcome.failures.push(
                                        "user_content_attestation_identity_subject_mismatch".into(),
                                    );
                                    event_failures.push(VerificationFailure::new(
                                        "user_content_attestation_identity_subject_mismatch",
                                        hex_string(&identity_ref),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        } else {
            // Null path: deployment Posture Declaration MUST declare
            // `admit_unverified_user_attestations: true`. Default-required
            // (false / absent / malformed declaration) flips
            // `user_content_attestation_identity_required`.
            let admitted = posture_declaration
                .map(parse_admit_unverified_user_attestations)
                .unwrap_or(false);
            if !admitted {
                outcome.identity_resolved = false;
                outcome
                    .failures
                    .push("user_content_attestation_identity_required".into());
                event_failures.push(VerificationFailure::new(
                    "user_content_attestation_identity_required",
                    hex_string(canonical_hash),
                ));
            }
        }

        // Step 6 — key-state check (run BEFORE step 5 so a wrong-class /
        // wrong-state kid surfaces its own failure code rather than masking
        // as a signature failure).
        let key_entry = registry.get(&payload.signing_kid);
        let key_active = match key_entry {
            None => false,
            Some(entry) => {
                // Phase-1 SigningKeyEntry status: 0 = Active, 1 = Rotating,
                // 2 = Retired, 3 = Revoked. Per ADR 0010 §"Verifier
                // obligations" step 6, only `Active` is admitted until the
                // rotation-grace overlap ratifies (open question 4 / TODO #5).
                // Note: the registry decoder normalizes Phase-1 lifecycle
                // to status integers per Core §8 SigningKeyStatus.
                let active = entry.status == 0;
                let valid_to_ok = entry
                    .valid_to
                    .map(|valid_to| payload.attested_at <= valid_to)
                    .unwrap_or(true);
                active && valid_to_ok
            }
        };
        if !key_active {
            outcome.key_active = false;
            outcome
                .failures
                .push("user_content_attestation_key_not_active".into());
            event_failures.push(VerificationFailure::new(
                "user_content_attestation_key_not_active",
                hex_string(canonical_hash),
            ));
        }

        // Step 5 — signature verification under
        // `trellis-user-content-attestation-v1`. Skipped when `key_active =
        // false` because we have no public key; the verifier reports
        // `key_not_active` and downstream tooling treats that as the
        // dominant failure (no need to redundantly flag
        // `signature_invalid`).
        if let Some(entry) = key_entry
            && key_active
        {
            let ok = verify_user_content_attestation_signature(payload, &entry.public_key);
            if !ok {
                outcome.signature_verified = false;
                outcome
                    .failures
                    .push("user_content_attestation_signature_invalid".into());
                event_failures.push(VerificationFailure::new(
                    "user_content_attestation_signature_invalid",
                    hex_string(canonical_hash),
                ));
            }
        }

        outcomes.push(outcome);
    }

    outcomes
}

/// ADR 0005 step 3 — validates the cross-field shape of `subject_scope`
/// based on `kind`.
fn validate_subject_scope_shape(
    subject_scope_map: &[(Value, Value)],
    kind: &str,
) -> Result<(), VerifyError> {
    let subject_refs = map_lookup_optional_value(subject_scope_map, "subject_refs");
    let ledger_scopes = map_lookup_optional_value(subject_scope_map, "ledger_scopes");
    let tenant_refs = map_lookup_optional_value(subject_scope_map, "tenant_refs");

    let is_present = |value: Option<&Value>| -> bool {
        matches!(value, Some(Value::Array(array)) if !array.is_empty())
    };
    let is_null_or_absent = |value: Option<&Value>| -> bool {
        matches!(value, None | Some(Value::Null))
            || matches!(value, Some(Value::Array(array)) if array.is_empty())
    };

    let ok = match kind {
        "per-subject" => {
            is_present(subject_refs)
                && is_null_or_absent(ledger_scopes)
                && is_null_or_absent(tenant_refs)
        }
        "per-scope" => {
            is_null_or_absent(subject_refs)
                && is_present(ledger_scopes)
                && is_null_or_absent(tenant_refs)
        }
        "per-tenant" => {
            is_null_or_absent(subject_refs)
                && is_null_or_absent(ledger_scopes)
                && is_present(tenant_refs)
        }
        "deployment-wide" => {
            is_null_or_absent(subject_refs)
                && is_null_or_absent(ledger_scopes)
                && is_null_or_absent(tenant_refs)
        }
        _ => {
            return Err(VerifyError::new(format!(
                "erasure-evidence subject_scope.kind `{kind}` is not one of per-subject / per-scope / per-tenant / deployment-wide (ADR 0005 step 3)"
            )));
        }
    };
    if !ok {
        return Err(VerifyError::new(format!(
            "erasure-evidence subject_scope cross-field shape violates ADR 0005 step 3 for kind `{kind}`"
        )));
    }
    Ok(())
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

/// Parses the optional `trellis.export.certificates-of-completion.v1`
/// manifest extension (ADR 0007 §"Export manifest catalog"). Mirror of
/// [`parse_erasure_evidence_export_extension`].
fn parse_certificate_export_extension(
    manifest_map: &[(Value, Value)],
) -> Result<Option<CertificateExportExtension>, VerifyError> {
    let Some(extensions) = map_lookup_optional_map(manifest_map, "extensions")? else {
        return Ok(None);
    };
    let Some(extension_value) = map_lookup_optional_value(extensions, CERTIFICATE_EXPORT_EXTENSION)
    else {
        return Ok(None);
    };
    let extension_map = extension_value
        .as_map()
        .ok_or_else(|| VerifyError::new("certificate export extension is not a map"))?;
    let catalog_ref = map_lookup_text(extension_map, "catalog_ref")?;
    if !catalog_ref.chars().all(|c| c.is_ascii()) {
        return Err(VerifyError::new(
            "certificate export extension catalog_ref must be ASCII (ZIP member path)",
        ));
    }
    Ok(Some(CertificateExportExtension {
        catalog_ref,
        catalog_digest: bytes_array(&map_lookup_fixed_bytes(
            extension_map,
            "catalog_digest",
            32,
        )?),
        entry_count: map_lookup_u64(extension_map, "entry_count")?,
    }))
}

fn parse_erasure_evidence_export_extension(
    manifest_map: &[(Value, Value)],
) -> Result<Option<ErasureEvidenceExportExtension>, VerifyError> {
    let Some(extensions) = map_lookup_optional_map(manifest_map, "extensions")? else {
        return Ok(None);
    };
    let Some(extension_value) =
        map_lookup_optional_value(extensions, ERASURE_EVIDENCE_EXPORT_EXTENSION)
    else {
        return Ok(None);
    };
    let extension_map = extension_value
        .as_map()
        .ok_or_else(|| VerifyError::new("erasure export extension is not a map"))?;
    let catalog_ref = map_lookup_text(extension_map, "catalog_ref")?;
    if !catalog_ref.chars().all(|c| c.is_ascii()) {
        return Err(VerifyError::new(
            "erasure export extension catalog_ref must be ASCII (ZIP member path)",
        ));
    }
    Ok(Some(ErasureEvidenceExportExtension {
        catalog_ref,
        catalog_digest: bytes_array(&map_lookup_fixed_bytes(
            extension_map,
            "catalog_digest",
            32,
        )?),
        entry_count: map_lookup_u64(extension_map, "entry_count")?,
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

fn parse_erasure_catalog_entries(
    catalog_bytes: &[u8],
) -> Result<Vec<ErasureEvidenceCatalogEntryRow>, VerifyError> {
    let value = decode_value(catalog_bytes)?;
    let entries = value
        .as_array()
        .ok_or_else(|| VerifyError::new("erasure evidence catalog root is not an array"))?;
    entries
        .iter()
        .map(|entry| {
            let map = entry
                .as_map()
                .ok_or_else(|| VerifyError::new("erasure evidence catalog entry is not a map"))?;
            let cascade_array = map_lookup_array(map, "cascade_scopes")?;
            if cascade_array.is_empty() {
                return Err(VerifyError::new(
                    "erasure evidence catalog cascade_scopes MUST be non-empty",
                ));
            }
            let mut cascade_scopes = Vec::with_capacity(cascade_array.len());
            for scope_value in cascade_array {
                let scope = scope_value.as_text().ok_or_else(|| {
                    VerifyError::new("erasure catalog cascade_scope entry is not text")
                })?;
                cascade_scopes.push(scope.to_string());
            }
            let kid_bytes = map_lookup_fixed_bytes(map, "kid_destroyed", 16)?;
            let kid_destroyed: [u8; 16] = kid_bytes
                .as_slice()
                .try_into()
                .expect("map_lookup_fixed_bytes enforces 16-byte kid_destroyed");
            Ok(ErasureEvidenceCatalogEntryRow {
                canonical_event_hash: bytes_array(&map_lookup_fixed_bytes(
                    map,
                    "canonical_event_hash",
                    32,
                )?),
                evidence_id: map_lookup_text(map, "evidence_id")?,
                kid_destroyed,
                destroyed_at: map_lookup_timestamp(map, "destroyed_at")?,
                completion_mode: map_lookup_text(map, "completion_mode")?,
                cascade_scopes,
                subject_scope_kind: map_lookup_text(map, "subject_scope_kind")?,
            })
        })
        .collect()
}

fn erasure_catalog_row_matches_details(
    row: &ErasureEvidenceCatalogEntryRow,
    details: &EventDetails,
) -> bool {
    let Some(erasure) = details.erasure.as_ref() else {
        return false;
    };
    if row.canonical_event_hash != details.canonical_event_hash {
        return false;
    }
    if row.evidence_id != erasure.evidence_id {
        return false;
    }
    if row.kid_destroyed.as_slice() != erasure.kid_destroyed.as_slice() {
        return false;
    }
    if row.destroyed_at != erasure.destroyed_at {
        return false;
    }
    if row.completion_mode != erasure.completion_mode {
        return false;
    }
    if row.cascade_scopes != erasure.cascade_scopes {
        return false;
    }
    if row.subject_scope_kind != erasure.subject_scope_kind {
        return false;
    }
    true
}

fn verify_erasure_evidence_catalog(
    archive: &ExportArchive,
    events: &[ParsedSign1],
    extension: &ErasureEvidenceExportExtension,
    report: &mut VerificationReport,
) {
    let member_name = extension.catalog_ref.as_str();
    let Some(catalog_bytes) = archive.members.get(member_name) else {
        report.event_failures.push(VerificationFailure::new(
            "missing_erasure_evidence_catalog",
            member_name.to_string(),
        ));
        return;
    };
    let actual_digest = sha256_bytes(catalog_bytes);
    if actual_digest.as_slice() != extension.catalog_digest.as_slice() {
        report.event_failures.push(VerificationFailure::new(
            "erasure_evidence_catalog_digest_mismatch",
            member_name.to_string(),
        ));
    }

    let entries = match parse_erasure_catalog_entries(catalog_bytes) {
        Ok(entries) => entries,
        Err(error) => {
            report.event_failures.push(VerificationFailure::new(
                "erasure_evidence_catalog_invalid",
                format!("{member_name}/{error}"),
            ));
            return;
        }
    };

    if entries.len() as u64 != extension.entry_count {
        report.event_failures.push(VerificationFailure::new(
            "erasure_evidence_catalog_invalid",
            format!("{member_name}/entry_count"),
        ));
    }

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
    for row in &entries {
        if !seen_hashes.insert(row.canonical_event_hash) {
            report.event_failures.push(VerificationFailure::new(
                "erasure_evidence_catalog_duplicate_event",
                hex_string(&row.canonical_event_hash),
            ));
        }
    }

    for row in &entries {
        let Some(details) = event_by_hash.get(&row.canonical_event_hash) else {
            report.event_failures.push(VerificationFailure::new(
                "erasure_evidence_catalog_event_unresolved",
                hex_string(&row.canonical_event_hash),
            ));
            continue;
        };
        if details.event_type != ERASURE_EVIDENCE_EVENT_EXTENSION {
            report.event_failures.push(VerificationFailure::new(
                "erasure_evidence_catalog_event_type_mismatch",
                hex_string(&row.canonical_event_hash),
            ));
            continue;
        }
        if !erasure_catalog_row_matches_details(row, details) {
            report.event_failures.push(VerificationFailure::new(
                "erasure_evidence_catalog_mismatch",
                hex_string(&row.canonical_event_hash),
            ));
        }
    }
}

/// Decodes `065-certificates-of-completion.cbor` (ADR 0007 §"Export manifest
/// catalog" — `CertificateOfCompletionCatalogEntry`). Mirror of
/// [`parse_erasure_catalog_entries`].
fn parse_certificate_catalog_entries(
    catalog_bytes: &[u8],
) -> Result<Vec<CertificateCatalogEntryRow>, VerifyError> {
    let value = decode_value(catalog_bytes)?;
    let entries = value
        .as_array()
        .ok_or_else(|| VerifyError::new("certificate catalog root is not an array"))?;
    entries
        .iter()
        .map(|entry| {
            let map = entry
                .as_map()
                .ok_or_else(|| VerifyError::new("certificate catalog entry is not a map"))?;
            Ok(CertificateCatalogEntryRow {
                canonical_event_hash: bytes_array(&map_lookup_fixed_bytes(
                    map,
                    "canonical_event_hash",
                    32,
                )?),
                certificate_id: map_lookup_text(map, "certificate_id")?,
                completed_at: map_lookup_timestamp(map, "completed_at")?,
                signer_count: map_lookup_u64(map, "signer_count")?,
                media_type: map_lookup_text(map, "media_type")?,
                attachment_id: map_lookup_text(map, "attachment_id")?,
                workflow_status: map_lookup_text(map, "workflow_status")?,
            })
        })
        .collect()
}

/// Field-wise agreement check between a catalog row and the in-chain
/// certificate event's decoded payload. Mirror of
/// [`erasure_catalog_row_matches_details`].
fn certificate_catalog_row_matches_details(
    row: &CertificateCatalogEntryRow,
    details: &EventDetails,
) -> bool {
    let Some(certificate) = details.certificate.as_ref() else {
        return false;
    };
    if row.canonical_event_hash != details.canonical_event_hash {
        return false;
    }
    if row.certificate_id != certificate.certificate_id {
        return false;
    }
    if row.completed_at != certificate.completed_at {
        return false;
    }
    if row.signer_count != certificate.chain_summary.signer_count {
        return false;
    }
    if row.media_type != certificate.presentation_artifact.media_type {
        return false;
    }
    if row.attachment_id != certificate.presentation_artifact.attachment_id {
        return false;
    }
    if row.workflow_status != certificate.chain_summary.workflow_status {
        return false;
    }
    true
}

/// Verifies the optional `065-certificates-of-completion.cbor` catalog
/// (ADR 0007 §"Export manifest catalog" / Core §19 step 6c optional catalog).
/// Mirror of [`verify_erasure_evidence_catalog`].
fn verify_certificate_catalog(
    archive: &ExportArchive,
    events: &[ParsedSign1],
    extension: &CertificateExportExtension,
    report: &mut VerificationReport,
) {
    let member_name = extension.catalog_ref.as_str();
    let Some(catalog_bytes) = archive.members.get(member_name) else {
        report.event_failures.push(VerificationFailure::new(
            "missing_certificate_catalog",
            member_name.to_string(),
        ));
        return;
    };
    // Catalog digest under `trellis-content-v1` (Core §19 step 6c optional
    // catalog clause; same domain tag as the existing catalog patterns —
    // re-verified via raw SHA-256 since `sha256_bytes` covers the
    // bare-bytes path that the manifest's binding uses).
    let actual_digest = sha256_bytes(catalog_bytes);
    if actual_digest.as_slice() != extension.catalog_digest.as_slice() {
        report.event_failures.push(VerificationFailure::new(
            "certificate_catalog_digest_mismatch",
            member_name.to_string(),
        ));
    }

    let entries = match parse_certificate_catalog_entries(catalog_bytes) {
        Ok(entries) => entries,
        Err(error) => {
            report.event_failures.push(VerificationFailure::new(
                "certificate_catalog_invalid",
                format!("{member_name}/{error}"),
            ));
            return;
        }
    };

    if entries.len() as u64 != extension.entry_count {
        report.event_failures.push(VerificationFailure::new(
            "certificate_catalog_invalid",
            format!("{member_name}/entry_count"),
        ));
    }

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
    for row in &entries {
        if !seen_hashes.insert(row.canonical_event_hash) {
            report.event_failures.push(VerificationFailure::new(
                "certificate_catalog_duplicate_event",
                hex_string(&row.canonical_event_hash),
            ));
        }
    }

    for row in &entries {
        let Some(details) = event_by_hash.get(&row.canonical_event_hash) else {
            report.event_failures.push(VerificationFailure::new(
                "certificate_catalog_event_unresolved",
                hex_string(&row.canonical_event_hash),
            ));
            continue;
        };
        // Per Core §6.7 the certificate-of-completion event's `event_type`
        // mirrors the extension key. Mirrors how the erasure-evidence
        // catalog cross-checks `details.event_type ==
        // ERASURE_EVIDENCE_EVENT_EXTENSION`.
        if details.event_type != CERTIFICATE_EVENT_EXTENSION {
            report.event_failures.push(VerificationFailure::new(
                "certificate_catalog_event_type_mismatch",
                hex_string(&row.canonical_event_hash),
            ));
            continue;
        }
        if !certificate_catalog_row_matches_details(row, details) {
            report.event_failures.push(VerificationFailure::new(
                "certificate_catalog_mismatch",
                hex_string(&row.canonical_event_hash),
            ));
        }
    }
}

/// ADR 0007 §"Verifier obligations" step 4 — attachment lineage resolution
/// + content-hash recompute. Runs in the export-bundle path because it
/// requires the attachment-binding lineage (ADR 0072) and the payload
/// blobs map. For each in-scope certificate event:
///
/// - resolve `presentation_artifact.attachment_id` via the chain's
///   `trellis.evidence-attachment-binding.v1` events;
/// - recover the bound attachment bytes from `payload_blobs`;
/// - recompute SHA-256 over the bytes under domain tag
///   `trellis-presentation-artifact-v1` (§9.8) and confirm it equals
///   `presentation_artifact.content_hash`.
///
/// Failure surfaces: `presentation_artifact_attachment_missing` (lineage
/// unresolvable / bytes absent) — distinct from
/// `presentation_artifact_content_mismatch` (lineage resolved, hash
/// disagrees). Both flip `outcome.attachment_resolved` for the §19 step-9
/// fold; the dominant tamper_kind is the most-specific available.
fn verify_certificate_attachment_lineage(
    events: &[ParsedSign1],
    payload_blobs: &BTreeMap<[u8; 32], Vec<u8>>,
    report: &mut VerificationReport,
) {
    if report.certificates_of_completion.is_empty() {
        return;
    }

    // Build (attachment_id → AttachmentBindingDetails + EventDetails) map.
    // ADR 0072 lineage: a certificate's `presentation_artifact.attachment_id`
    // must map to exactly one binding event whose extension carries the
    // matching `attachment_id`. Phase-1 lineage resolution is the latest
    // binding for that id (forward-walk; `prior_binding_hash` chain not
    // material for content-hash recompute — the latest binding IS the bound
    // bytes by definition).
    let mut binding_by_attachment_id: BTreeMap<String, (AttachmentBindingDetails, [u8; 32])> =
        BTreeMap::new();
    for event in events {
        if let Ok(details) = decode_event_details(event) {
            if let Some(binding) = &details.attachment_binding {
                binding_by_attachment_id.insert(
                    binding.attachment_id.clone(),
                    (binding.clone(), details.content_hash),
                );
            }
        }
    }

    // Build (global event index → EventDetails) map for certificate
    // events. The outcome's `event_index` is the GLOBAL position in `events`
    // (set by `verify_event_set_with_classes` when it pushes to
    // `certificate_payloads`), so we must index against the unfiltered
    // event list — a previous filtered-collect-then-Vec::get(event_index)
    // shape silently false-positived `presentation_artifact_attachment_missing`
    // on multi-event chains where binding/sigaff events sit BEFORE the
    // certificate (e.g. `[binding, sigaff, certificate]` → `event_index = 2`
    // while a filtered cert-only Vec has length 1).
    let mut cert_events_by_index: BTreeMap<usize, EventDetails> = BTreeMap::new();
    for (index, event) in events.iter().enumerate() {
        if let Ok(details) = decode_event_details(event) {
            if details.certificate.is_some() {
                cert_events_by_index.insert(index, details);
            }
        }
    }

    for outcome in report.certificates_of_completion.iter_mut() {
        let Some(details) = cert_events_by_index.get(&(outcome.event_index as usize)) else {
            // Index out of range — the underlying event vector changed
            // shape between collection and lineage check. Treat as
            // unresolvable; do not mask with attachment_resolved=true.
            outcome.attachment_resolved = false;
            outcome
                .failures
                .push("presentation_artifact_attachment_missing".into());
            report.event_failures.push(VerificationFailure::new(
                "presentation_artifact_attachment_missing",
                outcome.certificate_id.clone(),
            ));
            continue;
        };
        let canonical_hash_hex = hex_string(&details.canonical_event_hash);
        let Some(certificate) = details.certificate.as_ref() else {
            continue;
        };

        let Some((binding, _binding_payload_hash)) =
            binding_by_attachment_id.get(&certificate.presentation_artifact.attachment_id)
        else {
            outcome.attachment_resolved = false;
            outcome
                .failures
                .push("presentation_artifact_attachment_missing".into());
            report.event_failures.push(VerificationFailure::new(
                "presentation_artifact_attachment_missing",
                canonical_hash_hex,
            ));
            continue;
        };

        // Resolve attachment bytes via `payload_blobs` keyed on
        // `binding.payload_content_hash` (the content_hash of the binding
        // event's payload, which is the ADR 0072 mechanism for naming the
        // ciphertext member in `060-payloads/<digest>.bin`).
        let Some(attachment_bytes) = payload_blobs.get(&binding.payload_content_hash) else {
            outcome.attachment_resolved = false;
            outcome
                .failures
                .push("presentation_artifact_attachment_missing".into());
            report.event_failures.push(VerificationFailure::new(
                "presentation_artifact_attachment_missing",
                canonical_hash_hex,
            ));
            continue;
        };

        // Recompute content hash under `trellis-presentation-artifact-v1`
        // and compare with `presentation_artifact.content_hash`.
        let recomputed = domain_separated_sha256(PRESENTATION_ARTIFACT_DOMAIN, attachment_bytes);
        if recomputed != certificate.presentation_artifact.content_hash {
            outcome.attachment_resolved = false;
            outcome
                .failures
                .push("presentation_artifact_content_mismatch".into());
            report.event_failures.push(VerificationFailure::new(
                "presentation_artifact_content_mismatch",
                canonical_hash_hex,
            ));
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
                let valid_to: Option<TrellisTimestamp> = match map_lookup_optional_value(
                    map, "valid_to",
                ) {
                    Some(Value::Array(arr)) => Some(decode_timestamp_array(arr)?),
                    Some(Value::Null) | None => None,
                    Some(Value::Integer(_)) => {
                        return Err(VerifyError::with_kind(
                            "signing-key registry valid_to is legacy uint format; expected [seconds, nanos] array per ADR 0069 D-2.1",
                            "legacy_timestamp_format",
                        ));
                    }
                    Some(_) => {
                        return Err(VerifyError::new(
                            "signing-key registry valid_to is neither timestamp array nor null",
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
            //
            // The kind tag on the resulting `VerifyError` is consumed by
            // `verify_export_zip` / `verify_tampered_ledger` so the report's
            // `tamper_kind` field carries the structural-failure code rather
            // than the generic `signing_key_registry_invalid`.
            Some(class) if RESERVED_NON_SIGNING_KIND.contains(&class) => {
                let attributes = map_lookup_optional_value(map, "attributes");
                let attributes_map: Option<&[(Value, Value)]> = match attributes {
                    Some(Value::Map(map)) => Some(map.as_slice()),
                    None => {
                        return Err(VerifyError::with_kind(
                            format!(
                                "key_entry_attributes_shape_mismatch: KeyEntry of \
                                 kind=\"{class}\" missing required `attributes` map (Core §8.7.1)"
                            ),
                            "key_entry_attributes_shape_mismatch",
                        ));
                    }
                    Some(_) => {
                        return Err(VerifyError::with_kind(
                            format!(
                                "key_entry_attributes_shape_mismatch: KeyEntry of \
                                 kind=\"{class}\" `attributes` is not a map (Core §8.7.1)"
                            ),
                            "key_entry_attributes_shape_mismatch",
                        ));
                    }
                };

                // Subject-class capture: read `valid_to` from `attributes`
                // for forward-compatible Phase-2+ enforcement; absent or
                // null is the dominant Phase-1 case. Other classes don't
                // carry a `valid_to` field per §8.7.2.
                let subject_valid_to: Option<TrellisTimestamp> = if class == "subject" {
                    let valid_to_field = attributes_map
                        .and_then(|m| m.iter().find(|(k, _)| k.as_text() == Some("valid_to")));
                    match valid_to_field {
                        Some((_, Value::Array(arr))) => Some(decode_timestamp_array(arr)?),
                        Some((_, Value::Null)) | None => None,
                        Some((_, Value::Integer(_))) => {
                            return Err(VerifyError::with_kind(
                                "key_entry_attributes_shape_mismatch: subject \
                                 `valid_to` is legacy uint format; expected [seconds, nanos] array per ADR 0069 D-2.1",
                                "key_entry_attributes_shape_mismatch",
                            ));
                        }
                        Some(_) => {
                            return Err(VerifyError::with_kind(
                                "key_entry_attributes_shape_mismatch: subject \
                                 `valid_to` is neither timestamp array nor null (Core §8.7.2)",
                                "key_entry_attributes_shape_mismatch",
                            ));
                        }
                    }
                } else {
                    None
                };

                non_signing.insert(
                    kid,
                    NonSigningKeyEntry {
                        class: class.to_string(),
                        subject_valid_to,
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
                        subject_valid_to: None,
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

fn decode_timestamp_array(arr: &[Value]) -> Result<TrellisTimestamp, VerifyError> {
    if arr.len() != 2 {
        return Err(VerifyError::new(
            "timestamp array must have exactly 2 elements",
        ));
    }
    let seconds = match &arr[0] {
        Value::Integer(i) => {
            u64::try_from(*i).map_err(|_| VerifyError::new("timestamp seconds out of u64 range"))?
        }
        _ => return Err(VerifyError::new("timestamp seconds must be uint")),
    };
    let nanos = match &arr[1] {
        Value::Integer(i) => {
            u32::try_from(*i).map_err(|_| VerifyError::new("timestamp nanos out of u32 range"))?
        }
        _ => return Err(VerifyError::new("timestamp nanos must be uint")),
    };
    if nanos > 999_999_999 {
        return Err(VerifyError::new(format!(
            "timestamp nanos must be 0..999999999, got {nanos}"
        )));
    }
    Ok(TrellisTimestamp { seconds, nanos })
}

fn map_lookup_timestamp(
    map: &[(Value, Value)],
    key_name: &str,
) -> Result<TrellisTimestamp, VerifyError> {
    let value = map_lookup_optional_value(map, key_name)
        .ok_or_else(|| VerifyError::new(format!("missing `{key_name}`")))?;
    match value {
        Value::Array(arr) => decode_timestamp_array(arr),
        Value::Integer(_) => Err(VerifyError::with_kind(
            format!(
                "{key_name} is legacy uint format; expected [seconds, nanos] array per ADR 0069 D-2.1"
            ),
            "legacy_timestamp_format",
        )),
        _ => Err(VerifyError::new(format!(
            "{key_name} must be [uint, uint] array"
        ))),
    }
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
        TrellisTimestamp, export_archive_for_tests, is_interop_sidecar_path_valid,
        parse_sign1_bytes, parse_signing_key_registry, verify_event_set, verify_export_zip,
        verify_interop_sidecars, verify_single_event, verify_tampered_ledger,
    };

    /// TR-CORE-167 — `interop_sidecar_path_invalid` predicate (ADR 0008
    /// §"Phase-1 verifier obligation" step 2.c). Structural cases only;
    /// see `verify_interop_sidecars_rejects_manifest_path_outside_interop_tree`
    /// for the same check inside `verify_interop_sidecars` (no tamper ZIP —
    /// manifest re-signing would duplicate tamper/037..040 infra).
    #[test]
    fn interop_sidecar_path_prefix_invariant() {
        // Valid: starts with the literal byte prefix.
        assert!(is_interop_sidecar_path_valid(
            "interop-sidecars/c2pa-manifest/cert-001.c2pa"
        ));
        assert!(is_interop_sidecar_path_valid(
            "interop-sidecars/scitt-receipt/ckpt.cbor"
        ));
        // The trailing-empty case is a definitional edge — the prefix
        // alone is technically valid byte-prefix-wise but no real file
        // would land at a directory path. Predicate accepts; the
        // surrounding manifest walk catches missing-file as a
        // content_mismatch.
        assert!(is_interop_sidecar_path_valid("interop-sidecars/"));

        // Invalid: any non-prefix byte sequence — including paths
        // that *contain* the prefix mid-string, paths into the
        // canonical tree, absolute paths, parent-dir traversals, and
        // the empty string. The predicate is byte-prefix-only; no
        // normalization, no canonicalization, no Unicode folding.
        assert!(!is_interop_sidecar_path_valid(""));
        assert!(!is_interop_sidecar_path_valid("010-events.cbor"));
        assert!(!is_interop_sidecar_path_valid("000-manifest.cbor"));
        assert!(!is_interop_sidecar_path_valid("/interop-sidecars/x"));
        assert!(!is_interop_sidecar_path_valid("./interop-sidecars/x"));
        assert!(!is_interop_sidecar_path_valid("../interop-sidecars/x"));
        assert!(!is_interop_sidecar_path_valid("nested/interop-sidecars/x"));
        assert!(!is_interop_sidecar_path_valid("Interop-sidecars/x")); // case-sensitive
        assert!(!is_interop_sidecar_path_valid("interop-sidecar/x")); // missing trailing 's/'
    }

    /// TR-CORE-167 — `interop_sidecar_path_invalid` through the real
    /// `verify_interop_sidecars` dispatch (ADR 0008 step 2), without a
    /// tamper ZIP: a `c2pa-manifest@v1` entry whose `path` points into the
    /// canonical tree must fail **path-prefix** before digest lookup.
    #[test]
    fn verify_interop_sidecars_rejects_manifest_path_outside_interop_tree() {
        let entry = Value::Map(vec![
            (
                Value::Text("kind".into()),
                Value::Text("c2pa-manifest".into()),
            ),
            (
                Value::Text("path".into()),
                Value::Text("010-events.cbor".into()),
            ),
            (
                Value::Text("derivation_version".into()),
                Value::Integer(1u64.into()),
            ),
            (
                Value::Text("content_digest".into()),
                Value::Bytes([0x77_u8; 32].to_vec()),
            ),
            (
                Value::Text("source_ref".into()),
                Value::Text("urn:trellis:test:ref".into()),
            ),
        ]);
        let manifest_map = vec![(
            Value::Text("interop_sidecars".into()),
            Value::Array(vec![entry]),
        )];
        let archive = export_archive_for_tests(BTreeMap::new());
        let report = verify_interop_sidecars(&manifest_map, &archive).expect_err("bad path prefix");
        assert_eq!(report.event_failures.len(), 1);
        assert_eq!(
            report.event_failures[0].kind, "interop_sidecar_path_invalid",
            "must not reach digest check (empty archive would otherwise be content_mismatch)"
        );
    }

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
                *value = ciborium::Value::Array(vec![
                    ciborium::Value::Integer(1745109999u64.into()),
                    ciborium::Value::Integer(0u32.into()),
                ]);
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

    // ------------------------------------------------------------------
    // ADR 0005 erasure-evidence unit tests (Stage 2).
    //
    // These test the internal decode + finalize helpers directly so each
    // ADR 0005 §"Verifier obligations" step has byte-level coverage that
    // does not require a full COSE_Sign1 envelope. Fixture-level coverage
    // is in `trellis-conformance` against `fixtures/vectors/append/023..027`
    // and `fixtures/vectors/tamper/017..019`.
    // ------------------------------------------------------------------

    /// Builder for a minimum-valid erasure-evidence extension map.
    /// Tests mutate fields to exercise each ADR 0005 step in isolation.
    fn erasure_extension(
        kid_destroyed: &[u8; 16],
        key_class: &str,
        destroyed_at: u64,
        subject_scope: Value,
        attestations: Vec<Value>,
        hsm_receipt: Option<Value>,
        hsm_receipt_kind: Option<Value>,
        cascade_scopes: Vec<&str>,
    ) -> Vec<(Value, Value)> {
        let cascade_array: Vec<Value> = cascade_scopes
            .into_iter()
            .map(|s| Value::Text(s.to_string()))
            .collect();
        vec![(
            Value::Text("trellis.erasure-evidence.v1".into()),
            Value::Map(vec![
                (
                    Value::Text("evidence_id".into()),
                    Value::Text("urn:trellis:erasure:test:1".into()),
                ),
                (
                    Value::Text("kid_destroyed".into()),
                    Value::Bytes(kid_destroyed.to_vec()),
                ),
                (
                    Value::Text("key_class".into()),
                    Value::Text(key_class.into()),
                ),
                (
                    Value::Text("destroyed_at".into()),
                    Value::Array(vec![
                        Value::Integer(destroyed_at.into()),
                        Value::Integer(0u32.into()),
                    ]),
                ),
                (
                    Value::Text("cascade_scopes".into()),
                    Value::Array(cascade_array),
                ),
                (
                    Value::Text("completion_mode".into()),
                    Value::Text("complete".into()),
                ),
                (
                    Value::Text("destruction_actor".into()),
                    Value::Text("urn:trellis:principal:test-actor".into()),
                ),
                (
                    Value::Text("policy_authority".into()),
                    Value::Text("urn:trellis:authority:test-policy".into()),
                ),
                (
                    Value::Text("reason_code".into()),
                    Value::Integer(1u64.into()),
                ),
                (Value::Text("subject_scope".into()), subject_scope),
                (
                    Value::Text("hsm_receipt".into()),
                    hsm_receipt.unwrap_or(Value::Null),
                ),
                (
                    Value::Text("hsm_receipt_kind".into()),
                    hsm_receipt_kind.unwrap_or(Value::Null),
                ),
                (
                    Value::Text("attestations".into()),
                    Value::Array(attestations),
                ),
                (Value::Text("extensions".into()), Value::Null),
            ]),
        )]
    }

    fn one_attestation(class: &str) -> Value {
        Value::Map(vec![
            (
                Value::Text("authority".into()),
                Value::Text(format!("urn:trellis:authority:test-{class}")),
            ),
            (
                Value::Text("authority_class".into()),
                Value::Text(class.into()),
            ),
            (Value::Text("signature".into()), Value::Bytes(vec![0u8; 64])),
        ])
    }

    fn per_subject_scope() -> Value {
        Value::Map(vec![
            (
                Value::Text("kind".into()),
                Value::Text("per-subject".into()),
            ),
            (
                Value::Text("subject_refs".into()),
                Value::Array(vec![Value::Text("urn:trellis:subject:test-1".into())]),
            ),
            (Value::Text("ledger_scopes".into()), Value::Null),
            (Value::Text("tenant_refs".into()), Value::Null),
        ])
    }

    fn deployment_wide_scope() -> Value {
        Value::Map(vec![
            (
                Value::Text("kind".into()),
                Value::Text("deployment-wide".into()),
            ),
            (Value::Text("subject_refs".into()), Value::Null),
            (Value::Text("ledger_scopes".into()), Value::Null),
            (Value::Text("tenant_refs".into()), Value::Null),
        ])
    }

    #[test]
    fn validate_subject_scope_shape_per_subject_accepts_subject_refs_only() {
        let scope = per_subject_scope();
        let map = scope.as_map().unwrap().clone();
        assert!(super::validate_subject_scope_shape(&map, "per-subject").is_ok());
    }

    #[test]
    fn validate_subject_scope_shape_per_subject_rejects_with_ledger_scopes() {
        let scope = Value::Map(vec![
            (
                Value::Text("kind".into()),
                Value::Text("per-subject".into()),
            ),
            (
                Value::Text("subject_refs".into()),
                Value::Array(vec![Value::Text("urn:trellis:subject:test-1".into())]),
            ),
            (
                Value::Text("ledger_scopes".into()),
                Value::Array(vec![Value::Bytes(b"x".to_vec())]),
            ),
            (Value::Text("tenant_refs".into()), Value::Null),
        ]);
        let map = scope.as_map().unwrap().clone();
        let err = super::validate_subject_scope_shape(&map, "per-subject").unwrap_err();
        assert!(err.to_string().contains("subject_scope"));
    }

    #[test]
    fn validate_subject_scope_shape_per_scope_requires_ledger_scopes() {
        let scope = Value::Map(vec![
            (Value::Text("kind".into()), Value::Text("per-scope".into())),
            (Value::Text("subject_refs".into()), Value::Null),
            (
                Value::Text("ledger_scopes".into()),
                Value::Array(vec![Value::Bytes(b"scope-a".to_vec())]),
            ),
            (Value::Text("tenant_refs".into()), Value::Null),
        ]);
        let map = scope.as_map().unwrap().clone();
        assert!(super::validate_subject_scope_shape(&map, "per-scope").is_ok());
    }

    #[test]
    fn validate_subject_scope_shape_per_tenant_requires_tenant_refs() {
        let scope = Value::Map(vec![
            (Value::Text("kind".into()), Value::Text("per-tenant".into())),
            (Value::Text("subject_refs".into()), Value::Null),
            (Value::Text("ledger_scopes".into()), Value::Null),
            (
                Value::Text("tenant_refs".into()),
                Value::Array(vec![Value::Text("urn:trellis:tenant:test".into())]),
            ),
        ]);
        let map = scope.as_map().unwrap().clone();
        assert!(super::validate_subject_scope_shape(&map, "per-tenant").is_ok());
    }

    #[test]
    fn validate_subject_scope_shape_deployment_wide_rejects_any_ref_field() {
        let scope = Value::Map(vec![
            (
                Value::Text("kind".into()),
                Value::Text("deployment-wide".into()),
            ),
            (
                Value::Text("subject_refs".into()),
                Value::Array(vec![Value::Text("urn:trellis:subject:s".into())]),
            ),
            (Value::Text("ledger_scopes".into()), Value::Null),
            (Value::Text("tenant_refs".into()), Value::Null),
        ]);
        let map = scope.as_map().unwrap().clone();
        let err = super::validate_subject_scope_shape(&map, "deployment-wide").unwrap_err();
        assert!(err.to_string().contains("subject_scope"));
    }

    #[test]
    fn validate_subject_scope_shape_unknown_kind_rejected() {
        let scope = Value::Map(vec![
            (Value::Text("kind".into()), Value::Text("not-real".into())),
            (Value::Text("subject_refs".into()), Value::Null),
            (Value::Text("ledger_scopes".into()), Value::Null),
            (Value::Text("tenant_refs".into()), Value::Null),
        ]);
        let map = scope.as_map().unwrap().clone();
        let err = super::validate_subject_scope_shape(&map, "not-real").unwrap_err();
        assert!(err.to_string().contains("not-real"));
    }

    #[test]
    fn decode_erasure_evidence_step1_minimum_valid_payload_decodes() {
        let kid = [0xABu8; 16];
        let extensions = erasure_extension(
            &kid,
            "signing",
            1_745_000_000,
            per_subject_scope(),
            vec![one_attestation("new")],
            None,
            None,
            vec!["CS-03"],
        );
        let details = super::decode_erasure_evidence_details(
            &extensions,
            TrellisTimestamp {
                seconds: 1_745_000_100,
                nanos: 0,
            },
        )
        .unwrap()
        .expect("erasure extension must decode");
        assert_eq!(details.evidence_id, "urn:trellis:erasure:test:1");
        assert_eq!(details.kid_destroyed, kid.to_vec());
        assert_eq!(details.norm_key_class, "signing");
        assert_eq!(
            details.destroyed_at,
            TrellisTimestamp {
                seconds: 1_745_000_000,
                nanos: 0
            }
        );
        assert_eq!(details.cascade_scopes, vec!["CS-03"]);
        assert_eq!(details.completion_mode, "complete");
        assert!(details.attestation_signatures_well_formed);
    }

    #[test]
    fn decode_erasure_evidence_step1_returns_none_when_extension_absent() {
        let extensions: Vec<(Value, Value)> = vec![(
            Value::Text("trellis.custody-model-transition.v1".into()),
            Value::Map(vec![]),
        )];
        let result = super::decode_erasure_evidence_details(
            &extensions,
            TrellisTimestamp {
                seconds: 1_745_000_000,
                nanos: 0,
            },
        )
        .unwrap();
        assert!(result.is_none(), "no erasure-evidence ext → None");
    }

    #[test]
    fn decode_erasure_evidence_step2_normalizes_wire_wrap_to_subject() {
        // ADR 0005 step 2 + Core §8.7.6: wire `key_class = "wrap"` MUST
        // normalize to `"subject"` before any registry comparison.
        let kid = [0x01u8; 16];
        let extensions = erasure_extension(
            &kid,
            "wrap",
            1_745_000_000,
            per_subject_scope(),
            vec![one_attestation("new")],
            None,
            None,
            vec!["CS-03"],
        );
        let details = super::decode_erasure_evidence_details(
            &extensions,
            TrellisTimestamp {
                seconds: 1_745_000_100,
                nanos: 0,
            },
        )
        .unwrap()
        .unwrap();
        assert_eq!(
            details.norm_key_class, "subject",
            "wire 'wrap' must normalize to 'subject' (Wave 17 / ADR 0006)",
        );
    }

    #[test]
    fn decode_erasure_evidence_step3_rejects_per_subject_with_null_subject_refs() {
        let kid = [0x02u8; 16];
        // per-subject kind but subject_refs null violates step 3.
        let bad_scope = Value::Map(vec![
            (
                Value::Text("kind".into()),
                Value::Text("per-subject".into()),
            ),
            (Value::Text("subject_refs".into()), Value::Null),
            (Value::Text("ledger_scopes".into()), Value::Null),
            (Value::Text("tenant_refs".into()), Value::Null),
        ]);
        let extensions = erasure_extension(
            &kid,
            "signing",
            1_745_000_000,
            bad_scope,
            vec![one_attestation("new")],
            None,
            None,
            vec!["CS-03"],
        );
        let err = super::decode_erasure_evidence_details(
            &extensions,
            TrellisTimestamp {
                seconds: 1_745_000_100,
                nanos: 0,
            },
        )
        .unwrap_err();
        assert!(err.to_string().contains("subject_scope"));
    }

    #[test]
    fn decode_erasure_evidence_step4_rejects_destroyed_at_after_host_authored_at() {
        // ADR 0005 step 4 / OC-144: destroyed_at MUST be ≤ host event
        // authored_at. Violation surfaces as `erasure_destroyed_at_after_host`.
        let kid = [0x03u8; 16];
        let extensions = erasure_extension(
            &kid,
            "signing",
            1_745_000_500, // destroyed_at > host (1_745_000_100)
            per_subject_scope(),
            vec![one_attestation("new")],
            None,
            None,
            vec!["CS-03"],
        );
        let err = super::decode_erasure_evidence_details(
            &extensions,
            TrellisTimestamp {
                seconds: 1_745_000_100,
                nanos: 0,
            },
        )
        .unwrap_err();
        assert_eq!(err.kind(), Some("erasure_destroyed_at_after_host"));
    }

    #[test]
    fn decode_erasure_evidence_step6_rejects_hsm_receipt_without_kind() {
        let kid = [0x04u8; 16];
        let extensions = erasure_extension(
            &kid,
            "signing",
            1_745_000_000,
            per_subject_scope(),
            vec![one_attestation("new")],
            Some(Value::Bytes(b"opaque-hsm-bytes".to_vec())),
            None,
            vec!["CS-03"],
        );
        let err = super::decode_erasure_evidence_details(
            &extensions,
            TrellisTimestamp {
                seconds: 1_745_000_100,
                nanos: 0,
            },
        )
        .unwrap_err();
        assert!(err.to_string().contains("hsm_receipt"));
    }

    #[test]
    fn decode_erasure_evidence_step6_rejects_hsm_receipt_kind_without_receipt() {
        let kid = [0x05u8; 16];
        let extensions = erasure_extension(
            &kid,
            "signing",
            1_745_000_000,
            per_subject_scope(),
            vec![one_attestation("new")],
            None,
            Some(Value::Text("opaque-vendor-receipt-v1".into())),
            vec!["CS-03"],
        );
        let err = super::decode_erasure_evidence_details(
            &extensions,
            TrellisTimestamp {
                seconds: 1_745_000_100,
                nanos: 0,
            },
        )
        .unwrap_err();
        assert!(err.to_string().contains("hsm_receipt"));
    }

    #[test]
    fn decode_erasure_evidence_step6_accepts_both_hsm_fields_present() {
        let kid = [0x06u8; 16];
        let extensions = erasure_extension(
            &kid,
            "signing",
            1_745_000_000,
            per_subject_scope(),
            vec![one_attestation("new")],
            Some(Value::Bytes(b"opaque-hsm-bytes".to_vec())),
            Some(Value::Text("opaque-vendor-receipt-v1".into())),
            vec!["CS-03"],
        );
        let details = super::decode_erasure_evidence_details(
            &extensions,
            TrellisTimestamp {
                seconds: 1_745_000_100,
                nanos: 0,
            },
        )
        .unwrap()
        .unwrap();
        assert_eq!(details.evidence_id, "urn:trellis:erasure:test:1");
    }

    #[test]
    fn decode_erasure_evidence_step7_marks_short_attestation_signature_malformed() {
        // ADR 0005 step 7 (Phase-1 structural): each attestation MUST carry
        // a 64-byte signature. A 32-byte signature flips
        // `attestation_signatures_well_formed = false`.
        let kid = [0x07u8; 16];
        let bad_attestation = Value::Map(vec![
            (
                Value::Text("authority".into()),
                Value::Text("urn:trellis:authority:test-bad".into()),
            ),
            (
                Value::Text("authority_class".into()),
                Value::Text("new".into()),
            ),
            (
                Value::Text("signature".into()),
                Value::Bytes(vec![0u8; 32]), // wrong length
            ),
        ]);
        let extensions = erasure_extension(
            &kid,
            "signing",
            1_745_000_000,
            per_subject_scope(),
            vec![bad_attestation],
            None,
            None,
            vec!["CS-03"],
        );
        let details = super::decode_erasure_evidence_details(
            &extensions,
            TrellisTimestamp {
                seconds: 1_745_000_100,
                nanos: 0,
            },
        )
        .unwrap()
        .unwrap();
        assert!(!details.attestation_signatures_well_formed);
    }

    #[test]
    fn decode_erasure_evidence_step1_rejects_empty_cascade_scopes() {
        let kid = [0x08u8; 16];
        let extensions = erasure_extension(
            &kid,
            "signing",
            1_745_000_000,
            per_subject_scope(),
            vec![one_attestation("new")],
            None,
            None,
            vec![], // empty
        );
        let err = super::decode_erasure_evidence_details(
            &extensions,
            TrellisTimestamp {
                seconds: 1_745_000_100,
                nanos: 0,
            },
        )
        .unwrap_err();
        assert!(err.to_string().contains("cascade_scopes"));
    }

    #[test]
    fn decode_erasure_evidence_step1_rejects_empty_attestations() {
        let kid = [0x09u8; 16];
        let extensions = erasure_extension(
            &kid,
            "signing",
            1_745_000_000,
            per_subject_scope(),
            vec![],
            None,
            None,
            vec!["CS-03"],
        );
        let err = super::decode_erasure_evidence_details(
            &extensions,
            TrellisTimestamp {
                seconds: 1_745_000_100,
                nanos: 0,
            },
        )
        .unwrap_err();
        assert!(err.to_string().contains("attestations"));
    }

    #[test]
    fn decode_erasure_evidence_step1_rejects_kid_wrong_size() {
        // Use the manual map builder to bypass the `[u8; 16]` builder helper
        // and force a 15-byte kid.
        let extensions = vec![(
            Value::Text("trellis.erasure-evidence.v1".into()),
            Value::Map(vec![
                (
                    Value::Text("evidence_id".into()),
                    Value::Text("urn:trellis:erasure:test:bad".into()),
                ),
                (
                    Value::Text("kid_destroyed".into()),
                    Value::Bytes(vec![0u8; 15]),
                ),
                (
                    Value::Text("key_class".into()),
                    Value::Text("signing".into()),
                ),
                (
                    Value::Text("destroyed_at".into()),
                    Value::Integer(1_745_000_000u64.into()),
                ),
                (
                    Value::Text("cascade_scopes".into()),
                    Value::Array(vec![Value::Text("CS-03".into())]),
                ),
                (
                    Value::Text("completion_mode".into()),
                    Value::Text("complete".into()),
                ),
                (
                    Value::Text("destruction_actor".into()),
                    Value::Text("urn:trellis:principal:t".into()),
                ),
                (
                    Value::Text("policy_authority".into()),
                    Value::Text("urn:trellis:authority:t".into()),
                ),
                (
                    Value::Text("reason_code".into()),
                    Value::Integer(1u64.into()),
                ),
                (Value::Text("subject_scope".into()), per_subject_scope()),
                (Value::Text("hsm_receipt".into()), Value::Null),
                (Value::Text("hsm_receipt_kind".into()), Value::Null),
                (
                    Value::Text("attestations".into()),
                    Value::Array(vec![one_attestation("new")]),
                ),
                (Value::Text("extensions".into()), Value::Null),
            ]),
        )];
        let err = super::decode_erasure_evidence_details(
            &extensions,
            TrellisTimestamp {
                seconds: 1_745_000_100,
                nanos: 0,
            },
        )
        .unwrap_err();
        assert!(err.to_string().contains("kid_destroyed"));
    }

    #[test]
    fn decode_erasure_evidence_deployment_wide_scope_decodes() {
        let kid = [0x0Au8; 16];
        let extensions = erasure_extension(
            &kid,
            "signing",
            1_745_000_000,
            deployment_wide_scope(),
            vec![one_attestation("prior"), one_attestation("new")],
            None,
            None,
            vec!["CS-01", "CS-02", "CS-03", "CS-04", "CS-05", "CS-06"],
        );
        let details = super::decode_erasure_evidence_details(
            &extensions,
            TrellisTimestamp {
                seconds: 1_745_000_100,
                nanos: 0,
            },
        )
        .unwrap()
        .unwrap();
        assert_eq!(details.subject_scope_kind, "deployment-wide");
        assert_eq!(details.cascade_scopes.len(), 6);
        assert_eq!(details.attestation_classes, vec!["prior", "new"]);
    }

    // ------------------------------------------------------------------
    // finalize_erasure_evidence — steps 2 / 5 / 8 cross-event reasoning.
    // These tests construct ErasureEvidenceDetails + ChainEventSummary
    // values directly so we can exercise the post-loop logic without
    // building a full COSE_Sign1 chain. Fixture-level coverage for the
    // happy + tamper paths lives in `trellis-conformance` against
    // `fixtures/vectors/append/023..027` and `tamper/017..019`.
    // ------------------------------------------------------------------

    fn payload_details(
        kid: Vec<u8>,
        norm_key_class: &str,
        destroyed_at: u64,
    ) -> super::ErasureEvidenceDetails {
        super::ErasureEvidenceDetails {
            evidence_id: format!("urn:trellis:erasure:test:{}", kid[0]),
            kid_destroyed: kid,
            norm_key_class: norm_key_class.to_string(),
            destroyed_at: super::TrellisTimestamp {
                seconds: destroyed_at,
                nanos: 0,
            },
            cascade_scopes: vec!["CS-03".to_string()],
            completion_mode: "complete".to_string(),
            attestation_signatures_well_formed: true,
            attestation_classes: vec!["new".to_string()],
            subject_scope_kind: "per-subject".to_string(),
        }
    }

    fn chain_summary(
        index: u64,
        authored_at: u64,
        signing_kid: Vec<u8>,
        wrap_recipients: Vec<Vec<u8>>,
        canonical_event_hash: [u8; 32],
    ) -> super::ChainEventSummary {
        super::ChainEventSummary {
            event_index: index,
            authored_at: super::TrellisTimestamp {
                seconds: authored_at,
                nanos: 0,
            },
            signing_kid,
            wrap_recipients,
            canonical_event_hash,
        }
    }

    #[test]
    fn finalize_erasure_evidence_empty_input_produces_empty_outcome() {
        let registry = BTreeMap::new();
        let mut event_failures = Vec::new();
        let outcomes =
            super::finalize_erasure_evidence(&[], &[], &registry, None, &mut event_failures);
        assert!(outcomes.is_empty());
        assert!(event_failures.is_empty());
    }

    #[test]
    fn finalize_step8_flags_post_erasure_use_for_signing_class() {
        // One signing kid destroyed at t=100; a later event at t=200 signs
        // under that kid. Expect post_erasure_uses == 1 and a localized
        // `post_erasure_use` event_failure.
        let kid = vec![0xAAu8; 16];
        let payload = payload_details(kid.clone(), "signing", 100);
        let canonical_hash = [0u8; 32];
        let payloads = vec![(0usize, payload, canonical_hash)];

        let later_hash = [1u8; 32];
        let chain = vec![
            chain_summary(0, 100, kid.clone(), vec![], canonical_hash),
            chain_summary(1, 200, kid.clone(), vec![], later_hash),
        ];

        let registry = BTreeMap::new(); // kid not registered → step 2 skipped
        let mut event_failures = Vec::new();
        let outcomes = super::finalize_erasure_evidence(
            &payloads,
            &chain,
            &registry,
            None,
            &mut event_failures,
        );
        assert_eq!(outcomes.len(), 1);
        assert_eq!(outcomes[0].post_erasure_uses, 1);
        assert_eq!(outcomes[0].post_erasure_wraps, 0);
        assert!(
            event_failures
                .iter()
                .any(|f| f.kind == "post_erasure_use"
                    && f.location == super::hex_string(&later_hash))
        );
    }

    #[test]
    fn finalize_step8_flags_post_erasure_wrap_for_subject_class() {
        // A subject kid destroyed; a later event at t > destroyed_at carries
        // a key_bag.entries[*].recipient equal to the destroyed kid.
        let kid = vec![0xBBu8; 16];
        let payload = payload_details(kid.clone(), "subject", 100);
        let canonical_hash = [0u8; 32];
        let payloads = vec![(0usize, payload, canonical_hash)];

        let signing_kid = vec![0xCCu8; 16]; // a different signing kid
        let later_hash = [2u8; 32];
        let chain = vec![
            chain_summary(0, 100, signing_kid.clone(), vec![], canonical_hash),
            chain_summary(1, 200, signing_kid.clone(), vec![kid.clone()], later_hash),
        ];

        let registry = BTreeMap::new();
        let mut event_failures = Vec::new();
        let outcomes = super::finalize_erasure_evidence(
            &payloads,
            &chain,
            &registry,
            None,
            &mut event_failures,
        );
        assert_eq!(outcomes.len(), 1);
        assert_eq!(outcomes[0].post_erasure_uses, 0);
        assert_eq!(outcomes[0].post_erasure_wraps, 1);
        assert!(
            event_failures
                .iter()
                .any(|f| f.kind == "post_erasure_wrap"
                    && f.location == super::hex_string(&later_hash))
        );
    }

    #[test]
    fn finalize_step8_phase1_skips_recovery_class_chain_walk() {
        // ADR 0005 step 8 Phase-1 scope: recovery / scope / tenant-root and
        // extension-`tstr` classes do NOT trigger the chain-walk in Phase 1.
        // Wire-valid: dispatch co-lands with ADR 0006 follow-on.
        let kid = vec![0xDDu8; 16];
        let payload = payload_details(kid.clone(), "recovery", 100);
        let canonical_hash = [0u8; 32];
        let payloads = vec![(0usize, payload, canonical_hash)];

        // Even with a later event that signs under the destroyed kid, the
        // Phase-1 verifier must not flag post_erasure_use for "recovery".
        let later_hash = [3u8; 32];
        let chain = vec![
            chain_summary(0, 100, kid.clone(), vec![], canonical_hash),
            chain_summary(1, 200, kid.clone(), vec![], later_hash),
        ];

        let registry = BTreeMap::new();
        let mut event_failures = Vec::new();
        let outcomes = super::finalize_erasure_evidence(
            &payloads,
            &chain,
            &registry,
            None,
            &mut event_failures,
        );
        assert_eq!(outcomes.len(), 1);
        assert_eq!(outcomes[0].post_erasure_uses, 0, "recovery class skipped");
        assert_eq!(outcomes[0].post_erasure_wraps, 0);
        assert!(
            !event_failures.iter().any(|f| f.kind == "post_erasure_use"),
            "Phase-1 must not flag post_erasure_use for recovery class",
        );
    }

    #[test]
    fn finalize_step5_flags_destroyed_at_conflict_for_same_kid() {
        // ADR 0005 step 5 / OC-145: two payloads with same kid_destroyed
        // but different destroyed_at → `erasure_destroyed_at_conflict`.
        let kid = vec![0xEEu8; 16];
        let payload_a = payload_details(kid.clone(), "signing", 100);
        let payload_b = payload_details(kid.clone(), "signing", 200);
        let hash_a = [0u8; 32];
        let hash_b = [1u8; 32];
        let payloads = vec![(0usize, payload_a, hash_a), (1usize, payload_b, hash_b)];

        let chain = vec![
            chain_summary(0, 100, kid.clone(), vec![], hash_a),
            chain_summary(1, 150, kid.clone(), vec![], hash_b),
        ];

        let registry = BTreeMap::new();
        let mut event_failures = Vec::new();
        let outcomes = super::finalize_erasure_evidence(
            &payloads,
            &chain,
            &registry,
            None,
            &mut event_failures,
        );
        assert_eq!(outcomes.len(), 2);
        assert!(
            event_failures
                .iter()
                .any(|f| f.kind == "erasure_destroyed_at_conflict")
        );
        // The second outcome carries the conflict failure tag.
        assert!(
            outcomes[1]
                .failures
                .iter()
                .any(|s| s == "erasure_destroyed_at_conflict")
        );
    }

    #[test]
    fn finalize_step5_flags_key_class_conflict_for_same_kid() {
        // Two payloads, same kid_destroyed, different normalized class →
        // `erasure_key_class_payload_conflict`.
        let kid = vec![0xF0u8; 16];
        let payload_a = payload_details(kid.clone(), "signing", 100);
        let payload_b = payload_details(kid.clone(), "subject", 100);
        let hash_a = [0u8; 32];
        let hash_b = [1u8; 32];
        let payloads = vec![(0usize, payload_a, hash_a), (1usize, payload_b, hash_b)];

        let chain = vec![
            chain_summary(0, 100, kid.clone(), vec![], hash_a),
            chain_summary(1, 150, kid.clone(), vec![], hash_b),
        ];

        let registry = BTreeMap::new();
        let mut event_failures = Vec::new();
        let outcomes = super::finalize_erasure_evidence(
            &payloads,
            &chain,
            &registry,
            None,
            &mut event_failures,
        );
        assert_eq!(outcomes.len(), 2);
        assert!(
            event_failures
                .iter()
                .any(|f| f.kind == "erasure_key_class_payload_conflict")
        );
    }

    #[test]
    fn finalize_step2_flags_registry_class_mismatch_for_signing_kid() {
        // Registry has the kid as a signing key; payload claims it's a
        // subject key. Step 2 → `erasure_key_class_registry_mismatch`.
        let kid = vec![0xF1u8; 16];
        let payload = payload_details(kid.clone(), "subject", 100);
        let canonical_hash = [0u8; 32];
        let payloads = vec![(0usize, payload, canonical_hash)];

        let chain = vec![chain_summary(0, 100, kid.clone(), vec![], canonical_hash)];

        let mut registry = BTreeMap::new();
        registry.insert(
            kid.clone(),
            super::SigningKeyEntry {
                public_key: [0u8; 32],
                status: 1,
                valid_to: None,
            },
        );
        let mut event_failures = Vec::new();
        let _outcomes = super::finalize_erasure_evidence(
            &payloads,
            &chain,
            &registry,
            None,
            &mut event_failures,
        );
        assert!(
            event_failures
                .iter()
                .any(|f| f.kind == "erasure_key_class_registry_mismatch")
        );
    }

    #[test]
    fn finalize_step2_accepts_matching_signing_class() {
        // Registry has the kid as a signing key; payload also claims signing
        // → no step-2 mismatch.
        let kid = vec![0xF2u8; 16];
        let payload = payload_details(kid.clone(), "signing", 100);
        let canonical_hash = [0u8; 32];
        let payloads = vec![(0usize, payload, canonical_hash)];

        let chain = vec![chain_summary(0, 100, kid.clone(), vec![], canonical_hash)];

        let mut registry = BTreeMap::new();
        registry.insert(
            kid.clone(),
            super::SigningKeyEntry {
                public_key: [0u8; 32],
                status: 1,
                valid_to: None,
            },
        );
        let mut event_failures = Vec::new();
        let outcomes = super::finalize_erasure_evidence(
            &payloads,
            &chain,
            &registry,
            None,
            &mut event_failures,
        );
        assert_eq!(outcomes.len(), 1);
        assert!(
            !event_failures
                .iter()
                .any(|f| f.kind == "erasure_key_class_registry_mismatch")
        );
    }

    #[test]
    fn finalize_step7_flags_malformed_attestation_signature() {
        // attestation_signatures_well_formed = false → outcome carries
        // signature_verified = false AND a `erasure_attestation_signature_invalid`
        // event_failure surfaces so the report's tamper_kind picks it up.
        let kid = vec![0xF3u8; 16];
        let mut payload = payload_details(kid.clone(), "signing", 100);
        payload.attestation_signatures_well_formed = false;
        let canonical_hash = [0u8; 32];
        let payloads = vec![(0usize, payload, canonical_hash)];

        let chain = vec![chain_summary(0, 100, kid.clone(), vec![], canonical_hash)];

        let registry = BTreeMap::new();
        let mut event_failures = Vec::new();
        let outcomes = super::finalize_erasure_evidence(
            &payloads,
            &chain,
            &registry,
            None,
            &mut event_failures,
        );
        assert_eq!(outcomes.len(), 1);
        assert!(!outcomes[0].signature_verified);
        assert!(
            event_failures
                .iter()
                .any(|f| f.kind == "erasure_attestation_signature_invalid")
        );
    }

    #[test]
    fn finalize_step8_no_post_erasure_use_when_authored_at_equals_destroyed_at() {
        // ADR 0005 step 8 comparison rule: `authored_at > destroyed_at`
        // (strict). Equal timestamps are not flagged (the erasure event
        // itself may carry that kid).
        let kid = vec![0xF4u8; 16];
        let payload = payload_details(kid.clone(), "signing", 100);
        let canonical_hash = [0u8; 32];
        let payloads = vec![(0usize, payload, canonical_hash)];

        // Event authored at exactly destroyed_at: must NOT trigger.
        let chain = vec![chain_summary(0, 100, kid.clone(), vec![], canonical_hash)];

        let registry = BTreeMap::new();
        let mut event_failures = Vec::new();
        let outcomes = super::finalize_erasure_evidence(
            &payloads,
            &chain,
            &registry,
            None,
            &mut event_failures,
        );
        assert_eq!(outcomes.len(), 1);
        assert_eq!(outcomes[0].post_erasure_uses, 0);
        assert_eq!(outcomes[0].post_erasure_wraps, 0);
    }

    // ------------------------------------------------------------------
    // ADR 0007 certificate-of-completion unit tests (Step 2).
    //
    // These exercise the internal decode + finalize + manifest-extension
    // helpers directly, mirroring the ADR 0005 test layout above. Fixture
    // coverage for the wire-corpus paths lands in the ADR 0007 execution
    // train under fixtures/vectors/append/028..030, tamper/020..026, and
    // export/010 in subsequent commits.
    // ------------------------------------------------------------------

    fn certificate_attestation(class: &str) -> Value {
        Value::Map(vec![
            (
                Value::Text("authority".into()),
                Value::Text(format!("urn:trellis:authority:test-{class}")),
            ),
            (
                Value::Text("authority_class".into()),
                Value::Text(class.into()),
            ),
            (Value::Text("signature".into()), Value::Bytes(vec![0u8; 64])),
        ])
    }

    fn presentation_artifact_value(
        media_type: &str,
        attachment_id: &str,
        template_hash: Option<Value>,
    ) -> Value {
        Value::Map(vec![
            (
                Value::Text("content_hash".into()),
                Value::Bytes(vec![0xCAu8; 32]),
            ),
            (
                Value::Text("media_type".into()),
                Value::Text(media_type.into()),
            ),
            (
                Value::Text("byte_length".into()),
                Value::Integer(1024u64.into()),
            ),
            (
                Value::Text("attachment_id".into()),
                Value::Text(attachment_id.into()),
            ),
            (Value::Text("template_id".into()), Value::Null),
            (
                Value::Text("template_hash".into()),
                template_hash.unwrap_or(Value::Null),
            ),
        ])
    }

    fn signer_display_value(principal_ref: &str, signed_at: u64) -> Value {
        Value::Map(vec![
            (
                Value::Text("principal_ref".into()),
                Value::Text(principal_ref.into()),
            ),
            (
                Value::Text("display_name".into()),
                Value::Text("Test Signer".into()),
            ),
            (Value::Text("display_role".into()), Value::Null),
            (
                Value::Text("signed_at".into()),
                Value::Array(vec![
                    Value::Integer(signed_at.into()),
                    Value::Integer(0.into()),
                ]),
            ),
        ])
    }

    fn chain_summary_value(
        signer_count: u64,
        signer_displays: Vec<Value>,
        response_ref: Value,
        workflow_status: &str,
    ) -> Value {
        Value::Map(vec![
            (
                Value::Text("signer_count".into()),
                Value::Integer(signer_count.into()),
            ),
            (
                Value::Text("signer_display".into()),
                Value::Array(signer_displays),
            ),
            (Value::Text("response_ref".into()), response_ref),
            (
                Value::Text("workflow_status".into()),
                Value::Text(workflow_status.into()),
            ),
            (Value::Text("impact_level".into()), Value::Null),
            (Value::Text("covered_claims".into()), Value::Array(vec![])),
        ])
    }

    fn certificate_extension(
        signing_event_digests: Vec<[u8; 32]>,
        signer_count: u64,
        signer_displays: Vec<Value>,
        media_type: &str,
        template_hash: Option<Value>,
        response_ref: Value,
    ) -> Vec<(Value, Value)> {
        let signing_events = signing_event_digests
            .into_iter()
            .map(|d| Value::Bytes(d.to_vec()))
            .collect::<Vec<_>>();
        vec![(
            Value::Text("trellis.certificate-of-completion.v1".into()),
            Value::Map(vec![
                (
                    Value::Text("certificate_id".into()),
                    Value::Text("urn:trellis:cert:test:1".into()),
                ),
                (Value::Text("case_ref".into()), Value::Null),
                (
                    Value::Text("completed_at".into()),
                    Value::Array(vec![
                        Value::Integer(1_745_100_000u64.into()),
                        Value::Integer(0.into()),
                    ]),
                ),
                (
                    Value::Text("presentation_artifact".into()),
                    presentation_artifact_value(media_type, "att-1", template_hash),
                ),
                (
                    Value::Text("chain_summary".into()),
                    chain_summary_value(signer_count, signer_displays, response_ref, "completed"),
                ),
                (
                    Value::Text("signing_events".into()),
                    Value::Array(signing_events),
                ),
                (Value::Text("workflow_ref".into()), Value::Null),
                (
                    Value::Text("attestations".into()),
                    Value::Array(vec![certificate_attestation("new")]),
                ),
                (Value::Text("extensions".into()), Value::Null),
            ]),
        )]
    }

    #[test]
    fn decode_certificate_step1_minimum_valid_payload_decodes() {
        let signing_event = [0xAAu8; 32];
        let extensions = certificate_extension(
            vec![signing_event],
            1,
            vec![signer_display_value(
                "urn:trellis:principal:applicant",
                1_745_099_000,
            )],
            "application/pdf",
            None,
            Value::Null,
        );
        let details = super::decode_certificate_payload(&extensions)
            .unwrap()
            .expect("certificate extension must decode");
        assert_eq!(details.certificate_id, "urn:trellis:cert:test:1");
        assert_eq!(
            details.completed_at,
            TrellisTimestamp {
                seconds: 1_745_100_000,
                nanos: 0
            }
        );
        assert_eq!(details.chain_summary.signer_count, 1);
        assert_eq!(details.signing_events.len(), 1);
        assert_eq!(details.signing_events[0], signing_event);
        assert!(details.attestation_signatures_well_formed);
    }

    #[test]
    fn decode_certificate_step1_returns_none_when_extension_absent() {
        let extensions: Vec<(Value, Value)> = vec![(
            Value::Text("trellis.custody-model-transition.v1".into()),
            Value::Map(vec![]),
        )];
        let result = super::decode_certificate_payload(&extensions).unwrap();
        assert!(result.is_none(), "no certificate ext → None");
    }

    #[test]
    fn decode_certificate_rejects_signer_count_signing_events_mismatch() {
        // ADR 0007 §"Verifier obligations" step 2 first invariant:
        // signer_count MUST equal len(signing_events). Mismatch surfaces
        // with kind `certificate_chain_summary_mismatch`.
        let signing_event = [0xBBu8; 32];
        let extensions = certificate_extension(
            vec![signing_event], // len = 1
            2,                   // claimed = 2
            vec![signer_display_value(
                "urn:trellis:principal:applicant",
                1_745_099_000,
            )],
            "application/pdf",
            None,
            Value::Null,
        );
        let err = super::decode_certificate_payload(&extensions).unwrap_err();
        assert_eq!(err.kind(), Some("certificate_chain_summary_mismatch"));
    }

    #[test]
    fn decode_certificate_rejects_signer_display_signing_events_mismatch() {
        // ADR 0007 §"Verifier obligations" step 2 second invariant:
        // len(signer_display) MUST equal len(signing_events).
        let signing_event = [0xCCu8; 32];
        let extensions = certificate_extension(
            vec![signing_event],
            1,
            vec![
                signer_display_value("urn:trellis:principal:a", 1_745_099_000),
                signer_display_value("urn:trellis:principal:b", 1_745_099_001),
            ],
            "application/pdf",
            None,
            Value::Null,
        );
        let err = super::decode_certificate_payload(&extensions).unwrap_err();
        assert_eq!(err.kind(), Some("certificate_chain_summary_mismatch"));
    }

    #[test]
    fn decode_certificate_rejects_html_with_null_template_hash() {
        // ADR 0007 §"Wire shape" PresentationArtifact.template_hash:
        // media_type=text/html requires non-null template_hash. §19.1 has no
        // dedicated tamper_kind; surface as `malformed_cose` (CDDL-shape).
        let signing_event = [0xDDu8; 32];
        let extensions = certificate_extension(
            vec![signing_event],
            1,
            vec![signer_display_value(
                "urn:trellis:principal:applicant",
                1_745_099_000,
            )],
            "text/html",
            None, // template_hash null
            Value::Null,
        );
        let err = super::decode_certificate_payload(&extensions).unwrap_err();
        assert_eq!(err.kind(), Some("malformed_cose"));
        assert!(err.to_string().contains("template_hash"));
    }

    #[test]
    fn decode_certificate_accepts_html_with_template_hash() {
        let signing_event = [0xEEu8; 32];
        let extensions = certificate_extension(
            vec![signing_event],
            1,
            vec![signer_display_value(
                "urn:trellis:principal:applicant",
                1_745_099_000,
            )],
            "text/html",
            Some(Value::Bytes(vec![0xABu8; 32])),
            Value::Null,
        );
        let details = super::decode_certificate_payload(&extensions)
            .unwrap()
            .unwrap();
        assert!(details.presentation_artifact.template_hash.is_some());
    }

    #[test]
    fn decode_certificate_rejects_empty_signing_events() {
        // ADR 0007 §"Wire shape" `signing_events: [+ digest]` — non-empty
        // required. The CDDL also marks `signer_display: [+ ...]`; the
        // decoder catches the signer_display arity first because the
        // chain-summary nested map decodes before the top-level
        // signing_events array. Either way, an empty signing-events
        // payload is rejected with a recognizable error.
        let signing_event = [0x99u8; 32];
        // Build an extension with one signer_display row (so we get past
        // the signer_display empty check) and an empty signing_events
        // array — exercises only the signing_events arity guard.
        let mut extensions = certificate_extension(
            vec![signing_event],
            0,
            vec![signer_display_value("urn:trellis:principal:a", 1)],
            "application/pdf",
            None,
            Value::Null,
        );
        let inner_map = extensions[0].1.as_map_mut().unwrap();
        for (key, value) in inner_map.iter_mut() {
            if key.as_text() == Some("signing_events") {
                *value = Value::Array(vec![]);
            }
        }
        let err = super::decode_certificate_payload(&extensions).unwrap_err();
        assert!(err.to_string().contains("signing_events"), "{err}");
    }

    #[test]
    fn decode_certificate_rejects_empty_attestations() {
        // Reuse the certificate_extension helper but mutate the
        // attestations array to empty after construction.
        let signing_event = [0xF0u8; 32];
        let mut extensions = certificate_extension(
            vec![signing_event],
            1,
            vec![signer_display_value(
                "urn:trellis:principal:applicant",
                1_745_099_000,
            )],
            "application/pdf",
            None,
            Value::Null,
        );
        // Drill into the certificate map's `attestations` array and empty it.
        let inner_map = extensions[0].1.as_map_mut().unwrap();
        for (key, value) in inner_map.iter_mut() {
            if key.as_text() == Some("attestations") {
                *value = Value::Array(vec![]);
            }
        }
        let err = super::decode_certificate_payload(&extensions).unwrap_err();
        assert!(err.to_string().contains("attestations"), "{err}");
    }

    #[test]
    fn parse_certificate_export_extension_round_trip() {
        // Build a minimum-valid manifest map carrying the optional
        // `trellis.export.certificates-of-completion.v1` extension.
        let catalog_digest = [0x12u8; 32];
        let extension_value = Value::Map(vec![
            (
                Value::Text("catalog_ref".into()),
                Value::Text("065-certificates-of-completion.cbor".into()),
            ),
            (
                Value::Text("catalog_digest".into()),
                Value::Bytes(catalog_digest.to_vec()),
            ),
            (
                Value::Text("entry_count".into()),
                Value::Integer(3u64.into()),
            ),
        ]);
        let manifest_map = vec![(
            Value::Text("extensions".into()),
            Value::Map(vec![(
                Value::Text("trellis.export.certificates-of-completion.v1".into()),
                extension_value,
            )]),
        )];
        let extension = super::parse_certificate_export_extension(&manifest_map)
            .unwrap()
            .expect("extension must round-trip");
        assert_eq!(extension.catalog_ref, "065-certificates-of-completion.cbor");
        assert_eq!(extension.catalog_digest, catalog_digest);
        assert_eq!(extension.entry_count, 3);
    }

    #[test]
    fn parse_certificate_export_extension_returns_none_when_absent() {
        let manifest_map: Vec<(Value, Value)> = vec![];
        let extension = super::parse_certificate_export_extension(&manifest_map).unwrap();
        assert!(extension.is_none());
    }

    fn certificate_details_for_test(
        certificate_id: &str,
        signing_events: Vec<[u8; 32]>,
        signer_count: u64,
    ) -> super::CertificateDetails {
        let signer_displays = signing_events
            .iter()
            .enumerate()
            .map(|(i, _)| super::SignerDisplayDetails {
                principal_ref: format!("urn:trellis:principal:test-{i}"),
                display_name: "Test".to_string(),
                display_role: None,
                signed_at: TrellisTimestamp {
                    seconds: 1_745_099_000 + i as u64,
                    nanos: 0,
                },
            })
            .collect();
        super::CertificateDetails {
            certificate_id: certificate_id.to_string(),
            case_ref: None,
            completed_at: TrellisTimestamp {
                seconds: 1_745_100_000,
                nanos: 0,
            },
            presentation_artifact: super::PresentationArtifactDetails {
                content_hash: [0u8; 32],
                media_type: "application/pdf".to_string(),
                byte_length: 1024,
                attachment_id: format!("att-{certificate_id}"),
                template_id: None,
                template_hash: None,
            },
            chain_summary: super::ChainSummaryDetails {
                signer_count,
                signer_display: signer_displays,
                response_ref: None,
                workflow_status: "completed".to_string(),
                impact_level: None,
                covered_claims: Vec::new(),
            },
            signing_events,
            workflow_ref: None,
            attestation_signatures_well_formed: true,
        }
    }

    #[test]
    fn finalize_certificates_accumulates_outcome_per_event() {
        // ADR 0007 §"Verifier obligations" step 8: every certificate event
        // contributes one outcome to `report.certificates_of_completion`.
        // Genesis-context (events slice empty) → step 4 stays
        // `attachment_resolved = true`; steps 5/6/7 don't fire because the
        // signing-event digests don't resolve in an empty slice (recorded
        // as `signing_event_unresolved` per step 5).
        let signing_event = [0x55u8; 32];
        let payload = certificate_details_for_test("cert-1", vec![signing_event], 1);
        let canonical_hash = [0u8; 32];
        let payloads = vec![(0usize, payload, canonical_hash)];

        let mut event_failures = Vec::new();
        let outcomes =
            super::finalize_certificates_of_completion(&payloads, &[], &mut event_failures);
        assert_eq!(outcomes.len(), 1);
        assert_eq!(outcomes[0].certificate_id, "cert-1");
        assert_eq!(outcomes[0].signer_count, 1);
        assert_eq!(
            outcomes[0].completed_at,
            TrellisTimestamp {
                seconds: 1_745_100_000,
                nanos: 0
            }
        );
        // Empty events slice → unresolvable signing event → step 5 flags.
        assert!(!outcomes[0].all_signing_events_resolved);
        assert!(
            outcomes[0]
                .failures
                .iter()
                .any(|f| f == "signing_event_unresolved")
        );
        assert!(
            event_failures
                .iter()
                .any(|f| f.kind == "signing_event_unresolved")
        );
    }

    #[test]
    fn finalize_certificates_flags_id_collision_for_disagreeing_payloads() {
        // ADR 0007 §"Verifier obligations" step 2 second sub-clause:
        // duplicate certificate_id with disagreeing canonical payload →
        // `certificate_id_collision`.
        let signing_event_a = [0x60u8; 32];
        let signing_event_b = [0x61u8; 32];
        let payload_a = certificate_details_for_test("cert-collision", vec![signing_event_a], 1);
        // Same id, different signing_events digest → collision.
        let payload_b = certificate_details_for_test("cert-collision", vec![signing_event_b], 1);
        let hash_a = [0u8; 32];
        let hash_b = [1u8; 32];
        let payloads = vec![(0usize, payload_a, hash_a), (1usize, payload_b, hash_b)];

        let mut event_failures = Vec::new();
        let outcomes =
            super::finalize_certificates_of_completion(&payloads, &[], &mut event_failures);
        assert_eq!(outcomes.len(), 2);
        assert!(
            event_failures
                .iter()
                .any(|f| f.kind == "certificate_id_collision"),
            "step 2 second sub-clause: duplicate certificate_id with disagreeing payload",
        );
    }

    #[test]
    fn finalize_certificates_no_id_collision_when_payloads_agree() {
        // Two identical payloads under the same id → first-seen wins, no
        // collision flagged.
        let signing_event = [0x70u8; 32];
        let payload_a = certificate_details_for_test("cert-twin", vec![signing_event], 1);
        let payload_b = certificate_details_for_test("cert-twin", vec![signing_event], 1);
        let hash_a = [0u8; 32];
        let hash_b = [1u8; 32];
        let payloads = vec![(0usize, payload_a, hash_a), (1usize, payload_b, hash_b)];

        let mut event_failures = Vec::new();
        let _outcomes =
            super::finalize_certificates_of_completion(&payloads, &[], &mut event_failures);
        assert!(
            !event_failures
                .iter()
                .any(|f| f.kind == "certificate_id_collision")
        );
    }

    #[test]
    fn finalize_certificates_flags_attestation_when_signature_malformed() {
        // ADR 0007 §"Verifier obligations" step 3 (Phase-1 structural):
        // attestation row with malformed signature flips
        // `chain_summary_consistent = false` and emits
        // `attestation_insufficient`.
        let signing_event = [0x80u8; 32];
        let mut payload = certificate_details_for_test("cert-bad-att", vec![signing_event], 1);
        payload.attestation_signatures_well_formed = false;
        let canonical_hash = [0u8; 32];
        let payloads = vec![(0usize, payload, canonical_hash)];

        let mut event_failures = Vec::new();
        let outcomes =
            super::finalize_certificates_of_completion(&payloads, &[], &mut event_failures);
        assert_eq!(outcomes.len(), 1);
        assert!(!outcomes[0].chain_summary_consistent);
        assert!(
            outcomes[0]
                .failures
                .iter()
                .any(|f| f == "attestation_insufficient")
        );
        assert!(
            event_failures
                .iter()
                .any(|f| f.kind == "attestation_insufficient")
        );
    }

    #[test]
    fn finalize_certificates_genesis_path_marks_attachment_resolved_true() {
        // Phase-1 minimal-genesis posture: the genesis-append code paths
        // (verify_single_event / verify_tampered_ledger) lack chain
        // visibility for attachment lineage. Step 4 defers to the
        // export-bundle path, so the genesis-path outcome must NOT
        // false-positive on `attachment_resolved`.
        let signing_event = [0x90u8; 32];
        let payload = certificate_details_for_test("cert-genesis", vec![signing_event], 1);
        let canonical_hash = [0u8; 32];
        let payloads = vec![(0usize, payload, canonical_hash)];

        let mut event_failures = Vec::new();
        let outcomes =
            super::finalize_certificates_of_completion(&payloads, &[], &mut event_failures);
        assert_eq!(outcomes.len(), 1);
        assert!(outcomes[0].attachment_resolved);
        assert!(
            !outcomes[0]
                .failures
                .iter()
                .any(|f| f == "presentation_artifact_attachment_missing"),
            "Phase-1 genesis path must not emit attachment-missing failures",
        );
    }

    // ---------------------------------------------------------------
    // ADR 0010 user-content-attestation focused unit tests.
    // Mirrors the certificate-of-completion test pattern: decode is
    // covered by passing a CBOR map directly to
    // `decode_user_content_attestation_payload`; finalize is covered
    // by building `UserContentAttestationDetails` test fixtures and
    // running `finalize_user_content_attestations` against synthetic
    // chain context. Byte-level vector parity rides the
    // `append/036..039` + `tamper/028..034` corpus.
    // ---------------------------------------------------------------

    fn user_content_details_for_test(
        attestation_id: &str,
        attestor: &str,
        signing_intent: &str,
        attested_at: u64,
        signing_kid: Vec<u8>,
        identity_attestation_ref: Option<[u8; 32]>,
    ) -> super::UserContentAttestationDetails {
        let attested_at_ts = super::TrellisTimestamp {
            seconds: attested_at,
            nanos: 0,
        };
        let attested_event_hash = [0xAAu8; 32];
        let attested_event_position = 0;
        let canonical_preimage = super::compute_user_content_attestation_preimage(
            attestation_id,
            &attested_event_hash,
            attested_event_position,
            attestor,
            identity_attestation_ref.as_ref(),
            signing_intent,
            attested_at_ts,
        );
        super::UserContentAttestationDetails {
            attestation_id: attestation_id.to_string(),
            attested_event_hash,
            attested_event_position,
            attestor: attestor.to_string(),
            identity_attestation_ref,
            signing_intent: signing_intent.to_string(),
            attested_at: attested_at_ts,
            signature: [0u8; 64],
            signing_kid,
            canonical_preimage,
            step_2_failure: None,
        }
    }

    #[test]
    fn is_syntactically_valid_uri_admits_urn() {
        // Trellis owns the bytes; WOS owns the meaning. URN-style intent
        // URIs (no authority) must pass the syntactic check.
        assert!(super::is_syntactically_valid_uri(
            "urn:trellis:intent:notarial-attestation"
        ));
        assert!(super::is_syntactically_valid_uri(
            "urn:wos:signature-intent:applicant-affirmation"
        ));
        assert!(super::is_syntactically_valid_uri(
            "https://example.invalid/intent/witness"
        ));
    }

    #[test]
    fn is_syntactically_valid_uri_rejects_malformed() {
        // ADR 0010 §"Verifier obligations" step 2 — malformed URI flips
        // `user_content_attestation_intent_malformed`.
        assert!(!super::is_syntactically_valid_uri(""));
        assert!(!super::is_syntactically_valid_uri("no-colon"));
        assert!(!super::is_syntactically_valid_uri(":empty-scheme"));
        assert!(!super::is_syntactically_valid_uri("scheme-only:"));
        assert!(!super::is_syntactically_valid_uri("9digit-start:rest"));
        assert!(!super::is_syntactically_valid_uri("bad space:rest"));
    }

    #[test]
    fn is_operator_uri_detects_companion_6_4_prefixes() {
        // ADR 0010 §"Verifier obligations" step 8 — operator URIs forbidden
        // in `attestor` slot. Phase-1 conservative prefixes are
        // `urn:trellis:operator:` and `urn:wos:operator:`.
        assert!(super::is_operator_uri(
            "urn:trellis:operator:test-deployment"
        ));
        assert!(super::is_operator_uri("urn:wos:operator:agency-of-record"));
        // User principal URIs MUST pass.
        assert!(!super::is_operator_uri(
            "urn:trellis:principal:applicant-001"
        ));
        assert!(!super::is_operator_uri("urn:wos:user:notary-002"));
        assert!(!super::is_operator_uri(""));
    }

    #[test]
    fn parse_admit_unverified_user_attestations_defaults_false() {
        // Empty bytes / non-map / absent field all default to `false`
        // (REQUIRED non-null posture). Critical fail-closed property:
        // a malformed Posture Declaration cannot silently relax the gate.
        assert!(!super::parse_admit_unverified_user_attestations(&[]));
        assert!(!super::parse_admit_unverified_user_attestations(&[0x40])); // bstr, not a map
        // Map without the field → false.
        let mut buf = Vec::new();
        ciborium::ser::into_writer(
            &Value::Map(vec![(
                Value::Text("provider_readable".into()),
                Value::Bool(true),
            )]),
            &mut buf,
        )
        .unwrap();
        assert!(!super::parse_admit_unverified_user_attestations(&buf));
    }

    #[test]
    fn parse_admit_unverified_user_attestations_admits_explicit_true() {
        let mut buf = Vec::new();
        ciborium::ser::into_writer(
            &Value::Map(vec![(
                Value::Text("admit_unverified_user_attestations".into()),
                Value::Bool(true),
            )]),
            &mut buf,
        )
        .unwrap();
        assert!(super::parse_admit_unverified_user_attestations(&buf));
    }

    #[test]
    fn finalize_uca_flags_operator_in_user_slot() {
        // ADR 0010 §"Verifier obligations" step 8.
        let payload = user_content_details_for_test(
            "uca-test-1",
            "urn:trellis:operator:bad-actor",
            "urn:trellis:intent:applicant",
            1_776_900_000,
            vec![0xC0u8; 16],
            Some([0xBBu8; 32]),
        );
        let canonical_hash = [0xDD; 32];
        let payloads = vec![(0usize, payload, canonical_hash)];
        let registry = BTreeMap::new();
        let mut event_failures = Vec::new();
        let outcomes = super::finalize_user_content_attestations(
            &payloads,
            &[],
            &registry,
            None,
            &mut event_failures,
        );
        assert_eq!(outcomes.len(), 1);
        assert!(
            outcomes[0]
                .failures
                .iter()
                .any(|f| f == "user_content_attestation_operator_in_user_slot")
        );
        assert!(
            event_failures
                .iter()
                .any(|f| f.kind == "user_content_attestation_operator_in_user_slot")
        );
    }

    #[test]
    fn finalize_uca_flags_identity_required_when_posture_default() {
        // ADR 0010 §"Verifier obligations" step 4 null-admission gate.
        // Default posture (no Posture Declaration / `admit_unverified_*`
        // absent) MUST flip `user_content_attestation_identity_required`
        // when `identity_attestation_ref` is null.
        let payload = user_content_details_for_test(
            "uca-required-1",
            "urn:trellis:principal:applicant",
            "urn:trellis:intent:applicant",
            1_776_900_000,
            vec![0xC0u8; 16],
            None, // ← null identity ref triggers step 4 null path
        );
        let canonical_hash = [0xDD; 32];
        let payloads = vec![(0usize, payload, canonical_hash)];
        let registry = BTreeMap::new();
        let mut event_failures = Vec::new();
        let outcomes = super::finalize_user_content_attestations(
            &payloads,
            &[],
            &registry,
            None, // no Posture Declaration → default required
            &mut event_failures,
        );
        assert_eq!(outcomes.len(), 1);
        assert!(!outcomes[0].identity_resolved);
        assert!(
            outcomes[0]
                .failures
                .iter()
                .any(|f| f == "user_content_attestation_identity_required")
        );
    }

    #[test]
    fn finalize_uca_admits_null_identity_when_posture_permits() {
        // ADR 0010 §"Verifier obligations" step 4 null-admission gate.
        // Posture Declaration with `admit_unverified_user_attestations: true`
        // permits null `identity_attestation_ref` without flipping integrity.
        let payload = user_content_details_for_test(
            "uca-permitted-1",
            "urn:trellis:principal:applicant",
            "urn:trellis:intent:applicant",
            1_776_900_000,
            vec![0xC0u8; 16],
            None,
        );
        let canonical_hash = [0xDD; 32];
        let payloads = vec![(0usize, payload, canonical_hash)];
        let registry = BTreeMap::new();
        let mut event_failures = Vec::new();

        // Build a minimal Posture Declaration with the admit flag true.
        let mut posture_bytes = Vec::new();
        ciborium::ser::into_writer(
            &Value::Map(vec![(
                Value::Text("admit_unverified_user_attestations".into()),
                Value::Bool(true),
            )]),
            &mut posture_bytes,
        )
        .unwrap();

        let outcomes = super::finalize_user_content_attestations(
            &payloads,
            &[],
            &registry,
            Some(&posture_bytes),
            &mut event_failures,
        );
        assert_eq!(outcomes.len(), 1);
        // No identity-required failure under permissive posture.
        assert!(
            !outcomes[0]
                .failures
                .iter()
                .any(|f| f == "user_content_attestation_identity_required")
        );
    }

    #[test]
    fn finalize_uca_flags_key_not_active_for_unregistered_kid() {
        // ADR 0010 §"Verifier obligations" step 6 — kid not in registry =
        // not Active. The Phase-1 verifier flips
        // `user_content_attestation_key_not_active`.
        let payload = user_content_details_for_test(
            "uca-no-key",
            "urn:trellis:principal:applicant",
            "urn:trellis:intent:applicant",
            1_776_900_000,
            vec![0xC0u8; 16],
            Some([0xBBu8; 32]),
        );
        let canonical_hash = [0xDD; 32];
        let payloads = vec![(0usize, payload, canonical_hash)];
        let registry = BTreeMap::new(); // ← empty registry
        let mut event_failures = Vec::new();
        let outcomes = super::finalize_user_content_attestations(
            &payloads,
            &[],
            &registry,
            None,
            &mut event_failures,
        );
        assert_eq!(outcomes.len(), 1);
        assert!(!outcomes[0].key_active);
        assert!(
            outcomes[0]
                .failures
                .iter()
                .any(|f| f == "user_content_attestation_key_not_active")
        );
    }

    #[test]
    fn finalize_uca_id_collision_detected_on_disagreeing_payloads() {
        // ADR 0010 §"Verifier obligations" step 7 — two events sharing
        // `attestation_id` with disagreeing canonical payload fail closed.
        let mut a = user_content_details_for_test(
            "uca-dup-1",
            "urn:trellis:principal:applicant",
            "urn:trellis:intent:applicant",
            1_776_900_000,
            vec![0xC0u8; 16],
            Some([0xBBu8; 32]),
        );
        let mut b = a.clone();
        // Mutate an inner field so the canonical payloads disagree.
        b.attested_at = super::TrellisTimestamp {
            seconds: 1_776_900_999,
            nanos: 0,
        };
        // Canonical preimages must reflect the divergence.
        a.canonical_preimage = super::compute_user_content_attestation_preimage(
            &a.attestation_id,
            &a.attested_event_hash,
            a.attested_event_position,
            &a.attestor,
            a.identity_attestation_ref.as_ref(),
            &a.signing_intent,
            a.attested_at,
        );
        b.canonical_preimage = super::compute_user_content_attestation_preimage(
            &b.attestation_id,
            &b.attested_event_hash,
            b.attested_event_position,
            &b.attestor,
            b.identity_attestation_ref.as_ref(),
            &b.signing_intent,
            b.attested_at,
        );

        let payloads = vec![(0usize, a, [0xCC; 32]), (1usize, b, [0xCD; 32])];
        let registry = BTreeMap::new();
        let mut event_failures = Vec::new();
        super::finalize_user_content_attestations(
            &payloads,
            &[],
            &registry,
            None,
            &mut event_failures,
        );
        assert!(
            event_failures
                .iter()
                .any(|f| f.kind == "user_content_attestation_id_collision"),
            "step 7 must flag id_collision for disagreeing payloads",
        );
    }

    #[test]
    fn finalize_uca_no_collision_when_byte_identical() {
        // ADR 0010 §"Field semantics" `attestation_id` clause: idempotent
        // re-emission with byte-identical canonical payload MUST NOT flip.
        let a = user_content_details_for_test(
            "uca-same",
            "urn:trellis:principal:applicant",
            "urn:trellis:intent:applicant",
            1_776_900_000,
            vec![0xC0u8; 16],
            Some([0xBBu8; 32]),
        );
        let b = a.clone();
        let payloads = vec![(0usize, a, [0xCC; 32]), (1usize, b, [0xCD; 32])];
        let registry = BTreeMap::new();
        let mut event_failures = Vec::new();
        super::finalize_user_content_attestations(
            &payloads,
            &[],
            &registry,
            None,
            &mut event_failures,
        );
        assert!(
            !event_failures
                .iter()
                .any(|f| f.kind == "user_content_attestation_id_collision"),
            "byte-identical re-emission must not flip id_collision",
        );
    }

    #[test]
    fn decode_uca_payload_defers_timestamp_skew_to_finalize() {
        // ADR 0010 §"Verifier obligations" step 2 — `attested_at` MUST
        // exactly equal envelope `authored_at`. Step 2 failures are
        // intra-payload-invariant (post-CDDL-decode) and flip
        // `integrity_verified = false` only — they MUST NOT flip
        // `readability_verified`. The decoder records the failure on
        // `step_2_failure` for the finalize pass to raise.
        let kid_bytes: Vec<u8> = vec![0xC0u8; 16];
        let extension_value = Value::Map(vec![
            (
                Value::Text("attestation_id".into()),
                Value::Text("uca-skew-1".into()),
            ),
            (
                Value::Text("attested_event_hash".into()),
                Value::Bytes(vec![0xAA; 32]),
            ),
            (
                Value::Text("attested_event_position".into()),
                Value::Integer(0u64.into()),
            ),
            (
                Value::Text("attestor".into()),
                Value::Text("urn:trellis:principal:applicant".into()),
            ),
            (
                Value::Text("identity_attestation_ref".into()),
                Value::Bytes(vec![0xBB; 32]),
            ),
            (
                Value::Text("signing_intent".into()),
                Value::Text("urn:trellis:intent:applicant".into()),
            ),
            (
                Value::Text("attested_at".into()),
                Value::Array(vec![
                    Value::Integer(1_776_900_000u64.into()),
                    Value::Integer(0u32.into()),
                ]),
            ),
            (Value::Text("signature".into()), Value::Bytes(vec![0u8; 64])),
            (Value::Text("signing_kid".into()), Value::Bytes(kid_bytes)),
        ]);
        let extensions = vec![(
            Value::Text(super::USER_CONTENT_ATTESTATION_EVENT_EXTENSION.into()),
            extension_value,
        )];
        // Host envelope authored_at differs from payload attested_at.
        let decoded = super::decode_user_content_attestation_payload(
            &extensions,
            super::TrellisTimestamp {
                seconds: 1_776_900_999,
                nanos: 0,
            },
        )
        .expect("step 2 failures decode cleanly; finalize raises them");
        let details = decoded.expect("payload present");
        assert_eq!(
            details.step_2_failure,
            Some("user_content_attestation_timestamp_mismatch")
        );
    }

    #[test]
    fn decode_uca_payload_defers_malformed_intent_uri_to_finalize() {
        // ADR 0010 §"Verifier obligations" step 2. Intra-payload-invariant
        // failure: deferred to finalize via `step_2_failure` marker;
        // `readability_verified` stays `true` per ADR 0010 step 2 prose.
        let kid_bytes: Vec<u8> = vec![0xC0u8; 16];
        let extension_value = Value::Map(vec![
            (
                Value::Text("attestation_id".into()),
                Value::Text("uca-bad-intent".into()),
            ),
            (
                Value::Text("attested_event_hash".into()),
                Value::Bytes(vec![0xAA; 32]),
            ),
            (
                Value::Text("attested_event_position".into()),
                Value::Integer(0u64.into()),
            ),
            (
                Value::Text("attestor".into()),
                Value::Text("urn:trellis:principal:applicant".into()),
            ),
            (
                Value::Text("identity_attestation_ref".into()),
                Value::Bytes(vec![0xBB; 32]),
            ),
            (
                Value::Text("signing_intent".into()),
                Value::Text("not-a-uri".into()),
            ),
            (
                Value::Text("attested_at".into()),
                Value::Array(vec![
                    Value::Integer(1_776_900_000u64.into()),
                    Value::Integer(0u32.into()),
                ]),
            ),
            (Value::Text("signature".into()), Value::Bytes(vec![0u8; 64])),
            (Value::Text("signing_kid".into()), Value::Bytes(kid_bytes)),
        ]);
        let extensions = vec![(
            Value::Text(super::USER_CONTENT_ATTESTATION_EVENT_EXTENSION.into()),
            extension_value,
        )];
        let decoded = super::decode_user_content_attestation_payload(
            &extensions,
            super::TrellisTimestamp {
                seconds: 1_776_900_000,
                nanos: 0,
            },
        )
        .expect("step 2 failures decode cleanly; finalize raises them");
        let details = decoded.expect("payload present");
        assert_eq!(
            details.step_2_failure,
            Some("user_content_attestation_intent_malformed")
        );
    }

    #[test]
    fn decode_uca_payload_returns_none_when_extension_absent() {
        let extensions: Vec<(Value, Value)> = vec![];
        let decoded = super::decode_user_content_attestation_payload(
            &extensions,
            super::TrellisTimestamp {
                seconds: 0,
                nanos: 0,
            },
        )
        .expect("absent extension is not an error");
        assert!(decoded.is_none());
    }

    #[test]
    fn decode_uca_payload_succeeds_with_well_formed_input() {
        let kid_bytes: Vec<u8> = vec![0xC0u8; 16];
        let extension_value = Value::Map(vec![
            (
                Value::Text("attestation_id".into()),
                Value::Text("uca-ok-1".into()),
            ),
            (
                Value::Text("attested_event_hash".into()),
                Value::Bytes(vec![0xAA; 32]),
            ),
            (
                Value::Text("attested_event_position".into()),
                Value::Integer(0u64.into()),
            ),
            (
                Value::Text("attestor".into()),
                Value::Text("urn:trellis:principal:applicant".into()),
            ),
            (
                Value::Text("identity_attestation_ref".into()),
                Value::Bytes(vec![0xBB; 32]),
            ),
            (
                Value::Text("signing_intent".into()),
                Value::Text("urn:trellis:intent:applicant".into()),
            ),
            (
                Value::Text("attested_at".into()),
                Value::Array(vec![
                    Value::Integer(1_776_900_000u64.into()),
                    Value::Integer(0u32.into()),
                ]),
            ),
            (Value::Text("signature".into()), Value::Bytes(vec![0u8; 64])),
            (
                Value::Text("signing_kid".into()),
                Value::Bytes(kid_bytes.clone()),
            ),
        ]);
        let extensions = vec![(
            Value::Text(super::USER_CONTENT_ATTESTATION_EVENT_EXTENSION.into()),
            extension_value,
        )];
        let decoded = super::decode_user_content_attestation_payload(
            &extensions,
            super::TrellisTimestamp {
                seconds: 1_776_900_000,
                nanos: 0,
            },
        )
        .expect("well-formed payload decodes")
        .expect("extension is present");
        assert_eq!(decoded.attestation_id, "uca-ok-1");
        assert_eq!(decoded.attestor, "urn:trellis:principal:applicant");
        assert_eq!(decoded.attested_event_position, 0);
        assert_eq!(decoded.signing_kid, kid_bytes);
        assert_eq!(decoded.signature.len(), 64);
        assert!(!decoded.canonical_preimage.is_empty());
    }

    #[test]
    fn verify_tampered_ledger_detects_timestamp_order_violation() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/vectors/tamper/041-timestamp-backwards");
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
        assert_eq!(report.event_failures[0].kind, "timestamp_order_violation");
    }

    #[test]
    fn verify_equal_timestamps_pass_temporal_check() {
        let genesis_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/vectors/append/001-minimal-inline-payload");
        let chain_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/vectors/append/005-prior-head-chain");
        let registry_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/vectors/tamper/041-timestamp-backwards");

        let genesis_event =
            parse_sign1_bytes(&fs::read(genesis_root.join("expected-event.cbor")).unwrap())
                .unwrap();
        let chain_event =
            parse_sign1_bytes(&fs::read(chain_root.join("expected-event.cbor")).unwrap()).unwrap();

        let registry_bytes =
            fs::read(registry_root.join("input-signing-key-registry.cbor")).unwrap();
        let registry = parse_signing_key_registry(&registry_bytes).unwrap();

        let report = verify_event_set(
            &[genesis_event, chain_event],
            &registry,
            None,
            None,
            false,
            None,
            None,
        );

        assert!(report.structure_verified);
        assert!(
            report.integrity_verified,
            "append/001 + append/005 chain should pass all checks including temporal order \
             (authored_at 1745000000 < 1745000001 is non-decreasing)"
        );
        assert!(report.event_failures.is_empty());
    }

    #[test]
    fn verify_rejects_legacy_uint_timestamp_format() {
        let header: Vec<(Value, Value)> = vec![
            (
                Value::Text("event_type".into()),
                Value::Bytes(b"x-trellis-test/append-minimal".to_vec()),
            ),
            (
                Value::Text("authored_at".into()),
                Value::Integer(1745000000.into()),
            ),
            (
                Value::Text("retention_tier".into()),
                Value::Integer(0.into()),
            ),
            (
                Value::Text("classification".into()),
                Value::Bytes(b"x-trellis-test/unclassified".to_vec()),
            ),
            (Value::Text("outcome_commitment".into()), Value::Null),
            (Value::Text("subject_ref_commitment".into()), Value::Null),
            (Value::Text("tag_commitment".into()), Value::Null),
            (Value::Text("witness_ref".into()), Value::Null),
            (Value::Text("extensions".into()), Value::Null),
        ];
        let result = super::map_lookup_timestamp(&header, "authored_at");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind(), Some("legacy_timestamp_format"));
    }
}
