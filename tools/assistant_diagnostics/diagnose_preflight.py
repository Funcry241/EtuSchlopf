"""Read-only r1463 scratch preflight. Not a Take validator or installer."""
from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import subprocess
import tempfile
import xml.etree.ElementTree as ET

ROOT_FILES = ("Cargo.toml", "README.md", "ZIP_KIND.txt", "run.cmd", "strict.cmd",
              "rust-toolchain.toml", "rustfmt.toml", "clippy.toml")
DIRS = ("src", "docs", "tools", "tests")
MANIFEST = "src/build_gate/take_change_class.txt"
SMOKE = '''<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE score-partwise PUBLIC "-//Recordare//DTD MusicXML 4.0 Partwise//EN" "http://www.musicxml.org/dtds/partwise.dtd">
<score-partwise version="4.0"><work><work-title>Renderer preflight</work-title></work>
<part-list><score-part id="P1"><part-name>Piano</part-name></score-part></part-list>
<part id="P1"><measure number="1"><attributes><divisions>4</divisions>
<key><fifths>0</fifths></key><time><beats>4</beats><beat-type>4</beat-type></time>
<clef><sign>G</sign><line>2</line></clef></attributes>
<note><pitch><step>C</step><octave>4</octave></pitch><duration>2</duration><voice>1</voice><type>eighth</type><stem>up</stem><beam number="1">begin</beam></note>
<note><pitch><step>D</step><octave>4</octave></pitch><duration>2</duration><voice>1</voice><type>eighth</type><stem>up</stem><beam number="1">end</beam></note>
<note><pitch><step>E</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><type>quarter</type></note>
<note><pitch><step>F</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><type>quarter</type></note>
<note><pitch><step>G</step><octave>4</octave></pitch><duration>4</duration><voice>1</voice><type>quarter</type></note>
</measure></part></score-partwise>'''


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def fnv(data: bytes) -> str:
    value = 0xCBF29CE484222325
    for byte in data:
        value = ((value ^ byte) * 0x100000001B3) & 0xFFFFFFFFFFFFFFFF
    return f"fnv64:{value:016x}"


def relative_path(value: str) -> str:
    path = PurePosixPath(value)
    if not value or path.is_absolute() or ".." in path.parts or "\\" in value or ":" in value:
        raise ValueError(f"Unsafe or noncanonical manifest path: {value!r}")
    if path.as_posix() != value:
        raise ValueError(f"Noncanonical manifest path: {value!r}")
    return value


def managed_files(root: Path) -> dict[str, Path]:
    result = {}
    for name in ROOT_FILES:
        path = root / name
        if path.is_symlink():
            raise ValueError(f"Symlink not supported: {path}")
        if path.is_file():
            result[name] = path
    for directory in DIRS:
        base = root / directory
        if not base.is_dir() or base.is_symlink():
            raise ValueError(f"Missing or linked managed directory: {base}")
        for current, directories, filenames in os.walk(base, followlinks=False):
            directories[:] = [name for name in directories if name != "target"]
            for name in directories + filenames:
                if (Path(current) / name).is_symlink():
                    raise ValueError(f"Symlink not supported: {Path(current) / name}")
            for name in filenames:
                path = Path(current) / name
                result[path.relative_to(root).as_posix()] = path
    return result


def check_tree(root: Path, reference: dict) -> dict:
    actual = managed_files(root)
    baseline = reference.get("managed_sha256")
    if not isinstance(baseline, dict) or not baseline:
        raise ValueError("A nonempty managed-file reference is required")
    for name in baseline:
        relative_path(name)
    declarations = {}
    for line in (root / MANIFEST).read_text(encoding="utf-8-sig").splitlines():
        if not line.startswith("change="):
            continue
        fields = line[7:].split("|")
        if len(fields) != 3 or fields[0] not in ("added", "modified", "deleted"):
            raise ValueError(f"Malformed declaration: {line}")
        kind, name, digest = fields
        relative_path(name)
        if name == MANIFEST or name in declarations:
            raise ValueError(f"Duplicate or self-referential declaration: {name}")
        declarations[name] = (kind, digest)
    problems = []
    for name, (kind, digest) in declarations.items():
        if kind == "deleted":
            if name in actual or digest != "-":
                problems.append(f"Invalid deletion: {name}")
        elif name not in actual:
            problems.append(f"Declared file absent: {name}")
        elif fnv(actual[name].read_bytes()) != digest:
            problems.append(f"STALE MANIFEST HASH: {name}")
    changed = []
    for name in sorted(set(baseline) | set(actual)):
        digest = sha(actual[name].read_bytes()) if name in actual else None
        if digest == baseline.get(name):
            continue
        changed.append(name)
        if name == MANIFEST:
            continue  # Parent validator checks the complete manifest semantics.
        declaration = declarations.get(name)
        if declaration is None:
            problems.append(f"UNDECLARED CHANGE vs accepted r1463: {name}")
        elif name not in actual and declaration[0] != "deleted":
            problems.append(f"Missing deletion declaration: {name}")
        elif name in actual and declaration[0] == "deleted":
            problems.append(f"Changed file declared deleted: {name}")
    if problems:
        raise ValueError("\n".join(problems))
    return {"managed_files": len(actual), "changes_vs_reference": changed,
            "declared_hashes_checked": len(declarations),
            "root_cargo_lock": "local state, excluded; nested managed locks included",
            "scope": "cheap hash/undeclared-change check; not canonical Take admission"}


def execute(command: list[str], cwd: Path, timeout: int, env: dict) -> dict:
    completed = subprocess.run(command, cwd=cwd, env=env, shell=False,
                               capture_output=True, text=True, errors="replace", timeout=timeout)
    result = {"command": command, "exit_code": completed.returncode,
              "stdout": completed.stdout, "stderr": completed.stderr}
    if completed.returncode:
        raise ValueError(json.dumps(result, ensure_ascii=False))
    return result


