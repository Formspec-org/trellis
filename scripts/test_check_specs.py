"""Lint test harness for check-specs.py fixture-coverage rules.

Each test invokes check-specs.py as a subprocess with TRELLIS_LINT_ROOT
pointing at a synthetic scenario under check-specs-fixtures/.  Tests assert
on exit code and stderr content.

This is the RED-phase foundation for Tasks 5–7, which implement the actual
coverage rules.  All three tests are expected to fail until Task 4 wires up
TRELLIS_LINT_ROOT in check-specs.py.
"""

import os
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LINT = ROOT / "scripts" / "check-specs.py"
FIX = Path(__file__).resolve().parent / "check-specs-fixtures"


def run_lint(scenario: str) -> subprocess.CompletedProcess:
    env = os.environ.copy()
    env.pop("TRELLIS_SKIP_COVERAGE", None)
    env["TRELLIS_LINT_ROOT"] = str(FIX / scenario)
    return subprocess.run(
        ["python3", str(LINT)],
        env=env,
        capture_output=True,
        text=True,
    )


class TestCoverageLint(unittest.TestCase):
    def test_minimal_valid_scenario_passes(self):
        result = run_lint("minimal-valid")
        self.assertEqual(result.returncode, 0, msg=result.stderr)

    def test_testable_row_without_vector_fails(self):
        result = run_lint("missing-coverage")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("no vector covers", result.stderr.lower())

    def test_forbidden_generator_import_fails(self):
        result = run_lint("forbidden-import")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("forbidden import", result.stderr.lower())

    # I3 — relative imports in _generator/ must be rejected
    def test_relative_generator_import_fails(self):
        result = run_lint("forbidden-import")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("relative imports forbidden", result.stderr.lower())

    # I4 — stdlib modules beyond the original six must be accepted
    def test_stdlib_base64_import_passes(self):
        result = run_lint("stdlib-base64")
        self.assertEqual(result.returncode, 0, msg=result.stderr)

    # C1 / M6 — invariant-gap: hash-prefixed and multi-value cells are parsed correctly
    def test_invariant_gap_fails(self):
        result = run_lint("invariant-gap")
        self.assertNotEqual(result.returncode, 0)
        # Invariants #1 and #4 (from '#1, #4' in TR-CORE-006) are uncovered;
        # invariant #5 (from '#5' in TR-CORE-005) is covered by the vector.
        self.assertIn("#1", result.stderr)
        self.assertIn("#4", result.stderr)
        # #5 must NOT appear as an uncovered invariant
        self.assertNotIn("invariant #5 has no", result.stderr)

    # F1 / M6 — declared-vs-derived invariant mismatch is a WARNING, not an error.
    # Matrix rows with Invariant=— make bidirectional enforcement incoherent;
    # invariants declaration is commentary-only per amended design.
    def test_declared_invariants_mismatch_warns(self):
        result = run_lint("declared-mismatch")
        self.assertEqual(result.returncode, 0, msg=f"stderr={result.stderr!r}")
        self.assertIn("warning:", result.stderr.lower())
        self.assertIn("declared invariants", result.stderr.lower())
        self.assertIn("commentary only", result.stderr.lower())


class TestManifestPathsLint(unittest.TestCase):
    """A/F7 — check_vector_manifest_paths verifies every [inputs]/[expected]
    string value resolves to a real file."""

    def test_missing_sibling_input_fails(self):
        # Manifest references input-authored-event.cbor that does not exist.
        result = run_lint("manifest-missing-sibling")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("input-authored-event.cbor", result.stderr)
        self.assertIn("does not exist", result.stderr.lower())

    def test_missing_keys_relative_fails(self):
        # Manifest references ../../_keys/issuer-xyz.cose_key which is absent.
        result = run_lint("manifest-missing-keys-ref")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("issuer-xyz.cose_key", result.stderr)

    def test_all_manifest_paths_resolve_passes(self):
        # Manifest references sibling + ../../_keys/ + ../../_inputs/ files, all present.
        result = run_lint("manifest-all-present")
        self.assertEqual(result.returncode, 0, msg=result.stderr)

    def test_zip_sha256_is_not_treated_as_path(self):
        # Export manifest has a zip_sha256 hex digest in [expected]; it must NOT
        # be resolved as a path.
        result = run_lint("manifest-zip-sha256")
        self.assertEqual(result.returncode, 0, msg=result.stderr)

    def test_boolean_in_expected_report_is_skipped(self):
        # [expected.report] has booleans like structure_verified = true — nested
        # non-string values must be ignored by the path-resolution check.
        result = run_lint("manifest-boolean-expected")
        self.assertEqual(result.returncode, 0, msg=result.stderr)

    # F1 — lists of path strings must be recursed into, not silently skipped.
    def test_list_of_paths_with_missing_element_fails(self):
        result = run_lint("manifest-list-missing-element")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("missing-payload.bin", result.stderr)
        self.assertIn("does not exist", result.stderr.lower())

    # F2 — absolute paths bypass the vector_dir sandbox and must be rejected.
    def test_absolute_path_is_rejected(self):
        result = run_lint("manifest-absolute-path")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("/etc/passwd", result.stderr)
        self.assertIn("absolute", result.stderr.lower())

    # F2 — empty-string paths resolve to vector_dir itself and must be rejected.
    def test_empty_path_is_rejected(self):
        result = run_lint("manifest-empty-path")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("empty", result.stderr.lower())


