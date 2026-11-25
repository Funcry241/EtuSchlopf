"""Reconstruct the exact source files from the accepted Windows r2005 snapshot."""

import argparse
import hashlib
from pathlib import Path, PurePosixPath
from zipfile import ZIP_DEFLATED, ZipFile


EVIDENCE = Path("docs/evidence/r2005")
INVENTORY_SHA256 = "5253067a58f2984af4a28b5210c41298270dd69d5468bfe90fd5985bd1820ea8"
ARCHIVE_NAME = "EtuSchlopf-r2005-source.zip"


def sha256(payload):
    return hashlib.sha256(payload).hexdigest()


def prepare(public, build, downloads):
    inventory = (public / EVIDENCE / "accepted-source.tsv").read_bytes()
    if sha256(inventory) != INVENTORY_SHA256:
        raise ValueError("Unexpected r2005 source inventory checksum")
    header, *rows = inventory.decode("utf-8").splitlines()
    if header != "sha256\tpath" or len(rows) != 2247:
        raise ValueError("Unexpected r2005 source inventory format or size")

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
    print(f"Verified {len(files)} r2005 source files: {INVENTORY_SHA256}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("public", type=Path, help="Public r2005 checkout")
    parser.add_argument("build", type=Path, help="New native build directory")
    parser.add_argument("downloads", type=Path, help="New source archive directory")
    args = parser.parse_args()
    prepare(args.public, args.build, args.downloads)
