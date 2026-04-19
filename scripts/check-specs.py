#!/usr/bin/env python3
"""Lint normalized Trellis specification documents."""

from __future__ import annotations

import ast
import os
import re
import sys
from pathlib import Path


ROOT = Path(os.environ.get("TRELLIS_LINT_ROOT", Path(__file__).resolve().parents[1]))
SPECS = ROOT / "specs"
FIXTURES = ROOT / "fixtures" / "vectors"

TOP_LEVEL_SPECS = [
    SPECS / "trellis-agreement.md",
    SPECS / "trellis-core.md",
    SPECS / "trellis-operational-companion.md",
    SPECS / "trellis-requirements-matrix.md",
    SPECS / "cross-reference-map.md",
    SPECS / "README.md",
]

FORBIDDEN_PATTERNS = [
    (re.compile(r"signature field zeroed", re.IGNORECASE), "custom signature zero-fill prose"),
    (re.compile(r"zeroed to (?:a )?fixed-length", re.IGNORECASE), "custom signature zero-fill prose"),
    (re.compile(r"JSON Canonicalization Scheme|RFC 8785", re.IGNORECASE), "JCS canonicalization reference"),
    (re.compile(r"Trellis Core v0\.1"), "stale Core version"),
    (re.compile(r"Trellis Operational Companion v0\.1|Operational Companion v0\.1"), "stale Companion version"),
    (re.compile(r"\bforthcoming\b", re.IGNORECASE), "forthcoming companion language"),
    (re.compile(r"three spec documents", re.IGNORECASE), "old document-count language"),
    (re.compile(r"specs/(?:core|trust|export|projection|operations|forms|workflow|assurance)/"), "unarchived superseded spec path"),
]

PROFILE_ALLOWED_CONTEXT = re.compile(
    r"Profile A/B/C|Profile [A-F]|Profile-Namespace|Profile\" namespace|"
    r"Canonical CBOR profile|CBOR profile|encoding profile|signature profile|signing profile|"
    r"Core profile|Offline profile|Reader-Held profile|Delegated-Compute profile|"
    r"Disclosure profile|User-Held profile|Respondent-History profile|"
    r"legacy .*Profile|formerly \"Profile|renamed \"Conformance Classes\"|not profiles|profile\" letter|profile letter|"
    r"profile identifier",
    re.IGNORECASE,
)

PHASE_1_INVARIANTS = set(range(1, 16))  # #1..#15

GENERATOR_ALLOWED_IMPORTS = set(sys.stdlib_module_names) | {"cryptography", "cbor2"}

# Recognized vector operation directories under fixtures/vectors/. Wave-1
# extends the original four ops with `projection` and `shred` to support
# O-3 projection conformance vectors.
VECTOR_OPS = ("append", "verify", "export", "tamper", "projection", "shred")


def parse_invariants_cell(cell: str) -> set[int]:
    """Parse an Invariant-column cell. Handles '#5', '#1, #4', '1', '—'/'-' → empty."""
    result: set[int] = set()
    for m in re.finditer(r"#?(\d+)", cell):
        result.add(int(m.group(1)))
    return result


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def line_for(text: str, index: int) -> int:
    return text.count("\n", 0, index) + 1


def core_headings() -> dict[int, str]:
    headings: dict[int, str] = {}
    heading_pattern = re.compile(r"^## ([0-9]+)\.?\s+(.+)$", re.MULTILINE)
    for match in heading_pattern.finditer(read(SPECS / "trellis-core.md")):
        headings[int(match.group(1))] = match.group(2).strip()
    return headings


def companion_headings() -> dict[int, str]:
    """Mirror of core_headings() for trellis-operational-companion.md.

    Phase-1 Companion numbers `## N. Title` headings from §5 through §29.
    Higher-numbered Phase-2+ sections, when added, will slot in naturally.
    Appendix headings (`## A.N`) use alphabetic prefixes and are handled by
    companion_cddl_blocks() separately.
    """
    headings: dict[int, str] = {}
    heading_pattern = re.compile(r"^## ([0-9]+)\.?\s+(.+)$", re.MULTILINE)
    for match in heading_pattern.finditer(read(SPECS / "trellis-operational-companion.md")):
        headings[int(match.group(1))] = match.group(2).strip()
    return headings


def matrix_ids() -> list[str]:
    text = read(SPECS / "trellis-requirements-matrix.md")
    return re.findall(r"^\| (TR-(?:CORE|OP)-[0-9]{3}) \|", text, re.MULTILINE)


def tr_core_ids() -> list[str]:
    """Subset of matrix_ids() restricted to core-scope rows."""
    return [row_id for row_id in matrix_ids() if row_id.startswith("TR-CORE-")]


def tr_op_ids() -> list[str]:
    """Subset of matrix_ids() restricted to operational-scope rows."""
    return [row_id for row_id in matrix_ids() if row_id.startswith("TR-OP-")]


def matrix_rows() -> list[dict]:
    """Return parsed matrix rows.

    Matrix columns: Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes.
    `requirement`, `rationale`, and `notes` are exposed so spec-cross-ref row
    resolution (R6) can scan them for `Core §N` / `Companion §N` citations.
    """
    text = read(SPECS / "trellis-requirements-matrix.md")
    row_pattern = re.compile(r"^\| (TR-(?:CORE|OP)-[0-9]{3}) \|(.+)$", re.MULTILINE)
    rows = []
    for m in row_pattern.finditer(text):
        row_id = m.group(1)
        cells = [c.strip() for c in m.group(2).split("|")]
        rows.append({
            "id": row_id,
            "invariant": cells[1] if len(cells) > 1 else "—",
            "requirement": cells[2] if len(cells) > 2 else "",
            "rationale": cells[3] if len(cells) > 3 else "",
            "verification": cells[4] if len(cells) > 4 else "",
            "notes": cells[6] if len(cells) > 6 else "",
        })
    return rows


def testable_row_ids() -> set[str]:
    """Return IDs of matrix rows where Verification contains 'test-vector'."""
    return {r["id"] for r in matrix_rows() if "test-vector" in r["verification"]}


def projection_rebuild_drill_row_ids() -> set[str]:
    """Return TR-OP rows where Verification contains 'projection-rebuild-drill'."""
    return {
        r["id"]
        for r in matrix_rows()
        if r["id"].startswith("TR-OP-")
        and "projection-rebuild-drill" in r["verification"]
    }


