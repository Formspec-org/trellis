"""Unit tests for ADR 0005 cryptographic-erasure-evidence verification.

Mirrors `crates/trellis-verify/src/lib.rs` Stage 2 unit tests step-by-step
so Rust + Python remain byte-equivalent on the verifier-side semantics.
G-5 corpus parity (the same fixtures pass on both runtimes) is asserted by
`trellis_py.conformance`; these tests cover the focused decode + finalize
helpers.
"""

from __future__ import annotations

from typing import Optional

import pytest

from trellis_py.verify import (
    ErasureEvidenceDetails,
    NonSigningKeyEntry,
    SigningKeyEntry,
    TrellisTimestamp,
    VerificationFailure,
    VerifyError,
    _ChainEventSummary,
    _decode_erasure_evidence_details,
    _finalize_erasure_evidence,
    _validate_subject_scope_shape,
)

# ---------------------------------------------------------------------------
# Builders.
# ---------------------------------------------------------------------------


def per_subject_scope() -> dict:
    return {
        "kind": "per-subject",
        "subject_refs": ["urn:trellis:subject:test-1"],
        "ledger_scopes": None,
        "tenant_refs": None,
    }


def deployment_wide_scope() -> dict:
    return {
        "kind": "deployment-wide",
        "subject_refs": None,
        "ledger_scopes": None,
        "tenant_refs": None,
    }


def one_attestation(authority_class: str) -> dict:
    return {
        "authority": f"urn:trellis:authority:test-{authority_class}",
        "authority_class": authority_class,
        "signature": b"\x00" * 64,
    }


def erasure_extensions(
    *,
    kid_destroyed: bytes = b"\xab" * 16,
    key_class: str = "signing",
    destroyed_at: int = 1_745_000_000,
    subject_scope: Optional[dict] = None,
    attestations: Optional[list] = None,
    hsm_receipt: object = None,
    hsm_receipt_kind: object = None,
    cascade_scopes: Optional[list[str]] = None,
) -> dict:
    if subject_scope is None:
        subject_scope = per_subject_scope()
    if attestations is None:
        attestations = [one_attestation("new")]
    if cascade_scopes is None:
        cascade_scopes = ["CS-03"]
    return {
        "trellis.erasure-evidence.v1": {
            "evidence_id": "urn:trellis:erasure:test:1",
            "kid_destroyed": kid_destroyed,
            "key_class": key_class,
            "destroyed_at": [destroyed_at, 0],
            "cascade_scopes": list(cascade_scopes),
            "completion_mode": "complete",
            "destruction_actor": "urn:trellis:principal:test-actor",
            "policy_authority": "urn:trellis:authority:test-policy",
            "reason_code": 1,
            "subject_scope": subject_scope,
            "hsm_receipt": hsm_receipt,
            "hsm_receipt_kind": hsm_receipt_kind,
            "attestations": list(attestations),
            "extensions": None,
        }
    }


def payload_details(
    kid: bytes,
    norm_key_class: str,
    destroyed_at: int,
) -> ErasureEvidenceDetails:
    return ErasureEvidenceDetails(
        evidence_id="urn:trellis:erasure:test:py",
        kid_destroyed=kid,
        norm_key_class=norm_key_class,
        destroyed_at=TrellisTimestamp(seconds=destroyed_at, nanos=0),
        cascade_scopes=["CS-03"],
        completion_mode="complete",
        attestation_signatures_well_formed=True,
        attestation_classes=["new"],
        subject_scope_kind="per-subject",
    )


def chain_summary(
    index: int,
    authored_at: int,
    signing_kid: bytes,
    wrap_recipients: list[bytes],
    canonical_event_hash: bytes,
) -> _ChainEventSummary:
    return _ChainEventSummary(
        event_index=index,
        authored_at=TrellisTimestamp(seconds=authored_at, nanos=0),
        signing_kid=signing_kid,
        wrap_recipients=wrap_recipients,
        canonical_event_hash=canonical_event_hash,
    )


# ---------------------------------------------------------------------------
# Step 3 — _validate_subject_scope_shape.
# ---------------------------------------------------------------------------


