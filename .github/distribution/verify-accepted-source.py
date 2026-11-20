"""Verify accepted native source using the historical inventory and public document hashes."""
import argparse
import hashlib
from pathlib import Path


BASE = Path("docs/evidence/r1961/accepted_source_sha256.tsv")
BASE_SHA256 = "cb5f50ffb09285b764f08416d32bf529e283dea83d7a57219d393b13c7c3dc33"
ADDED_R1964 = (
    "docs/selector_derivation_contract_bound_duration_seed.md",
    "docs/selector_derivation_contract_bound_written_duration.md",
    "docs/selector_derivation_contract_inferred_duration_seed.md",
    "src/midi_seed/bound_duration.rs",
)
ADDED_R1965 = (
    "docs/MATHEMATIK-VOLLKORPUS.md",
    "docs/selector_derivation_contract_performance_trace_equivalence.md",
)
# The canonical 1961 inventory supplies the other original document hashes.
# These two public documents have different native r1965 source bytes.
NATIVE_R1965 = {
    "README.md": "3eec8eff3b7f57251c43553a9629e022d323899639e365591ddffd966f59632a",
    "docs/MATHEMATIK-VOLLKORPUS.md": "f698cda1fd58481e20fd8ca21be69a3600afec365e89d56e7a15be7f870093fb",
}
PROJECTED_DOCUMENTS = {
    "README.md", "docs/CONTRIBUTING.md", "docs/README.md",
    "docs/THIRD_PARTY_NOTICES.md", "docs/community/issue_templates/01_fehler.md",
}



def verified_rows(path, checksum, header):
    raw = path.read_bytes()
    if hashlib.sha256(raw).hexdigest() != checksum:
        raise ValueError(f"Checksum mismatch for {path}")
    columns, *lines = raw.decode("utf-8").splitlines()
    if columns != header:
        raise ValueError(f"Unexpected header in {path}")
    return [line.split("\t") for line in lines]


def verify(root, expected_source, expected_projection, revision="r1964"):
    base = {path: digest for digest, path in verified_rows(root / BASE, BASE_SHA256, "sha256\tpath")}
    table = root / "docs" / "evidence" / revision / "github_projection_sha256.tsv"
    rows = verified_rows(table, expected_projection, "path\tsha256")
    projection = dict(rows)
    paths = set(base).union(ADDED_R1964)
    if revision == "r1964":
        permitted = PROJECTED_DOCUMENTS | {"docs/MATHEMATIK.md"}
        expected_count = 2217
    elif revision == "r1965":
        paths.update(ADDED_R1965)
        permitted = PROJECTED_DOCUMENTS | {"docs/MATHEMATIK-VOLLKORPUS.md"}
        expected_count = 2219
    else:
        raise ValueError(f"Unknown native revision: {revision}")
    if len(paths) != expected_count or len(rows) != len(projection) or set(projection) != permitted:
        raise ValueError("Unexpected native source path set or document projection")
    # Use original digests only for the individually checked public projections.
    native = base | (NATIVE_R1965 if revision == "r1965" else {})
    combined = hashlib.sha256(b"sha256\tpath\n")
    for path in sorted(paths):
        actual = hashlib.sha256((root / path).read_bytes()).hexdigest()
        if path in projection:
            if actual != projection[path]:
                raise ValueError(f"Public documentation differs from its projection: {path}")
            actual = native[path]
        combined.update(f"{actual}\t{path}\n".encode("utf-8"))
    if combined.hexdigest() != expected_source:
        raise ValueError("The reconstructed source differs from the natively accepted revision")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source_sha256")
    parser.add_argument("projection_sha256")
    parser.add_argument("--revision", choices=("r1964", "r1965"), default="r1964")
    args = parser.parse_args()
    verify(Path.cwd(), args.source_sha256, args.projection_sha256, args.revision)
    print(f"Verified the accepted {args.revision} source and public documentation.")