def vector_manifests() -> list[tuple[Path, dict]]:
    """Return (manifest_path, parsed_toml) for every vector manifest under fixtures/vectors/."""
    import tomllib

    manifests = []
    if not FIXTURES.exists():
        return manifests
    for op_dir in VECTOR_OPS:
        op_path = FIXTURES / op_dir
        if not op_path.exists():
            continue
        for vector_dir in sorted(op_path.iterdir()):
            if not vector_dir.is_dir():
                continue
            manifest_path = vector_dir / "manifest.toml"
            if not manifest_path.exists():
                continue
            with manifest_path.open("rb") as f:
                manifests.append((manifest_path, tomllib.load(f)))
    return manifests


def manifest_op_from_path(manifest_path: Path) -> str | None:
    """Return the fixtures/vectors/<op>/ segment for a manifest path."""
    try:
        rel = manifest_path.parent.relative_to(FIXTURES)
    except ValueError:
        return None
    if not rel.parts:
        return None
    return rel.parts[0]


def derived_sections_for_tr_core(row_ids: list[str]) -> set[str]:
    """Scan Core prose to find which §N heading each TR-CORE-XXX anchor lives under."""
    core_text = read(SPECS / "trellis-core.md")
    # Matches headings like "## 6. Event Format" or "## §6 Event Format" or "### 6.2 Foo"
    heading_pattern = re.compile(r"^(#{2,3})\s+(?:§\s*)?([0-9]+(?:\.[0-9]+)*)\.?\s+(.+)$", re.MULTILINE)
    sections: list[tuple[int, str]] = []
    for m in heading_pattern.finditer(core_text):
        sections.append((m.start(), f"§{m.group(2)}"))
    derived: set[str] = set()
    for row_id in row_ids:
        anchor = core_text.find(row_id)
        if anchor == -1:
            continue
        current_section = None
        for start, label in sections:
            if start <= anchor:
                current_section = label
            else:
                break
        if current_section:
            derived.add(current_section)
    return derived


def derived_companion_sections_for_tr_op(
    row_ids: list[str], *, text: str | None = None
) -> set[str]:
    """Mirror of derived_sections_for_tr_core over the Operational Companion.

    Scans Companion prose to find which §N heading each TR-OP-XXX anchor
    lives under. Used by the declared-coverage round-trip rule (R5).

    `text` is exposed for tests that want to pass a synthetic companion
    document; real callers pass None and we read the canonical file.
    """
    companion_text = text if text is not None else read(SPECS / "trellis-operational-companion.md")
    # Strip the traceability appendix before scanning: it lists every TR-OP row
    # by ID, so leaving it in would make every TR-OP resolve to that appendix's
    # heading (drowning the real prose anchor). We tolerate label-text drift in
    # the appendix title — as long as the appendix still starts with a top-level
    # `## <letter>. Traceability` heading we'll find it.
    traceability_appendix = re.search(
        r"^## [A-Z]\.\s+Traceability\b", companion_text, re.MULTILINE
    )
    if traceability_appendix:
        companion_text = companion_text[: traceability_appendix.start()]
    heading_pattern = re.compile(
        r"^(#{2,4})\s+(?:§\s*)?([0-9]+(?:\.[0-9]+)*|[A-Z]\.[0-9]+(?:\.[0-9]+)*)\.?\s+(.+)$",
        re.MULTILINE,
    )
    sections: list[tuple[int, str]] = []
    for m in heading_pattern.finditer(companion_text):
        sections.append((m.start(), f"§{m.group(2)}"))
    derived: set[str] = set()
    for row_id in row_ids:
        for anchor in (m.start() for m in re.finditer(re.escape(row_id), companion_text)):
            current_section = None
            for start, label in sections:
                if start <= anchor:
                    current_section = label
                else:
                    break
            if current_section:
                derived.add(current_section)
    return derived


def derived_invariants_for_tr_core(row_ids: list[str]) -> set[int]:
    """Return the set of invariant numbers declared for the given TR-CORE rows in the matrix."""
    derived: set[int] = set()
    row_id_set = set(row_ids)
    for r in matrix_rows():
        if r["id"] in row_id_set:
            derived.update(parse_invariants_cell(r["invariant"]))
    return derived


def load_allowlist(
    path: Path,
    errors: list[str],
    *,
    int_field: str | None = None,
    str_field: str | None = None,
) -> dict[str, set]:
    """Generic TOML allowlist loader.

    One code path handles both `_pending-invariants.toml` (int list + str
    list) and `_pending-projection-drills.toml` (str list only). Missing
    file → empty sets, no error. Malformed TOML → single clean lint error,
    empty sets. Wrong element type → per-entry error, offending entry
    skipped.

    Callers pick which fields to extract by naming them. Returns a dict
    keyed by whichever of `int_field` / `str_field` the caller supplied;
    every requested key is present with at least an empty set.
    """
    import tomllib

    result: dict[str, set] = {}
    if int_field is not None:
        result[int_field] = set()
    if str_field is not None:
        result[str_field] = set()

    if not path.exists():
        return result

    try:
        rel = path.relative_to(ROOT)
    except ValueError:
        # Caller passed an absolute path outside ROOT (e.g., a test tmp file);
        # surface it as-is.
        rel = path

    try:
        with path.open("rb") as f:
            data = tomllib.load(f)
    except tomllib.TOMLDecodeError as e:
        errors.append(f"{rel}: malformed TOML ({e})")
        return result

    if int_field is not None:
        for entry in data.get(int_field, []):
            if isinstance(entry, bool) or not isinstance(entry, int):
                errors.append(
                    f"{rel}: {int_field} entry {entry!r} is not an integer"
                )
                continue
            result[int_field].add(entry)

    if str_field is not None:
        for entry in data.get(str_field, []):
            if not isinstance(entry, str):
                errors.append(
                    f"{rel}: {str_field} entry {entry!r} is not a string"
                )
                continue
            result[str_field].add(entry)

    return result


