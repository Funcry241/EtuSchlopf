from __future__ import annotations

import hashlib
import importlib.util
from pathlib import Path
import unittest
import zlib
import re

PATH = Path(__file__).resolve().parents[1] / "extract_clef_inputs.py"
SPEC = importlib.util.spec_from_file_location("extract_clef_inputs", PATH)
MOD = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MOD)


def frame(data: bytes) -> str:
    compressed = zlib.compress(data, 1)
    sha = hashlib.sha256(data).hexdigest()
    header = (f"Schema=1; InputFnv64={MOD.fnv64(data)}; InputBytes={len(data)}; "
              f"InputSha256={sha}; Encoding=zlib_hex; CompressedBytes={len(compressed)}; "
              "SourceRole=actual_function_entry; WriterCommit=not_observed; Authority=none")
    rows = "".join(MOD.HEX + compressed[i:i+64].hex() + "\n" for i in range(0, len(compressed), 64))
    return MOD.BEGIN + header + "\n" + rows + MOD.END + sha + "\n"


class ExactClefInputTests(unittest.TestCase):
    def test_empty_unicode_crlf_and_long_roundtrip(self):
        for data in [b"", "<score>\r\n<!-- ö𝄞 -->\n</score>".encode(), b"<note/>" * 10000]:
            with self.subTest(length=len(data)):
                self.assertEqual(MOD.extract(frame(data))[0]["data"], data)

    def test_equal_calls_are_not_deduplicated_and_lines_are_recorded(self):
        data = b"<score/>"
        text = "Other diagnostic\n" + frame(data) + frame(data) + "Other diagnostic\n"
        rows = MOD.extract(text)
        self.assertEqual([r["index"] for r in rows], [0, 1])
        self.assertEqual([r["data"] for r in rows], [data, data])
        self.assertLess(rows[0]["line_end"], rows[1]["line_start"])

    def test_missing_truncated_nested_and_orphaned_frames_fail(self):
        good = frame(b"<score/>")
        for bad in ["", good.rsplit(MOD.END, 1)[0], good.split(MOD.HEX)[0] + good,
                    MOD.END + "0" * 64, "ClefPlannerInputUnavailable: encoding_failed\n"]:
            with self.subTest(log=bad[:80]):
                with self.assertRaises(ValueError):
                    MOD.extract(bad)

    def test_identity_and_payload_corruption_fail(self):
        good = frame(b"<score/>")
        for bad in [good.replace("InputBytes=8;", "InputBytes=9;"),
                    good.replace("InputFnv64=", "InputFnv64=0"),
                    good.replace(MOD.HEX, MOD.HEX + "ff", 1),
                    good.replace(MOD.END, MOD.END + "0"),
                    good.replace("Encoding=zlib_hex", "Encoding=other"),
                    good.replace(MOD.HEX, "Unexpected: ", 1)]:
            with self.subTest(log=bad[:80]):
                with self.assertRaises(ValueError):
                    MOD.extract(bad)

    def test_valid_stream_with_wrong_identity_size_or_trailing_bytes_fails(self):
        good = frame(b"<score/>")
        wrong_hash = re.sub(r"InputSha256=[0-9a-f]{64}", "InputSha256=" + "0" * 64, good)
        wrong_fnv = re.sub(r"InputFnv64=[0-9a-f]{16}", "InputFnv64=" + "0" * 16, good)
        compressed = zlib.compress(b"<score/>", 1) + b"\0"
        header, rest = good.split("\n", 1)
        header = re.sub(r"CompressedBytes=[0-9]+", f"CompressedBytes={len(compressed)}", header)
        footer = next(line for line in rest.splitlines() if line.startswith(MOD.END))
        trailing = header + "\n" + MOD.HEX + compressed.hex() + "\n" + footer + "\n"
        for bad in [wrong_hash, wrong_fnv, good.replace("InputBytes=8;", "InputBytes=7;"), trailing]:
            with self.subTest(log=bad[:80]):
                with self.assertRaises(ValueError):
                    MOD.extract(bad)


if __name__ == "__main__":
    unittest.main()
