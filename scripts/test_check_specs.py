"""Lint test harness for check-specs.py fixture-coverage rules.

Each test invokes check-specs.py as a subprocess with TRELLIS_LINT_ROOT
pointing at a synthetic scenario under check-specs-fixtures/.  Tests assert
on exit code and stderr content.

This is the RED-phase foundation for Tasks 5–7, which implement the actual
coverage rules.  All three tests are expected to fail until Task 4 wires up
TRELLIS_LINT_ROOT in check-specs.py.
"""

import importlib.util
import os
import subprocess
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
LINT = ROOT / "scripts" / "check-specs.py"
FIX = Path(__file__).resolve().parent / "check-specs-fixtures"


def _load_check_specs_module():
    """Import check-specs.py as a module for direct helper-level tests.

    The hyphenated filename prevents a normal `import check_specs`, so we
    load via importlib. The module's ROOT defaults to the real repo when
    TRELLIS_LINT_ROOT is unset, which is what helper tests want.
    """
    spec = importlib.util.spec_from_file_location("check_specs", LINT)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


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


class TestVectorNaming(unittest.TestCase):
    """R1 — fixture-naming guard: every child of fixtures/vectors/<op>/ MUST
    match `^NNN-slug$` where NNN is three digits and slug is dash-separated
    lowercase alphanumeric segments."""

    def test_valid_naming_passes(self):
        result = run_lint("naming-valid")
        self.assertEqual(result.returncode, 0, msg=result.stderr)

    def test_missing_nnn_prefix_fails(self):
        # append/abc has no numeric prefix at all.
        result = run_lint("naming-missing-nnn")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("abc", result.stderr)
        self.assertIn("naming", result.stderr.lower())

    def test_no_dash_between_nnn_and_slug_fails(self):
        # append/001abc is missing the dash.
        result = run_lint("naming-no-dash")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("001abc", result.stderr)

    def test_uppercase_slug_fails(self):
        # append/001-Cap-Case has uppercase letters in the slug.
        result = run_lint("naming-uppercase-slug")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Cap-Case", result.stderr)

    def test_trailing_dash_fails(self):
        # append/001-foo- ends with a dash.
        result = run_lint("naming-trailing-dash")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("001-foo-", result.stderr)

    def test_two_digit_nnn_fails(self):
        # append/01-foo has only two digits.
        result = run_lint("naming-short-nnn")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("01-foo", result.stderr)

    def test_numeric_only_slug_passes(self):
        # append/002-123 is numeric-only-slug; digits are valid in [a-z0-9].
        result = run_lint("naming-numeric-slug")
        self.assertEqual(result.returncode, 0, msg=result.stderr)


class TestPendingProjectionDrillsLoader(unittest.TestCase):
    """R3 — _pending-projection-drills.toml loader. File parallels
    _pending-invariants.toml but covers projection-rebuild-drill rows. No
    rule consumes the loader yet (R7 lands it); these tests pin the
    load-side behavior only."""

    @classmethod
    def setUpClass(cls):
        cls.cs = _load_check_specs_module()

    def test_missing_file_returns_empty_set(self):
        # Point the loader at a path that does not exist; no error, empty set.
        errors: list[str] = []
        result = self.cs.load_pending_projection_drills(
            errors,
            path=Path("/tmp/does-not-exist-projection-drills.toml"),
        )
        self.assertEqual(result, set())
        self.assertEqual(errors, [])

    def test_malformed_toml_errors(self):
        tmp = Path(os.environ.get("TMPDIR", "/tmp")) / "trellis-pd-bad.toml"
        tmp.write_text("pending_matrix_rows = [not valid\n", encoding="utf-8")
        errors: list[str] = []
        self.cs.load_pending_projection_drills(errors, path=tmp)
        self.assertTrue(any("malformed TOML" in e for e in errors), errors)
        tmp.unlink()

    def test_valid_rows_load(self):
        tmp = Path(os.environ.get("TMPDIR", "/tmp")) / "trellis-pd-ok.toml"
        tmp.write_text(
            'pending_matrix_rows = ["TR-OP-008", "TR-OP-042"]\n',
            encoding="utf-8",
        )
        errors: list[str] = []
        result = self.cs.load_pending_projection_drills(errors, path=tmp)
        self.assertEqual(result, {"TR-OP-008", "TR-OP-042"})
        self.assertEqual(errors, [])
        tmp.unlink()