def load_pending_projection_drills(
    errors: list[str], *, path: Path | None = None
) -> set[str]:
    """Load the pending-projection-drills allowlist.

    Parallel to load_pending_invariants but for matrix rows whose
    Verification column is `projection-rebuild-drill` rather than
    `test-vector`. Schema:

        pending_matrix_rows = ["TR-OP-008", ...]

    `path` is exposed for tests; real callers pass None and use the
    canonical location under fixtures/vectors/.

    Consumed by `check_projection_rebuild_drill_coverage` (R7), which
    uses this allowlist to suppress uncovered-drill errors while a
    drill fixture is still being authored.
    """
    target = path if path is not None else (FIXTURES / "_pending-projection-drills.toml")
    data = load_allowlist(target, errors, str_field="pending_matrix_rows")
    return data["pending_matrix_rows"]


def load_pending_model_checks(
    errors: list[str], *, path: Path | None = None
) -> set[str]:
    """Load the pending-model-checks allowlist (R8).

    Parallel to load_pending_projection_drills but for matrix rows whose
    Verification column is `model-check`. The G-2 audit-paths brief pins
    model-check evidence to the G-4 Rust conformance crate (not yet
    landed), so every current model-check row is expected to sit in this
    allowlist until G-4 ships. Schema:

        pending_matrix_rows = ["TR-CORE-020", ...]

    Consumed by `check_model_check_evidence` (R8). `path` is exposed for
    tests; real callers pass None.
    """
    target = path if path is not None else (FIXTURES / "_pending-model-checks.toml")
    data = load_allowlist(target, errors, str_field="pending_matrix_rows")
    return data["pending_matrix_rows"]


def load_pending_invariants(errors: list[str]) -> tuple[set[int], set[str]]:
    """Load the pending-invariants allowlist (F5).

    The allowlist replaces the old TRELLIS_SKIP_COVERAGE=1 blanket bypass.
    Listed-but-now-covered entries are errors (forces cleanup). Entries that
    are pending AND uncovered are allowed. Missing file = empty allowlist.

    Schema (all fields optional):

        pending_invariants  = [3, 6, 7]          # Phase-1 invariant numbers
        pending_matrix_rows = ["TR-CORE-037"]    # testable matrix row IDs
                                                 # (both TR-CORE-* and TR-OP-*)
    """
    data = load_allowlist(
        FIXTURES / "_pending-invariants.toml",
        errors,
        int_field="pending_invariants",
        str_field="pending_matrix_rows",
    )
    return data["pending_invariants"], data["pending_matrix_rows"]


# Manifest keys whose string values are hex digests, not filesystem paths.
# Extend this list if the schema grows new non-path string fields.
MANIFEST_NON_PATH_STRING_KEYS = {"zip_sha256"}

# Manifest sub-tables that carry inline structured data (not sibling-file
# references). Per the G-3 fixture-system design, small structured outputs
# stay in the manifest; byte outputs go to sibling files. When the walker
# encounters a sub-table named here, it does NOT recurse into it — every
# string inside is opaque data (scopes, enum values, hex digests, URNs,
# or similar), not a path to be resolved.
MANIFEST_STRUCTURED_DATA_TABLES = {
    "report",            # verify/tamper: [expected.report] — booleans + failure codes
    "watermark_fields",  # projection: [expected.watermark_fields] — Watermark CDDL field values
    "cascade_report",    # shred: [expected.cascade_report] — A.7 class → post-state map
}


def _iter_manifest_path_strings(table: dict, path_stack: tuple[str, ...] = ()):
    """Yield (dotted_key, value) for every string value in a manifest table.

    Recurses into nested tables (e.g. [expected.report] in verify manifests)
    AND into lists of strings (e.g. [inputs] payloads = ["a.bin", "b.bin"]).
    Skips non-string values (booleans, ints), keys explicitly listed as
    non-path string fields (e.g. zip_sha256), and sub-tables listed as
    structured-data containers (e.g. watermark_fields).
    """
    for key, value in table.items():
        if isinstance(value, dict):
            if key in MANIFEST_STRUCTURED_DATA_TABLES:
                continue
            yield from _iter_manifest_path_strings(value, path_stack + (key,))
        elif isinstance(value, list):
            if key in MANIFEST_NON_PATH_STRING_KEYS:
                continue
            for index, element in enumerate(value):
                if isinstance(element, str):
                    dotted = ".".join(path_stack + (f"{key}[{index}]",))
                    yield (dotted, element)
        elif isinstance(value, str):
            if key in MANIFEST_NON_PATH_STRING_KEYS:
                continue
            yield (".".join(path_stack + (key,)), value)


VECTOR_NAMING_PATTERN = re.compile(r"^[0-9]{3}-[a-z0-9]+(?:-[a-z0-9]+)*$")


def check_vector_naming(errors: list[str]) -> None:
    """R1 — every vector directory MUST be named `NNN-slug`.

    `NNN` is exactly three digits; `slug` is one-or-more dash-separated
    segments of lowercase-alphanumeric characters. Trailing / leading
    dashes, uppercase, short numeric prefixes, and missing dashes are all
    rejected.

    The walker only considers directories directly under
    fixtures/vectors/<op>/ where <op> is one of VECTOR_OPS. Support dirs
    like `_generator` live at the op level or above, so they are not in
    this rule's scope.
    """
    if not FIXTURES.exists():
        return
    for op in VECTOR_OPS:
        op_path = FIXTURES / op
        if not op_path.exists():
            continue
        for entry in sorted(op_path.iterdir()):
            if not entry.is_dir():
                continue
            if not VECTOR_NAMING_PATTERN.match(entry.name):
                errors.append(
                    f"{entry.relative_to(ROOT)}: vector directory name "
                    f"{entry.name!r} does not match the required "
                    f"`NNN-slug` naming convention "
                    f"(3-digit prefix, dash, lowercase-alphanumeric slug)"
                )


def check_vector_manifest_paths(errors: list[str]) -> None:
    """A/F7 — every string in [inputs] / [expected] must resolve to a file.

    Paths are relative to the vector directory. Sibling paths and
    ``../../_keys/…`` / ``../../_inputs/…`` both resolve here.
    """
    for manifest_path, manifest in vector_manifests():
        vector_dir = manifest_path.parent
        rel = manifest_path.relative_to(ROOT)
        for section in ("inputs", "expected"):
            section_data = manifest.get(section, {})
            if not isinstance(section_data, dict):
                continue
            for dotted_key, value in _iter_manifest_path_strings(
                section_data, (section,)
            ):
                if value == "":
                    errors.append(
                        f"{rel}: {dotted_key} is empty; "
                        f"manifest path values must be non-empty relative paths"
                    )
                    continue
                if Path(value).is_absolute():
                    errors.append(
                        f"{rel}: {dotted_key}='{value}' is absolute; "
                        f"manifest path values must be relative to the vector directory"
                    )
                    continue
                resolved = (vector_dir / value).resolve()
                if not resolved.exists():
                    errors.append(
                        f"{rel}: "
                        f"{dotted_key}='{value}' does not exist "
                        f"(resolved to {resolved})"
                    )


