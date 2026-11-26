#!/usr/bin/env python3
"""Regenerate both synthetic PDF notehead proofs with a local MuseScore."""
import argparse
import json
from pathlib import Path
from shutil import copyfile
import subprocess
from tempfile import TemporaryDirectory

from check_heads import verify


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("musescore", type=Path, help="path to the MuseScore executable")
    args = parser.parse_args()
    musescore = args.musescore.resolve(strict=True)
    sources = Path(__file__).resolve().parent
    result = {}
    with TemporaryDirectory(prefix="etuschlopf-pdf-heads-") as temp:
        for name in ("up", "down"):
            fixture = Path(temp) / name
            fixture.mkdir()
            for variant in ("plain", "marked"):
                copyfile(sources / name / f"{variant}.musicxml", fixture / f"{variant}.musicxml")
            for output, score in (("plain.pdf", "plain"), ("marked.pdf", "marked"), ("marked.svg", "marked")):
                run = subprocess.run(
                    [str(musescore), "-o", str(fixture / output), str(fixture / f"{score}.musicxml")],
                    cwd=fixture, capture_output=True,
                )
                target = "marked-1.svg" if output.endswith(".svg") else output
                if run.returncode or not (fixture / target).is_file():
                    issue = run.stderr.decode(errors="replace").strip()
                    raise RuntimeError(f"MuseScore did not create {target} (exit {run.returncode}): {issue}")
            result[name] = verify(fixture, True)
    print(json.dumps(result, indent=2))


if __name__ == "__main__":
    main()
