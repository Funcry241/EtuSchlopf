"""Recover the exact embedded runtime ZIP from the pinned public r1526 EXE."""
import argparse
import hashlib
import io
from pathlib import Path
import struct
import zipfile

RELEASE_SHA256 = "70ef7a230b615fca2a8da888512c077825d5f6afadc1821d823d1c7a37f2a61f"


def extract(source: Path, destination: Path) -> None:
    data = source.read_bytes()
    if hashlib.sha256(data).hexdigest() != RELEASE_SHA256:
        raise ValueError("The source EXE does not match the published r1526 download")
    candidates = []
    offset = 0
    while (offset := data.find(b"PK\x05\x06", offset)) >= 0:
        end_record = offset
        offset += 4
        if end_record + 22 > len(data):
            continue
        disk, cd_disk, disk_count, count, cd_size, cd_offset, comment = struct.unpack_from(
            "<4H2IH", data, end_record + 4
        )
        if disk or cd_disk or not count or count != disk_count:
            continue
        start = end_record - cd_size - cd_offset
        end = end_record + 22 + comment
        if start < 0 or end > len(data) or data[start:start + 4] != b"PK\x03\x04":
            continue
        archive = data[start:end]
        try:
            with zipfile.ZipFile(io.BytesIO(archive)) as z:
                names = set(z.namelist())
                if "etuschlopf_engine.exe" not in names:
                    continue
                if not any(n.startswith("docs/normative_sources/musicxml/") for n in names):
                    continue
                if z.testzip() is not None:
                    continue
                engine_hash = hashlib.sha256(z.read("etuschlopf_engine.exe")).hexdigest()
        except (zipfile.BadZipFile, OSError, ValueError):
            continue
        candidates.append((archive, engine_hash))
    if len(candidates) != 1:
        raise ValueError(f"Expected exactly one embedded runtime, found {len(candidates)}")
    archive, engine_hash = candidates[0]
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(archive)
    print(f"Published EXE SHA256: {RELEASE_SHA256}")
    print(f"Unchanged embedded engine SHA256: {engine_hash}")
    print(f"Exact embedded runtime ZIP SHA256: {hashlib.sha256(archive).hexdigest()}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source", type=Path)
    parser.add_argument("destination", type=Path)
    args = parser.parse_args()
    extract(args.source, args.destination)