def test_validate_subject_scope_per_subject_accepts_subject_refs_only() -> None:
    _validate_subject_scope_shape(per_subject_scope(), "per-subject")  # no raise


def test_validate_subject_scope_per_subject_rejects_with_ledger_scopes() -> None:
    bad = dict(per_subject_scope())
    bad["ledger_scopes"] = [b"x"]
    with pytest.raises(VerifyError, match="subject_scope"):
        _validate_subject_scope_shape(bad, "per-subject")


def test_validate_subject_scope_per_scope_requires_ledger_scopes() -> None:
    scope = {
        "kind": "per-scope",
        "subject_refs": None,
        "ledger_scopes": [b"scope-a"],
        "tenant_refs": None,
    }
    _validate_subject_scope_shape(scope, "per-scope")  # no raise


def test_validate_subject_scope_per_tenant_requires_tenant_refs() -> None:
    scope = {
        "kind": "per-tenant",
        "subject_refs": None,
        "ledger_scopes": None,
        "tenant_refs": ["urn:trellis:tenant:test"],
    }
    _validate_subject_scope_shape(scope, "per-tenant")  # no raise


def test_validate_subject_scope_deployment_wide_rejects_any_ref_field() -> None:
    bad = {
        "kind": "deployment-wide",
        "subject_refs": ["urn:trellis:subject:s"],
        "ledger_scopes": None,
        "tenant_refs": None,
    }
    with pytest.raises(VerifyError, match="subject_scope"):
        _validate_subject_scope_shape(bad, "deployment-wide")


def test_validate_subject_scope_unknown_kind_rejected() -> None:
    bad = {
        "kind": "not-real",
        "subject_refs": None,
        "ledger_scopes": None,
        "tenant_refs": None,
    }
    with pytest.raises(VerifyError, match="not-real"):
        _validate_subject_scope_shape(bad, "not-real")


# ---------------------------------------------------------------------------
# decode_erasure_evidence_details — steps 1 / 2 / 3 / 4 / 6 / 7.
# ---------------------------------------------------------------------------


def test_decode_step1_minimum_valid_payload_decodes() -> None:
    extensions = erasure_extensions(kid_destroyed=b"\xab" * 16)
    details = _decode_erasure_evidence_details(extensions, host_authored_at=TrellisTimestamp(seconds=1_745_000_100, nanos=0))
    assert details is not None
    assert details.evidence_id == "urn:trellis:erasure:test:1"
    assert details.kid_destroyed == b"\xab" * 16
    assert details.norm_key_class == "signing"
    assert details.destroyed_at == TrellisTimestamp(seconds=1_745_000_000, nanos=0)
    assert details.cascade_scopes == ["CS-03"]
    assert details.completion_mode == "complete"
    assert details.attestation_signatures_well_formed is True


def test_decode_step1_returns_none_when_extension_absent() -> None:
    extensions = {"trellis.custody-model-transition.v1": {}}
    assert _decode_erasure_evidence_details(extensions, host_authored_at=TrellisTimestamp(seconds=1_745_000_000, nanos=0)) is None


def test_decode_step2_normalizes_wire_wrap_to_subject() -> None:
    """ADR 0005 step 2 + Core §8.7.6: wire `key_class = "wrap"` MUST
    normalize to `"subject"` before any registry comparison.
    """
    extensions = erasure_extensions(key_class="wrap")
    details = _decode_erasure_evidence_details(extensions, host_authored_at=TrellisTimestamp(seconds=1_745_000_100, nanos=0))
    assert details is not None
    assert details.norm_key_class == "subject"


def test_decode_step3_rejects_per_subject_with_null_subject_refs() -> None:
    bad_scope = {
        "kind": "per-subject",
        "subject_refs": None,
        "ledger_scopes": None,
        "tenant_refs": None,
    }
    extensions = erasure_extensions(subject_scope=bad_scope)
    with pytest.raises(VerifyError, match="subject_scope"):
        _decode_erasure_evidence_details(extensions, host_authored_at=TrellisTimestamp(seconds=1_745_000_100, nanos=0))


