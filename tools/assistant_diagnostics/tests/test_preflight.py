import hashlib
import importlib.util
from pathlib import Path
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location("preflight", Path(__file__).parents[1] / "diagnose_preflight.py")
P = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(P)


class SourceChecks(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        for name in P.DIRS:
            (self.root / name).mkdir()
        self.note = self.root / "src/example.rs"
        self.note.write_text("fn accepted() {}\n")
        self.manifest = self.root / P.MANIFEST
        self.manifest.parent.mkdir()
        self.manifest.write_text("schema_version=1\nchange_class=MusicMutation\n")
        self.lock = self.root / "tools/watch/Cargo.lock"
        self.lock.parent.mkdir()
        self.lock.write_text("nested managed lock\n")
        self.reference = {"accepted_revision": 1463, "managed_sha256": {
            n: hashlib.sha256(p.read_bytes()).hexdigest() for n, p in P.managed_files(self.root).items()}}

    def test_unchanged_and_local_root_lock(self):
        (self.root / "Cargo.lock").write_text("local transactional state\n")
        self.assertEqual(P.check_tree(self.root, self.reference)["changes_vs_reference"], [])

    def test_undeclared_source_change(self):
        self.note.write_text("fn instrumented() {}\n")
        with self.assertRaisesRegex(ValueError, "UNDECLARED CHANGE"):
            P.check_tree(self.root, self.reference)

    def test_stale_hash_is_rejected(self):
        self.manifest.write_text("change=modified|src/example.rs|fnv64:0000000000000000\n")
        with self.assertRaisesRegex(ValueError, "STALE MANIFEST HASH"):
            P.check_tree(self.root, self.reference)

    def test_explicit_diagnostic_declaration_is_checked(self):
        self.note.write_text("fn instrumented() {}\n")
        self.manifest.write_text("change=modified|src/example.rs|" + P.fnv(self.note.read_bytes()) + "\n")
        self.assertIn("src/example.rs", P.check_tree(self.root, self.reference)["changes_vs_reference"])

    def test_nested_lock_is_not_exempt(self):
        self.lock.write_text("changed nested lock\n")
        with self.assertRaisesRegex(ValueError, "UNDECLARED CHANGE"):
            P.check_tree(self.root, self.reference)

    def test_missing_member_is_not_silently_ignored(self):
        self.note.unlink()
        with self.assertRaisesRegex(ValueError, "UNDECLARED CHANGE"):
            P.check_tree(self.root, self.reference)

    def test_path_traversal_is_rejected(self):
        self.manifest.write_text("change=modified|../outside.rs|fnv64:0000000000000000\n")
        with self.assertRaisesRegex(ValueError, "Unsafe"):
            P.check_tree(self.root, self.reference)

    def test_duplicate_declaration_is_rejected(self):
        line = "change=modified|src/example.rs|" + P.fnv(self.note.read_bytes()) + "\n"
        self.manifest.write_text(line + line)
        with self.assertRaisesRegex(ValueError, "Duplicate"):
            P.check_tree(self.root, self.reference)


if __name__ == "__main__":
    unittest.main()
