from __future__ import annotations

import tomllib
from pathlib import Path

from trellis_py.verify import verify_export_zip, verify_tampered_ledger


def test_verify_tampered_ledger_detects_timestamp_order_violation() -> None:
    root = (
        Path(__file__).resolve().parents[2]
        / "fixtures"
        / "vectors"
        / "tamper"
        / "041-timestamp-backwards"
    )
    manifest = tomllib.loads((root / "manifest.toml").read_text("utf-8"))
    inputs = manifest["inputs"]

    report = verify_tampered_ledger(
        (root / inputs["signing_key_registry"]).read_bytes(),
        (root / inputs["ledger"]).read_bytes(),
        None,
        None,
    )

    assert report.structure_verified is True
    assert report.integrity_verified is False
    assert report.readability_verified is True
    assert report.event_failures
    assert report.event_failures[0].kind == "timestamp_order_violation"


def test_verify_export_zip_distinguishes_missing_interop_sidecar() -> None:
    root = (
        Path(__file__).resolve().parents[2]
        / "fixtures"
        / "vectors"
        / "tamper"
        / "044-interop-sidecar-missing"
    )
    manifest = tomllib.loads((root / "manifest.toml").read_text("utf-8"))
    inputs = manifest["inputs"]

    report = verify_export_zip((root / inputs["export_zip"]).read_bytes())

    assert report.structure_verified is False
    assert report.integrity_verified is False
    assert report.readability_verified is False
    assert report.event_failures
    assert report.event_failures[0].kind == "interop_sidecar_missing"
