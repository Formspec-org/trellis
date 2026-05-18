#!/usr/bin/env python3
"""Regression guard for SignedAct projection corpus parity.

The WOS/Formspec signature export generator is the Python oracle for the
projection fixture bytes. This check regenerates its output into a temp tree,
compares every generated binary artifact with the committed corpus, and runs
the Python WOS verifier over the signed-acts positive and negative vectors.
Rust consumes the same corpus through `cargo nextest run -p trellis-verify-wos`.
"""

from __future__ import annotations

import importlib.util
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "trellis-py" / "src"))

GENERATOR = ROOT / "fixtures" / "vectors" / "_generator" / "gen_signature_export_006.py"
VECTORS = ROOT / "fixtures" / "vectors"
AUTHORING_FILES = {"manifest.toml", "derivation.md"}
GENERATED_DIRS = [
    ("export", "006-signature-affirmations-inline"),
    ("export", "007-signature-admission-failed-inline"),
    ("export", "008-signed-acts-fallback-act-id"),
    ("export", "009-signed-acts-manifest-only"),
    ("verify", "014-export-006-signature-row-mismatch"),
    ("verify", "019-export-006-signed-acts-render-drift"),
    ("verify", "020-export-006-signed-acts-unsupported-rule"),
    ("verify", "021-signed-acts-manifest-tamper"),
    ("verify", "022-066-render-drift-tampered-only"),
    ("tamper", "014-signature-catalog-digest-mismatch"),
    ("tamper", "055-signed-acts-catalog-digest-mismatch"),
    ("tamper", "056-policy-closure-digest-mismatch"),
]


def tree_bytes(base: Path) -> dict[str, bytes]:
    files: dict[str, bytes] = {}
    for path in sorted(base.rglob("*")):
        if not path.is_file():
            continue
        rel = path.relative_to(base).as_posix()
        if rel in AUTHORING_FILES:
            continue
        files[rel] = path.read_bytes()
    return files


def load_generator():
    spec = importlib.util.spec_from_file_location("gen_signature_export_006", GENERATOR)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load generator at {GENERATOR}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def run_generator(tmp: Path) -> None:
    module = load_generator()
    module.OUT_EXPORT_006 = tmp / "export" / "006-signature-affirmations-inline"
    module.OUT_EXPORT_007 = tmp / "export" / "007-signature-admission-failed-inline"
    module.OUT_EXPORT_008 = tmp / "export" / "008-signed-acts-fallback-act-id"
    module.OUT_EXPORT_009 = tmp / "export" / "009-signed-acts-manifest-only"
    module.OUT_VERIFY_014 = tmp / "verify" / "014-export-006-signature-row-mismatch"
    module.OUT_VERIFY_019 = tmp / "verify" / "019-export-006-signed-acts-render-drift"
    module.OUT_VERIFY_020 = tmp / "verify" / "020-export-006-signed-acts-unsupported-rule"
    module.OUT_VERIFY_021 = tmp / "verify" / "021-signed-acts-manifest-tamper"
    module.OUT_VERIFY_022 = tmp / "verify" / "022-066-render-drift-tampered-only"
    module.OUT_TAMPER_014 = tmp / "tamper" / "014-signature-catalog-digest-mismatch"
    module.OUT_TAMPER_055 = tmp / "tamper" / "055-signed-acts-catalog-digest-mismatch"
    module.OUT_TAMPER_056 = tmp / "tamper" / "056-policy-closure-digest-mismatch"
    module.main()


def compare_generated_tree(tmp: Path) -> bool:
    ok = True
    for op, name in GENERATED_DIRS:
        rel = Path(op) / name
        if not (VECTORS / rel).is_dir():
            print(f"committed vector directory missing: {rel}", file=sys.stderr)
            ok = False
            continue
        if not (tmp / rel).is_dir():
            print(f"generator did not emit vector directory: {rel}", file=sys.stderr)
            ok = False
            continue
        expected = tree_bytes(VECTORS / rel)
        actual = tree_bytes(tmp / rel)
        if actual.keys() != expected.keys():
            only_expected = sorted(expected.keys() - actual.keys())
            only_actual = sorted(actual.keys() - expected.keys())
            print(
                f"Generated file set differs for {rel}:\n"
                f"  only in committed corpus: {only_expected}\n"
                f"  only in generated tree: {only_actual}",
                file=sys.stderr,
            )
            ok = False
            continue
        for child in sorted(actual):
            if actual[child] != expected[child]:
                print(f"bytes differ: {rel / child}", file=sys.stderr)
                ok = False
    return ok


