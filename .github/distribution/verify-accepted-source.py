"""Verify the accepted r1964 source without distributing a second path inventory."""
import argparse
import hashlib
from pathlib import Path


BASE = Path("docs/evidence/r1961/accepted_source_sha256.tsv")
BASE_SHA256 = "cb5f50ffb09285b764f08416d32bf529e283dea83d7a57219d393b13c7c3dc33"
PROJECTION = Path("docs/evidence/r1964/github_projection_sha256.tsv")
ADDED = (
    "docs/selector_derivation_contract_bound_duration_seed.md",
    "docs/selector_derivation_contract_bound_written_duration.md",
    "docs/selector_derivation_contract_inferred_duration_seed.md",
    "src/midi_seed/bound_duration.rs",
)


def verified_rows(path, checksum, header):
    raw = path.read_bytes()
    if hashlib.sha256(raw).hexdigest() != checksum:
        raise ValueError(f"Checksum mismatch for {path}")
    columns, *lines = raw.decode("utf-8").splitlines()
    if columns != header:
        raise ValueError(f"Unexpected header in {path}")
    return [line.split("\t") for line in lines]


def verify(root, expected_source, expected_projection):
    base = {path: digest for digest, path in verified_rows(root / BASE, BASE_SHA256, "sha256\tpath")}
    projection = dict(verified_rows(root / PROJECTION, expected_projection, "path\tsha256"))
    paths = set(base).union(ADDED)
    if len(paths) != 2217 or not projection.keys() <= base.keys():
        raise ValueError("Unexpected native source path set or document projection")
    combined = hashlib.sha256(b"sha256\tpath\n")
    for path in sorted(paths):
        actual = hashlib.sha256((root / path).read_bytes()).hexdigest()
        if path in projection:
            if actual != projection[path]:
                raise ValueError(f"Public documentation differs from its projection: {path}")
            # The six original document contents are unchanged since r1961.
            actual = base[path]
        combined.update(f"{actual}\t{path}\n".encode("utf-8"))
    if combined.hexdigest() != expected_source:
        raise ValueError("The reconstructed source differs from the natively accepted revision")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source_sha256")
    parser.add_argument("projection_sha256")
    args = parser.parse_args()
    verify(Path.cwd(), args.source_sha256, args.projection_sha256)
    print("Verified the accepted r1964 source and public documentation.")
