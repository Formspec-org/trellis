"""Unit tests for the vector renumbering pre-merge guard."""

import contextlib
import io
import importlib.util
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "scripts" / "check-vector-renumbering.py"


def _load_module():
    spec = importlib.util.spec_from_file_location("check_vector_renumbering", SCRIPT)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


class TestVectorRenumberingGuard(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.mod = _load_module()

    def test_extracts_prefixes_from_manifest_paths(self):
        prefixes = self.mod.vector_prefixes_from_paths(
            [
                "fixtures/vectors/append/001-minimal-inline-payload/manifest.toml",
                "fixtures/vectors/projection/003-cadence-positive-height/manifest.toml",
                "fixtures/vectors/_generator/gen_append_001.py",
                "fixtures/vectors/append/not-a-vector/manifest.toml",
                "fixtures/vectors/append/002-example/derivation.md",
            ]
        )
        self.assertEqual(
            prefixes,
            {
                self.mod.VectorPrefix("append", "001"),
                self.mod.VectorPrefix("projection", "003"),
            },
        )

    def test_slug_rename_preserving_prefix_passes(self):
        base = self.mod.vector_prefixes_from_paths(
            ["fixtures/vectors/append/001-old-slug/manifest.toml"]
        )
        current = self.mod.vector_prefixes_from_paths(
            ["fixtures/vectors/append/001-new-slug/manifest.toml"]
        )
        self.assertEqual(self.mod.missing_base_prefixes(base, current), [])

    def test_renumbered_prefix_fails(self):
        base = self.mod.vector_prefixes_from_paths(
            ["fixtures/vectors/append/001-old-slug/manifest.toml"]
        )
        current = self.mod.vector_prefixes_from_paths(
            ["fixtures/vectors/append/002-new-number/manifest.toml"]
        )
        self.assertEqual(
            self.mod.missing_base_prefixes(base, current),
            [self.mod.VectorPrefix("append", "001")],
        )

    def test_same_number_in_different_op_does_not_satisfy_prefix(self):
        base = self.mod.vector_prefixes_from_paths(
            ["fixtures/vectors/append/001-old-slug/manifest.toml"]
        )
        current = self.mod.vector_prefixes_from_paths(
            ["fixtures/vectors/projection/001-new-slug/manifest.toml"]
        )
        self.assertEqual(
            self.mod.missing_base_prefixes(base, current),
            [self.mod.VectorPrefix("append", "001")],
        )

    def test_cli_allows_slug_rename_preserving_prefix(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._init_repo_with_vector(root, "append/001-old-slug")
            (root / "fixtures/vectors/append/001-new-slug").mkdir(parents=True)
            (root / "fixtures/vectors/append/001-new-slug/manifest.toml").write_text(
                'id = "append/001-new-slug"\nop = "append"\n',
                encoding="utf-8",
            )
            self._remove_tree(root / "fixtures/vectors/append/001-old-slug")

            code, stdout, stderr = self._run_main(
                ["--root", str(root), "--base-ref", "HEAD"]
            )

        self.assertEqual(code, 0, msg=stderr)
        self.assertIn("renumbering check passed", stdout)

    def test_cli_rejects_renumbered_prefix(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._init_repo_with_vector(root, "append/001-old-slug")
            (root / "fixtures/vectors/append/002-new-number").mkdir(parents=True)
            (root / "fixtures/vectors/append/002-new-number/manifest.toml").write_text(
                'id = "append/002-new-number"\nop = "append"\n',
                encoding="utf-8",
            )
            self._remove_tree(root / "fixtures/vectors/append/001-old-slug")

            code, stdout, stderr = self._run_main(
                ["--root", str(root), "--base-ref", "HEAD"]
            )

        self.assertEqual(code, 1)
        self.assertEqual(stdout, "")
        self.assertIn("append/001-*", stderr)
        self.assertIn("renumbering", stderr)

    def test_cli_missing_base_ref_returns_two(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._init_repo_with_vector(root, "append/001-old-slug")

            code, stdout, stderr = self._run_main(
                ["--root", str(root), "--base-ref", "does-not-exist"]
            )

        self.assertEqual(code, 2)
        self.assertEqual(stdout, "")
        self.assertIn("could not read fixtures/vectors", stderr)

    def _init_repo_with_vector(self, root: Path, vector_id: str) -> None:
        subprocess.run(["git", "init", "-q"], cwd=root, check=True)
        subprocess.run(
            ["git", "config", "user.email", "tests@example.invalid"],
            cwd=root,
            check=True,
        )
        subprocess.run(
            ["git", "config", "user.name", "Trellis Tests"],
            cwd=root,
            check=True,
        )
        op = vector_id.split("/", 1)[0]
        vector_dir = root / "fixtures/vectors" / vector_id
        vector_dir.mkdir(parents=True)
        (vector_dir / "manifest.toml").write_text(
            f'id = "{vector_id}"\nop = "{op}"\n',
            encoding="utf-8",
        )
        subprocess.run(["git", "add", "fixtures"], cwd=root, check=True)
        subprocess.run(
            ["git", "commit", "-qm", "add base vector"],
            cwd=root,
            check=True,
        )

    def _run_main(self, argv: list[str]) -> tuple[int, str, str]:
        stdout = io.StringIO()
        stderr = io.StringIO()
        with contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(stderr):
            code = self.mod.main(argv)
        return code, stdout.getvalue(), stderr.getvalue()

    def _remove_tree(self, path: Path) -> None:
        for child in sorted(path.rglob("*"), reverse=True):
            if child.is_file():
                child.unlink()
            else:
                child.rmdir()
        path.rmdir()


if __name__ == "__main__":
    unittest.main()
