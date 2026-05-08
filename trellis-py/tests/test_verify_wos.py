from __future__ import annotations

import pytest

from trellis_py import verify as core
from trellis_py import verify_wos


def test_map_lookup_str_alias_returns_preferred_when_string() -> None:
    out = core._map_lookup_str_alias(  # noqa: SLF001
        {"preferred": "PREF", "legacy": "LEG"}, "preferred", "legacy"
    )
    assert out == "PREF"


def test_map_lookup_str_alias_falls_back_when_preferred_absent() -> None:
    out = core._map_lookup_str_alias(  # noqa: SLF001
        {"legacy": "LEG"}, "preferred", "legacy"
    )
    assert out == "LEG"


def test_map_lookup_str_alias_rejects_present_but_non_string_preferred() -> None:
    """Phase O narrowing (review F3): a present-but-malformed preferred key
    MUST NOT silently fall through to the legacy alias. Surface a parse
    error instead so the caller emits a localized failure rather than
    silently accepting the legacy value."""
    with pytest.raises(core.VerifyError):
        core._map_lookup_str_alias(  # noqa: SLF001
            {"preferred": 42, "legacy": "LEG"}, "preferred", "legacy"
        )


def test_signature_export_extension_parse_error_becomes_wos_finding() -> None:
    findings = verify_wos._validate_export(  # noqa: SLF001
        archive={},
        events=[],
        payload_blobs={},
        manifest_map={
            "extensions": {
                verify_wos.SIGNATURE_EXPORT_EXTENSION: b"not-a-cbor-map",
            }
        },
        generated_at=core.TrellisTimestamp(1, 0),
    )

    assert len(findings) == 1
    assert findings[0].kind == "signature_catalog_invalid"
    assert findings[0].severity == "failure"
    assert "signature export extension is invalid" in findings[0].detail


def test_intake_export_extension_parse_error_becomes_wos_finding() -> None:
    findings = verify_wos._validate_export(  # noqa: SLF001
        archive={},
        events=[],
        payload_blobs={},
        manifest_map={
            "extensions": {
                verify_wos.INTAKE_EXPORT_EXTENSION: b"not-a-cbor-map",
            }
        },
        generated_at=core.TrellisTimestamp(1, 0),
    )

    assert len(findings) == 1
    assert findings[0].kind == "intake_handoff_catalog_invalid"
    assert findings[0].severity == "failure"
    assert "intake export extension is invalid" in findings[0].detail
