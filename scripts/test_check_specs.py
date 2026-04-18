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


if __name__ == "__main__":
    unittest.main()
