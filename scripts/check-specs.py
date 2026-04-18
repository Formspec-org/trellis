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


def matrix_ids() -> list[str]:
    text = read(SPECS / "trellis-requirements-matrix.md")
    return re.findall(r"^\| (TR-(?:CORE|OP)-[0-9]{3}) \|", text, re.MULTILINE)


def matrix_rows() -> list[dict]:
    """Return parsed matrix rows: {id, scope, invariant, verification, ...}."""
    text = read(SPECS / "trellis-requirements-matrix.md")
    row_pattern = re.compile(r"^\| (TR-(?:CORE|OP)-[0-9]{3}) \|(.+)$", re.MULTILINE)
    rows = []
    for m in row_pattern.finditer(text):
        row_id = m.group(1)
        # Split remaining cells by '|'; matrix has columns:
        # Scope | Invariant | Requirement | Rationale | Verification | Legacy | Notes
        cells = [c.strip() for c in m.group(2).split("|")]
        # cells[0]=Scope, cells[1]=Invariant, cells[2]=Requirement,
        # cells[3]=Rationale, cells[4]=Verification
        invariant = cells[1] if len(cells) > 1 else "—"
        verification = cells[4] if len(cells) > 4 else ""
        rows.append({"id": row_id, "invariant": invariant, "verification": verification})
    return rows


def testable_row_ids() -> set[str]:
    """Return IDs of matrix rows where Verification contains 'test-vector'."""
    return {r["id"] for r in matrix_rows() if "test-vector" in r["verification"]}


def vector_manifests() -> list[tuple[Path, dict]]:
    """Return (manifest_path, parsed_toml) for every vector manifest under fixtures/vectors/."""
    import tomllib

    manifests = []
    if not FIXTURES.exists():
        return manifests
    for op_dir in ["append", "verify", "export", "tamper"]:
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


def derived_invariants_for_tr_core(row_ids: list[str]) -> set[int]:
    """Return the set of invariant numbers declared for the given TR-CORE rows in the matrix."""
    derived: set[int] = set()
    row_id_set = set(row_ids)
    for r in matrix_rows():
        if r["id"] in row_id_set:
            derived.update(parse_invariants_cell(r["invariant"]))
    return derived


def check_vector_declared_coverage(errors: list[str]) -> None:
    if os.environ.get("TRELLIS_SKIP_COVERAGE") == "1":
        return
    for path, manifest in vector_manifests():
        coverage = manifest.get("coverage", {})
        tr_core = coverage.get("tr_core", [])
        declared_sections = set(coverage.get("core_sections", [])) if "core_sections" in coverage else None
        declared_invariants = set(coverage.get("invariants", [])) if "invariants" in coverage else None

        if declared_sections is not None:
            derived = derived_sections_for_tr_core(tr_core)
            if declared_sections != derived:
                errors.append(
                    f"{path.relative_to(ROOT)}: declared core_sections={sorted(declared_sections)} "
                    f"does not equal matrix-derived={sorted(derived)}"
                )
        if declared_invariants is not None:
            derived = derived_invariants_for_tr_core(tr_core)
            if declared_invariants != derived:
                errors.append(
                    f"{path.relative_to(ROOT)}: declared invariants={sorted(declared_invariants)} "
                    f"does not equal matrix-derived={sorted(derived)}"
                )


def check_invariant_coverage(errors: list[str]) -> None:
    if os.environ.get("TRELLIS_SKIP_COVERAGE") == "1":
        return
    rows = matrix_rows()
    testable_by_invariant: dict[int, list[str]] = {}
    for r in rows:
        if "test-vector" not in r["verification"]:
            continue
        for inv in parse_invariants_cell(r["invariant"]):
            testable_by_invariant.setdefault(inv, []).append(r["id"])

    covered_ids: set[str] = set()
    for path, manifest in vector_manifests():
        covered_ids.update(manifest.get("coverage", {}).get("tr_core", []))

    for inv in sorted(PHASE_1_INVARIANTS):
        testable_rows = testable_by_invariant.get(inv, [])
        if not testable_rows:
            errors.append(
                f"specs/trellis-requirements-matrix.md: invariant #{inv} has no "
                f"row with Verification=test-vector"
            )
            continue
        if not any(rid in covered_ids for rid in testable_rows):
            errors.append(
                f"specs/trellis-requirements-matrix.md: invariant #{inv} has no "
                f"vector via any of its testable rows {testable_rows}"
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


def check_vector_coverage(errors: list[str]) -> None:
    if os.environ.get("TRELLIS_SKIP_COVERAGE") == "1":
        return
    testable = testable_row_ids()
    covered: set[str] = set()
    for _path, manifest in vector_manifests():
        covered.update(manifest.get("coverage", {}).get("tr_core", []))
    for row_id in sorted(testable - covered):
        errors.append(
            f"specs/trellis-requirements-matrix.md: no vector covers {row_id} "
            f"(row has Verification=test-vector but no fixtures/vectors/*/manifest.toml "
            f"references it in coverage.tr_core)"
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


def main() -> int:
    errors: list[str] = []
    check_forbidden_terms(errors)
    check_core_section_references(errors)
    check_requirement_ids(errors)
    check_traceability_anchors(errors)
    check_bare_profile(errors)
    check_archived_inputs(errors)
    check_vector_coverage(errors)
    check_vector_declared_coverage(errors)
    check_invariant_coverage(errors)
    check_generator_imports(errors)

    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        return 1

    print("Trellis spec checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
