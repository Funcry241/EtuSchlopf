"""Prepare the r1968 engine and both source archives from the accepted Take and lockfile."""
import argparse
import hashlib
from pathlib import Path
from shutil import copyfile
from zipfile import ZIP_DEFLATED, ZipFile, ZipInfo


TAKE = "EtuSchlopf_v0_10_1839_r1968_Reduktion_unbuilt_source.zip"
TAKE_SHA256 = "2138a8f2ad1746367c443f5c7ac80dada47b48f04414f2f21a773bf4f7a11880"
LOCK_SHA256 = "42bfebd3ba895e879b7ae8192da48eb951104cd356997dc6fb0a3ac4560b872d"


def prepare(review, build, downloads):
    take = review / TAKE
    lock = (review / "Cargo.lock").read_bytes()
    if hashlib.sha256(take.read_bytes()).hexdigest() != TAKE_SHA256:
        raise ValueError("The ZIP differs from the accepted r1968 Take")
    if hashlib.sha256(lock).hexdigest() != LOCK_SHA256:
        raise ValueError("The dependency lockfile differs from the accepted build")

    # Fresh directories prevent stale sources or build outputs from entering the package.
    build.mkdir()
    downloads.mkdir()
    copyfile(take, downloads / TAKE)
    source = downloads / "EtuSchlopf-r1968-source.zip"
    copyfile(take, source)
    with ZipFile(source, "a") as archive:
        info = ZipInfo("Cargo.lock", archive.getinfo("ZIP_KIND.txt").date_time)
        info.create_system = 3
        info.external_attr = 0o100644 << 16
        archive.writestr(info, lock, compress_type=ZIP_DEFLATED, compresslevel=6)
    with ZipFile(source) as archive:
        archive.extractall(build)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("review", type=Path, help="Directory containing the accepted Take and Cargo.lock")
    parser.add_argument("build", type=Path, help="New native build directory")
    parser.add_argument("downloads", type=Path, help="New output directory for the two source ZIPs")
    args = parser.parse_args()
    prepare(args.review, args.build, args.downloads)
