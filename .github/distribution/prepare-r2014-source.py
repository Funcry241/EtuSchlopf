"""Reconstruct the accepted engine source for the Windows release build."""
import argparse
from hashlib import sha256
import json
from pathlib import Path, PurePosixPath
import re
from zipfile import ZIP_DEFLATED, ZipFile


def digest(payload):
    return sha256(payload).hexdigest()


def validated_files(public, revision):
    evidence = public / "docs" / "evidence" / revision
    metadata = json.loads((evidence / "accepted-release.json").read_bytes())
    if metadata["version"] != "0.10.1839" or metadata["revision"] != revision:
        raise ValueError("Windows evidence targets an unexpected source revision")
    for key in (
        "accepted_take_sha256", "native_snapshot_sha256", "inventory_sha256",
        "native_documents_sha256", "cargo_lock_sha256",
    ):
        if not re.fullmatch(r"[0-9a-f]{64}", metadata[key]):
            raise ValueError(f"Missing or invalid accepted-source digest: {key}")

    inventory = (evidence / "accepted-source.tsv").read_bytes()
    if digest(inventory) != metadata["inventory_sha256"]:
        raise ValueError("Unexpected accepted-source inventory checksum")
    header, *rows = inventory.decode("utf8").splitlines()
    if header != "sha256\tpath" or len(rows) != metadata["source_file_count"]:
        raise ValueError("Unexpected accepted-source inventory coverage")

    documents = evidence / "native-documents.zip"
    if digest(documents.read_bytes()) != metadata["native_documents_sha256"]:
        raise ValueError("Unexpected native-documents checksum")
    files = {}
    folded = set()
    with ZipFile(documents) as native:
        projected = set(native.namelist())
        if len(projected) != len(native.namelist()):
            raise ValueError("Duplicate path in native-documents archive")
        for row in rows:
            expected, name = row.split("\t")
            path = PurePosixPath(name)
            if (
                not path.parts or path.is_absolute() or name != path.as_posix()
                or any(
                    part == ".." or part != part.rstrip(" .")
                    or re.search(r'[<>:"\\|?*\x00-\x1f]', part)
                    for part in path.parts
                )
                or name.casefold() in folded
            ):
                raise ValueError(f"Invalid source path: {name}")
            payload = native.read(name) if name in projected else (public / path).read_bytes()
            if digest(payload) != expected:
                raise ValueError(f"Accepted-source checksum mismatch: {name}")
            files[name] = payload
            folded.add(name.casefold())
        if not projected.issubset(files):
            raise ValueError("Native documents lie outside the accepted-source inventory")

    if digest(files["Cargo.lock"]) != metadata["cargo_lock_sha256"]:
        raise ValueError("Unexpected accepted Cargo.lock checksum")
    kind = dict(line.split("=", 1) for line in files["ZIP_KIND.txt"].decode().splitlines() if line)
    if kind.get("revision") != revision or kind.get("version") != metadata["version"]:
        raise ValueError("Verified source has the wrong release identity")
    print(f"Verified {len(files)} accepted {revision} source files: {metadata['inventory_sha256']}")
    return files


def prepare(public, build, downloads, revision="r2014"):
    files = validated_files(public, revision)
    if build.exists() or downloads.exists():
        raise FileExistsError("Use empty paths for the assembled build and download")
    build.mkdir()
    downloads.mkdir()
    with ZipFile(downloads / f"EtuSchlopf-{revision}-source.zip", "w", ZIP_DEFLATED) as archive:
        for name, payload in files.items():
            destination = build / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(payload)
            archive.writestr(name, payload)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("public", type=Path, help="Public r2014 checkout")
    parser.add_argument("build", type=Path, help="New native build directory")
    parser.add_argument("downloads", type=Path, help="New source archive directory")
    args = parser.parse_args()
    prepare(args.public, args.build, args.downloads)
