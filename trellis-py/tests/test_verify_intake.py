"""Unit tests for intake-handoff verification helpers (trellis_py.verify)."""

from __future__ import annotations

import hashlib

import cbor2
import pytest

from trellis_py.verify import VerifyError, _parse_intake_accepted_record, _response_hash_matches


def test_parse_intake_accepted_rejects_empty_outputs() -> None:
    payload = {
        "recordKind": "intakeAccepted",
        "data": {
            "intakeId": "handoff-1",
            "caseIntent": "requestGovernedCaseCreation",
            "caseDisposition": "createGovernedCase",
            "caseRef": "case-1",
        },
        "outputs": [],
    }
    with pytest.raises(VerifyError, match="outputs array is missing or empty"):
        _parse_intake_accepted_record(cbor2.dumps(payload))


def test_response_hash_matches_ok() -> None:
    body = b"hello-response"
    digest = hashlib.sha256(body).digest()
    text = "sha256:" + digest.hex()
    ok, err = _response_hash_matches(text, body)
    assert ok is True
    assert err is None


def test_response_hash_matches_wrong_bytes() -> None:
    ok, err = _response_hash_matches("sha256:" + "00" * 32, b"wrong-bytes")
    assert ok is False
    assert err is None


def test_response_hash_matches_bad_prefix() -> None:
    ok, err = _response_hash_matches("md5:abc", b"x")
    assert ok is False
    assert err is not None
    assert "sha256" in err.lower()