class TestSharedPlumbing(unittest.TestCase):
    """Commit-1 helpers: op dispatch widening, Companion section resolution,
    split matrix-id helpers, generic allowlist loader, Core §6.7 event-type
    registry, Companion CDDL-block extraction.

    These helpers are dead code wired in commit 1; they do not fire any rule
    in main() until commit 2+. Tests exercise them directly against the real
    repo specs.
    """

    @classmethod
    def setUpClass(cls):
        cls.cs = _load_check_specs_module()

    def test_vector_ops_includes_projection_and_shred(self):
        # Op dispatch must accept the two Wave-1 ops alongside the existing four.
        self.assertIn("projection", self.cs.VECTOR_OPS)
        self.assertIn("shred", self.cs.VECTOR_OPS)
        for op in ("append", "verify", "export", "tamper"):
            self.assertIn(op, self.cs.VECTOR_OPS)

    def test_companion_headings_extracts_part_I_section_numbers(self):
        # Companion §§5..29 are the top-level `## N. Title` headings today
        # (Part I starts at §5 because Part 0 consumes §§1..4 as front-matter
        # sections without ## numbering of their own).
        headings = self.cs.companion_headings()
        self.assertIn(5, headings)
        self.assertIn(29, headings)
        self.assertEqual(headings[10], "Posture-Transition Auditability")
        self.assertEqual(headings[19], "Delegated-Compute Honesty")
        # ≥25 sections expected; guard against regex breakage.
        self.assertGreaterEqual(len(headings), 25)

    def test_tr_op_ids_splits_matrix_ids(self):
        op_ids = self.cs.tr_op_ids()
        # Every id must be a TR-OP-NNN string; none may be TR-CORE-*.
        self.assertTrue(op_ids, "expected ≥1 TR-OP-* row in the matrix")
        for row_id in op_ids:
            self.assertTrue(row_id.startswith("TR-OP-"), row_id)

    def test_tr_core_ids_splits_matrix_ids(self):
        core_ids = self.cs.tr_core_ids()
        self.assertTrue(core_ids)
        for row_id in core_ids:
            self.assertTrue(row_id.startswith("TR-CORE-"), row_id)

    def test_derived_companion_sections_for_tr_op_resolves_anchor(self):
        # Pick a TR-OP row that has a prose anchor in the Companion and check
        # the derived section number lines up with its containing `## N.` heading.
        op_ids = self.cs.tr_op_ids()
        self.assertIn("TR-OP-042", op_ids)
        derived = self.cs.derived_companion_sections_for_tr_op(["TR-OP-042"])
        # TR-OP-042 anchors under Companion §A.5.1 → Appendix A → grandparent
        # `## A. Appendix A` or under the nearest `## N.` heading above it;
        # what we require is that a non-empty set is returned for a known row.
        self.assertTrue(derived, f"expected a section for TR-OP-042, got {derived}")

    def test_load_allowlist_with_int_field_parses_list(self):
        cs = self.cs
        tmp = Path(os.environ.get("TMPDIR", "/tmp")) / "trellis-load-allowlist-int.toml"
        tmp.write_text("pending_invariants = [3, 6, 7]\n", encoding="utf-8")
        errors: list[str] = []
        data = cs.load_allowlist(tmp, errors, int_field="pending_invariants")
        self.assertEqual(data["pending_invariants"], {3, 6, 7})
        self.assertEqual(errors, [])
        tmp.unlink()

    def test_load_allowlist_with_str_field_parses_list(self):
        cs = self.cs
        tmp = Path(os.environ.get("TMPDIR", "/tmp")) / "trellis-load-allowlist-str.toml"
        tmp.write_text('pending_matrix_rows = ["TR-CORE-001"]\n', encoding="utf-8")
        errors: list[str] = []
        data = cs.load_allowlist(tmp, errors, str_field="pending_matrix_rows")
        self.assertEqual(data["pending_matrix_rows"], {"TR-CORE-001"})
        self.assertEqual(errors, [])
        tmp.unlink()

    def test_load_allowlist_missing_file_returns_empty_without_errors(self):
        cs = self.cs
        missing = Path("/tmp/does-not-exist-check-specs-allowlist.toml")
        errors: list[str] = []
        data = cs.load_allowlist(missing, errors, int_field="pending_invariants")
        self.assertEqual(data["pending_invariants"], set())
        self.assertEqual(errors, [])

    def test_load_allowlist_malformed_toml_reports_error(self):
        cs = self.cs
        tmp = Path(os.environ.get("TMPDIR", "/tmp")) / "trellis-load-allowlist-bad.toml"
        tmp.write_text("pending_invariants = [not valid toml\n", encoding="utf-8")
        errors: list[str] = []
        cs.load_allowlist(tmp, errors, int_field="pending_invariants")
        self.assertTrue(any("malformed TOML" in e for e in errors), errors)
        tmp.unlink()

    def test_load_allowlist_rejects_wrong_element_type(self):
        cs = self.cs
        tmp = Path(os.environ.get("TMPDIR", "/tmp")) / "trellis-load-allowlist-bad-type.toml"
        tmp.write_text('pending_invariants = ["three"]\n', encoding="utf-8")
        errors: list[str] = []
        cs.load_allowlist(tmp, errors, int_field="pending_invariants")
        self.assertTrue(any("not an integer" in e for e in errors), errors)
        tmp.unlink()

    def test_core_event_type_registry_returns_registered_identifiers(self):
        registry = self.cs.core_event_type_registry()
        # Per Core §6.7 table — the two reject-if-unknown-at-version Phase-1
        # transition identifiers MUST both be present.
        self.assertIn("trellis.custody-model-transition.v1", registry)
        self.assertIn("trellis.disclosure-profile-transition.v1", registry)
        # Each entry exposes container / phase / purpose fields.
        entry = registry["trellis.custody-model-transition.v1"]
        self.assertEqual(entry["container"], "EventPayload.extensions")
        self.assertEqual(entry["phase"], "1")
        self.assertIn("Custody-model", entry["purpose"])

    def test_companion_cddl_blocks_extracts_named_rules(self):
        blocks = self.cs.companion_cddl_blocks()
        # Keys are (appendix_id, rule_name) tuples.
        keys = set(blocks.keys())
        self.assertIn(("A.5", "Attestation"), keys)
        self.assertIn(("A.5.1", "CustodyModelTransitionPayload"), keys)
        self.assertIn(("A.5.2", "DisclosureProfileTransitionPayload"), keys)
        # Block text includes the declaration opener.
        custody = blocks[("A.5.1", "CustodyModelTransitionPayload")]
        self.assertIn("transition_id", custody)


if __name__ == "__main__":
    unittest.main()
