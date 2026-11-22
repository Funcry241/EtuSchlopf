"""Reconstruct and verify the native r1985 source for the Windows preview."""

import argparse
import hashlib
import subprocess
from pathlib import Path, PurePosixPath
from zipfile import ZIP_DEFLATED, ZipFile


EVIDENCE = Path("docs/evidence/r1985")
NATIVE_DOCUMENTS = EVIDENCE / "native-documents"
R1985_PUBLIC_COMMIT = "89ed56ce9b8e018593bf5d47229773079d7bdb0b"
PROJECTED = {
    "README.md",
    "docs/README.md",
    "docs/MATHEMATIK.md",
    "docs/MATHEMATIK-VOLLKORPUS.md",
    "docs/CONTRIBUTING.md",
    "docs/THIRD_PARTY_NOTICES.md",
    "docs/community/issue_templates/01_fehler.md",
}


def prepare(public, build, downloads):
    table = (public / EVIDENCE / "accepted-source.tsv").read_bytes()
    if hashlib.sha256(table).hexdigest() != "8e0a177113312e0c86b244f5c168416cfd1cd0418e2ae2c3a9345a30b53620b1":
        raise ValueError("Unexpected r1985 inventory checksum")
    header, *rows = table.decode("utf-8").splitlines()
    if header != "sha256\tpath" or len(rows) != 2229:
        raise ValueError("Unexpected r1985 inventory")
    files = {}
    historical_source_ready = False
    for row in rows:
        expected, name = row.split("\t")
        path = PurePosixPath(name)
        if path.is_absolute() or ".." in path.parts or name in files:
            raise ValueError(f"Invalid source inventory path: {name}")
        source = public / (NATIVE_DOCUMENTS / name if name in PROJECTED else path)
        payload = source.read_bytes() if source.is_file() else None
        if name not in PROJECTED and (
            payload is None or hashlib.sha256(payload).hexdigest() != expected
        ):
            # The preview remains bound to r1985 as the main source advances.
            git = ["git", "-C", str(public)]
            if not historical_source_ready:
                if subprocess.run(
                    git + ["cat-file", "-e", f"{R1985_PUBLIC_COMMIT}^{{commit}}"],
                    capture_output=True,
                ).returncode != 0:
                    subprocess.run(
                        git + ["fetch", "--no-tags", "--depth=1", "origin", R1985_PUBLIC_COMMIT],
                        check=True,
                    )
                historical_source_ready = True
            payload = subprocess.check_output(git + ["show", f"{R1985_PUBLIC_COMMIT}:{name}"])
        if payload is None or hashlib.sha256(payload).hexdigest() != expected:
            raise ValueError(f"Native source checksum mismatch: {name}")
        files[name] = payload

    build.mkdir()
    downloads.mkdir()
    with ZipFile(downloads / "EtuSchlopf-r1985-source.zip", "w", ZIP_DEFLATED) as archive:
        for name, payload in files.items():
            destination = build / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(payload)
            archive.writestr(name, payload)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("public", type=Path, help="Public Git checkout")
    parser.add_argument("build", type=Path, help="New native build directory")
    parser.add_argument("downloads", type=Path, help="New output directory for the source archive")
    args = parser.parse_args()
    prepare(args.public, args.build, args.downloads)
