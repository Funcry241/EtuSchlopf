"""Verify accepted native source using the historical inventory and public document hashes."""
import argparse
import hashlib
import subprocess
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
# Only these four historical documentation files differ from both accepted
# snapshots. r1964 also projected two reader documents which r1965 binds verbatim.
PROJECTED_DOCUMENTS = {
    "docs/CONTRIBUTING.md", "docs/README.md", "docs/THIRD_PARTY_NOTICES.md",
    "docs/community/issue_templates/01_fehler.md",
}
CANONICAL_MATH = "docs/MATHEMATIK.md"
RETIRED_MATH = "docs/MATHEMATIK-VOLLKORPUS.md"
PUBLIC_ADDITIONS = {
    ".gitattributes", ".gitignore", "LICENSE",
    "docs/DEVELOPMENT.md", "docs/IMPLEMENTED_FOUNDATIONS.md", "docs/RESEARCH_CONTEXT.md",
}
# These public packaging, documentation and evidence areas extend the native Take.
# Every native file that also lives there is still compared below.
PUBLIC_DIRECTORIES = (".github/", "docs/evidence/", "docs/examples/", "docs/images/")


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
        permitted = PROJECTED_DOCUMENTS | {"README.md", "docs/MATHEMATIK.md"}
        expected_count = 2217
    elif revision == "r1965":
        paths.update(ADDED_R1965)
        permitted = PROJECTED_DOCUMENTS
        expected_count = 2219
    else:
        raise ValueError(f"Unknown native revision: {revision}")
    if len(paths) != expected_count or len(rows) != len(projection) or set(projection) != permitted:
        raise ValueError("Unexpected native source path set or document projection")
    combined = hashlib.sha256(b"sha256\tpath\n")
    for path in sorted(paths):
        actual = hashlib.sha256((root / path).read_bytes()).hexdigest()
        if path in projection:
            if actual != projection[path]:
                raise ValueError(f"Public documentation differs from its projection: {path}")
            # Each permitted original is unchanged from the trusted r1961 inventory.
            actual = base[path]
        combined.update(f"{actual}\t{path}\n".encode("utf-8"))
    if combined.hexdigest() != expected_source:
        raise ValueError("The reconstructed source differs from the natively accepted revision")


def regular_files(root, ignored=()):
    def walk(directory):
        for entry in directory.iterdir():
            if directory == root and entry.name in ignored:
                continue
            if entry.is_symlink():
                raise ValueError(f"Unexpected symbolic link: {entry}")
            if entry.is_dir():
                yield from walk(entry)
            elif entry.is_file():
                yield entry.relative_to(root).as_posix(), entry
            else:
                raise ValueError(f"Expected a regular file: {entry}")

    return dict(walk(root))


def verify_r1968(root, accepted_root, expected_source, expected_projection):
    """Compare the public Git checkout with the independently unpacked Windows Take."""
    native = {
        name: hashlib.sha256(path.read_bytes()).hexdigest()
        for name, path in regular_files(accepted_root).items()
    }
    # Same headerless UTF-8 inventory as the original Windows check, including Cargo.lock.
    inventory = hashlib.sha256()
    for name, digest in sorted(native.items()):
        inventory.update(f"{digest}\t{name}\n".encode("utf-8"))
    if len(native) != 2219 or inventory.hexdigest() != expected_source:
        raise ValueError("The supplied native tree differs from the accepted r1968 source")

    table = root / "docs/evidence/r1968/github_projection_sha256.tsv"
    rows = verified_rows(table, expected_projection, "path\tpublic_content")
    projection = dict(rows)
    allowed = PROJECTED_DOCUMENTS | {"README.md", CANONICAL_MATH, RETIRED_MATH}
    if len(rows) != len(allowed) or set(projection) != allowed or not allowed.issubset(native):
        raise ValueError("Unexpected r1968 document projection")
    if projection[CANONICAL_MATH] != "git:HEAD" or projection[RETIRED_MATH] != "absent":
        raise ValueError("Unexpected publication rule for the mathematics documents")

    public = regular_files(root, ignored={".git", "target"})
    if (root / RETIRED_MATH).exists() or (root / "docs/MATHEMATIK-AKTUELL.md").exists():
        raise ValueError("Retired public mathematics page still exists")
    for name in sorted(public.keys() - native.keys()):
        if name not in PUBLIC_ADDITIONS and not name.startswith(PUBLIC_DIRECTORIES):
            raise ValueError(f"Unexpected file outside the accepted source: {name}")
    for name, native_digest in sorted(native.items()):
        if name == RETIRED_MATH:
            continue
        if name not in public:
            raise ValueError(f"Missing public file: {name}")
        content = public[name].read_bytes()
        if name == CANONICAL_MATH:
            committed = subprocess.run(
                ["git", "cat-file", "blob", f"HEAD:{name}"], cwd=root,
                capture_output=True, check=False,
            )
            if committed.returncode or content != committed.stdout:
                raise ValueError(f"The mathematics page must match HEAD in the public Git checkout: {name}")
        elif hashlib.sha256(content).hexdigest() != projection.get(name, native_digest):
            raise ValueError(f"Public file differs from the accepted source or its declared projection: {name}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("source_sha256")
    parser.add_argument("projection_sha256")
    parser.add_argument("--revision", choices=("r1964", "r1965", "r1968"), default="r1964")
    parser.add_argument("--accepted-root", type=Path, help="Unpacked r1968 Take with the verified parent Cargo.lock")
    args = parser.parse_args()
    if args.revision == "r1968":
        if args.accepted_root is None:
            parser.error("--revision r1968 requires --accepted-root and a public Git checkout")
        verify_r1968(Path.cwd(), args.accepted_root, args.source_sha256, args.projection_sha256)
    else:
        if args.accepted_root is not None:
            parser.error("--accepted-root applies only to r1968")
        verify(Path.cwd(), args.source_sha256, args.projection_sha256, args.revision)
    print(f"Verified the accepted {args.revision} source and public documentation.")