def check_vector_manifest_identity(errors: list[str]) -> None:
    """Validate that manifest identity fields match the vector directory."""
    for manifest_path, manifest in vector_manifests():
        rel = manifest_path.relative_to(ROOT)
        path_op = manifest_op_from_path(manifest_path)
        if path_op is None:
            continue

        declared_op = manifest.get("op")
        if declared_op != path_op:
            errors.append(
                f"{rel}: manifest op={declared_op!r} does not match "
                f"directory op={path_op!r}"
            )

        declared_id = manifest.get("id")
        expected_id = f"{path_op}/{manifest_path.parent.name}"
        if declared_id != expected_id:
            errors.append(
                f"{rel}: manifest id={declared_id!r} does not match "
                f"directory id={expected_id!r}"
            )


def check_vector_manifest_coverage_ids(errors: list[str]) -> None:
    """Validate manifest coverage row IDs before coverage accounting."""
    known_tr_core = set(tr_core_ids())
    known_tr_op = set(tr_op_ids())
    for manifest_path, manifest in vector_manifests():
        rel = manifest_path.relative_to(ROOT)
        coverage = manifest.get("coverage", {})

        for row_id in coverage.get("tr_core", []):
            if not isinstance(row_id, str) or row_id not in known_tr_core:
                errors.append(
                    f"{rel}: coverage.tr_core entry {row_id!r} is not a known "
                    f"TR-CORE matrix row ID"
                )

        for row_id in coverage.get("tr_op", []):
            if not isinstance(row_id, str) or row_id not in known_tr_op:
                errors.append(
                    f"{rel}: coverage.tr_op entry {row_id!r} is not a known "
                    f"TR-OP matrix row ID"
                )


def check_vector_declared_coverage(errors: list[str], warnings: list[str]) -> None:
    for path, manifest in vector_manifests():
        coverage = manifest.get("coverage", {})
        tr_core = coverage.get("tr_core", [])
        tr_op = coverage.get("tr_op", [])
        declared_sections = set(coverage.get("core_sections", [])) if "core_sections" in coverage else None
        declared_companion_sections = (
            set(coverage.get("companion_sections", []))
            if "companion_sections" in coverage
            else None
        )
        declared_invariants = set(coverage.get("invariants", [])) if "invariants" in coverage else None

        if declared_sections is not None:
            derived = derived_sections_for_tr_core(tr_core)
            if declared_sections != derived:
                errors.append(
                    f"{path.relative_to(ROOT)}: declared core_sections={sorted(declared_sections)} "
                    f"does not equal matrix-derived={sorted(derived)}"
                )
        # R5 — same round-trip rule for the Operational Companion. `companion_sections`,
        # when declared, MUST equal the set derived from the manifest's tr_op rows.
        if declared_companion_sections is not None:
            derived_companion = derived_companion_sections_for_tr_op(tr_op)
            if declared_companion_sections != derived_companion:
                errors.append(
                    f"{path.relative_to(ROOT)}: declared companion_sections="
                    f"{sorted(declared_companion_sections)} does not equal "
                    f"matrix-derived={sorted(derived_companion)}"
                )
        # Per amended design F1: invariants is commentary-only. Mismatch is a
        # warning (non-fatal), not an error. Matrix rows with Invariant=— make
        # bidirectional enforcement incoherent; tr_core is the canonical anchor.
        if declared_invariants is not None:
            derived = derived_invariants_for_tr_core(tr_core)
            if declared_invariants != derived:
                warnings.append(
                    f"{path.relative_to(ROOT)}: declared invariants={sorted(declared_invariants)} "
                    f"does not equal matrix-derived={sorted(derived)} (commentary only)"
                )


def check_invariant_coverage(errors: list[str], pending_invariants: set[int]) -> None:
    rows = matrix_rows()
    testable_by_invariant: dict[int, list[str]] = {}
    for r in rows:
        if "test-vector" not in r["verification"]:
            continue
        for inv in parse_invariants_cell(r["invariant"]):
            testable_by_invariant.setdefault(inv, []).append(r["id"])

    covered_ids: set[str] = set()
    for path, manifest in vector_manifests():
        if _is_deprecated_vector(manifest):
            continue  # F6 — deprecated vectors are excluded from audits
        coverage = manifest.get("coverage", {})
        covered_ids.update(coverage.get("tr_core", []))
        covered_ids.update(coverage.get("tr_op", []))

    # Per amended design F2: narrowed rule. Only invariants that have ≥1
    # matrix row with Verification=test-vector are audited here (the byte-
    # testable subset). Invariants without any test-vector row are handled
    # via the separate G-2 non-byte-testable audit path (model-check,
    # declaration-doc-check, spec-cross-ref, etc.) and are NOT flagged here.
    covered_invariants: set[int] = set()
    for inv in sorted(testable_by_invariant.keys()):
        testable_rows = testable_by_invariant[inv]
        is_covered = any(rid in covered_ids for rid in testable_rows)
        if is_covered:
            covered_invariants.add(inv)
            continue
        if inv in pending_invariants:
            continue  # pending-and-uncovered is allowed
        errors.append(
            f"specs/trellis-requirements-matrix.md: invariant #{inv} has no "
            f"vector via any of its testable rows {testable_rows}"
        )

    # F5 — listed-but-now-covered forces allowlist cleanup.
    for inv in sorted(pending_invariants & covered_invariants):
        errors.append(
            f"fixtures/vectors/_pending-invariants.toml: invariant #{inv} is "
            f"listed as pending but is now covered by a vector; remove it from "
            f"pending_invariants"
        )


