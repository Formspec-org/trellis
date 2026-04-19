#!/usr/bin/env python3
"""Pre-merge guard for frozen Trellis vector number prefixes.

This check compares vector manifest prefixes from a ratification/base ref
against the current working tree. Once `fixtures/vectors/<op>/NNN-*` has
merged, the `<op>/NNN` prefix is stable: authors may rename the slug while
preserving the prefix, or mark the vector deprecated, but must not delete or
renumber it.
"""

from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(os.environ.get("TRELLIS_RENUMBERING_ROOT", Path(__file__).resolve().parents[1]))
VECTOR_OPS = ("append", "verify", "export", "tamper", "projection", "shred")

_MANIFEST_PATTERN = re.compile(
    r"^fixtures/vectors/"
    r"(?P<op>append|verify|export|tamper|projection|shred)/"
    r"(?P<prefix>[0-9]{3})-[^/]+/manifest\.toml$"
)


@dataclass(frozen=True, order=True)
class VectorPrefix:
    op: str
    prefix: str

    def label(self) -> str:
        return f"{self.op}/{self.prefix}-*"


def vector_prefixes_from_paths(paths: list[str]) -> set[VectorPrefix]:
    """Extract frozen `<op>/NNN` prefixes from manifest paths."""
    prefixes: set[VectorPrefix] = set()
    for path in paths:
        normalized = path.replace("\\", "/")
        match = _MANIFEST_PATTERN.match(normalized)
        if match:
            prefixes.add(
                VectorPrefix(op=match.group("op"), prefix=match.group("prefix"))
            )
    return prefixes


def current_vector_prefixes(root: Path) -> set[VectorPrefix]:
    """Read current working-tree vector manifests."""
    paths: list[str] = []
    fixtures = root / "fixtures" / "vectors"
    if not fixtures.exists():
        return set()

    for op in VECTOR_OPS:
        op_dir = fixtures / op
        if not op_dir.exists():
            continue
        for manifest in sorted(op_dir.glob("[0-9][0-9][0-9]-*/manifest.toml")):
            paths.append(manifest.relative_to(root).as_posix())
    return vector_prefixes_from_paths(paths)


def base_ref_vector_prefixes(root: Path, base_ref: str) -> set[VectorPrefix]:
    """Read vector manifest prefixes from `base_ref` using git ls-tree."""
    result = subprocess.run(
        [
            "git",
            "-C",
            str(root),
            "ls-tree",
            "-r",
            "--name-only",
            base_ref,
            "--",
            "fixtures/vectors",
        ],
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        raise RuntimeError(
            f"could not read fixtures/vectors from base ref {base_ref!r}: "
            f"{result.stderr.strip() or result.stdout.strip()}"
        )
    return vector_prefixes_from_paths(result.stdout.splitlines())


def missing_base_prefixes(
    base_prefixes: set[VectorPrefix], current_prefixes: set[VectorPrefix]
) -> list[VectorPrefix]:
    """Return upstream prefixes no longer present in the working tree."""
    return sorted(base_prefixes - current_prefixes)


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Fail when a vector <op>/NNN prefix present on the base ref is "
            "missing from the current working tree."
        )
    )
    parser.add_argument(
        "--base-ref",
        default=os.environ.get("TRELLIS_RATIFICATION_REF", "origin/main"),
        help=(
            "Ratification/base ref to compare against "
            "(default: TRELLIS_RATIFICATION_REF or origin/main)."
        ),
    )
    parser.add_argument(
        "--root",
        type=Path,
        default=ROOT,
        help="Repository root (default: script parent or TRELLIS_RENUMBERING_ROOT).",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    root = args.root.resolve()

    try:
        base_prefixes = base_ref_vector_prefixes(root, args.base_ref)
    except RuntimeError as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 2

    current_prefixes = current_vector_prefixes(root)
    missing = missing_base_prefixes(base_prefixes, current_prefixes)
    if missing:
        for prefix in missing:
            print(
                f"fixtures/vectors/{prefix.label()}: existed in {args.base_ref} "
                f"but no current vector preserves that op/NNN prefix; keep the "
                f"directory as a deprecated tombstone instead of deleting or "
                f"renumbering it",
                file=sys.stderr,
            )
        return 1

    print(
        "Trellis vector renumbering check passed "
        f"({len(base_prefixes)} base prefixes preserved)."
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
