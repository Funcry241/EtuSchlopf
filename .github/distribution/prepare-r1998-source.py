"""Reconstruct the exact source files from the accepted Windows r1998 snapshot."""

import argparse
import hashlib
from pathlib import Path, PurePosixPath
from zipfile import ZIP_DEFLATED, ZipFile


EVIDENCE = Path("docs/evidence/r1998")
INVENTORY_SHA256 = "dc563cb53b5595a699ca8c2b101a2eacee1353e463d8446908e0aa479a3b2ec8"
ARCHIVE_NAME = "EtuSchlopf-r1998-source.zip"


def sha256(payload):
    return hashlib.sha256(payload).hexdigest()


def prepare(public, build, downloads):
    inventory = (public / EVIDENCE / "accepted-source.tsv").read_bytes()
    if sha256(inventory) != INVENTORY_SHA256:
        raise ValueError("Unexpected r1998 source inventory checksum")
    header, *rows = inventory.decode("utf-8").splitlines()
    if header != "sha256\tpath" or len(rows) != 2243:
        raise ValueError("Unexpected r1998 source inventory format or size")

    files = {}
    with ZipFile(public / EVIDENCE / "native-documents.zip") as native:
        projected = set(native.namelist())
        for row in rows:
            expected, name = row.split("\t")
            path = PurePosixPath(name)
            if path.is_absolute() or ".." in path.parts or name in files:
                raise ValueError(f"Invalid source path: {name}")
            payload = native.read(name) if name in projected else (public / path).read_bytes()
            if sha256(payload) != expected:
                raise ValueError(f"Source checksum mismatch: {name}")
            files[name] = payload

    build.mkdir()
    downloads.mkdir()
    with ZipFile(downloads / ARCHIVE_NAME, "w", ZIP_DEFLATED) as archive:
        for name, payload in files.items():
            destination = build / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(payload)
            archive.writestr(name, payload)
    print(f"Verified {len(files)} r1998 source files: {INVENTORY_SHA256}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("public", type=Path, help="Public r1998 checkout")
    parser.add_argument("build", type=Path, help="New native build directory")
    parser.add_argument("downloads", type=Path, help="New source archive directory")
    args = parser.parse_args()
    prepare(args.public, args.build, args.downloads)
