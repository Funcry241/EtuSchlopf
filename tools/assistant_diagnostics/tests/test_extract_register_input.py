import hashlib
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from extract_register_input import extract


def report(data):
    rows = "".join(f"InputHex: {data[i:i + 64].hex()}\n" for i in range(0, len(data), 64))
    return (f"InputBytes: {len(data)}.\nInputSha256: {hashlib.sha256(data).hexdigest()}.\n"
            f"InputHexBegin\n{rows}InputHexEnd\n")


class ExtractRegisterInputTests(unittest.TestCase):
    def test_utf8_and_empty_roundtrip(self):
        for data in [b"", "<score><!-- Takt 17; Ä𝄞 --></score>\r\n".encode() * 20]:
            self.assertEqual(extract(report(data)), data)

    def test_corruption_is_rejected(self):
        text = report(b"abcd")
        for corrupted in [text.replace("61626364", "61626365"),
                          text.replace("InputBytes: 4", "InputBytes: 3"),
                          text.replace("61626364", "xyz")]:
            with self.assertRaises(ValueError):
                extract(corrupted)

    def test_duplicate_or_truncated_capture_is_rejected(self):
        text = report(b"abcd")
        for broken in [text + text, text.replace("InputHexEnd\n", ""),
                       text.replace("InputHexBegin\n", "")]:
            with self.assertRaises(ValueError):
                extract(broken)
