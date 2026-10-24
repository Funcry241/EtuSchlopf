import hashlib
import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest
import zipfile

SPEC = importlib.util.spec_from_file_location("compare_exports", Path(__file__).parents[1] / "compare_exports.py")
C = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(C)
try:
    import fitz
except ImportError:
    fitz = None


def pdf(text="Same notes", pages=1, title="metadata"):
    doc = fitz.open()
    for _ in range(pages):
        page = doc.new_page()
        page.insert_text((72, 72), text)
    doc.set_metadata({"title": title})
    result = doc.tobytes()
    doc.close()
    return result


class ComparisonTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)

    def export(self, name, *, xml=b"<score/>", code=b"accepted", document=None,
               reused=False, corrupt_binding=False, lock=b"local"):
        source = io.BytesIO()
        with zipfile.ZipFile(source, "w") as z:
            for n in ["Cargo.toml", "README.md", "ZIP_KIND.txt", "run.cmd", "strict.cmd",
                      "docs/note.md", "tools/helper.rs", "tests/check.rs"]:
                z.writestr(n, b"fixture")
            z.writestr("src/example.rs", code)
            z.writestr("Cargo.lock", lock)
        payload = source.getvalue()
        digest = "invalid" if corrupt_binding else hashlib.sha256(payload).hexdigest()
        path = self.root / name
        with zipfile.ZipFile(path, "w") as z:
            z.writestr("source_snapshot/source.zip", payload)
            z.writestr("source_snapshot/SOURCE_SNAPSHOT.txt", f"SourceArchive=source.zip\nsource_complete=true\nSourceZipSha256=sha256:{digest}\n")
            z.writestr("Probe.musicxml", xml)
            z.writestr("Probe.render.musicxml", b"<render/>")
            z.writestr("corpus_run_manifest.json", json.dumps({"run_id": name}))
            z.writestr("manifest.txt", "FinalExit: 0\nBuildExit: 0\nClippyExit: 0\n")
            z.writestr("visual_preflight.strict-tests.txt", "ETUSCHLOPF STRICT ALL-TARGET TEST RECEIPT REUSE" if reused
                       else "test result: ok. 7 passed; 0 failed; 1 ignored;")
            if document is not None:
                z.writestr("Probe.pdf", document)
        return path

    def test_reuse_is_not_counted_as_fresh_tests(self):
        self.assertEqual(C.tests_observation("reused_exact_validation_receipt")["mode"], "reported_reused_bound_receipt")
        self.assertEqual(C.tests_observation("unrecognized")["mode"], "unknown")

    def test_missing_pdfs_cannot_produce_complete_equality(self):
        a, b = self.export("a.zip"), self.export("b.zip")
        self.assertEqual(C.compare(a, b)["status"], "INCOMPLETE")

    @unittest.skipIf(fitz is None, "PyMuPDF required")
    def test_changed_metadata_and_new_run_do_not_change_notation(self):
        a = self.export("a.zip", document=pdf(title="old"))
        b = self.export("b.zip", document=pdf(title="new"), reused=True)
        r = C.compare(a, b)
        self.assertEqual(r["status"], "SAME_MANAGED_SOURCE_XML_AND_PDF_RASTER")
        self.assertTrue(r["different_run_ids"])
        self.assertFalse(r["pdf"]["files"]["Probe.pdf"]["bytes_identical"])
        self.assertEqual(r["before_run"]["strict_tests"]["passed"], 7)
        self.assertEqual(r["after_run"]["strict_tests"]["mode"], "reported_reused_bound_receipt")

    @unittest.skipIf(fitz is None, "PyMuPDF required")
    def test_xml_change_is_not_hidden_by_same_pdf(self):
        image = pdf()
        a = self.export("a.zip", document=image)
        b = self.export("b.zip", document=image, xml=b"<score changed='yes'/>")
        r = C.compare(a, b)
        self.assertEqual(r["status"], "CHANGED")
        self.assertEqual(r["xml"]["modified"], ["Probe.musicxml"])

    @unittest.skipIf(fitz is None, "PyMuPDF required")
    def test_source_change_is_reported(self):
        image = pdf()
        r = C.compare(self.export("a.zip", document=image), self.export("b.zip", document=image, code=b"changed"))
        self.assertEqual(r["status"], "CHANGED")
        self.assertEqual(r["source"]["managed_delta"]["modified"], ["src/example.rs"])

    @unittest.skipIf(fitz is None, "PyMuPDF required")
    def test_actual_visible_change_is_detected(self):
        r = C.compare(self.export("a.zip", document=pdf()), self.export("b.zip", document=pdf("Different notes")))
        self.assertEqual(r["pdf"]["files"]["Probe.pdf"]["different_pages"], [1])
        self.assertEqual(r["status"], "CHANGED")

    @unittest.skipIf(fitz is None, "PyMuPDF required")
    def test_added_page_is_detected(self):
        r = C.compare(self.export("a.zip", document=pdf()), self.export("b.zip", document=pdf(pages=2)))
        self.assertEqual(r["pdf"]["files"]["Probe.pdf"]["different_pages"], [2])

    @unittest.skipIf(fitz is None, "PyMuPDF required")
    def test_bad_snapshot_binding_is_incomplete(self):
        image = pdf()
        r = C.compare(self.export("a.zip", document=image), self.export("b.zip", document=image, corrupt_binding=True))
        self.assertEqual(r["status"], "INCOMPLETE")

    @unittest.skipIf(fitz is None, "PyMuPDF required")
    def test_local_lock_is_distinguished_from_managed_source(self):
        image = pdf()
        r = C.compare(self.export("a.zip", document=image), self.export("b.zip", document=image, lock=b"other lock"))
        self.assertEqual(r["source"]["managed_delta"]["modified"], [])
        self.assertEqual(r["source"]["all_snapshot_entries_delta"]["modified"], ["Cargo.lock"])

    def test_duplicate_normalized_names_are_rejected(self):
        data = io.BytesIO()
        with zipfile.ZipFile(data, "w") as z:
            z.writestr("source/x", "one")
            z.writestr("source\\x", "two")
        with zipfile.ZipFile(io.BytesIO(data.getvalue())) as z:
            with self.assertRaisesRegex(ValueError, "Duplicate"):
                C.index(z)


if __name__ == "__main__":
    unittest.main()