def assert_no_wos_failures(verify_wos, export_zip: Path) -> None:
    report = verify_wos.verify_export_zip(export_zip.read_bytes())
    failures = [finding.kind for finding in report.wos_findings if finding.severity == "failure"]
    if failures:
        raise AssertionError(f"{export_zip} emitted WOS failures: {failures}")
    if report.verdict.projection_integrity != "pass":
        raise AssertionError(
            f"{export_zip} projection_integrity={report.verdict.projection_integrity}"
        )


def assert_wos_failure(
    verify_wos,
    export_zip: Path,
    expected_failures: set[str],
    *,
    projection_integrity: str,
    blocking_reasons: list[str],
) -> None:
    report = verify_wos.verify_export_zip(export_zip.read_bytes())
    if not report.trellis.structure_verified:
        raise AssertionError(f"{export_zip} did not preserve substrate structure")
    if not report.trellis.integrity_verified:
        raise AssertionError(f"{export_zip} did not preserve substrate integrity")
    if not report.trellis.readability_verified:
        raise AssertionError(f"{export_zip} did not preserve substrate readability")
    if report.integrity_verified:
        raise AssertionError(f"{export_zip} unexpectedly passed composed integrity")
    if report.verdict.cryptographic_integrity != "pass":
        raise AssertionError(
            f"{export_zip} cryptographic_integrity={report.verdict.cryptographic_integrity}"
        )
    if report.verdict.projection_integrity != projection_integrity:
        raise AssertionError(
            f"{export_zip} projection_integrity={report.verdict.projection_integrity}"
        )
    if report.verdict.relying_party_result != "invalid":
        raise AssertionError(
            f"{export_zip} relying_party_result={report.verdict.relying_party_result}"
        )
    if report.verdict.domain_admissibility != "pass":
        raise AssertionError(
            f"{export_zip} domain_admissibility={report.verdict.domain_admissibility}"
        )
    if report.verdict.blocking_reasons != blocking_reasons:
        raise AssertionError(
            f"{export_zip} blocking_reasons={report.verdict.blocking_reasons}"
        )
    failures = {
        finding.kind for finding in report.wos_findings if finding.severity == "failure"
    }
    if failures != expected_failures:
        raise AssertionError(
            f"{export_zip} failures {sorted(failures)}, expected {sorted(expected_failures)}"
        )


def check_python_verifier_vectors() -> None:
    from trellis_py import verify_wos

    assert_no_wos_failures(
        verify_wos,
        VECTORS / "export/006-signature-affirmations-inline/expected-export.zip"
    )
    assert_no_wos_failures(
        verify_wos,
        VECTORS / "export/007-signature-admission-failed-inline/expected-export.zip"
    )
    assert_no_wos_failures(
        verify_wos,
        VECTORS / "export/008-signed-acts-fallback-act-id/expected-export.zip"
    )
    assert_no_wos_failures(
        verify_wos,
        VECTORS / "export/009-signed-acts-manifest-only/expected-export.zip"
    )
    # verify/019 + verify/022: 066 render-drift advisories — verdict stays
    # valid (the substrate-anchored 068 manifest is the load-bearing proof).
    assert_no_wos_failures(
        verify_wos,
        VECTORS / "verify/019-export-006-signed-acts-render-drift/input-export.zip"
    )
    assert_no_wos_failures(
        verify_wos,
        VECTORS / "verify/022-066-render-drift-tampered-only/input-export.zip"
    )
    assert_wos_failure(
        verify_wos,
        VECTORS / "verify/020-export-006-signed-acts-unsupported-rule/input-export.zip",
        {"signed_acts_catalog_invalid"},
        projection_integrity="fail",
        blocking_reasons=["projection_integrity"],
    )
    # verify/021 (068 manifest tamper): the Python 068 verifier landed in Task
    # A9, so this assertion is the permanent cross-runtime parity guard that
    # Python's verdict for the tampered 068 member matches Rust's
    # (`cargo nextest run -p trellis-conformance` exercises the same fixture).
    # Blocking reason is `projection_mismatch` because the verdict assembly in
    # `_verdict_from_parts` reads the finding-kind set to distinguish manifest
    # mismatch from generic projection-integrity failures.
    assert_wos_failure(
        verify_wos,
        VECTORS / "verify/021-signed-acts-manifest-tamper/input-export.zip",
        {"signed_acts_manifest_extension_digest_mismatch"},
        projection_integrity="fail",
        blocking_reasons=["projection_integrity"],
    )
    assert_wos_failure(
        verify_wos,
        VECTORS / "tamper/055-signed-acts-catalog-digest-mismatch/input-export.zip",
        {"signed_acts_catalog_digest_mismatch"},
        projection_integrity="fail",
        blocking_reasons=["projection_integrity"],
    )


