"""Reconstruct and hash-check every file of the accepted native source snapshot."""
from pathlib import Path, PurePosixPath
import hashlib
import json
import shutil
import sys
import zipfile


def reconstruct(base: Path, manifest_path: Path, overlay_path: Path, destination: Path):
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    assert manifest["revision"] == "r1537"
    assert manifest["source_snapshot_sha256"] == "40f1a5f76117b57cd342426c71c834b6225cd18d2dd0b31e6d134577a734fd12"
    if destination.exists():
        raise ValueError("Destination must be new; do not mix source trees")
    rows = manifest["files"]
    paths = [row["path"] for row in rows]
    assert len(set(paths)) == len(paths)
    with zipfile.ZipFile(overlay_path) as overlay:
        assert set(overlay.namelist()) == {row["path"] for row in rows if row["overlay"]}
        for row in rows:
            path = PurePosixPath(row["path"])
            assert not path.is_absolute() and ".." not in path.parts
            data = overlay.read(str(path)) if row["overlay"] else (base / str(path)).read_bytes()
            assert len(data) == row["bytes"], row["path"]
            assert hashlib.sha256(data).hexdigest() == row["sha256"], row["path"]
            target = destination / str(path)
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
    assert sorted(p.relative_to(destination).as_posix() for p in destination.rglob("*") if p.is_file()) == sorted(paths)
    print(f"Verified all {len(rows)} native source files against the accepted r1537 snapshot.")


if __name__ == "__main__":
    reconstruct(*(Path(value) for value in sys.argv[1:]))