def test_decode_step4_rejects_destroyed_at_after_host_authored_at() -> None:
    """ADR 0005 step 4 / Companion OC-144: `destroyed_at` MUST be ≤ host
    authored_at. Violation surfaces as `erasure_destroyed_at_after_host`.
    """
    extensions = erasure_extensions(destroyed_at=1_745_000_500)
    with pytest.raises(VerifyError) as excinfo:
        _decode_erasure_evidence_details(extensions, host_authored_at=TrellisTimestamp(seconds=1_745_000_100, nanos=0))
    assert excinfo.value.kind == "erasure_destroyed_at_after_host"


def test_decode_step6_rejects_hsm_receipt_without_kind() -> None:
    extensions = erasure_extensions(hsm_receipt=b"opaque-hsm-bytes")
    with pytest.raises(VerifyError, match="hsm_receipt"):
        _decode_erasure_evidence_details(extensions, host_authored_at=TrellisTimestamp(seconds=1_745_000_100, nanos=0))


def test_decode_step6_rejects_hsm_receipt_kind_without_receipt() -> None:
    extensions = erasure_extensions(hsm_receipt_kind="opaque-vendor-receipt-v1")
    with pytest.raises(VerifyError, match="hsm_receipt"):
        _decode_erasure_evidence_details(extensions, host_authored_at=TrellisTimestamp(seconds=1_745_000_100, nanos=0))


def test_decode_step6_accepts_both_hsm_fields_present() -> None:
    extensions = erasure_extensions(
        hsm_receipt=b"opaque-hsm-bytes",
        hsm_receipt_kind="opaque-vendor-receipt-v1",
    )
    details = _decode_erasure_evidence_details(extensions, host_authored_at=TrellisTimestamp(seconds=1_745_000_100, nanos=0))
    assert details is not None


def test_decode_step7_marks_short_attestation_signature_malformed() -> None:
    bad_attestation = {
        "authority": "urn:trellis:authority:test-bad",
        "authority_class": "new",
        "signature": b"\x00" * 32,  # wrong length
    }
    extensions = erasure_extensions(attestations=[bad_attestation])
    details = _decode_erasure_evidence_details(extensions, host_authored_at=TrellisTimestamp(seconds=1_745_000_100, nanos=0))
    assert details is not None
    assert details.attestation_signatures_well_formed is False


def test_decode_step1_rejects_empty_cascade_scopes() -> None:
    extensions = erasure_extensions(cascade_scopes=[])
    with pytest.raises(VerifyError, match="cascade_scopes"):
        _decode_erasure_evidence_details(extensions, host_authored_at=TrellisTimestamp(seconds=1_745_000_100, nanos=0))


def test_decode_step1_rejects_empty_attestations() -> None:
    extensions = erasure_extensions(attestations=[])
    with pytest.raises(VerifyError, match="attestations"):
        _decode_erasure_evidence_details(extensions, host_authored_at=TrellisTimestamp(seconds=1_745_000_100, nanos=0))


def test_decode_step1_rejects_kid_wrong_size() -> None:
    extensions = erasure_extensions(kid_destroyed=b"\x00" * 15)
    with pytest.raises(VerifyError, match="kid_destroyed"):
        _decode_erasure_evidence_details(extensions, host_authored_at=TrellisTimestamp(seconds=1_745_000_100, nanos=0))


def test_decode_deployment_wide_scope_decodes() -> None:
    extensions = erasure_extensions(
        subject_scope=deployment_wide_scope(),
        attestations=[one_attestation("prior"), one_attestation("new")],
        cascade_scopes=["CS-01", "CS-02", "CS-03", "CS-04", "CS-05", "CS-06"],
    )
    details = _decode_erasure_evidence_details(extensions, host_authored_at=TrellisTimestamp(seconds=1_745_000_100, nanos=0))
    assert details is not None
    assert details.subject_scope_kind == "deployment-wide"
    assert len(details.cascade_scopes) == 6
    assert details.attestation_classes == ["prior", "new"]


# ---------------------------------------------------------------------------
# finalize_erasure_evidence — steps 2 / 5 / 7 / 8.
# ---------------------------------------------------------------------------