def check_cross_runtime_manifest_byte_identity() -> None:
    """Assert Python `encode_signed_acts_manifest_v1` output is byte-identical
    to the committed Rust-emitted `068-signed-acts-manifest.cbor` member across
    every export fixture that ships a 068 member.

    The committed bytes ARE the Rust output (the fixture corpus is generated
    by the Rust writer pipeline). Re-deriving the manifest from the same
    `010-events.cbor` source events on the Python side and byte-comparing
    against the archive member is the canonical cross-runtime parity check
    this script promotes to a permanent invariant (Task A9).
    """
    from trellis_py import verify as core
    from trellis_py import verify_wos

    export_fixtures = [
        "export/006-signature-affirmations-inline",
        "export/007-signature-admission-failed-inline",
        "export/008-signed-acts-fallback-act-id",
        "export/009-signed-acts-manifest-only",
    ]
    for fixture in export_fixtures:
        export_zip_path = VECTORS / fixture / "expected-export.zip"
        archive = core.parse_export_zip(export_zip_path.read_bytes())
        committed_member_bytes = archive.get(
            verify_wos.SIGNED_ACTS_MANIFEST_MEMBER
        )
        if committed_member_bytes is None:
            raise AssertionError(
                f"{fixture} is registered for 068 byte-identity but archive "
                f"is missing {verify_wos.SIGNED_ACTS_MANIFEST_MEMBER}"
            )
        events_bytes = archive.get("010-events.cbor")
        if events_bytes is None:
            raise AssertionError(f"{fixture} archive is missing 010-events.cbor")
        events = core._parse_sign1_array(events_bytes)  # noqa: SLF001
        decoded: list[core.EventDetails] = []
        for event in events:
            try:
                decoded.append(core._decode_event_details(event))  # noqa: SLF001
            except core.VerifyError as exc:
                raise AssertionError(
                    f"{fixture}: cannot decode event for re-derivation: {exc}"
                ) from exc
        manifest = verify_wos.derive_signed_acts_manifest_v1(decoded)
        python_bytes = verify_wos.encode_signed_acts_manifest_v1(manifest)
        if python_bytes != committed_member_bytes:
            raise AssertionError(
                f"{fixture}: Python-derived 068 bytes "
                f"({python_bytes.hex()[:64]}...) do not match committed Rust "
                f"output ({committed_member_bytes.hex()[:64]}...)"
            )


def main() -> int:
    try:
        import cbor2  # noqa: F401
        import cryptography  # noqa: F401
    except ImportError:
        print(
            "Missing generator deps (cbor2, cryptography). Install with:\n"
            "  pip install -e ./trellis-py",
            file=sys.stderr,
        )
        return 2

    if not GENERATOR.is_file():
        print(f"generator not found at {GENERATOR}", file=sys.stderr)
        return 2

    with tempfile.TemporaryDirectory() as tmp_str:
        tmp = Path(tmp_str)
        run_generator(tmp)
        if not compare_generated_tree(tmp):
            return 1

    check_python_verifier_vectors()
    check_cross_runtime_manifest_byte_identity()
    print(
        "OK: signed-acts projection generator, Python verifier, and "
        "cross-runtime 068 byte-identity all match the corpus"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
