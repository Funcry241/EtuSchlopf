"""Recover the exact captured planner input; verify before creating a new file."""
from __future__ import annotations

import argparse
import hashlib
from pathlib import Path
import re


def extract(report: str) -> bytes:
    if report.count("InputHexBegin\n") != 1 or report.count("InputHexEnd\n") != 1:
        raise ValueError("Expected exactly one complete input capture")
    before, rest = report.split("InputHexBegin\n")
    block, _ = rest.split("InputHexEnd\n")
    rows = block.splitlines()
    if any(not re.fullmatch(r"InputHex: (?:[0-9a-f]{2}){1,64}", row) for row in rows):
        raise ValueError("Malformed input hex row")
    data = bytes.fromhex("".join(row.removeprefix("InputHex: ") for row in rows))
    sizes = re.findall(r"^InputBytes: ([0-9]+)\.$", before, re.MULTILINE)
    hashes = re.findall(r"^InputSha256: ([0-9a-f]{64})\.$", before, re.MULTILINE)
    if len(sizes) != 1 or len(hashes) != 1:
        raise ValueError("Missing or ambiguous input identity")
    if len(data) != int(sizes[0]) or hashlib.sha256(data).hexdigest() != hashes[0]:
        raise ValueError("Input byte count or SHA-256 mismatch")
    return data


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("report", type=Path)
    parser.add_argument("output", type=Path)
    args = parser.parse_args()
    data = extract(args.report.read_text(encoding="utf-8-sig"))
    with args.output.open("xb") as output:
        output.write(data)
    print(f"Verified {len(data)} bytes; SHA-256 {hashlib.sha256(data).hexdigest()}")


if __name__ == "__main__":
    main()