class TestPendingInvariantsAllowlist(unittest.TestCase):
    """F5 — fixtures/vectors/_pending-invariants.toml replaces
    TRELLIS_SKIP_COVERAGE=1 blanket bypass."""

    def test_skip_coverage_env_has_no_effect(self):
        # TRELLIS_SKIP_COVERAGE=1 must NOT silence an uncovered testable row.
        env = os.environ.copy()
        env["TRELLIS_SKIP_COVERAGE"] = "1"
        env["TRELLIS_LINT_ROOT"] = str(FIX / "missing-coverage")
        result = subprocess.run(
            ["python3", str(LINT)], env=env, capture_output=True, text=True,
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("no vector covers", result.stderr.lower())

    def test_tr_row_listed_pending_and_uncovered_passes(self):
        # pending_matrix_rows lists TR-CORE-001; no vector references it → OK.
        result = run_lint("allowlist-tr-pending-ok")
        self.assertEqual(result.returncode, 0, msg=result.stderr)

    def test_tr_row_listed_pending_but_now_covered_fails(self):
        # pending_matrix_rows lists TR-CORE-001 but a vector covers it → must fail,
        # forcing cleanup of the allowlist.
        result = run_lint("allowlist-tr-listed-but-covered")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("TR-CORE-001", result.stderr)
        self.assertIn("pending", result.stderr.lower())

    def test_tr_row_uncovered_and_not_listed_fails(self):
        # TR-CORE-001 uncovered and not in pending_matrix_rows → must fail.
        result = run_lint("allowlist-tr-uncovered-not-listed")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("no vector covers", result.stderr.lower())
        self.assertIn("TR-CORE-001", result.stderr)

    # F8 — allowlist entry that isn't a real matrix row ID must error out.
    def test_tr_row_unknown_id_fails(self):
        result = run_lint("allowlist-tr-unknown-id")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("TR-CORE-999", result.stderr)
        self.assertIn("not a matrix row", result.stderr.lower())

    # F8 — non-integer in pending_invariants must produce a clean lint error,
    # not a raw Python ValueError traceback.
    def test_invariant_bad_type_reports_clean_error(self):
        result = run_lint("allowlist-inv-bad-type")
        self.assertNotEqual(result.returncode, 0)
        # Must NOT expose a Python traceback.
        self.assertNotIn("Traceback", result.stderr)
        self.assertNotIn("ValueError", result.stderr)
        # Must reference the offending file and offending value.
        self.assertIn("_pending-invariants.toml", result.stderr)
        self.assertIn("three", result.stderr)

    def test_invariant_pending_and_uncovered_passes(self):
        # pending_invariants = [6]; no vector for invariant 6 → OK.
        result = run_lint("allowlist-inv-pending-ok")
        self.assertEqual(result.returncode, 0, msg=result.stderr)

    def test_invariant_listed_but_now_covered_fails(self):
        # pending_invariants = [6] but a vector covers TR-CORE tied to inv 6 → fail.
        result = run_lint("allowlist-inv-listed-but-covered")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("invariant", result.stderr.lower())
        self.assertIn("6", result.stderr)

    def test_missing_allowlist_file_behaves_as_empty(self):
        # Scenario has uncovered rows and no _pending-invariants.toml → must
        # fail as if the allowlist were empty.
        result = run_lint("allowlist-absent-file")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("no vector covers", result.stderr.lower())

    def test_malformed_allowlist_toml_errors(self):
        # A malformed _pending-invariants.toml must yield a clear lint error,
        # not a Python traceback or silent skip.
        result = run_lint("allowlist-malformed-toml")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("_pending-invariants.toml", result.stderr)


class TestVectorLifecycleFields(unittest.TestCase):
    """F6 — manifest-level status / deprecated_at lifecycle fields.

    status is optional and defaults to 'active'. When status = 'deprecated',
    deprecated_at is required and must be a 'YYYY-MM-DD' ISO-8601 date string.
    Deprecated vectors are excluded from byte-testable coverage audits.
    """

    def test_status_active_without_deprecated_at_passes(self):
        result = run_lint("lifecycle-status-active")
        self.assertEqual(result.returncode, 0, msg=result.stderr)

    def test_deprecated_vector_does_not_count_toward_coverage(self):
        # The only vector is deprecated and covers TR-CORE-001..015; the audit
        # must treat those rows as uncovered.
        result = run_lint("lifecycle-status-deprecated-excluded")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("no vector covers", result.stderr.lower())
        self.assertIn("TR-CORE-001", result.stderr)

    def test_deprecated_without_deprecated_at_fails(self):
        result = run_lint("lifecycle-deprecated-missing-date")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("deprecated_at", result.stderr)

    def test_deprecated_with_malformed_date_fails(self):
        result = run_lint("lifecycle-deprecated-malformed-date")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("deprecated_at", result.stderr)
        self.assertIn("yesterday", result.stderr)

    def test_unknown_status_value_fails(self):
        result = run_lint("lifecycle-status-unknown")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("status", result.stderr.lower())
        self.assertIn("foo", result.stderr)


if __name__ == "__main__":
    unittest.main()