def check_generator_imports(errors: list[str]) -> None:
    gen_dir = FIXTURES / "_generator"
    if not gen_dir.exists():
        return
    for py_file in sorted(gen_dir.rglob("*.py")):
        try:
            tree = ast.parse(py_file.read_text(encoding="utf-8"))
        except SyntaxError as e:
            errors.append(f"{py_file.relative_to(ROOT)}: syntax error at line {e.lineno}")
            continue
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                for alias in node.names:
                    top = alias.name.split(".")[0]
                    if top not in GENERATOR_ALLOWED_IMPORTS:
                        errors.append(
                            f"{py_file.relative_to(ROOT)}:{node.lineno}: forbidden import "
                            f"'{alias.name}' (allowed top-levels: {sorted(GENERATOR_ALLOWED_IMPORTS)})"
                        )
            elif isinstance(node, ast.ImportFrom):
                if node.level > 0:
                    errors.append(
                        f"{py_file.relative_to(ROOT)}:{node.lineno}: relative imports forbidden in _generator/ "
                        f"(level={node.level})"
                    )
                    continue
                top = (node.module or "").split(".")[0]
                if top and top not in GENERATOR_ALLOWED_IMPORTS:
                    errors.append(
                        f"{py_file.relative_to(ROOT)}:{node.lineno}: forbidden import "
                        f"'from {node.module}' (allowed top-levels: {sorted(GENERATOR_ALLOWED_IMPORTS)})"
                    )


ALLOWED_VECTOR_STATUSES = {"active", "deprecated"}
ISO_DATE_PATTERN = re.compile(r"^\d{4}-\d{2}-\d{2}$")


def _is_deprecated_vector(manifest: dict) -> bool:
    """Return True iff the manifest declares a 'deprecated' lifecycle status."""
    return manifest.get("status") == "deprecated"


def check_vector_lifecycle_fields(errors: list[str]) -> None:
    """F6 — validate manifest-level `status` and `deprecated_at` fields.

    Rules:
      * `status` is optional; when present, MUST be "active" or "deprecated".
      * `deprecated_at` is required iff `status = "deprecated"`.
      * `deprecated_at` MUST be an ISO-8601 date string (YYYY-MM-DD).
      * `deprecated_at` without `status = "deprecated"` is permitted but noisy;
         the design doc does not forbid it, so this lint does not flag it.
    """
    from datetime import date

    for manifest_path, manifest in vector_manifests():
        rel = manifest_path.relative_to(ROOT)
        status = manifest.get("status")
        if status is not None:
            if not isinstance(status, str) or status not in ALLOWED_VECTOR_STATUSES:
                errors.append(
                    f"{rel}: status={status!r} is not one of "
                    f"{sorted(ALLOWED_VECTOR_STATUSES)}"
                )
                continue  # can't reason about deprecated_at when status is bogus

        if status != "deprecated":
            continue

        deprecated_at = manifest.get("deprecated_at")
        if deprecated_at is None:
            errors.append(
                f"{rel}: status='deprecated' requires a deprecated_at "
                f"ISO-8601 date (YYYY-MM-DD)"
            )
            continue
        if not isinstance(deprecated_at, str) or not ISO_DATE_PATTERN.match(
            deprecated_at
        ):
            errors.append(
                f"{rel}: deprecated_at={deprecated_at!r} is not a valid "
                f"ISO-8601 date (YYYY-MM-DD)"
            )
            continue
        try:
            date.fromisoformat(deprecated_at)
        except ValueError:
            errors.append(
                f"{rel}: deprecated_at={deprecated_at!r} is not a valid "
                f"ISO-8601 date (YYYY-MM-DD)"
            )


def check_vector_coverage_prefixes(errors: list[str]) -> None:
    """R8 — coverage-bucket prefix discipline.

    `coverage.tr_core` entries MUST start with `TR-CORE-` and `coverage.tr_op`
    entries MUST start with `TR-OP-`. Misfiling an ID into the wrong bucket
    is otherwise silently ignored by R4/R5: a TR-OP row placed under
    `coverage.tr_core` would never land in the TR-OP coverage set and would
    create false-green lint on the TR-OP side. Running this ahead of R4/R5
    surfaces the mistake with a clear diagnostic instead.
    """
    for manifest_path, manifest in vector_manifests():
        if _is_deprecated_vector(manifest):
            continue  # F6 — deprecated vectors are excluded from audits
        rel = manifest_path.relative_to(ROOT)
        coverage = manifest.get("coverage", {})
        for row_id in coverage.get("tr_core", []):
            if not isinstance(row_id, str) or not row_id.startswith("TR-CORE-"):
                errors.append(
                    f"{rel}: coverage.tr_core entry {row_id!r} is not a "
                    f"TR-CORE-* id; misfiled IDs are silently ignored by the "
                    f"bucket-audit rules"
                )
        for row_id in coverage.get("tr_op", []):
            if not isinstance(row_id, str) or not row_id.startswith("TR-OP-"):
                errors.append(
                    f"{rel}: coverage.tr_op entry {row_id!r} is not a "
                    f"TR-OP-* id; misfiled IDs are silently ignored by the "
                    f"bucket-audit rules"
                )


def check_vector_coverage(errors: list[str], pending_matrix_rows: set[str]) -> None:
    testable = {row_id for row_id in testable_row_ids() if row_id.startswith("TR-CORE-")}
    covered: set[str] = set()
    for _path, manifest in vector_manifests():
        if _is_deprecated_vector(manifest):
            continue  # F6 — deprecated vectors are excluded from audits
        covered.update(manifest.get("coverage", {}).get("tr_core", []))
    for row_id in sorted(testable - covered):
        if row_id in pending_matrix_rows:
            continue  # pending-and-uncovered is allowed
        errors.append(
            f"specs/trellis-requirements-matrix.md: no vector covers {row_id} "
            f"(row has Verification=test-vector but no fixtures/vectors/*/manifest.toml "
            f"references it in coverage.tr_core)"
        )

    # F5 — listed-but-now-covered forces allowlist cleanup.
    for row_id in sorted(pending_matrix_rows & covered):
        errors.append(
            f"fixtures/vectors/_pending-invariants.toml: {row_id} is listed in "
            f"pending_matrix_rows but is now covered by a vector; remove it"
        )
    # Unknown row IDs in the allowlist are suspicious — warn via errors.
    known_ids = set(matrix_ids())
    for row_id in sorted(pending_matrix_rows - known_ids):
        errors.append(
            f"fixtures/vectors/_pending-invariants.toml: {row_id} is not a "
            f"matrix row ID; remove it from pending_matrix_rows"
        )


