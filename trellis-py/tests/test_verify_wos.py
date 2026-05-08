from __future__ import annotations

from trellis_py import verify as core
from trellis_py import verify_wos


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