def test_finalize_empty_input_produces_empty_outcome() -> None:
    failures: list[VerificationFailure] = []
    out = _finalize_erasure_evidence([], [], {}, None, failures)
    assert out == []
    assert failures == []


def test_finalize_step8_flags_post_erasure_use_for_signing_class() -> None:
    kid = b"\xaa" * 16
    payload = payload_details(kid, "signing", 100)
    canonical_hash = b"\x00" * 32
    payloads = [(0, payload, canonical_hash)]

    later_hash = b"\x01" * 32
    chain = [
        chain_summary(0, 100, kid, [], canonical_hash),
        chain_summary(1, 200, kid, [], later_hash),
    ]

    failures: list[VerificationFailure] = []
    outcomes = _finalize_erasure_evidence(payloads, chain, {}, None, failures)
    assert len(outcomes) == 1
    assert outcomes[0].post_erasure_uses == 1
    assert outcomes[0].post_erasure_wraps == 0
    assert any(f.kind == "post_erasure_use" and f.location == later_hash.hex() for f in failures)


def test_finalize_step8_flags_post_erasure_wrap_for_subject_class() -> None:
    kid = b"\xbb" * 16
    payload = payload_details(kid, "subject", 100)
    canonical_hash = b"\x00" * 32
    payloads = [(0, payload, canonical_hash)]

    signing_kid = b"\xcc" * 16
    later_hash = b"\x02" * 32
    chain = [
        chain_summary(0, 100, signing_kid, [], canonical_hash),
        chain_summary(1, 200, signing_kid, [kid], later_hash),
    ]

    failures: list[VerificationFailure] = []
    outcomes = _finalize_erasure_evidence(payloads, chain, {}, None, failures)
    assert len(outcomes) == 1
    assert outcomes[0].post_erasure_uses == 0
    assert outcomes[0].post_erasure_wraps == 1
    assert any(f.kind == "post_erasure_wrap" and f.location == later_hash.hex() for f in failures)


def test_finalize_step8_phase1_skips_recovery_class_chain_walk() -> None:
    """ADR 0005 step 8 Phase-1 scope: recovery / scope / tenant-root and
    extension-`tstr` classes do NOT trigger the chain-walk in Phase 1.
    """
    kid = b"\xdd" * 16
    payload = payload_details(kid, "recovery", 100)
    canonical_hash = b"\x00" * 32
    payloads = [(0, payload, canonical_hash)]

    later_hash = b"\x03" * 32
    chain = [
        chain_summary(0, 100, kid, [], canonical_hash),
        chain_summary(1, 200, kid, [], later_hash),
    ]

    failures: list[VerificationFailure] = []
    outcomes = _finalize_erasure_evidence(payloads, chain, {}, None, failures)
    assert len(outcomes) == 1
    assert outcomes[0].post_erasure_uses == 0
    assert outcomes[0].post_erasure_wraps == 0
    assert not any(f.kind == "post_erasure_use" for f in failures)


def test_finalize_step5_flags_destroyed_at_conflict_for_same_kid() -> None:
    kid = b"\xee" * 16
    payload_a = payload_details(kid, "signing", 100)
    payload_b = payload_details(kid, "signing", 200)
    hash_a = b"\x00" * 32
    hash_b = b"\x01" * 32
    payloads = [(0, payload_a, hash_a), (1, payload_b, hash_b)]

    chain = [
        chain_summary(0, 100, kid, [], hash_a),
        chain_summary(1, 150, kid, [], hash_b),
    ]

    failures: list[VerificationFailure] = []
    outcomes = _finalize_erasure_evidence(payloads, chain, {}, None, failures)
    assert len(outcomes) == 2
    assert any(f.kind == "erasure_destroyed_at_conflict" for f in failures)
    assert any(s == "erasure_destroyed_at_conflict" for s in outcomes[1].failures)