def check_tr_op_coverage(errors: list[str], pending_matrix_rows: set[str]) -> None:
    """R5 — mirror of check_vector_coverage for TR-OP rows.

    A TR-OP row with `Verification=test-vector` is covered when ≥1
    non-deprecated manifest lists it under `coverage.tr_op`. The
    `pending_matrix_rows` allowlist is shared with TR-CORE (both prefixes
    live in the same list); the unknown-ID audit is handled by
    check_vector_coverage and not repeated here.
    """
    testable = {row_id for row_id in testable_row_ids() if row_id.startswith("TR-OP-")}
    covered: set[str] = set()
    for _path, manifest in vector_manifests():
        if _is_deprecated_vector(manifest):
            continue  # F6 — deprecated vectors are excluded from audits
        covered.update(manifest.get("coverage", {}).get("tr_op", []))
    for row_id in sorted(testable - covered):
        if row_id in pending_matrix_rows:
            continue  # pending-and-uncovered is allowed
        errors.append(
            f"specs/trellis-requirements-matrix.md: no vector covers {row_id} "
            f"(row has Verification=test-vector but no fixtures/vectors/*/manifest.toml "
            f"references it in coverage.tr_op)"
        )

    # F5 — listed-but-now-covered forces allowlist cleanup.
    for row_id in sorted(pending_matrix_rows & covered):
        errors.append(
            f"fixtures/vectors/_pending-invariants.toml: {row_id} is listed in "
            f"pending_matrix_rows but is now covered by a vector; remove it"
        )


def check_projection_rebuild_drill_coverage(
    errors: list[str], pending_matrix_rows: set[str]
) -> None:
    """R7 — projection-rebuild-drill rows need projection/shred fixtures.

    A TR-OP row with `Verification=projection-rebuild-drill` is covered when
    at least one non-deprecated manifest under `projection/` or `shred/` lists
    the row in `coverage.tr_op`. `_pending-projection-drills.toml` is the
    narrow allowlist for rows whose drill fixture has not landed yet.
    """
    drill_rows = projection_rebuild_drill_row_ids()
    covered: set[str] = set()
    for manifest_path, manifest in vector_manifests():
        if _is_deprecated_vector(manifest):
            continue
        op = manifest_op_from_path(manifest_path)
        if op not in {"projection", "shred"}:
            continue
        covered.update(
            row_id
            for row_id in manifest.get("coverage", {}).get("tr_op", [])
            if row_id in drill_rows
        )

    for row_id in sorted(drill_rows - covered):
        if row_id in pending_matrix_rows:
            continue
        errors.append(
            f"specs/trellis-requirements-matrix.md: no projection rebuild drill "
            f"covers {row_id} (row has Verification=projection-rebuild-drill "
            f"but no projection/shred manifest references it in coverage.tr_op)"
        )

    for row_id in sorted(pending_matrix_rows & covered):
        errors.append(
            f"fixtures/vectors/_pending-projection-drills.toml: {row_id} is "
            f"listed in pending_matrix_rows but is now covered by a projection "
            f"or shred fixture; remove it"
        )

    known_ids = set(matrix_ids())
    for row_id in sorted(pending_matrix_rows - known_ids):
        errors.append(
            f"fixtures/vectors/_pending-projection-drills.toml: {row_id} is "
            f"not a matrix row ID; remove it from pending_matrix_rows"
        )
    for row_id in sorted((pending_matrix_rows & known_ids) - drill_rows):
        errors.append(
            f"fixtures/vectors/_pending-projection-drills.toml: {row_id} is "
            f"not a TR-OP row with Verification=projection-rebuild-drill; "
            f"remove it from pending_matrix_rows"
        )


_SPEC_CITE_PATTERN = re.compile(
    r"\b(Core|Companion)\s+§([0-9]+(?:\.[0-9]+)*)"
)


def spec_cross_ref_row_ids() -> set[str]:
    """Matrix rows where Verification contains `spec-cross-ref`."""
    return {r["id"] for r in matrix_rows() if "spec-cross-ref" in r["verification"]}


def model_check_row_ids() -> set[str]:
    """Matrix rows where Verification contains `model-check`."""
    return {r["id"] for r in matrix_rows() if "model-check" in r["verification"]}


def check_spec_cross_ref_rows(errors: list[str], warnings: list[str]) -> None:
    """R6 — spec-cross-ref rows must cite resolvable §N headings.

    For every matrix row with `Verification=spec-cross-ref`, scan the
    Requirement / Rationale / Notes cells for `Core §N` or `Companion §N`
    citations. Each cited §N MUST resolve to a top-level heading in the
    named spec; a non-resolving cite is a hard error.

    Rows that declare `spec-cross-ref` but carry no `Core §N` / `Companion §N`
    citation in any of the three text cells are warned, not errored. Many
    existing rows rely on prose anchors that do not use this exact phrasing;
    forcing a hard error would require annotating ~25 rows that are out of
    scope for a lint-only refactor. The warning still surfaces the gap on
    every run so new rows land with cites from day one.
    """
    core_section_numbers = {str(n) for n in core_headings().keys()}
    # Companion section numbers include `A.5.1` / `A.5` style Appendix ids
    # whenever the heading scanner picked them up. Build a permissive set of
    # "any heading anchor number we recognise" for Companion so Appendix
    # cites resolve the same way top-level `## 10.` headings do.
    companion_section_numbers = {str(n) for n in companion_headings().keys()}
    appendix_pattern = re.compile(
        r"^#{2,4}\s+([A-Z]\.[0-9]+(?:\.[0-9]+)*)\b", re.MULTILINE
    )
    companion_text = read(SPECS / "trellis-operational-companion.md")
    for m in appendix_pattern.finditer(companion_text):
        companion_section_numbers.add(m.group(1))

    for row in matrix_rows():
        if "spec-cross-ref" not in row["verification"]:
            continue
        haystack = " ".join((row["requirement"], row["rationale"], row["notes"]))
        matches = list(_SPEC_CITE_PATTERN.finditer(haystack))
        if not matches:
            warnings.append(
                f"specs/trellis-requirements-matrix.md: {row['id']} declares "
                f"Verification=spec-cross-ref but cites no `Core §N` / "
                f"`Companion §N` heading in Requirement/Rationale/Notes"
            )
            continue
        for match in matches:
            spec_name, section = match.group(1), match.group(2)
            if spec_name == "Core":
                known = core_section_numbers
                spec_path = "specs/trellis-core.md"
            else:
                known = companion_section_numbers
                spec_path = "specs/trellis-operational-companion.md"
            # A cite like `Core §5.1` passes when either the top-level
            # `## 5.` heading exists OR any heading matches the full dotted
            # path (`### 5.1`). Split on dots and walk outward.
            if section in known:
                continue
            top = section.split(".", 1)[0]
            if top in known:
                continue
            errors.append(
                f"specs/trellis-requirements-matrix.md: {row['id']} cites "
                f"`{spec_name} §{section}` but no matching heading exists "
                f"in {spec_path}"
            )


