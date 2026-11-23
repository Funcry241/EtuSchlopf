"""Reconstruct the Windows-accepted r1994 source from the pinned public tree."""

import argparse
import hashlib
from pathlib import Path, PurePosixPath
from zipfile import ZIP_DEFLATED, ZipFile


R1985_EVIDENCE = Path("docs/evidence/r1985")
R1985_INVENTORY_SHA256 = "8e0a177113312e0c86b244f5c168416cfd1cd0418e2ae2c3a9345a30b53620b1"
R1994_INVENTORY_SHA256 = "85fe93c6ba9b1e3d4053a270327bfc9607ceaf9cd907ec28903d673ce074cc42"
ADDED_PATHS = {
    "docs/selector_derivation_contract_phrase_slur_clearance.md",
    "docs/selector_derivation_contract_phrase_slur_clearance_implementation.md",
    "src/structural_write_authority/phrase_svg_geometry.rs",
    "src/structural_write_authority/phrase_svg_placement.rs",
}


def sha256(payload):
    return hashlib.sha256(payload).hexdigest()


def prepare(public, build, downloads):
    inventory = (public / R1985_EVIDENCE / "accepted-source.tsv").read_bytes()
    if sha256(inventory) != R1985_INVENTORY_SHA256:
        raise ValueError("Unexpected r1985 path inventory checksum")
    header, *rows = inventory.decode("utf-8").splitlines()
    if header != "sha256\tpath":
        raise ValueError("Unexpected source inventory header")
    names = {row.split("\t", 1)[1] for row in rows}
    if len(names) != len(rows) or len(names | ADDED_PATHS) != 2233:
        raise ValueError("Unexpected r1994 source path count")

    files = {}
    actual = hashlib.sha256(b"sha256\tpath\n")
    for name in sorted(names | ADDED_PATHS):
        path = PurePosixPath(name)
        if path.is_absolute() or ".." in path.parts:
            raise ValueError(f"Invalid source path: {name}")
        archived = public / R1985_EVIDENCE / "native-documents" / path
        payload = (archived if archived.is_file() else public / path).read_bytes()
        files[name] = payload
        actual.update(f"{sha256(payload)}\t{name}\n".encode("utf-8"))
    if actual.hexdigest() != R1994_INVENTORY_SHA256:
        raise ValueError("Reconstructed files do not match the Windows-accepted r1994 source")

    build.mkdir()
    downloads.mkdir()
    with ZipFile(downloads / "EtuSchlopf-r1994-source.zip", "w", ZIP_DEFLATED) as archive:
        for name, payload in files.items():
            destination = build / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(payload)
            archive.writestr(name, payload)
    print(f"Verified {len(files)} r1994 source files: {actual.hexdigest()}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("public", type=Path, help="Pinned public r1994 checkout")
    parser.add_argument("build", type=Path, help="New native build directory")
    parser.add_argument("downloads", type=Path, help="New output directory for the source archive")
    args = parser.parse_args()
    prepare(args.public, args.build, args.downloads)
