"""Recover every exact clef-planner entry from a console log; no musical inference."""
from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import zlib

BEGIN = "ClefPlannerInputBegin: "
HEX = "ClefPlannerInputHex: "
END = "ClefPlannerInputEnd: InputSha256="
HEADER = re.compile(
    r"Schema=1; InputFnv64=([0-9a-f]{16}); InputBytes=([0-9]+); "
    r"InputSha256=([0-9a-f]{64}); Encoding=zlib_hex; CompressedBytes=([0-9]+); "
    r"SourceRole=actual_function_entry; WriterCommit=not_observed; Authority=none"
)


def fnv64(data: bytes) -> str:
    value = 0xCBF29CE484222325
    for byte in data:
        value = ((value ^ byte) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return f"{value:016x}"


def extract(log: str) -> list[dict]:
    rows = []
    header = None
    chunks: list[str] = []
    start_line = 0
    for line_number, line in enumerate(log.splitlines(), 1):
        if line.startswith(BEGIN):
            if header is not None:
                raise ValueError("Nested or incomplete clef input frame")
            header = HEADER.fullmatch(line.removeprefix(BEGIN))
            if header is None:
                raise ValueError("Malformed clef input header")
            chunks = []
            start_line = line_number
        elif header is not None:
            if line.startswith(HEX):
                chunk = line.removeprefix(HEX)
                if not re.fullmatch(r"(?:[0-9a-f]{2}){1,64}", chunk):
                    raise ValueError("Malformed compressed hex row")
                if chunks and len(chunks[-1]) != 128:
                    raise ValueError("Short non-final compressed hex row")
                chunks.append(chunk)
            elif line.startswith(END):
                fnv, size_text, sha, compressed_size = header.groups()
                if line.removeprefix(END) != sha:
                    raise ValueError("Clef input footer identity mismatch")
                compressed = bytes.fromhex("".join(chunks))
                if len(compressed) != int(compressed_size):
                    raise ValueError("Compressed byte count mismatch")
                size = int(size_text)
                decoder = zlib.decompressobj()
                try:
                    data = decoder.decompress(compressed, size + 1)
                except zlib.error as error:
                    raise ValueError("Invalid compressed input") from error
                if not decoder.eof or decoder.unused_data or decoder.unconsumed_tail:
                    raise ValueError("Incomplete, oversized or trailing compressed input")
                if len(data) != size or hashlib.sha256(data).hexdigest() != sha or fnv64(data) != fnv:
                    raise ValueError("Decoded input identity mismatch")
                data.decode("utf-8")
                rows.append({"index": len(rows), "line_start": start_line, "line_end": line_number,
                             "input_bytes": size, "input_sha256": sha, "input_fnv64": fnv,
                             "compressed_bytes": len(compressed), "data": data})
                header = None
                chunks = []
            else:
                raise ValueError("Unexpected line inside clef input frame")
        elif line.startswith("ClefPlannerInput"):
            raise ValueError("Unavailable, orphaned or unsupported clef input record")
    if header is not None:
        raise ValueError("Truncated clef input frame")
    if not rows:
        raise ValueError("No exact clef input captures observed")
    return rows


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("console_log", type=Path)
    parser.add_argument("output_directory", type=Path, help="New directory; captures retain repeated occurrences")
    args = parser.parse_args()
    rows = extract(args.console_log.read_text(encoding="utf-8-sig"))
    args.output_directory.mkdir(parents=True, exist_ok=False)
    manifest = []
    for row in rows:
        item = dict(row)
        data = item.pop("data")
        name = f"input_{item['index']:06d}_{item['input_sha256']}.musicxml"
        (args.output_directory / name).write_bytes(data)
        item["file"] = name
        manifest.append(item)
    (args.output_directory / "index.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    print(f"Verified {len(rows)} exact input observations; no deduplication or musical classification")


if __name__ == "__main__":
    main()