def test_finalize_step5_flags_key_class_conflict_for_same_kid() -> None:
    kid = b"\xf0" * 16
    payload_a = payload_details(kid, "signing", 100)
    payload_b = payload_details(kid, "subject", 100)
    hash_a = b"\x00" * 32
    hash_b = b"\x01" * 32
    payloads = [(0, payload_a, hash_a), (1, payload_b, hash_b)]

    chain = [
        chain_summary(0, 100, kid, [], hash_a),
        chain_summary(1, 150, kid, [], hash_b),
    ]

    failures: list[VerificationFailure] = []
    outcomes = _finalize_erasure_evidence(payloads, chain, {}, None, failures)
    assert len(outcomes) == 2
    assert any(f.kind == "erasure_key_class_payload_conflict" for f in failures)


def test_finalize_step2_flags_registry_class_mismatch_for_signing_kid() -> None:
    kid = b"\xf1" * 16
    payload = payload_details(kid, "subject", 100)
    canonical_hash = b"\x00" * 32
    payloads = [(0, payload, canonical_hash)]
    chain = [chain_summary(0, 100, kid, [], canonical_hash)]

    registry = {kid: SigningKeyEntry(public_key=b"\x00" * 32, status=1, valid_to=None)}
    failures: list[VerificationFailure] = []
    _ = _finalize_erasure_evidence(payloads, chain, registry, None, failures)
    assert any(f.kind == "erasure_key_class_registry_mismatch" for f in failures)


def test_finalize_step2_accepts_matching_signing_class() -> None:
    kid = b"\xf2" * 16
    payload = payload_details(kid, "signing", 100)
    canonical_hash = b"\x00" * 32
    payloads = [(0, payload, canonical_hash)]
    chain = [chain_summary(0, 100, kid, [], canonical_hash)]

    registry = {kid: SigningKeyEntry(public_key=b"\x00" * 32, status=1, valid_to=None)}
    failures: list[VerificationFailure] = []
    outcomes = _finalize_erasure_evidence(payloads, chain, registry, None, failures)
    assert len(outcomes) == 1
    assert not any(f.kind == "erasure_key_class_registry_mismatch" for f in failures)


def test_finalize_step2_flags_registry_class_mismatch_for_subject_kid() -> None:
    """Non-signing registry has the kid as a subject class; payload claims
    signing. Step 2 → erasure_key_class_registry_mismatch.
    """
    kid = b"\xf5" * 16
    payload = payload_details(kid, "signing", 100)
    canonical_hash = b"\x00" * 32
    payloads = [(0, payload, canonical_hash)]
    chain = [chain_summary(0, 100, kid, [], canonical_hash)]

    non_signing = {kid: NonSigningKeyEntry(class_="subject")}
    failures: list[VerificationFailure] = []
    _ = _finalize_erasure_evidence(payloads, chain, {}, non_signing, failures)
    assert any(f.kind == "erasure_key_class_registry_mismatch" for f in failures)


def test_finalize_step7_flags_malformed_attestation_signature() -> None:
    kid = b"\xf3" * 16
    payload = payload_details(kid, "signing", 100)
    payload.attestation_signatures_well_formed = False
    canonical_hash = b"\x00" * 32
    payloads = [(0, payload, canonical_hash)]
    chain = [chain_summary(0, 100, kid, [], canonical_hash)]

    failures: list[VerificationFailure] = []
    outcomes = _finalize_erasure_evidence(payloads, chain, {}, None, failures)
    assert len(outcomes) == 1
    assert outcomes[0].signature_verified is False
    assert any(f.kind == "erasure_attestation_signature_invalid" for f in failures)


def test_finalize_step8_no_post_erasure_use_when_authored_at_equals_destroyed_at() -> None:
    """ADR 0005 step 8 comparison rule: `authored_at > destroyed_at`
    (strict). Equal timestamps are not flagged.
    """
    kid = b"\xf4" * 16
    payload = payload_details(kid, "signing", 100)
    canonical_hash = b"\x00" * 32
    payloads = [(0, payload, canonical_hash)]
    chain = [chain_summary(0, 100, kid, [], canonical_hash)]

    failures: list[VerificationFailure] = []
    outcomes = _finalize_erasure_evidence(payloads, chain, {}, None, failures)
    assert len(outcomes) == 1
    assert outcomes[0].post_erasure_uses == 0
    assert outcomes[0].post_erasure_wraps == 0