def load_model_check_evidence(errors: list[str]) -> dict[str, str]:
    """Load thoughts/model-checks/evidence.toml.

    Schema:

        [evidence]
        "TR-CORE-020" = "thoughts/model-checks/tr-core-020/linear-order.tla"
        "TR-CORE-050" = "thoughts/model-checks/tr-core-050/idempotency.tla"

    Missing file → empty mapping, no error (expected until G-4's
    conformance crate lands). Malformed TOML → lint error.
    """
    import tomllib

    path = ROOT / "thoughts" / "model-checks" / "evidence.toml"
    if not path.exists():
        return {}
    try:
        rel = path.relative_to(ROOT)
    except ValueError:
        rel = path
    try:
        with path.open("rb") as f:
            data = tomllib.load(f)
    except tomllib.TOMLDecodeError as e:
        errors.append(f"{rel}: malformed TOML ({e})")
        return {}
    table = data.get("evidence", {})
    if not isinstance(table, dict):
        errors.append(f"{rel}: [evidence] must be a TOML table")
        return {}
    mapping: dict[str, str] = {}
    for row_id, artifact in table.items():
        if not isinstance(artifact, str):
            errors.append(
                f"{rel}: evidence[{row_id!r}]={artifact!r} is not a string path"
            )
            continue
        mapping[row_id] = artifact
    return mapping


def check_model_check_evidence(
    errors: list[str], pending_matrix_rows: set[str]
) -> None:
    """R8 — matrix rows with `Verification=model-check` need an evidence artifact.

    A row is satisfied when either:
      (a) `thoughts/model-checks/evidence.toml` names the row under
          `[evidence]` with a path that resolves relative to the repo root, or
      (b) the row is listed in `_pending-model-checks.toml`'s
          `pending_matrix_rows` (the narrow escape hatch for rows awaiting
          the G-4 Rust conformance crate).

    Allowlist hygiene: evidence-present AND listed-as-pending → error (forces
    cleanup). Unknown row IDs and non-model-check rows in the allowlist →
    error.
    """
    model_check_rows = model_check_row_ids()
    evidence = load_model_check_evidence(errors)

    for row_id in sorted(model_check_rows):
        if row_id in pending_matrix_rows:
            continue
        artifact = evidence.get(row_id)
        if artifact is None:
            errors.append(
                f"specs/trellis-requirements-matrix.md: no model-check "
                f"evidence for {row_id} (row has Verification=model-check "
                f"but thoughts/model-checks/evidence.toml has no entry and "
                f"the row is not in _pending-model-checks.toml)"
            )
            continue
        resolved = (ROOT / artifact).resolve()
        if not resolved.exists():
            errors.append(
                f"thoughts/model-checks/evidence.toml: "
                f"{row_id} evidence path {artifact!r} does not exist "
                f"(resolved to {resolved})"
            )

    for row_id in sorted(pending_matrix_rows & set(evidence.keys())):
        errors.append(
            f"fixtures/vectors/_pending-model-checks.toml: {row_id} is "
            f"listed in pending_matrix_rows but evidence is present in "
            f"thoughts/model-checks/evidence.toml; remove it"
        )

    known_ids = set(matrix_ids())
    for row_id in sorted(pending_matrix_rows - known_ids):
        errors.append(
            f"fixtures/vectors/_pending-model-checks.toml: {row_id} is "
            f"not a matrix row ID; remove it from pending_matrix_rows"
        )
    for row_id in sorted((pending_matrix_rows & known_ids) - model_check_rows):
        errors.append(
            f"fixtures/vectors/_pending-model-checks.toml: {row_id} is "
            f"not a TR-* row with Verification=model-check; remove it "
            f"from pending_matrix_rows"
        )


def check_forbidden_terms(errors: list[str]) -> None:
    for path in TOP_LEVEL_SPECS:
        text = read(path)
        for pattern, label in FORBIDDEN_PATTERNS:
            for match in pattern.finditer(text):
                errors.append(f"{path.relative_to(ROOT)}:{line_for(text, match.start())}: forbidden {label}")


def check_core_section_references(errors: list[str]) -> None:
    headings = core_headings()
    companion = SPECS / "trellis-operational-companion.md"
    text = read(companion)
    ref_pattern = re.compile(r"Core §([0-9]+)(?: \(([^)]+)\))?")
    for match in ref_pattern.finditer(text):
        number = int(match.group(1))
        label = match.group(2)
        heading = headings.get(number)
        line = line_for(text, match.start())
        if heading is None:
            errors.append(f"{companion.relative_to(ROOT)}:{line}: Core §{number} does not exist")
            continue
        if label is None:
            errors.append(
                f"{companion.relative_to(ROOT)}:{line}: Core §{number} must include heading label "
                f'`Core §{number} ({heading})`'
            )
            continue
        if label.strip().lower() != heading.lower():
            errors.append(
                f"{companion.relative_to(ROOT)}:{line}: Core §{number} label `{label}` does not match `{heading}`"
            )


def check_requirement_ids(errors: list[str]) -> None:
    ids = matrix_ids()
    seen: set[str] = set()
    for requirement_id in ids:
        if requirement_id in seen:
            errors.append(f"specs/trellis-requirements-matrix.md: duplicate requirement ID {requirement_id}")
        seen.add(requirement_id)

    known = set(ids)
    for path in TOP_LEVEL_SPECS:
        text = read(path)
        for match in re.finditer(r"\bTR-(?:CORE|OP)-[0-9]{3}\b", text):
            requirement_id = match.group(0)
            if requirement_id not in known:
                errors.append(f"{path.relative_to(ROOT)}:{line_for(text, match.start())}: unknown {requirement_id}")


