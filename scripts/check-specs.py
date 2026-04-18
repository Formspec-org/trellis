#!/usr/bin/env python3
"""Lint normalized Trellis specification documents."""

from __future__ import annotations

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

    if errors:
        for error in errors:
            print(error, file=sys.stderr)
        return 1

    print("Trellis spec checks passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