def executable(value: str) -> str:
    resolved = shutil.which(value)
    if resolved is None:
        raise ValueError(f"Executable not found: {value}")
    return str(Path(resolved).resolve())


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--tree", type=Path, required=True)
    parser.add_argument("--reference-tree", type=Path, help="Optional separately verified accepted parent tree; otherwise frozen r1463 reference")
    parser.add_argument("--out", type=Path, required=True, help="New output directory outside the project")
    parser.add_argument("--renderer", required=True, help="MuseScore executable, or Python for a wrapper")
    parser.add_argument("--renderer-arg", action="append", default=[], help="Fixed prefix argument, e.g. wrapper path")
    parser.add_argument("--renderer-binary", type=Path, help="Real MuseScore binary; required with wrapper")
    parser.add_argument("--rustc", default="rustc")
    parser.add_argument("--cargo", default="cargo")
    parser.add_argument("--timeout", type=int, default=60, help="Seconds per smoke command, not a productive search budget")
    args = parser.parse_args()
    root = args.tree.resolve()
    out = args.out.resolve()
    if out == root or root in out.parents or out.exists():
        parser.error("--out must be a new directory outside --tree")
    if args.timeout <= 0:
        parser.error("--timeout must be positive")
    if args.reference_tree and args.reference_tree.resolve() == root:
        parser.error("--reference-tree must not be the diagnostic tree itself")
    out.mkdir(parents=True)
    report = {"status": "FAILED", "take_admission": "NOT_RUN", "native_corpus": "NOT_RUN",
              "tree": str(root), "steps": {}}
    try:
        reference = json.loads(Path(__file__).with_name("accepted_r1463_managed_sha256.json").read_text())
        if args.reference_tree:
            reference = {"managed_sha256": {n: sha(p.read_bytes()) for n, p in managed_files(args.reference_tree.resolve()).items()}}
        report["reference"] = str(args.reference_tree.resolve()) if args.reference_tree else "packaged accepted-r1463 reference"
        report["steps"]["source_manifest"] = check_tree(root, reference)
        env = os.environ.copy()
        rustc, cargo = executable(args.rustc), executable(args.cargo)
        env["PATH"] = str(Path(cargo).parent) + os.pathsep + env.get("PATH", "")
        for name, command, pattern in (
            ("rustc", [rustc, "--version"], r"\brustc 1\.98\.0\b"),
            ("cargo", [cargo, "--version"], r"\bcargo 1\.98\.0\b"),
            ("clippy", [cargo, "clippy", "--version"], r"\bclippy 0\.1\.98\b"),
        ):
            result = execute(command, out, args.timeout, env)
            report["steps"][name] = result
            if not re.search(pattern, result["stdout"] + result["stderr"]):
                raise ValueError(f"Unexpected {name} version")
        renderer = executable(args.renderer)
        if args.renderer_arg and args.renderer_binary is None:
            raise ValueError("With a wrapper, specify --renderer-binary for the actual MuseScore SHA-256")
        binary = (args.renderer_binary or Path(renderer)).resolve()
        if not binary.is_file():
            raise ValueError(f"Renderer binary absent: {binary}")
        prefix = [renderer, *args.renderer_arg]
        version = execute([*prefix, "--version"], out, args.timeout, env)
        report["steps"]["renderer_version"] = version
        if not re.search(r"\b4\.7\.4\b", version["stdout"] + version["stderr"]):
            raise ValueError("Expected MuseScore 4.7.4")
        report["renderer_identity"] = {"binary": str(binary), "sha256": sha(binary.read_bytes()),
                                       "prefix": prefix, "cwd": str(out),
                                       "limit": "version/binary and smoke command recorded; profile/fonts/environment not fully fingerprinted"}
        with tempfile.TemporaryDirectory(prefix="smoke-", dir=out) as directory:
            smoke = Path(directory)
            source = smoke / "smoke.musicxml"
            source.write_text(SMOKE, encoding="utf-8")
            original = source.read_bytes()
            for suffix in ("pdf", "svg"):
                target = smoke / f"smoke.{suffix}"
                result = execute([*prefix, "-o", str(target), str(source)], out, args.timeout, env)
                report["steps"][f"renderer_{suffix}"] = result
                outputs = [target] if target.is_file() else sorted(smoke.glob(f"smoke-*.{suffix}"))
                if not outputs:
                    raise ValueError(f"No fresh {suffix} output")
                for path in outputs:
                    data = path.read_bytes()
                    if suffix == "pdf":
                        if not data.startswith(b"%PDF-") or b"%%EOF" not in data[-4096:]:
                            raise ValueError("Invalid PDF smoke output")
                    elif ET.fromstring(data).tag.rsplit("}", 1)[-1] != "svg":
                        raise ValueError("Invalid SVG smoke output")
                if source.read_bytes() != original:
                    raise ValueError("Renderer mutated smoke MusicXML")
                result["outputs"] = [{"name": p.name, "bytes": p.stat().st_size,
                                      "sha256": sha(p.read_bytes())} for p in outputs]
        report["status"] = "PREFLIGHT_READY"
    except (OSError, ValueError, subprocess.TimeoutExpired, ET.ParseError) as error:
        report["error"] = str(error)
    (out / "preflight.json").write_text(json.dumps(report, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"{report['status']}: {out / 'preflight.json'}")
    if report["status"] != "PREFLIGHT_READY":
        print(report.get("error", "unknown error"))
        return 1
    print("No build or corpus started. Canonical validators and geometry readback still required.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