def check_traceability_anchors(errors: list[str]) -> None:
    core_text = read(SPECS / "trellis-core.md")
    companion_text = read(SPECS / "trellis-operational-companion.md")
    for requirement_id in matrix_ids():
        if requirement_id.startswith("TR-CORE-") and requirement_id not in core_text:
            errors.append(f"specs/trellis-core.md: missing prose anchor for {requirement_id}")
        if requirement_id.startswith("TR-OP-") and requirement_id not in companion_text:
            errors.append(f"specs/trellis-operational-companion.md: missing prose anchor for {requirement_id}")


def check_bare_profile(errors: list[str]) -> None:
    for path in TOP_LEVEL_SPECS:
        text = read(path)
        for match in re.finditer(r"\b[Pp]rofile(?:s)?\b", text):
            line_start = text.rfind("\n", 0, match.start()) + 1
            line_end = text.find("\n", match.start())
            if line_end == -1:
                line_end = len(text)
            line = text[line_start:line_end]
            if PROFILE_ALLOWED_CONTEXT.search(line):
                continue
            if any(qualified in line for qualified in ("Posture", "Custody", "Conformance Class", "Trust-Profile")):
                continue
            errors.append(f"{path.relative_to(ROOT)}:{line_for(text, match.start())}: bare Profile/profile wording")


def check_archived_inputs(errors: list[str]) -> None:
    for family in ["core", "trust", "export", "projection", "operations", "forms", "workflow", "assurance"]:
        path = SPECS / family
        if path.exists():
            errors.append(f"{path.relative_to(ROOT)}: superseded spec family directory must be under specs/archive/")


def core_event_type_registry() -> dict[str, dict[str, str]]:
    """Parse Core §6.7's Extension Registration table.

    Returns a mapping from registered identifier to a dict with
    `container`, `phase`, and `purpose` fields. Used by the event-type
    registry rule (R9) once that wires up in a later commit.

    The table is a plain markdown 4-column table under `### 6.7 Extension
    Registration`; the header row and separator row are skipped.
    """
    core_text = read(SPECS / "trellis-core.md")
    section_match = re.search(r"^### 6\.7\b.*$", core_text, re.MULTILINE)
    if not section_match:
        return {}
    # Slice from §6.7 heading to the next same-or-higher heading.
    start = section_match.end()
    next_heading = re.search(r"^#{1,3}\s", core_text[start:], re.MULTILINE)
    end = start + next_heading.start() if next_heading else len(core_text)
    section = core_text[start:end]

    registry: dict[str, dict[str, str]] = {}
    row_pattern = re.compile(
        r"^\|\s*`([^`]+)`\s*\|\s*`([^`]+)`\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|\s*$",
        re.MULTILINE,
    )
    for m in row_pattern.finditer(section):
        container, identifier, phase, purpose = m.group(1, 2, 3, 4)
        # Skip header rows disguised as data (unlikely here, but defensive).
        if container.startswith("-") or identifier.startswith("-"):
            continue
        registry[identifier.strip()] = {
            "container": container.strip(),
            "phase": phase.strip(),
            "purpose": purpose.strip(),
        }
    return registry


_CDDL_RULE_NAME_PATTERN = re.compile(r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*=", re.MULTILINE)


def companion_cddl_blocks() -> dict[tuple[str, str], str]:
    """Extract ```cddl fenced blocks from the Operational Companion.

    Returns a mapping keyed by (appendix_id, rule_name) to the raw block
    body. `appendix_id` is the nearest enclosing `## A.N` / `### A.N.M`
    heading (e.g. "A.5", "A.5.1"). `rule_name` is the first CDDL rule
    name declared in the block (e.g. "CustodyModelTransitionPayload"), and
    must match the identifier pattern `[A-Za-z_][A-Za-z0-9_]*` followed by
    `=`.

    Used by the CDDL cross-ref rule (R10) once that wires up.
    """
    companion_text = read(SPECS / "trellis-operational-companion.md")
    heading_pattern = re.compile(
        r"^(?:#{2,4})\s+(A\.[0-9]+(?:\.[0-9]+)*)\b", re.MULTILINE
    )
    headings: list[tuple[int, str]] = [
        (m.start(), m.group(1)) for m in heading_pattern.finditer(companion_text)
    ]

    block_pattern = re.compile(r"^```cddl\s*\n(.*?)^```", re.MULTILINE | re.DOTALL)
    blocks: dict[tuple[str, str], str] = {}
    for m in block_pattern.finditer(companion_text):
        body = m.group(1)
        rule_match = _CDDL_RULE_NAME_PATTERN.search(body)
        if not rule_match:
            continue
        rule_name = rule_match.group(1)
        # Find the nearest heading at or before this block.
        appendix_id: str | None = None
        for start, label in headings:
            if start <= m.start():
                appendix_id = label
            else:
                break
        if appendix_id is None:
            continue
        blocks[(appendix_id, rule_name)] = body
    return blocks


def main() -> int:
    errors: list[str] = []
    warnings: list[str] = []
    pending_invariants, pending_matrix_rows = load_pending_invariants(errors)
    pending_projection_drills = load_pending_projection_drills(errors)
    pending_model_checks = load_pending_model_checks(errors)
    check_forbidden_terms(errors)
    check_core_section_references(errors)
    check_requirement_ids(errors)
    check_traceability_anchors(errors)
    check_bare_profile(errors)
    check_archived_inputs(errors)
    check_vector_naming(errors)
    check_vector_manifest_identity(errors)
    check_vector_manifest_coverage_ids(errors)
    check_vector_lifecycle_fields(errors)
    check_vector_coverage_prefixes(errors)
    check_vector_coverage(errors, pending_matrix_rows)
    check_tr_op_coverage(errors, pending_matrix_rows)
    check_projection_rebuild_drill_coverage(errors, pending_projection_drills)
    check_spec_cross_ref_rows(errors, warnings)
    check_model_check_evidence(errors, pending_model_checks)
    check_vector_declared_coverage(errors, warnings)
    check_invariant_coverage(errors, pending_invariants)
    check_vector_manifest_paths(errors)
    check_generator_imports(errors)

    for warning in warnings:
        print(f"warning: {warning}", file=sys.stderr)

    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        return 1

    print("Trellis spec checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
