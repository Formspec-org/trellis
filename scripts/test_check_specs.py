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


if __name__ == "__main__":
    unittest.main()
